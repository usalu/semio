//! 🌉️ Composite beams and bridge girders: steel section, concrete slab, effective-width ghost, profiled sheeting, shear studs by zone with shear-flow colouring, reinforcement, plastic neutral axis plane, cracked zone, supports, loads and annotations.

use super::facts::{rib_centreline, BeamFacts};
use super::kit::*;
use super::layout::{Layout, Pose};
use super::norm;
use super::scene::*;
use super::statics::{uniform_load_line, Line};
use super::{handles, SceneInput};

const STUD_PREFIXES: [&str; 3] = ["en1994.6.6.", "en1994.2.6.8.3.stud.", "en1994.6.2.1.3."];
const PER_STUD_PREFIXES: [&str; 1] = ["en1994.6.6.2.perstud."];
pub(crate) const MAX_STUDS_DRAWN: usize = 2000;
const SLAB_PREFIXES: [&str; 3] = ["en1994.6.6.6.", "en1994.7.4.", "en1994.5.4.1.2."];
const SHEETING_PREFIXES: [&str; 2] = ["en1994.9.3.", "en1994.6.2.1.construction."];
const BAR_PREFIXES: [&str; 1] = ["en1994.7.4."];
const MAX_TRANSVERSE_BARS: f64 = 400.0;

pub(crate) fn local_frame(pose: &Pose, origin: (f64, f64, f64), x: (f64, f64, f64), y: (f64, f64, f64)) -> Frame {
    let (ex, ey) = (pose.dir(x.0, x.1, x.2), pose.dir(y.0, y.1, y.2));
    Frame { origin: pose.point(origin.0, origin.1, origin.2), z: ex.cross(&ey), x: ex, y: ey }
}

pub(crate) fn steel_shape(steel: &super::model::SteelSection) -> SectionShape {
    let (h, b) = (steel.height_m.max(1e-3), steel.width_m.max(1e-3));
    if steel.tw_m > 0.0 && steel.tf_m > 0.0 && 2.0 * steel.tf_m < h && steel.tw_m < b {
        SectionShape::parametric(ProfileKind::IShape, &[("h", h), ("b", b), ("tw", steel.tw_m), ("tf", steel.tf_m)])
    } else {
        SectionShape::parametric(ProfileKind::Rect, &[("h", h), ("b", b)])
    }
}

pub(crate) fn case_kind(kind: semio_s_artifact_norm_en1994_model::ActionKind) -> LoadKind {
    use semio_s_artifact_norm_en1994_model::ActionKind as K;
    match kind {
        K::Permanent => LoadKind::Permanent,
        K::Imposed | K::Construction => LoadKind::Variable,
        K::Snow => LoadKind::Snow,
        K::Wind => LoadKind::Wind,
        K::Fatigue => LoadKind::Other,
    }
}

pub(crate) fn along(path_from: &Vec3, direction: &Vec3, length: f64) -> Path3 {
    polyline(&[path_from.clone(), add(path_from, &mul(direction, length))])
}

fn bar_diameter(area_per_m: f64, spacing: f64) -> f64 {
    (4.0 * area_per_m * spacing / std::f64::consts::PI).sqrt().clamp(0.006, 0.025)
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for beam in &input.document.beams {
        if !out.wants(&beam.id) {
            continue;
        }
        let Some(pose) = layout.pose(&beam.id) else { continue };
        let facts = BeamFacts::of(input.document, beam);
        if facts.l_total <= 1e-6 || facts.steel.height_m <= 1e-6 {
            continue;
        }
        let line = uniform_load_line(&facts.layout, facts.stiffness);
        Builder { input, facts: &facts, pose, out, line }.run();
    }
}

struct Builder<'a, 'b> {
    input: &'a SceneInput<'a>,
    facts: &'a BeamFacts,
    pose: &'a Pose,
    out: &'b mut Out,
    line: Option<Line>,
}

impl Builder<'_, '_> {
    fn p(&self, x: f64, y: f64, z: f64) -> Vec3 {
        self.pose.point(x, y, z)
    }

    fn id(&self) -> &str {
        &self.facts.beam.id
    }

    fn subject_binding(&self, prefixes: &[&str]) -> Option<UtilisationBinding> {
        binding(self.input.report, self.id(), prefixes)
    }

    fn stations(&self) -> Vec<StationValue> {
        envelope(self.input.report.checks.iter().filter(|c| c.subject.entity_id == self.id()))
    }

    fn hogging_zones(&self) -> Vec<(f64, f64)> {
        self.line.as_ref().map(|l| l.hogging.iter().map(|&(a, b)| (a * self.facts.l_total, b * self.facts.l_total)).collect()).unwrap_or_default()
    }

