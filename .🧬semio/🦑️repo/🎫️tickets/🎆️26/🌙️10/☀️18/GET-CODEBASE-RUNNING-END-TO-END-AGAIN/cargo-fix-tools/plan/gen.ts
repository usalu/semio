import { readFileSync, existsSync, statSync, readdirSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

// 🗺️ Wave 1020d ordered compile plan (executor cargo-fix). Replaces cargo-plan-tools/gen.ts: no `find`, dependency-ordered chunks of at most 8 crates.
// Run: bun cargo-fix-tools/plan/gen.ts  (reads git ls-files + stat, writes steps.tsv, plan-commands.txt, summary.json next to this file)

const ROOT = "/Users/ueli/Documents/semio";
const C = `${ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`;
const T = `${C}/cargo-fix-tools/plan`;
const G = `${C}/🗑️generated/cargo-fix`;
const DAY0 = Date.parse("2026-10-19T00:00:00+02:00");
const TODAY = Date.parse("2026-10-20T00:00:00+02:00");
const MAXP = 8;
const ALARM = Number(process.env.ALARM ?? 3300);
const LOCK_ALARM = 3000;
const lines = (p: string) => (existsSync(p) ? readFileSync(p, "utf8").split("\n").filter(Boolean) : []);
const toml = (p: string): any => { try { return Bun.TOML.parse(readFileSync(p, "utf8")); } catch { return null; } };
const mt = (p: string) => { try { return statSync(p).mtimeMs; } catch { return 0; } };
const rel = (p: string) => relative(ROOT, p);

const skipPath = (f: string) => f.startsWith(".🧬semio/🦑️repo/🎫️tickets/") || f.includes("🗑️generated/") || f.includes("/node_modules/");
const listed = Bun.spawnSync(["git", "ls-files", "-z", "-co", "--exclude-standard", "--", "*.rs", "*Cargo.toml"], { cwd: ROOT, maxBuffer: 1 << 30 }).stdout.toString().split("\0").filter(Boolean).filter((f) => !skipPath(f));
const manifests = listed.filter((f) => f.endsWith("Cargo.toml") && existsSync(join(ROOT, f))).map((f) => resolve(ROOT, f));
const rsTouched = listed.filter((f) => f.endsWith(".rs")).map((f) => resolve(ROOT, f)).filter((f) => mt(f) >= DAY0);

type Pkg = { name: string; dir: string; ws: string; manifest: string; raw: any; hasLib: boolean; touched: Set<string> };
const wsInfo = new Map<string, { dir: string; deps: Record<string, any>; members: string[] }>();
const parsed = new Map<string, any>();
for (const m of manifests) parsed.set(m, toml(m));
const wsRoots = new Set<string>();
for (const [m, t] of parsed) if (t?.workspace) wsRoots.add(m);

const wsCache = new Map<string, string | null>();
function findWorkspaceOf(manifest: string): string | null {
  const t = parsed.get(manifest) ?? toml(manifest);
  if (!t) return null;
  if (t.workspace) return manifest;
  const explicit = t.package?.workspace;
  if (typeof explicit === "string") {
    const cand = resolve(dirname(manifest), explicit, "Cargo.toml");
    if (existsSync(cand)) return cand;
  }
  let d = dirname(dirname(manifest));
  for (;;) {
    const hit = wsCache.get(d);
    if (hit !== undefined && hit !== null) return hit;
    const cand = join(d, "Cargo.toml");
    if (parsed.get(cand)?.workspace) { wsCache.set(d, cand); return cand; }
    if (d.length <= ROOT.length) break;
    d = dirname(d);
  }
  return null;
}

const pkgByDir = new Map<string, Pkg>();
const pkgsByName = new Map<string, Pkg[]>();
for (const [manifest, t] of parsed) {
  if (!t?.package?.name) continue;
  const dir = dirname(manifest);
  const ws = findWorkspaceOf(manifest);
  if (!ws) continue;
  const hasLib = !!t.lib || existsSync(join(dir, "src/lib.rs"));
  const p: Pkg = { name: t.package.name, dir, ws, manifest, raw: t, hasLib, touched: new Set() };
  pkgByDir.set(dir, p);
  (pkgsByName.get(p.name) ?? pkgsByName.set(p.name, []).get(p.name)!).push(p);
}
for (const w of wsRoots) {
  const t = parsed.get(w);
  wsInfo.set(w, { dir: dirname(w), deps: t.workspace.dependencies ?? {}, members: [...pkgByDir.values()].filter((p) => p.ws === w).map((p) => p.dir) });
}

const moduleDirOf = (d: string) => { const i = d.indexOf("/📦️packages/"); return i >= 0 ? d.slice(0, i) : d; };
const byModule = new Map<string, Pkg[]>();
for (const p of pkgByDir.values()) { const k = moduleDirOf(p.dir); (byModule.get(k) ?? byModule.set(k, []).get(k)!).push(p); }
const touch = (p: Pkg | undefined, why: string) => { if (p) p.touched.add(why); };
const pkgsOfFile = (f: string): Pkg[] => {
  let d = dirname(f);
  while (d.length > ROOT.length) {
    const a = byModule.get(d);
    if (a) return a;
    const own = pkgByDir.get(d);
    if (own) return [own];
    d = dirname(d);
  }
  return [];
};
const man1019 = manifests.filter((m) => mt(m) >= DAY0);
const man1020 = manifests.filter((m) => mt(m) >= TODAY);
for (const m of man1019) touch(pkgByDir.get(dirname(m)), "manifest-1019");
for (const m of man1020) touch(pkgByDir.get(dirname(m)), "manifest-1020");
for (const f of rsTouched) for (const p of pkgsOfFile(f)) touch(p, "rs-1019");

const resultFiles = readdirSync(C).filter((f) => /^result-(value-fix|cc2|lockref|rg-.*|split-.*|rs-sweep.*|tco2|fixups.*|fem-fix|course-img|testfix-.*|fnsplit.*|cargo-fix)\.md$/.test(f) && !/files\.md$/.test(f));
const mentioned = new Set<string>();
for (const f of resultFiles) for (const tok of readFileSync(`${C}/${f}`, "utf8").match(/[a-z][a-z0-9]*(?:-[a-z0-9]+)+/g) ?? []) if (pkgsByName.has(tok)) mentioned.add(tok);
for (const n of mentioned) for (const p of pkgsByName.get(n)!) touch(p, "result-mention");
const findingsFile = process.env.FINDINGS ?? `${G}/findings-w1.jsonl`;
const findingCrates = new Set<string>();
for (const l of lines(findingsFile)) { try { findingCrates.add(JSON.parse(l).crate.split("#")[0]); } catch {} }
for (const n of findingCrates) for (const p of pkgsByName.get(n) ?? []) touch(p, "static-finding");

const wsToday = new Set<string>();
for (const m of man1020) { const w = findWorkspaceOf(m); if (w) wsToday.add(w); }
const wsRank = (w: string) => { const r = rel(w); return r === "🧰️framework/Cargo.toml" ? 0 : r === "Cargo.toml" ? 1 : r === "🌎️hub/Cargo.toml" ? 2 : r === "🎓️teaching/Cargo.toml" ? 3 : 4; };
const seedOrder = new Map<string, number>(); lines(`${C}/🗑️generated/lockref/order.txt`).forEach((m, i) => seedOrder.set(resolve(ROOT, m), i));
const wsCmp = (a: string, b: string) => wsRank(a) - wsRank(b) || (seedOrder.get(a) ?? 1e9) - (seedOrder.get(b) ?? 1e9) || a.localeCompare(b);

type Edge = { pkg: Pkg; dev: boolean };
const edgeCache = new Map<Pkg, Edge[]>();
const edgesOf = (p: Pkg): Edge[] => {
  let out = edgeCache.get(p);
  if (out) return out;
  out = [];
  const info = wsInfo.get(p.ws);
  for (const sec of ["dependencies", "build-dependencies", "dev-dependencies"]) {
    for (const [k, v] of Object.entries<any>(p.raw[sec] ?? {})) {
      let path: string | undefined = typeof v === "object" ? v.path : undefined;
      let name = typeof v === "object" && v.package ? v.package : k;
      let base = p.dir;
      if (typeof v === "object" && v.workspace && info) { const wd = info.deps[k]; if (wd && typeof wd === "object") { path = wd.path ?? path; name = wd.package ?? name; base = info.dir; } }
      let q: Pkg | undefined;
      if (path) q = pkgByDir.get(resolve(base, path));
      if (!q) q = (pkgsByName.get(name) ?? []).find((x) => x.ws === p.ws);
      if (q && q !== p) out.push({ pkg: q, dev: sec === "dev-dependencies" });
    }
    for (const [, tv] of Object.entries<any>(p.raw.target ?? {})) {
      for (const [k, v] of Object.entries<any>(tv?.[sec] ?? {})) {
        let name = typeof v === "object" && v.package ? v.package : k;
        let q: Pkg | undefined;
        if (typeof v === "object" && v.path) q = pkgByDir.get(resolve(p.dir, v.path));
        if (!q) q = (pkgsByName.get(name) ?? []).find((x) => x.ws === p.ws);
        if (q && q !== p) out.push({ pkg: q, dev: sec === "dev-dependencies" });
      }
    }
  }
  edgeCache.set(p, out);
  return out;
};
const normalDeps = (p: Pkg) => edgesOf(p).filter((e) => !e.dev).map((e) => e.pkg);
const revIndex = new Map<Pkg, Pkg[]>();
for (const p of pkgByDir.values()) for (const e of edgesOf(p)) (revIndex.get(e.pkg) ?? revIndex.set(e.pkg, []).get(e.pkg)!).push(p);

const closure = (roots: Iterable<Pkg>): Set<Pkg> => { const seen = new Set<Pkg>(); const st = [...roots]; for (const r of st) seen.add(r); while (st.length) { const p = st.pop()!; for (const q of normalDeps(p)) if (!seen.has(q)) { seen.add(q); st.push(q); } } return seen; };
const depthMemo = new Map<Pkg, number>();
const sameWs = (p: Pkg) => normalDeps(p).filter((q) => q.ws === p.ws);
const depthOf = (p: Pkg, stack = new Set<Pkg>()): number => {
  const hit = depthMemo.get(p);
  if (hit !== undefined) return hit;
  if (stack.has(p)) return 0;
  stack.add(p);
  let d = 0;
  for (const q of sameWs(p)) d = Math.max(d, 1 + depthOf(q, stack));
  stack.delete(p);
  depthMemo.set(p, d);
  return d;
};

type Step = { id: string; group: string; kind: "lock" | "cargo"; alarm: number; manifest: string; sub: string; flags: string; target: string; pkgs: string[]; est: number; label: string };
const steps: Step[] = [];
const scheduled = new Set<Pkg>();
let seq = 0;
const nid = (g: string) => `tmp-${++seq}-${g}`;
const klass = (p: Pkg, touchedOnly: boolean) => `${p.ws}|${p.hasLib ? "L" : "B"}|${touchedOnly && p.touched.size ? "T" : "U"}`;
const flagsFor = (sub: string, hasLib: boolean, touched: boolean) => {
  const base = sub === "test" ? "--no-fail-fast --lib --bins --tests" : touched ? "--lib --bins --tests" : "--lib --bins";
  const lb = hasLib ? base : base.replace("--lib ", "");
  return `${lb} --keep-going --message-format=short`;
};

/** 🧱 Chunks an ordered package list into runs of at most MAXP crates of one workspace and one flag class; order is preserved so every dependency precedes its dependents. */
function emitOrdered(group: string, label: string, ordered: Pkg[], opts: { sub?: string; target?: string; est?: number; dedupe?: boolean; touchedFlags?: boolean } = {}) {
  const sub = opts.sub ?? "check";
  const list = ordered.filter((p, i) => ordered.indexOf(p) === i && (opts.dedupe === false || !scheduled.has(p)));
  let cur: Pkg[] = [];
  const flush = () => {
    if (!cur.length) return;
    const first = cur[0];
    const touchedFlag = opts.touchedFlags !== false && cur.some((p) => p.touched.size > 0);
    steps.push({ id: nid(group), group, kind: "cargo", alarm: ALARM, manifest: rel(first.ws), sub, flags: opts.target ? `--lib --keep-going --message-format=short` : flagsFor(sub, first.hasLib, touchedFlag), target: opts.target ?? "", pkgs: cur.map((p) => p.name), est: opts.est ?? 6, label });
    if (opts.dedupe !== false) cur.forEach((p) => scheduled.add(p));
    cur = [];
  };
  for (const p of list) {
    if (cur.length && (cur.length >= MAXP || klass(p, opts.touchedFlags !== false) !== klass(cur[0], opts.touchedFlags !== false))) flush();
    cur.push(p);
  }
  flush();
}
const wsEdges = new Map<string, Set<string>>();
for (const p of pkgByDir.values()) for (const q of normalDeps(p)) if (q.ws !== p.ws) (wsEdges.get(p.ws) ?? wsEdges.set(p.ws, new Set()).get(p.ws)!).add(q.ws);
const wsIndex = new Map<string, number>();
{
  const state = new Map<string, number>();
  const visit = (w: string) => {
    if (state.has(w)) return;
    state.set(w, 1);
    for (const d of [...(wsEdges.get(w) ?? [])].sort(wsCmp)) visit(d);
    wsIndex.set(w, wsIndex.size);
  };
  for (const w of [...wsInfo.keys()].sort(wsCmp)) visit(w);
}
const topoOrder = (set: Iterable<Pkg>): Pkg[] => [...set].sort((a, b) => (wsIndex.get(a.ws) ?? 0) - (wsIndex.get(b.ws) ?? 0) || depthOf(a) - depthOf(b) || a.name.localeCompare(b.name));
const emitClosure = (group: string, label: string, roots: Pkg[], opts: Parameters<typeof emitOrdered>[3] = {}) => emitOrdered(group, label, topoOrder(closure(roots)), opts);

const missingNames = new Set<string>();
const named = (names: string[]): Pkg[] => names.flatMap((n) => { const a = pkgsByName.get(n); if (!a) missingNames.add(n); return a ?? []; });
const pre = (s: string) => `semio-framework-${s}`;

//1 lock refresh
const lockWs: { w: string; why: string }[] = [];
for (const w of [...wsRoots].sort(wsCmp)) {
  const info = wsInfo.get(w);
  if (!info || !info.members.length) continue;
  const lock = join(dirname(w), "Cargo.lock");
  const newest = Math.max(mt(w), ...info.members.map((d) => mt(join(d, "Cargo.toml"))));
  const why = wsToday.has(w) ? "manifest-today" : newest >= DAY0 && newest > mt(lock) ? "lock-older-than-manifest" : "";
  if (why) lockWs.push({ w, why });
}
const lockMembers: string[] = [];
for (const { w } of lockWs) { const info = wsInfo.get(w)!; lockMembers.push(`${rel(w)}\t${rel(w)}`); for (const d of info.members) lockMembers.push(`${rel(w)}\t${rel(join(d, "Cargo.toml"))}`); }
writeFileSync(`${T}/lock-members.tsv`, lockMembers.join("\n") + "\n");

//2 value family (leaf-first)
const fam = lines(`${T}/value-family.txt`)[0].split(/\s+/).filter(Boolean);
emitClosure("value", "value family, leaf first", named(fam), { est: 10 });

//3 retained chain (testfix-real carve) and its consumers, rg-core downstream, fem assembly chain
const retainedRoots = named([pre("replication-mutation-retained-field"), pre("replication-mutation-retained"), pre("replication-mutation"), pre("replication-mutation-messages"), pre("replication")]);
emitClosure("retained", "retained-field, retained, mutation, messages, facade (leaf first)", retainedRoots, { est: 10 });
const consumers = new Set<Pkg>();
for (const r of named([pre("replication-mutation-retained-field"), pre("replication-mutation-retained")])) for (const q of revIndex.get(r) ?? []) consumers.add(q);
emitOrdered("retained", "retained consumers (direct dependents, dependency order)", topoOrder(consumers), { est: 10 });
const rgCore = [pre("pack-json"), pre("pack-json-float"), pre("pack-json-source-sink"), pre("tool-run"), pre("tool-run-tick-wire"), pre("job"), pre("job-payload")];
emitClosure("retained", "rg-core downstream (pack-json, tool-run, job)", named(rgCore), { est: 12 });
emitClosure("retained", "root-workspace rg-core crates (pets, store testing)", named([pre("pets-staging-draft"), pre("pets-staging"), pre("os-store-testing-prelude"), pre("os-store-testing-fixtures")]), { est: 8 });
emitClosure("fem", "fem assembly and analyses (leaf first)", named(["semio-s-fem-analyses-assembly", "semio-s-fem-analyses"]), { est: 12 });

//4 teaching proctor and course crates
const teaching = [...pkgByDir.values()].filter((p) => rel(p.ws) === "🎓️teaching/Cargo.toml");
const proctor = teaching.filter((p) => /proctor/.test(p.name));
const course = [...pkgByDir.values()].filter((p) => /^semio-framework-course(-|$)/.test(p.name));
emitClosure("proctor", "teaching proctor closure (leaf first)", proctor, { est: 15 });
emitClosure("course", "course crates (root workspace)", course, { est: 10 });

//5 play and demonstrator closure: engines, then the hub lane roots
const lane = JSON.parse(readFileSync(`${T}/lane-roots.json`, "utf8"));
const hubNames: string[] = lane.hubRoots.map((r: any) => r.name);
const extraAll = (lane.extraRoots as string[]).map((d) => pkgByDir.get(resolve(ROOT, d)));
const unresolvedExtra = (lane.extraRoots as string[]).filter((d, i) => !extraAll[i]);
const extra = extraAll.filter(Boolean) as Pkg[];
const extraByDir = (frag: string) => extra.filter((p) => rel(p.dir).includes(frag));
emitClosure("engines", "engine roots (surface, editor, canvas, node-graph, describe, infinite, wgpu)", extra.filter((p) => !/^semio-hub-/.test(p.name)), { est: 30 });
emitClosure("hub", "hub lane roots (play and demonstrator components), leaf first", [...named(hubNames), ...extra.filter((p) => /^semio-hub-/.test(p.name))], { est: 25 });

//6 all other touched
const rest5 = [...pkgByDir.values()].filter((p) => p.touched.size && !scheduled.has(p));
const reach = new Set<Pkg>(); for (const p of rest5) for (const q of normalDeps(p)) reach.add(q);
const roots5 = rest5.filter((p) => !reach.has(p));
emitClosure("touched", "remaining touched crates, leaf first (closure included)", roots5, { est: 8 });

//7 wasm
const wasmPk = [...extraByDir("🗺️surface"), ...extraByDir("✍️editor"), ...extraByDir("NodeGraph"), ...extraByDir("🧩️puzzle"), ...named(["semio-framework-os-flow-wasm"]), ...extraByDir("🧊️wgpu")];
emitOrdered("wasm", "wasm32-unknown-unknown engines", topoOrder(wasmPk), { target: "wasm32-unknown-unknown", dedupe: false, est: 20, touchedFlags: false });
const canary = named(["semio-hub-lowpoly-component"]);
emitOrdered("wasm", "wasm32-wasip2 canary", canary, { target: "wasm32-wasip2", dedupe: false, est: 15, touchedFlags: false });
emitOrdered("wasm", "wasm32-wasip2 lane components", named(hubNames).filter((p) => !canary.includes(p)), { target: "wasm32-wasip2", dedupe: false, est: 15, touchedFlags: false });

//8 tests: run the tests of touched crates of the priority groups, dependency order (value, retained, fem, proctor, course)
const prioGroups = new Set(["value", "retained", "fem", "proctor", "course"]);
const prioPk: Pkg[] = [];
for (const s of steps) if (prioGroups.has(s.group) && s.sub === "check") for (const n of s.pkgs) for (const p of (pkgsByName.get(n) ?? []).filter((x) => rel(x.ws) === s.manifest)) if (p.touched.size) prioPk.push(p);
const testSteps: Step[] = [];
{
  const save = steps.splice(0, steps.length);
  emitOrdered("test", "run tests of touched crates of the priority groups (dependency order)", prioPk, { sub: "test", est: 12, dedupe: false });
  testSteps.push(...steps.splice(0, steps.length));
  steps.push(...save);
}

const cargoSteps = steps;
const usedWs = new Set(cargoSteps.map((s) => s.manifest));
const lockReq = lockWs.filter(({ w }) => usedWs.has(rel(w)));
const lockExtra = lockWs.filter(({ w }) => !usedWs.has(rel(w)));
const lockSteps = (list: typeof lockWs, group: string): Step[] => {
  const out: Step[] = [];
  for (let i = 0; i < list.length; i += MAXP) {
    const chunk = list.slice(i, i + MAXP);
    out.push({ id: "", group, kind: "lock", alarm: LOCK_ALARM, manifest: "", sub: "metadata", flags: "--locked --format-version 1", target: "", pkgs: chunk.map((c) => rel(c.w)), est: chunk.length * 2, label: group === "lock" ? "lock refresh of workspaces used by the plan (metadata --locked, update --workspace on refusal)" : "lock refresh of the remaining workspaces whose manifests changed (optional, TIER_B=1)" });
  }
  return out;
};
const order = ["value", "retained", "fem", "proctor", "course", "engines", "hub", "touched", "wasm"];
const sortedCargo = [...cargoSteps].sort((a, b) => order.indexOf(a.group) - order.indexOf(b.group));
// the test steps follow the group they test: after the last check step of the groups value..course
const lastPrio = sortedCargo.map((s) => s.group).lastIndexOf("course");
const withTests = [...sortedCargo.slice(0, lastPrio + 1), ...testSteps, ...sortedCargo.slice(lastPrio + 1)];
const allSteps: Step[] = [...lockSteps(lockReq, "lock"), ...withTests, ...lockSteps(lockExtra, "lock-extra")];
const estOf = (s: Step) => (s.kind === "cargo" ? 3 + 0.8 * s.pkgs.length : s.est);
allSteps.forEach((s, i) => { s.id = `${String(i + 1).padStart(3, "0")}-${s.group}`; s.est = Math.round(estOf(s)); });
mkdirSync(T, { recursive: true });
writeFileSync(`${T}/steps.tsv`, allSteps.map((s) => [s.id, s.group, s.kind, s.alarm, s.manifest || "-", s.sub, s.flags, s.target || "-", s.pkgs.join(" "), s.est, s.label].join("\t")).join("\n") + "\n");
const LOGDIR = rel(G) + "/window";
const cmdOf = (s: Step) => s.kind === "lock" ? `lock refresh of ${s.pkgs.length} workspace(s): per workspace [skip if Cargo.lock newer than all member manifests] metadata --locked; on refusal update --workspace; metadata again` : `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- ${s.sub} --manifest-path ${s.manifest} ${s.pkgs.map((p) => "-p " + p).join(" ")}${s.target ? " --target " + s.target : ""} ${s.flags}`;
writeFileSync(`${T}/plan-commands.txt`, allSteps.map((s) => `[${s.id}] ${s.group} | est ${s.est}m | log ${LOGDIR}/${s.id}.log\n    ${cmdOf(s)}`).join("\n") + "\n");

const byGroup: Record<string, { steps: number; pkgs: number; est: number }> = {};
for (const s of allSteps) { const g = (byGroup[s.group] ??= { steps: 0, pkgs: 0, est: 0 }); g.steps++; g.pkgs += s.pkgs.length; g.est += s.est; }
const touchedAll = [...pkgByDir.values()].filter((p) => p.touched.size && !p.dir.includes("/.🧬semio/"));
const notScheduled = touchedAll.filter((p) => !scheduled.has(p));
const summary = { missingNames: [...missingNames], packages: pkgByDir.size, workspaces: wsInfo.size, touchedPackages: touchedAll.length, notScheduledTouched: notScheduled.map((p) => p.name), unresolvedExtra, findingCrates: findingCrates.size, byGroup, steps: allSteps.length, estMinutes: allSteps.filter((s) => s.group !== "lock-extra").reduce((a, s) => a + s.est, 0), lockWorkspaces: lockWs.length };
writeFileSync(`${T}/summary.json`, JSON.stringify(summary, null, 1));
writeFileSync(`${T}/touched.tsv`, touchedAll.map((p) => `${rel(p.ws)}\t${p.name}\t${[...p.touched].join(",")}`).join("\n") + "\n");
console.log(JSON.stringify({ ...summary, notScheduledTouched: notScheduled.length }, null, 1));
