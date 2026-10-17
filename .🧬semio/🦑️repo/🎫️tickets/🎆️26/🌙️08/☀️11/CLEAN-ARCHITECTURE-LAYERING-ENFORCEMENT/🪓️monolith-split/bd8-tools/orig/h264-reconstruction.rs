//! 🧮️ Reconstruction of the baseline H.264 decoder: the inverse transform, the picture buffer and the intra and inter prediction. See [`super`].

use super::*;

// #region 🔖️Transform
/// 🧮️ `normAdjust4x4` base values `{v0, v1, v2}` per `m = qP % 6` (clause 8.5.9, Table not numbered — the
/// three distinct scale factors placed at `(even,even)`, `(odd,odd)`, and mixed-parity 4×4 positions).
const LEVEL_SCALE_BASE: [[i32; 3]; 6] = [[10, 13, 16], [11, 14, 18], [13, 16, 20], [14, 18, 23], [16, 20, 25], [18, 23, 29]];

/// 🧮️ `normAdjust4x4(m, i, j)` (clause 8.5.9).
fn norm_adjust(m: usize, i: usize, j: usize) -> i32 {
    if i.is_multiple_of(2) && j.is_multiple_of(2) {
        LEVEL_SCALE_BASE[m][0]
    } else if i % 2 == 1 && j % 2 == 1 {
        LEVEL_SCALE_BASE[m][1]
    } else {
        LEVEL_SCALE_BASE[m][2]
    }
}

/// 🧮️ Scales quantized 4×4 residual coefficients (raster order) back to the transform domain (clause 8.5.12.1).
pub(super) fn dequant4x4(coeffs: &[i32; 16], qp: i32) -> [i32; 16] {
    let m = qp.rem_euclid(6) as usize;
    let shift = qp.div_euclid(6);
    let mut out = [0i32; 16];
    for i in 0..4 {
        for j in 0..4 {
            let pos = i * 4 + j;
            let scale = norm_adjust(m, i, j);
            out[pos] = if shift >= 4 { (coeffs[pos] * scale) << (shift - 4) } else { (coeffs[pos] * scale + (1 << (3 - shift))) >> (4 - shift) };
        }
    }
    out
}

/// 🌊️ H.264's integer core transform butterfly, applied separably (clause 8.5.12.2); its own inverse up to
/// the final `(x + 32) >> 6` rounding/normalization.
fn idct4x4_1d(input: [i32; 4]) -> [i32; 4] {
    let e0 = input[0] + input[2];
    let e1 = input[0] - input[2];
    let e2 = (input[1] >> 1) - input[3];
    let e3 = input[1] + (input[3] >> 1);
    [e0 + e3, e1 + e2, e1 - e2, e0 - e3]
}

/// 🌊️ Full separable 4×4 inverse core transform: dequantized coefficients (raster order) → spatial residual.
pub(super) fn idct4x4(d: &[i32; 16]) -> [i32; 16] {
    let mut cols = [0i32; 16];
    for c in 0..4 {
        let t = idct4x4_1d([d[c], d[4 + c], d[8 + c], d[12 + c]]);
        for (r, &v) in t.iter().enumerate() {
            cols[r * 4 + c] = v;
        }
    }
    let mut out = [0i32; 16];
    for r in 0..4 {
        let t = idct4x4_1d([cols[r * 4], cols[r * 4 + 1], cols[r * 4 + 2], cols[r * 4 + 3]]);
        for (c, &v) in t.iter().enumerate() {
            out[r * 4 + c] = (v + 32) >> 6;
        }
    }
    out
}

/// 🌊️ 4×4 Hadamard butterfly (symmetric, its own inverse up to scale), shared by the luma-DC transform.
fn hadamard4_1d(input: [i32; 4]) -> [i32; 4] {
    let e0 = input[0] + input[2];
    let e1 = input[0] - input[2];
    let e2 = input[1] - input[3];
    let e3 = input[1] + input[3];
    [e0 + e3, e1 + e2, e1 - e2, e0 - e3]
}

