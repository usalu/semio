//! 🧪️ Ticket-local vector authoring harness (T12): prints an editor vocabulary's default state-lane snapshot, or applies
//! a hand-written mutation to a hand-written before-snapshot through production dispatch and prints the report, so each
//! specification vector is written against the real field shapes and reviewed before it is committed.
extern crate semio_framework_os_kernel as kernel;

use kernel::{FromValue, Mutation, ToValue};

fn shape<S: FromValue + ToValue>() -> Result<String, String> {
    let value: S = kernel::os_pack::json::from_json_str("{}").map_err(|error| error.to_string())?;
    Ok(kernel::os_pack::json::to_json_string(&value))
}

fn run<S: Clone + FromValue + ToValue, M: Mutation<S>>(before: &str, mutation: &str) -> Result<String, String> {
    let base: S = kernel::os_pack::json::from_json_str(before).map_err(|error| format!("before: {error}"))?;
    let mutation: M = kernel::os_pack::json::from_json_str(mutation).map_err(|error| format!("mutation: {error}"))?;
    let mut applied = base.clone();
    let forward = mutation.diff(&base).apply_to(&mut applied);
    let mut undone = applied.clone();
    for step in mutation.inverse(&base) {
        step.diff(&undone).apply_to(&mut undone);
    }
    let json = |value: kernel::DslValue| kernel::os_pack::json::from_dsl_value(&value);
    Ok(kernel::os_pack::json::to_string(&kernel::os_pack::json::object([
        ("before".to_string(), json(base.to_value())),
        ("mutation".to_string(), json(mutation.to_value())),
        ("after".to_string(), json(applied.to_value())),
        ("diff".to_string(), json(forward.diff().to_value())),
        ("messages".to_string(), json(forward.messages().to_vec().to_value())),
        ("restored".to_string(), json(undone.to_value())),
    ])))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let index: usize = args.get(1).and_then(|value| value.parse().ok()).expect("usage: shape|run <index> [before.json mutation.json]");
    let result = if args[0] == "shape" {
        match index {
        0 => shape::<semio_s_artifact_energy_model::editor::model::modes::edit::windows::model::config::EnergyModelWindowConfig>(),
        1 => shape::<semio_s_artifact_energy_model::editor::model::config::EnergyModelConfig>(),
        2 => shape::<semio_s_artifact_energy_model::viewer::model::modes::view::windows::model::config::EnergyModelViewerWindowConfig>(),
        3 => shape::<semio_s_artifact_playbook_playbook::editor::playbook::config::PlaybookConfig>(),
        4 => shape::<semio_s_artifact_fem_3d::editor::fem3d::modes::edit::windows::results::transient::Fem3dResultsWindowTransient>(),
        5 => shape::<semio_s_artifact_mathematical_equation::editor::equation::modes::edit::windows::graph::config::EquationGraphWindowConfig>(),
        6 => shape::<semio_s_artifact_shooting_shooting::editor::shooting::presence::ShootingPresence>(),
        7 => shape::<semio_s_artifact_shooting_shooting::editor::shooting::config::ShootingConfig>(),
        8 => shape::<semio_s_artifact_architect_program::editor::architect::presence::ArchitectPresence>(),
        9 => shape::<semio_s_artifact_architect_program::editor::architect::config::ArchitectConfig>(),
        10 => shape::<semio_s_artifact_forms_forms::editor::forms::config::FormsConfig>(),
        11 => shape::<semio_s_artifact_note_note::editor::note::presence::NotePresence>(),
        12 => shape::<semio_s_artifact_imperative_procedure::editor::procedure::config::ImperativeConfig>(),
        13 => shape::<semio_s_artifact_wfc_2d::editor::wfc2d::transient::Wfc2dTransient>(),
        14 => shape::<semio_s_artifact_wfc_2d::editor::wfc2d::config::Wfc2dConfig>(),
        15 => shape::<semio_s_artifact_wfc_bitmap::editor::bitmap::transient::BitmapTransient>(),
        16 => shape::<semio_s_artifact_wfc_3d::editor::wfc3d::transient::Wfc3dTransient>(),
        17 => shape::<semio_s_artifact_wfc_3d::editor::wfc3d::config::Wfc3dConfig>(),
        18 => shape::<semio_s_artifact_gis_gismap::editor::gis2d::modes::edit::windows::map::config::MapWindowConfig>(),
        19 => shape::<semio_s_artifact_procedural_generation2d::editor::generation2d::transient::Generation2dTransient>(),
        20 => shape::<semio_s_artifact_procedural_generation3d::editor::generation3d::transient::Generation3dTransient>(),
        21 => shape::<semio_s_artifact_procedural_generation3d::editor::generation3d::config::Generation3dConfig>(),
        22 => shape::<semio_s_artifact_procedural_generation3d::viewer::generation3d::transient::Generation3dViewTransient>(),
        23 => shape::<semio_s_artifact_procedural_generation3d::viewer::generation3d::presence::Generation3dViewPresence>(),
        24 => shape::<semio_s_artifact_procedural_generation3d::viewer::generation3d::config::Generation3dViewConfig>(),
        25 => shape::<semio_s_artifact_animate_presentation::editor::animate::presence::PresentationPresence>(),
        26 => shape::<semio_s_artifact_animate_presentation::editor::animate::config::PresentationConfig>(),
        27 => shape::<semio_s_artifact_writer_writer::editor::writer::modes::edit::windows::main::transient::WriterMainWindowTransient>(),
        28 => shape::<semio_s_artifact_writer_writer::editor::writer::modes::edit::windows::main::config::WriterMainWindowConfig>(),
        29 => shape::<semio_s_artifact_reasoning_wires::editor::wires::window_transient::WiresCanvasTransient>(),
        30 => shape::<semio_s_artifact_reasoning_wires::editor::wires::modes::edit::windows::canvas::config::WiresCanvasWindowConfig>(),
        31 => shape::<semio_s_artifact_dag_dag::editor::dag::presence::DagPresence>(),
        32 => shape::<semio_s_artifact_dag_dag::editor::dag::config::DagConfig>(),
        33 => shape::<semio_s_artifact_block_3d::editor::block3d::modes::edit::windows::world::transient::Block3dWorldWindowTransient>(),
        35 => shape::<semio_s_artifact_trinity_jack::editor::jack::window_config::JackGraphWindowConfig>(),
        36 => shape::<semio_s_artifact_trinity_jack::editor::jack::transient::JackResultsWindowTransient>(),
        37 => shape::<semio_s_artifact_trinity_jack::editor::jack::transient::JackEditorWindowTransient>(),
        38 => shape::<semio_s_artifact_trinity_jack::editor::jack::query_window_config::JackEditorWindowConfig>(),
        39 => shape::<semio_s_artifact_gis_gismap::viewer::gismap::modes::view::windows::map::config::GisMapViewerWindowConfig>(),
            _ => Err(format!("no vocabulary {index}")),
        }
    } else {
        let before = std::fs::read_to_string(&args[2]).expect("before file");
        let mutation = std::fs::read_to_string(&args[3]).expect("mutation file");
        match index {
        0 => run::<semio_s_artifact_energy_model::editor::model::modes::edit::windows::model::config::EnergyModelWindowConfig, semio_s_artifact_energy_model::editor::model::modes::edit::windows::model::config::EnergyModelWindowConfigMutation>(&before, &mutation),
        1 => run::<semio_s_artifact_energy_model::editor::model::config::EnergyModelConfig, semio_s_artifact_energy_model::editor::model::config::mutations::EnergyModelConfigMutation>(&before, &mutation),
        2 => run::<semio_s_artifact_energy_model::viewer::model::modes::view::windows::model::config::EnergyModelViewerWindowConfig, semio_s_artifact_energy_model::viewer::model::modes::view::windows::model::config::EnergyModelViewerWindowConfigMutation>(&before, &mutation),
        3 => run::<semio_s_artifact_playbook_playbook::editor::playbook::config::PlaybookConfig, semio_s_artifact_playbook_playbook::editor::playbook::config::PlaybookConfigMutation>(&before, &mutation),
        4 => run::<semio_s_artifact_fem_3d::editor::fem3d::modes::edit::windows::results::transient::Fem3dResultsWindowTransient, semio_s_artifact_fem_3d::editor::fem3d::modes::edit::windows::results::transient::Fem3dResultsWindowTransientMutation>(&before, &mutation),
        5 => run::<semio_s_artifact_mathematical_equation::editor::equation::modes::edit::windows::graph::config::EquationGraphWindowConfig, semio_s_artifact_mathematical_equation::editor::equation::modes::edit::windows::graph::config::EquationGraphWindowConfigMutation>(&before, &mutation),
        6 => run::<semio_s_artifact_shooting_shooting::editor::shooting::presence::ShootingPresence, semio_s_artifact_shooting_shooting::editor::shooting::presence::ShootingPresenceMutation>(&before, &mutation),
        7 => run::<semio_s_artifact_shooting_shooting::editor::shooting::config::ShootingConfig, semio_s_artifact_shooting_shooting::editor::shooting::config::ShootingConfigMutation>(&before, &mutation),
        8 => run::<semio_s_artifact_architect_program::editor::architect::presence::ArchitectPresence, semio_s_artifact_architect_program::editor::architect::presence::ArchitectPresenceMutation>(&before, &mutation),
        9 => run::<semio_s_artifact_architect_program::editor::architect::config::ArchitectConfig, semio_s_artifact_architect_program::editor::architect::config::ArchitectConfigMutation>(&before, &mutation),
        10 => run::<semio_s_artifact_forms_forms::editor::forms::config::FormsConfig, semio_s_artifact_forms_forms::editor::forms::config::FormsConfigMutation>(&before, &mutation),
        11 => run::<semio_s_artifact_note_note::editor::note::presence::NotePresence, semio_s_artifact_note_note::editor::note::presence::NotePresenceMutation>(&before, &mutation),
        12 => run::<semio_s_artifact_imperative_procedure::editor::procedure::config::ImperativeConfig, semio_s_artifact_imperative_procedure::editor::procedure::config::ImperativeConfigMutation>(&before, &mutation),
        13 => run::<semio_s_artifact_wfc_2d::editor::wfc2d::transient::Wfc2dTransient, semio_s_artifact_wfc_2d::editor::wfc2d::transient::mutations::Wfc2dTransientMutation>(&before, &mutation),
        14 => run::<semio_s_artifact_wfc_2d::editor::wfc2d::config::Wfc2dConfig, semio_s_artifact_wfc_2d::editor::wfc2d::config::mutations::Wfc2dConfigMutation>(&before, &mutation),
        15 => run::<semio_s_artifact_wfc_bitmap::editor::bitmap::transient::BitmapTransient, semio_s_artifact_wfc_bitmap::editor::bitmap::transient::BitmapTransientMutation>(&before, &mutation),
        16 => run::<semio_s_artifact_wfc_3d::editor::wfc3d::transient::Wfc3dTransient, semio_s_artifact_wfc_3d::editor::wfc3d::transient::mutations::Wfc3dTransientMutation>(&before, &mutation),
        17 => run::<semio_s_artifact_wfc_3d::editor::wfc3d::config::Wfc3dConfig, semio_s_artifact_wfc_3d::editor::wfc3d::config::mutations::Wfc3dConfigMutation>(&before, &mutation),
        18 => run::<semio_s_artifact_gis_gismap::editor::gis2d::modes::edit::windows::map::config::MapWindowConfig, semio_s_artifact_gis_gismap::editor::gis2d::modes::edit::windows::map::config::mutations::MapWindowConfigMutation>(&before, &mutation),
        19 => run::<semio_s_artifact_procedural_generation2d::editor::generation2d::transient::Generation2dTransient, semio_s_artifact_procedural_generation2d::editor::generation2d::transient::Generation2dTransientMutation>(&before, &mutation),
        20 => run::<semio_s_artifact_procedural_generation3d::editor::generation3d::transient::Generation3dTransient, semio_s_artifact_procedural_generation3d::editor::generation3d::transient::Generation3dTransientMutation>(&before, &mutation),
        21 => run::<semio_s_artifact_procedural_generation3d::editor::generation3d::config::Generation3dConfig, semio_s_artifact_procedural_generation3d::editor::generation3d::config::Generation3dConfigMutation>(&before, &mutation),
        22 => run::<semio_s_artifact_procedural_generation3d::viewer::generation3d::transient::Generation3dViewTransient, semio_s_artifact_procedural_generation3d::viewer::generation3d::transient::Generation3dViewTransientMutation>(&before, &mutation),
        23 => run::<semio_s_artifact_procedural_generation3d::viewer::generation3d::presence::Generation3dViewPresence, semio_s_artifact_procedural_generation3d::viewer::generation3d::presence::Generation3dViewPresenceMutation>(&before, &mutation),
        24 => run::<semio_s_artifact_procedural_generation3d::viewer::generation3d::config::Generation3dViewConfig, semio_s_artifact_procedural_generation3d::viewer::generation3d::config::Generation3dViewConfigMutation>(&before, &mutation),
        25 => run::<semio_s_artifact_animate_presentation::editor::animate::presence::PresentationPresence, semio_s_artifact_animate_presentation::editor::animate::presence::PresentationPresenceMutation>(&before, &mutation),
        26 => run::<semio_s_artifact_animate_presentation::editor::animate::config::PresentationConfig, semio_s_artifact_animate_presentation::editor::animate::config::PresentationConfigMutation>(&before, &mutation),
        27 => run::<semio_s_artifact_writer_writer::editor::writer::modes::edit::windows::main::transient::WriterMainWindowTransient, semio_s_artifact_writer_writer::editor::writer::modes::edit::windows::main::transient::WriterMainWindowTransientMutation>(&before, &mutation),
        28 => run::<semio_s_artifact_writer_writer::editor::writer::modes::edit::windows::main::config::WriterMainWindowConfig, semio_s_artifact_writer_writer::editor::writer::modes::edit::windows::main::config::WriterMainWindowConfigMutation>(&before, &mutation),
        29 => run::<semio_s_artifact_reasoning_wires::editor::wires::window_transient::WiresCanvasTransient, semio_s_artifact_reasoning_wires::editor::wires::window_transient::WiresCanvasTransientMutation>(&before, &mutation),
        30 => run::<semio_s_artifact_reasoning_wires::editor::wires::modes::edit::windows::canvas::config::WiresCanvasWindowConfig, semio_s_artifact_reasoning_wires::editor::wires::modes::edit::windows::canvas::config::WiresCanvasWindowConfigMutation>(&before, &mutation),
        31 => run::<semio_s_artifact_dag_dag::editor::dag::presence::DagPresence, semio_s_artifact_dag_dag::editor::dag::presence::mutations::DagPresenceMutation>(&before, &mutation),
        32 => run::<semio_s_artifact_dag_dag::editor::dag::config::DagConfig, semio_s_artifact_dag_dag::editor::dag::config::mutations::DagConfigMutation>(&before, &mutation),
        33 => run::<semio_s_artifact_block_3d::editor::block3d::modes::edit::windows::world::transient::Block3dWorldWindowTransient, semio_s_artifact_block_3d::editor::block3d::modes::edit::windows::world::transient::Block3dWorldWindowTransientMutation>(&before, &mutation),
        35 => run::<semio_s_artifact_trinity_jack::editor::jack::window_config::JackGraphWindowConfig, semio_s_artifact_trinity_jack::editor::jack::window_config::JackGraphWindowConfigMutation>(&before, &mutation),
        36 => run::<semio_s_artifact_trinity_jack::editor::jack::transient::JackResultsWindowTransient, semio_s_artifact_trinity_jack::editor::jack::transient::JackResultsWindowTransientMutation>(&before, &mutation),
        37 => run::<semio_s_artifact_trinity_jack::editor::jack::transient::JackEditorWindowTransient, semio_s_artifact_trinity_jack::editor::jack::transient::JackEditorWindowTransientMutation>(&before, &mutation),
        38 => run::<semio_s_artifact_trinity_jack::editor::jack::query_window_config::JackEditorWindowConfig, semio_s_artifact_trinity_jack::editor::jack::query_window_config::JackEditorWindowConfigMutation>(&before, &mutation),
        39 => run::<semio_s_artifact_gis_gismap::viewer::gismap::modes::view::windows::map::config::GisMapViewerWindowConfig, semio_s_artifact_gis_gismap::viewer::gismap::modes::view::windows::map::config::GisMapViewerWindowConfigMutation>(&before, &mutation),
            _ => Err(format!("no vocabulary {index}")),
        }
    };
    match result {
        Ok(text) => println!("{text}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
