//! 🔍 Feature detection, binary description, matching and optical flow: oriented FAST, rBRIEF, Hamming matching and pyramidal Lucas-Kanade.

use remodel_image::{build_pyramid, extract_patch, gaussian_blur, scharr_gradients, zncc, GradientField, ImageGray, Pyramid};

// #region 🔖Keypoint
/// 📍 A detected image feature: subpixel position, source pyramid octave, dominant orientation in radians (intensity-centroid convention), and detector response/score.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Keypoint {
    pub x: f32,
    pub y: f32,
    pub octave: u8,
    pub angle: f32,
    pub response: f32,
}
// #endregion 🔖Keypoint

// #region 🔖Detect
const FAST_CIRCLE_OFFSETS: [(i32, i32); 16] = [
    (0, -3),
    (1, -3),
    (2, -2),
    (3, -1),
    (3, 0),
    (3, 1),
    (2, 2),
    (1, 3),
    (0, 3),
    (-1, 3),
    (-2, 2),
    (-3, 1),
    (-3, 0),
    (-3, -1),
    (-2, -2),
    (-1, -3),
];
const FAST_ARC_LENGTH: usize = 9;
const FAST_BORDER_MARGIN: i64 = 3;
const ORB_HARRIS_K: f32 = 0.04;
const ORB_STRUCTURE_TENSOR_SIGMA: f32 = 1.5;
const ORB_FAST_THRESHOLD: f32 = 0.08;
const ORB_GRID_CELL: u32 = 24;
const ORB_CENTROID_RADIUS: i32 = 9;

fn fast_longest_arc(signs: &[i8; 16]) -> Option<(usize, usize)> {
    let mut best: Option<(usize, usize)> = None;
    for start in 0..16 {
        let sign = signs[start];
        if sign == 0 {
            continue;
        }
        let mut len = 1;
        while len < 16 && signs[(start + len) % 16] == sign {
            len += 1;
        }
        if best.is_none_or(|(_, best_len)| len > best_len) {
            best = Some((start, len));
        }
    }
    best
}

/// 🟢 Classic FAST-9 corner response: for every pixel with a 3px border margin, samples the 16-point Bresenham radius-3 circle and looks for a contiguous arc of at least 9 points all brighter than `center + threshold` or all darker than `center - threshold`; the returned score is the summed absolute intensity difference from the center over the qualifying arc.
/// <https://www.edwardrosten.com/work/fast.html>
pub fn fast_corners(img: &ImageGray, threshold: f32) -> Vec<(u32, u32, f32)> {
    let (w, h) = (i64::from(img.width), i64::from(img.height));
    let mut out = Vec::new();
    if w <= 2 * FAST_BORDER_MARGIN || h <= 2 * FAST_BORDER_MARGIN {
        return out;
    }
    for y in FAST_BORDER_MARGIN..(h - FAST_BORDER_MARGIN) {
        for x in FAST_BORDER_MARGIN..(w - FAST_BORDER_MARGIN) {
            let center = img.get(x as u32, y as u32);
            let circle: [f32; 16] = std::array::from_fn(|i| {
                let (dx, dy) = FAST_CIRCLE_OFFSETS[i];
                img.get((x + i64::from(dx)) as u32, (y + i64::from(dy)) as u32)
            });
            let signs: [i8; 16] = std::array::from_fn(|i| {
                if circle[i] > center + threshold {
                    1
                } else if circle[i] < center - threshold {
                    -1
                } else {
                    0
                }
            });
            if let Some((start, len)) = fast_longest_arc(&signs) {
                if len >= FAST_ARC_LENGTH {
                    let score = (0..len).map(|k| (circle[(start + k) % 16] - center).abs()).sum();
                    out.push((x as u32, y as u32, score));
                }
            }
        }
    }
    out
}

fn structure_tensor_fields(img: &ImageGray) -> (ImageGray, ImageGray, ImageGray) {
    let g = scharr_gradients(img);
    let gx2 = ImageGray { width: img.width, height: img.height, data: g.gx.iter().map(|&v| v * v).collect() };
    let gy2 = ImageGray { width: img.width, height: img.height, data: g.gy.iter().map(|&v| v * v).collect() };
    let gxy = ImageGray { width: img.width, height: img.height, data: g.gx.iter().zip(g.gy.iter()).map(|(&a, &b)| a * b).collect() };
    (gaussian_blur(&gx2, ORB_STRUCTURE_TENSOR_SIGMA), gaussian_blur(&gy2, ORB_STRUCTURE_TENSOR_SIGMA), gaussian_blur(&gxy, ORB_STRUCTURE_TENSOR_SIGMA))
}

/// 🌄 Per-pixel Harris corner response `det(M) - k trace(M)^2` from the gaussian-weighted structure tensor `M` of the Scharr gradients.
/// <https://en.wikipedia.org/wiki/Corner_detection#The_Harris_.26_Stephens_.2F_Plessey_.2F_Shi.E2.80.93Tomasi_corner_detection_algorithms>
pub fn harris_response(img: &ImageGray, k: f32) -> Vec<f32> {
    let (sxx, syy, sxy) = structure_tensor_fields(img);
    sxx.data
        .iter()
        .zip(syy.data.iter())
        .zip(sxy.data.iter())
        .map(|((&a, &d), &b)| {
            let det = a * d - b * b;
            let trace = a + d;
            det - k * trace * trace
        })
        .collect()
}

/// 🗺️ Grid-based adaptive non-maximal suppression: partitions the image into `cell x cell` bins and keeps, per bin, the top `per_cell` pixels by Shi-Tomasi minimum eigenvalue `mean(M) - sqrt(diff(M)^2 + off(M)^2)` of the gaussian-weighted structure tensor, for spatially uniform coverage.
/// <https://en.wikipedia.org/wiki/Corner_detection#The_Harris_.26_Stephens_.2F_Plessey_.2F_Shi.E2.80.93Tomasi_corner_detection_algorithms>
pub fn shi_tomasi_grid(img: &ImageGray, cell: u32, per_cell: usize) -> Vec<(u32, u32, f32)> {
    let cell = cell.max(1);
    let (sxx, syy, sxy) = structure_tensor_fields(img);
    let cells_x = img.width.div_ceil(cell).max(1);
    let cells_y = img.height.div_ceil(cell).max(1);
    let mut buckets: Vec<Vec<(u32, u32, f32)>> = vec![Vec::new(); (cells_x * cells_y) as usize];
    for (idx, ((&a, &d), &b)) in sxx.data.iter().zip(syy.data.iter()).zip(sxy.data.iter()).enumerate() {
        let x = idx as u32 % img.width;
        let y = idx as u32 / img.width;
        let mean = (a + d) * 0.5;
        let diff = (a - d) * 0.5;
        let min_eig = mean - (diff * diff + b * b).sqrt();
        let bucket = ((y / cell) * cells_x + (x / cell)) as usize;
        buckets[bucket].push((x, y, min_eig));
    }
    let mut out = Vec::new();
    for bucket in &mut buckets {
        bucket.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        out.extend(bucket.iter().take(per_cell).copied());
    }
    out
}

fn bucket_top_k(points: &[(u32, u32, f32)], cell: u32, cells_x: u32, per_cell: usize) -> Vec<(u32, u32, f32)> {
    let mut buckets: std::collections::HashMap<u32, Vec<(u32, u32, f32)>> = std::collections::HashMap::new();
    for &(x, y, score) in points {
        let bucket = (y / cell) * cells_x + (x / cell);
        buckets.entry(bucket).or_default().push((x, y, score));
    }
    let mut keys: Vec<u32> = buckets.keys().copied().collect();
    keys.sort_unstable();
    let mut out = Vec::new();
    for key in keys {
        let bucket = buckets.get_mut(&key).expect("key was collected from buckets.keys()");
        bucket.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        out.extend(bucket.iter().take(per_cell).copied());
    }
    out
}

fn intensity_centroid_angle(level: &ImageGray, cx: f32, cy: f32, radius: i32) -> f32 {
    let mut m10 = 0.0f32;
    let mut m01 = 0.0f32;
    let radius_sq = (radius * radius) as f32;
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let (fx, fy) = (dx as f32, dy as f32);
            if fx * fx + fy * fy > radius_sq {
                continue;
            }
            let intensity = level.sample(cx + fx, cy + fy);
            m10 += fx * intensity;
            m01 += fy * intensity;
        }
    }
    m01.atan2(m10)
}

