import { readFileSync, renameSync, writeFileSync } from "node:fs";
const root = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin" + "/";
const temp = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR" + "/🗑️generated/";
const docs = "    /// 🎛️ Advances the format owner on the retained receiving source.\n";
const advance = "    async fn advance_original_picker(\n";
const files = [
    ["⚛️reactor/🔌️app/🦀️.rs", [
        ["    fn advance(&mut self, picker: &mut RawMediaPicker, cx: &mut semio_framework_job::StepContext<'_>)\n        -> Result<(), semio_framework_job::OriginalTurnFailure>;\n", ""],
        ["    fn original_picker(&mut self) -> Option<&mut dyn OriginalPickerPublication> { None }\n",
            "    fn original_picker(&mut self) -> Option<&mut dyn OriginalPickerPublication> { None }\n\n" + docs + advance +
            "        &mut self, _picker: &mut RawMediaPicker, _cx: &mut semio_framework_job::StepContext<'_>,\n" +
            "    ) -> Result<(), semio_framework_job::OriginalTurnFailure> { Ok(()) }\n"],
    ]],
    ["🧵️instance/🔌️plugin-app-impl/🎟️picker-publication/🦀️.rs", [
        ["    fn advance(&mut self, picker: &mut RawMediaPicker, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {\n        semio_framework_async::poll::resolve_ready(self.advance_picker_publication(picker, cx))\n    }\n\n", ""],
        ["    async fn advance_picker_publication(", "    pub(super) async fn advance_picker_publication("],
    ]],
    ["🧵️instance/🔌️plugin-app-impl/🦀️.rs", [
        ["    fn original_picker(&mut self) -> Option<&mut dyn semio_plugin_reactor_app::OriginalPickerPublication> { Some(self) }\n",
            "    fn original_picker(&mut self) -> Option<&mut dyn semio_plugin_reactor_app::OriginalPickerPublication> { Some(self) }\n\n" +
            advance + "        &mut self, picker: &mut semio_plugin_decl_media::original_media::RawMediaPicker, cx: &mut StepContext<'_>,\n" +
            "    ) -> Result<(), OriginalTurnFailure> { self.advance_picker_publication(picker, cx).await }\n"],
    ]],
    ["⚛️reactor/🏃️runtime/🧰️ops/🎞️raw-media/🦀️.rs", [[
        '    let owner = active.app.original_picker().ok_or_else(|| reactor_close_fault("admitted original picker has no mounted publication owner"))?;\n' +
        "    owner.advance(&mut slot.raw_media.picker, cx)?;",
        "    active.app.advance_original_picker(&mut slot.raw_media.picker, cx).await?;",
    ]]],
] as const;
for (const [index, [file, replacements]] of files.entries()) {
    const path = root + file;
    const old = readFileSync(path, "utf8");
    let next = old;
    for (const [from, to] of replacements) {
        if (next.split(from).length !== 2) throw new Error("Picker source changed: " + path);
        next = next.replace(from, () => to);
    }
    const output = temp + "picker-async-" + index;
    writeFileSync(output, next);
    if (readFileSync(path, "utf8") !== old) throw new Error("Shared source changed: " + path);
    renameSync(output, path);
    console.log("[DEBUG]", file, Buffer.byteLength(next) - Buffer.byteLength(old));
}
