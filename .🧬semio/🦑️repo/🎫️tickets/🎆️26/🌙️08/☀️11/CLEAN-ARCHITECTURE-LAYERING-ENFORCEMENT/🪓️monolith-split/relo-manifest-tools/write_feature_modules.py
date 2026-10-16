"""⏯️ Writes the history-edit, tool-run and onboarding facade modules from the product-vocabulary text, verbatim apart from headers."""
import pathlib
R = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules")
pv = (R / "🛂️manifest/🧳️product-vocabulary/🦀️.rs").read_text()
SCAFFOLD = {"// #region 🛂️Manifest", "//#region 🔖️Manifest", "//#endregion 🔖️Manifest", "// #endregion 🛂️Manifest"}

def between(start, end):
    assert pv.count(start) == 1 and pv.count(end) == 1, (start[:50], end[:50])
    i = pv.index(start)
    return pv[i:pv.index(end, i)]

def strip_scaffold(text):
    return "\n".join(line for line in text.split("\n") if line not in SCAFFOLD)

history = between("//#region 🔖️HistoryEdit\n", "//#region 🔖️ToolRun\n").rstrip("\n") + "\n"
history = history.replace('#[cfg(test)]\n#[path = "../🧪️tests/🧪️history-edit-actions/🦀️.rs"]\nmod history_edit_actions_tests;\n', "")
assert "#[cfg(test)]" not in history
history = history.replace("/// ✏️ The thirteen reserved history-edit verbs, part of [`history_action_definitions`]: never in the palette, no chords",
                          "/// ✏️ The thirteen reserved history-edit verbs, joined to every hosting app roster by [`RESERVED`]: never in the palette, no chords")
history = history.replace("/// 🗳️ The framework-injected finalize prompt of a history edit (every non-viewer app declares it)",
                          "/// 🗳️ The finalize prompt of a history edit, joined to every hosting app that may edit by [`RESERVED`]")
history_end = '''
/// 🧩️ The history-edit contribution: the thirteen verbs join the app roster, the finalize dialog joins every app that may
/// edit.
pub const RESERVED: ReservedContribution = ReservedContribution::new("historyEdit").with_app_actions(history_edit_action_definitions).with_editor_dialogs(history_edit_finalize_dialog);

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🔖️HistoryEdit
'''
assert history.endswith("}\n\n//#endregion 🔖️HistoryEdit\n"), repr(history[-60:])
history = history[: -len("\n//#endregion 🔖️HistoryEdit\n")] + history_end
history_file = '''//! ⏪️ The reserved history-edit verbs of the time-travel session and their finalize dialog, contributed to every hosting app
//! through [`RESERVED`]. The session reducer is the `semio-framework-time-travel` crate rooted in the parent folder; this
//! module builds manifest declarations, so the framework facade compiles it as `semio_framework::history_edit`.
use crate::manifest::{ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, ActionRef, ArgPresentation, ArgSchema, DialogChoice, DialogDefinition, ReservedContribution, DIALOG_CHOICE_ARG};
use semio_framework_ui_locale::LocalizedLabel;

''' + history
(R / "⏪️time-travel/🕹️actions/🦀️.rs").write_text(history_file)

tool = between("//#region 🔖️ToolRun\n", "//#endregion 🔖️Manifest\n// #endregion 🛂️Manifest\n\n//#region 🔖️IntroductionAction\n").rstrip("\n") + "\n"
tool = tool.replace('#[cfg(test)]\n#[path = "../🧪️tests/🔬️tool-run-actions/🦀️.rs"]\nmod tool_run_actions_tests;\n', "")
assert "#[cfg(test)]" not in tool
tool = tool.replace("/// 🕹️ The seven framework-reserved tool run actions (§2.5 of the tool run contract), auto-injected into any\n/// `AppDefinition` whose tools or utilities declare `run` — mirrors `interaction_action_definitions`.",
                    "/// 🕹️ The seven reserved tool run actions (§2.5 of the tool run contract), joined to every window kind of any\n/// `AppDefinition` whose tools or utilities declare `run` by [`RESERVED`] — mirrors `interaction_action_definitions`.")
