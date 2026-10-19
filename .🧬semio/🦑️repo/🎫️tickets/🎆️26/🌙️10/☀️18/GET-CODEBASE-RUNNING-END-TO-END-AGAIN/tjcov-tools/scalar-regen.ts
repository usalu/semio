import { join } from "node:path";
import { readFileSync, writeFileSync } from "node:fs";
import { toolJobOwnerSourceEvidence, toolJobOwnerSourceText } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
const W = "/Users/ueli/Documents/semio";
const path = join(W, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🧫️fixtures/🎚️scalar-config-cohort.json");
const raw = readFileSync(path, "utf8");
const fixture = JSON.parse(raw);
const files = new Map<string, string>(fixture.sources.map((file: string) => [file, toolJobOwnerSourceText(W, file)]));
const evidence = toolJobOwnerSourceEvidence(files);
if (evidence.failures.length) throw new Error("evidence failures " + evidence.failures.length);
for (const owner of fixture.owners) {
  const before = new Map(owner.routes.map((r: any) => [r.id, r]));
  owner.routes = evidence.rows.filter((row) => row.file === owner.file).map((row) => row.id).sort().map((id) => {
    const disposition = evidence.dispositions.find((d) => d.key === `${owner.file}\0${id}`)?.disposition;
    const owned = evidence.appOwned.find((o) => o.ownerFile === owner.file && o.toolId === id);
    if (disposition === "Migrated") return { id, disposition, lanes: owned?.publicationLanes ?? [], blocker: "" };
    const old = before.get(id) as any;
    if (!old?.blocker) throw new Error(`non-migrated route without blocker ${owner.id}/${id}`);
    return { id, disposition, lanes: [], blocker: old.blocker };
  });
}
writeFileSync(path, JSON.stringify(fixture, null, 2) + (raw.endsWith("\n") ? "\n" : ""));
