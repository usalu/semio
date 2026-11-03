//! 🧱️ Composite slabs on profiled sheeting: a one-metre strip with concrete topping, sheeting, transverse bars, supports, loads and dimensions.

use super::beams::{case_kind, local_frame};
use super::facts::{rib_centreline, SlabFacts};
use super::kit::*;
use super::layout::{Layout, Pose, STRIP_WIDTH};
use super::norm;
use super::scene::*;
use super::{handles, SceneInput};

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for slab in &input.document.slabs {
        if !out.wants(&slab.id) {
            continue;
        }
        let Some(pose) = layout.pose(&slab.id) else { continue };
        if slab.span_m <= 1e-6 || slab.concrete_thickness_m <= 1e-6 {
            continue;
        }
        let facts = SlabFacts::of(input.document, slab);
        Builder { input, facts: &facts, pose, out }.run();
    }
}

struct Builder<'a, 'b> {
    input: &'a SceneInput<'a>,
    facts: &'a SlabFacts,
    pose: &'a Pose,
    out: &'b mut Out,
}

impl Builder<'_, '_> {
    fn id(&self) -> &str {
        &self.facts.slab.id
    }

    fn p(&self, x: f64, y: f64, z: f64) -> Vec3 {
        self.pose.point(x, y, z)
    }

    fn run(&mut self) {
        let f = self.facts;
        let s = &f.slab;
        let w = STRIP_WIDTH;
        let all = binding(self.input.report, self.id(), &[]);
        let frame = local_frame(self.pose, (0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0));
        let geometry = Geometry::Plate(PlateGeometry { frame, outline: rect(0.0, f.l_total, -w / 2.0, w / 2.0), thickness: f.hc, offset: f.hp + f.hc / 2.0, stiffeners: Vec::new() });
        let detail = loc(
            &format!("span {:.2} m, h = {:.0} mm, h_p = {:.0} mm, f_ck = {:.0} MPa, m = {:.0}, k = {:.3}", s.span_m, s.concrete_thickness_m * 1e3, f.hp * 1e3, s.f_ck_pa / 1e6, s.m_factor, s.k_factor),
            &format!("Spannweite {:.2} m, h = {:.0} mm, h_p = {:.0} mm, f_ck = {:.0} MPa, m = {:.0}, k = {:.3}", s.span_m, s.concrete_thickness_m * 1e3, f.hp * 1e3, s.f_ck_pa / 1e6, s.m_factor, s.k_factor),
        );
        let mut slab = Spec::new(EntityKind::Plate, s.id.clone(), "slabs", geometry, MaterialRole::Concrete, loc(&format!("Composite slab {}", s.id), &format!("Verbunddecke {}", s.id)), LAYER_SLAB).bound(all.clone()).detailed(detail);
        slab.handles = handles::slab_ids(self.facts, self.pose, self.id());
        self.out.push(slab, &[self.p(0.0, -w / 2.0, 0.0), self.p(f.l_total, w / 2.0, s.concrete_thickness_m)]);
        self.sheeting();
        self.bars();
        self.supports();
        self.loads();
        self.annotations(&all);
    }

    fn sheeting(&mut self) {
        let f = self.facts;
        let s = &f.slab;
        let w = STRIP_WIDTH;
        let parallel = s.sheeting.ribs_parallel_to_beam;
        let run = if parallel { w } else { f.l_total };
        let centre = rib_centreline(&s.sheeting, run);
        let t = s.sheeting.thickness_m.max(0.0006);
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
        let detail = loc(&format!("{} ribs {}, t = {:.2} mm", s.sheeting.profile, if parallel { "parallel to the span" } else { "transverse to the span" }, t * 1e3), &format!("{} Rippen {}, t = {:.2} mm", s.sheeting.profile, if parallel { "parallel zur Spannrichtung" } else { "quer zur Spannrichtung" }, t * 1e3));
        let spec = Spec::new(EntityKind::Member, format!("{}:sheeting", s.id), "slabs", geometry, MaterialRole::Steel, loc(&format!("Profiled sheeting of {}", s.id), &format!("Profilblech von {}", s.id)), LAYER_SHEETING)
            .bound(binding(self.input.report, self.id(), &["en1994.9.3.", "en1994.9.7.3."]))
            .detailed(detail);
        self.out.push(spec, &[frame.origin, end]);
    }

