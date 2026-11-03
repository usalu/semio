//! 🧩️ EN 1991 scene projection: the traffic and hazard modules and the `fragment` that composes every module of the scene members into the part of the scene a caller asks for. See [`semio_s_artifact_norm_en1991_scene_members`].

extern crate semio_s_artifact_norm_en1991_checks as checks;
extern crate semio_s_artifact_norm_en1991_model as model;
extern crate semio_s_artifact_norm_en1991_scene_fields as fields;
extern crate semio_s_artifact_norm_en1991_scene_members as members;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

use members::kit::Out;
use members::layout::Layout;
use members::{accidental, cases, collections, dynamics, envelope, gravity, handles, kit, layout, silo, thermal_fire, wave3, wave3_rows, wind, Ctx, SceneInput};
use scene::projection::{Fragment, Part};

#[path = "../🌉️traffic/🦀️.rs"]
pub mod traffic;
#[path = "../❄️snow/🦀️.rs"]
pub mod snow;
#[path = "../🌀️deflection/🦀️.rs"]
pub mod deflection;
#[path = "../🚁hazards/🦀️.rs"]
pub mod hazards;

/// 🧩️ The part of the scene of `input` that belongs to `part`: every scene entity is owned by one snapshot row or one global entity (`kit::global`), and each module emits only what its guard `Out::own` lets through.
pub fn fragment(input: &SceneInput, part: Part<'_>) -> Fragment {
    let layout = Layout::of(input.document, input.placements);
    let ctx = Ctx { input, layout: &layout };
    let mut out = Out::scoped(part);
    envelope::project(&ctx, &mut out);
    wind::project(&ctx, &mut out);
    snow::project(&ctx, &mut out);
    gravity::floors(&ctx, &mut out);
    gravity::stack(&ctx, &mut out);
    thermal_fire::thermal(&ctx, &mut out);
    thermal_fire::fire(&ctx, &mut out);
    accidental::project(&ctx, &mut out);
    traffic::bridge(&ctx, &mut out);
    traffic::crane(&ctx, &mut out);
    collections::stages(&ctx, &mut out);
    collections::surfaces(&ctx, &mut out);
    silo::project(&ctx, &mut out);
    wave3::vehicles(&ctx, &mut out);
    wave3::bridge_wind(&ctx, &mut out);
    wave3::dynamic(&ctx, &mut out);
    wave3::impacts(&ctx, &mut out);
    wave3::explosions(&ctx, &mut out);
    wave3::thermal_sections(&ctx, &mut out);
    hazards::helicopters(&ctx, &mut out);
    hazards::ties(&ctx, &mut out);
    hazards::construction_loads(&ctx, &mut out);
    hazards::falling_loads(&ctx, &mut out);
    hazards::imperfections(&ctx, &mut out);
    hazards::derailments(&ctx, &mut out);
    traffic::hoists(&ctx, &mut out);
    collections::footbridges(&ctx, &mut out);
    collections::rail_dynamics(&ctx, &mut out);
    collections::dispersals(&ctx, &mut out);
    collections::bulk_solids(&ctx, &mut out);
    collections::machines(&ctx, &mut out);
    wave3_rows::fatigue_models(&ctx, &mut out);
    wave3_rows::support_collisions(&ctx, &mut out);
    wave3_rows::local_fires(&ctx, &mut out);
    wave3_rows::snow_guards(&ctx, &mut out);
    deflection::project(&ctx, &mut out);
    cases::project(&ctx, &mut out);
    scene::projection_kit::highlight_owners(out.into_fragment(), input.report, part)
}

