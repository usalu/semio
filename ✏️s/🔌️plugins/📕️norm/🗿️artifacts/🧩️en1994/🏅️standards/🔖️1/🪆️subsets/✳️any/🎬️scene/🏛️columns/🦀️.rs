//! 🏛️ Composite columns: steel section (CHS, RHS or encased and partially encased I section), concrete body with reinforcement bars, loads, supports and dimension annotations.

use super::beams::{case_kind, local_frame};
use super::facts::{column_bars, ColumnFacts, ColumnShape};
use super::kit::*;
use super::layout::{Layout, Pose};
use super::norm;
use super::scene::*;
use super::{handles, SceneInput};

const COVER: f64 = 0.05;

fn i_shape(h: f64, b: f64, tw: f64, tf: f64) -> SectionShape {
    SectionShape::parametric(ProfileKind::IShape, &[("h", h), ("b", b), ("tw", tw), ("tf", tf)])
}

pub(crate) fn steel_section(shape: &ColumnShape) -> SectionShape {
    match shape {
        ColumnShape::Chs { d, t } => SectionShape::parametric(ProfileKind::Chs, &[("d", *d), ("t", *t)]),
        ColumnShape::Rhs { side, t } => SectionShape::parametric(ProfileKind::Rhs, &[("h", *side), ("b", *side), ("t", *t)]),
        ColumnShape::Encased { h, b, tf, tw, .. } | ColumnShape::Partial { h, b, tf, tw } => i_shape(*h, *b, *tw, *tf),
    }
}

fn perimeter(n: usize, half_w: f64, half_h: f64) -> Vec<Vec2> {
    let per_side = (n - 4) / 4;
    let mut points = Vec::new();
    for (sx, sy) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
        points.push(v2(sx * half_w, sy * half_h));
    }
    for k in 1..=per_side {
        let t = -1.0 + 2.0 * k as f64 / (per_side + 1) as f64;
        points.extend([v2(t * half_w, -half_h), v2(t * half_w, half_h), v2(-half_w, t * half_h), v2(half_w, t * half_h)]);
    }
    points
}

fn ring(n: usize, radius: f64) -> Vec<Vec2> {
    (0..n).map(|i| v2(radius * (std::f64::consts::TAU * i as f64 / n as f64).cos(), radius * (std::f64::consts::TAU * i as f64 / n as f64).sin())).collect()
}

fn concrete_section(facts: &ColumnFacts) -> SectionShape {
    let part = |shape: SectionShape, offset: Vec2| CompositePart { shape: Box::new(shape), role: MaterialRole::Concrete, offset };
    let (n, bar) = column_bars(&facts.column);
    let positions;
    let parts;
    match &facts.shape {
        ColumnShape::Chs { d, t } => {
            parts = vec![part(SectionShape::parametric(ProfileKind::Circle, &[("d", d - 2.0 * t)]), Vec2::ZERO)];
            positions = ring(n, (d / 2.0 - t - COVER).max(bar));
        }
        ColumnShape::Rhs { side, t } => {
            parts = vec![part(SectionShape::parametric(ProfileKind::Rect, &[("h", side - 2.0 * t), ("b", side - 2.0 * t)]), Vec2::ZERO)];
            positions = perimeter(n, (side / 2.0 - t - COVER).max(bar), (side / 2.0 - t - COVER).max(bar));
        }
        ColumnShape::Encased { side, h, b, tf, tw } => {
            let hole = i_shape(*h, *b, *tw, *tf).outline().ok().and_then(|r| r.into_iter().next()).map(|p| p.outer);
            let outer = rect(-side / 2.0, side / 2.0, -side / 2.0, side / 2.0).outer;
            parts = vec![part(SectionShape::Polygon { polygon: Polygon2 { outer, holes: hole.into_iter().collect() } }, Vec2::ZERO)];
            positions = perimeter(n, side / 2.0 - COVER, side / 2.0 - COVER);
        }
        ColumnShape::Partial { h, b, tf, tw } => {
            let (chamber_h, chamber_b) = (h - 2.0 * tf, (b - tw) / 2.0);
            let shift = tw / 2.0 + chamber_b / 2.0;
            parts = [-shift, shift].iter().map(|x| part(SectionShape::parametric(ProfileKind::Rect, &[("h", chamber_h), ("b", chamber_b)]), v2(*x, 0.0))).collect();
            let per = n / 2;
            let reach = (chamber_h / 2.0 - 0.04).max(0.01);
            positions = [-shift, shift].iter().flat_map(|x| (0..per).map(move |k| v2(*x, -reach + 2.0 * reach * k as f64 / (per - 1).max(1) as f64))).collect();
        }
    }
    SectionShape::Composite { parts, rebar: vec![RebarLayout { bar_diameter: bar, positions, cover: COVER }] }
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for column in &input.document.columns {
        if !out.wants(&column.id) {
            continue;
        }
        let Some(pose) = layout.pose(&column.id) else { continue };
        if column.length_m <= 1e-6 || column.outer_size_m <= 1e-6 {
            continue;
        }
        let facts = ColumnFacts::of(input.document, column);
        Builder { input, facts: &facts, pose, out }.run();
    }
}

