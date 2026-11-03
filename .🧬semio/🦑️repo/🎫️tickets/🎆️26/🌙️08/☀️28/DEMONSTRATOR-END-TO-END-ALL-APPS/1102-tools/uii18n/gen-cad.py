import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from mkpatch import patch
root = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
out = str(pathlib.Path(__file__).parent / "patches")
L = lambda k, en, de: f'        {k}: native_en "{en}", native_de "{de}", reuse_en "{en}", reuse_de "{de}";\n'
labels = "".join([
 L("step_idle", "Idle", "Bereit"), L("step_moved", "Moved", "Verschoben"), L("step_copied", "Copied", "Kopiert"),
 L("step_rotated", "Rotated", "Gedreht"), L("step_scaled", "Scaled", "Skaliert"),
 L("step_nothing", "Nothing to commit", "Nichts zu übernehmen"), L("step_committed", "object(s) committed", "Objekt(e) übernommen"),
 L("step_unsupported", "Unsupported", "Nicht unterstützt"), L("step_unknown", "Unknown", "Unbekannt")])
patch(root + "🗣️terminology/🦀️.rs", [('        step: native_en "Step", native_de "Schritt", reuse_en "Step", reuse_de "Schritt";\n', '        step: native_en "Step", native_de "Schritt", reuse_en "Step", reuse_de "Schritt";\n' + labels)], out + "/cad-step-labels-1.patch")
fn = '''/// 🪧️ The engagement step as the viewer reads it: the kit stores the step as a machine key, so the status row words it through `labels` and shows engine state names as data.
fn engagement_step_text(labels: &CadLabels, step: &str) -> String {
    match step {
        "Idle" => labels.step_idle.as_str().into(),
        "Moved" => labels.step_moved.as_str().into(),
        "Copied" => labels.step_copied.as_str().into(),
        "Rotated" => labels.step_rotated.as_str().into(),
        "Scaled" => labels.step_scaled.as_str().into(),
        "Nothing to commit" => labels.step_nothing.as_str().into(),
        _ => match (step.strip_prefix("Committed ").and_then(|rest| rest.strip_suffix(" object(s)")), step.strip_prefix("Unsupported: "), step.strip_prefix("Unknown: ")) {
            (Some(count), _, _) => format!("{count} {}", labels.step_committed.as_str()),
            (_, Some(action), _) => format!("{}: {action}", labels.step_unsupported.as_str()),
            (_, _, Some(input)) => format!("{}: {input}", labels.step_unknown.as_str()),
            _ => step.into(),
        },
    }
}

'''
patch(root + "🎭️modes/✏️edit/🦀️.rs", [
 ('pub fn cad_window_engagement(envelope: &CadPlayView', fn + 'pub fn cad_window_engagement(envelope: &CadPlayView'),
 ('let step_text = pane_session.map_or_else(|| envelope.runtime.engagement_step.clone(), state_prompt);', 'let step_text = pane_session.map_or_else(|| engagement_step_text(labels, &envelope.runtime.engagement_step), state_prompt);')],
 out + "/cad-step-labels-2.patch")
