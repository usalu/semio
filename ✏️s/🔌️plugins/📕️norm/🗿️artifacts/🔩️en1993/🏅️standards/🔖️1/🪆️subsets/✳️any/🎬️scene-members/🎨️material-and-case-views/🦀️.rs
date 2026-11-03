//! 🎨️ Materials and load cases in 3D: every material stands as a strength column (height f_y over `handles::STRENGTH_PER_METRE`, slim sibling for f_u) coloured by the shared yield-strength legend and gripped on its top; every load case stands as a card whose height counts its member actions. The member-action glyphs sit on one layer per case, so the load-case track shows one case at a time.

use super::handles;
use super::kit::*;
use super::layout::Layout;
use super::loads_and_supports::case_kind;
use super::scene::*;
use super::SceneInput;

const GRADE_FIELD: &str = "yield-strength";
const WIDTH: f64 = 0.45;
const SLIM: f64 = 0.12;
const CARD_HEIGHT: f64 = 0.15;
const ACTION_HEIGHT: f64 = 0.1;
const MAX_ACTIONS: usize = 12;

fn plate(origin: &Vec3, dx: f64, width: f64, height: f64) -> Geometry {
    let centre = along(&along(origin, &Vec3::X, dx), &Vec3::Z, height / 2.0);
    let half = width / 2.0;
    Geometry::Plate(PlateGeometry { frame: member_frame(&centre, &Vec3::X), outline: Polygon2::solid(vec![Vec2::new(-half, -half), Vec2::new(half, -half), Vec2::new(half, half), Vec2::new(-half, half)]), thickness: height.max(1e-3), offset: 0.0, stiffeners: Vec::new() })
}

fn material_column(input: &SceneInput, layout: &Layout, material: &super::model::SteelMaterial, out: &mut Out) {
    let d = input.document;
    let origin = layout.origin(&material.id);
    let users = d.members.iter().filter(|m| m.material_id == material.id || m.web_material_id == material.id).count();
    let text = format!("{}: f_y = {:.0} MPa, f_u = {:.0} MPa, E = {:.0} GPa, used by {users}", material.grade, material.fy / 1.0e6, material.fu / 1.0e6, material.e_modulus / 1.0e9);
    let text_de = format!("{}: f_y = {:.0} MPa, f_u = {:.0} MPa, E = {:.0} GPa, verwendet von {users}", material.grade, material.fy / 1.0e6, material.fu / 1.0e6, material.e_modulus / 1.0e9);
    let top = along(&origin, &Vec3::Z, material.fy / handles::STRENGTH_PER_METRE);
    let binding = binding(input.report, &[material.id.as_str()]);
    let field = ScalarField { id: GRADE_FIELD.into(), unit: QuantityKind::Stress, samples: FieldSamples::Instance { values: vec![material.fy / 1.0e6] } };
    let tone = tone_of(&binding);
    out.push_tuned(
        Spec {
            kind: EntityKind::Plate,
            id: material.id.clone(),
            collection: "materials",
            geometry: plate(&origin, 0.0, WIDTH, material.fy / handles::STRENGTH_PER_METRE),
            role: MaterialRole::Steel,
            binding,
            name: loc(&format!("Material {}", material.id), &format!("Werkstoff {}", material.id)),
            detail: loc(&text, &text_de),
            layer: LAYER_GRADES,
            handles: handles::ids(d, layout, &material.id),
            deformation: None,
        },
        &[origin.clone(), top.clone()],
        |entity| {
            if entity.utilisation.is_none() {
                entity.appearance.channel = ColourChannel::Fields { active: GRADE_FIELD.into(), fields: vec![field] };
            }
        },
    );
    let ultimate = material.fu / handles::STRENGTH_PER_METRE;
    out.push(
        Spec {
            kind: EntityKind::Plate,
            id: format!("{}:fu", material.id),
            collection: "materials",
            geometry: plate(&origin, WIDTH, SLIM, ultimate),
            role: MaterialRole::Steel,
            binding: None,
            name: loc(&format!("Ultimate strength of {}", material.id), &format!("Zugfestigkeit von {}", material.id)),
            detail: loc(&format!("f_u = {:.0} MPa", material.fu / 1.0e6), &format!("f_u = {:.0} MPa", material.fu / 1.0e6)),
            layer: LAYER_GRADES,
            handles: Vec::new(),
            deformation: None,
        },
        &[along(&origin, &Vec3::X, WIDTH), along(&along(&origin, &Vec3::X, WIDTH), &Vec3::Z, ultimate)],
    );
    out.annotate(Annotation::Label { id: format!("{}:label", material.id), anchor: top.clone(), text: loc(&text, &text_de), tone }, &top);
}

fn case_card(input: &SceneInput, layout: &Layout, case: &super::model::LoadCase, out: &mut Out) {
    let d = input.document;
    let origin = layout.origin(&case.id);
    let count = d.member_actions.iter().filter(|a| a.load_case_id == case.id).count();
    let height = CARD_HEIGHT + ACTION_HEIGHT * count.min(MAX_ACTIONS) as f64;
    let text = format!("{} ({}): {count} member actions", case.name, case.kind);
    let text_de = format!("{} ({}): {count} Stabeinwirkungen", case.name, case.kind);
    let top = along(&origin, &Vec3::Z, height);
    let binding = binding(input.report, &[case.id.as_str()]);
    let tone = tone_of(&binding);
    out.push(
        Spec {
            kind: EntityKind::Plate,
            id: case.id.clone(),
            collection: "loadCases",
            geometry: plate(&origin, 0.0, WIDTH, height),
            role: MaterialRole::Load { kind: case_kind(case.kind) },
            binding,
            name: loc(&format!("Load case {}", case.name), &format!("Lastfall {}", case.name)),
            detail: loc(&text, &text_de),
            layer: LAYER_ANNOTATIONS,
            handles: handles::ids(d, layout, &case.id),
            deformation: None,
        },
        &[origin, top.clone()],
    );
    out.annotate(Annotation::Label { id: format!("{}:label", case.id), anchor: top.clone(), text: loc(&text, &text_de), tone }, &top);
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for material in &d.materials {
        if out.own("materials", &material.id) {
            material_column(input, layout, material, out);
        }
    }
    for case in &d.load_cases {
        if out.own("loadCases", &case.id) {
            case_card(input, layout, case, out);
        }
    }
}