    fn run(&mut self) {
        self.steel();
        self.slab();
        self.effective_width();
        self.sheeting();
        self.studs();
        self.reinforcement();
        self.neutral_axis();
        self.cracked_zone();
        self.supports();
        self.loads();
        self.annotations();
    }

    fn steel(&mut self) {
        let f = self.facts;
        let s = &f.steel;
        let stations = self.stations();
        let all = self.subject_binding(&[]).map(|b| UtilisationBinding { per_station: Some(stations.clone()).filter(|v| !v.is_empty()).or(b.per_station), ..b });
        let frame = local_frame(self.pose, (0.0, 0.0, -s.height_m / 2.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0));
        let end = self.p(f.l_total, 0.0, -s.height_m / 2.0);
        let (kind_en, kind_de) = if f.bridge { ("Bridge girder", "Brückenträger") } else { ("Composite beam", "Verbundträger") };
        let name = loc(&format!("{kind_en} {}", f.beam.id), &format!("{kind_de} {}", f.beam.id));
        let detail = loc(
            &format!("{}, span {:.2} m, {}, η = {:.2}", s.designation, f.beam.span_m, f.beam.support.as_str().replace('_', " "), f.eta),
            &format!("{}, Spannweite {:.2} m, {}, η = {:.2}", s.designation, f.beam.span_m, f.beam.support.as_str().replace('_', " "), f.eta),
        );
        let overlay = self.deflection();
        let geometry = Geometry::Member(MemberGeometry {
            axis: polyline(&[frame.origin.clone(), end.clone()]),
            frame: frame.clone(),
            section: steel_shape(s),
            section_taper: None,
            start_offset: Vec2::ZERO,
            end_offset: Vec2::ZERO,
            start_release: EndRelease::Rigid,
            end_release: EndRelease::Rigid,
            stations: stations.iter().map(|s| s.at).collect(),
        });
        let mut spec = Spec::new(EntityKind::Member, self.id().to_string(), "beams", geometry, MaterialRole::Steel, name, LAYER_STEEL).bound(all).detailed(detail);
        spec.handles = handles::ids(self.input.document, self.facts, self.pose, self.id());
        spec.deformation = overlay;
        self.out.push(spec, &[frame.origin, end]);
    }

    fn deflection(&self) -> Option<DeformationOverlay> {
        let f = self.facts;
        let delta = check_with(self.input.report, &format!("en1994.7.3.1.deflection.{}", f.beam.id)).map(|c| c.computed.value).filter(|d| d.is_finite() && *d > 1e-9)?;
        let down = self.pose.dir(0.0, 0.0, -1.0);
        let samples = self.line.as_ref()?.deflection.iter().map(|&(t, w)| DisplacementSample { at: t, delta: mul(&down, delta * w) }).collect();
        Some(DeformationOverlay { surface: Vec::new(), motion: if f.hogging { Motion::Ramp } else { Motion::Static }, load_factor: None, kind: DeformationKind::Deformed, index: 0, frequency: None, participation: 1.0, scale: (0.02 * f.l_total / delta).clamp(1.0, 500.0), samples })
    }

    fn slab(&mut self) {
        let f = self.facts;
        let w = f.slab_width();
        let frame = local_frame(self.pose, (0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0));
        let geometry = Geometry::Plate(PlateGeometry { frame, outline: rect(0.0, f.l_total, -w / 2.0, w / 2.0), thickness: f.hc, offset: f.hp + f.hc / 2.0, stiffeners: Vec::new() });
        let name = loc(&format!("Concrete slab of {}", f.beam.id), &format!("Betongurt von {}", f.beam.id));
        let detail = loc(
            &format!("h = {:.0} mm, h_p = {:.0} mm, f_ck = {:.0} MPa, b_eff = {:.2} m", f.depth * 1e3, f.hp * 1e3, f.beam.concrete_f_ck_pa / 1e6, f.b_eff),
            &format!("h = {:.0} mm, h_p = {:.0} mm, f_ck = {:.0} MPa, b_eff = {:.2} m", f.depth * 1e3, f.hp * 1e3, f.beam.concrete_f_ck_pa / 1e6, f.b_eff),
        );
        let top = self.p(f.l_total, w / 2.0, f.depth);
        let spec = Spec::new(EntityKind::Plate, format!("{}:slab", f.beam.id), "beams", geometry, MaterialRole::Concrete, name, LAYER_SLAB).bound(self.subject_binding(&SLAB_PREFIXES)).detailed(detail);
        self.out.push(spec, &[self.p(0.0, -w / 2.0, 0.0), top]);
    }