/// 🎯 Oriented-FAST keypoint detector across a pyramid: runs [`fast_corners`] per level, rescoring survivors with [`harris_response`] (dropping edge/flat responses), distributes `target_count` across levels proportional to pixel area, applies a grid-bucketed top-k (à la [`shi_tomasi_grid`]) for spatial spread, and assigns an orientation via the intensity centroid `atan2(sum(y I), sum(x I))` over a circular neighbourhood.
/// <https://en.wikipedia.org/wiki/Oriented_FAST_and_rotated_BRIEF>
pub fn detect_orb_keypoints(pyramid: &Pyramid, target_count: usize) -> Vec<Keypoint> {
    if pyramid.levels.is_empty() || target_count == 0 {
        return Vec::new();
    }
    let areas: Vec<f64> = pyramid.levels.iter().map(|lvl| f64::from(lvl.width) * f64::from(lvl.height)).collect();
    let total_area: f64 = areas.iter().sum();
    let mut keypoints = Vec::new();
    for (octave, level) in pyramid.levels.iter().enumerate() {
        if level.width <= 2 * FAST_BORDER_MARGIN as u32 || level.height <= 2 * FAST_BORDER_MARGIN as u32 || octave > usize::from(u8::MAX) {
            continue;
        }
        let level_target = if total_area > 0.0 { ((areas[octave] / total_area) * target_count as f64).round() as usize } else { 0 };
        if level_target == 0 {
            continue;
        }
        let harris = harris_response(level, ORB_HARRIS_K);
        let candidates: Vec<(u32, u32, f32)> = fast_corners(level, ORB_FAST_THRESHOLD)
            .into_iter()
            .map(|(x, y, _)| (x, y, harris[(y * level.width + x) as usize]))
            .filter(|&(_, _, score)| score > 0.0)
            .collect();
        let cells_x = level.width.div_ceil(ORB_GRID_CELL).max(1);
        let cells_y = level.height.div_ceil(ORB_GRID_CELL).max(1);
        let per_cell = level_target.div_ceil((cells_x * cells_y) as usize).max(1);
        let mut selected = bucket_top_k(&candidates, ORB_GRID_CELL, cells_x, per_cell);
        selected.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        selected.truncate(level_target);
        for (x, y, response) in selected {
            let angle = intensity_centroid_angle(level, x as f32, y as f32, ORB_CENTROID_RADIUS);
            keypoints.push(Keypoint { x: x as f32, y: y as f32, octave: octave as u8, angle, response });
        }
    }
    keypoints
}

/// 🌄 Standalone Harris-only keypoint detector — the third `detector: harris` option promised by the plugin UI, independent of FAST: scores every pixel with [`harris_response`], keeps the positive-response ones, applies the same grid-bucketed top-k spatial spread as [`detect_orb_keypoints`], and assigns orientation via the intensity centroid, so the resulting [`Keypoint`]s (all at `octave: 0`, single-level) feed straight into [`describe_orb`]/[`match_brute`] unchanged.
/// <https://en.wikipedia.org/wiki/Harris_Corner_Detector>
pub fn detect_harris_keypoints(image: &ImageGray, target_count: usize) -> Vec<Keypoint> {
    if target_count == 0 || image.width == 0 || image.height == 0 {
        return Vec::new();
    }
    let harris = harris_response(image, ORB_HARRIS_K);
    let candidates: Vec<(u32, u32, f32)> = harris
        .iter()
        .enumerate()
        .map(|(idx, &score)| (idx as u32 % image.width, idx as u32 / image.width, score))
        .filter(|&(_, _, score)| score > 0.0)
        .collect();
    let cells_x = image.width.div_ceil(ORB_GRID_CELL).max(1);
    let cells_y = image.height.div_ceil(ORB_GRID_CELL).max(1);
    let per_cell = target_count.div_ceil((cells_x * cells_y) as usize).max(1);
    let mut selected = bucket_top_k(&candidates, ORB_GRID_CELL, cells_x, per_cell);
    selected.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    selected.truncate(target_count);
    selected
        .into_iter()
        .map(|(x, y, response)| {
            let angle = intensity_centroid_angle(image, x as f32, y as f32, ORB_CENTROID_RADIUS);
            Keypoint { x: x as f32, y: y as f32, octave: 0, angle, response }
        })
        .collect()
}
// #endregion 🔖Detect

// #region 🔖Describe
const BRIEF_PATCH_RADIUS: u32 = 15;
const BRIEF_SEED: u64 = 0xB817_ED0B_5D3E_C0DE;
const BRIEF_BLUR_SIGMA: f32 = 1.2;

/// 🧬 256-bit rotation-aware BRIEF descriptor packed into 4 `u64` words (bit `i` lives at word `i / 64`, position `i % 64`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Descriptor256(pub [u64; 4]);

impl Descriptor256 {
    /// 📏 Hamming distance to `other`, as an inherent method for callers that prefer `a.hamming_distance(&b)` over the free [`hamming`] function; both are the same population-count-of-XOR computation, so AKAZE M-LDB descriptors from [`describe_akaze`] (emitted into this same type) match through either.
    pub fn hamming_distance(&self, other: &Descriptor256) -> u32 {
        hamming(self, other)
    }
}

/// 📏 Hamming distance between two descriptors: population count of the XOR of each of the 4 words, summed.
/// <https://en.wikipedia.org/wiki/Hamming_distance>
pub fn hamming(a: &Descriptor256, b: &Descriptor256) -> u32 {
    a.0.iter().zip(b.0.iter()).map(|(&x, &y)| (x ^ y).count_ones()).sum()
}

type BriefOffsetPair = ((i32, i32), (i32, i32));

fn brief_pattern() -> &'static [BriefOffsetPair; 256] {
    static PATTERN: std::sync::OnceLock<[BriefOffsetPair; 256]> = std::sync::OnceLock::new();
    PATTERN.get_or_init(|| {
        let mut rng = mathematical_random::Rng::from_seed(BRIEF_SEED);
        let span = 2 * BRIEF_PATCH_RADIUS as u64 + 1;
        std::array::from_fn(|_| {
            let mut next_offset = || rng.next_range(0, span) as i32 - BRIEF_PATCH_RADIUS as i32;
            ((next_offset(), next_offset()), (next_offset(), next_offset()))
        })
    })
}

/// 🧬 Oriented rBRIEF description: for each keypoint, extracts a `(2 BRIEF_PATCH_RADIUS + 1)^2` patch steered by the keypoint's angle (rotation folded into the bilinear sampling grid of [`extract_patch`]), gaussian pre-blurs it to damp noise, then sets bit `i` when the blurred intensity at the pattern's first fixed point-pair offset is less than at its second. The 256 offset pairs are generated once from a fixed published seed via `mathematical_random`, so the pattern — and hence every descriptor — is identical across builds and runs.
/// <https://en.wikipedia.org/wiki/Oriented_FAST_and_rotated_BRIEF>
pub fn describe_orb(pyramid: &Pyramid, keypoints: &[Keypoint]) -> Vec<Descriptor256> {
    let pattern = brief_pattern();
    let side = 2 * BRIEF_PATCH_RADIUS + 1;
    keypoints
        .iter()
        .map(|kp| {
            let level_idx = (kp.octave as usize).min(pyramid.levels.len().saturating_sub(1));
            let level = &pyramid.levels[level_idx];
            let patch = extract_patch(level, kp.x, kp.y, BRIEF_PATCH_RADIUS, kp.angle);
            let patch_img = ImageGray { width: side, height: side, data: patch.data };
            let blurred = gaussian_blur(&patch_img, BRIEF_BLUR_SIGMA);
            let mut words = [0u64; 4];
            for (bit, &((ax, ay), (bx, by))) in pattern.iter().enumerate() {
                let sample_a = blurred.get((ax + BRIEF_PATCH_RADIUS as i32) as u32, (ay + BRIEF_PATCH_RADIUS as i32) as u32);
                let sample_b = blurred.get((bx + BRIEF_PATCH_RADIUS as i32) as u32, (by + BRIEF_PATCH_RADIUS as i32) as u32);
                if sample_a < sample_b {
                    words[bit / 64] |= 1u64 << (bit % 64);
                }
            }
            Descriptor256(words)
        })
        .collect()
}
// #endregion 🔖Describe

