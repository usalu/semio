#!/usr/bin/env bun
/** 🎚️ Rewrites `🧰️framework/…/🔌️plugin/🧵️retained-command/🧫️fixtures/🎚️scalar-config-cohort.json`'s
 * per-owner route rows from the live Rust sources through the same exported evidence reader the gate
 * uses (`toolJobOwnerSourceEvidence`), so the language-neutral fixture stops describing routes and
 * dispositions the plugins no longer declare (the OS-wide locale move dropped every `setLocale`
 * route; fem2d finished its retained migration). Batch-only blockers are preserved verbatim. */
import { readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const { toolJobOwnerSourceEvidence } = await import(resolve(repoRoot, "📜️script.ts")) as {
  toolJobOwnerSourceEvidence: (files: ReadonlyMap<string, string>) => {
    rows: { file: string; id: string }[];
    dispositions: { key: string; disposition: string }[];
    appOwned: { ownerFile: string; toolId: string; publicationLanes: string[] }[];
  };
};

type Route = { id: string; disposition: string; lanes: string[]; blocker: string };
type Owner = { id: string; file: string; prefix: string; routes: Route[] };
const fixturePath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧫️fixtures/🎚️scalar-config-cohort.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as { sources: string[]; owners: Owner[] };
const files = new Map(fixture.sources.map((file) => [file, readFileSync(join(repoRoot, file), "utf8")]));
const evidence = toolJobOwnerSourceEvidence(files);

for (const owner of fixture.owners) {
  const sourceIds = evidence.rows.filter((row) => row.file === owner.file).map((row) => row.id);
  const previous = new Map(owner.routes.map((route) => [route.id, route]));
  const dropped = owner.routes.filter((route) => !sourceIds.includes(route.id)).map((route) => route.id);
  const added = sourceIds.filter((id) => !previous.has(id));
  owner.routes = sourceIds.map((id) => {
    const disposition = evidence.dispositions.find((candidate) => candidate.key === `${owner.file}\0${id}`)?.disposition;
    if (!disposition) throw new Error(`${owner.id}/${id} has no explicit disposition in source`);
    if (disposition === "Migrated") {
      const owned = evidence.appOwned.find((candidate) => candidate.ownerFile === owner.file && candidate.toolId === id);
      if (!owned) throw new Error(`${owner.id}/${id} is Migrated without an app-owned publication row`);
      return { id, disposition, lanes: owned.publicationLanes, blocker: "" };
    }
    const blocker = previous.get(id)?.blocker;
    if (!blocker) throw new Error(`${owner.id}/${id} is ${disposition} without a blocker sentence to carry over`);
    return { id, disposition, lanes: [], blocker };
  });
  console.log(`${owner.id}: ${owner.routes.length} routes; dropped [${dropped.join(",")}]; added [${added.join(",")}]`);
}

writeFileSync(fixturePath, `${JSON.stringify(fixture, null, 2)}\n`);
console.log(`rewrote ${fixturePath}`);
