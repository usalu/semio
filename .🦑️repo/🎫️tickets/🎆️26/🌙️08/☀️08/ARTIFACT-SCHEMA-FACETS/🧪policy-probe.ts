import { policyArtifactSchemaBreaches, policy } from "/Users/ueli/Documents/semio/📜️script.ts";
const asb = policyArtifactSchemaBreaches("/Users/ueli/Documents/semio").filter((b) =>
  JSON.stringify(b).toLowerCase().includes("lowpoly"),
);
console.log("lowpoly artifact-schema breaches:", asb.length);
const all = (policy as any)({});
const low = all.filter((b: any) => JSON.stringify(b).toLowerCase().includes("lowpoly"));
console.log("lowpoly total policy breaches:", low.length);
