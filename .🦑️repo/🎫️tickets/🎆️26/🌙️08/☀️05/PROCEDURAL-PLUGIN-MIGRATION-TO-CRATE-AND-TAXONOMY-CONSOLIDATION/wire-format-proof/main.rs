//! 🔬 Scratch wire-format-diff proof (ticket-local, not part of any crate's permanent source).
//!
//! Proves that the new consolidated `semio-s-plugin-procedural` crate's `Procedural2dCommand`/
//! `Procedural3dCommand` enums encode/print BYTE- and TEXT-identically to the OLD, still-on-disk
//! `semio-s-app-procedural-{2d,3d}-protocol` crates' hand-rolled `Procedural{2d,3d}Command` enums,
//! for one representative value per row (same literals `every_command()` already uses in the new
//! crate's own tests, and — where the old crate had its own literal test fixtures — the exact same
//! literals as those old tests, for maximum apples-to-apples signal).

use protocol::{OpBinary, OpText};

use old_2d_protocol::Procedural2dCommand as Old2d;
use old_3d_protocol::Procedural3dCommand as Old3d;

use new_procedural::apps::procedural2d::commands::eval::{flow_eval_tick as n2_flow_eval_tick, set_eval_outputs as n2_set_eval_outputs};
use new_procedural::apps::procedural2d::commands::generation::{
    add_generation as n2_add_generation, enter_generate as n2_enter_generate, remove_generation as n2_remove_generation, rename_generation as n2_rename_generation, select_generation as n2_select_generation, update_generation_values as n2_update_generation_values,
};
use new_procedural::apps::procedural2d::commands::graph::{
    connect_media_ports as n2_connect_media_ports, move_media_node as n2_move_media_node, node_graph_edit as n2_node_graph_edit, node_graph_hover as n2_node_graph_hover, node_graph_select as n2_node_graph_select, node_graph_viewport as n2_node_graph_viewport, reorganize as n2_reorganize,
};
use new_procedural::apps::procedural2d::commands::locale::set_locale as n2_set_locale;
use new_procedural::apps::procedural2d::commands::selection::{select_node as n2_select_node, set_selection as n2_set_selection};
use new_procedural::apps::procedural2d::commands::view::{canvas_pointer_down as n2_canvas_pointer_down, canvas_pointer_move as n2_canvas_pointer_move, canvas_pointer_up as n2_canvas_pointer_up, canvas_wheel as n2_canvas_wheel, set_show_mode as n2_set_show_mode};
use new_procedural::apps::procedural2d::commands::widget::{add_widget as n2_add_widget, remove_widget as n2_remove_widget};
use new_procedural::apps::procedural2d::Procedural2dCommand as New2d;

