// [DEBUG] closer-scratch: inspects policySchemaOverhaulPCBreaches (the 5 PC-seeded rules) directly,
// bypassing runPolicyExit's high-priority-only stdout filter (these rules are all `priority: "low"`).
import { policySchemaOverhaulPCBreaches } from "/Users/ueli/Documents/semio/📜️script.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const all = policySchemaOverhaulPCBreaches(repoRoot);

const myArtifacts = ["🎞️gif", "📷️jpg", "🖼️bmp", "🖼️tiff", "🗜️deflate", "☁️las", "🖊️dwg"];

const tally = new Map<string, number>();
for (const b of all) tally.set(b.kind, (tally.get(b.kind) ?? 0) + 1);
console.log(`TOTAL repo-wide breaches (5 PC rules): ${all.length}`);
for (const [kind, count] of [...tally].sort()) console.log(`  ${count}  ${kind}`);

console.log("\n--- breaches touching FG2's 9 standards (gif/jpg/bmp/tiff/deflate/las/dwg) ---");
const mine = all.filter((b) => myArtifacts.some((a) => b.scope.includes(a)));
for (const b of mine) console.log(`  [${b.kind}] ${b.scope}: ${b.summary}`);
console.log(`\nTOTAL touching FG2 artifacts: ${mine.length}`);
