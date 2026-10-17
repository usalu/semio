// 🔁️ pi2: checks the path-dependency graph of the plugin crates for cycles (normal and dev edges) and for upward edges against crates.json ranks.
import {readFileSync, existsSync} from "node:fs";
import {resolve, join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const crates = JSON.parse(readFileSync(join(import.meta.dir, "crates.json"), "utf8")) as Record<string, any>;
const byName = new Map(Object.entries(crates).map(([k, v]) => [v.name as string, k]));
const edges = new Map<string, {to: string; dev: boolean}[]>();
for (const [k, v] of Object.entries(crates)) {
  const f = join(ROOT, v.pkg, "Cargo.toml");
  if (!existsSync(f)) continue;
  const t = Bun.TOML.parse(readFileSync(f, "utf8")) as any;
  const list: {to: string; dev: boolean}[] = [];
  for (const [table, dev] of [["dependencies", false], ["dev-dependencies", true]] as const) {
    for (const [name, spec] of Object.entries(t[table] ?? {}) as [string, any][]) {
      const real = spec?.package ?? name;
      if (byName.has(real)) list.push({to: byName.get(real)!, dev});
    }
  }
  edges.set(k, list);
}
const state = new Map<string, number>();
const cycles: string[] = [];
const visit = (k: string, path: string[]) => {
  state.set(k, 1);
  for (const {to} of edges.get(k) ?? []) {
    if (state.get(to) === 1) cycles.push([...path, k, to].join(" -> "));
    else if (!state.has(to)) visit(to, [...path, k]);
  }
  state.set(k, 2);
};
for (const k of edges.keys()) if (!state.has(k)) visit(k, []);
const up: string[] = [];
for (const [k, list] of edges) for (const {to, dev} of list) if (crates[to].rank > crates[k].rank && crates[k].kind !== "companion") up.push(`${k} -> ${to}${dev ? " (dev)" : ""}`);
console.log(JSON.stringify({crates: edges.size, edges: [...edges.values()].reduce((s, l) => s + l.length, 0), cycles, upwardEdges: up}, null, 1));