// #region 🔖Match
const MATCH_EPIPOLAR_RATIO: f32 = 0.75;
const MATCH_ZNCC_THRESHOLD: f32 = 0.6;
const MATCH_ZNCC_PATCH_RADIUS: u32 = 7;

/// 🔗 A correspondence between descriptor/keypoint index `a` (in the first set) and `b` (in the second set), with the Hamming (or Hamming-equivalent) distance that supported it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Match {
    pub a: u32,
    pub b: u32,
    pub distance: u32,
}

fn best_and_second_hamming(query: &Descriptor256, pool: &[Descriptor256]) -> (u32, u32, u32) {
    let mut best = (u32::MAX, u32::MAX);
    let mut second = u32::MAX;
    for (j, candidate) in pool.iter().enumerate() {
        let d = hamming(query, candidate);
        if d < best.0 {
            second = best.0;
            best = (d, j as u32);
        } else if d < second {
            second = d;
        }
    }
    (best.0, best.1, second)
}

/// 🤝 Brute-force Hamming matching from `desc_a` into `desc_b` with Lowe's ratio test (best distance must be less than `ratio` times the second-best), optionally cross-checked so a match only survives when it is also `desc_a`'s best match for its own best-in-`desc_b` partner.
/// <https://en.wikipedia.org/wiki/Nearest_neighbor_search#Nearest_neighbor_algorithms_in_high-dimensional_spaces>
pub fn match_brute(desc_a: &[Descriptor256], desc_b: &[Descriptor256], ratio: f32, mutual: bool) -> Vec<Match> {
    if desc_b.is_empty() {
        return Vec::new();
    }
    let mut matches = Vec::new();
    for (i, da) in desc_a.iter().enumerate() {
        let (best_dist, best_idx, second_dist) = best_and_second_hamming(da, desc_b);
        if second_dist == u32::MAX || (best_dist as f32) < ratio * (second_dist as f32) {
            matches.push(Match { a: i as u32, b: best_idx, distance: best_dist });
        }
    }
    if mutual {
        matches.retain(|m| {
            let (_, back_idx, _) = best_and_second_hamming(&desc_b[m.b as usize], desc_a);
            back_idx == m.a
        });
    }
    matches
}

fn epipolar_line_candidates(grid: &mathematical_spatial::Grid2, line: (f64, f64, f64), bounds: (f32, f32, f32, f32), step: f64) -> Vec<u32> {
    let (l0, l1, l2) = line;
    let (min_x, max_x, min_y, max_y) = bounds;
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    let mut collect_at = |p: [f64; 2]| {
        for id in grid.neighbors9(p) {
            if seen.insert(id) {
                out.push(id);
            }
        }
    };
    if l1.abs() >= l0.abs() {
        let mut x = f64::from(min_x);
        while x <= f64::from(max_x) {
            collect_at([x, -(l0 * x + l2) / l1]);
            x += step;
        }
    } else {
        let mut y = f64::from(min_y);
        while y <= f64::from(max_y) {
            collect_at([-(l1 * y + l2) / l0, y]);
            y += step;
        }
    }
    out
}

/// 🧭 Epipolar-guided matching: for each keypoint in `kp_a`, computes its epipolar line `l = F [x, y, 1]` in image B, walks that line (stepping through a [`mathematical_spatial::Grid2`] bucketed over `kp_b` for efficiency) to gather candidates within `band_px` of the line by point-to-line distance, then runs the same Lowe's-ratio matching as [`match_brute`] restricted to those candidates.
/// <https://en.wikipedia.org/wiki/Epipolar_geometry>
pub fn match_guided_epipolar(kp_a: &[Keypoint], desc_a: &[Descriptor256], kp_b: &[Keypoint], desc_b: &[Descriptor256], f_matrix: &[[f64; 3]; 3], band_px: f32) -> Vec<Match> {
    if kp_b.is_empty() {
        return Vec::new();
    }
    let cell = f64::from(band_px.max(1.0));
    let mut grid = mathematical_spatial::Grid2::new(cell);
    for (j, kp) in kp_b.iter().enumerate() {
        grid.insert([f64::from(kp.x), f64::from(kp.y)], j as u32);
    }
    let bounds = kp_b.iter().fold((f32::MAX, f32::MIN, f32::MAX, f32::MIN), |(lox, hix, loy, hiy), kp| (lox.min(kp.x), hix.max(kp.x), loy.min(kp.y), hiy.max(kp.y)));
    let mut matches = Vec::new();
    for (i, (kpa, da)) in kp_a.iter().zip(desc_a.iter()).enumerate() {
        let x = f64::from(kpa.x);
        let y = f64::from(kpa.y);
        let l0 = f_matrix[0][0] * x + f_matrix[0][1] * y + f_matrix[0][2];
        let l1 = f_matrix[1][0] * x + f_matrix[1][1] * y + f_matrix[1][2];
        let l2 = f_matrix[2][0] * x + f_matrix[2][1] * y + f_matrix[2][2];
        let norm = l0.hypot(l1);
        if norm < 1e-12 {
            continue;
        }
        let candidates = epipolar_line_candidates(&grid, (l0, l1, l2), bounds, cell);
        let mut best = (u32::MAX, u32::MAX);
        let mut second = u32::MAX;
        for j in candidates {
            let kpb = &kp_b[j as usize];
            let dist_line = (l0 * f64::from(kpb.x) + l1 * f64::from(kpb.y) + l2).abs() / norm;
            if dist_line > f64::from(band_px) {
                continue;
            }
            let d = hamming(da, &desc_b[j as usize]);
            if d < best.0 {
                second = best.0;
                best = (d, j);
            } else if d < second {
                second = d;
            }
        }
        if best.1 != u32::MAX && (second == u32::MAX || (best.0 as f32) < MATCH_EPIPOLAR_RATIO * (second as f32)) {
            matches.push(Match { a: i as u32, b: best.1, distance: best.0 });
        }
    }
    matches
}

/// 🩹 ZNCC patch-correlation fallback for pairs where binary descriptors fail (cross-sensor or low-texture imagery): for each keypoint in `kp_a`, gathers `kp_b` candidates within `search_radius` via a [`mathematical_spatial::Grid2`], scores each with [`zncc`] over patches from [`extract_patch`], and keeps the best candidate above a `0.6` correlation floor. The reported `distance` is `round((1 - zncc) * 1000)`, a monotone integer proxy so lower still means a better match.
pub fn match_zncc_fallback(img_a: &ImageGray, kp_a: &[Keypoint], img_b: &ImageGray, kp_b: &[Keypoint], search_radius: f32) -> Vec<Match> {
    if kp_b.is_empty() {
        return Vec::new();
    }
    let cell = f64::from(search_radius.max(1.0));
    let mut grid = mathematical_spatial::Grid2::new(cell);
    for (j, kp) in kp_b.iter().enumerate() {
        grid.insert([f64::from(kp.x), f64::from(kp.y)], j as u32);
    }
    let mut matches = Vec::new();
    for (i, kpa) in kp_a.iter().enumerate() {
        let patch_a = extract_patch(img_a, kpa.x, kpa.y, MATCH_ZNCC_PATCH_RADIUS, 0.0);
        let mut best_score = MATCH_ZNCC_THRESHOLD;
        let mut best_idx = u32::MAX;
        for j in grid.neighbors9([f64::from(kpa.x), f64::from(kpa.y)]) {
            let kpb = &kp_b[j as usize];
            if (kpb.x - kpa.x).hypot(kpb.y - kpa.y) > search_radius {
                continue;
            }
            let patch_b = extract_patch(img_b, kpb.x, kpb.y, MATCH_ZNCC_PATCH_RADIUS, 0.0);
            let score = zncc(&patch_a, &patch_b);
            if score > best_score {
                best_score = score;
                best_idx = j;
            }
        }
        if best_idx != u32::MAX {
            matches.push(Match { a: i as u32, b: best_idx, distance: ((1.0 - best_score) * 1000.0).round() as u32 });
        }
    }
    matches
}
// #endregion 🔖Match