/// 🌊️ Inverse-transforms and dequantizes the 16 `Intra16x16DCLevel` coefficients (raster block order, i.e.
/// `dc[blockRow*4+blockCol]`) into per-block DC values to splice into position 0 of each luma 4×4 AC block
/// (clause 8.5.10).
pub(super) fn transform_luma16x16_dc(dc: &[i32; 16], qp: i32) -> [i32; 16] {
    let mut cols = [0i32; 16];
    for c in 0..4 {
        let t = hadamard4_1d([dc[c], dc[4 + c], dc[8 + c], dc[12 + c]]);
        for (r, &v) in t.iter().enumerate() {
            cols[r * 4 + c] = v;
        }
    }
    let mut f = [0i32; 16];
    for r in 0..4 {
        let t = hadamard4_1d([cols[r * 4], cols[r * 4 + 1], cols[r * 4 + 2], cols[r * 4 + 3]]);
        for (c, &v) in t.iter().enumerate() {
            f[r * 4 + c] = v;
        }
    }
    let m = qp.rem_euclid(6) as usize;
    let shift = qp.div_euclid(6);
    let scale = LEVEL_SCALE_BASE[m][0];
    let mut out = [0i32; 16];
    for (o, &fv) in out.iter_mut().zip(f.iter()) {
        *o = if shift >= 6 { (fv * scale) << (shift - 6) } else { (fv * scale + (1 << (5 - shift))) >> (6 - shift) };
    }
    out
}

/// 🌊️ Inverse-transforms and dequantizes the 4 `ChromaDCLevel` coefficients (`[cb00,cb01,cb10,cb11]` raster)
/// into per-4×4-block DC values (clause 8.5.11).
pub(super) fn transform_chroma_dc(dc: &[i32; 4], qp: i32) -> [i32; 4] {
    let e00 = dc[0] + dc[1] + dc[2] + dc[3];
    let e01 = dc[0] - dc[1] + dc[2] - dc[3];
    let e10 = dc[0] + dc[1] - dc[2] - dc[3];
    let e11 = dc[0] - dc[1] - dc[2] + dc[3];
    let m = qp.rem_euclid(6) as usize;
    let shift = qp.div_euclid(6);
    let scale = LEVEL_SCALE_BASE[m][0];
    [e00, e01, e10, e11].map(|f| ((f * scale) << shift) >> 5)
}
// #endregion 🔖️Transform

// #region 🔖️Picture
/// 🖼️ One in-progress or fully reconstructed picture's working state: 4:2:0 sample planes plus the per-4×4
/// (luma) / per-4×4-chroma-quadrant side information (`is_intra`, motion, non-zero counts) later stages
/// (deblocking, next-frame prediction) need. Sample planes are macroblock-padded (`mb_width*16 × mb_height*16`
/// luma, half that for chroma) — [`Picture::crop_to`] trims to `width × height` once decode finishes.
pub(super) struct Picture {
    pub(super) mb_width: u32,
    pub(super) mb_height: u32,
    pub(super) luma: Vec<u8>,
    pub(super) cb: Vec<u8>,
    pub(super) cr: Vec<u8>,
    pub(super) decoded_luma4: Vec<bool>,
    pub(super) mb_is_intra: Vec<bool>,
    pub(super) mb_qp: Vec<i32>,
    pub(super) mv: Vec<[i32; 2]>,
    pub(super) ref_idx: Vec<i8>,
    pub(super) nnz_luma: Vec<u8>,
    pub(super) nnz_cb: Vec<u8>,
    pub(super) nnz_cr: Vec<u8>,
    pub(super) intra4x4_mode: Vec<i8>,
    pub(super) frame_num: u32,
    pub(super) poc: i32,
}

impl Picture {
    pub(super) fn new(mb_width: u32, mb_height: u32) -> Self {
        let lw = (mb_width * 16) as usize;
        let lh = (mb_height * 16) as usize;
        let cw = (mb_width * 8) as usize;
        let ch = (mb_height * 8) as usize;
        let luma4_w = (mb_width * 4) as usize;
        let luma4_h = (mb_height * 4) as usize;
        let mb_count = (mb_width * mb_height) as usize;
        Self {
            mb_width,
            mb_height,
            luma: vec![0; lw * lh],
            cb: vec![128; cw * ch],
            cr: vec![128; cw * ch],
            decoded_luma4: vec![false; luma4_w * luma4_h],
            mb_is_intra: vec![false; mb_count],
            mb_qp: vec![0; mb_count],
            mv: vec![[0, 0]; luma4_w * luma4_h],
            ref_idx: vec![-1; luma4_w * luma4_h],
            nnz_luma: vec![0; luma4_w * luma4_h],
            nnz_cb: vec![0; luma4_w * luma4_h / 4],
            nnz_cr: vec![0; luma4_w * luma4_h / 4],
            intra4x4_mode: vec![-1; luma4_w * luma4_h],
            frame_num: 0,
            poc: 0,
        }
    }