    fn bars(&mut self) {
        let f = self.facts;
        let s = &f.slab;
        let spacing = 0.15_f64.max(f.l_total / 400.0);
        let d = (4.0 * s.as_m2_per_m * spacing / std::f64::consts::PI).sqrt().clamp(0.006, 0.02);
        let z = (s.concrete_thickness_m - 0.03).max(f.hp + 0.01);
        let count = ((f.l_total / spacing).floor() as usize).max(1);
        let from = self.p(spacing / 2.0, -STRIP_WIDTH / 2.0 + 0.03, z);
        let geometry = Geometry::Fastener(FastenerGeometry {
            kind: FastenerKind::Rebar { bend: None },
            axis: Segment3 { from: from.clone(), to: self.p(spacing / 2.0, STRIP_WIDTH / 2.0 - 0.03, z) },
            diameter: d,
            head: HeadShape::None,
            pattern: Some(Pattern::Along { path: super::beams::along(&from, &self.pose.ex(), (count - 1) as f64 * spacing + 0.25 * spacing), spacing }),
            hole_clearance: None,
        });
        let detail = loc(&format!("Ø{:.0} mm every {:.0} mm, A_s = {:.0} mm²/m", d * 1e3, spacing * 1e3, s.as_m2_per_m * 1e6), &format!("Ø{:.0} mm alle {:.0} mm, A_s = {:.0} mm²/m", d * 1e3, spacing * 1e3, s.as_m2_per_m * 1e6));
        let spec = Spec::new(EntityKind::Fastener, format!("{}:rebar", s.id), "slabs", geometry, MaterialRole::Rebar, loc(&format!("Bars of {}", s.id), &format!("Bewehrung von {}", s.id)), LAYER_REINFORCEMENT).bound(binding(self.input.report, self.id(), &["en1994.9.7.2."])).detailed(detail);
        self.out.push(spec, &[from, self.p(f.l_total, STRIP_WIDTH / 2.0, z)]);
    }

    fn supports(&mut self) {
        let f = self.facts;
        let up = self.pose.dir(0.0, 0.0, 1.0);
        let mut stations = vec![("support-a", 0.0, SupportKind::Pinned, loc("Pinned support", "Gelenklager"))];
        if f.continuous {
            stations.push(("support-m", f.slab.span_m, SupportKind::Roller { normal: up.clone() }, loc("Internal support", "Mittelauflager")));
        }
        stations.push(("support-b", f.l_total, SupportKind::Roller { normal: up }, loc("Roller support", "Rollenlager")));
        for (tag, x, kind, name) in stations {
            let at = self.p(x, 0.0, 0.0);
            let geometry = Geometry::Support(SupportGeometry { kind, frame: local_frame(self.pose, (x, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)), at: at.clone() });
            self.out.push(Spec::new(EntityKind::Support, format!("{}:{tag}", self.id()), "slabs", geometry, MaterialRole::Support, name, LAYER_SUPPORTS), &[at]);
        }
    }

