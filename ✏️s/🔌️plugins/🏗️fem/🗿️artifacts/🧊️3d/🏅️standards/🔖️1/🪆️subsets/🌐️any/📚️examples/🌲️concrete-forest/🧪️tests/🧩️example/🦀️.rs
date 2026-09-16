//! 🧪️ Tests for example `🌲️concrete-forest`: the asset parses and round-trips, its topology is the
//! stacked CAD structure-classic member set on the puzzle3d vortex coordinates, and the engine solves
//! its deformation (self weight + slab dead + live, ULS/SLS) to a balanced, physically ordered result.

use crate::fem3d_engine::fem3d_solve_all;
use crate::model::{Dof, StaticResult};
use crate::standards::v1::subsets::any::schema::snapshot::text::{parse_dsl, print_dsl};
use crate::{element_id, FemElement, Fem3dSnapshot};
use std::collections::HashMap;

const VORTEX_XY: [(f64, f64); 7] = [(4.05001, 4.676537), (6.75001, 4.676537), (9.45001, 4.676537), (6.75001, 0.0), (4.05001, 0.0), (1.35001, 0.0), (9.45001, 2.338269)];
const COLUMN_XY: [(f64, f64); 2] = [(2.70001, 2.338269), (8.10001, 2.338269)];
const GRAVITY: f64 = 9.81;

fn document() -> Fem3dSnapshot {
    parse_dsl(super::PRIMARY_TEXT).expect("concrete-forest example dsl parses")
}

fn node(doc: &Fem3dSnapshot, id: &str) -> [f64; 3] {
    let node = doc.nodes.iter().find(|node| node.id == id).unwrap_or_else(|| panic!("node {id}"));
    [node.x, node.y, node.z]
}

fn frame_ends(element: &FemElement) -> (&str, &str) {
    match element {
        FemElement::Frame { start, end, .. } => (start, end),
        FemElement::Bar { id, .. } => panic!("{id} must be a frame"),
    }
}

fn tz(result: &StaticResult, node_id: &str) -> f64 {
    result.displacements.iter().find(|row| row.node_id == node_id).unwrap_or_else(|| panic!("displacement {node_id}")).values[2]
}

fn applied_vertical_load(doc: &Fem3dSnapshot, case_id: &str) -> f64 {
    let case = doc.load_cases.iter().find(|case| case.id == case_id).unwrap_or_else(|| panic!("case {case_id}"));
    let length = |element: &FemElement| {
        let (start, end) = frame_ends(element);
        let (a, b) = (node(doc, start), node(doc, end));
        ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
    };
    let udl: f64 = case
        .loads
        .iter()
        .map(|load| match load {
            crate::FemLoad::MemberUdl { element_id: target, wz, .. } => wz * length(doc.elements.iter().find(|element| element_id(element) == target).expect("udl element")),
            crate::FemLoad::Nodal { dof: crate::FemDof::Tz, value, .. } => *value,
            _ => 0.0,
        })
        .sum();
    let self_weight: f64 = if case.self_weight {
        doc.elements
            .iter()
            .map(|element| {
                let (material_id, section_id) = match element {
                    FemElement::Frame { material_id, section_id, .. } | FemElement::Bar { material_id, section_id, .. } => (material_id, section_id),
                };
                let rho = doc.materials.iter().find(|material| &material.id == material_id).expect("material").rho;
                let area = doc.sections.iter().find(|section| &section.id == section_id).expect("section").area;
                -rho * area * GRAVITY * length(element)
            })
            .sum()
    } else {
        0.0
    };
    udl + self_weight
}

#[test]
fn primary_asset_parses_and_round_trips() {
    let doc = document();
    assert!(super::PRIMARY_TEXT.len() > 64, "dsl fixture must carry real payload");
    let reparsed = parse_dsl(&print_dsl(&doc)).expect("printed dsl reparses");
    assert_eq!(reparsed, doc, "print → parse must be the identity");
    assert!(doc.solids.is_empty(), "nodes and line elements only — no meshed solids");
    assert!(doc.elements.iter().all(|element| matches!(element, FemElement::Frame { .. })), "every member is a 6-DOF frame");
}

