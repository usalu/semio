import { readFileSync, renameSync, writeFileSync } from "node:fs";
const root = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin" + "/🧵️instance/";
const temp = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR" + "/🗑️generated/";
const caller = root + "🔌️plugin-app-impl/🎟️picker-publication/🦀️.rs";
const owner = root + "📄️docs/📥️staging/🧱️components/🎟️picker-publication.rs";
const before = readFileSync(caller, "utf8");
const source = readFileSync(owner, "utf8");
const start = before.indexOf("    /// 🧹 Returns");
const end = before.indexOf("    /// 🧾 Posts", start);
if (start < 0 || end < 0) throw new Error("Picker close source changed");
const next = before.slice(0, start) + `    pub(super) fn close_picker(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant)
        -> Option<Result<PluginLifecycleStep, semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>>
    {
        (self.tools.tool_operations.is_empty() && self.media.original_picker.is_some())
            .then(|| PickerPublication::close_mounted(&mut self.media.original_picker, cx, grant))
    }

` + before.slice(end);
const imports = "use semio_plugin_decl_emission::ActionMeta;\n";
const anchor = "    /// 🧾 Releases this carrier";
if (!source.includes(imports) || !source.includes(anchor) || source.includes("close_mounted(")) throw new Error("Picker owner source changed");
const expanded = source.replace(imports, imports + `use semio_plugin_ownership::bounded_ownership::{
    PluginLifecycleFailure, PluginLifecycleStep, original_lifecycle_delta, original_lifecycle_step,
};
`).replace("OriginalRetirementFailure, RetainedCloneProgress,", "OriginalRetirementFailure, RetainedCloneGrant, RetainedCloneProgress,").replace(anchor, `    /// 🧹 Returns the input and releases the slot on the original receiving source.
    pub fn close_mounted(slot: &mut Option<Box<Self>>, cx: &mut StepContext<'_>, grant: RetainedCloneGrant)
        -> Result<PluginLifecycleStep, PluginLifecycleFailure>
    {
        let Some(owner) = slot.as_ref() else {
            return Ok(PluginLifecycleStep::Complete(Default::default()));
        };
        if !owner.source.matches(cx) {
            return Ok(PluginLifecycleStep::AwaitingInput { reason: "Original picker awaits its retained receiving source" });
        }
        let demand = owner.close_demand(grant.maximum_copy_bytes)?;
        original_lifecycle_step(cx, grant, demand, |same, _| {
            let before = same.retained_progress();
            if slot.as_mut().unwrap().close_input(same)? {
                same.with_original_retirement(|same| Self::retire(slot, same))?;
            }
            let progress = original_lifecycle_delta(before, same.retained_progress())?;
            Ok((PluginLifecycleStep::Progress(progress), false))
        })
    }

` + anchor);
for (const [index, [path, old, updated]] of [[owner, source, expanded], [caller, before, next]].entries()) {
    if (readFileSync(path, "utf8") !== old) throw new Error("Shared source changed: " + path);
    const output = temp + "picker-close-owner-" + index;
    writeFileSync(output, updated);
    if (readFileSync(path, "utf8") !== old) throw new Error("Shared source changed: " + path);
    renameSync(output, path);
}
console.log("[DEBUG] Instance app bytes reduced by", Buffer.byteLength(before) - Buffer.byteLength(next));