    pub(super) fn luma_width(&self) -> usize {
        (self.mb_width * 16) as usize
    }

    pub(super) fn luma4_width(&self) -> usize {
        (self.mb_width * 4) as usize
    }

    pub(super) fn chroma_width(&self) -> usize {
        (self.mb_width * 8) as usize
    }

    pub(super) fn chroma4_width(&self) -> usize {
        (self.mb_width * 2) as usize
    }

    /// 🔍️ Reconstructed luma sample at `(x, y)`, or `None` outside the (macroblock-padded) picture.
    pub(super) fn luma_at(&self, x: i32, y: i32) -> Option<u8> {
        if x < 0 || y < 0 || x as usize >= self.luma_width() || y as usize >= (self.mb_height * 16) as usize {
            return None;
        }
        Some(self.luma[y as usize * self.luma_width() + x as usize])
    }

    pub(super) fn luma4_available(&self, bx: i32, by: i32) -> bool {
        if bx < 0 || by < 0 || bx as usize >= self.luma4_width() || by as usize >= (self.mb_height * 4) as usize {
            return false;
        }
        self.decoded_luma4[by as usize * self.luma4_width() + bx as usize]
    }

    pub(super) fn mb_index(&self, mb_x: u32, mb_y: u32) -> usize {
        (mb_y * self.mb_width + mb_x) as usize
    }

    /// 🎨️ Writes one reconstructed 4×4 luma block (clamped to `[0,255]`) at 4×4-block grid position `(bx, by)`
    /// and marks it decoded.
    pub(super) fn write_luma4(&mut self, bx: u32, by: u32, block: &[i32; 16]) {
        let w = self.luma_width();
        let (px, py) = (bx as usize * 4, by as usize * 4);
        for r in 0..4 {
            for c in 0..4 {
                self.luma[(py + r) * w + px + c] = block[r * 4 + c].clamp(0, 255) as u8;
            }
        }
        let idx = by as usize * self.luma4_width() + bx as usize;
        self.decoded_luma4[idx] = true;
    }

    /// 🎨️ Writes one reconstructed 4×4 chroma block into `plane` (`cb`/`cr`) at chroma-pixel origin `(px, py)`.
    pub(super) fn write_chroma4(plane: &mut [u8], stride: usize, px: usize, py: usize, block: &[i32; 16]) {
        for r in 0..4 {
            for c in 0..4 {
                plane[(py + r) * stride + px + c] = block[r * 4 + c].clamp(0, 255) as u8;
            }
        }
    }

    /// ✂️ Crops the macroblock-padded planes down to the sequence's coded `width`×`height`, producing the
    /// final displayable [`remodeling_image::ImageRgba8`] via [`ycbcr420_to_rgba`].
    pub(super) fn crop_to(&self, width: u32, height: u32) -> remodeling_image::ImageRgba8 {
        ycbcr420_to_rgba(&self.luma, self.luma_width(), &self.cb, &self.cr, self.chroma_width(), width, height)
    }
}
// #endregion 🔖️Picture

// #region 🔖️Intra
/// 🎨️ One-sided (or corner) neighbor availability + samples for 4×4 luma intra prediction: `top`/`left` hold 4
/// samples each, `top_right` holds the 4 samples beyond the top row's right edge (replicated from the
/// rightmost available top sample when genuinely unavailable, per clause 8.3.1.2.1), `corner` is the top-left
/// sample.
pub(super) struct Intra4Neighbors {
    pub(super) top: Option<[i32; 4]>,
    pub(super) left: Option<[i32; 4]>,
    pub(super) top_right: [i32; 4],
    pub(super) corner: i32,
}