    fn loads(&mut self) {
        let f = self.facts;
        let s = &f.slab;
        let top = s.concrete_thickness_m + 0.02;
        for action in s.actions.iter().filter(|a| a.kind != semio_s_artifact_norm_en1994_model::ActionKind::Fatigue) {
            let kind = case_kind(action.kind);
            let case = LoadCaseRef { id: action.id.clone(), kind, psi: None };
            let (glyph, at, name) = if action.q_area_pa.abs() > 1e-9 {
                let w = STRIP_WIDTH;
                let ring = vec![self.p(0.0, -w / 2.0, top), self.p(f.l_total, -w / 2.0, top), self.p(f.l_total, w / 2.0, top), self.p(0.0, w / 2.0, top)];
                let text = format!("{} {:.2} kN/m²", action.id, action.q_area_pa / 1e3);
                (LoadGlyph::Area { polygon: Polygon3 { outer: Ring3 { points: ring }, holes: Vec::new() }, direction: mul(&Vec3::Z, -1.0), intensity: Quantity::new(QuantityKind::Pressure, action.q_area_pa.abs()), case }, self.p(f.l_total / 2.0, 0.0, top), loc(&text, &text))
            } else if action.f_k_n.abs() > 1.0 {
                let at = self.p(f.point_x(action), f.point_y(action, STRIP_WIDTH), top);
                let text = format!("{} {:.1} kN", action.id, action.f_k_n / 1e3);
                (LoadGlyph::Point { at: at.clone(), direction: mul(&Vec3::Z, -1.0), magnitude: Quantity::new(QuantityKind::Force, action.f_k_n.abs()), case }, at, loc(&text, &text))
            } else {
                continue;
            };
            self.out.push(Spec::new(EntityKind::Load, format!("{}:load-{}", s.id, action.id), "slabs", Geometry::Load(glyph), MaterialRole::Load { kind }, name, LAYER_LOADS), &[at]);
        }
    }

    fn annotations(&mut self, all: &Option<UtilisationBinding>) {
        let f = self.facts;
        let s = &f.slab;
        let label_at = self.p(f.l_total / 2.0, STRIP_WIDTH / 2.0, s.concrete_thickness_m + 0.3);
        self.out.annotate(Annotation::Label { id: format!("{}:label", s.id), anchor: label_at.clone(), text: loc(&format!("{} h = {:.0} mm", s.id, s.concrete_thickness_m * 1e3), &format!("{} h = {:.0} mm", s.id, s.concrete_thickness_m * 1e3)), tone: tone_of(all) }, &label_at);
        let span_from = self.p(0.0, 0.0, -0.4);
        self.out.annotate(
            Annotation::Dimension { id: format!("{}:span", s.id), from: span_from.clone(), to: self.p(s.span_m, 0.0, -0.4), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, s.span_m), binding: Some(FieldBinding::new(format!("slabs[id={}].spanM", s.id), Affine::IDENTITY, "m")) },
            &span_from,
        );
        let edge = self.p(0.0, -STRIP_WIDTH / 2.0 - 0.05, 0.0);
        self.out.annotate(
            Annotation::Dimension { id: format!("{}:thickness", s.id), from: edge.clone(), to: self.p(0.0, -STRIP_WIDTH / 2.0 - 0.05, s.concrete_thickness_m), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, s.concrete_thickness_m), binding: Some(FieldBinding::new(format!("slabs[id={}].concreteThicknessM", s.id), Affine::IDENTITY, "m")) },
            &edge,
        );
        if let Some(clause) = all.as_ref().and_then(|b| clause_of(self.input.report, &b.check_id)) {
            let at = self.p(f.l_total / 2.0, 0.0, s.concrete_thickness_m / 2.0);
            self.out.annotate(Annotation::Clause { id: format!("{}:clause", s.id), anchor: entity_ref(EntityKind::Plate, &s.id, "slabs"), clause }, &at);
        }
        let at = self.p(f.l_total / 2.0, 0.0, s.concrete_thickness_m / 2.0);
        for (prefix, mode, tag) in [("en1994.9.7.2.", FailureMode::Yielding, "bending"), ("en1994.9.3.", FailureMode::Yielding, "sheeting"), ("en1994.9.7.3.", FailureMode::Other, "shear"), ("en1994.9.7.5.", FailureMode::Punching, "vertical")] {
            if worst(self.input.report, &s.id, &[prefix]).is_some_and(|c| matches!(c.status, norm::CheckStatus::Fail | norm::CheckStatus::Warning)) {
                self.out.annotate(Annotation::FailureGlyph { id: format!("{}:failure-{tag}", s.id), at: entity_ref(EntityKind::Plate, &s.id, "slabs"), mode, extent: None }, &at);
            }
        }
    }
}
