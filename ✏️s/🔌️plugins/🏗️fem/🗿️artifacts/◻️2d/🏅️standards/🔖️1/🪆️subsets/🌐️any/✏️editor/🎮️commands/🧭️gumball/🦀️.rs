//! 📡️ (`🕹️interaction/🧭️gumball`): one dispatch without a `phase` is ONE tool transaction; a host streaming a gesture
//! 🧭️ Fem2d transform gumball commands — translate, rotate and scale the selection through the gumball TOOL
//! sends `phase: "stream"` ticks that accumulate in the window's ONE open transaction (previewed by every window,
//! never history) until `phase: "commit"` commits the net `move-selection` leaf as one edit or `phase: "abort"`
//! (with a `reason`) drops it with zero trace — plus the per-window handle flags the Transform utility's options
//! rail toggles.

use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::gumball::fem2d_gumball_tick;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::gumball::Fem2dGumballMotion;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::gumball::Fem2dGumballTool;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model as model_window;
use semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results as results_window;
use semio_s_artifact_fem_2d_editor_transient::editor::fem2d::transient::fem_gumball_drive;
use semio_s_artifact_fem_2d_editor_transient::editor::fem2d::transient::FemGumballTransientMutation;
use crate::editor::fem2d::Fem2dPlayApp;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_framework::kernel::UiDirtyScope;
use semio_plugin::retained_command::ArtifactCommandWorkStep;
use semio_plugin::AppOperationContext;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::EditorApp;
use semio_plugin::Emit;
use semio_plugin::EphemeralEmit;
use semio_plugin::Fault;
use semio_plugin::NoConfig;
use semio_plugin::NoConfigMutation;
use semio_plugin::ViewModel;
use semio_framework_tool_machine::GesturePhase;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

/// 🪟️ The dirty scope a gumball dispatch declares: both canvas windows paint the gesture or its commit.
fn fem2d_gumball_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: vec![model_window::BODY_KEY.into(), results_window::BODY_KEY.into()],
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: false,
        measures: false,
        labels: false,
    }
}

/// 🛠️ The retained route of every gumball verb: builds the dispatch's relative tick on the committed `snapshot`
/// (an id-less payload moves the live `fem2d` selection), drives the dispatching window's gumball tool through its
/// phase against the gesture the FEM gumball transient holds for that window, and publishes the committed
/// transaction as ONE edit stamped with its ref plus the window's next transient.
#[expect(clippy::too_many_arguments, reason = "One retained gumball dispatch: verb, motion, payload ids and phase, plus the retained inputs it reads.")]
pub fn gumball_step(
    verb: &str,
    motion: Fem2dGumballMotion,
    ids: &[String],
    phase: Option<&str>,
    reason: Option<&str>,
    snapshot: &Fem2dSnapshot,
    selected: &[String],
    context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Fem2dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<ArtifactCommandWorkStep<EditorApp<Fem2dPlayApp>>, Fault> {
    let phase = GesturePhase::parse(phase, reason).ok_or_else(|| Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("fem.gumball.phase-unknown"), "unknown gumball phase or abort reason"))?;
    let tick = fem2d_gumball_tick(snapshot, if ids.is_empty() { selected } else { ids }, motion);
    let window = context.and_then(|context| context.view_state.as_deref()).and_then(|view| view.window_id.clone()).unwrap_or_default();
    let transient = context.map(|context| context.transient.as_ref().clone()).unwrap_or_default();
    let base_revision: String = operation.canonical_base_revision.iter().map(|byte| format!("{byte:02x}")).collect();
    let drive = fem_gumball_drive::<Fem2dGumballTool>(&transient, &window, verb, phase, tick, &operation.authoring_seed, &base_revision)?;
    let emit = match drive.committed {
        Some((reference, mutations)) if !operation.authoring_seed.is_empty() => Emit { ui_scope: fem2d_gumball_scope(), ..Emit::commit_transaction(reference, mutations) },
        Some((_, mutations)) => Emit { ui_scope: fem2d_gumball_scope(), ..Emit::mutations(mutations) },
        None => Emit { ui_scope: if drive.transient.is_some() { fem2d_gumball_scope() } else { UiDirtyScope::None }, ..Emit::default() },
    };
    Ok(match drive.transient {
        Some(transient) => ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![FemGumballTransientMutation::Snapshot { transient }], window_transient: Vec::new() } },
        None => ArtifactCommandWorkStep::Complete(emit),
    })
}