// #region 🔖Flow
const KLT_SINGULAR_EPSILON: f32 = 1e-6;
const KLT_CONVERGE_EPS: f32 = 1e-4;

/// 🌊 One point's optical-flow track: subpixel position in the target frame, validity, and the final RMS intensity-residual error over its window.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrackPoint {
    pub x: f32,
    pub y: f32,
    pub valid: bool,
    pub error: f32,
}

fn klt_lucas_kanade_level(level_a: &ImageGray, level_b: &ImageGray, lx: f32, ly: f32, initial_disp: (f32, f32), window_radius: i32, max_iters: usize) -> ((f32, f32), bool, f32) {
    let r = window_radius;
    let mut window = Vec::with_capacity(((2 * r + 1) * (2 * r + 1)) as usize);
    for dy in -r..=r {
        for dx in -r..=r {
            let (fx, fy) = (dx as f32, dy as f32);
            let t = level_a.sample(lx + fx, ly + fy);
            let gx = (level_a.sample(lx + fx + 1.0, ly + fy) - level_a.sample(lx + fx - 1.0, ly + fy)) * 0.5;
            let gy = (level_a.sample(lx + fx, ly + fy + 1.0) - level_a.sample(lx + fx, ly + fy - 1.0)) * 0.5;
            window.push((fx, fy, t, gx, gy));
        }
    }
    let mut sxx = 0.0f32;
    let mut sxy = 0.0f32;
    let mut syy = 0.0f32;
    for &(_, _, _, gx, gy) in &window {
        sxx += gx * gx;
        sxy += gx * gy;
        syy += gy * gy;
    }
    let det = sxx * syy - sxy * sxy;
    if det.abs() < KLT_SINGULAR_EPSILON {
        return (initial_disp, false, f32::INFINITY);
    }
    let mut disp = initial_disp;
    let mut error = f32::INFINITY;
    for _ in 0..max_iters {
        let mut bx = 0.0f32;
        let mut by = 0.0f32;
        let mut sum_sq = 0.0f32;
        for &(fx, fy, t, gx, gy) in &window {
            let sample_x = lx + disp.0 + fx;
            let sample_y = ly + disp.1 + fy;
            if sample_x < 0.0 || sample_y < 0.0 || sample_x > (level_b.width - 1) as f32 || sample_y > (level_b.height - 1) as f32 {
                return (disp, false, f32::INFINITY);
            }
            let residual = level_b.sample(sample_x, sample_y) - t;
            bx += gx * residual;
            by += gy * residual;
            sum_sq += residual * residual;
        }
        let delta_x = (sxy * by - syy * bx) / det;
        let delta_y = (sxy * bx - sxx * by) / det;
        disp.0 += delta_x;
        disp.1 += delta_y;
        error = (sum_sq / window.len() as f32).sqrt();
        if delta_x.hypot(delta_y) < KLT_CONVERGE_EPS {
            break;
        }
    }
    (disp, true, error)
}

fn klt_track_single(pyr_a: &Pyramid, pyr_b: &Pyramid, x0: f32, y0: f32, window_radius: i32, max_iters: usize) -> TrackPoint {
    let n_levels = pyr_a.levels.len().min(pyr_b.levels.len());
    if n_levels == 0 {
        return TrackPoint { x: x0, y: y0, valid: false, error: f32::INFINITY };
    }
    let mut disp = (0.0f32, 0.0f32);
    let mut ok = true;
    let mut error = f32::INFINITY;
    for level in (0..n_levels).rev() {
        let scale = pyr_a.scale.powi(level as i32);
        let level_a = &pyr_a.levels[level];
        let level_b = &pyr_b.levels[level];
        let (level_disp, level_ok, level_error) = klt_lucas_kanade_level(level_a, level_b, x0 * scale, y0 * scale, disp, window_radius, max_iters);
        disp = level_disp;
        ok = level_ok;
        error = level_error;
        if level > 0 {
            disp = (disp.0 / pyr_a.scale, disp.1 / pyr_a.scale);
        }
    }
    let (fx, fy) = (x0 + disp.0, y0 + disp.1);
    let finest = &pyr_a.levels[0];
    let in_bounds = fx >= 0.0 && fy >= 0.0 && fx < finest.width as f32 && fy < finest.height as f32;
    TrackPoint { x: fx, y: fy, valid: ok && in_bounds, error }
}

/// 🌊 Pyramidal Lucas-Kanade tracking of `points` (given in `pyr_a`'s level-0 coordinates) from `pyr_a` into `pyr_b`: coarse-to-fine over shared pyramid levels, each level running Gauss-Newton on a 2-DoF translation using the template's gradient structure tensor (an inverse-compositional simplification, since a pure translation warp has a constant Jacobian) and the image-difference residual over a `(2 window_radius + 1)^2` window. A point is marked invalid if its structure tensor is near-singular, a sample step leaves the image bounds, or the final position lands outside the level-0 image.
/// <https://en.wikipedia.org/wiki/Lucas%E2%80%93Kanade_method>
pub fn klt_track(pyr_a: &Pyramid, pyr_b: &Pyramid, points: &[(f32, f32)], window_radius: i32, max_iters: usize) -> Vec<TrackPoint> {
    points.iter().map(|&(x0, y0)| klt_track_single(pyr_a, pyr_b, x0, y0, window_radius, max_iters)).collect()
}

/// ↩️ Forward-backward consistency pruning: re-tracks each already-valid `tracked` point from `pyr_b` back into `pyr_a` and invalidates it (in place) if the backward track itself fails or its round-trip distance to the original `points` entry exceeds `max_fb_error`.
/// <https://en.wikipedia.org/wiki/Lucas%E2%80%93Kanade_method>
pub fn forward_backward_prune(pyr_a: &Pyramid, pyr_b: &Pyramid, points: &[(f32, f32)], tracked: &mut [TrackPoint], window_radius: i32, max_iters: usize, max_fb_error: f32) {
    for (tp, &(ox, oy)) in tracked.iter_mut().zip(points.iter()) {
        if !tp.valid {
            continue;
        }
        let back = klt_track_single(pyr_b, pyr_a, tp.x, tp.y, window_radius, max_iters);
        let fb_error = (back.x - ox).hypot(back.y - oy);
        if !back.valid || fb_error > max_fb_error {
            tp.valid = false;
        }
    }
}
// #endregion 🔖Flow

// #region 🔖ScaleSpace
const AKAZE_BASE_SIGMA: f32 = 1.6;
const AKAZE_FED_TAU_MAX: f32 = 0.25;
const AKAZE_CONTRAST_PERCENTILE: f32 = 0.7;

/// 🌀 One evolved level of the nonlinear diffusion scale space: the diffused image, its octave/sublevel indices, and its effective smoothing scale `esigma` expressed in ORIGINAL-image pixel units (`esigma = base_sigma * 2^(octave + sublevel / sublevels)`).
#[derive(Clone, Debug, PartialEq)]
pub struct ScaleLevel {
    pub image: ImageGray,
    pub octave: u8,
    pub sublevel: u8,
    pub esigma: f32,
}

/// 🌀 A full AKAZE nonlinear diffusion scale space: evolution levels in `(octave, sublevel)` order, alongside the base image's dimensions (any level's coordinates convert to base-image pixels by multiplying by `2^octave`).
#[derive(Clone, Debug, PartialEq)]
pub struct ScaleSpace {
    pub levels: Vec<ScaleLevel>,
    pub base_width: u32,
    pub base_height: u32,
}

/// 📊 Perona-Malik contrast factor `k`: the 70th-percentile gradient magnitude of a `sigma=1` gaussian-presmoothed image — the standard AKAZE contrast-factor estimation procedure that keeps the diffusion's edge-stopping behaviour scaled to the image's own contrast.
/// <https://en.wikipedia.org/wiki/Anisotropic_diffusion>
fn estimate_contrast_factor(img: &ImageGray) -> f32 {
    let smoothed = gaussian_blur(img, 1.0);
    let g = scharr_gradients(&smoothed);
    let mut magnitudes: Vec<f32> = g.gx.iter().zip(g.gy.iter()).map(|(&gx, &gy)| gx.hypot(gy)).collect();
    if magnitudes.is_empty() {
        return 1e-3;
    }
    magnitudes.sort_by(f32::total_cmp);
    let idx = (((magnitudes.len() - 1) as f32) * AKAZE_CONTRAST_PERCENTILE).round() as usize;
    magnitudes[idx.min(magnitudes.len() - 1)].max(1e-6)
}

