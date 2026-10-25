import { readFileSync, existsSync, readdirSync } from "node:fs";
import { dirname, join, resolve, relative } from "node:path";
import { parse } from "/Users/ueli/Documents/semio/node_modules/smol-toml/dist/index.js";
const root = "/Users/ueli/Documents/semio";
type Row = { name: string; version: string; first: string; sections: Set<string>; users: Set<string> };
function rustClosure(starts: string[]) {
  const seen = new Set<string>(), third = new Map<string, Row>();
  const wsCache = new Map<string, any>();
  const wsOf = (dir: string, t: any): any => {
    const rel = t.package?.workspace;
    let cur = rel ? resolve(dir, rel) : dir;
    for (let i = 0; i < 14; i += 1) {
      const m = join(cur, "Cargo.toml");
      if (existsSync(m)) { if (!wsCache.has(m)) { try { wsCache.set(m, parse(readFileSync(m, "utf8"))); } catch { wsCache.set(m, {}); } } const w = wsCache.get(m); if (w.workspace) return { dir: cur, ws: w.workspace }; }
      cur = dirname(cur);
    }
    return { dir, ws: {} };
  };
  const queue = [...starts];
  while (queue.length) {
    const manifest = queue.pop()!;
    if (seen.has(manifest) || !existsSync(manifest)) continue;
    seen.add(manifest);
    const t: any = parse(readFileSync(manifest, "utf8")), dir = dirname(manifest), { dir: wsDir, ws } = wsOf(dir, t);
    const tables: [string, any][] = [];
    for (const s of ["dependencies", "build-dependencies"]) { if (t[s]) tables.push([s, t[s]]); for (const tg of Object.values<any>(t.target ?? {})) if (tg[s]) tables.push([s, tg[s]]); }
    for (const [section, table] of tables) for (const [key, raw] of Object.entries<any>(table)) {
      let spec = raw, base = dir;
      if (typeof raw === "object" && raw.workspace === true) { const w = ws.dependencies?.[key]; spec = typeof w === "string" ? { version: w } : { ...(w ?? {}), ...raw }; if (w?.path) base = wsDir; }
      const path = typeof spec === "object" ? spec.path : undefined;
      if (path) { queue.push(join(resolve(base, path), "Cargo.toml")); continue; }
      const name = typeof spec === "object" && spec.package ? spec.package : key;
      const version = typeof spec === "string" ? spec : spec.version ?? "*";
      const row = third.get(name) ?? { name, version, first: relative(root, manifest), sections: new Set(), users: new Set() };
      row.users.add(relative(root, manifest));
      row.sections.add(section + (typeof spec === "object" && spec.optional ? "(optional)" : ""));
      third.set(name, row);
    }
  }
  return { crates: seen.size, third, seen: [...seen] };
}
function jsClosure(starts: string[]) {
  const pkgs = new Map<string, string>();
  const walk = (dir: string, depth = 0): void => { if (depth > 12) return; for (const e of readdirSync(dir, { withFileTypes: true })) { if (!e.isDirectory() || ["node_modules", ".git", ".cache", ".🧬semio", "dist", "bazel-out", "temp", "target"].includes(e.name) || e.name.startsWith("bazel-")) continue; const p = join(dir, e.name); if (existsSync(join(p, "package.json"))) { try { const j = JSON.parse(readFileSync(join(p, "package.json"), "utf8")); if (j.name) pkgs.set(j.name, join(p, "package.json")); } catch {} } walk(p, depth + 1); } };
  for (const top of ["🧰️framework", "🎓️teaching", "🏢️semio-tech", "🌎️hub", "✏️s", "♻️mit-bestand"]) walk(join(root, top));
  const seen = new Set<string>(), third = new Map<string, Row>(), queue = [...starts];
  while (queue.length) {
    const manifest = queue.pop()!;
    if (seen.has(manifest)) continue; seen.add(manifest);
    const j = JSON.parse(readFileSync(manifest, "utf8"));
    for (const section of ["dependencies", "optionalDependencies", "peerDependencies"]) for (const [name, version] of Object.entries<string>(j[section] ?? {})) {
      if (pkgs.has(name)) { queue.push(pkgs.get(name)!); continue; }
      const row = third.get(name) ?? { name, version, first: relative(root, manifest), sections: new Set(), users: new Set() };
      row.users.add(relative(root, manifest)); row.sections.add(section); third.set(name, row);
    }
  }
  return { packages: seen.size, third };
}

const R = (p: string) => join(root, p, "Cargo.toml");
const D = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust";
const P = "🧰️framework/🛍️products/📓️print/📦️packages";
const out: any = { dashboard: (rustClosure([R(D)]) as any).seen.map((m: string) => relative(root, m)), print: (rustClosure([R(`${P}/🦀️rust`)]) as any).seen.map((m: string) => relative(root, m)) };
console.log(JSON.stringify(out));
