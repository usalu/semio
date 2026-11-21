import { readFileSync, renameSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
const root = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin" + "/";
const ticket = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR" + "/🗑️generated/";
const candidate = JSON.parse(readFileSync(ticket + "raw-picker-mounted-overlay.json", "utf8")).candidate;
const app = root + "⚛️reactor/🔌️app/🦀️.rs";
const instance = root + "🧵️instance/🔌️plugin-app-impl/🦀️.rs";
const owner = root + "🧵️instance/🔌️plugin-app-impl/🎟️picker-publication/🦀️.rs";
const outbox = root + "⚛️reactor/🏃️runtime/🧰️ops/🎞️raw-media/📤️picker.rs";
const turn = root + "⚛️reactor/🔄️turn/🌀️poll-kernel-turn/🦀️.rs";
function replace(body: string, before: string, after: string) {
  if (body.split(before).length !== 2) throw new Error("Missing unique anchor: " + before);
  return body.replace(before, () => after);
}
function excerpt(path: string, from: string, to: string) {
  const body: string = candidate[path];
  const start = body.indexOf(from), end = body.indexOf(to, start + from.length);
  if (start < 0 || end < 0) throw new Error(path);
  return body.slice(start, end);
}
const edits = new Map<string, [string, string]>();
function edit(path: string, patch: (before: string) => string) {
  const before = readFileSync(path, "utf8");
  edits.set(path, [before, patch(before)]);
}
edit(app, body => replace(body, "    fn status(&self) -> OriginalPickerStatus<'_>;", `    fn status(&self) -> OriginalPickerStatus<'_>;
    fn pending_effect(&mut self) -> Option<&mut Effect>;
    fn discard_effect(&mut self);`));
edit(instance, body => {
  const demand = "    fn close_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {\n";
  const step = "    fn close_step(&mut self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{\n";
  let next = replace(body, demand, demand + '        if let Some(demand) = self.picker_close_demands(body) { return demand; }\n');
  next = replace(next, step, step + '        if let Some(result) = self.close_picker(cx, loan) { return result; }\n');
  const pop = "    fn take_typed_operation_effect(&mut self) -> Option<Effect> {\n        self.tools.typed_effect_outbox.pop()";
  return replace(next, pop, "    fn take_typed_operation_effect(&mut self) -> Option<Effect> {\n        if semio_plugin_reactor_app::OriginalPickerPublication::pending_effect(self).is_some() { return None; }\n        self.tools.typed_effect_outbox.pop()");
});
edit(owner, body => {
  let next = replace(body, "    fn advance(&mut self,", `    fn pending_effect(&mut self) -> Option<&mut Effect> {
        self.tools.typed_effect_outbox.items.front_mut().filter(|effect|
            semio_plugin_decl_media::original_media::issue::pending_picker_effect(effect))
    }

    fn discard_effect(&mut self) {
        if self.pending_effect().is_some() { self.tools.typed_effect_outbox.pop(); }
    }

    fn advance(&mut self,`);
  const method = "    /// 🧾 Posts the pre-funded success";
  return replace(next, method, `    /// 💰 Quotes the picker close once the mounted typed operation has released its loan.
    pub(super) fn picker_close_demands(&self, body: usize) -> Option<Result<RetirementDemand, ValueError>> {
        self.media.original_picker.as_ref().filter(|_| self.tools.tool_operations.is_empty())
            .map(|owner| owner.close_demand(body))
    }

    /// 🧹 Returns the picker input before retiring its publication slot.
    pub(super) fn close_picker(
        &mut self, cx: &mut StepContext<'_>, loan: RetainedCloneGrant,
    ) -> Option<Result<PluginLifecycleStep, semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>> {
        let owner = self.media.original_picker.as_ref().filter(|_| self.tools.tool_operations.is_empty())?;
        if !owner.source.matches(cx) {
            return Some(Ok(PluginLifecycleStep::AwaitingInput { reason: "Original picker awaits its retained receiving source" }));
        }
        let demand = match owner.close_demand(loan.maximum_copy_bytes) {
            Ok(demand) => demand,
            Err(error) => return Some(Err(error.into())),
        };
        let slot = &mut self.media.original_picker;
        Some(semio_plugin_ownership::bounded_ownership::original_lifecycle_step(cx, loan, demand, |same, _| {
            let before = same.retained_progress();
            if slot.as_mut().unwrap().close_input(same)? {
                same.with_original_retirement(|same| PickerPublication::retire(slot, same))?;
            }
            let progress = semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(before, same.retained_progress())?;
            Ok((PluginLifecycleStep::Progress(progress), false))
        }))
    }

` + method);
});
edit(outbox, body => {
  let next = replace(body, "        if active.app.original_picker_effect().is_none() { continue; }", `        let Some(owner) = active.app.original_picker() else { continue; };
        if owner.pending_effect().is_none() { continue; }`);
  next = replace(next, "advance_picker(&mut active.app, slot, key, cx)?;", "advance_picker(owner, slot, key, cx)?;");
  next = replace(next, "fn advance_picker<PA: PluginApp>(\n    app: &mut PA,", "fn advance_picker<PA: PluginApp>(\n    owner: &mut dyn semio_plugin_reactor_app::OriginalPickerPublication,");
  next = replace(next, "        let Some((document_id, revision)) = app.original_media_live_document() else {", "        let status = owner.status();\n        let Some((document_id, revision)) = status.document else {");
  next = replace(next, "live, app.original_media_format_closed(), cx", "live, status.format_closed, cx");
  next = replace(next, "discard_effect(app, cx)?;", "discard_effect(owner, cx)?;");
  next = replace(next, "fn discard_effect<PA: PluginApp>(app: &mut PA, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {", "fn discard_effect(owner: &mut dyn semio_plugin_reactor_app::OriginalPickerPublication, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {");
  return next.replaceAll("app.original_picker_effect()", "owner.pending_effect()").replaceAll("app.discard_original_picker_effect()", "owner.discard_effect()");
});
edit(turn, body => {
  const ingress = "    let command_ingress = drive_command_ingress(runtime, command_page, close_instances, &mut effects, identity, cx).await?;\n";
  let next = replace(body, ingress, excerpt(turn, "    let raw = ", "    advance_original_receiving"));
  const trace = '    trace_turn_phase_retention("ingress");';
  next = replace(next, trace, excerpt(turn, "    let picker_pending", trace) + trace);
  next = replace(next, "typed_operation_contended, pending_effects_runnable, reconcile_work", "typed_operation_contended, pending_effects_runnable: pending_effects_runnable || picker_pending, reconcile_work");
  const result = "    let result = semio_framework::kernel::TurnResult";
  return replace(next, result, excerpt(turn, "    if effects.iter().any(semio_plugin_runtime_ops::ops_raw_media::pending_picker_effect)", result) + result);
});
const digest = (contents: string) => createHash("sha256").update(contents).digest("hex");
const receipt = [...edits].map(([path, [before, after]]) => ({ path, before: digest(before), after: digest(after), bytes: Buffer.byteLength(after) }));
if (process.argv.includes("--apply")) {
  for (const [path, [before]] of edits) {
    if (readFileSync(path, "utf8") !== before) throw new Error("Shared source changed: " + path);
  }
  let index = 0;
  for (const [path, [before, after]] of edits) {
    const output = ticket + "raw-picker-shared-join-" + index++;
    writeFileSync(output, after);
    if (readFileSync(path, "utf8") !== before) throw new Error("Shared source changed: " + path);
    renameSync(output, path);
  }
}
console.log(JSON.stringify(receipt, null, 2));
