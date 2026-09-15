/** 🧺️ Evaluates the interactivity audit's TURN-PATCH clauses (📜️script.ts,
 * `interactivityLiveReconcileFailures`'s "TurnResult patch ownership …" block) against the working
 * tree, so this lane's strings are verified rather than eyeballed. Prints every clause that misses.
 * Usage: cd <ticket> && bun 🐍️turn-patch-gate-check.mjs
 * @see 📓️ui-turn-patch-batching-2026-09-15.md §4.3 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
const repo = "/Users/ueli/Documents/semio";
const read = (p) => { try { return readFileSync(join(repo, p), "utf8"); } catch { return ""; } };
const kernel = read("🧰️framework/🔨️modules/🎠️kernel/🦀️.rs");
const schema = kernel.slice(kernel.indexOf("//#region 🔖️TurnResult"), kernel.indexOf("//#endregion 🔖️TurnResult"));
const reactor = [
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎠️runtime/🦀️.rs",
  "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
].map(read).join("\n");
const schemaNeeds = [
  "pub const UI_TURN_PATCHES_MAXIMUM: usize = semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / UI_TURN_PATCH_OWNER_BYTES",
  "pub const UI_TURN_PATCH_BUDGET_BYTES: usize = semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES / 4",
  "pub fn ui_patch_turn_bytes(patch: &UiPatch) -> usize",
  "pub struct UiTurnPatches",
  "entries: [UiTurnPatchEntry; UI_TURN_PATCHES_MAXIMUM]",
  "assert!(size_of::<UiTurnPatches>() <= semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES)",
  "pub fn try_push_ui_patch(&mut self, patch: UiPatch) -> Result<(), UiPatch>",
  "pub const UI_TURN_PATCH_RETIRE_SLOTS: usize = 64",
  "slots: [UiTurnPatchRetireSlot; UI_TURN_PATCH_RETIRE_SLOTS]",
  "retirement: Option<UiTurnPatchRetireKey>",
  "impl Drop for UiTurnPatches",
  "pub fn close_ui_turn_patch_owner_one() -> bool",
  "pub const UI_TURN_PATCH_TRANSPORT_SLOTS: usize = 64",
  "pub struct UiTurnPatchTransportProducer",
  "pub fn drive_one(&mut self, session: u64, cancelled: bool, deadline_expired: bool)",
  "if !self.ready || self.transferred",
  "pub struct UiTurnPatchTransportLease",
  "pub fn try_from_token(token: &[u8], expected_session: u64)",
  "pub fn close_ui_turn_patch_transport_one() -> bool",
  "impl<'de> Deserialize<'de> for UiTurnPatches",
  "struct UiTurnPatchesVisitor",
  "pub ui_patches: UiTurnPatches",
];
const reactorNeeds = [
  "let mut page = semio_framework::kernel::UiTurnPatches::default()",
  "page.try_push_ui_patch(patch)",
  "pub(crate) fn take_turn_patch_page(budget_bytes: usize)",
  "semio_framework::kernel::ui_patch_turn_bytes(&patch)",
  "close_ui_turn_patch_owner_one()",
  "result.ui_patches.iter().map(kernel_ui_patch_to_wit)",
  "Result<semio_framework_actor::TurnResult, semio_framework::Fault>",
  "to_actor_turn_result(mut result: TurnResult",
  "UiTurnPatchTransportProducer::try_new(session, owner)",
  "patch_transport.drive_one(session, false, false)",
  "let ui_patches = patch_transport\n        .take_ready()",
  "semio_framework_async::yield_once().await",
];
let bad = 0;
for (const need of schemaNeeds) if (!schema.includes(need)) { bad += 1; console.log(`SCHEMA MISS: ${need}`); }
for (const need of reactorNeeds) if (!reactor.includes(need)) { bad += 1; console.log(`REACTOR MISS: ${need}`); }
if (schema.includes("pub const UI_TURN_PATCHES_MAXIMUM: usize = 1")) { bad += 1; console.log("SCHEMA STILL CAPS AT ONE"); }
if (schema.includes("pub ui_patches: Vec<UiPatch>")) { bad += 1; console.log("SCHEMA VEC"); }
console.log(`[DEBUG] turn-patch gate clauses missing=${bad}`);
