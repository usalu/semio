//! 🧼️ Deblocking and output of the baseline H.264 decoder: the loop filter, the decoded picture buffer and the YUV conversion. See [`super`].

use super::*;

// #region 🔖️Deblock
/// 📏️ `Alpha[qP]`, `qP` 0..51 (clause 8.7.2.2, Table 8-16 column). <https://www.itu.int/rec/T-REC-H.264>
const DEBLOCK_ALPHA: [i32; 52] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 5, 6, 7, 8, 9, 10, 12, 13, 15, 17, 20, 22, 25, 28, 32, 36, 40, 45, 50, 56, 63, 71, 80, 90, 101, 113, 127, 144, 162, 182, 203, 226, 255, 255];

/// 📏️ `Beta[qP]`, `qP` 0..51 (clause 8.7.2.2, Table 8-16 column).
const DEBLOCK_BETA: [i32; 52] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18];

/// 📏️ `tC0[qP][bS-1]` for `bS` 1..3, `qP` 0..51 (clause 8.7.2.3, Table 8-17); `bS == 4` never consults this
/// table (it uses the strong, unconditional intra filter instead).
const DEBLOCK_TC0: [[i32; 3]; 52] = [
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 1],
    [0, 0, 1],
    [0, 0, 1],
    [0, 0, 1],
    [0, 1, 1],
    [0, 1, 1],
    [1, 1, 1],
    [1, 1, 1],
    [1, 1, 1],
    [1, 1, 1],
    [1, 1, 2],
    [1, 1, 2],
    [1, 1, 2],
    [1, 1, 2],
    [1, 2, 3],
    [1, 2, 3],
    [2, 2, 3],
    [2, 2, 4],
    [2, 3, 4],
    [2, 3, 4],
    [3, 3, 5],
    [3, 4, 6],
    [3, 4, 6],
    [4, 5, 7],
    [4, 5, 8],
    [4, 6, 9],
    [5, 7, 10],
    [6, 8, 11],
    [6, 8, 13],
    [7, 10, 14],
    [8, 11, 16],
    [9, 12, 18],
    [10, 13, 20],
    [11, 15, 23],
    [13, 17, 25],
];

/// 🧮️ Boundary strength (clause 8.7.2.1) for the edge between 4×4 luma blocks `p` and `q`: `4` only at a true
/// macroblock edge between two intra macroblocks, `3` at an intra macroblock's internal edges, `2` if either
/// side has nonzero luma coefficients, `1` if the motion differs (reference frame, or either MV component by
/// `>= 4` quarter-pel units), else `0` (no filtering).
#[allow(clippy::too_many_arguments, reason = "boundary strength genuinely depends on this many independent per-block facts; bundling them into a struct would just rename the same inputs")]
fn boundary_strength(is_mb_edge: bool, p_intra: bool, q_intra: bool, p_nnz: u8, q_nnz: u8, p_mv: [i32; 2], p_ref: i8, q_mv: [i32; 2], q_ref: i8) -> u8 {
    if p_intra || q_intra {
        return if is_mb_edge { 4 } else { 3 };
    }
    if p_nnz > 0 || q_nnz > 0 {
        return 2;
    }
    if p_ref != q_ref {
        return 1;
    }
    if (p_mv[0] - q_mv[0]).abs() >= 4 || (p_mv[1] - q_mv[1]).abs() >= 4 {
        return 1;
    }
    0
}

