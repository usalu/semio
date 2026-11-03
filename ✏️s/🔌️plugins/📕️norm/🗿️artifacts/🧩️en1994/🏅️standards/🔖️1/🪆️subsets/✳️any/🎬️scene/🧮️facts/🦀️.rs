//! 🧮️ Facts: every derived number the 3D scene, the diagrams and the handles of one member share, computed once from the snapshot with the same check helpers the report uses.

use super::checks::catalogue::continuous::{Layout, Stiffness};
use super::checks::catalogue as cat;
use super::checks::{part_1_1, part_column, part_en1990, part_studs, AnnexParams};
use super::model::{CharacteristicAction, CompositeBeam, CompositeColumn, CompositeSlab, En1994Snapshot, ProfiledSheeting, SlabAction, SteelSection};
use super::norm::AnnexChoice;

pub const ZONES: usize = 6;
pub const MAX_RIBS: usize = 220;

/// 🧩️ The plastic neutral axis of a composite section under sagging bending (EN 1994-1-1, 6.2.1.2 and 6.2.1.3).
#[derive(Clone, Debug, PartialEq)]
pub struct Pna {
    pub n_a: f64,
    pub n_c_full: f64,
    pub n_c: f64,
    pub x_c: f64,
    pub a_comp: f64,
    pub z_s: f64,
    pub residual: f64,
}

fn plates_area(s: &SteelSection) -> f64 {
    2.0 * s.width_m * s.tf_m + (s.height_m - 2.0 * s.tf_m).max(0.0) * s.tw_m
}

/// 📐️ Steel area above depth `z` below the top flange face, from the plate model of the I section.
pub fn area_above(s: &SteelSection, z: f64) -> f64 {
    let z = z.clamp(0.0, s.height_m);
    let flange = z.min(s.tf_m) * s.width_m;
    let web = (z.min(s.height_m - s.tf_m) - s.tf_m).max(0.0) * s.tw_m;
    let bottom = (z - (s.height_m - s.tf_m)).max(0.0) * s.width_m;
    flange + web + bottom
}

fn depth_for_area(s: &SteelSection, area: f64) -> f64 {
    let flange = s.width_m * s.tf_m;
    if area <= flange {
        area / s.width_m.max(1e-9)
    } else {
        (s.tf_m + (area - flange) / s.tw_m.max(1e-9)).min(s.height_m - s.tf_m)
    }
}

/// 🧩️ PNA by force equilibrium: `N_c = η·N_cf`, `C_a = (N_a − N_c)/2`; `fcd85` is `0.85·f_cd`.
pub fn plastic_neutral_axis(s: &SteelSection, fyd: f64, fcd85: f64, b_eff: f64, hc: f64, eta: f64) -> Pna {
    let a = plates_area(s);
    let n_a = a * fyd;
    let n_c_full = fcd85 * b_eff * hc;
    let n_c = eta.clamp(0.0, 1.0) * n_a.min(n_c_full);
    let x_c = n_c / (fcd85 * b_eff).max(1e-9);
    let a_comp = ((a - n_c / fyd) / 2.0).max(0.0);
    let z_s = if a_comp <= 1e-12 { 0.0 } else { depth_for_area(s, a_comp) };
    let residual = n_c + fyd * (area_above(s, z_s) - (a - area_above(s, z_s)));
    Pna { n_a, n_c_full, n_c, x_c, a_comp, z_s, residual }
}

/// 🌉️ Everything one composite beam projection reads.
#[derive(Clone, Debug)]
pub struct BeamFacts {
    pub beam: CompositeBeam,
    pub annex: AnnexChoice,
    pub reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass,
    pub steel: SteelSection,
    pub l_total: f64,
    pub continuous: bool,
    pub hogging: bool,
    pub layout: Layout,
    pub stiffness: Stiffness,
    pub depth: f64,
    pub hp: f64,
    pub hc: f64,
    pub b_eff: f64,
    pub b_eff_capped: bool,
    pub fy: f64,
    pub params: AnnexParams,
    pub p_rd: f64,
    pub n_req: u32,
    pub n_f: u32,
    pub eta: f64,
    pub eta_min: f64,
    pub m_pla: f64,
    pub m_pl_rd: f64,
    pub m_rd: f64,
    pub m_ed: f64,
    pub pna: Pna,
    pub studs: part_studs::BeamStuds,
    pub bridge: bool,
}

impl BeamFacts {
    /// 📍️ The positions along the beam of the stud stations inside the designed span, ascending (studs at one position form one station).
    pub fn station_xs(&self) -> Vec<f64> {
        self.station_heads().iter().map(|r| r.x_m).collect()
    }

