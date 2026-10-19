import { policyReadFileSafe } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
import { interactivityCheckedRustBlockScope } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit/🖥️mounted/🎞️frame-transaction/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const shell = policyReadFileSafe(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🦀️.rs");
const f: string[] = [];
const d = interactivityCheckedRustBlockScope(shell, "pub fn dispatch_action<'a>(&'a mut self, mut action: ActionDescriptor) -> ShellTurn<'a, Result<(), String>>", "x", f)?.replace(/\s+/g, "") ?? "";
console.log(f, d.length);
for (const n of ["letrecord_tutorial_after_acceptance=action.action==semio_framework::onboarding::RECORD_TUTORIAL_ACTION_ID;","letanswer=program.handle_action(session.instance_id,&action_json,&live_view_state).await;letmutresult=answer.map_err(|refusal|self.prefs.refused_guest_call(refusal,&session.app,&mut self.chrome))?;","ifrecord_tutorial_after_acceptance{self.as_window_actions().tutorial_start_recording();}"]) console.log(d.indexOf(n));
console.log((d.match(/tutorial_start_recording\(\);/g) ?? []).length);