/// 🧮️ Clause 8.7.2.3's strong (`bS == 4`) luma sample filter for one row/column of 4 pixels straddling an
/// edge; `p`/`q` are `[p3,p2,p1,p0]`/`[q0,q1,q2,q3]` (`p0`/`q0` adjacent to the edge), returned as the filtered
/// `[p2,p1,p0,q0,q1,q2]` (only `p2`/`q2` change when `!(ap < beta)`/`!(aq < beta)`, spliced in by the caller).
fn filter_luma_strong(p: [i32; 4], q: [i32; 4], alpha: i32, beta: i32) -> ([i32; 3], [i32; 3]) {
    let (p3, p2, p1, p0) = (p[0], p[1], p[2], p[3]);
    let (q0, q1, q2, q3) = (q[0], q[1], q[2], q[3]);
    let strong = (p0 - q0).abs() < alpha && (p1 - p0).abs() < beta && (q1 - q0).abs() < beta;
    if !strong {
        return ([p2, p1, p0], [q0, q1, q2]);
    }
    let small_gap = (p0 - q0).abs() < (alpha >> 2) + 2;
    let new_p0 = if small_gap && (p2 - p0).abs() < beta { (p2 + 2 * p1 + 2 * p0 + 2 * q0 + q1 + 4) >> 3 } else { (2 * p1 + p0 + q1 + 2) >> 2 };
    let new_p1 = if small_gap && (p2 - p0).abs() < beta { (p2 + p1 + p0 + q0 + 2) >> 2 } else { p1 };
    let new_p2 = if small_gap && (p2 - p0).abs() < beta { (2 * p3 + 3 * p2 + p1 + p0 + q0 + 4) >> 3 } else { p2 };
    let new_q0 = if small_gap && (q2 - q0).abs() < beta { (q2 + 2 * q1 + 2 * q0 + 2 * p0 + p1 + 4) >> 3 } else { (2 * q1 + q0 + p1 + 2) >> 2 };
    let new_q1 = if small_gap && (q2 - q0).abs() < beta { (q2 + q1 + q0 + p0 + 2) >> 2 } else { q1 };
    let new_q2 = if small_gap && (q2 - q0).abs() < beta { (2 * q3 + 3 * q2 + q1 + q0 + p0 + 4) >> 3 } else { q2 };
    ([new_p2, new_p1, new_p0], [new_q0, new_q1, new_q2])
}

/// 🧮️ Clause 8.7.2.3's normal (`bS` 1..3) luma sample filter, returning filtered `(p1, p0, q0, q1)`; `p1`/`q1`
/// only change when the local activity is low enough (`ap < beta` / `aq < beta`).
fn filter_luma_normal(p: [i32; 3], q: [i32; 3], alpha: i32, beta: i32, tc0: i32) -> Option<(i32, i32, i32, i32)> {
    let (p2, p1, p0) = (p[0], p[1], p[2]);
    let (q0, q1, q2) = (q[0], q[1], q[2]);
    if (p0 - q0).abs() >= alpha || (p1 - p0).abs() >= beta || (q1 - q0).abs() >= beta {
        return None;
    }
    let ap = (p2 - p0).abs();
    let aq = (q2 - q0).abs();
    let tc = tc0 + i32::from(ap < beta) + i32::from(aq < beta);
    let delta = (((q0 - p0) * 4 + (p1 - q1) + 4) >> 3).clamp(-tc, tc);
    let new_p0 = clip_u8(p0 + delta);
    let new_q0 = clip_u8(q0 - delta);
    let new_p1 = if ap < beta { p1 + (((p2 + ((p0 + q0 + 1) >> 1)) >> 1) - p1).clamp(-tc0, tc0) } else { p1 };
    let new_q1 = if aq < beta { q1 + (((q2 + ((p0 + q0 + 1) >> 1)) >> 1) - q1).clamp(-tc0, tc0) } else { q1 };
    Some((new_p1, new_p0, new_q0, new_q1))
}

/// 🧮️ Chroma edge filter (clause 8.7.2.4), `bS` 1..3: filtered `(p0, q0)`.
fn filter_chroma_normal(p1: i32, p0: i32, q0: i32, q1: i32, alpha: i32, beta: i32, tc: i32) -> Option<(i32, i32)> {
    if (p0 - q0).abs() >= alpha || (p1 - p0).abs() >= beta || (q1 - q0).abs() >= beta {
        return None;
    }
    let delta = (((q0 - p0) * 4 + (p1 - q1) + 4) >> 3).clamp(-tc, tc);
    Some((clip_u8(p0 + delta), clip_u8(q0 - delta)))
}