/// 🛠️ The unmounted route of a gumball verb (no retained context, so no persisted gesture): a one-shot or a commit at
/// rest is ONE tool transaction; a streamed phase needs the retained route's transient.
fn gumball_once(verb: &str, motion: Fem2dGumballMotion, ids: &[String], phase: Option<&str>, reason: Option<&str>, doc: &ArtifactView<'_, Fem2dSnapshot>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    if !matches!(GesturePhase::parse(phase, reason), Some(GesturePhase::Once | GesturePhase::Commit)) {
        return Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("fem.gumball.transient-context-required"), "a streamed gumball phase needs the retained route's transient"));
    }
    let seed = doc.operation_optional().map(|operation| operation.authoring_seed.clone()).unwrap_or_default();
    let tick = fem2d_gumball_tick(doc.snapshot, ids, motion);
    let drive = fem_gumball_drive::<Fem2dGumballTool>(&Default::default(), "", verb, GesturePhase::Once, tick, &seed, "")?;
    Ok(match drive.committed {
        Some((reference, mutations)) if !seed.is_empty() => Emit { ui_scope: fem2d_gumball_scope(), ..Emit::commit_transaction(reference, mutations) },
        Some((_, mutations)) => Emit { ui_scope: fem2d_gumball_scope(), ..Emit::mutations(mutations) },
        None => Emit::default(),
    })
}

//#region 🔖️TranslateSelection
pub mod translate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "translate-selection")]
    pub struct TranslateSelection {
        pub ids: Vec<String>,
        pub dx: f64,
        pub dy: f64,
        pub dz: f64,
        pub phase: Option<String>,
        pub reason: Option<String>,
    }

    pub fn handle(payload: &TranslateSelection, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
        gumball_once("translateSelection", Fem2dGumballMotion::Translate { dx: payload.dx, dy: payload.dy }, &payload.ids, payload.phase.as_deref(), payload.reason.as_deref(), doc)
    }
}
//#endregion 🔖️TranslateSelection

//#region 🔖️RotateSelection
pub mod rotate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "rotate-selection")]
    pub struct RotateSelection {
        pub ids: Vec<String>,
        pub ax: f64,
        pub ay: f64,
        pub az: f64,
        pub angle: f64,
        pub phase: Option<String>,
        pub reason: Option<String>,
    }

    pub fn handle(payload: &RotateSelection, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
        gumball_once("rotateSelection", Fem2dGumballMotion::Rotate { angle: payload.angle }, &payload.ids, payload.phase.as_deref(), payload.reason.as_deref(), doc)
    }
}
//#endregion 🔖️RotateSelection

//#region 🔖️ScaleSelection
pub mod scale_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "scale-selection")]
    pub struct ScaleSelection {
        pub ids: Vec<String>,
        pub sx: f64,
        pub sy: f64,
        pub sz: f64,
        pub phase: Option<String>,
        pub reason: Option<String>,
    }

    pub fn handle(payload: &ScaleSelection, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
        gumball_once("scaleSelection", Fem2dGumballMotion::Scale { sx: payload.sx, sy: payload.sy }, &payload.ids, payload.phase.as_deref(), payload.reason.as_deref(), doc)
    }
}
//#endregion 🔖️ScaleSelection

//#region 🔖️SetTransformGumballFlag
pub mod set_transform_gumball_flag {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "set-transform-gumball-flag")]
    pub struct SetTransformGumballFlag {
        pub flag: String,
        pub pressed: Option<bool>,
    }

    pub fn handle(_payload: &SetTransformGumballFlag, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
        Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("fem.gumball-flag.window-context-required"), "the gumball flag toggle needs a window context"))
    }

    /// 🎚️ Toggles one handle flag of the addressed MODEL window's gumball: window config, so the choice survives a refresh and a split layout keeps one gumball per pane.
    pub fn handle_window(payload: &SetTransformGumballFlag, cfg: &ConfigView<'_, NoConfig>, view: &ViewModel) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
        let window_id = view.window_id.as_deref().ok_or_else(|| Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("fem.gumball-flag.window-required"), "the gumball flag toggle names no window"))?;
        let kind = view.window_instances.iter().find(|window| window.id == window_id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("fem.gumball-flag.window-stale"), "the gumball flag toggle names a closed window"))?;
        if kind != model_window::WINDOW_KIND_ID {
            return Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("fem.gumball-flag.window-kind"), "the transform gumball lives on the model window"));
        }
        let mut next = model_window::config::current(cfg);
        next.gumball.set_flag(&payload.flag, payload.pressed)?;
        Ok(Emit {
            window_config_mutations: vec![model_window::config::addressed_to(window_id, next)],
            ui_scope: UiDirtyScope::Partial { window_bodies: vec![model_window::BODY_KEY.into()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: true, labels: false },
            ..Default::default()
        })
    }
}
//#endregion 🔖️SetTransformGumballFlag