    /// 🔩️ The first stud of every stud station inside the designed span, ascending.
    pub fn station_heads(&self) -> Vec<&part_studs::StudResistance> {
        let mut out: Vec<&part_studs::StudResistance> = Vec::new();
        for row in self.studs.in_span() {
            if out.last().is_none_or(|last| last.station != row.station) {
                out.push(row);
            }
        }
        out
    }

    /// 📏️ The mean distance between the stud stations (the designed span without two stations).
    pub fn station_pitch(&self) -> f64 {
        let xs = self.station_xs();
        if xs.len() < 2 { self.beam.span_m } else { (xs[xs.len() - 1] - xs[0]) / (xs.len() - 1) as f64 }
    }

    /// ↔️ The transverse offset `y` of every stud in `studs.rows` order: the studs of one station spread over the flange symmetrically about the beam axis.
    pub fn stud_ys(&self) -> Vec<f64> {
        let mut lane = 0_usize;
        let mut last = usize::MAX;
        let width = self.steel.width_m;
        self.studs
            .rows
            .iter()
            .map(|row| {
                lane = if row.station == last { lane + 1 } else { 0 };
                last = row.station;
                let n_r = f64::from(row.n_r.max(1));
                let pitch = (0.07_f64).min((width - 2.0 * row.diameter_m.max(0.006)).max(0.0) / (n_r - 1.0).max(1.0)).max(row.diameter_m.max(0.006) * 2.0);
                (lane as f64 - (n_r - 1.0) / 2.0) * pitch
            })
            .collect()
    }

    /// 📍️ The first stud station, or the middle of the designed span when the beam has no stud.
    pub fn first_station_x(&self) -> f64 {
        self.station_xs().first().copied().unwrap_or_else(|| self.studs.mid_m())
    }

    /// ↕️ The height of the studs [m]: the shortest stud inside the span, the slab depth without studs.
    pub fn stud_top(&self) -> f64 {
        let h = self.studs.min_height_m();
        if h > 0.0 { h } else { self.depth }
    }
}

impl BeamFacts {
    /// 📍️ Position along the beam of a point force `x_m` from the first support of the designed span, clamped to that span (unset: the middle of the designed span).
    pub fn point_x(&self, action: &CharacteristicAction) -> f64 {
        self.layout.origin(self.layout.own) + if action.x_m > 0.0 { action.x_m.clamp(0.0, self.beam.span_m) } else { self.beam.span_m / 2.0 }
    }
}

impl BeamFacts {
    pub fn of(document: &En1994Snapshot, beam: &CompositeBeam) -> Self {
        let annex = document.annex;
        let fy = document.steel_f_y_pa;
        let mut resolved = beam.clone();
        resolved.steel = SteelSection::resolve(&beam.steel);
        let steel = resolved.steel.clone();
        let params = AnnexParams::for_annex(annex);
        let b_eff = part_1_1::effective_width_m(beam.span_m, steel.width_m, beam.spacing_m);
        let b_uncapped = part_1_1::uncapped_effective_width_m(beam.span_m, steel.width_m);
        let continuous = beam.is_continuous();
        let hp = beam.sheeting.height_m.max(0.0);
        let hc = (beam.slab_thickness_m - hp).max(0.04);
        let m_pla = part_1_1::steel_mpl_from_plates_nm(&steel, fy, params.gamma_m0).max(part_1_1::steel_plastic_moment_nm(steel.w_pl_y_m3, fy, params.gamma_m0));
        let m_pl_rd = part_1_1::full_plastic_moment_nm(&resolved, b_eff, fy, annex);
        let studs = part_studs::of(&resolved, &document.studs, annex);
        let n_req = part_1_1::n_f_req(&resolved, &studs, b_eff, fy, annex);
        let n_f = studs.half_count().floor() as u32;
        let eta = part_1_1::declared_shear_connection_degree(&studs, part_1_1::n_cf_n(&resolved, b_eff, fy, annex));
        let pna = plastic_neutral_axis(&steel, fy / params.gamma_m0, 0.85 * beam.concrete_f_ck_pa / params.gamma_c, b_eff, hc, eta);
        let layout = Layout::of(cat::span_position(beam.support), beam.span_m, beam.adjacent_span_m);
        let l_total = layout.total();
        let section = part_1_1::isection(&resolved.steel);
        let ei1 = cat::constants::E_A * cat::transformed(&section, beam.slab_thickness_m, b_eff, hc, cat::modular_ratio(beam.concrete_e_cm_pa, 0.0, 0.0)).i;
        let ei2 = cat::constants::E_A * cat::cracked(&section, beam.slab_thickness_m, beam.as_hogging_m2_per_m * b_eff, cat::constants::HOGGING_REBAR_DEPTH_M).i;
        let stiffness = Stiffness { ei1, ei2: ei2.min(ei1), cracked_fraction: beam.cracked_length_fraction.clamp(0.0, 0.5) };
        Self {
            annex,
            reliability_class: document.reliability_class,
            steel,
            l_total,
            continuous,
            hogging: beam.has_hogging(),
            layout,
            stiffness,
            depth: beam.slab_thickness_m,
            hp,
            hc,
            b_eff,
            b_eff_capped: b_uncapped > beam.spacing_m,
            fy,
            p_rd: part_1_1::connector_resistance_n(&studs),
            n_req,
            n_f,
            eta,
            eta_min: part_1_1::min_shear_connection_degree(beam.span_m, fy),
            m_pla,
            m_pl_rd,
            m_rd: part_1_1::plastic_moment_partial_nm(m_pla, m_pl_rd, eta),
            m_ed: part_en1990::uls_composite(&beam.actions, beam.span_m, beam.adjacent_span_m, beam.support, beam.spacing_m, annex, document.reliability_class).m_nm,
            pna,
            studs,
            bridge: document.structure_kind == semio_s_artifact_norm_en1994_model::StructureKind::Bridge,
            params,
            beam: resolved,
        }
    }