    fn effective_width(&mut self) {
        let f = self.facts;
        let frame = local_frame(self.pose, (0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0));
        let geometry = Geometry::Plate(PlateGeometry { frame, outline: rect(0.0, f.l_total, -f.b_eff / 2.0, f.b_eff / 2.0), thickness: 0.012, offset: f.depth + 0.006, stiffeners: Vec::new() });
        let name = loc(&format!("Effective width of {}", f.beam.id), &format!("Mitwirkende Breite von {}", f.beam.id));
        let detail = loc(
            &format!("b_eff = {:.3} m{}", f.b_eff, if f.b_eff_capped { ", capped by the beam spacing" } else { "" }),
            &format!("b_eff = {:.3} m{}", f.b_eff, if f.b_eff_capped { ", durch den Trägerabstand begrenzt" } else { "" }),
        );
        let spec = Spec::new(EntityKind::Plate, format!("{}:beff", f.beam.id), "beams", geometry, MaterialRole::Concrete, name, LAYER_ANALYSIS).detailed(detail).ghosted(0.35);
        self.out.push(spec, &[self.p(0.0, -f.b_eff / 2.0, f.depth), self.p(f.l_total, f.b_eff / 2.0, f.depth)]);
    }

    fn sheeting(&mut self) {
        let f = self.facts;
        let sh = &f.beam.sheeting;
        let w = f.slab_width();
        let parallel = sh.ribs_parallel_to_beam;
        let run = if parallel { w } else { f.l_total };
        let centre: Vec<(f64, f64)> = rib_centreline(sh, run);
        let t = sh.thickness_m.max(0.0006);
        let (frame, length, points) = if parallel {
            (local_frame(self.pose, (0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)), f.l_total, centre.iter().map(|(o, h)| v2(o - w / 2.0, *h)).collect::<Vec<_>>())
        } else {
            (local_frame(self.pose, (f.l_total / 2.0, -w / 2.0, 0.0), (0.0, 1.0, 0.0), (-1.0, 0.0, 0.0)), w, centre.iter().map(|(o, h)| v2(f.l_total / 2.0 - o, *h)).collect::<Vec<_>>())
        };
        let end = add(&frame.origin, &mul(&frame.x, length));
        let segments = points.len() - 1;
        let geometry = Geometry::Member(MemberGeometry {
            axis: polyline(&[frame.origin.clone(), end.clone()]),
            frame: frame.clone(),
            section: SectionShape::ThinWalled { centreline: Polyline2 { points }, thickness: vec![t; segments] },
            section_taper: None,
            start_offset: Vec2::ZERO,
            end_offset: Vec2::ZERO,
            start_release: EndRelease::Rigid,
            end_release: EndRelease::Rigid,
            stations: Vec::new(),
        });
        let name = loc(&format!("Profiled sheeting of {}", f.beam.id), &format!("Profilblech von {}", f.beam.id));
        let detail = loc(
            &format!("{} ribs {}, h_p = {:.0} mm, t = {:.2} mm", sh.profile, if parallel { "parallel to the beam" } else { "transverse to the beam" }, f.hp * 1e3, t * 1e3),
            &format!("{} Rippen {}, h_p = {:.0} mm, t = {:.2} mm", sh.profile, if parallel { "parallel zum Träger" } else { "quer zum Träger" }, f.hp * 1e3, t * 1e3),
        );
        let spec = Spec::new(EntityKind::Member, format!("{}:sheeting", f.beam.id), "beams", geometry, MaterialRole::Steel, name, LAYER_SHEETING).bound(self.subject_binding(&SHEETING_PREFIXES)).detailed(detail);
        self.out.push(spec, &[frame.origin, end]);
    }

