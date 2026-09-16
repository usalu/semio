//! 🏠️ Example `house` — a real-world single-storey masonry house built ENTIRELY from meshed
//! continuum solids: a raft foundation, a ground slab and an attic slab, four load-bearing walls
//! with window and door openings, and a gable roof — every solid extruded along its own
//! [`crate::FemAxis`] so a wall is drawn in ELEVATION (openings are holes in its footprint) and the
//! roof as its chevron SECTION swept along the ridge.
//!
//! 📐️ CONFORMITY, the whole point of the dimensions below: every solid is meshed independently and
//! the mesher merges nodes within `NODE_MERGE_TOLERANCE`, so two solids are only connected where
//! their mesh nodes COINCIDE. The house is therefore drawn on a 0.5 m module — every outline corner,
//! opening edge, wall thickness and storey height is a multiple of [`MODULE`] — and meshed at
//! `mesh_size = √2 · MODULE` so the mesher's `max_edge / √2` lattice spacing lands exactly on the
//! module. Touching faces (a wall foot on the raft, a slab edge on a wall, the roof on the gable's
//! slanted top, a long wall's end on a gable's inner face) then share every node along the
//! interface and the walls, slabs and roof solve as ONE structure. The walls do not overlap at the
//! corners: the gable walls span the full width and the long walls sit between them.
//!
//! 🪨️ The raft's underside rests on a 1 m grid of document nodes fixed in translation (`Tet4`
//! carries no rotations); each lies on the raft's own 0.5 m lattice, so the mesh reuses those nodes
//! by id and the supports carry the model.

use crate::{FemAnalysisSettings, FemAxis, FemCombination, FemDof, FemLoad, FemLoadCase, FemMaterial, FemNode, FemSolid, FemSupport, Fem3dSnapshot};
use semio_framework_plugin::{ExampleSource, LocalizedLabel};
use std::collections::BTreeMap;

pub const ID: &str = "house";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("House", "Haus")
}
pub const ICON: &str = "home";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏠️house/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

//#region 🔖️Dimensions
/// 📏️ The drawing module every dimension is a multiple of, metres.
pub const MODULE: f64 = 0.5;
/// 🕸️ The `FemSolid::mesh_size` that makes the mesher's lattice spacing (`max_edge / √2`) exactly [`MODULE`].
pub const MESH_SIZE: f64 = std::f64::consts::SQRT_2 * MODULE;
/// 🏠️ Plan extent along x and y, metres.
pub const WIDTH: f64 = 6.0;
pub const LENGTH: f64 = 8.0;
/// 🧱️ Wall and slab thickness, metres — one module.
pub const THICKNESS: f64 = MODULE;
/// 📐️ Storey height (wall top), metres.
pub const STOREY: f64 = 3.0;
/// 🏔️ Ridge height of the roof UNDERSIDE, metres.
pub const RIDGE: f64 = 4.5;
/// 🪵️ Roof slab thickness measured vertically, metres.
pub const ROOF_THICKNESS: f64 = 0.25;
/// 🪨️ Raft thickness, metres — the raft top is z = 0.
pub const RAFT_THICKNESS: f64 = MODULE;
/// 🪨️ Support grid spacing under the raft, metres.
pub const SUPPORT_GRID: f64 = 1.0;
//#endregion 🔖️Dimensions

//#region 🔖️Builder
fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<[f64; 2]> {
    vec![[x0, y0], [x1, y0], [x1, y1], [x0, y1]]
}

fn solid(id: &str, name: &str, outline: Vec<[f64; 2]>, holes: Vec<Vec<[f64; 2]>>, axis: FemAxis, base: f64, height: f64, material_id: &str) -> FemSolid {
    FemSolid { id: id.into(), name: name.into(), outline, holes, base_z: base, height, layers: (height / MODULE).round().max(1.0) as usize, mesh_size: MESH_SIZE, material_id: material_id.into(), axis }
}

/// 🏠️ The chevron of the roof in the x–z plane: a flat seat over each long wall, then the two
/// slopes meeting at the ridge, offset upwards by [`ROOF_THICKNESS`] for the top surface.
fn roof_section() -> Vec<[f64; 2]> {
    let seat = THICKNESS;
    let mid = WIDTH / 2.0;
    vec![
        [0.0, STOREY],
        [seat, STOREY],
        [mid, RIDGE],
        [WIDTH - seat, STOREY],
        [WIDTH, STOREY],
        [WIDTH, STOREY + ROOF_THICKNESS],
        [WIDTH - seat, STOREY + ROOF_THICKNESS],
        [mid, RIDGE + ROOF_THICKNESS],
        [seat, STOREY + ROOF_THICKNESS],
        [0.0, STOREY + ROOF_THICKNESS],
    ]
}

/// 🏠️ A gable wall's elevation in the x–z plane: the storey rectangle topped by the roof underside,
/// so its slanted top edges are the very edges the roof section starts from.
fn gable_outline() -> Vec<[f64; 2]> {
    let seat = THICKNESS;
    let mid = WIDTH / 2.0;
    vec![[0.0, 0.0], [WIDTH, 0.0], [WIDTH, STOREY], [WIDTH - seat, STOREY], [mid, RIDGE], [seat, STOREY], [0.0, STOREY]]
}

