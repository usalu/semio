/** 🧾 Connects the receipt fixtures to the current receiving context and physical owner. */
import { readFileSync, renameSync, writeFileSync } from "node:fs";
const file = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🟦️.ts";
const temporary = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR" + "/🗑️generated/pdf-current-receipt-laws.ts";
const before = readFileSync(file, "utf8");
let after = before;
const edit = (from: string, to: string) => {
    if (after.split(from).length !== 2) throw Error("Receipt fixture anchor changed: " + from);
    after = after.replace(from, () => to);
};
edit('source.indexOf("effect_bytes.extend",inlineStart));expect(inline.includes("!typed_output.ownership_receipt_pending()"))',
    'source.indexOf("state.effects.append",inlineStart));expect(inline.includes(\x27!state.typed_output.as_mut().expect("retained actual typed output").ownership_receipt_pending()\x27))');
edit('expect(step.includes("grant: RetainedCloneGrant")).toBe(true);expect(step.includes("original_content_step_owners"))',
    'expect(step.includes("let grant = effect.grant()")).toBe(true);expect(step.includes("&mut effect.nested(child_grant)")).toBe(true);expect(step.includes("original_content_step_owners"))');
const oldWorker = before.split("\n").find(line => line.includes('const frameworkStart=source.indexOf("framework reserved mounted worker checkout'));
if (!oldWorker) throw Error("Missing reserved worker fixture");
edit(oldWorker, ` const framework=readFileSync(new URL("../../👷️job/🏛️reserved/⚙️worker/🦀️.rs",import.meta.url),"utf8");
 expect(framework).toContain("session.take_checked_out_retained_step_receipt()");
 expect(framework).toContain("session.checked_out_outcome()");
 expect(framework).toContain("self.terminal=match &view");
 expect(framework).not.toContain("JobOutcomeSlot");
 expect(framework).toContain("cx.with_retained_loan(grant,|same|session.acknowledge_checked_out_outcome(same))");
 expect(framework).toContain("session.checked_out_outcome_descriptor().is_none()");`);
edit('expect(body.includes("grant: RetainedCloneGrant")).toBe(true);expect(body.includes("Result<PluginLifecycleStep, Fault>")).toBe(true);expect(body.includes("disposer.retirement_demands(store,grant.maximum_copy_bytes)")).toBe(true);expect(body.includes("disposer.close_step(store,grant)"))',
    'expect(body.includes("let grant = effect.grant()")).toBe(true);expect(body.includes("Result<PluginLifecycleStep, Fault>")).toBe(true);expect(body.includes("disposer.retirement_demands(store,grant.maximum_copy_bytes)")).toBe(true);expect(body.includes("disposer.close_step(store,effect)"))');
edit('expect(run).toContain("original_lifecycle_net_after(cx.retained_progress(),reserved_before,cx.reserved_return_progress())");expect(run).toContain("original_lifecycle_delta(original_before,after)")',
    'expect(run).toContain("original_lifecycle_net_after(cx.retained_progress(), reserved_before, cx.reserved_return_progress())");expect(run).toContain("original_lifecycle_delta(original_before, after)")');
writeFileSync(temporary, after);
if (readFileSync(file, "utf8") !== before) throw Error("Shared receipt fixture changed");
renameSync(temporary, file);