    pub fn slab_width(&self) -> f64 {
        self.beam.spacing_m.max(self.b_eff).max(self.steel.width_m)
    }

    /// 🧭️ PNA elevation in the local frame (`z = 0` is the top flange face, the slab top is `z = depth`).
    pub fn pna_z(&self) -> f64 {
        if self.pna.z_s > 1e-9 { -self.pna.z_s } else { self.depth - self.pna.x_c }
    }

    /// 🧭️ Span-local position `0..1` of a point at `x` along the beam.
    pub fn span_fraction(&self, x: f64) -> f64 {
        let (index, local) = self.layout.locate(x);
        (local / self.layout.lengths[index].max(1e-9)).clamp(0.0, 1.0)
    }

    /// 📏️ Beam interval `(from, to)` of the neighbouring span next to the designed span, if any.
    pub fn neighbour_interval(&self) -> Option<(f64, f64)> {
        match self.layout.lengths.len() {
            1 => None,
            _ if self.layout.own == 0 => Some((self.layout.lengths[0], self.l_total)),
            _ => Some((0.0, self.layout.lengths[0])),
        }
    }

    /// 🌊️ Elastic shear-flow share of a point: `|1 − 2u|` of its span-local position `u`.
    pub fn flow_share(&self, x: f64) -> f64 {
        (1.0 - 2.0 * self.span_fraction(x)).abs()
    }
}

/// 〰️ Profile geometry derived from the model's rib height and mean rib width (schematic pitch): `(pitch, crest, bottom, web run)`.
pub fn rib_profile(sheeting: &ProfiledSheeting) -> (f64, f64, f64, f64) {
    let hp = sheeting.height_m.max(0.02);
    let b0 = sheeting.rib_width_m.max(0.02);
    let e = (0.35 * hp).min(0.4 * b0);
    let bottom = b0 - e;
    let pitch = (2.0 * b0 + e).max(0.1);
    let crest = pitch - bottom - 2.0 * e;
    (pitch, crest, bottom, e)
}

/// 〰️ Sawtooth centreline of a trapezoidal profile of exactly `length` as `(offset, height)` pairs; the schematic pitch is stretched to fit a whole number of ribs, at most [`MAX_RIBS`].
pub fn rib_centreline(sheeting: &ProfiledSheeting, length: f64) -> Vec<(f64, f64)> {
    let hp = sheeting.height_m.max(0.02);
    let (pitch, crest, bottom, e) = rib_profile(sheeting);
    let count = ((length / pitch).round() as usize).clamp(1, MAX_RIBS);
    let k = length / (count as f64 * pitch);
    let (pitch, crest, bottom, e) = (pitch * k, crest * k, bottom * k, e * k);
    let mut points = vec![(0.0, hp)];
    for i in 0..count {
        let x = i as f64 * pitch;
        points.push((x + crest, hp));
        points.push((x + crest + e, 0.0));
        points.push((x + crest + e + bottom, 0.0));
        points.push((x + pitch, hp));
    }
    points
}

/// 🏛️ The drawn cross-section family of a composite column, derived from the model's outer size, wall and areas.
#[derive(Clone, Debug, PartialEq)]
pub enum ColumnShape {
    Chs { d: f64, t: f64 },
    Rhs { side: f64, t: f64 },
    Encased { side: f64, h: f64, b: f64, tf: f64, tw: f64 },
    Partial { h: f64, b: f64, tf: f64, tw: f64 },
}

fn i_flange(h: f64, area: f64) -> (f64, f64) {
    let disc = 6.76 * h * h - 4.8 * area;
    let tf = if disc >= 0.0 { (2.6 * h - disc.sqrt()) / 2.4 } else { area / (2.6 * h) };
    let tf = tf.clamp(0.004, 0.15 * h);
    (tf, (0.6 * tf).max(0.003))
}