    fn studs(&mut self) {
        let f = self.facts;
        let gov = self.subject_binding(&STUD_PREFIXES);
        let per = self.subject_binding(&PER_STUD_PREFIXES);
        let per_check = check_with(self.input.report, &format!("en1994.6.6.2.perstud.{}", f.beam.id));
        let first_share = f.flow_share(f.first_station_x()).max(1e-9);
        for (row, y) in f.studs.rows.iter().zip(f.stud_ys()).take(MAX_STUDS_DRAWN) {
            let d = row.diameter_m.max(0.006);
            let h_sc = row.height_m.clamp(0.03, (f.depth - 0.02).max(0.03));
            let from = self.p(row.x_m, y, 0.0);
            let top = self.p(row.x_m, y, h_sc);
            let geometry = Geometry::Fastener(FastenerGeometry {
                kind: FastenerKind::Stud { head_height: (0.4 * d).max(0.005), head_diameter: 1.5 * d },
                axis: Segment3 { from: from.clone(), to: top.clone() },
                diameter: d,
                head: HeadShape::Headed,
                pattern: None,
                hole_clearance: None,
            });
            let ratio = per_check.map(|c| field_along(c, &[(row.x_m / f.l_total.max(1e-9)).clamp(0.0, 1.0)])[0]);
            let bound = match (ratio, per.as_ref()) {
                (Some(u), Some(p)) => scaled(&per, (u / p.ratio.max(1e-9)).clamp(0.0, 1.0)),
                _ => scaled(&gov, (f.flow_share(row.x_m) / first_share).clamp(0.02, 1.0)),
            };
            let name = loc(&format!("Shear stud {} of {}", row.id, f.beam.id), &format!("Kopfbolzendübel {} von {}", row.id, f.beam.id));
            let detail = loc(
                &format!("x = {:.3} m, Ø{:.0} mm, h = {:.0} mm, {} per rib, k_t/k_l = {:.2}, P_Rd = {:.1} kN{}", row.x_m, d * 1e3, h_sc * 1e3, row.n_r, row.reduction, row.p_rd_n / 1e3, ratio.map_or_else(String::new, |u| format!(", utilisation {:.0} %", u * 100.0))),
                &format!("x = {:.3} m, Ø{:.0} mm, h = {:.0} mm, {} je Rippe, k_t/k_l = {:.2}, P_Rd = {:.1} kN{}", row.x_m, d * 1e3, h_sc * 1e3, row.n_r, row.reduction, row.p_rd_n / 1e3, ratio.map_or_else(String::new, |u| format!(", Ausnutzung {:.0} %", u * 100.0))),
            );
            let spec = Spec::new(EntityKind::Fastener, format!("{}:stud-{}", f.beam.id, row.id), "beams", geometry, MaterialRole::Steel, name, LAYER_CONNECTORS).bound(bound).detailed(detail);
            self.out.push(spec, &[from, top]);
        }
    }

    fn reinforcement(&mut self) {
        let f = self.facts;
        let w = f.slab_width();
        let spacing = f.beam.bar_spacing_m.max(0.05).max(f.l_total / MAX_TRANSVERSE_BARS);
        let z = (f.depth - 0.04).max(f.hp + 0.01);
        let binding = self.subject_binding(&BAR_PREFIXES);
        let d = bar_diameter(f.beam.transverse_as_m2_per_m, spacing);
        let count = ((f.l_total / spacing).floor() as usize).max(1);
        let from = self.p(spacing / 2.0, -w / 2.0 + 0.03, z);
        let geometry = Geometry::Fastener(FastenerGeometry {
            kind: FastenerKind::Rebar { bend: None },
            axis: Segment3 { from: from.clone(), to: self.p(spacing / 2.0, w / 2.0 - 0.03, z) },
            diameter: d,
            head: HeadShape::None,
            pattern: Some(Pattern::Along { path: along(&from, &self.pose.ex(), (count - 1) as f64 * spacing + 0.25 * spacing), spacing }),
            hole_clearance: None,
        });
        let name = loc(&format!("Transverse bars of {}", f.beam.id), &format!("Querbewehrung von {}", f.beam.id));
        let detail = loc(&format!("Ø{:.0} mm every {:.0} mm, A_s = {:.0} mm²/m", d * 1e3, spacing * 1e3, f.beam.transverse_as_m2_per_m * 1e6), &format!("Ø{:.0} mm alle {:.0} mm, A_s = {:.0} mm²/m", d * 1e3, spacing * 1e3, f.beam.transverse_as_m2_per_m * 1e6));
        let spec = Spec::new(EntityKind::Fastener, format!("{}:rebar-transverse", f.beam.id), "beams", geometry, MaterialRole::Rebar, name, LAYER_REINFORCEMENT).bound(binding.clone()).detailed(detail);
        self.out.push(spec, &[from, self.p(f.l_total, w / 2.0, z)]);
        if !f.hogging {
            return;
        }
        for (k, (x_a, x_b)) in self.hogging_zones().into_iter().enumerate() {
        let tag = if k == 0 { "rebar-hogging".to_string() } else { format!("rebar-hogging-{}", k + 1) };
        let (d_h, bars) = (bar_diameter(f.beam.as_hogging_m2_per_m, spacing), (((f.b_eff / spacing).floor() as usize) + 1).clamp(2, 40));
        let y0 = -((bars - 1) as f64 * spacing) / 2.0;
        let start = self.p(x_a, y0, z + 0.02);
        let geometry = Geometry::Fastener(FastenerGeometry {
            kind: FastenerKind::Rebar { bend: None },
            axis: Segment3 { from: start.clone(), to: self.p(x_b, y0, z + 0.02) },
            diameter: d_h,
            head: HeadShape::None,
            pattern: Some(Pattern::Along { path: along(&start, &self.pose.ey(), (bars - 1) as f64 * spacing + 0.25 * spacing), spacing }),
            hole_clearance: None,
        });
        let name = loc(&format!("Hogging bars of {}", f.beam.id), &format!("Stützbewehrung von {}", f.beam.id));
        let detail = loc(&format!("{bars} bars Ø{:.0} mm over the internal support, A_s = {:.0} mm²/m", d_h * 1e3, f.beam.as_hogging_m2_per_m * 1e6), &format!("{bars} Stäbe Ø{:.0} mm über dem Mittelauflager, A_s = {:.0} mm²/m", d_h * 1e3, f.beam.as_hogging_m2_per_m * 1e6));
        let spec = Spec::new(EntityKind::Fastener, format!("{}:{tag}", f.beam.id), "beams", geometry, MaterialRole::Rebar, name, LAYER_REINFORCEMENT).bound(binding.clone()).detailed(detail);
        self.out.push(spec, &[start, self.p(x_b, y0 + (bars - 1) as f64 * spacing, z + 0.02)]);
        }
    }

