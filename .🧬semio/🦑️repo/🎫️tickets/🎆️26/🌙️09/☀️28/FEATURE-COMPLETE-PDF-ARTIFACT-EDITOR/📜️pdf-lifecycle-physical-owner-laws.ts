/** 🪜 Checks the live callers that carry the funded original effect into closure and handback. */
import { readFileSync, renameSync, writeFileSync } from "node:fs";
const path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🟦️.ts";
const temporary = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR" + "/🗑️generated/pdf-physical-owner-laws.ts";
const before = readFileSync(path, "utf8");
let after = before;
const changes = [
 ["MAINTENANCE_CHILD_ROOT_STAGE => self.child_root_retirement_step(grant).map(PluginLifecycleStep::Progress)",
  "if stage==MAINTENANCE_CHILD_ROOT_STAGE{return semio_plugin_ownership::bounded_ownership::plugin_effect_close(cx,grant,|effect|self.child_root_retirement_step(effect)"],
 ["CloseRung::ChildRootRetirements=>Ok(RetainedCloneStep::Progress(self.ladder_ctx().child_root_retirement_step(grant)?))",
  "CloseRung::ChildRootRetirements=>Ok(RetainedCloneStep::Progress(semio_plugin_ownership::bounded_ownership::plugin_effect_close(cx,grant,|effect|self.ladder_ctx().child_root_retirement_step(effect)"],
 ["self.child_root_retirement_unit(grant, self.close.close_child_root_detached)",
  "self.child_root_retirement_unit(effect, self.close.close_child_root_detached)"],
 ["&mut self.maintenance.maintenance_child_root_cursor, grant, closing)",
  "&mut self.maintenance.maintenance_child_root_cursor, effect, closing)"],
 ["fn maintenance_retirement_demands(&self, body: usize)", "fn maintenance_retirement_demands(&self, body: usize,maximum_work_units:u64)"],
 ['expect(transport.includes("semio_framework_value::advance_retirement_turn(demand,grant,owner)")).toBe(true);',
  'expect(transport).toContain("semio_framework_value::advance_retirement_turn(demand,grant,|grant|");expect(transport).toContain("result:owner(grant)");'],
 ["erased_step(retirement, grant)", "erased_step(cx, retirement, grant)"],
 ["completed_step(retirement.as_mut(), grant)", "completed_step(cx, retirement.as_mut(), grant)"],
 ['expect(body.includes("plugin_turn_admitted(demand, grant)?")).toBe(true);expect(body.includes("admit_retained_clone_close(grant, step,")).toBe(true);',
  'expect(body).toContain("original_lifecycle_effect_step(cx, grant, demand, |_, effect|");expect(body).toContain("let child = effect.grant()");expect(body).toContain("admit_retained_clone_close(child, step,");'],
 ["let grant=state.mounted_policy.close;", "MountedOwnerTurnV1::admit(state.instance_id, identity, MountedOwnerPhaseV1::Close, state.mounted_policy, cx)?"],
 ["drive_envelope_completed_record_returns(grant, true)", "drive_envelope_completed_record_returns(cx, grant, true)"],
 ["drive_envelope_field_decoder_returns(grant, true)", "drive_envelope_field_decoder_returns(cx, grant, true)"],
 ["if stage==13{return self.drive_envelope_completed_record_returns(grant,false)}", "if stage==13{return self.drive_envelope_completed_record_returns(cx,grant,false)}"],
 ["if stage==12{return self.drive_envelope_field_decoder_returns(grant,false)}", "if stage==12{return self.drive_envelope_field_decoder_returns(cx,grant,false)}"],
] as const;
for (const [from, to] of changes) {
    if (after.split(from).length !== 2) throw Error("Lifecycle law changed: " + from);
    after = after.replace(from, () => to);
}
writeFileSync(temporary, after);
if (readFileSync(path, "utf8") !== before) throw Error("Live lifecycle laws changed");
renameSync(temporary, path);
