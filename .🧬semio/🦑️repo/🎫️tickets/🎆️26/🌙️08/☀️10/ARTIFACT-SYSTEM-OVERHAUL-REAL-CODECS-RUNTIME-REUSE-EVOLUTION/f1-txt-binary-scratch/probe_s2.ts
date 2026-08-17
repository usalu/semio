import { policySchemaOverhaulS2Breaches } from "/Users/ueli/Documents/semio/📜️script.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const all = policySchemaOverhaulS2Breaches(repoRoot);
const mine = all.filter((b: any) => {
  const scope = String(b.scope ?? "");
  return scope.includes("🗿️artifacts/📄txt/") || scope.includes("🗿️artifacts/💾️binary/");
});
console.log(`total S2 breaches: ${all.length}`);
console.log(`txt/binary-scoped: ${mine.length}`);
for (const b of mine) {
  console.log(`${b.kind}\t${b.priority}\t${b.scope}\t${b.summary}`);
}
console.log("--- first 10 unfiltered (to see scope shape) ---");
for (const b of all.slice(0, 10)) {
  console.log(`${b.kind}\t${b.priority}\t${b.scope}`);
}
console.log("--- kinds breakdown ---");
const byKind: Record<string, number> = {};
for (const b of all) byKind[b.kind] = (byKind[b.kind] ?? 0) + 1;
console.log(byKind);
