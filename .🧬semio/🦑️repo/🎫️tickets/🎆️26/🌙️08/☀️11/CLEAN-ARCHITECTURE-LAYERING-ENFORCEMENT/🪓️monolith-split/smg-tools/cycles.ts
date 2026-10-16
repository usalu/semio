// 🔁️ Static cargo dependency graph over the root and framework workspaces: reports strongly connected components of the normal-dependency graph and dev-dependency edges that close a cycle through normal edges.
import {readFileSync, existsSync} from "node:fs";
import {dirname, join, resolve} from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
type Node = {name: string; dir: string; normal: Set<string>; dev: Set<string>};
const nodes = new Map<string, Node>();
const byDir = new Map<string, Node>();
const seeds: string[] = [];
for (const workspace of ["Cargo.toml", "🧰️framework/Cargo.toml"]) {
  const parsed = Bun.TOML.parse(readFileSync(join(ROOT, workspace), "utf8")) as any;
  const base = dirname(join(ROOT, workspace));
  for (const pattern of parsed.workspace?.members ?? []) {
    if (pattern.includes("*")) for (const file of new Bun.Glob(pattern + "/Cargo.toml").scanSync({cwd: base})) seeds.push(resolve(base, dirname(file)));
    else seeds.push(resolve(base, pattern));
  }
}
const wsDeps = (Bun.TOML.parse(readFileSync(join(ROOT, "Cargo.toml"), "utf8")) as any).workspace?.dependencies ?? {};
const queue = [...new Set(seeds)];
const pending: {node: Node; parsed: any}[] = [];
while (queue.length) {
  const dir = queue.pop()!;
  if (byDir.has(dir)) continue;
  const manifest = join(dir, "Cargo.toml");
  if (!existsSync(manifest)) continue;
  const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  if (!parsed.package) continue;
  const node: Node = {name: parsed.package.name, dir, normal: new Set(), dev: new Set()};
  nodes.set(node.name, node);
  byDir.set(dir, node);
  pending.push({node, parsed});
  const tables = (p: any, key: string) => [p[key], ...Object.values(p.target ?? {}).map((t: any) => t[key])];
  for (const table of [...tables(parsed, "dependencies"), ...tables(parsed, "build-dependencies"), ...tables(parsed, "dev-dependencies")]) {
    for (const [key, raw] of Object.entries(table ?? {}) as [string, any][]) {
      const spec = typeof raw === "string" ? {} : raw;
      const inherited = spec.workspace === true;
      const resolved = inherited ? (wsDeps[spec.package ?? key] ?? {}) : spec;
      if (typeof resolved.path === "string") queue.push(resolve(inherited ? ROOT : dir, resolved.path));
    }
  }
}
for (const {node, parsed} of pending) {
  const tables = (p: any, key: string) => [p[key], ...Object.values(p.target ?? {}).map((t: any) => t[key])];
  const collect = (keys: string[], into: Set<string>) => {
    for (const key of keys) for (const table of tables(parsed, key)) for (const [name, raw] of Object.entries(table ?? {}) as [string, any][]) {
      const spec = typeof raw === "string" ? {} : raw;
      const inherited = spec.workspace === true;
      const resolved = inherited ? (wsDeps[spec.package ?? name] ?? {}) : spec;
      if (typeof resolved.path !== "string") continue;
      const target = byDir.get(resolve(inherited ? ROOT : node.dir, resolved.path));
      if (target && target.name !== node.name) into.add(target.name);
    }
  };
  collect(["dependencies", "build-dependencies"], node.normal);
  collect(["dev-dependencies"], node.dev);
}
let index = 0;
const stack: string[] = [], on = new Set<string>(), idx = new Map<string, number>(), low = new Map<string, number>();
const sccs: string[][] = [];
const strong = (v: string) => {
  idx.set(v, index); low.set(v, index); index++; stack.push(v); on.add(v);
  for (const w of nodes.get(v)!.normal) {
    if (!idx.has(w)) { strong(w); low.set(v, Math.min(low.get(v)!, low.get(w)!)); }
    else if (on.has(w)) low.set(v, Math.min(low.get(v)!, idx.get(w)!));
  }
  if (low.get(v) === idx.get(v)) {
    const comp: string[] = [];
    let w: string;
    do { w = stack.pop()!; on.delete(w); comp.push(w); } while (w !== v);
    if (comp.length > 1) sccs.push(comp);
  }
};
for (const name of nodes.keys()) if (!idx.has(name)) strong(name);
const reach = (from: string, to: string) => {
  const seen = new Set<string>([from]), todo = [from];
  while (todo.length) { const cur = todo.pop()!; if (cur === to) return true; for (const next of nodes.get(cur)!.normal) if (!seen.has(next)) { seen.add(next); todo.push(next); } }
  return false;
};
const devCycles: string[] = [];
for (const node of nodes.values()) for (const dev of node.dev) if (reach(dev, node.name)) devCycles.push(`${node.name} -dev-> ${dev}`);
const focus = process.argv.slice(2);
console.log(JSON.stringify({packages: nodes.size, normalCycles: sccs.length, devCycles: devCycles.length}));
for (const comp of sccs) console.log("NORMAL-CYCLE", comp.join(" | "));
for (const row of devCycles) if (!focus.length || focus.some(f => row.includes(f)) || process.env.ALL_DEV) console.log("DEV-CYCLE", row);
for (const f of focus) {
  const node = nodes.get(f);
  if (node) console.log(f, "normal ->", [...node.normal].join(", "), "| dev ->", [...node.dev].join(", "));
}
