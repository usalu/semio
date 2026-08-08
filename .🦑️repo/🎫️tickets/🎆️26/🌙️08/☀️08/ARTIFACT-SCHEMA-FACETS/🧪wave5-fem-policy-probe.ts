import { policyArtifactSchemaBreaches, policy } from "/Users/ueli/Documents/semio/📜️script.ts";
const root = "/Users/ueli/Documents/semio";
const asb = policyArtifactSchemaBreaches(root).filter((b) =>
  JSON.stringify(b).toLowerCase().includes("fem"),
);
console.log("fem artifact-schema breaches:", asb.length);
for (const b of asb) {
  console.log("-", (b as any).kind ?? (b as any).id, (b as any).summary ?? (b as any).reason ?? JSON.stringify(b).slice(0, 200));
}
const all = (policy as any)({});
const fem = all.filter((b: any) => JSON.stringify(b).toLowerCase().includes("fem"));
console.log("fem total policy breaches:", fem.length);
const byKind: Record<string, number> = {};
for (const b of fem) {
  const k = b.kind ?? b.id ?? "?";
  byKind[k] = (byKind[k] ?? 0) + 1;
}
console.log("byKind", JSON.stringify(byKind, null, 2));
for (const b of fem.slice(0, 40)) {
  console.log("*", b.kind ?? b.id, (b.summary ?? b.reason ?? "").toString().slice(0, 160));
}
