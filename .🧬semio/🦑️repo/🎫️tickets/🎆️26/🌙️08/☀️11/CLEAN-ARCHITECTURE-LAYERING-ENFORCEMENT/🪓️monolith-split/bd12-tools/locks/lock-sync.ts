import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
const root = "/Users/ueli/Documents/semio";
const [S, mode] = [process.argv[2], process.argv[3] ?? "check"];
const FAMILY = ["semio-framework-2d", "semio-framework-2d-compute"];
const tomls = readFileSync(S + "/tomls.txt", "utf8").split("\n").filter(Boolean);
const locks = [...readFileSync(S + "/locks.txt", "utf8").split("\n"), ...readFileSync(S + "/locks-untracked.txt", "utf8").split("\n").filter((l) => l !== "DONE")].filter(Boolean).filter((p) => !p.startsWith(".🧬semio") && existsSync(join(root, p)));
type M = { path: string; name: string; doc: any; dir: string };
const byName = new Map<string, M[]>();
const byDir = new Map<string, M>();
const wsRoots = new Map<string, any>();
for (const rel of tomls) {
  let doc: any;
  try { doc = Bun.TOML.parse(readFileSync(join(root, rel), "utf8")); } catch { continue; }
  const dir = dirname(join(root, rel));
  if (doc.workspace) wsRoots.set(dir, doc);
  if (doc.package?.name) { const m = { path: rel, name: doc.package.name, doc, dir }; byDir.set(dir, m); if (!byName.has(m.name)) byName.set(m.name, []); byName.get(m.name)!.push(m); }
}
function wsOf(m: M): any {
  const w = m.doc.package?.workspace;
  if (typeof w === "string") return wsRoots.get(resolve(m.dir, w));
  let d = m.dir;
  while (d.length >= root.length) { if (wsRoots.has(d)) return wsRoots.get(d); d = dirname(d); }
}
function depNames(m: M, includeDev: boolean): Set<string> {
  const out = new Set<string>();
  const ws = wsOf(m);
  const kinds = ["dependencies", "build-dependencies", ...(includeDev ? ["dev-dependencies"] : [])];
  const tables: any[] = [];
  for (const k of kinds) if (m.doc[k]) tables.push(m.doc[k]);
  for (const t of Object.values<any>(m.doc.target ?? {})) for (const k of kinds) if (t[k]) tables.push(t[k]);
  for (const t of tables) for (const [k, v] of Object.entries<any>(t)) {
    let name = k;
    if (v && typeof v === "object") {
      if (v.package) name = v.package;
      else if (v.workspace === true) { const wd = ws?.workspace?.dependencies?.[k]; if (wd && typeof wd === "object" && wd.package) name = wd.package; }
    }
    out.add(name);
  }
  return out;
}
const bytes = (a: string, b: string) => Buffer.compare(Buffer.from(a), Buffer.from(b));
type B = { raw: string; name: string; version: string; source?: string; deps: string[]; orig: string };
function parseBlock(raw: string): B {
  const name = /^name = "([^"]+)"/m.exec(raw)![1];
  const version = /^version = "([^"]+)"/m.exec(raw)![1];
  const source = /^source = "([^"]+)"/m.exec(raw)?.[1];
  const dm = /^dependencies = \[\n([\s\S]*?)^\]\n/m.exec(raw);
  const deps = dm ? [...dm[1].matchAll(/^ "([^"]+)",$/gm)].map((x) => x[1]) : [];
  return { raw, name, version, source, deps, orig: deps.join("\n") };
}
function render(b: B): string {
  if (b.deps.join("\n") === b.orig) return b.raw;
  const head = b.raw.replace(/^dependencies = \[\n[\s\S]*?^\]\n/m, "");
  if (!b.deps.length) return head;
  const lines = b.raw.split("\n");
  const tail = head.includes("\nchecksum = ") || head.includes("\nsource = ") ? null : null;
  void lines; void tail;
  const depText = "dependencies = [\n" + b.deps.map((d) => ` "${d}",\n`).join("") + "]\n";
  const m = /^(name = .*\n(?:version = .*\n)?(?:source = .*\n)?(?:checksum = .*\n)?)([\s\S]*)$/.exec(head)!;
  return m[1] + depText + m[2];
}

