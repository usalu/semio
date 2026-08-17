import { policyArtifactSchemaBreaches, policy } from "/Users/ueli/Documents/semio/📜️script.ts";
const root = "/Users/ueli/Documents/semio";
const asb = policyArtifactSchemaBreaches(root).filter((b) =>
  JSON.stringify(b).toLowerCase().includes("procedural"),
);
console.log("procedural artifact-schema breaches:", asb.length);
for (const b of asb) {
  console.log("-", b.kind, b.summary ?? b.reason);
}
const all = (policy as any)({});
const proc = all.filter((b: any) => JSON.stringify(b).toLowerCase().includes("procedural"));
console.log("procedural total policy breaches:", proc.length);
for (const b of proc.slice(0, 30)) {
  console.log("*", b.kind ?? b.id, b.summary ?? b.reason ?? JSON.stringify(b).slice(0, 120));
}
