import {execFileSync} from "node:child_process";
import {readFileSync, existsSync} from "node:fs";
const root = process.argv[2];
const R = await import(`file://${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs`);
const files = execFileSync("git", ["ls-files", "-z", "--", "*Cargo.toml"], {cwd: root, maxBuffer: 1 << 28}).toString().split("\0").filter(f => f && !f.includes(".🧬semio") && !f.startsWith("compose/") && !f.startsWith("temp/compose/") && existsSync(`${root}/${f}`));
const components = [], playgrounds = [];
for (const f of files) {
  const m = Bun.TOML.parse(readFileSync(`${root}/${f}`, "utf8")).package?.metadata;
  if (!m?.component?.package || !["plugin", "extension"].includes(m.semio?.["component-kind"])) continue;
  const id = m.component.package.slice(6), s = m.semio;
  components.push({pluginId: id, dependsOn: [...(s.extends ? [s.extends] : []), ...(s["depends-on"] ?? [])], consumes: s.consumes ?? [], contributes: s.contributes ?? [], extends: s.extends, host: s.host, role: s.role, manifest: f});
  for (const row of s.playground ?? []) playgrounds.push({...row, pluginId: id});
}
const rowOf = v => playgrounds.find(p => p.variant === v);
const closure = (roots) => R.runtimeComponentClosure(components, roots);
const out = {components: components.length, playgrounds: playgrounds.length};
const catalog = JSON.parse(readFileSync(`${root}/♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🔣️.json`, "utf8"));
const variants = new Set(catalog.panes.flatMap(p => [p.variant, p.runtimeVariant]));
out.variants = [...variants];
out.missingRows = [...variants].filter(v => !rowOf(v));
const unionRoots = playgrounds.filter(p => variants.has(p.variant)).map(p => p.pluginId);
out.unionRoots = [...new Set(unionRoots)];
try { out.union = closure(out.unionRoots); } catch (e) { out.unionError = e.message; }
out.lanes = {};
for (const lane of ["generator", "energy", "fem3d"]) { const r = rowOf(lane); try { out.lanes[lane] = {root: r.pluginId, appScoped: r.app !== undefined, selected: closure([{id: r.pluginId, appScoped: r.app !== undefined}])}; } catch (e) { out.lanes[lane] = {error: e.message}; } }
const covered = new Set(Object.values(out.lanes).flatMap(l => l.selected ?? []));
out.missingFromLanes = (out.union ?? []).filter(i => !covered.has(i));
out.extraInLanes = [...covered].filter(i => !(out.union ?? []).includes(i));
out.hostComponents = components.filter(c => c.host).map(c => c.pluginId);
const ids = new Set(components.map(c => c.pluginId));
out.danglingDeps = components.flatMap(c => c.dependsOn.filter(d => !ids.has(d)).map(d => `${c.pluginId}->${d}`));
const contributed = new Set(components.flatMap(c => c.contributes));
out.consumesWithoutContributor = components.flatMap(c => c.consumes.filter(t => !contributed.has(t)).map(t => `${c.pluginId}:${t}`));
console.log(JSON.stringify(out, null, 1));
