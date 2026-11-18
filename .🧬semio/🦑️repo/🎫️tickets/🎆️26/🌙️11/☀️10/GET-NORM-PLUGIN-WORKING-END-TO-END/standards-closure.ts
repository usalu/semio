#!/usr/bin/env repo
import { resolve, join, dirname, relative } from "node:path";
import { readFileSync, existsSync, readdirSync, statSync } from "node:fs";
const root = resolve(import.meta.dir, "../../../../../../..");
const lib = resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const { runtimeComponentClosure } = await import(resolve(lib, "🕸️dependencies/🧩️runtime/🟨️.mjs"));
const { declaredBrowserSessionEnginesV1 } = await import(resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚙️engine/🧭️selection/🟨️.mjs"));
const { frameworkOsEngineSourceOwnersV1 } = await import(resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/🟦️.ts"));
const variant = process.argv[2] ?? "din4108";
const manifests: string[] = [];
const out = Bun.spawnSync(["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "*Cargo.toml"], { cwd: root }).stdout.toString().split("\0").filter(Boolean);
for (const p of out) if (!p.includes(".🧬semio") && existsSync(join(root, p))) manifests.push(p);
const components: any[] = [], playgrounds: any[] = [];
for (const p of manifests) {
  const m = Bun.TOML.parse(readFileSync(join(root, p), "utf8")) as any, md = m.package?.metadata;
  if (!md?.component?.package || !["plugin", "extension"].includes(md.semio?.["component-kind"])) continue;
  const id = md.component.package.slice(6), crate = dirname(p);
  components.push({ ...md.semio, pluginId: id, crate, manifest: p, dependsOn: [...(md.semio.extends ? [md.semio.extends] : []), ...(md.semio["depends-on"] ?? [])] });
  for (const row of md.semio.playground ?? []) playgrounds.push({ ...row, pluginId: id });
}
const pg = playgrounds.find(r => r.variant === variant);
if (!pg) throw Error("no playground " + variant);
const boot = runtimeComponentClosure(components, [{ id: pg.pluginId, appScoped: true }]);
const sel = runtimeComponentClosure(components, [{ id: pg.pluginId, appScoped: pg.app !== undefined }]);
const engines = [...new Set([...(pg.engines ?? []), ...declaredBrowserSessionEnginesV1(root, pg.devContribution)])];
const byId = new Map(components.map(c => [c.pluginId, c]));
console.log(JSON.stringify({ variant, pluginId: pg.pluginId, app: pg.app, boot: boot.map(id => byId.get(id).manifest), selected: sel.length, engines, rendererEngines: frameworkOsEngineSourceOwnersV1("renderer"), pg }, null, 1));