/// 🎨️ Predicts one 4×4 luma block for `mode` (0..8, clause 8.3.1.2); callers add the residual afterward.
pub(super) fn predict_intra4x4(mode: u8, n: &Intra4Neighbors) -> Result<[i32; 16], H264Error> {
    let t = n.top.unwrap_or([128; 4]);
    let l = n.left.unwrap_or([128; 4]);
    let tl = n.corner;
    let tr = n.top_right;
    let tt = |i: i32| if i < 0 { tl } else { t[i as usize] };
    let ll = |i: i32| if i < 0 { tl } else { l[i as usize] };
    let tfull = |i: i32| if i < 4 { tt(i) } else { tr[(i - 4) as usize] };
    let mut out = [0i32; 16];
    let mut px = |x: usize, y: usize, v: i32| out[y * 4 + x] = v;
    match mode {
        0 => {
            if n.top.is_none() {
                return Err(H264Error::Malformed("intra4x4 vertical mode needs top neighbor"));
            }
            for y in 0..4 {
                for (x, &tv) in t.iter().enumerate() {
                    px(x, y, tv);
                }
            }
        }
        1 => {
            if n.left.is_none() {
                return Err(H264Error::Malformed("intra4x4 horizontal mode needs left neighbor"));
            }
            for (y, &lv) in l.iter().enumerate() {
                for x in 0..4 {
                    px(x, y, lv);
                }
            }
        }
        2 => {
            let dc = match (n.top, n.left) {
                (Some(t), Some(l)) => (t.iter().sum::<i32>() + l.iter().sum::<i32>() + 4) >> 3,
                (Some(t), None) => (t.iter().sum::<i32>() + 2) >> 2,
                (None, Some(l)) => (l.iter().sum::<i32>() + 2) >> 2,
                (None, None) => 128,
            };
            out = [dc; 16];
        }
        3 => {
            for y in 0..4usize {
                for x in 0..4usize {
                    let v = if x == 3 && y == 3 { (tfull(6) + 3 * tfull(7) + 2) >> 2 } else { (tfull((x + y) as i32) + 2 * tfull((x + y + 1) as i32) + tfull((x + y + 2) as i32) + 2) >> 2 };
                    px(x, y, v);
                }
            }
        }
        4 => {
            for y in 0..4i32 {
                for x in 0..4i32 {
                    let v = match x.cmp(&y) {
                        std::cmp::Ordering::Greater => (tt(x - y - 2) + 2 * tt(x - y - 1) + tt(x - y) + 2) >> 2,
                        std::cmp::Ordering::Less => (ll(y - x - 2) + 2 * ll(y - x - 1) + ll(y - x) + 2) >> 2,
                        std::cmp::Ordering::Equal => (t[0] + 2 * tl + l[0] + 2) >> 2,
                    };
                    px(x as usize, y as usize, v);
                }
            }
        }
        5 => {
            for y in 0..4i32 {
                for x in 0..4i32 {
                    let z_vr = 2 * x - y;
                    let v = if z_vr >= 0 && z_vr % 2 == 0 {
                        (tt(x - (y >> 1) - 1) + tt(x - (y >> 1)) + 1) >> 1
                    } else if z_vr >= 1 && z_vr % 2 == 1 {
                        (tt(x - (y >> 1) - 2) + 2 * tt(x - (y >> 1) - 1) + tt(x - (y >> 1)) + 2) >> 2
                    } else if z_vr == -1 {
                        (l[0] + 2 * tl + t[0] + 2) >> 2
                    } else {
                        (ll(y - 1) + 2 * ll(y - 2) + ll(y - 3) + 2) >> 2
                    };
                    px(x as usize, y as usize, v);
                }
            }
        }
        6 => {
            for y in 0..4i32 {
                for x in 0..4i32 {
                    let z_hd = 2 * y - x;
                    let v = if z_hd >= 0 && z_hd % 2 == 0 {
                        (ll(y - (x >> 1) - 1) + ll(y - (x >> 1)) + 1) >> 1
                    } else if z_hd >= 1 && z_hd % 2 == 1 {
                        (ll(y - (x >> 1) - 2) + 2 * ll(y - (x >> 1) - 1) + ll(y - (x >> 1)) + 2) >> 2
                    } else if z_hd == -1 {
                        (l[0] + 2 * tl + t[0] + 2) >> 2
                    } else {
                        (tt(x - 1) + 2 * tt(x - 2) + tt(x - 3) + 2) >> 2
                    };
                    px(x as usize, y as usize, v);
                }
            }
        }
        7 => {
            for y in 0..4usize {
                for x in 0..4usize {
                    let v = if y % 2 == 0 { (tfull((x + y / 2) as i32) + tfull((x + y / 2 + 1) as i32) + 1) >> 1 } else { (tfull((x + y / 2) as i32) + 2 * tfull((x + y / 2 + 1) as i32) + tfull((x + y / 2 + 2) as i32) + 2) >> 2 };
                    px(x, y, v);
                }
            }
        }
        8 => {
            for y in 0..4i32 {
                for x in 0..4i32 {
                    let z_hu = x + 2 * y;
                    let v = if (0..=4).contains(&z_hu) && z_hu % 2 == 0 {
                        (ll(y + (x >> 1)) + ll(y + (x >> 1) + 1) + 1) >> 1
                    } else if z_hu <= 3 && z_hu % 2 == 1 {
                        (ll(y + (x >> 1)) + 2 * ll(y + (x >> 1) + 1) + ll(y + (x >> 1) + 2) + 2) >> 2
                    } else if z_hu == 5 {
                        (l[2] + 3 * l[3] + 2) >> 2
                    } else {
                        l[3]
                    };
                    px(x as usize, y as usize, v);
                }
            }
        }
        _ => return Err(H264Error::Malformed("intra4x4 pred mode out of range")),
    }
    Ok(out)
}

