// 🧭️ Read-only: cargo-semantics workspace membership (explicit member beats exclude) for the manifests listed in a file.
import {readFileSync, existsSync, readdirSync, statSync} from "node:fs";
import {dirname, join, relative, resolve} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const listFile = process.argv[2]!;
const manifests = readFileSync(listFile, "utf8").split("\n").filter(Boolean);
const toml = (p: string) => Bun.TOML.parse(readFileSync(join(ROOT, p), "utf8")) as any;
const expand = (rootDir: string, pat: string): string[] => {
  if (!/[*?\[]/.test(pat)) return [pat.replace(/\/$/, "")];
  const out: string[] = [];
  const glob = new Bun.Glob(pat);
  const walk = (dir: string, depth: number) => {
    if (depth > 12) return;
    let ents: string[] = [];
    try { ents = readdirSync(join(ROOT, rootDir, dir)); } catch { return; }
    for (const e of ents) {
      const rel = dir ? `${dir}/${e}` : e;
      let st; try { st = statSync(join(ROOT, rootDir, rel)); } catch { continue; }
      if (!st.isDirectory()) continue;
      if (glob.match(rel)) out.push(rel);
      walk(rel, depth + 1);
    }
  };
  walk("", 0);
  return out;
};
const wsCache = new Map<string, any>();
const wsOf = (dir: string) => {
  if (wsCache.has(dir)) return wsCache.get(dir);
  const m = join(dir, "Cargo.toml");
  let v: any = null;
  if (existsSync(join(ROOT, m))) {
    const t = toml(m);
    if (t.workspace) {
      const members = new Set<string>();
      for (const p of t.workspace.members ?? []) for (const e of expand(dir, p)) members.add(e);
      v = {dir, manifest: m, members, rawMembers: t.workspace.members ?? [], exclude: t.workspace.exclude ?? []};
    }
  }
  wsCache.set(dir, v);
  return v;
};
const rows: string[] = [];
let ok = 0, bad = 0;
for (const m of manifests) {
  const pdir = dirname(m);
  const hits: string[] = [];
  const near: string[] = [];
  let d = dirname(pdir);
  let cur = pdir;
  while (true) {
    const ws = cur === pdir ? null : wsOf(cur);
    if (ws) {
      const rel = relative(cur, pdir);
      const explicit = [...ws.members].some((mem: string) => rel === mem || rel.startsWith(mem + "/"));
      const exactly = ws.members.has(rel);
      const excluded = ws.exclude.some((ex: string) => rel === ex || rel.startsWith(ex.replace(/\/$/, "") + "/"));
      if (exactly) hits.push(`${ws.manifest}`);
      else near.push(`${ws.manifest} explicitPrefix=${explicit} excl=${excluded}`);
      break;
    }
    const up = dirname(cur);
    if (up === cur || up === ".") break;
    cur = up;
  }
  const pkg = toml(m).package?.name ?? "(none)";
  if (hits.length === 1) ok++; else bad++;
  rows.push([m, pkg, hits.length, hits.join(";"), near.join(";")].join("\t"));
}
console.log(`manifests ${manifests.length} exactly-one-member-of-nearest-workspace ${ok} other ${bad}`);
await Bun.write(process.argv[3]!, rows.join("\n") + "\n");