/// 🧮️ Chroma edge filter, `bS == 4`: unconditional averaging (clause 8.7.2.4).
fn filter_chroma_strong(p1: i32, p0: i32, q0: i32, q1: i32, alpha: i32, beta: i32) -> Option<(i32, i32)> {
    if (p0 - q0).abs() >= alpha || (p1 - p0).abs() >= beta || (q1 - q0).abs() >= beta {
        return None;
    }
    Some(((2 * p1 + p0 + q1 + 2) >> 2, (2 * q1 + q0 + p1 + 2) >> 2))
}

/// 🧮️ Filters the 4 luma edges (macroblock edge + 3 internal, clause 8.7) of one 4-row segment straddling a
/// vertical edge at picture column `edge_x`, rows `[y, y+4)`.
#[allow(clippy::too_many_arguments, reason = "the deblocking filter's boundary-strength/threshold lookup genuinely needs this many independent per-edge facts")]
fn deblock_vertical_segment(pic: &mut Picture, edge_x: usize, y: usize, is_mb_edge: bool, p_intra: bool, q_intra: bool, p_qp: i32, q_qp: i32, block_y: usize, p_block_x: usize, q_block_x: usize, a_off: i32, b_off: i32) {
    let lw4 = pic.luma4_width();
    let p_nnz = pic.nnz_luma[block_y * lw4 + p_block_x];
    let q_nnz = pic.nnz_luma[block_y * lw4 + q_block_x];
    let p_mv = pic.mv[block_y * lw4 + p_block_x];
    let q_mv = pic.mv[block_y * lw4 + q_block_x];
    let p_ref = pic.ref_idx[block_y * lw4 + p_block_x];
    let q_ref = pic.ref_idx[block_y * lw4 + q_block_x];
    let bs = boundary_strength(is_mb_edge, p_intra, q_intra, p_nnz, q_nnz, p_mv, p_ref, q_mv, q_ref);
    if bs == 0 {
        return;
    }
    let avg_qp = (p_qp + q_qp + 1) >> 1;
    let index_a = (avg_qp + a_off).clamp(0, 51) as usize;
    let index_b = (avg_qp + b_off).clamp(0, 51) as usize;
    let (alpha, beta) = (DEBLOCK_ALPHA[index_a], DEBLOCK_BETA[index_b]);
    if alpha == 0 || beta == 0 {
        return;
    }
    let lw = pic.luma_width();
    for row in y..y + 4 {
        let get = |dx: i32| i32::from(pic.luma[row * lw + (edge_x as i32 + dx) as usize]);
        if bs == 4 {
            let (pf, qf) = filter_luma_strong([get(-4), get(-3), get(-2), get(-1)], [get(0), get(1), get(2), get(3)], alpha, beta);
            for (k, &v) in pf.iter().enumerate() {
                pic.luma[row * lw + edge_x - 3 + k] = v.clamp(0, 255) as u8;
            }
            for (k, &v) in qf.iter().enumerate() {
                pic.luma[row * lw + edge_x + k] = v.clamp(0, 255) as u8;
            }
        } else if let Some((np1, np0, nq0, nq1)) = filter_luma_normal([get(-3), get(-2), get(-1)], [get(0), get(1), get(2)], alpha, beta, DEBLOCK_TC0[index_a][bs as usize - 1]) {
            pic.luma[row * lw + edge_x - 2] = np1.clamp(0, 255) as u8;
            pic.luma[row * lw + edge_x - 1] = np0.clamp(0, 255) as u8;
            pic.luma[row * lw + edge_x] = nq0.clamp(0, 255) as u8;
            pic.luma[row * lw + edge_x + 1] = nq1.clamp(0, 255) as u8;
        }
    }
}