/// 🎨️ Neighbor-average DC prediction for an `n×n` block (Intra16x16 luma `n=16`, chroma DC quadrant `n=4`).
pub(super) fn dc_pred(top: Option<&[i32]>, left: Option<&[i32]>, n: usize) -> i32 {
    match (top, left) {
        (Some(t), Some(l)) => (t.iter().sum::<i32>() + l.iter().sum::<i32>() + n as i32) / (2 * n as i32),
        (Some(t), None) => (t.iter().sum::<i32>() + (n as i32) / 2) / n as i32,
        (None, Some(l)) => (l.iter().sum::<i32>() + (n as i32) / 2) / n as i32,
        (None, None) => 128,
    }
}

/// 🎨️ Plane-mode gradient prediction shared by Intra16x16 luma (`size=16`) and intra chroma (`size=8`), clause
/// 8.3.3.4 / 8.3.4.4 (the `(5,32,6)` vs `(17,16,5)` rounding constants are the only difference between them).
pub(super) fn plane_pred(top: &[i32], left: &[i32], corner: i32, size: usize, h_mul: i32, h_round: i32, h_shift: u32) -> Vec<i32> {
    let half = size / 2;
    let t = |i: i32| if i < 0 { corner } else { top[i as usize] };
    let l = |i: i32| if i < 0 { corner } else { left[i as usize] };
    let mut h_sum = 0i32;
    let mut v_sum = 0i32;
    for x in 0..half as i32 {
        h_sum += (x + 1) * (t(half as i32 + x) - t(half as i32 - 2 - x));
        v_sum += (x + 1) * (l(half as i32 + x) - l(half as i32 - 2 - x));
    }
    let a = 16 * (t(size as i32 - 1) + l(size as i32 - 1));
    let b = (h_mul * h_sum + h_round) >> h_shift;
    let c = (h_mul * v_sum + h_round) >> h_shift;
    let mut out = vec![0i32; size * size];
    for y in 0..size {
        for x in 0..size {
            out[y * size + x] = (a + b * (x as i32 - (half as i32 - 1)) + c * (y as i32 - (half as i32 - 1)) + 16) >> 5;
        }
    }
    out
}
// #endregion 🔖️Intra

// #region 🔖️Inter
/// 🖼️ A reconstructed reference picture's luma plane plus both planes' dimensions, addressed with implicit
/// border-clamped extension (clause 8.4.2.2.1's "samples outside the picture are the nearest picture sample")
/// — every fetch clamps its coordinates in bounds first, so motion vectors may point arbitrarily far outside
/// the picture safely. Chroma planes are passed separately to [`mc_chroma_block`] (they don't need luma's
/// wide interpolation neighborhood, just [`RefPlanes::chroma_px`]'s clamped bilinear taps).
pub(super) struct RefPlanes<'a> {
    pub(super) luma: &'a [u8],
    pub(super) luma_w: i32,
    pub(super) luma_h: i32,
    pub(super) chroma_w: i32,
    pub(super) chroma_h: i32,
}

