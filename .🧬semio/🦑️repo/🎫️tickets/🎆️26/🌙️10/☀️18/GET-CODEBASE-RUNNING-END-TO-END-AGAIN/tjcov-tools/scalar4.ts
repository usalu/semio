import { join } from "node:path";
import { readFileSync } from "node:fs";
import { toolJobOwnerSourceEvidence, toolJobOwnerSourceText } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
const W = "/Users/ueli/Documents/semio";
const fixture = JSON.parse(readFileSync(join(W, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🧫️fixtures/🎚️scalar-config-cohort.json"), "utf8"));
const files = new Map<string, string>(fixture.sources.map((file: string) => [file, toolJobOwnerSourceText(W, file)]));
const evidence = toolJobOwnerSourceEvidence(files);
for (const o of fixture.owners) for (const a of evidence.appOwned.filter((x) => x.ownerFile === o.file)) if (!a.publicationReady) console.log(o.id, a.toolId, JSON.stringify(a.publicationLanes), a.factory, o.routes.find((r: any) => r.id === a.toolId) ? "" : "NOT-A-ROW");
for (const o of fixture.owners) { const ids = evidence.appOwned.filter((x) => x.ownerFile === o.file).map((a) => a.toolId); console.log(o.id, "dups", ids.filter((x, i) => ids.indexOf(x) !== i)); }
