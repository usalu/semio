/**
 * 🧺️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B — runs the repo's own batched-publication static
 * gate (`toolJobStoreBatchPublicationBounded`) plus the mounted-publisher freshness gate directly
 * against the LIVE `🏪️store`/`🔌️plugin` sources.
 *
 * `bun ./📜️script.ts verify interactivity tool-jobs` currently aborts earlier on an unrelated
 * peer-owned failure (`scalar Config route disposition generation2d/addGeneration`), so this probe
 * exercises the two predicates this wave changed without waiting for that cohort to heal.
 *
 * Run: `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🔍️verify-wave-B-batched-publication.ts`
 */
import { join } from "node:path";
import { readFileSync } from "node:fs";
import { WORKSPACE_ROOT, toolJobStoreBatchPublicationBounded, toolJobEphemeralOneItemPublicationBounded, toolJobPublicationFreshnessBeforeEveryTurn } from "../../../../../../../📜️script.ts";

const store = readFileSync(join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"), "utf8");
const plugin = readFileSync(join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"), "utf8");

const checks: [string, boolean][] = [
  ["batched Store publication is bounded", toolJobStoreBatchPublicationBounded(store, plugin)],
  ["ephemeral one-item publication is bounded", toolJobEphemeralOneItemPublicationBounded(store, plugin)],
  ["document freshness precedes every publication turn", toolJobPublicationFreshnessBeforeEveryTurn(plugin)],
  ["a replay hidden inside batched admission is rejected", !toolJobStoreBatchPublicationBounded(store.replace("let footprint = match source.footprint(lane) {", "replay_mutations(); let footprint = match source.footprint(lane) {"), plugin)],
  ["a whole-lane clone-then-pop publisher is rejected", !toolJobStoreBatchPublicationBounded(store, plugin.replace("std::mem::take(&mut emit.artifact_mutations)", "emit.artifact_mutations.last().cloned()"))],
  ["a per-publication factory reconstruction is rejected", !toolJobStoreBatchPublicationBounded(store, plugin.replace("self.artifact_one_item_factory.as_ref()", "A::build_artifact_store_one_item_preparation_factory()"))],
];

const failed = checks.filter(([, ok]) => !ok);
for (const [name, ok] of checks) console.log(`${ok ? "ok  " : "FAIL"}  ${name}`);
if (failed.length > 0) {
  console.error(`${failed.length} batched-publication gate(s) failed`);
  process.exit(1);
}