/// 🌊 Perona-Malik `g2` conductivity `1 / (1 + (|∇L| / k)^2)` per pixel: decays toward `0` across strong edges (preserving them) and stays near `1` in flat regions (smoothing them freely).
/// <https://en.wikipedia.org/wiki/Anisotropic_diffusion>
fn perona_malik_g2(gradient: &GradientField, k: f32) -> Vec<f32> {
    let k_sq = (k * k).max(1e-12);
    gradient.gx.iter().zip(gradient.gy.iter()).map(|(&gx, &gy)| 1.0 / (1.0 + (gx * gx + gy * gy) / k_sq)).collect()
}

/// ⏱️ Fast Explicit Diffusion step schedule reaching total evolution time `t` in one stable cycle: `n = ceil(sqrt(3t/tau_max + 1/4) - 1/2)` steps with `tau_i = tau_max / (2 cos(pi (2i+1) / (4n+2)))` — the FED scheme AKAZE/KAZE use to take far larger stable steps than plain explicit diffusion.
/// <https://en.wikipedia.org/wiki/Anisotropic_diffusion>
fn fed_tau_schedule(t: f32, tau_max: f32) -> Vec<f32> {
    if t <= 0.0 {
        return Vec::new();
    }
    let n = ((3.0 * t / tau_max + 0.25).sqrt() - 0.5).ceil().max(1.0) as usize;
    let n_f = n as f32;
    (0..n).map(|i| tau_max / (2.0 * (std::f32::consts::PI * (2.0 * i as f32 + 1.0) / (4.0 * n_f + 2.0)).cos())).collect()
}

/// 🧮 One explicit nonlinear-diffusion step: for every pixel, sums flux across its 4-neighbourhood weighted by the average conductivity between center and neighbour, `L' = L + tau * div(g grad L)`, with clamped (zero-flux) borders.
/// <https://en.wikipedia.org/wiki/Anisotropic_diffusion>
fn diffusion_step(l: &ImageGray, g: &[f32], tau: f32) -> ImageGray {
    let (w, h) = (l.width, l.height);
    let mut out = l.clone();
    if w == 0 || h == 0 {
        return out;
    }
    let at = |x: i64, y: i64| -> (f32, f32) {
        let cx = x.clamp(0, i64::from(w) - 1) as u32;
        let cy = y.clamp(0, i64::from(h) - 1) as u32;
        let idx = (cy * w + cx) as usize;
        (l.data[idx], g[idx])
    };
    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) as usize;
            let (lc, gc) = (l.data[idx], g[idx]);
            let (le, ge) = at(i64::from(x) + 1, i64::from(y));
            let (lw, gw) = at(i64::from(x) - 1, i64::from(y));
            let (ls, gs) = at(i64::from(x), i64::from(y) + 1);
            let (ln, gn) = at(i64::from(x), i64::from(y) - 1);
            let flux = 0.5 * (gc + ge) * (le - lc) + 0.5 * (gc + gw) * (lw - lc) + 0.5 * (gc + gs) * (ls - lc) + 0.5 * (gc + gn) * (ln - lc);
            out.data[idx] = lc + tau * flux;
        }
    }
    out
}

/// 🌀 Builds a full AKAZE nonlinear diffusion scale space over `octaves` octaves of `sublevels` sublevels each: per octave, starts from [`build_pyramid`]'s anti-aliased downsample of `image` (sublevel 0, not yet diffused), estimates a per-octave Perona-Malik contrast factor once via [`estimate_contrast_factor`] on that octave's base image, then for each further sublevel recomputes conductivity from the currently-evolved image's [`scharr_gradients`] and walks a [`fed_tau_schedule`] of explicit [`diffusion_step`]s covering the incremental evolution time `0.5 sigma^2` between consecutive sublevel scales `sigma = base_sigma * 2^(sublevel / sublevels)`.
/// <https://en.wikipedia.org/wiki/Anisotropic_diffusion>
pub fn build_akaze_scale_space(image: &ImageGray, octaves: u8, sublevels: u8) -> ScaleSpace {
    let octaves = octaves.max(1);
    let sublevels = sublevels.max(1);
    let pyramid = build_pyramid(image, usize::from(octaves));
    let mut levels = Vec::new();
    for (o, octave_base) in pyramid.levels.iter().enumerate() {
        if octave_base.width == 0 || octave_base.height == 0 {
            continue;
        }
        let ratio = 2f32.powi(o as i32);
        let k = estimate_contrast_factor(octave_base);
        let mut current = octave_base.clone();
        let mut prev_t = 0.5 * AKAZE_BASE_SIGMA * AKAZE_BASE_SIGMA;
        levels.push(ScaleLevel { image: current.clone(), octave: o as u8, sublevel: 0, esigma: AKAZE_BASE_SIGMA * ratio });
        for s in 1..sublevels {
            let sigma_local = AKAZE_BASE_SIGMA * 2f32.powf(f32::from(s) / f32::from(sublevels));
            let t_now = 0.5 * sigma_local * sigma_local;
            let dt = (t_now - prev_t).max(0.0);
            let gradient = scharr_gradients(&current);
            let conductivity = perona_malik_g2(&gradient, k);
            for tau in fed_tau_schedule(dt, AKAZE_FED_TAU_MAX) {
                current = diffusion_step(&current, &conductivity, tau);
            }
            levels.push(ScaleLevel { image: current.clone(), octave: o as u8, sublevel: s, esigma: sigma_local * ratio });
            prev_t = t_now;
        }
    }
    ScaleSpace { levels, base_width: image.width, base_height: image.height }
}
// #endregion 🔖ScaleSpace

// #region 🔖Akaze
const MLDB_GRID: u32 = 4;
const MLDB_CELLS: usize = (MLDB_GRID * MLDB_GRID) as usize;
const MLDB_VALUES: usize = MLDB_CELLS * 3;
const MLDB_PATCH_RADIUS: u32 = 8;
const MLDB_SEED: u64 = 0xA11A_2EDE_5C81_B70F;
const AKAZE_RESPONSE_EPS: f32 = 1e-9;
const AKAZE_BORDER_MARGIN: u32 = 2;

/// 🧮 Scale-normalized Hessian determinant `sigma_local^4 (L_xx L_yy - L_xy^2)` at every pixel of `image`, with second derivatives from two passes of [`scharr_gradients`] (gradient-of-the-gradient) — the standard blob-strength measure driving AKAZE's scale-space extrema search.
/// <https://en.wikipedia.org/wiki/Blob_detection#The_determinant_of_the_Hessian>
fn hessian_determinant_response(image: &ImageGray, sigma_local: f32) -> Vec<f32> {
    let g1 = scharr_gradients(image);
    let gx_img = ImageGray { width: image.width, height: image.height, data: g1.gx };
    let gy_img = ImageGray { width: image.width, height: image.height, data: g1.gy };
    let g2x = scharr_gradients(&gx_img);
    let g2y = scharr_gradients(&gy_img);
    let norm = sigma_local.powi(4).max(1e-12);
    g2x.gx
        .iter()
        .zip(g2y.gy.iter())
        .zip(g2x.gy.iter().zip(g2y.gx.iter()))
        .map(|((&lxx, &lyy), (&lxy_a, &lxy_b))| {
            let lxy = 0.5 * (lxy_a + lxy_b);
            (lxx * lyy - lxy * lxy) * norm
        })
        .collect()
}