function headText(rel: string): string | undefined {
  const r = Bun.spawnSync(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "show", "HEAD:" + rel], { cwd: root, stdout: "pipe", stderr: "pipe" });
  return r.exitCode === 0 ? r.stdout.toString() : undefined;
}
type Kinds = { normal: Set<string>; dev: Set<string>; optional: Set<string> };
function familyKinds(doc: any, m: M | undefined): Kinds {
  const out: Kinds = { normal: new Set(), dev: new Set(), optional: new Set() };
  const ws = m ? wsOf(m) : undefined;
  const scan = (t: any, kind: "normal" | "dev") => {
    for (const [k, v] of Object.entries<any>(t ?? {})) {
      let name = k;
      if (v && typeof v === "object") {
        if (v.package) name = v.package;
        else if (v.workspace === true) { const wd = ws?.workspace?.dependencies?.[k]; if (wd && typeof wd === "object" && wd.package) name = wd.package; }
      }
      if (!FAMILY.includes(name)) continue;
      if (v && typeof v === "object" && v.optional === true) out.optional.add(name);
      else out[kind].add(name);
    }
  };
  for (const k of ["dependencies", "build-dependencies"]) { scan(doc[k], "normal"); for (const t of Object.values<any>(doc.target ?? {})) scan(t[k], "normal"); }
  scan(doc["dev-dependencies"], "dev"); for (const t of Object.values<any>(doc.target ?? {})) scan(t["dev-dependencies"], "dev");
  return out;
}
const deltas = new Map<string, { removed: Set<string>; addedNormal: Set<string>; addedDev: Set<string>; addedOptional: Set<string> }>();
for (const [name, ms] of byName) for (const m of ms) {
  const old = headText(m.path);
  let oldDoc: any = {}; try { if (old) oldDoc = Bun.TOML.parse(old); } catch {}
  const o = familyKinds(oldDoc, m), n = familyKinds(m.doc, m);
  const oAll = new Set([...o.normal, ...o.dev, ...o.optional]), nAll = new Set([...n.normal, ...n.dev, ...n.optional]);
  const removed = new Set([...oAll].filter((x) => !nAll.has(x)));
  const addedNormal = new Set([...n.normal].filter((x) => !oAll.has(x) || !o.normal.has(x)));
  const addedDev = new Set([...n.dev].filter((x) => !oAll.has(x)));
  const addedOptional = new Set([...n.optional].filter((x) => !oAll.has(x)));
  if (removed.size || addedNormal.size || addedDev.size || addedOptional.size) {
    const key = name;
    const prev = deltas.get(key);
    if (prev && ms.length > 1) { for (const x of removed) prev.removed.add(x); for (const x of addedNormal) prev.addedNormal.add(x); } else deltas.set(key, { removed, addedNormal, addedDev, addedOptional });
  }
}
await Bun.write(S + "/lock-deltas.json", JSON.stringify([...deltas].map(([k, v]) => [k, { removed: [...v.removed], addedNormal: [...v.addedNormal], addedDev: [...v.addedDev], addedOptional: [...v.addedOptional] }]), null, 1));
const report: any = { locks: 0, changed: 0, ops: {} as Record<string, number>, anomalies: [] as string[], touched: [] as string[] };
for (const lock of locks) {
  const text = readFileSync(join(root, lock), "utf8");
  const parts = text.split("\n[[package]]\n");
  const header = parts[0];
  const blocks = parts.slice(1).map(parseBlock);
  const lockDir = dirname(join(root, lock));
  const wsDoc = wsRoots.get(lockDir);
  const memberDirs: string[] = [];
  if (wsDoc) {
    for (const mm of wsDoc.workspace.members ?? []) memberDirs.push(resolve(lockDir, mm));
    if (wsDoc.package) memberDirs.push(lockDir);
  } else { memberDirs.push(lockDir); }
  const memberNames = new Set<string>();
  for (const d of memberDirs) { const m = byDir.get(d); if (m) memberNames.add(m.name); else report.anomalies.push(`${lock}: member dir without manifest ${d.slice(root.length)}`); }
  const idx = new Map<string, B[]>();
  for (const b of blocks) { if (!idx.has(b.name)) idx.set(b.name, []); idx.get(b.name)!.push(b); }
  const reach = (): Set<string> => {
    const seen = new Set<string>(); const stack = [...memberNames];
    while (stack.length) { const n = stack.pop()!; if (seen.has(n)) continue; seen.add(n); for (const b of idx.get(n) ?? []) for (const d of b.deps) stack.push(d.split(" ")[0]); }
    return seen;
  };
  const r0 = reach();
  const orphans0 = blocks.filter((b) => !r0.has(b.name)).map((b) => b.name);
  if (orphans0.length) report.anomalies.push(`${lock}: ${orphans0.length} blocks unreachable before (${orphans0.slice(0, 3).join(",")})`);
  let changed = false;
  const log = (op: string) => { report.ops[op] = (report.ops[op] ?? 0) + 1; changed = true; };
  const needed: string[] = [];
  for (const b of blocks) {
    if (b.source) continue;
    const dl = deltas.get(b.name);
    if (!dl) continue;
    const isMember = memberNames.has(b.name);
    const drop = (f: string) => { if (b.deps.some((d) => d.split(" ")[0] === f)) { b.deps = b.deps.filter((d) => d.split(" ")[0] !== f); log("drop-edge:" + f); } };
    const add = (f: string) => { if (!b.deps.some((d) => d.split(" ")[0] === f)) { const i = b.deps.findIndex((d) => bytes(d.split(" ")[0], f) > 0); b.deps.splice(i < 0 ? b.deps.length : i, 0, f); log("add-edge:" + f); needed.push(f); } };
    for (const f of dl.removed) drop(f);
    for (const f of dl.addedNormal) add(f);
    if (isMember) for (const f of dl.addedDev) add(f);
    for (const f of dl.addedOptional) report.anomalies.push(`${lock}: ${b.name} gained optional ${f} (edge only if a feature activates it)`);
    if (!isMember) for (const f of dl.addedDev) if (b.deps.some((d) => d.split(" ")[0] === "semio-framework-2d")) { }
  }
  const r1 = reach();
  const missing = FAMILY.filter((f) => needed.includes(f) && !idx.has(f));
  if (missing.length) report.anomalies.push(`${lock}: needs block(s) ${missing.join(",")}`);
  const prune = blocks.filter((b) => r0.has(b.name) && !r1.has(b.name)).map((b) => b.name);
  for (const n of new Set(prune)) log("prune-block:" + n);
  if (changed) {
    report.changed++; report.touched.push(lock);
    if (mode === "apply" && !missing.length) {
      const kept = blocks.filter((b) => !prune.includes(b.name) || !idx.get(b.name)!.includes(b));
      const out = [header, ...kept.map((b) => render(b))].join("\n[[package]]\n");
      writeFileSync(join(root, lock), out);
    }
  }
  report.locks++;
}
await Bun.write(S + "/lock-sync-report.json", JSON.stringify(report, null, 1));
console.log(JSON.stringify({ locks: report.locks, changed: report.changed, ops: report.ops, anomalies: report.anomalies.length }));