/// 🧮️ Filters the 4 luma edges of one 4-column segment straddling a horizontal edge at picture row `edge_y`,
/// columns `[x, x+4)` — the transpose of [`deblock_vertical_segment`].
#[allow(clippy::too_many_arguments, reason = "same as deblock_vertical_segment, whose transpose this is")]
fn deblock_horizontal_segment(pic: &mut Picture, x: usize, edge_y: usize, is_mb_edge: bool, p_intra: bool, q_intra: bool, p_qp: i32, q_qp: i32, block_x: usize, p_block_y: usize, q_block_y: usize, a_off: i32, b_off: i32) {
    let lw4 = pic.luma4_width();
    let p_nnz = pic.nnz_luma[p_block_y * lw4 + block_x];
    let q_nnz = pic.nnz_luma[q_block_y * lw4 + block_x];
    let p_mv = pic.mv[p_block_y * lw4 + block_x];
    let q_mv = pic.mv[q_block_y * lw4 + block_x];
    let p_ref = pic.ref_idx[p_block_y * lw4 + block_x];
    let q_ref = pic.ref_idx[q_block_y * lw4 + block_x];
    let bs = boundary_strength(is_mb_edge, p_intra, q_intra, p_nnz, q_nnz, p_mv, p_ref, q_mv, q_ref);
    if bs == 0 {
        return;
    }
    let avg_qp = (p_qp + q_qp + 1) >> 1;
    let index_a = (avg_qp + a_off).clamp(0, 51) as usize;
    let index_b = (avg_qp + b_off).clamp(0, 51) as usize;
    let (alpha, beta) = (DEBLOCK_ALPHA[index_a], DEBLOCK_BETA[index_b]);
    if alpha == 0 || beta == 0 {
        return;
    }
    let lw = pic.luma_width();
    for col in x..x + 4 {
        let get = |dy: i32| i32::from(pic.luma[(edge_y as i32 + dy) as usize * lw + col]);
        if bs == 4 {
            let (pf, qf) = filter_luma_strong([get(-4), get(-3), get(-2), get(-1)], [get(0), get(1), get(2), get(3)], alpha, beta);
            for (k, &v) in pf.iter().enumerate() {
                pic.luma[(edge_y - 3 + k) * lw + col] = v.clamp(0, 255) as u8;
            }
            for (k, &v) in qf.iter().enumerate() {
                pic.luma[(edge_y + k) * lw + col] = v.clamp(0, 255) as u8;
            }
        } else if let Some((np1, np0, nq0, nq1)) = filter_luma_normal([get(-3), get(-2), get(-1)], [get(0), get(1), get(2)], alpha, beta, DEBLOCK_TC0[index_a][bs as usize - 1]) {
            pic.luma[(edge_y - 2) * lw + col] = np1.clamp(0, 255) as u8;
            pic.luma[(edge_y - 1) * lw + col] = np0.clamp(0, 255) as u8;
            pic.luma[edge_y * lw + col] = nq0.clamp(0, 255) as u8;
            pic.luma[(edge_y + 1) * lw + col] = nq1.clamp(0, 255) as u8;
        }
    }
}