/// 🏠️ Builds the house document — the committed `🗣️.dsl.semio` is exactly `print_dsl(&build())`.
pub fn build() -> Fem3dSnapshot {
    let inner_y = (THICKNESS, LENGTH - THICKNESS);
    let inner_x = (THICKNESS, WIDTH - THICKNESS);
    let mut nodes = Vec::new();
    let mut supports = Vec::new();
    let columns = (WIDTH / SUPPORT_GRID).round() as usize;
    let rows = (LENGTH / SUPPORT_GRID).round() as usize;
    for row in 0..=rows {
        for column in 0..=columns {
            let id = format!("f{column}_{row}");
            nodes.push(FemNode { id: id.clone(), x: column as f64 * SUPPORT_GRID, y: row as f64 * SUPPORT_GRID, z: -RAFT_THICKNESS });
            supports.push(FemSupport { id: format!("s{column}_{row}"), node_id: id, fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] });
        }
    }
    let window = |a: f64, b: f64| rect(a, 1.0, b, 2.0);
    let solids = vec![
        solid("raft", "Raft Foundation", rect(0.0, 0.0, WIDTH, LENGTH), vec![], FemAxis::Z, -RAFT_THICKNESS, RAFT_THICKNESS, "concrete"),
        solid("ground_slab", "Ground Slab", rect(inner_x.0, inner_y.0, inner_x.1, inner_y.1), vec![], FemAxis::Z, 0.0, THICKNESS, "concrete"),
        solid("wall_west", "West Wall", rect(inner_y.0, 0.0, inner_y.1, STOREY), vec![window(1.5, 3.0), window(4.5, 6.0)], FemAxis::X, 0.0, THICKNESS, "masonry"),
        solid("wall_east", "East Wall", rect(inner_y.0, 0.0, inner_y.1, STOREY), vec![window(1.5, 3.0), window(4.5, 6.0)], FemAxis::X, WIDTH - THICKNESS, THICKNESS, "masonry"),
        solid("wall_south", "South Gable Wall", gable_outline(), vec![rect(2.5, THICKNESS, 3.5, 2.5), rect(4.0, 1.0, 5.0, 2.0)], FemAxis::Y, 0.0, THICKNESS, "masonry"),
        solid("wall_north", "North Gable Wall", gable_outline(), vec![rect(1.0, 1.0, 2.0, 2.0), rect(4.0, 1.0, 5.0, 2.0)], FemAxis::Y, LENGTH - THICKNESS, THICKNESS, "masonry"),
        solid("attic_slab", "Attic Slab", rect(inner_x.0, inner_y.0, inner_x.1, inner_y.1), vec![], FemAxis::Z, STOREY - THICKNESS, THICKNESS, "concrete"),
        solid("roof", "Gable Roof", roof_section(), vec![], FemAxis::Y, 0.0, LENGTH, "timber"),
    ];
    Fem3dSnapshot {
        nodes,
        elements: vec![],
        materials: vec![
            FemMaterial { id: "concrete".into(), name: "C30/37 Concrete".into(), e: 33e9, g: 13.75e9, nu: 0.2, rho: 2400.0 },
            FemMaterial { id: "masonry".into(), name: "Clay Brick Masonry".into(), e: 5e9, g: 2e9, nu: 0.25, rho: 1800.0 },
            FemMaterial { id: "timber".into(), name: "C24 Timber Roof".into(), e: 11e9, g: 0.69e9, nu: 0.3, rho: 420.0 },
        ],
        sections: vec![],
        solids,
        supports,
        load_cases: vec![
            FemLoadCase { id: "dead".into(), name: "Dead Load".into(), loads: vec![FemLoad::Area { id: "finish".into(), solid_id: "attic_slab".into(), pressure: 1000.0 }], self_weight: true },
            FemLoadCase { id: "live".into(), name: "Live Load".into(), loads: vec![FemLoad::Area { id: "attic_live".into(), solid_id: "attic_slab".into(), pressure: 2000.0 }, FemLoad::Area { id: "ground_live".into(), solid_id: "ground_slab".into(), pressure: 2000.0 }], self_weight: false },
            FemLoadCase { id: "snow".into(), name: "Snow".into(), loads: vec![FemLoad::Area { id: "roof_snow".into(), solid_id: "roof".into(), pressure: 1000.0 }], self_weight: false },
        ],
        combinations: vec![
            FemCombination { id: "uls".into(), name: "ULS".into(), terms: BTreeMap::from([("dead".into(), 1.35), ("live".into(), 1.5), ("snow".into(), 0.75)]) },
            FemCombination { id: "sls".into(), name: "SLS".into(), terms: BTreeMap::from([("dead".into(), 1.0), ("live".into(), 1.0), ("snow".into(), 0.5)]) },
        ],
        analysis: FemAnalysisSettings { modal_count: 3, buckling_count: 3, deformation_scale: 500.0 },
    }
}
//#endregion 🔖️Builder