use new_procedural::apps::procedural3d::commands::eval::{flow_eval_resolve as n3_flow_eval_resolve, flow_eval_tick as n3_flow_eval_tick};
use new_procedural::apps::procedural3d::commands::example::set_active_example as n3_set_active_example;
use new_procedural::apps::procedural3d::commands::generation::{
    add_generation as n3_add_generation, remove_generation as n3_remove_generation, rename_generation as n3_rename_generation, select_generation as n3_select_generation, update_generation_values as n3_update_generation_values,
};
use new_procedural::apps::procedural3d::commands::graph::{
    graph_pointer_down as n3_graph_pointer_down, move_media_node as n3_move_media_node, node_graph_edit as n3_node_graph_edit, node_graph_hover as n3_node_graph_hover, node_graph_select as n3_node_graph_select, node_graph_viewport as n3_node_graph_viewport, reorganize as n3_reorganize,
};
use new_procedural::apps::procedural3d::commands::gumball::{rotate_selection as n3_rotate_selection, scale_selection as n3_scale_selection, translate_selection as n3_translate_selection};
use new_procedural::apps::procedural3d::commands::locale::{set_contributions as n3_set_contributions, set_locale as n3_set_locale};
use new_procedural::apps::procedural3d::commands::selection::{
    select_node as n3_select_node, set_hover as n3_set_hover, set_selection as n3_set_selection, set_selection_method as n3_set_selection_method, world_hover as n3_world_hover, world_pointer_down as n3_world_pointer_down, world_select as n3_world_select,
};
use new_procedural::apps::procedural3d::commands::sun::{set_sun_azimuth as n3_set_sun_azimuth, set_sun_elevation as n3_set_sun_elevation, set_sun_intensity as n3_set_sun_intensity, toggle_sun as n3_toggle_sun};
use new_procedural::apps::procedural3d::commands::view::{set_active_utility as n3_set_active_utility, set_camera as n3_set_camera, set_lod_mode as n3_set_lod_mode, set_show_mode as n3_set_show_mode};
use new_procedural::apps::procedural3d::commands::widget::{add_widget as n3_add_widget, delete_selection as n3_delete_selection, patch_flow_widgets as n3_patch_flow_widgets, remove_widget as n3_remove_widget};
use new_procedural::apps::procedural3d::config::Procedural3dPreviewCamera as NewPreviewCamera;
use new_procedural::apps::procedural3d::Procedural3dCommand as New3d;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn cmp<A: OpText + OpBinary, B: OpText + OpBinary>(name: &str, old: &A, new: &B, failures: &mut Vec<String>) {
    let old_text = old.print_op();
    let new_text = new.print_op();
    let old_bytes = old.encode_op().unwrap_or_else(|e| panic!("{name}: old encode failed: {e:?}"));
    let new_bytes = new.encode_op().unwrap_or_else(|e| panic!("{name}: new encode failed: {e:?}"));
    let mut ok = true;
    if old_text != new_text {
        failures.push(format!("{name}: TEXT MISMATCH\n  old: {old_text:?}\n  new: {new_text:?}"));
        ok = false;
    }
    if old_bytes != new_bytes {
        failures.push(format!("{name}: BYTES MISMATCH\n  old: {}\n  new: {}", hex(&old_bytes), hex(&new_bytes)));
        ok = false;
    }
    if ok {
        println!("OK   {name:<48} {old_text:?}");
    } else {
        println!("FAIL {name}");
    }
}

