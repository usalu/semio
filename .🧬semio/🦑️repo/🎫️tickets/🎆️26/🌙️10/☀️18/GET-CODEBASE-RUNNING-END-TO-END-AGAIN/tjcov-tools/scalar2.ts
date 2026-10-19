import { join } from "node:path";
import { readFileSync } from "node:fs";
import { toolJobOwnerSourceEvidence, toolJobOwnerSourceText } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
const W = "/Users/ueli/Documents/semio";
const fixture = JSON.parse(readFileSync(join(W, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🧫️fixtures/🎚️scalar-config-cohort.json"), "utf8"));
const files = new Map<string, string>(fixture.sources.map((file: string) => [file, toolJobOwnerSourceText(W, file)]));
const evidence = toolJobOwnerSourceEvidence(files);
for (const o of fixture.owners) {
  const mine = evidence.appOwned.filter((a) => a.ownerFile === o.file);
  console.log(o.id, "appOwned", mine.length, "ready", mine.filter((a) => a.publicationReady).length, "rows", o.routes.length);
  for (const r of o.routes) { const a = mine.find((x) => x.toolId === r.id); if (!a || !a.publicationReady || JSON.stringify(a.publicationLanes) !== JSON.stringify(r.lanes)) console.log("  BAD", r.id, JSON.stringify(a)?.slice(0, 300), JSON.stringify(r.lanes)); }
}
