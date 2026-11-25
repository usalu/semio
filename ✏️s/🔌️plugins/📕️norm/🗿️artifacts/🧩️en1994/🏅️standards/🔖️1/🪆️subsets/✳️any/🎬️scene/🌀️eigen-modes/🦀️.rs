//! 🌀️ Eigen modes of the EN 1994 scene from the shared mechanics kernel: the flexural buckling of a composite column and the lateral-torsional buckling of the bare steel girder, solved once per distinct member.

use super::checks::{catalogue, part_column};
use super::facts::{BeamFacts, ColumnFacts};
use super::mechanics as mech;

const STEEL_E: f64 = 210.0e9;
const STEEL_G: f64 = 81.0e9;
const MEMO_CAPACITY: usize = 512;
const AXIAL_STIFFNESS_RATIO: f64 = 1.0e6;

type Memo<T> = std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<Vec<u64>, T>>>;
type ColumnShape = Option<std::sync::Arc<(f64, Vec<(f64, [f64; 3])>)>>;
type GirderShape = Option<std::sync::Arc<(f64, Vec<(f64, f64)>)>>;

/// 🧠️ The solution of one eigen analysis for the bit patterns `key` of its mechanical inputs: the layout stamp asks every pass whether the document offers a mode, so an unchanged member is solved once and not once per drag step.
fn memoised<T: Clone>(table: &'static Memo<T>, key: Vec<u64>, solve: impl FnOnce() -> T) -> T {
    let memo = table.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    if let Some(known) = memo.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&key) {
        return known.clone();
    }
    let solved = solve();
    let mut known = memo.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if known.len() >= MEMO_CAPACITY {
        known.clear();
    }
    known.insert(key, solved.clone());
    solved
}

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|value| value.to_bits()).collect()
}

/// 🌀️ The critical load of a pinned composite column, the Euler load of its effective stiffness (EN 1994-1-1, 6.7.3.3), with the first eigenform. The member carries the effective stiffness as `E` over unit second moments; its torsion and axial stiffness are made so large that only the planar flexural mode is critical (the unit section has a polar radius of gyration of 1, so a free torsion mode would otherwise appear at `G·J/i_p²` and mask the buckling load).
pub fn column_buckling(f: &ColumnFacts) -> Option<(f64, Vec<(f64, [f64; 3])>)> {
    static TABLE: Memo<ColumnShape> = std::sync::OnceLock::new();
    let c = &f.column;
    let ei = part_column::ei_eff_nmm2(c, f.annex, catalogue::ecm_pa(c.concrete_f_ck_pa));
    let length = c.length_m.max(0.1);
    let shape = memoised(&TABLE, bits(&[length, ei]), || mech::member_modes::buckling(&mech::member_modes::Props { length, e: ei, g: ei, a: AXIAL_STIFFNESS_RATIO, iy: 1.0, iz: 1.0, j: 1.0 }, mech::member_modes::Ends::Pinned).ok().map(std::sync::Arc::new))?;
    Some(shape.as_ref().clone())
}

/// 🦋️ The critical moment of the bare steel girder under uniform moment between fork ends, braced every `ltb_length_m`, with the first eigenform as `(at, lateral)`.
pub fn girder_ltb(f: &BeamFacts) -> Option<(f64, Vec<(f64, f64)>)> {
    static TABLE: Memo<GirderShape> = std::sync::OnceLock::new();
    let s = &f.steel;
    let key = bits(&[f.l_total, s.height_m, s.width_m, s.tw_m, s.tf_m, s.i_y_m4, s.a_m2, f.beam.ltb_length_m]);
    memoised(&TABLE, key, || solve_girder(f).map(std::sync::Arc::new)).map(|shape| shape.as_ref().clone())
}

/// 🦋️ The eigen analysis of [`girder_ltb`] for one set of inputs.
fn solve_girder(f: &BeamFacts) -> Option<(f64, Vec<(f64, f64)>)> {
    let (s, l) = (&f.steel, f.l_total);
    let (h, b, tw, tf) = (s.height_m, s.width_m, s.tw_m, s.tf_m);
    let iz = 2.0 * tf * b.powi(3) / 12.0 + (h - 2.0 * tf) * tw.powi(3) / 12.0;
    let spacing = f.beam.ltb_length_m;
    let restraints = if spacing > 1e-6 && spacing < l - 1e-6 { (1..).map(|k| k as f64 * spacing).take_while(|x| *x < l - 1e-6).map(|x| mech::LtbRestraint { x, lateral: true, twist: true }).collect() } else { Vec::new() };
    let member = mech::LtbMember {
        length: l,
        e: STEEL_E,
        g: STEEL_G,
        iz,
        iw: iz * (h - tf).powi(2) / 4.0,
        j: (2.0 * b * tf.powi(3) + (h - 2.0 * tf) * tw.powi(3)) / 3.0,
        polar_radius_sq: (s.i_y_m4 + iz) / s.a_m2.max(1e-9),
        end_i: mech::ltb::fork_end(),
        end_j: mech::ltb::fork_end(),
        restraints,
        moment: mech::ltb::uniform_moment(1.0, l),
        elements: 32,
        ..Default::default()
    };
    let result = mech::ltb::ltb_analyze(&member, &mech::Control::none()).ok()?;
    let last = result.shape.len().saturating_sub(1);
    let shape = result.shape.iter().enumerate().filter(|(i, _)| i % 4 == 0 || *i == last).map(|(_, p)| (p.x / l, p.lateral + p.twist * h / 2.0)).collect();
    Some((result.mcr, shape))
}