struct Builder<'a, 'b> {
    input: &'a SceneInput<'a>,
    facts: &'a ColumnFacts,
    pose: &'a Pose,
    out: &'b mut Out,
}

impl Builder<'_, '_> {
    fn id(&self) -> &str {
        &self.facts.column.id
    }

    fn p(&self, x: f64, y: f64, z: f64) -> Vec3 {
        self.pose.point(x, y, z)
    }

    fn member(&self, section: SectionShape) -> (Geometry, Vec3, Vec3) {
        let c = &self.facts.column;
        let frame = local_frame(self.pose, (0.0, 0.0, 0.0), (0.0, 0.0, 1.0), (1.0, 0.0, 0.0));
        let end = self.p(0.0, 0.0, c.length_m);
        let geometry = Geometry::Member(MemberGeometry {
            axis: polyline(&[frame.origin.clone(), end.clone()]),
            frame: frame.clone(),
            section,
            section_taper: None,
            start_offset: Vec2::ZERO,
            end_offset: Vec2::ZERO,
            start_release: EndRelease::Rigid,
            end_release: EndRelease::Pinned,
            stations: Vec::new(),
        });
        (geometry, frame.origin, end)
    }

    fn run(&mut self) {
        let f = self.facts;
        let c = &f.column;
        let all = binding(self.input.report, self.id(), &[]);
        let (kind_en, kind_de) = match c.kind {
            semio_s_artifact_norm_en1994_model::ColumnKind::Encased => ("Encased composite column", "Einbetonierte Verbundstütze"),
            semio_s_artifact_norm_en1994_model::ColumnKind::PartiallyEncased => ("Partially encased composite column", "Teilweise einbetonierte Verbundstütze"),
            _ => ("Concrete-filled composite column", "Betongefüllte Verbundstütze"),
        };
        let (geometry, base, top) = self.member(steel_section(&f.shape));
        let detail = loc(
            &format!("{}, L = {:.2} m, size {:.0} mm, f_ck = {:.0} MPa, f_y = {:.0} MPa", c.kind.as_str().replace('_', " "), c.length_m, c.outer_size_m * 1e3, c.concrete_f_ck_pa / 1e6, c.steel_f_y_pa / 1e6),
            &format!("{}, L = {:.2} m, Maß {:.0} mm, f_ck = {:.0} MPa, f_y = {:.0} MPa", c.kind.as_str().replace('_', " "), c.length_m, c.outer_size_m * 1e3, c.concrete_f_ck_pa / 1e6, c.steel_f_y_pa / 1e6),
        );
        let mut steel = Spec::new(EntityKind::Member, c.id.clone(), "columns", geometry, MaterialRole::Steel, loc(&format!("{kind_en} {}", c.id), &format!("{kind_de} {}", c.id)), LAYER_COLUMNS).bound(all.clone()).detailed(detail);
        steel.handles = handles::column_ids(self.facts, self.pose, self.id());
        self.out.push(steel, &[base.clone(), top.clone()]);
        let (geometry, _, _) = self.member(concrete_section(f));
        let (n, bar) = column_bars(c);
        let concrete = Spec::new(EntityKind::Member, format!("{}:concrete", c.id), "columns", geometry, MaterialRole::Concrete, loc(&format!("Concrete of {}", c.id), &format!("Beton von {}", c.id)), LAYER_COLUMNS)
            .bound(binding(self.input.report, self.id(), &["en1994.6.7.3.mn."]))
            .detailed(loc(&format!("{n} bars Ø{:.0} mm, A_s = {:.0} mm²", bar * 1e3, c.reinforcement_as_m2 * 1e6), &format!("{n} Stäbe Ø{:.0} mm, A_s = {:.0} mm²", bar * 1e3, c.reinforcement_as_m2 * 1e6)));
        self.out.push(concrete, &[base.clone(), top.clone()]);
        self.supports(&base, &top);
        self.loads(&top);
        self.annotations(&all, &top);
    }