tool_end = '''
/// 🧩️ The tool run contribution: the seven reserved actions join every window kind of an app that declares a run.
pub const RESERVED: ReservedContribution = ReservedContribution::new("toolRun").with_window_actions(tool_run_action_definitions);

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🔖️ToolRun
'''
assert tool.endswith("}\n\n//#endregion 🔖️ToolRun\n"), repr(tool[-60:])
tool = tool[: -len("\n//#endregion 🔖️ToolRun\n")] + tool_end
tool_file = '''//! ⏯️ The framework surface of the tool run feature: the run vocabulary a `ToolDefinition`/`UtilityDefinition` declares
//! (re-exported from the `semio-framework-tool-run` crate rooted in the parent folder, because manifest clients need it to
//! fill `run`) and the reserved run actions contributed through [`RESERVED`]. The facade compiles it as
//! `semio_framework::tool_run`.
use crate::manifest::{ActionArgDef, ActionDefinition, ActionKind, AppDefinition, ArgPresentation, ArgSchema, ReservedContribution};
use semio_framework_ui_locale::LocalizedLabel;

''' + tool
(R / "⏯️tool-run/🕹️actions/🦀️.rs").write_text(tool_file)

onboarding = between("//#region 🔖️IntroductionAction\n", "//#region 🔖️AppRole\n")
onboarding = strip_scaffold(onboarding).rstrip("\n") + "\n"
onboarding = onboarding.replace("/// 🎓️ The framework-owned action id apps dispatch to (re)start an app's introduction —\n/// auto-injected as a shell-intercepted View action into any\n/// `AppDefinition` that declares one (mirrors `SET_ACTIVE_UTILITY_ACTION_ID`).",
                                "/// 🎓️ The reserved action id apps dispatch to (re)start an app's introduction — injected by the app builder as a\n/// shell-intercepted View action into any `AppDefinition` that declares one (mirrors `SET_ACTIVE_UTILITY_ACTION_ID`).")
onboarding = onboarding.replace("/// 🎓️ The framework-injected `startIntroduction` View action:", "/// 🎓️ The reserved `startIntroduction` View action:")
onboarding = onboarding.replace("/// 🎬️ The framework-owned action id apps dispatch to (re)start a tutorial — auto-injected as a\n/// fully shell-intercepted View action into any `AppDefinition` that declares one",
                                "/// 🎬️ The reserved action id apps dispatch to (re)start a tutorial — injected by the app builder as a\n/// fully shell-intercepted View action into any `AppDefinition` that declares one")
onboarding = onboarding.replace("/// 🎬️ The framework-injected `startTutorial` View action:", "/// 🎬️ The reserved `startTutorial` View action:")
onboarding = onboarding.replace("/// ⏺️ The framework-owned action id that opens the tutorial recorder chrome — auto-injected into\n/// EVERY `AppDefinition`",
                                "/// ⏺️ The reserved action id that opens the tutorial recorder chrome — injected by the app builder into\n/// EVERY `AppDefinition`")
onboarding = onboarding.replace("/// ⏺️ The framework-injected `recordTutorial` View action:", "/// ⏺️ The reserved `recordTutorial` View action:")
onboarding_file = '''//! 🎓️ App onboarding: the step-gated first-run introduction, the recorded tutorial timeline with its playback engine, and
//! the reserved actions that start or record them. `AppDefinition.introduction`/`AppDefinition.tutorials` carry these
//! declarations; the framework UI renders the introduction overlay and the shells play and record tutorials. The facade
//! compiles this module as `semio_framework::onboarding`.
use crate::manifest::{ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, ActionRef, ToolRef, UtilityRef};
use crate::DomainSelection;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_value::{DslValue, FromValue, ToValue};
use serde::{Deserialize, Serialize};
use ui_wgpu::wgpu::WindowLayout;

''' + onboarding + '''
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
'''
(R / "🎓️onboarding/🦀️.rs").write_text(onboarding_file)
for f in ["⏪️time-travel/🕹️actions/🦀️.rs", "⏯️tool-run/🕹️actions/🦀️.rs", "🎓️onboarding/🦀️.rs"]:
    t = (R / f).read_text()
    print(f, len(t.splitlines()), t.count("//#region"), t.count("//#endregion"))