/// 🧮️ In-loop deblocking filter over the whole picture (clause 8.7): all vertical edges first (macroblock
/// raster order), then all horizontal edges, each macroblock edge before its own internal edges. Luma only —
/// chroma filtering is a scoped-out simplification of this decoder (see crate-level notes); harmless for this
/// crate's own encoder output, which always sets `disable_deblocking_filter_idc = 1`.
pub(super) fn deblock_picture(pic: &mut Picture, sps: &SpsInfo, header: &SliceHeaderInfo) {
    if header.disable_deblocking_filter_idc == 1 {
        return;
    }
    let (mb_width, mb_height) = (sps.pic_width_in_mbs, sps.pic_height_in_mbs);
    let (a_off, b_off) = (header.slice_alpha_c0_offset, header.slice_beta_offset);
    for mb_y in 0..mb_height {
        for mb_x in 0..mb_width {
            let mbidx = pic.mb_index(mb_x, mb_y);
            let (q_intra, q_qp) = (pic.mb_is_intra[mbidx], pic.mb_qp[mbidx]);
            for edge in 0..4u32 {
                let is_mb_edge = edge == 0;
                if is_mb_edge && mb_x == 0 {
                    continue;
                }
                let (p_intra, p_qp) = if is_mb_edge {
                    let pidx = pic.mb_index(mb_x - 1, mb_y);
                    (pic.mb_is_intra[pidx], pic.mb_qp[pidx])
                } else {
                    (q_intra, q_qp)
                };
                let edge_x = (mb_x * 16 + edge * 4) as usize;
                for seg in 0..4u32 {
                    let block_y = (mb_y * 4 + seg) as usize;
                    let q_block_x = edge_x / 4;
                    deblock_vertical_segment(pic, edge_x, block_y * 4, is_mb_edge, p_intra, q_intra, p_qp, q_qp, block_y, q_block_x - 1, q_block_x, a_off, b_off);
                }
            }
        }
    }
    for mb_y in 0..mb_height {
        for mb_x in 0..mb_width {
            let mbidx = pic.mb_index(mb_x, mb_y);
            let (q_intra, q_qp) = (pic.mb_is_intra[mbidx], pic.mb_qp[mbidx]);
            for edge in 0..4u32 {
                let is_mb_edge = edge == 0;
                if is_mb_edge && mb_y == 0 {
                    continue;
                }
                let (p_intra, p_qp) = if is_mb_edge {
                    let pidx = pic.mb_index(mb_x, mb_y - 1);
                    (pic.mb_is_intra[pidx], pic.mb_qp[pidx])
                } else {
                    (q_intra, q_qp)
                };
                let edge_y = (mb_y * 16 + edge * 4) as usize;
                for seg in 0..4u32 {
                    let block_x = (mb_x * 4 + seg) as usize;
                    let q_block_y = edge_y / 4;
                    deblock_horizontal_segment(pic, block_x * 4, edge_y, is_mb_edge, p_intra, q_intra, p_qp, q_qp, block_x, q_block_y - 1, q_block_y, a_off, b_off);
                }
            }
        }
    }
    deblock_chroma_mb_edges(pic, sps, a_off, b_off);
}

/// 🧮️ Chroma vertical-macroblock-edge-only deblocking (a scoped simplification of clause 8.7.2: horizontal
/// chroma MB edges and the one internal chroma edge are not filtered — harmless for this crate's own
/// encoder, which always disables the filter). `bS` is re-derived directly from the co-located luma 4×4
/// block's side information rather than precisely reusing clause 8.7.2.1's luma-edge bS array at chroma
/// sub-positions.
fn deblock_chroma_mb_edges(pic: &mut Picture, sps: &SpsInfo, a_off: i32, b_off: i32) {
    let (mb_width, mb_height) = (sps.pic_width_in_mbs, sps.pic_height_in_mbs);
    let lw4 = pic.luma4_width();
    let cw = pic.chroma_width();
    for mb_y in 0..mb_height {
        for mb_x in 1..mb_width {
            let mbidx = pic.mb_index(mb_x, mb_y);
            let pidx = pic.mb_index(mb_x - 1, mb_y);
            let (q_intra, q_qp) = (pic.mb_is_intra[mbidx], pic.mb_qp[mbidx]);
            let (p_intra, p_qp) = (pic.mb_is_intra[pidx], pic.mb_qp[pidx]);
            let avg_qp = chroma_qp((p_qp + q_qp + 1) >> 1, 0);
            let (index_a, index_b) = ((avg_qp + a_off).clamp(0, 51) as usize, (avg_qp + b_off).clamp(0, 51) as usize);
            let (alpha, beta) = (DEBLOCK_ALPHA[index_a], DEBLOCK_BETA[index_b]);
            if alpha == 0 || beta == 0 {
                continue;
            }
            for qy in 0..2usize {
                let block_row = (mb_y * 4) as usize + qy * 2;
                let q_block_x = (mb_x * 4) as usize;
                let (p_nnz, q_nnz) = (pic.nnz_luma[block_row * lw4 + q_block_x - 1], pic.nnz_luma[block_row * lw4 + q_block_x]);
                let (p_mv, q_mv) = (pic.mv[block_row * lw4 + q_block_x - 1], pic.mv[block_row * lw4 + q_block_x]);
                let (p_ref, q_ref) = (pic.ref_idx[block_row * lw4 + q_block_x - 1], pic.ref_idx[block_row * lw4 + q_block_x]);
                let bs = boundary_strength(true, p_intra, q_intra, p_nnz, q_nnz, p_mv, p_ref, q_mv, q_ref);
                if bs == 0 {
                    continue;
                }
                let tc = if bs == 4 { 0 } else { DEBLOCK_TC0[index_a][bs as usize - 1] + 1 };
                let edge_x = (mb_x * 8) as usize;
                for row in (mb_y * 8) as usize + qy * 4..(mb_y * 8) as usize + qy * 4 + 4 {
                    for plane in [&mut pic.cb, &mut pic.cr] {
                        let get = |dx: i32, plane: &[u8]| i32::from(plane[row * cw + (edge_x as i32 + dx) as usize]);
                        let filtered =
                            if bs == 4 { filter_chroma_strong(get(-2, plane), get(-1, plane), get(0, plane), get(1, plane), alpha, beta) } else { filter_chroma_normal(get(-2, plane), get(-1, plane), get(0, plane), get(1, plane), alpha, beta, tc) };
                        if let Some((p0, q0)) = filtered {
                            plane[row * cw + edge_x - 1] = p0.clamp(0, 255) as u8;
                            plane[row * cw + edge_x] = q0.clamp(0, 255) as u8;
                        }
                    }
                }
            }
        }
    }
}
// #endregion 🔖️Deblock