/// 📐 Sub-pixel offset from a local 2D quadratic (Taylor) fit of a response map around integer pixel `(x, y)`: solves the Newton step `H offset = -grad` from central-difference first/second derivatives, clamped to `[-0.5, 0.5]` per axis; a near-singular Hessian yields a zero offset (keep the integer position).
fn quadratic_refine_2d(response: &[f32], width: u32, x: u32, y: u32) -> (f32, f32) {
    let at = |dx: i32, dy: i32| response[((y as i32 + dy) as u32 * width + (x as i32 + dx) as u32) as usize];
    let center = at(0, 0);
    let dx = (at(1, 0) - at(-1, 0)) * 0.5;
    let dy = (at(0, 1) - at(0, -1)) * 0.5;
    let dxx = at(1, 0) - 2.0 * center + at(-1, 0);
    let dyy = at(0, 1) - 2.0 * center + at(0, -1);
    let dxy = (at(1, 1) - at(1, -1) - at(-1, 1) + at(-1, -1)) * 0.25;
    let det = dxx * dyy - dxy * dxy;
    if det.abs() < 1e-9 {
        return (0.0, 0.0);
    }
    let ox = -(dyy * dx - dxy * dy) / det;
    let oy = -(dxx * dy - dxy * dx) / det;
    (ox.clamp(-0.5, 0.5), oy.clamp(-0.5, 0.5))
}

struct AkazeCandidate {
    x: f32,
    y: f32,
    response: f32,
    level: u32,
}

fn akaze_grid_top_k(candidates: Vec<AkazeCandidate>, image_w: u32, image_h: u32, cell: u32, target_count: usize) -> Vec<AkazeCandidate> {
    if candidates.is_empty() || target_count == 0 {
        return Vec::new();
    }
    let cells_x = image_w.div_ceil(cell).max(1);
    let cells_y = image_h.div_ceil(cell).max(1);
    let per_cell = target_count.div_ceil((cells_x * cells_y) as usize).max(1);
    let mut buckets: std::collections::HashMap<u32, Vec<AkazeCandidate>> = std::collections::HashMap::new();
    for c in candidates {
        let bx = ((c.x as u32) / cell).min(cells_x - 1);
        let by = ((c.y as u32) / cell).min(cells_y - 1);
        buckets.entry(by * cells_x + bx).or_default().push(c);
    }
    let mut keys: Vec<u32> = buckets.keys().copied().collect();
    keys.sort_unstable();
    let mut out = Vec::new();
    for key in keys {
        let mut bucket = buckets.remove(&key).expect("key was collected from buckets.keys()");
        bucket.sort_by(|a, b| b.response.partial_cmp(&a.response).unwrap_or(std::cmp::Ordering::Equal));
        bucket.truncate(per_cell);
        out.extend(bucket);
    }
    out.sort_by(|a, b| b.response.partial_cmp(&a.response).unwrap_or(std::cmp::Ordering::Equal));
    out.truncate(target_count);
    out
}

/// 🎯 Hessian-determinant blob detector over an AKAZE [`ScaleSpace`]: scores every level (every octave × every sublevel) with [`hessian_determinant_response`], keeps pixels that are a strict spatial local maximum in their level's own `3x3` neighbourhood and positive, sub-pixel refines the `(x, y)` position via [`quadratic_refine_2d`], rescales to base-image coordinates, and assigns an intensity-centroid orientation from the owning level's diffused image. Candidates from every level are then pooled and passed through the same grid-bucketed top-k spatial spread as [`detect_orb_keypoints`] (which, since a genuine blob typically survives as a local max across several neighbouring sublevels, also acts as the cross-scale duplicate suppression). Each returned [`Keypoint`]'s `octave` field is the flat index into `scale_space.levels` it was detected at (not a true octave number), so [`describe_akaze`] can look the owning [`ScaleLevel`] back up directly.
/// <https://en.wikipedia.org/wiki/Blob_detection>
pub fn detect_akaze_keypoints(scale_space: &ScaleSpace, target_count: usize) -> Vec<Keypoint> {
    if scale_space.levels.is_empty() || target_count == 0 {
        return Vec::new();
    }
    let mut candidates = Vec::new();
    for (li, level) in scale_space.levels.iter().enumerate() {
        let ratio = 2f32.powi(level.octave as i32);
        let (w, h) = (level.image.width, level.image.height);
        if w <= 2 * AKAZE_BORDER_MARGIN + 2 || h <= 2 * AKAZE_BORDER_MARGIN + 2 {
            continue;
        }
        let response = hessian_determinant_response(&level.image, level.esigma / ratio);
        for y in (AKAZE_BORDER_MARGIN + 1)..(h - AKAZE_BORDER_MARGIN - 1) {
            for x in (AKAZE_BORDER_MARGIN + 1)..(w - AKAZE_BORDER_MARGIN - 1) {
                let idx = (y * w + x) as usize;
                let value = response[idx];
                if value <= AKAZE_RESPONSE_EPS {
                    continue;
                }
                let mut is_max = true;
                'window: for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nidx = ((y as i32 + dy) as u32 * w + (x as i32 + dx) as u32) as usize;
                        if response[nidx] > value {
                            is_max = false;
                            break 'window;
                        }
                    }
                }
                if !is_max {
                    continue;
                }
                let (ox, oy) = quadratic_refine_2d(&response, w, x, y);
                let (lx, ly) = (x as f32 + ox, y as f32 + oy);
                candidates.push(AkazeCandidate { x: lx * ratio, y: ly * ratio, response: value, level: li as u32 });
            }
        }
    }
    let selected = akaze_grid_top_k(candidates, scale_space.base_width, scale_space.base_height, ORB_GRID_CELL, target_count);
    selected
        .into_iter()
        .map(|c| {
            let level = &scale_space.levels[c.level as usize];
            let ratio = 2f32.powi(level.octave as i32);
            let angle = intensity_centroid_angle(&level.image, c.x / ratio, c.y / ratio, ORB_CENTROID_RADIUS);
            Keypoint { x: c.x, y: c.y, octave: c.level.min(u32::from(u8::MAX)) as u8, angle, response: c.response }
        })
        .collect()
}

fn mldb_pattern() -> &'static [(u8, u8); 256] {
    static PATTERN: std::sync::OnceLock<[(u8, u8); 256]> = std::sync::OnceLock::new();
    PATTERN.get_or_init(|| {
        let mut rng = mathematical_random::Rng::from_seed(MLDB_SEED);
        std::array::from_fn(|_| {
            let a = rng.next_range(0, MLDB_VALUES as u64) as u8;
            let mut b = rng.next_range(0, MLDB_VALUES as u64) as u8;
            while b == a {
                b = rng.next_range(0, MLDB_VALUES as u64) as u8;
            }
            (a, b)
        })
    })
}

