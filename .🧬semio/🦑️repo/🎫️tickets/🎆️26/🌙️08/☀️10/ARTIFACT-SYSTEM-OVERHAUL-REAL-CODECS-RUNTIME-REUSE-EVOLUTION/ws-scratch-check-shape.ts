// [DEBUG] WS scratch: inspects policyFacetMirrorDriftBreaches's real signal directly, bypassing
// runPolicyExit's high-priority-only stdout filter (this rule is `priority: "low"`).
// policyFacetMirrorDriftBreaches itself isn't exported, so this goes through
// policySchemaOverhaulS2Breaches (which is exported and includes it) and filters by
// kind === "stdio-artifacts/facet-mirror-drift". Reusable for future waves — keep this file.
import { policySchemaOverhaulS2Breaches } from "/Users/ueli/Documents/semio/📜️script.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const all = policySchemaOverhaulS2Breaches(repoRoot).filter((b) => b.kind === "stdio-artifacts/facet-mirror-drift");

console.log(`TOTAL facet-mirror-drift breaches: ${all.length}`);

// Split into "extra" (new reverse-direction signal) vs "missing-only" (old forward-direction signal)
// vs "MISSING_FILE" vs "PARSE_ERROR" so we can see exactly what the new harvesters contributed.
const extraOnly = all.filter((b) => b.summary.includes(":extra:"));
const parseErrors = all.filter((b) => b.summary.includes(":PARSE_ERROR"));
console.log(`  of which mention an :extra: field breach: ${extraOnly.length}`);
console.log(`  of which mention a :PARSE_ERROR: ${parseErrors.length}`);

// NOTE: scope/summary use the repo's emoji-prefixed path segments (e.g. "🔣️json", "📷️png"), not
// the ASCII standard slug — match on the emoji artifact folder name, not "stdio/json/" etc.
const KNOWN_REAL_SPOT_CHECK = ["🔣️json", "📷️png", "🗜️zip", "🎞️gif", "🖼️bmp"];
console.log("\n--- spot-check pre-existing-standard mirrors (json/png/zip/gif/bmp, all in the allowlist) ---");
for (const artifact of KNOWN_REAL_SPOT_CHECK) {
  const hits = all.filter((b) => b.scope.includes(artifact));
  console.log(`  ${artifact}: ${hits.length} breach(es)`);
  for (const b of hits) console.log(`    [${b.id}] ${b.summary}`);
}

console.log("\n--- sample of scaffolded artifacts now correctly showing :extra: (first 15) ---");
for (const b of extraOnly.slice(0, 15)) console.log(`  [${b.id}] ${b.summary}`);

console.log("\n--- full breach dump ---");
for (const b of all) console.log(`  [${b.id}] scope=${b.scope} :: ${b.summary}`);

// Reconstruct what the OLD (forward-only) rule would have flagged, by parsing the
// "(sib:token, sib:token, ...)" list embedded in each summary and keeping only tokens the old
// rule could ever produce (sib:MISSING_FILE or sib:<digits>) — never sib:extra:<n> or
// sib:PARSE_ERROR, both of which are new signal from this wave's reverse-direction check.
const stale = all.filter((b) => b.id.startsWith("facet-mirror-drift-stale-"));
const real = all.filter((b) => !b.id.startsWith("facet-mirror-drift-stale-"));
let oldWouldBreach = 0;
for (const b of real) {
  const inner = b.summary.match(/\(([^)]*)\)$/)?.[1] ?? "";
  const tokens = inner.split(", ").filter(Boolean);
  const oldTokens = tokens.filter((t) => /^[^:]+:(MISSING_FILE|\d+)$/.test(t));
  if (oldTokens.length > 0) oldWouldBreach++;
}
console.log(`\n--- before/after reconstruction ---`);
console.log(`  "stale allowlist" breaches (unaffected by this change): ${stale.length}`);
console.log(`  BEFORE (forward-only, reconstructed from this run's own data): ${oldWouldBreach} real breaches`);
console.log(`  AFTER  (forward + reverse, this run):                          ${real.length} real breaches`);
console.log(`  NEW breaches surfaced purely by the reverse/extra-field check: ${real.length - oldWouldBreach}`);
