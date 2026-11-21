/** 🪜 Allows archive loading to release the mounted owners that block its handoff. */
import { readFileSync, renameSync, writeFileSync } from "node:fs";
const root = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin" + "/";
const generated = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR" + "/🗑️generated/";
const tests = "🧪️tests/🔬️plugin-runtime-runtime-close-budget/";
const files = [
    ["🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs", [
        ["            let archive=matches!(unit,MaintenanceUnit::Stage(24));",
            "            let archive=matches!(unit,MaintenanceUnit::Stage(24));\n            let archive_dependency = matches!(unit, MaintenanceUnit::Stage(8 | 14 | 15));"],
        ["if !archive&&available_items<=DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS", "if !archive && !archive_dependency && available_items <= DOCUMENT_ARCHIVE_STEP_MINIMUM_ITEMS"],
    ]],
    ["🪜️maintenance-ladder/🦀️.rs", [
        ["        if stage==15{return match self.tools.live_runtime_instance_id{Some(instance)=>A::mounted_job_maintenance_step(instance,cx,grant),None=>Ok(idle())}}",
            `        if stage == 15 {
            return match self.tools.live_runtime_instance_id {
                Some(instance_id) if self.ingest.live_document_load().is_some() =>
                    A::mounted_job_close_step(instance_id, cx, grant),
                Some(instance_id) => A::mounted_job_maintenance_step(instance_id, cx, grant),
                None => Ok(idle()),
            };
        }`],
        ["                Some(instance_id) if A::mounted_job_maintenance_phase(instance_id).is_some() => Some(A::mounted_job_maintenance_demands(instance_id, body)?),",
            `                Some(instance_id) if self.ingest.live_document_load().is_some() =>
                    (!A::mounted_jobs_terminal_is_empty(instance_id)).then(|| A::mounted_job_close_demands(instance_id, body)).transpose()?,
                Some(instance_id) if A::mounted_job_maintenance_phase(instance_id) == Some(MountedOwnerPhaseV1::Maintenance) =>
                    Some(A::mounted_job_maintenance_demands(instance_id, body)?),`],
        ["&& self.tools.live_runtime_instance_id.is_none_or(|instance_id| A::mounted_job_maintenance_phase(instance_id).is_none())",
            `&& self.tools.live_runtime_instance_id.is_none_or(|instance_id| {
                if self.ingest.live_document_load().is_some() {
                    A::mounted_jobs_terminal_is_empty(instance_id)
                } else {
                    A::mounted_job_maintenance_phase(instance_id).is_none()
                }
            })`],
    ]],
    [tests + "🟦️.ts", [
        ['expect(live).toContain("self.ingest.document_blocks_pending_effects()");', 'expect(live).toContain("self.ingest.live_document_load().is_some()");'],
        ['expect(mountedQuote).toContain("self.ingest.document_blocks_pending_effects()");', 'expect(mountedQuote).toContain("self.ingest.live_document_load().is_some()");'],
    ]],
    [tests + "🧫️fixtures/🪴️live-owner/🔣️.json", [[`[
  { "resident": false, "phase": null, "live": "ladder", "close": "ladder" },
  { "resident": true, "phase": null, "live": "ladder", "close": "mounted" },
  { "resident": true, "phase": "preparation", "live": "mounted", "close": "mounted" },
  { "resident": true, "phase": "maintenance", "live": "mounted", "close": "mounted" }
]`, `[
  { "resident": false, "phase": null, "documentHandoff": false, "live": "ladder", "close": "ladder", "mountedStep": null },
  { "resident": true, "phase": null, "documentHandoff": false, "live": "ladder", "close": "mounted", "mountedStep": null },
  { "resident": true, "phase": "preparation", "documentHandoff": false, "live": "mounted", "close": "mounted", "mountedStep": null },
  { "resident": true, "phase": "maintenance", "documentHandoff": false, "live": "ladder", "close": "mounted", "mountedStep": "maintain" },
  { "resident": true, "phase": "preparation", "documentHandoff": true, "live": "ladder", "close": "mounted", "mountedStep": "retire" },
  { "resident": true, "phase": "maintenance", "documentHandoff": true, "live": "ladder", "close": "mounted", "mountedStep": "retire" },
  { "resident": false, "phase": null, "documentHandoff": true, "live": "ladder", "close": "ladder", "mountedStep": null }
]`]]],
] as const;
const edits = files.map(([file, changes]) => {
    const path = root + file;
    const before = readFileSync(path, "utf8");
    let after = before;
    for (const [from, to] of changes) {
        if (after.split(from).length !== 2) throw new Error("Shared maintenance path changed: " + file);
        after = after.replace(from, () => to);
    }
    if (file === "🪜️maintenance-ladder/🦀️.rs" && !before.split("\n").some(line => /^use .*MountedOwnerPhaseV1/.test(line))) {
        const anchor = "use semio_plugin_reactor_app::PluginApp;";
        if (!after.includes(anchor)) throw new Error("Missing mounted owner import anchor");
        after = after.replace(anchor, anchor + "\nuse semio_plugin_mounted_owner::MountedOwnerPhaseV1;");
    }
    return {path, before, after};
});
for (const [index, edit] of edits.entries()) {
    const temporary = generated + "pdf-open-maintenance-" + index;
    writeFileSync(temporary, edit.after);
    if (readFileSync(edit.path, "utf8") !== edit.before) throw new Error("Shared maintenance path changed: " + edit.path);
    renameSync(temporary, edit.path);
    console.log("[DEBUG]", edit.path);
}