/// 📣️ The en/de notices of the FEM editors' gumball and canvas refusal codes (design §20.12) — fem 2d and fem 3d declare the same table.
pub fn fem_fault_notices() -> &'static [(&'static str, semio_ui_locale::LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<Vec<(&str, semio_ui_locale::LocalizedLabel)>> = std::sync::LazyLock::new(|| {
        let notice = semio_ui_locale::LocalizedLabel::native;
        vec![
            ("fem.gumball.phase-unknown", notice("The transform step is not known.", "Der Transformationsschritt ist unbekannt.")),
            ("fem.gumball.transient-context-required", notice("Dragging the gumball needs an open FEM window.", "Das Ziehen des Gumballs braucht ein geöffnetes FEM-Fenster.")),
            ("fem.gumball-flag.window-context-required", notice("Gumball handles can only be toggled in an open FEM window.", "Gumball-Griffe lassen sich nur in einem geöffneten FEM-Fenster umschalten.")),
            ("fem.gumball-flag.window-required", notice("Focus a model window to toggle gumball handles.", "Ein Modellfenster fokussieren, um Gumball-Griffe umzuschalten.")),
            ("fem.gumball-flag.window-stale", notice("The window of this gumball is no longer open.", "Das Fenster dieses Gumballs ist nicht mehr geöffnet.")),
            ("fem.gumball-flag.window-kind", notice("The transform gumball lives on the model window.", "Der Transformations-Gumball gehört zum Modellfenster.")),
            ("fem.canvas.window-required", notice("This canvas action needs a focused window.", "Diese Zeichenflächenaktion braucht ein fokussiertes Fenster.")),
            ("fem.canvas.window-kind", notice("This canvas action is not available in this window.", "Diese Zeichenflächenaktion ist in diesem Fenster nicht verfügbar.")),
            ("fem.result-animation.window-context-required", notice("The result animation belongs to a Results window; click one first.", "Die Ergebnisanimation gehört zu einem Ergebnisfenster; klicke zuerst eines an.")),
            ("fem.result-animation-tick.window-context-required", notice("The result animation belongs to a Results window; click one first.", "Die Ergebnisanimation gehört zu einem Ergebnisfenster; klicke zuerst eines an.")),
            ("fem.result-animation-tick.retained-route-required", notice("The result animation tick needs the running editor.", "Der Animationstakt braucht den laufenden Editor.")),
        ]
    });
    &NOTICES
}

/// 📣️ The en/de notices of the per-editor refusals every FEM editor raises under its own `prefix` (`fem2d`, `fem3d`): the `patch*` family for each of `kinds` (`(code kind, English noun, German noun)`), `focusEntity`, `setCamera` and the results window, so a refused action shows readable text in the shell instead of a bare code.
pub fn fem_editor_fault_notices(prefix: &'static str, kinds: &[(&'static str, &'static str, &'static str)]) -> Vec<(&'static str, semio_ui_locale::LocalizedLabel)> {
    let notice = semio_ui_locale::LocalizedLabel::native;
    let code = |tail: String| -> &'static str { Box::leak(format!("{prefix}.{tail}").into_boxed_str()) };
    let mut table = Vec::new();
    for (kind, english, german) in kinds {
        table.push((code(format!("patch.{kind}-missing")), notice(&format!("No {english} with this id exists in the document."), &format!("Im Dokument gibt es kein {german} mit dieser Kennung."))));
        table.push((code(format!("patch.{kind}-field")), notice(&format!("A {english} has no field of this name."), &format!("Dieses Objekt ({german}) hat kein Feld mit diesem Namen."))));
        table.push((code(format!("patch.{kind}-value")), notice(&format!("This value is not valid for that field of the {english}."), &format!("Dieser Wert ist für dieses Feld nicht gültig ({german})."))));
        table.push((code(format!("reference.{kind}-unknown")), notice(&format!("This document has no {english} with the id '{{id}}'. Use the id of an existing one."), &format!("Dieses Dokument hat kein Objekt ({german}) mit der Kennung '{{id}}'. Nutze die Kennung eines vorhandenen."))));
    }
    for (window, english, german) in [("focus-entity", "Focus", "Fokussieren"), ("camera", "The camera", "Die Kamera")] {
        table.push((code(format!("{window}.window-required")), notice(&format!("{english} works in a Model or Results window; click one first."), &format!("{german} wirkt in einem Modell- oder Ergebnisfenster; klicke zuerst eines an."))));
        table.push((code(format!("{window}.window-stale")), notice("That window is closed; pick an open Model or Results window.", "Dieses Fenster ist geschlossen; wähle ein offenes Modell- oder Ergebnisfenster.")));
        table.push((code(format!("{window}.window-kind")), notice("Only a Model or Results window can do this.", "Das kann nur ein Modell- oder Ergebnisfenster.")));
        table.push((code(format!("{window}.window-context-required")), notice("This needs the window it is run from.", "Das braucht das Fenster, aus dem es gestartet wird.")));
    }
    table.push((code("focus-entity.unknown-entity".into()), notice("Nothing in the document has this id, so there is nothing to focus.", "Nichts im Dokument hat diese Kennung, daher gibt es nichts zu fokussieren.")));
    table.push((code("member.degenerate".into()), notice("A member needs two different nodes.", "Ein Stab braucht zwei verschiedene Knoten.")));
    table.push((code("results.window-context-required".into()), notice("Choose the result display from a Results window.", "Wähle die Ergebnisanzeige in einem Ergebnisfenster.")));
    table
}

//#region 🧪️Tests
//#endregion 🧪️Tests
