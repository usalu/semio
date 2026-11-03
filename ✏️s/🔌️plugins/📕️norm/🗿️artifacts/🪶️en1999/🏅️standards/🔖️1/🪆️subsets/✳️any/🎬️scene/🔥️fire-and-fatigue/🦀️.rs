//! 🔥️ Fire scenarios (EN 1999-1-2) as thermal action glyphs under the member and fatigue details (EN 1999-1-3) as selectable groups beside it.

use super::checks;
use super::checks::part_1_2;
use super::kit::*;
use super::layout::Layout;
use super::scene::*;
use super::{facts, handles, SceneInput};

const FIELD_SKIN: f64 = 0.001;
const FIELD_MAX_C: f64 = 700.0;

/// ⏱️ Fire times [s] of the strip temperature track: every strip carries the field `fire-theta@<seconds>` for each of them.
pub const FIRE_TIMES_S: [f64; 10] = [0.0, 300.0, 600.0, 900.0, 1200.0, 1800.0, 2700.0, 3600.0, 5400.0, 7200.0];
pub const FIRE_FIELD_PREFIX: &str = "fire-theta@";
pub const FIRE_LEGEND: &str = "fire-theta";

pub fn fire_field_id(seconds: f64) -> String { format!("{FIRE_FIELD_PREFIX}{seconds:.0}") }

fn clause_limit(report: &super::norm::CheckReport, id: &str) -> Option<f64> {
    report.checks.iter().find(|c| c.id == id && c.limit.value.is_finite()).map(|c| c.limit.value)
}

pub fn member_temperature(d: &super::model::En1999Snapshot, fire: &super::model::FireScenario, section: &super::model::AluminiumSection) -> f64 {
    let cover = d.fire_insulations.iter().find(|i| i.fire_id == fire.id).map(part_1_2::Insulation::from);
    part_1_2::member_temperature_c(fire.theta_a, part_1_2::section_factor(section), fire.duration_s.max(0.0), cover.as_ref())
}


fn fatigue_site_glyphs(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    let params = checks::na_de::AnnexParams::for_choice(d.annex);
    for cat in &d.fatigue_categories {
        if !out.own("fatigueCategories", &cat.id) {
            continue;
        }
        let Some((pose, section)) = layout.member(&cat.member_id).zip(d.members.iter().find(|m| m.id == cat.member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id))) else { continue };
        let damage = checks::part_1_3::category_damage(cat, &d.fatigue_details, params.gamma_ff, params.gamma_mf);
        let total = damage.as_ref().map_or(0.0, |x| x.total);
        let binding = out.binding_with(input, &[cat.id.as_str()], &["en1999.1-3.fat.category."]);
        let base = along(&along(&pose.frame.origin, &pose.frame.x, cat.location_x.clamp(0.0, pose.length)), &pose.frame.z, section_depth(section) / 2.0);
        let frame = Frame { origin: base.clone(), x: pose.frame.x.clone(), y: pose.frame.y.clone(), z: pose.frame.z.clone() };
        let height = 0.06 + 0.3 * total.min(1.5);
        let name = loc(&format!("Fatigue site {} ({:?} {}): D = {total:.3}", cat.id, cat.detail_type, cat.detail_category), &format!("Ermüdungsstelle {} ({:?} {}): D = {total:.3}", cat.id, cat.detail_type, cat.detail_category));
        out.push(
            Spec {
                kind: EntityKind::Solid,
                id: cat.id.clone(),
                collection: "fatigueCategories",
                geometry: Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame, radius: 0.04, height } }),
                role: MaterialRole::Aluminium,
                binding: binding.clone(),
                detail: name.clone(),
                name,
                layer: LAYER_ANNOTATIONS,
                handles: handles::ids(d, layout, &cat.id),
                deformation: None,
                style: None,
            },
            &[base.clone()],
        );
        let at = along(&base, &pose.frame.z, height + 0.1);
        out.annotate(Annotation::Label { id: format!("{}:label", cat.id), anchor: at.clone(), text: loc(&format!("{:?} {}: D = {total:.2}", cat.detail_type, cat.detail_category), &format!("{:?} {}: D = {total:.2}", cat.detail_type, cat.detail_category)), tone: tone_of(&binding) }, &at);
        if binding.as_ref().is_some_and(|b| b.status == CheckStatus::Fail) {
            out.annotate(Annotation::FailureGlyph { id: format!("{}:failure", cat.id), at: entity_ref(EntityKind::Member, &cat.member_id, "members"), mode: FailureMode::Fatigue, extent: None }, &at);
        }
    }
}

