import { resolve } from "node:path";
import { readModuleTreeSource } from "@semio-tech/framework/compiler/syntax/module-tree";
import { readFileSync } from "node:fs";
const plugin = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block";
const t = readFileSync(resolve(plugin, "🧪️tests/🔬️typescript-package-commands/🟦️.ts"), "utf8");
const block = t.slice(t.indexOf("const ANCHORS = ["), t.indexOf("];", t.indexOf("const ANCHORS = [")));
const anchors = [...block.matchAll(/^\s*("(?:[^"\\]|\\.)*"),?$/gm)].map((m) => JSON.parse(m[1]!));
const fixture = await Bun.file(resolve(plugin, "🧪️tests/🧫️fixtures/🧪️publication-authority/🔣️.json")).json() as any;
for (const app of fixture.apps) {
  const source = readModuleTreeSource(resolve(plugin, app.source));
  const ids = [...source.match(new RegExp(`${app.toolIdsConstant}: &\\[&str\\] = &\\[([^\\]]*)\\]`, "s"))?.[1]?.matchAll(/"([^"]+)"/g) ?? []].map((m: any) => m[1]);
  const contracts = [...source.matchAll(/ArtifactToolPublicationContract \{ tool_id: "([^"]+)", lanes: &\[([^\]]*)\] \}/g)].map((m) => m[1]);
  const cls = [...source.matchAll(/\.action_interactive_job\("([^"]+)", InteractiveJobClassification::Migrated\)/g)].map((m) => m[1]);
  console.log(app.owner, "missing", anchors.filter((a: string) => !source.includes(a)), "laws", Object.entries(app.laws).filter(([, v]) => !v).map(([k]) => k), ids.length, contracts.length, cls.length, app.routes.length, "ui", JSON.stringify(app.ui));
}
const variant = (value: string): string => value.split("-").map((part) => `${part[0]!.toUpperCase()}${part.slice(1)}`).join("");
const exact = (l: string[], r: string[]) => JSON.stringify([...l].sort()) === JSON.stringify([...r].sort()) && new Set(l).size === l.length && new Set(r).size === r.length;
for (const app of fixture.apps) {
  const source = readModuleTreeSource(resolve(plugin, app.source));
  const contracts = [...source.matchAll(/ArtifactToolPublicationContract \{ tool_id: "([^"]+)", lanes: &\[([^\]]*)\] \}/g)].map((m) => `${m[1]}:${[...m[2]!.matchAll(/ArtifactToolPublicationLane::(\w+)/g)].map((l) => l[1]).sort().join("+")}`);
  const want = app.routes.map(({ id, lanes }: any) => `${id}:${[...lanes].map(variant).sort().join("+")}`);
  console.log(app.owner, "contractsExact", exact(contracts, want), contracts.filter((c) => !want.includes(c)).slice(0, 3), want.filter((c: string) => !contracts.includes(c)).slice(0, 3));
  const ids = [...source.match(new RegExp(`${app.toolIdsConstant}: &\\[&str\\] = &\\[([^\\]]*)\\]`, "s"))?.[1]?.matchAll(/"([^"]+)"/g) ?? []].map((m: any) => m[1]);
  const cls = [...source.matchAll(/\.action_interactive_job\("([^"]+)", InteractiveJobClassification::Migrated\)/g)].map((m) => m[1]!);
  const exp = app.routes.map((r: any) => r.id);
  console.log(" ids", exact(ids, exp), "cls", exact(cls, exp), ids.filter((i: string) => !exp.includes(i)), exp.filter((i: string) => !ids.includes(i)));
}
