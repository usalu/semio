//! 🧩️ EN 1993 scene projection: the view modules of details, loads, plates, fatigue, torsion, annex blocks and the frame model, and the `fragment` that composes every module of the scene members. See [`semio_s_artifact_norm_en1993_scene_members`].

extern crate semio_s_artifact_norm_en1993_checks as checks;
extern crate semio_s_artifact_norm_en1993_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_mechanics as mechanics;
extern crate semio_s_plugin_norm_structural_scene as scene;
extern crate semio_s_artifact_norm_en1993_scene_members as base;

use base::kit::Out;
use base::layout::Layout;
use base::{eigen, handles, joint_rows, joints, kit, layout, members, plate_layout, tracks, SceneInput};
use scene::projection::{Fragment, Part};

#[path = "../🧩️detail-sketches/🦀️.rs"]
pub mod detail_sketches;
#[path = "../🏹️loads-and-supports/🦀️.rs"]
pub mod loads_and_supports;
#[path = "../🧱️plates-and-shells/🦀️.rs"]
pub mod plates_and_shells;
#[path = "../🧵️fatigue-views/🦀️.rs"]
pub mod fatigue_views;
#[path = "../🌀️torsion-views/🦀️.rs"]
pub mod torsion_views;
#[path = "../🔬️plate-details/🦀️.rs"]
pub mod plate_details;
#[path = "../🏛️annex-blocks/🦀️.rs"]
pub mod annex_blocks;
#[path = "../🕸️frame-model/🦀️.rs"]
pub mod frame_model;
#[path = "../🔩️plate-views/🦀️.rs"]
pub mod plate_views;
#[path = "../🎨️material-and-case-views/🦀️.rs"]
pub mod material_and_case_views;
#[path = "../🧮️norm-views/🦀️.rs"]
pub mod norm_views;

/// 🧩️ The part of the scene of `input` that belongs to `part`; `input.report` may be restricted to the rows the part reads.
pub fn fragment(input: &SceneInput, part: Part<'_>) -> Fragment {
    let layout = Layout::of(input.document, input.placements);
    let mut out = Out::scoped(part);
    members::project(input, &layout, &mut out);
    joints::project(input, &layout, &mut out);
    joint_rows::project(input, &layout, &mut out);
    plate_views::project(input, &layout, &mut out);
    detail_sketches::project(input, &layout, &mut out);
    frame_model::project(input, &layout, &mut out);
    plates_and_shells::project(input, &layout, &mut out);
    annex_blocks::project(input, &layout, &mut out);
    plate_details::project(input, &layout, &mut out);
    torsion_views::project(input, &layout, &mut out);
    fatigue_views::project(input, &layout, &mut out);
    loads_and_supports::project(input, &layout, &mut out);
    material_and_case_views::project(input, &layout, &mut out);
    norm_views::project(input, &layout, &mut out);
    scene::projection_kit::highlight_owners(out.into_fragment(), input.report, part)
}

