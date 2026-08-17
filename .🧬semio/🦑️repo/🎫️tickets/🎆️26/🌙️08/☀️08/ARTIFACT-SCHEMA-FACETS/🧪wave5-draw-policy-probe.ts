
import { policyArtifactSchemaBreaches } from "../../../../../../📜️script.ts";
const root = new URL("../../../../../../", import.meta.url).pathname;
const breaches = policyArtifactSchemaBreaches(root).filter((b: any) =>
  String(b.id).toLowerCase().includes("draw") ||
  String(b.scope).toLowerCase().includes("draw") ||
  String(b.summary).toLowerCase().includes("draw")
);
console.log(JSON.stringify({ count: breaches.length, breaches }, null, 2));