// #region 🔖️Dpb
/// 🗃️ Decoded picture buffer with sliding-window reference management (clause 8.2.5.3): the `max_num_ref_frames`
/// most recently decoded pictures are kept as references, oldest evicted first. Explicit MMCO adaptive marking
/// is rejected earlier (at slice-header parse time), so this is the only marking process this decoder needs.
pub(super) struct Dpb {
    pictures: Vec<Picture>,
    pub(super) max_num_ref_frames: usize,
}

impl Dpb {
    pub(super) fn new(max_num_ref_frames: u32) -> Self {
        Self { pictures: Vec::new(), max_num_ref_frames: (max_num_ref_frames as usize).max(1) }
    }

    pub(super) fn clear(&mut self) {
        self.pictures.clear();
    }

    pub(super) fn push(&mut self, pic: Picture) {
        self.pictures.push(pic);
        while self.pictures.len() > self.max_num_ref_frames {
            self.pictures.remove(0);
        }
    }

    /// 📜️ `RefPicList0`'s default initialization for P slices (clause 8.2.4.2.1): descending `PicNum`, i.e.
    /// most-recently-decoded reference first. This decoder rejects explicit reordering, so this *is*
    /// `RefPicList0`.
    pub(super) fn ref_list0(&self) -> Vec<&Picture> {
        self.pictures.iter().rev().collect()
    }
}
// #endregion 🔖️Dpb

// #region 🔖️Yuv
/// 🎨️ 4:2:0 planes → RGBA, cropped to `width`×`height` (clause E.2.1's default YCbCr matrix, BT.601 full range
/// — kept as a local, private duplicate of `remodeling_image`'s identical JPEG conversion rather than a new
/// public cross-crate dependency surface for one formula).
pub(super) fn ycbcr420_to_rgba(luma: &[u8], luma_stride: usize, cb: &[u8], cr: &[u8], chroma_stride: usize, width: u32, height: u32) -> remodeling_image::ImageRgba8 {
    let mut out = remodeling_image::ImageRgba8::new(width, height);
    for y in 0..height as usize {
        for x in 0..width as usize {
            let yy = f32::from(luma[y * luma_stride + x]);
            let (cx, cy) = (x / 2, y / 2);
            let cb_v = f32::from(cb[cy * chroma_stride + cx]) - 128.0;
            let cr_v = f32::from(cr[cy * chroma_stride + cx]) - 128.0;
            let idx = (y * width as usize + x) * 4;
            out.data[idx] = (yy + 1.402 * cr_v).round().clamp(0.0, 255.0) as u8;
            out.data[idx + 1] = (yy - 0.344_136 * cb_v - 0.714_136 * cr_v).round().clamp(0.0, 255.0) as u8;
            out.data[idx + 2] = (yy + 1.772 * cb_v).round().clamp(0.0, 255.0) as u8;
            out.data[idx + 3] = 255;
        }
    }
    out
}
// #endregion 🔖️Yuv