/// 🧬 M-LDB (Modified Local Difference Binary) description: for each keypoint, extracts a `(2 MLDB_PATCH_RADIUS + 1)^2` patch from its owning [`ScaleLevel`] (looked up via `Keypoint::octave` as a flat scale-space index, per [`detect_akaze_keypoints`]), steered by the keypoint's angle via [`extract_patch`], splits it into a `4x4` grid of sub-cells and averages 3 channels per cell — mean intensity and the two mean [`scharr_gradients`] components — into 48 scalar values, then sets bit `i` from a fixed published-seed pattern of 256 value-index pairs (generated once via `mathematical_random`, so identical across builds and runs) whenever the pattern's first value is less than its second. Emits into the same [`Descriptor256`] the rBRIEF path produces, so [`match_brute`]/[`hamming`] work unchanged on AKAZE descriptors.
/// <https://en.wikipedia.org/wiki/AKAZE>
pub fn describe_akaze(scale_space: &ScaleSpace, keypoints: &[Keypoint]) -> Vec<Descriptor256> {
    let pattern = mldb_pattern();
    let side = 2 * MLDB_PATCH_RADIUS + 1;
    let cell = (side / MLDB_GRID).max(1);
    keypoints
        .iter()
        .map(|kp| {
            let level_idx = (kp.octave as usize).min(scale_space.levels.len().saturating_sub(1));
            let level = &scale_space.levels[level_idx];
            let ratio = 2f32.powi(level.octave as i32);
            let (lx, ly) = (kp.x / ratio, kp.y / ratio);
            let patch = extract_patch(&level.image, lx, ly, MLDB_PATCH_RADIUS, kp.angle);
            let patch_img = ImageGray { width: side, height: side, data: patch.data };
            let grad = scharr_gradients(&patch_img);
            let mut values = [0f32; MLDB_VALUES];
            for gy in 0..MLDB_GRID {
                for gx in 0..MLDB_GRID {
                    let x0 = gx * cell;
                    let x1 = if gx + 1 == MLDB_GRID { side } else { (gx + 1) * cell };
                    let y0 = gy * cell;
                    let y1 = if gy + 1 == MLDB_GRID { side } else { (gy + 1) * cell };
                    let (mut sum_i, mut sum_dx, mut sum_dy, mut n) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
                    for y in y0..y1 {
                        for x in x0..x1 {
                            let idx = (y * side + x) as usize;
                            sum_i += patch_img.data[idx];
                            sum_dx += grad.gx[idx];
                            sum_dy += grad.gy[idx];
                            n += 1.0;
                        }
                    }
                    let cell_idx = (gy * MLDB_GRID + gx) as usize;
                    if n > 0.0 {
                        values[cell_idx] = sum_i / n;
                        values[MLDB_CELLS + cell_idx] = sum_dx / n;
                        values[2 * MLDB_CELLS + cell_idx] = sum_dy / n;
                    }
                }
            }
            let mut words = [0u64; 4];
            for (bit, &(a, b)) in pattern.iter().enumerate() {
                if values[a as usize] < values[b as usize] {
                    words[bit / 64] |= 1u64 << (bit % 64);
                }
            }
            Descriptor256(words)
        })
        .collect()
}
// #endregion 🔖Akaze

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use remodel_image::{build_pyramid, warp_affine};

    // #region 🔖Fixtures
    fn corner_image(size: u32) -> ImageGray {
        let mut img = ImageGray::new(size, size);
        for y in 0..size {
            for x in 0..size {
                let half = size / 2;
                img.set(x, y, if x < half && y < half { 0.9 } else { 0.1 });
            }
        }
        img
    }

    fn textured_image(size: u32) -> ImageGray {
        let mut img = ImageGray::new(size, size);
        for v in img.data.iter_mut() {
            *v = 0.1;
        }
        let (step, square) = (8u32, 4u32);
        let mut y = 2;
        while y + square <= size {
            let mut x = 2;
            while x + square <= size {
                for dy in 0..square {
                    for dx in 0..square {
                        img.set(x + dx, y + dy, 0.9);
                    }
                }
                x += step;
            }
            y += step;
        }
        img
    }

    fn lcg_next(state: &mut u32) -> f32 {
        *state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (*state >> 8) as f32 / 16_777_216.0
    }

    fn lcg_texture(size: u32, seed: u32) -> ImageGray {
        let mut img = ImageGray::new(size, size);
        let mut state = seed;
        for v in img.data.iter_mut() {
            *v = lcg_next(&mut state);
        }
        img
    }

    fn smooth_texture(size: u32) -> ImageGray {
        let mut img = ImageGray::new(size, size);
        for y in 0..size {
            for x in 0..size {
                let (fx, fy) = (x as f32, y as f32);
                let v = 0.5 + 0.3 * (fx * 0.15).sin() * (fy * 0.12).cos() + 0.15 * (fx * 0.05 + fy * 0.07).sin();
                img.set(x, y, v);
            }
        }
        img
    }
    // #endregion 🔖Fixtures

    // #region 🔖DetectTests
    #[test]
    fn fast_corners_detects_planted_corner_not_flat_region() {
        let img = corner_image(48);
        let corners = fast_corners(&img, 0.2);
        assert!(!corners.is_empty(), "expected at least one detected corner");
        assert!(corners.iter().any(|&(x, y, _)| (x as i32 - 24).abs() <= 3 && (y as i32 - 24).abs() <= 3), "expected a corner near the planted L-corner at (24, 24)");
        assert!(!corners.iter().any(|&(x, y, _)| x <= 6 && y <= 6), "flat interior pixels should not be reported as corners");
        let flat = ImageGray::new(48, 48);
        assert!(fast_corners(&flat, 0.2).is_empty(), "a flat image should have no corners");
    }

    #[test]
    fn harris_response_is_low_on_flat_and_high_on_corner() {
        let mut flat = ImageGray::new(32, 32);
        for v in flat.data.iter_mut() {
            *v = 0.5;
        }
        let corner = corner_image(32);
        let flat_resp = harris_response(&flat, 0.04);
        let corner_resp = harris_response(&corner, 0.04);
        let max_flat = flat_resp.iter().copied().fold(f32::MIN, f32::max);
        let max_corner = corner_resp.iter().copied().fold(f32::MIN, f32::max);
        assert!(max_flat.abs() < 1e-6, "flat image should have ~0 harris response, got {max_flat}");
        assert!(max_corner > 1e-4, "corner response {max_corner} should be clearly positive");
        assert!(max_corner > max_flat * 10.0 + 1e-4, "corner response {max_corner} should exceed flat response {max_flat}");
    }

    #[test]
    fn detect_orb_keypoints_returns_roughly_target_count_and_spread() {
        let img = textured_image(64);
        let pyramid = build_pyramid(&img, 3);
        let target = 40usize;
        let keypoints = detect_orb_keypoints(&pyramid, target);
        assert!(!keypoints.is_empty(), "expected some keypoints on a textured image");
        assert!(keypoints.len() as f64 <= target as f64 * 1.5, "too many keypoints: {}", keypoints.len());
        assert!(keypoints.len() as f64 >= target as f64 * 0.3, "too few keypoints: {}", keypoints.len());
        let min_x = keypoints.iter().map(|k| k.x).fold(f32::MAX, f32::min);
        let max_x = keypoints.iter().map(|k| k.x).fold(f32::MIN, f32::max);
        let min_y = keypoints.iter().map(|k| k.y).fold(f32::MAX, f32::min);
        let max_y = keypoints.iter().map(|k| k.y).fold(f32::MIN, f32::max);
        assert!(max_x - min_x > 20.0, "keypoints should spread across x, got span {}", max_x - min_x);
        assert!(max_y - min_y > 20.0, "keypoints should spread across y, got span {}", max_y - min_y);
    }

    #[test]
    fn detect_harris_keypoints_clusters_near_known_corner() {
        let img = corner_image(48);
        let keypoints = detect_harris_keypoints(&img, 30);
        assert!(!keypoints.is_empty(), "expected some harris keypoints on a corner image");
        assert!(
            keypoints.iter().any(|kp| (kp.x - 24.0).abs() <= 4.0 && (kp.y - 24.0).abs() <= 4.0),
            "expected a harris keypoint near the planted L-corner at (24, 24), got {:?}",
            keypoints.iter().map(|kp| (kp.x, kp.y)).collect::<Vec<_>>()
        );
        let flat = ImageGray::new(48, 48);
        assert!(detect_harris_keypoints(&flat, 30).is_empty(), "a flat image should have no harris keypoints");
    }
    // #endregion 🔖DetectTests

    // #region 🔖DescribeTests
    #[test]
    fn describe_orb_is_deterministic_and_self_hamming_zero() {
        let img = textured_image(48);
        let pyramid = build_pyramid(&img, 2);
        let kp = Keypoint { x: 24.0, y: 24.0, octave: 0, angle: 0.4, response: 1.0 };
        let d1 = describe_orb(&pyramid, &[kp]);
        let d2 = describe_orb(&pyramid, &[kp]);
        assert_eq!(d1[0], d2[0], "describing the same keypoint twice should give an identical descriptor");
        assert_eq!(hamming(&d1[0], &d1[0]), 0);
    }
    // #endregion 🔖DescribeTests

    // #region 🔖MatchTests
    #[test]
    fn match_brute_recovers_known_translation_correspondences() {
        let size = 72u32;
        let img_a = lcg_texture(size, 123);
        let (tx, ty) = (3.0f32, 2.0f32);
        let m = [[1.0, 0.0, -tx], [0.0, 1.0, -ty]];
        let img_b = warp_affine(&img_a, &m, size, size);
        let pyr_a = build_pyramid(&img_a, 1);
        let pyr_b = build_pyramid(&img_b, 1);
        let kp_a = detect_orb_keypoints(&pyr_a, 80);
        let kp_b = detect_orb_keypoints(&pyr_b, 80);
        let desc_a = describe_orb(&pyr_a, &kp_a);
        let desc_b = describe_orb(&pyr_b, &kp_b);
        let matches = match_brute(&desc_a, &desc_b, 0.85, true);
        assert!(!matches.is_empty(), "expected some matches between the translated pair");
        let margin = 12.0f32;
        let mut checked = 0u32;
        let mut correct = 0u32;
        for mat in &matches {
            let a = kp_a[mat.a as usize];
            let b = kp_b[mat.b as usize];
            if a.x < margin || a.y < margin || a.x > size as f32 - margin || a.y > size as f32 - margin {
                continue;
            }
            checked += 1;
            if (b.x - a.x - tx).abs() < 2.0 && (b.y - a.y - ty).abs() < 2.0 {
                correct += 1;
            }
        }
        assert!(checked > 0, "expected some interior matches to check");
        assert!(f64::from(correct) / f64::from(checked) >= 0.9, "expected at least 90% correct correspondences, got {correct}/{checked}");
    }
    // #endregion 🔖MatchTests

    // #region 🔖FlowTests
    #[test]
    fn klt_track_recovers_known_shift_and_flags_out_of_bounds() {
        let size = 48u32;
        let img_a = smooth_texture(size);
        let shift = 2.3f32;
        let m = [[1.0, 0.0, -shift], [0.0, 1.0, -shift]];
        let img_b = warp_affine(&img_a, &m, size, size);
        let pyr_a = build_pyramid(&img_a, 3);
        let pyr_b = build_pyramid(&img_b, 3);
        let points = [(12.0, 12.0), (24.0, 20.0), (30.0, 30.0), (46.0, 46.0)];
        let tracked = klt_track(&pyr_a, &pyr_b, &points, 5, 20);
        for (i, tp) in tracked.iter().enumerate().take(3) {
            assert!(tp.valid, "point {i} should track validly");
            assert!((tp.x - (points[i].0 + shift)).abs() < 0.1, "x error too high for point {i}: {}", tp.x);
            assert!((tp.y - (points[i].1 + shift)).abs() < 0.1, "y error too high for point {i}: {}", tp.y);
        }
        assert!(!tracked[3].valid, "a point leaving the image bounds after translation should be invalid");
    }

    #[test]
    fn forward_backward_prune_invalidates_degenerate_track_and_keeps_good_one() {
        let size = 48u32;
        let img_a = smooth_texture(size);
        let shift = 1.5f32;
        let m = [[1.0, 0.0, -shift], [0.0, 1.0, -shift]];
        let img_b = warp_affine(&img_a, &m, size, size);
        let pyr_a = build_pyramid(&img_a, 3);
        let pyr_b = build_pyramid(&img_b, 3);
        let points = [(20.0, 20.0), (5.0, 5.0)];
        let mut tracked = klt_track(&pyr_a, &pyr_b, &points, 5, 20);
        assert!(tracked[0].valid, "well-textured point should track validly first");
        tracked[1] = TrackPoint { x: 5.0, y: 40.0, valid: true, error: 0.0 };
        forward_backward_prune(&pyr_a, &pyr_b, &points, &mut tracked, 5, 20, 0.5);
        assert!(tracked[0].valid, "well-tracked point should remain valid after fb-pruning");
        assert!(!tracked[1].valid, "an implausible/degenerate track should be invalidated by the fb round-trip check");
    }
    // #endregion 🔖FlowTests

    // #region 🔖AkazeTests
    fn blob_image(size: u32) -> ImageGray {
        let mut img = ImageGray::new(size, size);
        for v in img.data.iter_mut() {
            *v = 0.1;
        }
        let (step, square) = (16u32, 8u32);
        let mut y = 4;
        while y + square <= size {
            let mut x = 4;
            while x + square <= size {
                for dy in 0..square {
                    for dx in 0..square {
                        img.set(x + dx, y + dy, 0.9);
                    }
                }
                x += step;
            }
            y += step;
        }
        gaussian_blur(&img, 1.5)
    }

    struct TestTransform {
        cx: f32,
        cy: f32,
        theta: f32,
        scale: f32,
    }

    fn rotate_scale_matrix(t: &TestTransform) -> [[f32; 3]; 2] {
        let (sin_t, cos_t) = t.theta.sin_cos();
        let inv_s = 1.0 / t.scale;
        [
            [cos_t * inv_s, sin_t * inv_s, t.cx - t.cx * cos_t * inv_s - t.cy * sin_t * inv_s],
            [-sin_t * inv_s, cos_t * inv_s, t.cy + t.cx * sin_t * inv_s - t.cy * cos_t * inv_s],
        ]
    }

    fn transform_point(t: &TestTransform, x: f32, y: f32) -> (f32, f32) {
        let (sin_t, cos_t) = t.theta.sin_cos();
        let (dx, dy) = (x - t.cx, y - t.cy);
        (t.cx + t.scale * (cos_t * dx - sin_t * dy), t.cy + t.scale * (sin_t * dx + cos_t * dy))
    }

    fn repeatability_fraction(base: &[Keypoint], transformed: &[Keypoint], t: &TestTransform, size: u32, margin: f32, tolerance: f32) -> f64 {
        let mut checked = 0u32;
        let mut matched = 0u32;
        for kp in base {
            if kp.x < margin || kp.y < margin || kp.x > size as f32 - margin || kp.y > size as f32 - margin {
                continue;
            }
            let (px, py) = transform_point(t, kp.x, kp.y);
            if px < 0.0 || py < 0.0 || px > size as f32 || py > size as f32 {
                continue;
            }
            checked += 1;
            if transformed.iter().any(|c| (c.x - px).hypot(c.y - py) <= tolerance) {
                matched += 1;
            }
        }
        if checked == 0 {
            0.0
        } else {
            f64::from(matched) / f64::from(checked)
        }
    }

    #[test]
    fn akaze_repeatability_is_at_least_orb_repeatability_under_scale_and_rotation() {
        let size = 96u32;
        let img_a = blob_image(size);
        let transform = TestTransform { cx: size as f32 / 2.0, cy: size as f32 / 2.0, theta: 12f32.to_radians(), scale: 1.25 };
        let m = rotate_scale_matrix(&transform);
        let img_b = warp_affine(&img_a, &m, size, size);
        let margin = 10.0f32;
        let tolerance = 2.5f32;

        let pyr_a = build_pyramid(&img_a, 3);
        let pyr_b = build_pyramid(&img_b, 3);
        let orb_a = detect_orb_keypoints(&pyr_a, 80);
        let orb_b = detect_orb_keypoints(&pyr_b, 80);
        let orb_repeatability = repeatability_fraction(&orb_a, &orb_b, &transform, size, margin, tolerance);

        let scale_space_a = build_akaze_scale_space(&img_a, 3, 4);
        let scale_space_b = build_akaze_scale_space(&img_b, 3, 4);
        let akaze_a = detect_akaze_keypoints(&scale_space_a, 80);
        let akaze_b = detect_akaze_keypoints(&scale_space_b, 80);
        let akaze_repeatability = repeatability_fraction(&akaze_a, &akaze_b, &transform, size, margin, tolerance);

        assert!(!orb_a.is_empty() && !akaze_a.is_empty(), "both detectors should find keypoints on the base image");
        assert!(akaze_repeatability > 0.0, "expected some AKAZE keypoints to survive the scale+rotation transform, got {akaze_repeatability:.3}");
        assert!(
            akaze_repeatability >= orb_repeatability,
            "expected AKAZE repeatability ({akaze_repeatability:.3}) to be at least ORB's repeatability ({orb_repeatability:.3}) under a 1.25x scale + 12deg rotation"
        );
    }

    #[test]
    fn describe_akaze_is_deterministic_and_self_hamming_zero() {
        let img = textured_image(64);
        let scale_space = build_akaze_scale_space(&img, 3, 4);
        let kp = Keypoint { x: 32.0, y: 32.0, octave: 0, angle: 0.3, response: 1.0 };
        let d1 = describe_akaze(&scale_space, &[kp]);
        let d2 = describe_akaze(&scale_space, &[kp]);
        assert_eq!(d1[0], d2[0], "describing the same AKAZE keypoint twice should give an identical M-LDB descriptor");
        assert_eq!(d1[0].hamming_distance(&d1[0]), 0);
    }
    // #endregion 🔖AkazeTests
}
// #endregion 🔖Tests
