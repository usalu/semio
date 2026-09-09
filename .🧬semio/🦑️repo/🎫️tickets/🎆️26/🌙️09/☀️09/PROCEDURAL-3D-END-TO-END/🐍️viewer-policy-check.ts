/** 👁️ Ticket-local gate runner for the generation3d viewer packet.
 *
 * `bun ./📜️script.ts verify taxonomy report` is blocked repo-wide by an unrelated gitlink inside
 * `.🧬semio/…/26/08/17/END-TO-END-TAXONOMY-NORMALIZATION/📓️rust-join-provenance/🧪️runs/🧪️path-ambiguous-Izy9Ww`
 * ("Normalization requires an explicit repository-boundary decision before authored classification"),
 * which is another ticket's artefact and not this packet's to delete. The taxonomy policies this
 * packet actually needs are pure functions exported from `📜️script.ts`, so this runner calls them
 * directly and filters to the procedural plugin.
 *
 * Usage: `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🐍️viewer-policy-check.ts`
 */
import { policyViewerPurityBreaches, policySubsetSurfaceCompletenessBreaches, policyContributedSurfaceTargetBreaches, policyAppSchemaBreaches, policyEmojiPrefixBreaches } from "../../../../../../../📜️script.ts";

const repoRoot = process.cwd();
const scope = "🌀️procedural";

const suites = [
  { name: "viewer-purity", run: () => policyViewerPurityBreaches(repoRoot) },
  { name: "subset-surface-completeness", run: () => policySubsetSurfaceCompletenessBreaches(repoRoot) },
  { name: "contributed-surface-target", run: () => policyContributedSurfaceTargetBreaches(repoRoot) },
  { name: "app-schema", run: () => policyAppSchemaBreaches(repoRoot) },
  { name: "emoji-prefix", run: () => policyEmojiPrefixBreaches(repoRoot) },
] as const;

let failed = 0;
for (const suite of suites) {
  const all = suite.run();
  const mine = all.filter((breach) => JSON.stringify(breach).includes(scope));
  console.log(`${suite.name}: ${mine.length} procedural breach(es) of ${all.length} repo-wide`);
  for (const breach of mine) {
    failed += 1;
    console.log(`  - [${breach.kind}] ${breach.summary}`);
  }
}
console.log(failed === 0 ? "OK: no procedural-scoped breaches" : `FAIL: ${failed} procedural-scoped breach(es)`);
process.exit(failed === 0 ? 0 : 1);
