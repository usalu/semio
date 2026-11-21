/** 📨 Reconnects private picker issuance to the mounted retained effect outbox. */
import { readFileSync, renameSync, writeFileSync } from "node:fs";
const root = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin" + "/";
const generated = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR" + "/🗑️generated/";
const outbox = root + "⚛️reactor/🏃️runtime/🧰️ops/🎞️raw-media/📤️picker.rs";
const turn = root + "⚛️reactor/🔄️turn/🌀️poll-kernel-turn/🦀️.rs";
const paths = [outbox, turn];
const live = paths.map(path => readFileSync(path, "utf8"));
const stage = JSON.parse(readFileSync(generated + "raw-picker-mounted-overlay.json", "utf8")).candidate;
if (!live[0].includes("runtime: &PluginRuntime<PA>, instance: u32, effect: &mut Effect") ||
    live[1].includes("let picker_pending")) throw new Error("Outbox source changed");
const status = "        let Some((document_id, revision)) = app.original_media_live_document() else {";
let picker = stage[outbox];
if (typeof picker !== "string" || picker.split(status).length !== 2) throw new Error("Missing retained outbox design");
picker = picker
    .replace("        if active.app.original_picker_effect().is_none() { continue; }", `        let Some(owner) = active.app.original_picker() else { continue; };
        if owner.pending_effect().is_none() { continue; }`)
    .replace("advance_picker(&mut active.app, slot, key, cx)?;", "advance_picker(owner, slot, key, cx)?;")
    .replace("fn advance_picker<PA: PluginApp>(\n    app: &mut PA,", "fn advance_picker<PA: PluginApp>(\n    owner: &mut dyn semio_plugin_reactor_app::OriginalPickerPublication,")
    .replace(status, "        let status = owner.status();\n        let Some((document_id, revision)) = status.document else {")
    .replace("live, app.original_media_format_closed(), cx", "live, status.format_closed, cx")
    .replace("discard_effect(app, cx)?;", "discard_effect(owner, cx)?;")
    .replace("fn discard_effect<PA: PluginApp>(app: &mut PA,", "fn discard_effect(owner: &mut dyn semio_plugin_reactor_app::OriginalPickerPublication,")
    .replaceAll("app.original_picker_effect()", "owner.pending_effect()")
    .replaceAll("app.discard_original_picker_effect()", "owner.discard_effect()");
const edit = (body: string, from: string, to: string) => {
    if (body.split(from).length !== 2) throw new Error("Outbox turn anchor changed: " + from);
    return body.replace(from, () => to);
};
const ingress = '    trace_turn_phase_retention("ingress");';
const stagedTurn = stage[turn];
const workStart = stagedTurn.indexOf("    let picker_pending");
const workEnd = stagedTurn.indexOf(ingress, workStart);
if (workStart < 0 || workEnd < 0) throw new Error("Missing retained outbox turn");
const composed = edit(edit(live[1], ingress, stagedTurn.slice(workStart, workEnd) + ingress),
    "pending_effects_runnable: pending_effects_runnable || picker_work",
    "pending_effects_runnable: pending_effects_runnable || picker_pending");
for (const [i, output] of [picker, composed].entries()) {
    const path = paths[i];
    const temporary = generated + "picker-outbox-" + i;
    writeFileSync(temporary, output);
    if (readFileSync(path, "utf8") !== live[i]) throw new Error("Shared source changed: " + path);
    renameSync(temporary, path);
    console.log("[DEBUG]", path, Buffer.byteLength(output) - Buffer.byteLength(live[i]));
}
