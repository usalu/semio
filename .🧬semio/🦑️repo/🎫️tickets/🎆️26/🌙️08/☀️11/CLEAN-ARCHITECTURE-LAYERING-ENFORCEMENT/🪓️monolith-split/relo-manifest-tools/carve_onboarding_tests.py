"""🎓️ Carves the introduction and tutorial tests out of the manifest app-label test into the onboarding test module."""
import pathlib, re
R = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules")
a = R / "🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs"
s = a.read_text()
T = "#[semio_framework_async_macros::async_test]\n"

def cut(start, end):
    global s
    assert s.count(start) == 1 and s.count(end) == 1, (start[:60], end[:60])
    i = s.index(start)
    j = s.index(end, i)
    block = s[i:j]
    s = s[:i] + s[j:]
    return block

intro_a = cut(T + "async fn introduction_step_serde_defaults() {", T + "async fn element_id_authoring_helpers() {")
rest_end = "//#endregion 🔖️TutorialTests\n"
i = s.index(T + "async fn introduction_interaction_kind_round_trips_tagged() {")
j = s.index(rest_end, i) + len(rest_end)
block_b = s[i:j]
k = j
while s[k:k + 1] == "\n":
    k += 1
s = s[:i] + s[k:]
tut_at = block_b.index("//#region 🔖️TutorialTests\n")
intro_b = block_b[:tut_at]
tutorial = block_b[tut_at:]
moved = set(re.findall(r"\b(?:Introduction|Tutorial)[A-Za-z]+\b", intro_a + block_b))
moved |= {"RECORD_TUTORIAL_ACTION_ID", "START_TUTORIAL_ACTION_ID", "compose_tutorial_ui", "interpolate_tutorial_camera", "record_tutorial_action_definition", "start_tutorial_action_definition", "tutorial_camera_at", "tutorial_slice", "validate_tutorial"}
imp_start = s.index("use crate::ui::{\n")
imp_end = s.index("};\n", imp_start) + 3
imp = s[imp_start:imp_end]
lines = imp.split("\n")
kept = [line for line in lines if line.strip().rstrip(",") not in moved]
s = s[:imp_start] + "\n".join(kept) + s[imp_end:]
remaining = sorted(set(re.findall(r"\b(?:Introduction[A-Z]|Tutorial[A-Z])[A-Za-z]*\b|\b(?:RECORD|START)_TUTORIAL_ACTION_ID\b|\b(?:compose_tutorial_ui|interpolate_tutorial_camera|record_tutorial_action_definition|start_tutorial_action_definition|tutorial_camera_at|tutorial_slice|validate_tutorial)\b", s)))
a.write_text(s)
body = intro_a + intro_b
body = body.replace('include_str!("../🧫️fixtures/🎞️tutorial-document-track.json")', 'include_str!("🧫️fixtures/🎞️tutorial-document-track.json")')
tutorial = tutorial.replace('include_str!("../🧫️fixtures/🎞️tutorial-document-track.json")', 'include_str!("🧫️fixtures/🎞️tutorial-document-track.json")')
used = set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", body + tutorial))
manifest_names = ["ActionArgControl", "window_element_id", "panel_tab_element_id", "Locale", "Terminology"]
needed = [n for n in manifest_names if n in used]
header = '''//! 🎓️ The onboarding declarations and their playback engine: introduction steps, interactions, points, gestures and
//! demonstrations round-trip their tagged camelCase wire form; tutorials validate, interpolate cameras, compose UI state,
//! slice the timeline (against the language-agnostic `🧫️fixtures/🎞️tutorial-document-track.json`) and declare their
//! reserved start and record actions.

use super::*;
'''
extra = []
mn = [n for n in needed if n not in ("Locale", "Terminology")]
if mn:
    extra.append("use crate::manifest::{" + ", ".join(sorted(mn)) + "};")
ui = [n for n in needed if n in ("Locale", "Terminology")]
if ui:
    extra.append("use semio_framework_ui_locale::{" + ", ".join(sorted(ui)) + "};")
out = header + ("\n".join(extra) + "\n" if extra else "") + "\n//#region 🔖️IntroductionTests\n" + body.rstrip("\n") + "\n//#endregion 🔖️IntroductionTests\n\n" + tutorial
(R / "🎓️onboarding/🧪️tests/🦀️.rs").write_text(out)
print("remaining in app-label:", remaining)
print("needed:", needed)
print(len(out.splitlines()))