fn patch_load_glyphs(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for load in &d.patch_loads {
        if !out.own("patchLoads", &load.id) {
            continue;
        }
        let Some(pose) = handles::patch_pose(d, layout, load) else { continue };
        let top = along(&along(&pose.origin, &pose.x, load.x.clamp(0.0, pose.length)), &pose.z, pose.depth / 2.0);
        let binding = out.binding(input, &[load.id.as_str()]);
        let case = LoadCaseRef { id: load.id.clone(), kind: LoadKind::Other, psi: None };
        let name = loc(&format!("Patch load {} on {}: F_Ed = {:.1} kN over {:.0} mm ({:?})", load.id, load.subject_id, load.load / 1e3, load.bearing_length * 1e3, load.position), &format!("Einzellast {} auf {}: F_Ed = {:.1} kN über {:.0} mm ({:?})", load.id, load.subject_id, load.load / 1e3, load.bearing_length * 1e3, load.position));
        out.push(
            Spec {
                kind: EntityKind::Load,
                id: load.id.clone(),
                collection: "patchLoads",
                geometry: Geometry::Load(LoadGlyph::Point { at: top.clone(), direction: mul(&pose.z, -1.0), magnitude: Quantity::new(QuantityKind::Force, load.load.abs()), case }),
                role: MaterialRole::Load { kind: LoadKind::Other },
                binding: binding.clone(),
                detail: name.clone(),
                name,
                layer: LAYER_LOADS,
                handles: handles::ids(d, layout, &load.id),
                deformation: None,
                style: None,
            },
            &[top.clone()],
        );
        let patch = Frame { origin: along(&top, &pose.x, -0.5 * load.bearing_length), x: pose.x.clone(), y: pose.y.clone(), z: pose.z.clone() };
        let patch_name = loc(&format!("Bearing length s_s = {:.0} mm of {}", load.bearing_length * 1e3, load.id), &format!("Lasteinleitungslänge s_s = {:.0} mm von {}", load.bearing_length * 1e3, load.id));
        out.push(
            Spec {
                kind: EntityKind::Solid,
                id: format!("{}:patch", load.id),
                collection: "patchLoads",
                geometry: Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame: patch, size: v3(load.bearing_length.max(0.005), 0.05, 0.01) } }),
                role: MaterialRole::Load { kind: LoadKind::Other },
                binding,
                detail: patch_name.clone(),
                name: patch_name,
                layer: LAYER_LOADS,
                handles: Vec::new(),
                deformation: None,
                style: None,
            },
            &[top],
        );
        let s_s = if load.bearing_length > 0.0 { load.bearing_length } else { d.members.iter().find(|m| m.id == load.subject_id).map_or(0.0, |m| m.bearing_length) };
        let Some(r) = checks::patch_resistance_at(load, d, d.annex, s_s).filter(|r| r.h_w > 1e-6 && r.l_eff > 1e-6) else { continue };
        let at = |x: f64| along(&along(&pose.origin, &pose.x, x), &pose.z, 0.0);
        let (start, end) = (at((load.x - 0.5 * r.l_eff).max(0.0)), at((load.x + 0.5 * r.l_eff).min(pose.length)));
        let half = (0.5 * r.t_w + 0.002, 0.5 * r.h_w);
        let zone_name = loc(&format!("Web buckling zone of {}: l_eff = {:.0} mm of l_y = {:.0} mm, F_Rd = {:.1} kN, F_Ed = {:.1} kN (coefficients unverified)", load.id, r.l_eff * 1e3, r.l_y * 1e3, r.f_rd / 1e3, load.load.abs() / 1e3), &format!("Stegbeulzone von {}: l_eff = {:.0} mm von l_y = {:.0} mm, F_Rd = {:.1} kN, F_Ed = {:.1} kN (Beiwerte ungeprüft)", load.id, r.l_eff * 1e3, r.l_y * 1e3, r.f_rd / 1e3, load.load.abs() / 1e3));
        out.push(
            Spec {
                kind: EntityKind::Member,
                id: format!("{}:zone", load.id),
                collection: "patchLoads",
                geometry: Geometry::Member(MemberGeometry {
                    axis: polyline(&[start.clone(), end.clone()]),
                    frame: Frame { origin: start.clone(), x: pose.x.clone(), y: pose.y.clone(), z: pose.z.clone() },
                    section: SectionShape::Polygon { polygon: Polygon2::solid(vec![Vec2::new(-half.0, -half.1), Vec2::new(half.0, -half.1), Vec2::new(half.0, half.1), Vec2::new(-half.0, half.1)]) },
                    section_taper: None,
                    start_offset: Vec2::ZERO,
                    end_offset: Vec2::ZERO,
                    start_release: EndRelease::Rigid,
                    end_release: EndRelease::Rigid,
                    stations: Vec::new(),
                }),
                role: MaterialRole::Aluminium,
                binding: out.binding(input, &[load.id.as_str()]),
                detail: zone_name.clone(),
                name: zone_name,
                layer: LAYER_EFFECTIVE,
                handles: Vec::new(),
                deformation: None,
                style: None,
            },
            &[start, end],
        );
    }
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    fatigue_site_glyphs(input, layout, out);
    patch_load_glyphs(input, layout, out);
    for fire in &d.fire_scenarios {
        if !out.own("fireScenarios", &fire.id) {
            continue;
        }
        let Some((pose, member, section)) = layout.member(&fire.member_id).zip(d.members.iter().find(|m| m.id == fire.member_id)).and_then(|(p, m)| d.sections.iter().find(|s| s.id == m.section_id).map(|s| (p, m, s))) else { continue };
        let theta = member_temperature(d, fire, section);
        let at= along(&along(&pose.frame.origin, &pose.frame.x, pose.length / 2.0), &pose.frame.z, -section_depth(section) / 2.0 - 0.2);
        let binding = out.binding(input, &[fire.id.as_str()]);
        let name = loc(&format!("Fire {} on {}: θ_a = {theta:.0} °C", fire.id, member.id), &format!("Brand {} an {}: θ_a = {theta:.0} °C", fire.id, member.id));
        out.push(
            Spec {
                kind: EntityKind::Load,
                id: fire.id.clone(),
                collection: "fireScenarios",
                geometry: Geometry::Load(LoadGlyph::Thermal { target: entity_ref(EntityKind::Member, &fire.member_id, "members"), delta_t: Quantity::new(QuantityKind::Temperature, (theta - 20.0).max(1e-3)), case: LoadCaseRef { id: fire.id.clone(), kind: LoadKind::Accidental, psi: None } }),
                role: MaterialRole::Load { kind: LoadKind::Accidental },
                binding,
                detail: name.clone(),
                name,
                layer: LAYER_LOADS,
                handles: handles::ids(d, layout, &fire.id),
                deformation: None,
                style: None,
            },
            &[at],
        );
    }
    for fire in &d.fire_scenarios {
        if !out.own("fireScenarios", &fire.id) {
            continue;
        }
        let Some((pose, member, section)) = layout.member(&fire.member_id).zip(d.members.iter().find(|m| m.id == fire.member_id)).and_then(|(p, m)| d.sections.iter().find(|s| s.id == m.section_id).map(|s| (p, m, s))) else { continue };
        let Some((_, alloy)) = facts::member_parts(d, member) else { continue };
        let theta = member_temperature(d, fire, section);
        let Some(field) = part_1_2::temperature_field(section, alloy.f_o_pa, alloy.buckling_class_a, fire.theta_a, fire.duration_s.max(0.0), theta) else { continue };
        let theta_cr = clause_limit(input.report, &format!("en1999.1-2.fire.theta-cr.{}", fire.id)).unwrap_or(550.0);
        let history = part_1_2::strip_history(section, alloy.f_o_pa, alloy.buckling_class_a, fire.theta_a, &FIRE_TIMES_S);
        let end = along(&pose.frame.origin, &pose.frame.x, pose.length);
        for (k, (band, th)) in facts::gross_bands(section, &alloy).iter().zip(&field.thetas).enumerate() {
            let ratio = th / theta_cr.max(1.0);
            let status = super::scene::utilisation_status(ratio, 0.8);
            let name = loc(&format!("Strip {} of {}: θ = {th:.0} °C", k + 1, member.id), &format!("Streifen {} von {}: θ = {th:.0} °C", k + 1, member.id));
            let (w, h) = (band.dy + 2.0 * FIELD_SKIN, band.dz + 2.0 * FIELD_SKIN);
            let polygon = Polygon2::solid(vec![Vec2::new(band.y - w / 2.0, band.z - h / 2.0), Vec2::new(band.y + w / 2.0, band.z - h / 2.0), Vec2::new(band.y + w / 2.0, band.z + h / 2.0), Vec2::new(band.y - w / 2.0, band.z + h / 2.0)]);
            let scalars: Vec<ScalarField> = FIRE_TIMES_S.iter().enumerate().map(|(n, s)| {
                let v = history.get(k).and_then(|h| h.get(n)).copied().unwrap_or(*th);
                ScalarField { id: fire_field_id(*s), unit: QuantityKind::Temperature, samples: FieldSamples::Station { values: vec![StationValue { at: 0.0, value: v }, StationValue { at: 1.0, value: v }] } }
            }).collect();
            let active = FIRE_TIMES_S.iter().rev().find(|s| **s <= fire.duration_s).map_or_else(|| fire_field_id(0.0), |s| fire_field_id(*s));
            if !out.legends.iter().any(|l| l.id == FIRE_LEGEND) {
                out.legends.push(Legend { id: FIRE_LEGEND.into(), title: loc("Strip temperature θ in the standard fire (°C)", "Streifentemperatur θ im Normbrand (°C)"), scale: LegendScale::Scalar { field_id: fire_field_id(FIRE_TIMES_S[FIRE_TIMES_S.len() - 1]), map: ColourMapRef { id: colour_maps::RampId::Warm.wire().into(), min: 20.0, max: FIELD_MAX_C }, unit: QuantityKind::Temperature } });
            }
            out.push_with_channel(
                Spec {
                    kind: EntityKind::Member,
                    id: format!("{}:field-{}", fire.id, k + 1),
                    collection: "fireScenarios",
                    geometry: Geometry::Member(MemberGeometry { axis: polyline(&[pose.frame.origin.clone(), end.clone()]), frame: pose.frame.clone(), section: SectionShape::Polygon { polygon }, section_taper: None, start_offset: Vec2::ZERO, end_offset: Vec2::ZERO, start_release: EndRelease::Rigid, end_release: EndRelease::Rigid, stations: Vec::new() }),
                    role: MaterialRole::Aluminium,
                    binding: Some(UtilisationBinding { check_id: format!("en1999.1-2.fire.theta-cr.{}", fire.id), ratio, status, per_station: None, stale: false }),
                    detail: name.clone(),
                    name,
                    layer: LAYER_LOADS,
                    handles: Vec::new(),
                    deformation: None,
                    style: Some((StyleState::Normal, 0.6, None)),
                },
                ColourChannel::Fields { active, fields: scalars },
                &[pose.frame.origin.clone(), end.clone()],
            );
        }
    }
    for fat in &d.fatigue_details {
        if !out.own("fatigueDetails", &fat.id) {
            continue;
        }
        let Some((pose, section)) = layout.member(&fat.member_id).zip(d.members.iter().find(|m| m.id == fat.member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id))) else { continue };
        let binding = out.binding(input, &[fat.id.as_str()]);
        let at = along(&along(&pose.frame.origin, &pose.frame.x, pose.length * 0.75), &pose.frame.z, section_depth(section) / 2.0 + 0.35);
        out.push(
            Spec {
                kind: EntityKind::Group,
                id: fat.id.clone(),
                collection: "fatigueDetails",
                geometry: Geometry::Group { members: vec![entity_ref(EntityKind::Member, &fat.member_id, "members")] },
                role: MaterialRole::Aluminium,
                binding: binding.clone(),
                name: loc(&format!("Fatigue detail {}", fat.id), &format!("Ermüdungsdetail {}", fat.id)),
                detail: loc(&format!("category {}, Δσ_Ed = {:.1} MPa, n = {:.2e}", fat.detail_category, fat.delta_sigma_ed / 1e6, fat.n_cycles), &format!("Kerbfall {}, Δσ_Ed = {:.1} MPa, n = {:.2e}", fat.detail_category, fat.delta_sigma_ed / 1e6, fat.n_cycles)),
                layer: LAYER_ANNOTATIONS,
                handles: handles::ids(d, layout, &fat.id),
                deformation: None,
                style: None,
            },
            &[at.clone()],
        );
        let params = checks::na_de::AnnexParams::for_choice(d.annex);
        let share = checks::part_1_3::spectrum_curve(fat).map_or(0.0, |(dc, m1, m2)| checks::part_1_3::damage(fat.delta_sigma_ed, fat.n_cycles, dc, m1, m2, params.gamma_ff, params.gamma_mf));
        let top = along(&along(&pose.frame.origin, &pose.frame.x, pose.length * 0.75), &pose.frame.z, section_depth(section) / 2.0);
        let bar = Frame { origin: top.clone(), x: pose.frame.x.clone(), y: pose.frame.y.clone(), z: pose.frame.z.clone() };
        let bar_name = loc(&format!("Hot spot of {}: D_i = {share:.3}", fat.id), &format!("Hot Spot von {}: D_i = {share:.3}", fat.id));
        out.push(
            Spec {
                kind: EntityKind::Solid,
                id: format!("{}:hotspot", fat.id),
                collection: "fatigueDetails",
                geometry: Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: bar, radius: 0.03, height: 0.05 + 0.25 * share.min(1.5) } }),
                role: MaterialRole::Aluminium,
                binding: binding.clone(),
                detail: bar_name.clone(),
                name: bar_name,
                layer: LAYER_ANNOTATIONS,
                handles: Vec::new(),
                deformation: None,
                style: None,
            },
            &[top],
        );
        out.annotate(Annotation::Label { id: format!("{}:label", fat.id), anchor: at.clone(), text: loc(&format!("Δσ_C = {:.0} MPa", fat.delta_sigma_c / 1e6), &format!("Kerbfall Δσ_C = {:.0} MPa", fat.delta_sigma_c / 1e6)), tone: tone_of(&binding) }, &at);
        if let Some(clause) = clause_of(input.report, &format!("en1999.1-3.fat.{}", fat.id)) {
            out.annotate(Annotation::Clause { id: format!("{}:clause", fat.id), anchor: entity_ref(EntityKind::Member, &fat.member_id, "members"), clause }, &at);
        }
        if binding.as_ref().is_some_and(|b| b.status == CheckStatus::Fail) {
            out.annotate(Annotation::FailureGlyph { id: format!("{}:failure", fat.id), at: entity_ref(EntityKind::Member, &fat.member_id, "members"), mode: FailureMode::Fatigue, extent: None }, &at);
        }
    }
}