impl ColumnShape {
    pub fn of(column: &CompositeColumn) -> Self {
        let outer = column.outer_size_m.max(0.05);
        let t = column.wall_thickness_m.clamp(0.002, outer / 4.0);
        match column.kind {
            semio_s_artifact_norm_en1994_model::ColumnKind::Encased => {
                let h = (0.5 * outer).max(0.1);
                let (tf, tw) = i_flange(h, column.steel_a_m2);
                Self::Encased { side: outer, h, b: h, tf, tw }
            }
            semio_s_artifact_norm_en1994_model::ColumnKind::PartiallyEncased => {
                let (tf, tw) = i_flange(outer, column.steel_a_m2);
                Self::Partial { h: outer, b: outer, tf, tw }
            }
            _ => {
                let chs = std::f64::consts::PI * t * (outer - t);
                let rhs = 4.0 * t * (outer - t);
                if (column.steel_a_m2 - chs).abs() <= (column.steel_a_m2 - rhs).abs() { Self::Chs { d: outer, t } } else { Self::Rhs { side: outer, t } }
            }
        }
    }

    pub fn outer_width(&self) -> f64 {
        match self {
            Self::Chs { d, .. } => *d,
            Self::Rhs { side, .. } | Self::Encased { side, .. } => *side,
            Self::Partial { b, .. } => *b,
        }
    }

    pub fn outer_depth(&self) -> f64 {
        match self {
            Self::Chs { d, .. } => *d,
            Self::Rhs { side, .. } | Self::Encased { side, .. } => *side,
            Self::Partial { h, .. } => *h,
        }
    }
}

/// 🏛️ Reinforcement bar count and diameter of a column: 20 mm bars in multiples of four, 4 to 16.
pub fn column_bars(column: &CompositeColumn) -> (usize, f64) {
    let bar = 0.020;
    let n = (column.reinforcement_as_m2 / (std::f64::consts::PI * bar * bar / 4.0)).ceil() as usize;
    (n.clamp(4, 16).div_ceil(4) * 4, bar)
}

/// 🏛️ Design effects of one column and its polygon.
#[derive(Clone, Debug)]
pub struct ColumnFacts {
    pub column: CompositeColumn,
    pub annex: AnnexChoice,
    pub reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass,
    pub shape: ColumnShape,
    pub n_ed: f64,
    pub m_ed: f64,
    pub polygon: [part_column::NmPoint; 4],
    pub alpha_m: f64,
    pub lambda: f64,
    pub chi: f64,
}

impl ColumnFacts {
    pub fn of(document: &En1994Snapshot, column: &CompositeColumn) -> Self {
        let annex = document.annex;
        let effects = part_en1990::uls_column(&column.actions, annex, document.reliability_class);
        Self {
            column: column.clone(),
            annex,
            reliability_class: document.reliability_class,
            shape: ColumnShape::of(column),
            n_ed: effects.n_n,
            m_ed: effects.m_nm,
            polygon: part_column::interaction_polygon(column, annex),
            alpha_m: part_column::alpha_m(column.steel_f_y_pa),
            lambda: part_column::relative_slenderness(column, annex),
            chi: part_column::chi(column, annex),
        }
    }
}

/// 🧱️ Slab strip facts.
#[derive(Clone, Debug)]
pub struct SlabFacts {
    pub slab: CompositeSlab,
    pub annex: AnnexChoice,
    pub reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass,
    pub l_total: f64,
    pub continuous: bool,
    pub hp: f64,
    pub hc: f64,
}

impl SlabFacts {
    /// 📍️ Position along the strip of a point force `x_m`, clamped to the span (unset: the middle of the whole length).
    pub fn point_x(&self, action: &SlabAction) -> f64 {
        if action.x_m > 0.0 { action.x_m.clamp(0.0, self.slab.span_m) } else { self.l_total / 2.0 }
    }

    /// 📍️ Position across the strip of a point force `y_m` from the strip axis, clamped to the strip width.
    pub fn point_y(&self, action: &SlabAction, width_m: f64) -> f64 {
        action.y_m.clamp(-width_m / 2.0, width_m / 2.0)
    }
}

impl SlabFacts {
    pub fn of(document: &En1994Snapshot, slab: &CompositeSlab) -> Self {
        let continuous = slab.support == semio_s_artifact_norm_en1994_model::SupportCondition::Continuous2Span;
        let hp = slab.sheeting.height_m.max(0.0);
        Self { slab: slab.clone(), annex: document.annex, reliability_class: document.reliability_class, l_total: if continuous { 2.0 * slab.span_m } else { slab.span_m }, continuous, hp, hc: (slab.concrete_thickness_m - hp).max(0.04) }
    }
}
