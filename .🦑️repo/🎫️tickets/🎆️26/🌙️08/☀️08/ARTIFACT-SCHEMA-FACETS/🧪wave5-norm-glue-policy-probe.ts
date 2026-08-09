import { policyArtifactSchemaBreaches, policy } from "/Users/ueli/Documents/semio/📜️script.ts";
const root = "/Users/ueli/Documents/semio";
const scope = (b: any) => JSON.stringify(b).includes("🔌️plugins/📕️norm");
const asb = policyArtifactSchemaBreaches(root).filter(scope);
console.log("norm artifact-schema breaches:", asb.length);
for (const b of asb) {
  console.log("-", (b as any).kind ?? (b as any).id, (b as any).summary ?? (b as any).reason ?? JSON.stringify(b).slice(0, 240));
}
const all = (policy as any)({});
const scoped = all.filter(scope);
console.log("norm total policy breaches:", scoped.length);
const byKind: Record<string, number> = {};
for (const b of scoped) {
  const k = String(b.kind ?? b.id ?? "?");
  byKind[k] = (byKind[k] ?? 0) + 1;
}
console.log("byKind", JSON.stringify(byKind, null, 2));
for (const b of scoped.slice(0, 80)) {
  console.log("*", b.kind ?? b.id, (b.summary ?? b.reason ?? "").toString().slice(0, 200));
}