impl<'a> RefPlanes<'a> {
    fn luma_px(&self, x: i32, y: i32) -> i32 {
        let cx = x.clamp(0, self.luma_w - 1);
        let cy = y.clamp(0, self.luma_h - 1);
        i32::from(self.luma[(cy * self.luma_w + cx) as usize])
    }

    fn chroma_px(plane: &[u8], w: i32, h: i32, x: i32, y: i32) -> i32 {
        let cx = x.clamp(0, w - 1);
        let cy = y.clamp(0, h - 1);
        i32::from(plane[(cy * w + cx) as usize])
    }
}

pub(super) fn clip_u8(v: i32) -> i32 {
    v.clamp(0, 255)
}

/// 🌊️ Raw (unrounded, unclipped) horizontal 6-tap sum at the half-pel position between luma columns `x`/`x+1`.
fn h264_h6_raw(rp: &RefPlanes<'_>, x: i32, y: i32) -> i32 {
    rp.luma_px(x - 2, y) - 5 * rp.luma_px(x - 1, y) + 20 * rp.luma_px(x, y) + 20 * rp.luma_px(x + 1, y) - 5 * rp.luma_px(x + 2, y) + rp.luma_px(x + 3, y)
}

/// 🌊️ Raw (unrounded, unclipped) vertical 6-tap sum at the half-pel position between luma rows `y`/`y+1`.
fn h264_v6_raw(rp: &RefPlanes<'_>, x: i32, y: i32) -> i32 {
    rp.luma_px(x, y - 2) - 5 * rp.luma_px(x, y - 1) + 20 * rp.luma_px(x, y) + 20 * rp.luma_px(x, y + 1) - 5 * rp.luma_px(x, y + 2) + rp.luma_px(x, y + 3)
}

/// 🎯️ Quarter-pel luma sample at integer base `(x0, y0)` plus fractional `(fx, fy)` in `0..=3` quarter-pel
/// units (clause 8.4.2.2.1): the six named half-pel positions (`b`, `h`, `j`, plus the row-below/col-right
/// helpers `m`/`s`) are derived via 6-tap filtering, and the twelve quarter-pel positions average their two
/// nearest half/integer neighbors.
fn luma_qpel_sample(rp: &RefPlanes<'_>, x0: i32, y0: i32, fx: i32, fy: i32) -> i32 {
    if fx == 0 && fy == 0 {
        return rp.luma_px(x0, y0);
    }
    let b = clip_u8((h264_h6_raw(rp, x0, y0) + 16) >> 5);
    let h = clip_u8((h264_v6_raw(rp, x0, y0) + 16) >> 5);
    if fx == 2 && fy == 0 {
        return b;
    }
    if fx == 0 && fy == 2 {
        return h;
    }
    let g = rp.luma_px(x0, y0);
    let hh = rp.luma_px(x0 + 1, y0);
    let m_pel = rp.luma_px(x0, y0 + 1);
    if fy == 0 {
        return if fx == 1 { (g + b + 1) >> 1 } else { (hh + b + 1) >> 1 };
    }
    if fx == 0 {
        return if fy == 1 { (g + h + 1) >> 1 } else { (m_pel + h + 1) >> 1 };
    }
    let j_raw = h264_v6_raw_of_horiz(rp, x0, y0);
    let j = clip_u8((j_raw + 512) >> 10);
    if fx == 2 && fy == 2 {
        return j;
    }
    let m = clip_u8((h264_h6_raw(rp, x0, y0 + 1) + 16) >> 5);
    let s = clip_u8((h264_v6_raw(rp, x0 + 1, y0) + 16) >> 5);
    match (fx, fy) {
        (2, 1) => (b + j + 1) >> 1,
        (2, 3) => (j + m + 1) >> 1,
        (1, 2) => (h + j + 1) >> 1,
        (3, 2) => (j + s + 1) >> 1,
        (1, 1) => (b + h + 1) >> 1,
        (3, 1) => (b + m + 1) >> 1,
        (1, 3) => (h + s + 1) >> 1,
        (3, 3) => (m + s + 1) >> 1,
        _ => unreachable!("fx,fy in 0..=3 exhausted above"),
    }
}

