//! 🏹️ Loads and attachments: design-action glyphs on members, silo pressure, wind on tower legs, wheel loads on crane runways, fire exposure, and the fatigue details attached to members.

use super::kit::*;
use super::handles;
use super::layout::Layout;
use super::scene::*;
use super::{checks, model, SceneInput};

pub fn case_kind(kind: semio_s_artifact_norm_en1993_model::ActionKind) -> LoadKind {
    use semio_s_artifact_norm_en1993_model::ActionKind as K;
    match kind {
        K::Permanent => LoadKind::Permanent,
        K::Imposed => LoadKind::Variable,
        K::Wind => LoadKind::Wind,
        K::Snow => LoadKind::Snow,
        K::Accidental => LoadKind::Accidental,
    }
}

fn case_ref(id: &str, kind: LoadKind) -> LoadCaseRef {
    LoadCaseRef { id: id.to_string(), kind, psi: None }
}

fn glyph(id: String, collection: &'static str, glyph: LoadGlyph, kind: LoadKind, binding: Option<UtilisationBinding>, name: LocalizedCopy, at: Vec3, out: &mut Out) {
    gripped_glyph(id, collection, glyph, kind, binding, name, at, Vec::new(), out);
}

#[allow(clippy::too_many_arguments)]
fn gripped_glyph(id: String, collection: &'static str, glyph: LoadGlyph, kind: LoadKind, binding: Option<UtilisationBinding>, name: LocalizedCopy, at: Vec3, handles: Vec<String>, out: &mut Out) {
    out.push(
        Spec { kind: EntityKind::Load, id, collection, geometry: Geometry::Load(glyph), role: MaterialRole::Load { kind }, binding, detail: name.clone(), name, layer: LAYER_LOADS, handles, deformation: None },
        &[at],
    );
}

#[allow(clippy::too_many_arguments)]
fn case_glyph(id: String, glyph: LoadGlyph, kind: LoadKind, name: LocalizedCopy, at: Vec3, handles: Vec<String>, case: &str, out: &mut Out) {
    let layer = case_layer(case);
    out.push_tuned(Spec { kind: EntityKind::Load, id, collection: "memberActions", geometry: Geometry::Load(glyph), role: MaterialRole::Load { kind }, binding: None, detail: name.clone(), name, layer: LAYER_LOADS, handles, deformation: None }, &[at], |entity| entity.layer = layer);
}