fn main() {
    let mut failures: Vec<String> = Vec::new();

    //#region 🔖️Procedural2d
    cmp("2d.NodeGraphEdit", &Old2d::NodeGraphEdit { operations_json: "[]".into() }, &New2d::NodeGraphEdit(n2_node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }), &mut failures);
    cmp("2d.MoveMediaNode", &Old2d::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 }, &New2d::MoveMediaNode(n2_move_media_node::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 }), &mut failures);
    cmp(
        "2d.AddWidget",
        &Old2d::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: Some(10.0), y: None },
        &New2d::AddWidget(n2_add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: Some(10.0), y: None }),
        &mut failures,
    );
    cmp("2d.RemoveWidget", &Old2d::RemoveWidget { widget_id: "n1".into() }, &New2d::RemoveWidget(n2_remove_widget::RemoveWidget { widget_id: "n1".into() }), &mut failures);
    cmp(
        "2d.ConnectMediaPorts",
        &Old2d::ConnectMediaPorts { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() },
        &New2d::ConnectMediaPorts(n2_connect_media_ports::ConnectMediaPorts { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() }),
        &mut failures,
    );
    cmp("2d.Reorganize", &Old2d::Reorganize, &New2d::Reorganize(n2_reorganize::Reorganize {}), &mut failures);
    cmp("2d.AddGeneration", &Old2d::AddGeneration, &New2d::AddGeneration(n2_add_generation::AddGeneration {}), &mut failures);
    cmp("2d.RemoveGeneration", &Old2d::RemoveGeneration { id: "g1".into() }, &New2d::RemoveGeneration(n2_remove_generation::RemoveGeneration { id: "g1".into() }), &mut failures);
    cmp("2d.RenameGeneration", &Old2d::RenameGeneration { id: "g1".into(), name: "Copy".into() }, &New2d::RenameGeneration(n2_rename_generation::RenameGeneration { id: "g1".into(), name: "Copy".into() }), &mut failures);
    cmp(
        "2d.UpdateGenerationValues",
        &Old2d::UpdateGenerationValues { generation_id: Some("g1".into()), question_id: "q1".into(), value: dsl::DslValue::Number(5.0) },
        &New2d::UpdateGenerationValues(n2_update_generation_values::UpdateGenerationValues { generation_id: Some("g1".into()), question_id: "q1".into(), value: dsl::DslValue::Number(5.0) }),
        &mut failures,
    );
    cmp("2d.NodeGraphViewport", &Old2d::NodeGraphViewport { viewport_json: "{}".into() }, &New2d::NodeGraphViewport(n2_node_graph_viewport::NodeGraphViewport { viewport_json: "{}".into() }), &mut failures);
    cmp("2d.SetSelection", &Old2d::SetSelection { ids: vec!["n1".into()] }, &New2d::SetSelection(n2_set_selection::SetSelection { ids: vec!["n1".into()] }), &mut failures);
    cmp("2d.SelectNode", &Old2d::SelectNode { ids: vec!["n1".into()] }, &New2d::SelectNode(n2_select_node::SelectNode { ids: vec!["n1".into()] }), &mut failures);
    cmp("2d.NodeGraphSelect", &Old2d::NodeGraphSelect { ids: vec!["n1".into(), "n2".into()] }, &New2d::NodeGraphSelect(n2_node_graph_select::NodeGraphSelect { ids: vec!["n1".into(), "n2".into()] }), &mut failures);
    cmp("2d.NodeGraphHover", &Old2d::NodeGraphHover, &New2d::NodeGraphHover(n2_node_graph_hover::NodeGraphHover {}), &mut failures);
    cmp("2d.SetShowMode", &Old2d::SetShowMode { value: "wire".into() }, &New2d::SetShowMode(n2_set_show_mode::SetShowMode { value: "wire".into() }), &mut failures);
    cmp("2d.Generate", &Old2d::Generate, &New2d::Generate(n2_enter_generate::Generate {}), &mut failures);
    cmp("2d.SetEvalOutputs", &Old2d::SetEvalOutputs { outputs_json: "{}".into() }, &New2d::SetEvalOutputs(n2_set_eval_outputs::SetEvalOutputs { outputs_json: "{}".into() }), &mut failures);
    cmp("2d.CanvasPointerDown", &Old2d::CanvasPointerDown, &New2d::CanvasPointerDown(n2_canvas_pointer_down::CanvasPointerDown {}), &mut failures);
    cmp("2d.CanvasPointerMove", &Old2d::CanvasPointerMove, &New2d::CanvasPointerMove(n2_canvas_pointer_move::CanvasPointerMove {}), &mut failures);
    cmp("2d.CanvasPointerUp", &Old2d::CanvasPointerUp, &New2d::CanvasPointerUp(n2_canvas_pointer_up::CanvasPointerUp {}), &mut failures);
    cmp("2d.CanvasWheel", &Old2d::CanvasWheel, &New2d::CanvasWheel(n2_canvas_wheel::CanvasWheel {}), &mut failures);
    cmp("2d.SelectGeneration", &Old2d::SelectGeneration { id: Some("g1".into()) }, &New2d::SelectGeneration(n2_select_generation::SelectGeneration { id: Some("g1".into()) }), &mut failures);
    cmp("2d.FlowEvalTick", &Old2d::FlowEvalTick, &New2d::FlowEvalTick(n2_flow_eval_tick::FlowEvalTick {}), &mut failures);
    cmp("2d.SetLocale", &Old2d::SetLocale { value: "de-DE".into() }, &New2d::SetLocale(n2_set_locale::SetLocale { value: "de-DE".into() }), &mut failures);
    //#endregion 🔖️Procedural2d

    //#region 🔖️Procedural3d
    cmp("3d.SetActiveExample", &Old3d::SetActiveExample { example_id: "hexagonal-mushroom-column".into() }, &New3d::SetActiveExample(n3_set_active_example::SetActiveExample { example_id: "hexagonal-mushroom-column".into() }), &mut failures);
    cmp("3d.NodeGraphEdit", &Old3d::NodeGraphEdit { operations_json: "[]".into() }, &New3d::NodeGraphEdit(n3_node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }), &mut failures);
    cmp("3d.DeleteSelection", &Old3d::DeleteSelection, &New3d::DeleteSelection(n3_delete_selection::DeleteSelection {}), &mut failures);
    cmp("3d.RemoveWidget", &Old3d::RemoveWidget { widget_id: "extrude".into() }, &New3d::RemoveWidget(n3_remove_widget::RemoveWidget { widget_id: "extrude".into() }), &mut failures);
    cmp("3d.MoveMediaNode", &Old3d::MoveMediaNode { node_id: "extrude".into(), x: 1.0, y: 2.0 }, &New3d::MoveMediaNode(n3_move_media_node::MoveMediaNode { node_id: "extrude".into(), x: 1.0, y: 2.0 }), &mut failures);
    cmp("3d.AddWidget", &Old3d::AddWidget { kind: "inputSlider".into(), x: Some(10.0), y: None }, &New3d::AddWidget(n3_add_widget::AddWidget { kind: "inputSlider".into(), x: Some(10.0), y: None }), &mut failures);
    cmp(
        "3d.PatchFlowWidgets",
        &Old3d::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5) },
        &New3d::PatchFlowWidgets(n3_patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5) }),
        &mut failures,
    );
    cmp("3d.Reorganize", &Old3d::Reorganize, &New3d::Reorganize(n3_reorganize::Reorganize {}), &mut failures);
    cmp(
        "3d.TranslateSelection",
        &Old3d::TranslateSelection { node_ids: vec!["extrude".into()], dx: 1.0, dy: 2.0, dz: 3.0 },
        &New3d::TranslateSelection(n3_translate_selection::TranslateSelection { node_ids: vec!["extrude".into()], dx: 1.0, dy: 2.0, dz: 3.0 }),
        &mut failures,
    );
    cmp(
        "3d.RotateSelection",
        &Old3d::RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.5 },
        &New3d::RotateSelection(n3_rotate_selection::RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.5 }),
        &mut failures,
    );
    cmp(
        "3d.ScaleSelection",
        &Old3d::ScaleSelection { node_ids: vec!["extrude".into()], sx: 2.0, sy: 2.0, sz: 2.0 },
        &New3d::ScaleSelection(n3_scale_selection::ScaleSelection { node_ids: vec!["extrude".into()], sx: 2.0, sy: 2.0, sz: 2.0 }),
        &mut failures,
    );
    cmp("3d.AddGeneration", &Old3d::AddGeneration, &New3d::AddGeneration(n3_add_generation::AddGeneration {}), &mut failures);
    cmp("3d.RemoveGeneration", &Old3d::RemoveGeneration { id: "generation-1".into() }, &New3d::RemoveGeneration(n3_remove_generation::RemoveGeneration { id: "generation-1".into() }), &mut failures);
    cmp("3d.RenameGeneration", &Old3d::RenameGeneration { id: "generation-1".into(), name: "Renamed".into() }, &New3d::RenameGeneration(n3_rename_generation::RenameGeneration { id: "generation-1".into(), name: "Renamed".into() }), &mut failures);
    cmp(
        "3d.UpdateGenerationValues",
        &Old3d::UpdateGenerationValues { generation_id: Some("generation-1".into()), question_id: "q1".into(), value: dsl::DslValue::Number(5.0) },
        &New3d::UpdateGenerationValues(n3_update_generation_values::UpdateGenerationValues { generation_id: Some("generation-1".into()), question_id: "q1".into(), value: dsl::DslValue::Number(5.0) }),
        &mut failures,
    );
    cmp(
        "3d.NodeGraphViewport",
        &Old3d::NodeGraphViewport { camera: flow_core::CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } },
        &New3d::NodeGraphViewport(n3_node_graph_viewport::NodeGraphViewport { camera: flow_core::CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } }),
        &mut failures,
    );
    cmp("3d.SetSelection", &Old3d::SetSelection { node_ids: vec!["a".into()] }, &New3d::SetSelection(n3_set_selection::SetSelection { node_ids: vec!["a".into()] }), &mut failures);
    cmp("3d.SelectNode", &Old3d::SelectNode { node_ids: vec!["a".into()] }, &New3d::SelectNode(n3_select_node::SelectNode { node_ids: vec!["a".into()] }), &mut failures);
    cmp("3d.NodeGraphSelect", &Old3d::NodeGraphSelect { node_ids: vec!["a".into()] }, &New3d::NodeGraphSelect(n3_node_graph_select::NodeGraphSelect { node_ids: vec!["a".into()] }), &mut failures);
    cmp("3d.NodeGraphHover", &Old3d::NodeGraphHover { widget_id: Some("extrude".into()) }, &New3d::NodeGraphHover(n3_node_graph_hover::NodeGraphHover { widget_id: Some("extrude".into()) }), &mut failures);
    cmp("3d.SetHover", &Old3d::SetHover { object_id: None }, &New3d::SetHover(n3_set_hover::SetHover { object_id: None }), &mut failures);
    cmp("3d.WorldPointerDown", &Old3d::WorldPointerDown, &New3d::WorldPointerDown(n3_world_pointer_down::WorldPointerDown {}), &mut failures);
    cmp("3d.GraphPointerDown", &Old3d::GraphPointerDown, &New3d::GraphPointerDown(n3_graph_pointer_down::GraphPointerDown {}), &mut failures);
    cmp("3d.WorldSelect", &Old3d::WorldSelect { ids: vec!["a".into()], merge: "replace".into() }, &New3d::WorldSelect(n3_world_select::WorldSelect { ids: vec!["a".into()], merge: "replace".into() }), &mut failures);
    cmp("3d.WorldHover", &Old3d::WorldHover { id: Some("a".into()) }, &New3d::WorldHover(n3_world_hover::WorldHover { id: Some("a".into()) }), &mut failures);
    cmp("3d.SetSelectionMethod", &Old3d::SetSelectionMethod { method: "lasso".into() }, &New3d::SetSelectionMethod(n3_set_selection_method::SetSelectionMethod { method: "lasso".into() }), &mut failures);
    cmp("3d.SetLodMode", &Old3d::SetLodMode { value: "coarse".into() }, &New3d::SetLodMode(n3_set_lod_mode::SetLodMode { value: "coarse".into() }), &mut failures);
    cmp("3d.SetShowMode", &Old3d::SetShowMode { value: "wireframe".into() }, &New3d::SetShowMode(n3_set_show_mode::SetShowMode { value: "wireframe".into() }), &mut failures);
    cmp("3d.ToggleSun", &Old3d::ToggleSun, &New3d::ToggleSun(n3_toggle_sun::ToggleSun {}), &mut failures);
    cmp("3d.SetSunAzimuth", &Old3d::SetSunAzimuth { value: 90.0 }, &New3d::SetSunAzimuth(n3_set_sun_azimuth::SetSunAzimuth { value: 90.0 }), &mut failures);
    cmp("3d.SetSunElevation", &Old3d::SetSunElevation { value: 45.0 }, &New3d::SetSunElevation(n3_set_sun_elevation::SetSunElevation { value: 45.0 }), &mut failures);
    cmp("3d.SetSunIntensity", &Old3d::SetSunIntensity { value: 1.0 }, &New3d::SetSunIntensity(n3_set_sun_intensity::SetSunIntensity { value: 1.0 }), &mut failures);
    cmp(
        "3d.SetCamera",
        &Old3d::SetCamera { camera: old_3d_engine::Procedural3dPreviewCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 } },
        &New3d::SetCamera(n3_set_camera::SetCamera { camera: NewPreviewCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 } }),
        &mut failures,
    );
    cmp("3d.SelectGeneration", &Old3d::SelectGeneration { id: "generation-1".into() }, &New3d::SelectGeneration(n3_select_generation::SelectGeneration { id: "generation-1".into() }), &mut failures);
    cmp("3d.SetActiveUtility", &Old3d::SetActiveUtility { utility_id: "rotate".into() }, &New3d::SetActiveUtility(n3_set_active_utility::SetActiveUtility { utility_id: "rotate".into() }), &mut failures);
    cmp("3d.SetLocale", &Old3d::SetLocale { value: "de-DE".into() }, &New3d::SetLocale(n3_set_locale::SetLocale { value: "de-DE".into() }), &mut failures);
    cmp("3d.SetContributions", &Old3d::SetContributions { json: "[]".into() }, &New3d::SetContributions(n3_set_contributions::SetContributions { json: "[]".into() }), &mut failures);
    cmp("3d.FlowEvalTick", &Old3d::FlowEvalTick, &New3d::FlowEvalTick(n3_flow_eval_tick::FlowEvalTick {}), &mut failures);
    cmp(
        "3d.FlowEvalResolve",
        &Old3d::FlowEvalResolve { node_hash: 42, output_json: "{}".into() },
        &New3d::FlowEvalResolve(n3_flow_eval_resolve::FlowEvalResolve { node_hash: 42, output_json: "{}".into() }),
        &mut failures,
    );
    //#endregion 🔖️Procedural3d

    println!();
    println!("=== {} rows checked, {} failures ===", 25 + 39, failures.len());
    if !failures.is_empty() {
        for f in &failures {
            println!("\n{f}");
        }
        std::process::exit(1);
    }
}