/// 🌊️ The `j` center half-pel's raw two-pass sum: the horizontal 6-tap filter's *unrounded* output at 6
/// consecutive rows, combined with the vertical 6-tap filter (clause 8.4.2.2.1); callers finish with
/// `(raw + 512) >> 10` and clip.
fn h264_v6_raw_of_horiz(rp: &RefPlanes<'_>, x: i32, y: i32) -> i32 {
    let r = |dy: i32| h264_h6_raw(rp, x, y + dy);
    r(-2) - 5 * r(-1) + 20 * r(0) + 20 * r(1) - 5 * r(2) + r(3)
}

/// 🎯️ Bilinear chroma sample at 1/8-pel precision (clause 8.4.2.2.2): `mv_x8`/`mv_y8` are the chroma motion
/// vector components in eighth-chroma-pel units (numerically identical to the luma quarter-pel MV, since
/// chroma is half-resolution in 4:2:0).
fn chroma_bilinear_sample(plane: &[u8], w: i32, h: i32, x: i32, y: i32, mv_x8: i32, mv_y8: i32) -> i32 {
    let full_x = x + (mv_x8 >> 3);
    let full_y = y + (mv_y8 >> 3);
    let frac_x = mv_x8 & 7;
    let frac_y = mv_y8 & 7;
    let a = RefPlanes::chroma_px(plane, w, h, full_x, full_y);
    let b = RefPlanes::chroma_px(plane, w, h, full_x + 1, full_y);
    let c = RefPlanes::chroma_px(plane, w, h, full_x, full_y + 1);
    let d = RefPlanes::chroma_px(plane, w, h, full_x + 1, full_y + 1);
    ((8 - frac_x) * (8 - frac_y) * a + frac_x * (8 - frac_y) * b + (8 - frac_x) * frac_y * c + frac_x * frac_y * d + 32) >> 6
}

/// 🎯️ Motion-compensates one `w×h` luma block at picture position `(px, py)` from `mv` (quarter-pel), writing
/// samples into `out` (row-major, `w*h`).
pub(super) fn mc_luma_block(rp: &RefPlanes<'_>, px: i32, py: i32, w: usize, h: usize, mv: [i32; 2], out: &mut [i32]) {
    for y in 0..h {
        for x in 0..w {
            let full_x = px + x as i32 + (mv[0] >> 2);
            let full_y = py + y as i32 + (mv[1] >> 2);
            out[y * w + x] = luma_qpel_sample(rp, full_x, full_y, mv[0] & 3, mv[1] & 3);
        }
    }
}

/// 🎯️ Motion-compensates one `w×h` chroma block (either plane) at chroma position `(px, py)` from a luma-space
/// `mv` (quarter-pel; reinterpreted as eighth-chroma-pel per clause 8.4.1.4).
#[allow(
    clippy::too_many_arguments,
    reason = "one plane plus its geometry (dims, origin, size) plus the motion vector is the natural, self-describing parameter list for a block motion-compensation primitive; bundling them into a struct would just rename the same fields"
)]
pub(super) fn mc_chroma_block(plane: &[u8], plane_w: i32, plane_h: i32, px: i32, py: i32, w: usize, h: usize, mv: [i32; 2], out: &mut [i32]) {
    for y in 0..h {
        for x in 0..w {
            out[y * w + x] = chroma_bilinear_sample(plane, plane_w, plane_h, px + x as i32, py + y as i32, mv[0], mv[1]);
        }
    }
}

/// 🧭️ Median motion-vector predictor (clause 8.4.1.3): `A`/`B`/`C` are the left/above/above-right neighbors'
/// `(mv, ref_idx)`; `D` is the above-left fallback used only when `C` is unavailable. The 16×8/8×16 single-
/// neighbor shortcuts are handled by the caller (they only apply to those partition shapes).
pub(super) fn median_mv_predict(a: ([i32; 2], i8), b: ([i32; 2], i8), c: ([i32; 2], i8), ref_idx: i8) -> [i32; 2] {
    let matches: Vec<[i32; 2]> = [a, b, c].iter().filter(|&&(_, r)| r == ref_idx).map(|&(mv, _)| mv).collect();
    if matches.len() == 1 {
        return matches[0];
    }
    let mut xs = [a.0[0], b.0[0], c.0[0]];
    let mut ys = [a.0[1], b.0[1], c.0[1]];
    xs.sort_unstable();
    ys.sort_unstable();
    [xs[1], ys[1]]
}
// #endregion 🔖️Inter