#[test]
fn topology_is_two_stacked_pieces_on_the_puzzle_vortices() {
    let doc = document();
    assert_eq!(doc.nodes.len(), 20, "2 column bases + 2 shared column heads per storey + 7 tips per storey");
    assert_eq!(doc.elements.len(), 20, "per piece: 2 columns + 1 spine + 7 cantilevers");
    for (storey, z) in [("l", 3.0), ("u", 6.0)] {
        for (index, (x, y)) in VORTEX_XY.iter().enumerate() {
            assert_eq!(node(&doc, &format!("{storey}v{index}")), [*x, *y, z], "tip {storey}v{index} sits on puzzle vortex v{index} lifted to z {z}");
        }
        for (index, (x, y)) in COLUMN_XY.iter().enumerate() {
            assert_eq!(node(&doc, &format!("{storey}c{}t", index + 1)), [*x, *y, z], "column head {storey}c{} on the c-t vortex", index + 1);
        }
    }
    for (index, (x, y)) in COLUMN_XY.iter().enumerate() {
        assert_eq!(node(&doc, &format!("lc{}b", index + 1)), [*x, *y, 0.0], "column base on the c-b vortex");
    }
    let by_id: HashMap<&str, &FemElement> = doc.elements.iter().map(|element| (element_id(element), element)).collect();
    assert_eq!(frame_ends(by_id["u_col1"]), ("lc1t", "uc1t"), "the upper column starts on the lower column head — one shared joint, the puzzle's c-b/c-t attraction");
    assert_eq!(frame_ends(by_id["u_col2"]), ("lc2t", "uc2t"));
    for storey in ["l", "u"] {
        let head = |column: usize| format!("{storey}c{column}t");
        assert_eq!(frame_ends(by_id[format!("{storey}_spine").as_str()]), (head(2).as_str(), head(1).as_str()), "spine joins the two column heads like CAD structure member 8");
        for (tip, column) in [(0, 1), (1, 2), (2, 2), (3, 2), (4, 1), (5, 1), (6, 2)] {
            assert_eq!(frame_ends(by_id[format!("{storey}_b{tip}").as_str()]), (head(column).as_str(), format!("{storey}v{tip}").as_str()), "cantilever {storey}_b{tip} fans out of column {column} like CAD structure members 1–7");
        }
    }
    assert_eq!(doc.supports.len(), 2, "only the two lower column bases are fixed");
    assert!(doc.supports.iter().all(|support| support.fixed.len() == 6 && support.node_id.ends_with('b')));
}

#[test]
fn deformation_solves_balanced_and_stacks() {
    let doc = document();
    let results = fem3d_solve_all(&doc).expect("stacked concrete forest solves");
    for case_id in ["dead", "live", "uls", "sls"] {
        assert!(results.contains_key(case_id), "result for {case_id}");
    }
    for (case_id, result) in &results {
        assert!(result.displacements.iter().all(|row| row.values.iter().all(|value| value.is_finite())), "{case_id}: finite displacements");
        assert!(result.checks.residual_norm < 1e-6, "{case_id}: residual {}", result.checks.residual_norm);
    }
    for case_id in ["dead", "live"] {
        let result = &results[case_id];
        let reaction_z: f64 = result.reactions.iter().filter(|reaction| reaction.dof == Dof::Tz).map(|reaction| reaction.value).sum();
        let applied = applied_vertical_load(&doc, case_id);
        assert!(((reaction_z + applied) / applied).abs() < 1e-6, "{case_id}: vertical reactions {reaction_z} balance the applied load {applied}");
    }
    let sls = &results["sls"];
    let row = |id: &str| sls.displacements.iter().find(|row| row.node_id == id).unwrap_or_else(|| panic!("displacement {id}")).values;
    for head in ["lc1t", "lc2t", "uc1t", "uc2t"] {
        assert!(tz(sls, head) < 0.0, "column head {head} shortens under gravity");
    }
    assert!(tz(sls, "uc1t") < tz(sls, "lc1t") && tz(sls, "uc2t") < tz(sls, "lc2t"), "upper heads ride on the lower storey's shortening");
    assert!(row("lc1t")[1] * row("lc2t")[1] < 0.0, "the hexagonal cut is asymmetric: column 1 carries two −y cantilevers, column 2 two +y ones, so the heads lean apart in y");
    assert!(row("uc1t")[1].abs() > row("lc1t")[1].abs(), "the lean accumulates storey over storey");
    let beam = doc.sections.iter().find(|section| section.id == "beam30x45").expect("beam section");
    let concrete = doc.materials.iter().find(|material| material.id == "c30").expect("concrete");
    let w = 10298.0 + 4952.0 + concrete.rho * beam.area * GRAVITY;
    for (storey, column_of) in [("l", [1, 2, 2, 2, 1, 1, 2]), ("u", [1, 2, 2, 2, 1, 1, 2])] {
        for (tip, column) in column_of.iter().enumerate() {
            let head_id = format!("{storey}c{column}t");
            let tip_id = format!("{storey}v{tip}");
            let (head, tip_node) = (node(&doc, &head_id), node(&doc, &tip_id));
            let (rx, ry) = (tip_node[0] - head[0], tip_node[1] - head[1]);
            let length = (rx * rx + ry * ry).sqrt();
            let head_row = row(&head_id);
            let rigid = head_row[2] + head_row[3] * ry - head_row[4] * rx;
            let sag = tz(sls, &tip_id) - rigid;
            let cantilever = -w * length.powi(4) / (8.0 * concrete.e * beam.iy);
            assert!(((sag - cantilever) / cantilever).abs() < 1e-6, "{tip_id}: sag beyond the head's rigid motion {sag} is the cantilever's own wL⁴/8EI {cantilever}");
        }
    }
    let max_sls = sls.displacements.iter().map(|row| row.values[2].abs()).fold(0.0, f64::max);
    assert!(max_sls > 1e-4 && max_sls < 0.05, "SLS peak vertical deflection {max_sls} m is millimetres, not noise and not a mechanism");
    let uls = &results["uls"];
    let dead = &results["dead"];
    let live = &results["live"];
    let expected = 1.35 * tz(dead, "uv5") + 1.5 * tz(live, "uv5");
    assert!((tz(uls, "uv5") - expected).abs() < 1e-12, "ULS superposes the solved cases linearly");
    for row in &sls.displacements {
        eprintln!("[DEBUG] concrete-forest sls {} ux={:.6e} uy={:.6e} uz={:.6e}", row.node_id, row.values[0], row.values[1], row.values[2]);
    }
}