fn forces_of(document: &model::En1993Snapshot, actions: &[model::ForceAction], kinds: &[semio_s_artifact_norm_en1993_model::ActionKind]) -> f64 {
    actions.iter().filter(|a| document.load_cases.iter().any(|lc| lc.id == a.load_case_id && (kinds.is_empty() || kinds.contains(&lc.kind)))).map(|a| a.force.abs()).sum()
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for (index, row) in d.member_actions.iter().enumerate() {
        if !out.own("memberActions", &row.id) {
            continue;
        }
        let (Some(pose), Some(section), Some(case)) = (
            layout.member(&row.member_id),
            d.members.iter().find(|m| m.id == row.member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id)),
            d.load_cases.iter().find(|l| l.id == row.load_case_id),
        ) else { continue };
        let kind = case_kind(case.kind);
        let shift = 0.2 * (index % 4) as f64;
        let f = &pose.frame;
        let a = &row.action;
        let end = along(&along(&f.origin, &f.x, pose.length), &f.y, shift);
        let mid = along(&along(&along(&f.origin, &f.x, pose.length / 2.0), &f.z, section.h / 2.0), &f.y, shift);
        let case_ref = case_ref(&case.id, kind);
        let tag = |en: &str, de: &str, value: String| loc(&format!("{en} {value} ({})", case.name), &format!("{de} {value} ({})", case.name));
        if a.n.abs() > 1e-9 {
            let direction = mul(&f.x, if a.n > 0.0 { -1.0 } else { 1.0 });
            case_glyph(format!("{}:n", row.id), LoadGlyph::Point { at: end.clone(), direction, magnitude: Quantity::new(QuantityKind::Force, a.n.abs()), case: case_ref.clone() }, kind, tag("N_Ed", "N_Ed", format!("{:.1} kN", a.n / 1e3)), end.clone(), handles::ids(d, layout, &format!("{}:n", row.id)), &case.id, out);
        }
        if a.vz.abs() > 1e-9 {
            let direction = mul(&f.z, if a.vz > 0.0 { -1.0 } else { 1.0 });
            case_glyph(format!("{}:vz", row.id), LoadGlyph::Point { at: mid.clone(), direction, magnitude: Quantity::new(QuantityKind::Force, a.vz.abs()), case: case_ref.clone() }, kind, tag("V_z,Ed", "V_z,Ed", format!("{:.1} kN", a.vz / 1e3)), mid.clone(), handles::ids(d, layout, &format!("{}:vz", row.id)), &case.id, out);
        }
        if a.my.abs() > 1e-9 {
            case_glyph(format!("{}:my", row.id), LoadGlyph::Moment { at: end.clone(), axis: f.y.clone(), magnitude: Quantity::new(QuantityKind::Moment, a.my.abs()), case: case_ref }, kind, tag("M_y,Ed", "M_y,Ed", format!("{:.1} kNm", a.my / 1e3)), end, handles::ids(d, layout, &format!("{}:my", row.id)), &case.id, out);
        }
    }
    for silo in d.silo_shells.iter().filter(|s| s.radius > 1e-6 && s.depth > 1e-6) {
        if !out.own("siloShells", &silo.id) {
            continue;
        }
        let o = layout.origin(&silo.id);
        let pressure = checks::part_4::janssen_pressure(silo.k, silo.gamma, silo.depth);
        if !pressure.is_finite() || pressure <= 0.0 {
            continue;
        }
        let half = 0.2 * silo.radius;
        let x = o.x + silo.radius;
        let ring = vec![v3(x, o.y - half, o.z), v3(x, o.y + half, o.z), v3(x, o.y + half, o.z + silo.depth), v3(x, o.y - half, o.z + silo.depth)];
        let zone = PressureZone { polygon: Polygon3 { outer: Ring3 { points: ring }, holes: Vec::new() }, value: Quantity::new(QuantityKind::Pressure, pressure), label: loc(&format!("Janssen p_h = {:.1} kPa", pressure / 1e3), &format!("Janssen p_h = {:.1} kPa", pressure / 1e3)) };
        glyph(
            format!("{}:janssen", silo.id),
            "siloShells",
            LoadGlyph::Pressure { face: entity_ref(EntityKind::Shell, &silo.id, "siloShells"), zones: vec![zone], case: case_ref("silo-fill", LoadKind::Variable) },
            LoadKind::Variable,
            None,
            loc(&format!("Silo pressure on {}", silo.id), &format!("Silodruck auf {}", silo.id)),
            v3(x, o.y, o.z),
            out,
        );
    }
    for tower in &d.tower_legs {
        if !out.own("towerLegs", &tower.id) {
            continue;
        }
        let Some((pose, section)) = layout.member(&tower.member_id).zip(d.members.iter().find(|m| m.id == tower.member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id))) else { continue };
        let force = forces_of(d, &tower.actions, &[semio_s_artifact_norm_en1993_model::ActionKind::Wind]);
        if force <= 0.0 {
            continue;
        }
        let at = along(&along(&pose.frame.origin, &pose.frame.x, pose.length / 2.0), &pose.frame.y, section.b / 2.0);
        gripped_glyph(tower.id.clone(), "towerLegs", LoadGlyph::Point { at: at.clone(), direction: Vec3::Y, magnitude: Quantity::new(QuantityKind::Force, force), case: case_ref("wind", LoadKind::Wind) }, LoadKind::Wind, binding(input.report, &[tower.id.as_str()]), loc(&format!("Wind on tower leg {}", tower.id), &format!("Wind auf Turmstab {}", tower.id)), at, handles::ids(d, layout, &tower.id), out);
    }
    for crane in &d.crane_runways {
        if !out.own("craneRunways", &crane.id) {
            continue;
        }
        let Some((pose, section)) = layout.member(&crane.member_id).zip(d.members.iter().find(|m| m.id == crane.member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id))) else { continue };
        let force = forces_of(d, &crane.actions, &[]);
        if force <= 0.0 {
            continue;
        }
        let at = along(&along(&pose.frame.origin, &pose.frame.x, pose.length / 2.0), &pose.frame.z, section.h / 2.0);
        gripped_glyph(crane.id.clone(), "craneRunways", LoadGlyph::Point { at: at.clone(), direction: mul(&Vec3::Z, -1.0), magnitude: Quantity::new(QuantityKind::Force, force * crane.phi.max(1.0)), case: case_ref("crane-wheel", LoadKind::Variable) }, LoadKind::Variable, binding(input.report, &[crane.id.as_str()]), loc(&format!("Wheel load on runway {}", crane.id), &format!("Radlast auf Kranbahn {}", crane.id)), at, handles::ids(d, layout, &crane.id), out);
    }
    for fire in &d.fire_exposures {
        if !out.own("fireExposures", &fire.id) {
            continue;
        }
        let Some((pose, section)) = layout.member(&fire.member_id).zip(d.members.iter().find(|m| m.id == fire.member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id))) else { continue };
        let at = along(&along(&pose.frame.origin, &pose.frame.x, pose.length / 2.0), &pose.frame.z, -section.h / 2.0 - 0.2);
        gripped_glyph(
            fire.id.clone(),
            "fireExposures",
            LoadGlyph::Thermal { target: entity_ref(EntityKind::Member, &fire.member_id, "members"), delta_t: Quantity::new(QuantityKind::Temperature, (fire.design_temperature - 20.0).max(0.0)), case: case_ref(&fire.id, LoadKind::Accidental) },
            LoadKind::Accidental,
            binding(input.report, &[fire.id.as_str()]),
            loc(&format!("Fire {} on {}", fire.rating.as_str().to_uppercase(), fire.member_id), &format!("Brand {} an {}", fire.rating.as_str().to_uppercase(), fire.member_id)),
            at,
            handles::ids(d, layout, &fire.id),
            out,
        );
    }
    for storey in &d.storeys {
        if !out.own("storeys", &storey.id) {
            continue;
        }
        let Some(pose) = layout.member(&storey.member_id) else { continue };
        let top = along(&pose.frame.origin, &pose.frame.x, pose.length);
        let binding = binding(input.report, &[storey.id.as_str()]);
        let alpha = checks::clauses::global::alpha_cr_storey(storey.horizontal_load, storey.vertical_load, storey.height, storey.drift);
        let label = alpha.map_or_else(|| "α_cr = -".to_string(), |a| format!("α_cr = {a:.1}"));
        out.push(
            Spec {
                kind: EntityKind::Group,
                id: storey.id.clone(),
                collection: "storeys",
                geometry: Geometry::Group { members: vec![entity_ref(EntityKind::Member, &storey.member_id, "members")] },
                role: MaterialRole::Steel,
                binding: binding.clone(),
                name: loc(&format!("Storey {}", storey.id), &format!("Geschoss {}", storey.id)),
                detail: loc(&label, &label),
                layer: LAYER_ANNOTATIONS,
                handles: Vec::new(),
                deformation: None,
            },
            &[top.clone()],
        );
        out.annotate(Annotation::Label { id: format!("{}:label", storey.id), anchor: top.clone(), text: loc(&label, &label), tone: tone_of(&binding) }, &top);
        if storey.horizontal_load > 0.0 {
            glyph(
                format!("{}:sway", storey.id),
                "storeys",
                LoadGlyph::Point { at: top.clone(), direction: Vec3::Y, magnitude: Quantity::new(QuantityKind::Force, storey.horizontal_load), case: case_ref("storey-sway", LoadKind::Variable) },
                LoadKind::Variable,
                binding,
                loc(&format!("Storey horizontal load H_Ed on {}", storey.id), &format!("Geschoss-Horizontallast H_Ed auf {}", storey.id)),
                top.clone(),
                out,
            );
        }
        if let Some(clause) = clause_of(input.report, &format!("en1993.1-1.5.2.alpha.{}", storey.id)) {
            out.annotate(Annotation::Clause { id: format!("{}:clause", storey.id), anchor: entity_ref(EntityKind::Member, &storey.member_id, "members"), clause }, &top);
        }
    }
    for restraint in &d.restraints {
        if !out.own("restraints", &restraint.id) {
            continue;
        }
        let Some(pose) = layout.member(&restraint.member_id) else { continue };
        let f = &pose.frame;
        let binding = binding(input.report, &[restraint.id.as_str()]);
        let kind_of = |fixity: &str| if fixity == "fixed" { SupportKind::Fixed } else { SupportKind::Pinned };
        let mut sites = vec![("start", 0.0, kind_of(restraint.start_support.as_str())), ("end", pose.length, kind_of(restraint.end_support.as_str()))];
        if restraint.intermediate_position > 0.0 && restraint.intermediate_position < pose.length {
            sites.push(("intermediate", restraint.intermediate_position, SupportKind::Roller { normal: f.y.clone() }));
        }
        for (tag, x, kind) in sites {
            let at = along(&f.origin, &f.x, x);
            out.push(
                Spec {
                    kind: EntityKind::Support,
                    id: format!("{}:{tag}", restraint.id),
                    collection: "restraints",
                    geometry: Geometry::Support(SupportGeometry { kind, at: at.clone(), frame: f.clone() }),
                    role: MaterialRole::Steel,
                    binding: binding.clone(),
                    name: loc(&format!("Support {tag} of {}", restraint.member_id), &format!("Lager {tag} von {}", restraint.member_id)),
                    detail: loc(&restraint.label, &restraint.label),
                    layer: LAYER_ANNOTATIONS,
                    handles: Vec::new(),
                    deformation: None,
                },
                &[at],
            );
        }
        if restraint.lateral_spring_stiffness > 0.0 && restraint.lateral_spring_position > 0.0 && restraint.lateral_spring_position < pose.length {
            let at = along(&f.origin, &f.x, restraint.lateral_spring_position);
            out.annotate(Annotation::Label { id: format!("{}:spring", restraint.id), anchor: at.clone(), text: loc(&format!("k = {:.0} kN/m", restraint.lateral_spring_stiffness / 1e3), &format!("k = {:.0} kN/m", restraint.lateral_spring_stiffness / 1e3)), tone: tone_of(&binding) }, &at);
        }
    }
    for built in &d.built_up_members {
        if !out.own("builtUpMembers", &built.id) {
            continue;
        }
        let Some(pose) = layout.member(&built.member_id) else { continue };
        let f = &pose.frame;
        let binding = binding(input.report, &[built.id.as_str()]);
        let at = along(&along(&f.origin, &f.x, pose.length / 2.0), &f.y, built.chord_distance / 2.0 + 0.1);
        let text = format!("{} h₀ = {:.0} mm, a = {:.0} mm", built.kind, built.chord_distance * 1e3, built.panel_spacing * 1e3);
        out.push(
            Spec {
                kind: EntityKind::Group,
                id: built.id.clone(),
                collection: "builtUpMembers",
                geometry: Geometry::Group { members: vec![entity_ref(EntityKind::Member, &built.member_id, "members")] },
                role: MaterialRole::Steel,
                binding: binding.clone(),
                name: loc(&format!("Built-up member {}", built.id), &format!("Mehrteiliger Stab {}", built.id)),
                detail: loc(&text, &text),
                layer: LAYER_ANNOTATIONS,
                handles: Vec::new(),
                deformation: None,
            },
            &[at.clone()],
        );
        out.annotate(Annotation::Label { id: format!("{}:label", built.id), anchor: at.clone(), text: loc(&text, &text), tone: tone_of(&binding) }, &at);
        let n = (pose.length / built.panel_spacing.max(0.05)).floor() as usize;
        for i in 1..n.min(40) {
            let x = along(&f.origin, &f.x, i as f64 * built.panel_spacing);
            out.annotate(Annotation::Label { id: format!("{}:panel-{i}", built.id), anchor: x.clone(), text: loc("|", "|"), tone: tone_of(&binding) }, &x);
        }
    }
    for weld in &d.weld_groups {
        if !out.own("weldGroups", &weld.id) {
            continue;
        }
        let Some(pose) = layout.member(&weld.member_id) else { continue };
        let f = &pose.frame;
        let binding = binding(input.report, &[weld.id.as_str()]);
        let end = along(&f.origin, &f.x, pose.length);
        let text = format!("a = {:.1} mm, {:.0} x {:.0} mm", weld.throat * 1e3, weld.width * 1e3, weld.depth * 1e3);
        out.push(
            Spec {
                kind: EntityKind::Group,
                id: weld.id.clone(),
                collection: "weldGroups",
                geometry: Geometry::Group { members: vec![entity_ref(EntityKind::Member, &weld.member_id, "members")] },
                role: MaterialRole::Steel,
                binding: binding.clone(),
                name: loc(&format!("Weld group {}", weld.id), &format!("Schweißnahtgruppe {}", weld.id)),
                detail: loc(&text, &text),
                layer: LAYER_ANNOTATIONS,
                handles: Vec::new(),
                deformation: None,
            },
            &[end.clone()],
        );
        out.annotate(Annotation::Label { id: format!("{}:label", weld.id), anchor: end.clone(), text: loc(&text, &text), tone: tone_of(&binding) }, &end);
        if weld.shear_force != 0.0 {
            glyph(format!("{}:force", weld.id), "weldGroups", LoadGlyph::Point { at: along(&end, &f.y, weld.eccentricity), direction: mul(&Vec3::Z, -1.0), magnitude: Quantity::new(QuantityKind::Force, weld.shear_force.abs()), case: case_ref("weld-group", LoadKind::Variable) }, LoadKind::Variable, binding, loc(&format!("Weld group force on {}", weld.id), &format!("Kraft auf Schweißnahtgruppe {}", weld.id)), end.clone(), out);
        }
    }
    for joint in &d.hollow_joints {
        if !out.own("hollowJoints", &joint.id) {
            continue;
        }
        let Some(pose) = layout.member(&joint.member_id) else { continue };
        let f = &pose.frame;
        let binding = binding(input.report, &[joint.id.as_str()]);
        let at = along(&f.origin, &f.x, pose.length / 2.0);
        let text = format!("RHS {:.0}x{:.0}x{:.1}, g = {:.0} mm", joint.chord_width * 1e3, joint.chord_height * 1e3, joint.chord_thickness * 1e3, joint.gap * 1e3);
        out.push(
            Spec {
                kind: EntityKind::Group,
                id: joint.id.clone(),
                collection: "hollowJoints",
                geometry: Geometry::Group { members: vec![entity_ref(EntityKind::Member, &joint.member_id, "members")] },
                role: MaterialRole::Steel,
                binding: binding.clone(),
                name: loc(&format!("Hollow section joint {}", joint.id), &format!("Hohlprofilknoten {}", joint.id)),
                detail: loc(&text, &text),
                layer: LAYER_ANNOTATIONS,
                handles: Vec::new(),
                deformation: None,
            },
            &[at.clone()],
        );
        out.annotate(Annotation::Label { id: format!("{}:label", joint.id), anchor: at.clone(), text: loc(&text, &text), tone: tone_of(&binding) }, &at);
        for (tag, angle, force) in [("1", joint.brace1_angle, joint.brace1_force), ("2", joint.brace2_angle, joint.brace2_force)] {
            if force == 0.0 {
                continue;
            }
            let sign = if tag == "1" { 1.0 } else { -1.0 };
            let direction = v3(angle.to_radians().cos() * sign, 0.0, angle.to_radians().sin());
            glyph(format!("{}:brace{tag}", joint.id), "hollowJoints", LoadGlyph::Point { at: at.clone(), direction, magnitude: Quantity::new(QuantityKind::Force, force.abs()), case: case_ref("hollow-brace", LoadKind::Variable) }, LoadKind::Variable, binding.clone(), loc(&format!("Brace {tag} force on {}", joint.id), &format!("Strebenkraft {tag} am Knoten {}", joint.id)), at.clone(), out);
        }
    }
    let details = d.fatigue_details.iter().map(|f| (f.id.as_str(), f.member_id.as_str(), format!("{}", f.category), "en1993.1-9.fat.")).chain(d.bridge_fatigue.iter().map(|b| (b.id.as_str(), b.member_id.as_str(), format!("{}", b.category), "en1993.2.fat.")));
    for (id, member, category, prefix) in details {
        if !out.own(if prefix.contains("1-9") { "fatigueDetails" } else { "bridgeFatigue" }, id) {
            continue;
        }
        let Some((pose, section)) = layout.member(member).zip(d.members.iter().find(|m| m.id == member).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id))) else { continue };
        let binding = binding(input.report, &[id]);
        let at = along(&along(&pose.frame.origin, &pose.frame.x, pose.length * 0.75), &pose.frame.z, section.h / 2.0 + 0.35);
        out.push(
            Spec {
                kind: EntityKind::Group,
                id: id.to_string(),
                collection: if prefix.contains("1-9") { "fatigueDetails" } else { "bridgeFatigue" },
                geometry: Geometry::Group { members: vec![entity_ref(EntityKind::Member, member, "members")] },
                role: MaterialRole::Steel,
                binding: binding.clone(),
                name: loc(&format!("Fatigue detail {id}"), &format!("Ermüdungsdetail {id}")),
                detail: loc(&format!("category {category}"), &format!("Kerbfall {category}")),
                layer: LAYER_ANNOTATIONS,
                handles: Vec::new(),
                deformation: None,
            },
            &[at.clone()],
        );
        out.annotate(Annotation::Label { id: format!("{id}:label"), anchor: at.clone(), text: loc(&format!("Δσ_C = {category}"), &format!("Kerbfall Δσ_C = {category}")), tone: tone_of(&binding) }, &at);
        if let Some(clause) = clause_of(input.report, &format!("{prefix}{id}")) {
            out.annotate(Annotation::Clause { id: format!("{id}:clause"), anchor: entity_ref(EntityKind::Member, member, "members"), clause }, &at);
        }
        if binding.as_ref().is_some_and(|b| b.status == CheckStatus::Fail) {
            out.annotate(Annotation::FailureGlyph { id: format!("{id}:failure"), at: entity_ref(EntityKind::Member, member, "members"), mode: FailureMode::Fatigue, extent: None }, &at);
        }
    }
}