    fn plane(&self, z: f64, x0: f64, x1: f64, half: f64) -> Geometry {
        let ring = vec![self.p(x0, -half, z), self.p(x1, -half, z), self.p(x1, half, z), self.p(x0, half, z)];
        Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: Polygon3 { outer: Ring3 { points: ring }, holes: Vec::new() } } })
    }

    fn neutral_axis(&mut self) {
        let f = self.facts;
        let z = f.pna_z();
        let half = f.b_eff.max(f.steel.width_m + 0.2) / 2.0;
        let name = loc(&format!("Plastic neutral axis of {}", f.beam.id), &format!("Plastische Nulllinie von {}", f.beam.id));
        let in_steel = f.pna.z_s > 1e-9;
        let detail = loc(
            &format!("{} at {:.1} mm below the slab top, N_c = {:.0} kN, N_a = {:.0} kN, residual {:.2} kN", if in_steel { "In the steel section" } else { "In the slab" }, (f.depth - z) * 1e3, f.pna.n_c / 1e3, f.pna.n_a / 1e3, f.pna.residual / 1e3),
            &format!("{} bei {:.1} mm unter Plattenoberkante, N_c = {:.0} kN, N_a = {:.0} kN, Rest {:.2} kN", if in_steel { "Im Stahlquerschnitt" } else { "In der Platte" }, (f.depth - z) * 1e3, f.pna.n_c / 1e3, f.pna.n_a / 1e3, f.pna.residual / 1e3),
        );
        let spec = Spec::new(EntityKind::Surface, format!("{}:pna", f.beam.id), "beams", self.plane(z, 0.0, f.l_total, half), MaterialRole::Neutral, name, LAYER_ANALYSIS).detailed(detail).ghosted(0.3);
        self.out.push(spec, &[self.p(0.0, -half, z), self.p(f.l_total, half, z)]);
    }

    fn cracked_intervals(&self) -> Vec<(f64, f64)> {
        let f = self.facts;
        let fraction = f.stiffness.cracked_fraction;
        if fraction <= 1e-9 {
            return self.hogging_zones();
        }
        (1..f.layout.lengths.len()).map(|i| (f.layout.origin(i) - fraction * f.layout.lengths[i - 1], f.layout.origin(i) + fraction * f.layout.lengths[i])).collect()
    }

    fn cracked_zone(&mut self) {
        let f = self.facts;
        if !f.hogging {
            return;
        }
        let half = f.slab_width() / 2.0;
        for (k, (x_a, x_b)) in self.cracked_intervals().into_iter().enumerate() {
            let tag = if k == 0 { "cracked".to_string() } else { format!("cracked-{}", k + 1) };
            let name = loc(&format!("Cracked zone of {}", f.beam.id), &format!("Gerissene Zone von {}", f.beam.id));
            let detail = loc(
                &format!("Slab over the stiffness EI₂ of the cracked section: {:.0} % of the span each side of the internal support, or the contraflexure zone of the elastic analysis when the fraction is 0 (EN 1994-1-1, 5.4.2.3).", f.stiffness.cracked_fraction * 100.0),
                &format!("Platte über der Steifigkeit EI₂ des gerissenen Querschnitts: {:.0} % der Stützweite beidseits der Innenstütze, oder die Momentennullpunkte der elastischen Berechnung bei Anteil 0 (EN 1994-1-1, 5.4.2.3).", f.stiffness.cracked_fraction * 100.0),
            );
            let spec = Spec::new(EntityKind::Surface, format!("{}:{tag}", f.beam.id), "beams", self.plane(f.depth + 0.004, x_a, x_b, half), MaterialRole::Concrete, name, LAYER_ANALYSIS).bound(self.subject_binding(&BAR_PREFIXES)).detailed(detail).ghosted(0.5);
            self.out.push(spec, &[self.p(x_a, -half, f.depth), self.p(x_b, half, f.depth)]);
        }
    }

    fn support(&mut self, tag: &str, x: f64, kind: SupportKind, name: LocalizedCopy) {
        let at = self.p(x, 0.0, -self.facts.steel.height_m);
        let geometry = Geometry::Support(SupportGeometry { kind, frame: local_frame(self.pose, (x, 0.0, -self.facts.steel.height_m), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)), at: at.clone() });
        self.out.push(Spec::new(EntityKind::Support, format!("{}:{tag}", self.id()), "beams", geometry, MaterialRole::Support, name, LAYER_SUPPORTS), &[at]);
    }

    fn supports(&mut self) {
        let f = self.facts;
        let up = self.pose.dir(0.0, 0.0, 1.0);
        self.support("support-a", 0.0, SupportKind::Pinned, loc("Pinned support", "Gelenklager"));
        let xs = f.layout.supports();
        let mut interior = 0;
        for (k, x) in xs.iter().enumerate().skip(1) {
            let end = k + 1 == xs.len() && !f.layout.cantilever;
            let (tag, name) = if end {
                ("support-b".to_string(), loc("Roller support", "Rollenlager"))
            } else {
                interior += 1;
                (if interior == 1 { "support-m".to_string() } else { format!("support-m-{interior}") }, loc("Internal support", "Mittelauflager"))
            };
            self.support(&tag, *x, SupportKind::Roller { normal: up.clone() }, name);
        }
        let spacing = f.beam.ltb_length_m;
        if f.beam.construction == semio_s_artifact_norm_en1994_model::ConstructionMethod::Unpropped && spacing > 1e-6 && spacing < f.l_total - 1e-6 {
            let normal = self.pose.ey();
            for k in 1..=12 {
                let x = k as f64 * spacing;
                if x >= f.l_total - 1e-6 {
                    break;
                }
                let at = self.p(x, 0.0, 0.0);
                let geometry = Geometry::Support(SupportGeometry { kind: SupportKind::Roller { normal: normal.clone() }, frame: local_frame(self.pose, (x, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)), at: at.clone() });
                self.out.push(Spec::new(EntityKind::Support, format!("{}:restraint-{k}", f.beam.id), "beams", geometry, MaterialRole::Support, loc("Lateral restraint during construction", "Seitliche Halterung im Bauzustand"), LAYER_SUPPORTS), &[at]);
            }
        }
    }

    fn loads(&mut self) {
        let f = self.facts;
        let w = f.beam.spacing_m;
        for action in f.beam.actions.iter().filter(|a| a.kind != semio_s_artifact_norm_en1994_model::ActionKind::Fatigue) {
            let kind = case_kind(action.kind);
            let case = LoadCaseRef { id: action.id.clone(), kind, psi: None };
            let top = f.depth + 0.02;
            let (geometry, at, label) = if action.q_area_pa.abs() > 1e-9 {
                let ring = vec![self.p(0.0, -w / 2.0, top), self.p(f.l_total, -w / 2.0, top), self.p(f.l_total, w / 2.0, top), self.p(0.0, w / 2.0, top)];
                (
                    Geometry::Load(LoadGlyph::Area { polygon: Polygon3 { outer: Ring3 { points: ring }, holes: Vec::new() }, direction: mul(&Vec3::Z, -1.0), intensity: Quantity::new(QuantityKind::Pressure, action.q_area_pa.abs()), case }),
                    self.p(f.l_total / 2.0, 0.0, top),
                    loc(&format!("{} {:.2} kN/m² ({})", action.id, action.q_area_pa / 1e3, action.stage), &format!("{} {:.2} kN/m² ({})", action.id, action.q_area_pa / 1e3, action.stage)),
                )
            } else if action.f_k_n.abs() > 1.0 {
                let at = self.p(f.point_x(action), 0.0, top);
                (
                    Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: mul(&Vec3::Z, -1.0), magnitude: Quantity::new(QuantityKind::Force, action.f_k_n.abs()), case }),
                    at,
                    loc(&format!("{} {:.1} kN ({})", action.id, action.f_k_n / 1e3, action.stage), &format!("{} {:.1} kN ({})", action.id, action.f_k_n / 1e3, action.stage)),
                )
            } else {
                continue;
            };
            self.out.push(Spec::new(EntityKind::Load, format!("{}:load-{}", f.beam.id, action.id), "beams", geometry, MaterialRole::Load { kind }, label, LAYER_LOADS), &[at]);
        }
    }

    fn failure_glyphs(&mut self) {
        let f = self.facts;
        let entity = entity_ref(EntityKind::Member, &f.beam.id, "beams");
        let at = self.p(f.l_total / 2.0, 0.0, f.depth / 2.0);
        for (prefix, mode, tag) in [("en1994.6.4.ltb.", FailureMode::LateralTorsionalBuckling, "ltb"), ("en1994.6.2.1.3.mrd.", FailureMode::Yielding, "bending"), ("en1994.6.6.", FailureMode::Other, "connection"), ("en1994.2.", FailureMode::Fatigue, "fatigue"), ("en1994.7.4.", FailureMode::Other, "crack"), ("en1994.3.materials.", FailureMode::Other, "materials")] {
            if worst(self.input.report, self.id(), &[prefix]).is_some_and(|c| matches!(c.status, norm::CheckStatus::Fail | norm::CheckStatus::Warning)) {
                self.out.annotate(Annotation::FailureGlyph { id: format!("{}:failure-{tag}", f.beam.id), at: entity.clone(), mode, extent: None }, &at);
            }
        }
    }

    fn annotations(&mut self) {
        let f = self.facts;
        let w = f.slab_width();
        let binding = self.subject_binding(&[]);
        let mid = f.l_total / 2.0;
        let tag = if f.bridge { loc(&format!("{} bridge girder {}", f.beam.id, f.steel.designation), &format!("{} Brückenträger {}", f.beam.id, f.steel.designation)) } else { loc(&format!("{} {}", f.beam.id, f.steel.designation), &format!("{} {}", f.beam.id, f.steel.designation)) };
        let label_at = self.p(mid, w / 2.0, f.depth + 0.3);
        self.out.annotate(Annotation::Label { id: format!("{}:label", f.beam.id), anchor: label_at.clone(), text: tag, tone: tone_of(&binding) }, &label_at);
        let below = -f.steel.height_m - 0.5;
        let span_from = self.p(0.0, 0.0, below);
        self.out.annotate(
            Annotation::Dimension { id: format!("{}:span", f.beam.id), from: span_from.clone(), to: self.p(f.beam.span_m, 0.0, below), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, f.beam.span_m), binding: Some(FieldBinding::new(format!("beams[id={}].spanM", f.beam.id), Affine::IDENTITY, "m")) },
            &span_from,
        );
        let top = f.depth + 0.1;
        let (beff_path, beff_map) = if f.b_eff_capped { (format!("beams[id={}].spacingM", f.beam.id), Affine::IDENTITY) } else { (format!("beams[id={}].spanM", f.beam.id), Affine { base: -4.0 * f.steel.width_m, k: 4.0 }) };
        let beff_from = self.p(mid, -f.b_eff / 2.0, top);
        self.out.annotate(
            Annotation::Dimension { id: format!("{}:beff", f.beam.id), from: beff_from.clone(), to: self.p(mid, f.b_eff / 2.0, top), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, f.b_eff), binding: Some(FieldBinding::new(beff_path, beff_map, "m")) },
            &beff_from,
        );
        let edge = self.p(0.0, -w / 2.0 - 0.05, 0.0);
        self.out.annotate(
            Annotation::Dimension { id: format!("{}:slab-depth", f.beam.id), from: edge.clone(), to: self.p(0.0, -w / 2.0 - 0.05, f.depth), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, f.depth), binding: Some(FieldBinding::new(format!("beams[id={}].slabThicknessM", f.beam.id), Affine::IDENTITY, "m")) },
            &edge,
        );
        if let [first, second, ..] = f.station_heads()[..] {
            let stud_from = self.p(first.x_m, -f.steel.width_m / 2.0 - 0.1, 0.0);
            self.out.annotate(
                Annotation::Dimension { id: format!("{}:stud-spacing", f.beam.id), from: stud_from.clone(), to: self.p(second.x_m, -f.steel.width_m / 2.0 - 0.1, 0.0), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, second.x_m - first.x_m), binding: Some(FieldBinding::new(format!("studs[id={}].xM", second.id), Affine { base: first.x_m, k: 1.0 }, "m")) },
                &stud_from,
            );
        }
        let half = f.l_total / 2.0;
        let eta_from = self.p(0.0, f.steel.width_m / 2.0 + 0.3, f.stud_top());
        self.out.annotate(
            Annotation::Dimension { id: format!("{}:eta", f.beam.id), from: eta_from.clone(), to: self.p(f.eta.clamp(0.0, 1.0) * half, f.steel.width_m / 2.0 + 0.3, f.stud_top()), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, f.eta.clamp(0.0, 1.0) * half), binding: None },
            &eta_from,
        );
        let min_from = self.p(0.0, f.steel.width_m / 2.0 + 0.45, f.stud_top());
        self.out.annotate(
            Annotation::Dimension { id: format!("{}:eta-min", f.beam.id), from: min_from.clone(), to: self.p(f.eta_min.clamp(0.0, 1.0) * half, f.steel.width_m / 2.0 + 0.45, f.stud_top()), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, f.eta_min.clamp(0.0, 1.0) * half), binding: None },
            &min_from,
        );
        if let Some((x_from, x_to)) = f.neighbour_interval() {
            let from = self.p(x_from, 0.0, below - 0.25);
            self.out.annotate(
                Annotation::Dimension { id: format!("{}:adjacent-span", f.beam.id), from: from.clone(), to: self.p(x_to, 0.0, below - 0.25), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, f.beam.neighbour_span_m()), binding: Some(FieldBinding::new(format!("beams[id={}].adjacentSpanM", f.beam.id), Affine::IDENTITY, "m")) },
                &from,
            );
        }
        if f.beam.actions.iter().any(|a| a.kind != semio_s_artifact_norm_en1994_model::ActionKind::Fatigue && a.f_k_n.abs() > 1e-9) {
            let ss = if f.beam.patch_bearing_length_m > 1e-9 { f.beam.patch_bearing_length_m } else { 0.1 };
            let from = self.p(mid - ss / 2.0, 0.0, -f.steel.height_m - 0.2);
            self.out.annotate(
                Annotation::Dimension { id: format!("{}:patch-bearing", f.beam.id), from: from.clone(), to: self.p(mid + ss / 2.0, 0.0, -f.steel.height_m - 0.2), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, ss), binding: Some(FieldBinding::new(format!("beams[id={}].patchBearingLengthM", f.beam.id), Affine::IDENTITY, "m")) },
                &from,
            );
        }
        if f.bridge {
            let lanes = if f.beam.bridge_lanes > 0 { f.beam.bridge_lanes } else { ((f.beam.spacing_m / 3.0).floor() as u32).clamp(1, 3) };
            let width = 3.0 * f64::from(lanes);
            let from = self.p(mid, -width / 2.0, top + 0.05);
            self.out.annotate(
                Annotation::Dimension { id: format!("{}:lanes", f.beam.id), from: from.clone(), to: self.p(mid, width / 2.0, top + 0.05), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, width), binding: Some(FieldBinding::new(format!("beams[id={}].bridgeLanes", f.beam.id), Affine { base: 0.0, k: 1.0 / 3.0 }, "m")) },
                &from,
            );
        }
        let pna_at = self.p(0.9 * f.l_total, w / 2.0, f.pna_z());
        self.out.annotate(
            Annotation::Label { id: format!("{}:pna-label", f.beam.id), anchor: pna_at.clone(), text: loc(&format!("Plastic neutral axis {:.0} mm below slab top", (f.depth - f.pna_z()) * 1e3), &format!("Plastische Nulllinie {:.0} mm unter Plattenoberkante", (f.depth - f.pna_z()) * 1e3)), tone: Tone::Info },
            &pna_at,
        );
        let eta_at = self.p(f.first_station_x(), w / 2.0, f.depth + 0.15);
        let eta_tone = if f.eta < f.eta_min * (1.0 - norm::UTILIZATION_TOLERANCE) { Tone::Error } else { Tone::Info };
        self.out.annotate(
            Annotation::Label { id: format!("{}:eta-label", f.beam.id), anchor: eta_at.clone(), text: loc(&format!("Shear connection η = {:.2} (η_min = {:.2}), {} of {} studs", f.eta, f.eta_min, f.n_f, f.n_req), &format!("Verdübelungsgrad η = {:.2} (η_min = {:.2}), {} von {} Dübeln", f.eta, f.eta_min, f.n_f, f.n_req)), tone: eta_tone },
            &eta_at,
        );
        if let Some(clause) = binding.as_ref().and_then(|b| clause_of(self.input.report, &b.check_id)) {
            let anchor = entity_ref(EntityKind::Member, &f.beam.id, "beams");
            let at = self.p(mid, 0.0, -f.steel.height_m / 2.0);
            self.out.annotate(Annotation::Clause { id: format!("{}:clause", f.beam.id), anchor, clause }, &at);
        }
        self.failure_glyphs();
    }
}