    fn supports(&mut self, base: &Vec3, top: &Vec3) {
        for (tag, at, kind, name) in [("support-a", base, SupportKind::Fixed, loc("Fixed base", "Einspannung")), ("support-b", top, SupportKind::Pinned, loc("Pinned head", "Gelenkiger Kopf"))] {
            let frame = Frame { origin: at.clone(), x: self.pose.ex(), y: self.pose.ey(), z: self.pose.dir(0.0, 0.0, 1.0) };
            let geometry = Geometry::Support(SupportGeometry { kind, frame, at: at.clone() });
            self.out.push(Spec::new(EntityKind::Support, format!("{}:{tag}", self.id()), "columns", geometry, MaterialRole::Support, name, LAYER_SUPPORTS), &[at.clone()]);
        }
    }

    fn loads(&mut self, top: &Vec3) {
        let c = &self.facts.column;
        for (index, action) in c.actions.iter().enumerate() {
            let kind = case_kind(action.kind);
            let case = LoadCaseRef { id: action.id.clone(), kind, psi: None };
            let lift = 0.15 * index as f64;
            let at = add(top, &self.pose.dir(0.0, 0.0, lift));
            if action.n_k_n.abs() > 1.0 {
                let geometry = Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: mul(&Vec3::Z, -1.0), magnitude: Quantity::new(QuantityKind::Force, action.n_k_n.abs()), case: case.clone() });
                let name = loc(&format!("{} N = {:.0} kN", action.id, action.n_k_n / 1e3), &format!("{} N = {:.0} kN", action.id, action.n_k_n / 1e3));
                self.out.push(Spec::new(EntityKind::Load, format!("{}:load-{}-n", c.id, action.id), "columns", geometry, MaterialRole::Load { kind }, name, LAYER_LOADS), &[at.clone()]);
            }
            if action.m_k_nm.abs() > 1.0 {
                let geometry = Geometry::Load(LoadGlyph::Moment { at: at.clone(), axis: self.pose.ey(), magnitude: Quantity::new(QuantityKind::Moment, action.m_k_nm.abs()), case });
                let name = loc(&format!("{} M = {:.1} kNm", action.id, action.m_k_nm / 1e3), &format!("{} M = {:.1} kNm", action.id, action.m_k_nm / 1e3));
                self.out.push(Spec::new(EntityKind::Load, format!("{}:load-{}-m", c.id, action.id), "columns", geometry, MaterialRole::Load { kind }, name, LAYER_LOADS), &[at]);
            }
        }
    }

    fn annotations(&mut self, all: &Option<UtilisationBinding>, top: &Vec3) {
        let f = self.facts;
        let c = &f.column;
        let side = f.shape.outer_width();
        let label_at = add(top, &self.pose.dir(0.0, 0.0, 0.3));
        self.out.annotate(Annotation::Label { id: format!("{}:label", c.id), anchor: label_at.clone(), text: loc(&format!("{} {:.0} mm", c.id, c.outer_size_m * 1e3), &format!("{} {:.0} mm", c.id, c.outer_size_m * 1e3)), tone: tone_of(all) }, &label_at);
        let intro = (2.0 * c.outer_size_m).min(c.length_m / 3.0);
        let intro_from = self.p(side / 2.0 + 0.5, 0.0, c.length_m - intro);
        self.out.annotate(
            Annotation::Dimension { id: format!("{}:introduction", c.id), from: intro_from.clone(), to: self.p(side / 2.0 + 0.5, 0.0, c.length_m), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, intro), binding: None },
            &intro_from,
        );
        let length_from = self.p(side / 2.0 + 0.3, 0.0, 0.0);
        self.out.annotate(
            Annotation::Dimension { id: format!("{}:length", c.id), from: length_from.clone(), to: self.p(side / 2.0 + 0.3, 0.0, c.length_m), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, c.length_m), binding: Some(FieldBinding::new(format!("columns[id={}].lengthM", c.id), Affine::IDENTITY, "m")) },
            &length_from,
        );
        let size_from = self.p(-c.outer_size_m / 2.0, -side / 2.0 - 0.15, c.length_m / 2.0);
        self.out.annotate(
            Annotation::Dimension { id: format!("{}:size", c.id), from: size_from.clone(), to: self.p(c.outer_size_m / 2.0, -side / 2.0 - 0.15, c.length_m / 2.0), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, c.outer_size_m), binding: Some(FieldBinding::new(format!("columns[id={}].outerSizeM", c.id), Affine { base: 0.0, k: 1.0 }, "m")) },
            &size_from,
        );
        if c.kind == semio_s_artifact_norm_en1994_model::ColumnKind::ConcreteFilled {
            let wall_from = self.p(c.outer_size_m / 2.0 - c.wall_thickness_m, side / 2.0 + 0.15, c.length_m / 2.0);
            self.out.annotate(
                Annotation::Dimension { id: format!("{}:wall", c.id), from: wall_from.clone(), to: self.p(c.outer_size_m / 2.0, side / 2.0 + 0.15, c.length_m / 2.0), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, c.wall_thickness_m), binding: Some(FieldBinding::new(format!("columns[id={}].wallThicknessM", c.id), Affine::IDENTITY, "m")) },
                &wall_from,
            );
        }
        let chi_at = self.p(0.0, side / 2.0 + 0.2, c.length_m * 0.75);
        self.out.annotate(Annotation::Label { id: format!("{}:slenderness", c.id), anchor: chi_at.clone(), text: loc(&format!("λ̄ = {:.2}, χ = {:.2}", f.lambda, f.chi), &format!("λ̄ = {:.2}, χ = {:.2}", f.lambda, f.chi)), tone: Tone::Info }, &chi_at);
        if let Some(clause) = all.as_ref().and_then(|b| clause_of(self.input.report, &b.check_id)) {
            let at = self.p(0.0, 0.0, c.length_m / 2.0);
            self.out.annotate(Annotation::Clause { id: format!("{}:clause", c.id), anchor: entity_ref(EntityKind::Member, &c.id, "columns"), clause }, &at);
        }
        if worst(self.input.report, &c.id, &["en1994.6.7.3."]).is_some_and(|w| matches!(w.status, norm::CheckStatus::Fail | norm::CheckStatus::Warning)) {
            let at = self.p(0.0, 0.0, c.length_m / 2.0);
            self.out.annotate(Annotation::FailureGlyph { id: format!("{}:failure-crushing", c.id), at: entity_ref(EntityKind::Member, &c.id, "columns"), mode: FailureMode::Crushing, extent: None }, &at);
        }
    }
}
