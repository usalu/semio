// [DEBUG] FG4 closer-scratch: inspects policySchemaOverhaulPCBreaches (the 5 PC-seeded rules)
// directly, bypassing runPolicyExit's high-priority-only stdout filter (these rules are all
// `priority: "low"`). Mirrors FG2's/FG3's own policy_pc_breach_check*.ts, retargeted to FG4's 5
// standards (docx, xlsx, pptx, bcf, ifc/2x3 — note ifc/2x3 shares the artifact scope string "🏗️ifc"
// with ifc/4, so its own breaches are filtered by scope substring further below).
import { policySchemaOverhaulPCBreaches } from "/Users/ueli/Documents/semio/📜️script.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const all = policySchemaOverhaulPCBreaches(repoRoot);

const myArtifacts = ["📜️docx", "📕️xlsx", "🎞️pptx", "💬️bcf", "🏗️ifc"];

const tally = new Map<string, number>();
for (const b of all) tally.set(b.kind, (tally.get(b.kind) ?? 0) + 1);
console.log(`TOTAL repo-wide breaches (5 PC rules): ${all.length}`);
for (const [kind, count] of [...tally].sort()) console.log(`  ${count}  ${kind}`);

// NOTE: `scope` for the grammar/protocol/fixture-honesty/language-registration rules is the
// ARTIFACT-level relPath (no standard tag), so ifc/2x3 vs ifc/4 cannot be disambiguated via `scope`
// alone — use `id`/`summary` instead, which embed the full per-file relPath including the standard
// tag (`🔖️2x3` vs `🔖️4`).
const haystack = (b: (typeof all)[number]) => `${b.id} ${b.summary}`;
console.log("\n--- breaches touching FG4's 5 standards (docx/xlsx/pptx/bcf/ifc-2x3 ONLY, ifc/4 excluded) ---");
const is2x3 = (b: (typeof all)[number]) => haystack(b).includes("🔖️2x3") || haystack(b).includes("standards#2x3");
const mine = all.filter((b) => {
  if (b.scope.includes("🏗️ifc")) return is2x3(b);
  return myArtifacts.some((a) => a !== "🏗️ifc" && b.scope.includes(a));
});
for (const b of mine) console.log(`  [${b.kind}] ${b.scope}: ${b.summary}`);
console.log(`\nTOTAL touching FG4 artifacts (ifc/4 excluded): ${mine.length}`);

console.log("\n--- (diagnostic only) ifc/4 breaches, NOT part of FG4, must stay untouched ---");
const ifc4 = all.filter((b) => b.scope.includes("🏗️ifc") && !is2x3(b));
for (const b of ifc4) console.log(`  [${b.kind}] ${b.scope}: ${b.summary}`);
console.log(`TOTAL ifc/4 (untouched): ${ifc4.length}`);
