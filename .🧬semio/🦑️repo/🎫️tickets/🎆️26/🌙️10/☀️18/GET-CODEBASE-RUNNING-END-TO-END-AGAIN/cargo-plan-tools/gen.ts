import { readFileSync, existsSync, statSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const C = `${ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`;
const T = `${C}/cargo-plan-tools`;
const DAY0 = Date.parse("2026-10-19T00:00:00+02:00");
const TODAY = Date.parse("2026-10-20T00:00:00+02:00");
const MAXP = 8;
const ALARM = Number(process.env.ALARM ?? 3300);
const LOCK_ALARM = 3000;
const lines = (p: string) => (existsSync(p) ? readFileSync(p, "utf8").split("\n").filter(Boolean) : []);
const toml = (p: string): any => { try { return Bun.TOML.parse(readFileSync(p, "utf8")); } catch { return null; } };
const mt = (p: string) => { try { return statSync(p).mtimeMs; } catch { return 0; } };
const rel = (p: string) => relative(ROOT, p);

type Pkg = { name: string; dir: string; ws: string; manifest: string; raw: any; hasLib: boolean; touched: Set<string> };
const wsRoots = new Set<string>();
const pkgByDir = new Map<string, Pkg>();
const pkgsByName = new Map<string, Pkg[]>();
const wsInfo = new Map<string, { dir: string; deps: Record<string, any>; members: string[] }>();

for (const m of lines(`${C}/🗑️generated/lockref/order.txt`)) wsRoots.add(resolve(ROOT, m));
const man1019 = new Set(lines(`${T}/scan/man-1019.txt`).map((l) => resolve(ROOT, l)));
const man1020 = new Set(lines(`${T}/scan/man-1020.txt`).map((l) => resolve(ROOT, l)));
const rsTouched = lines(`${T}/scan/rs-1019.txt`).map((l) => resolve(ROOT, l));
const touchedManifests = new Set([...man1019, ...man1020]);

function findWorkspaceOf(manifest: string): string | null {
  const t = toml(manifest);
  if (!t) return null;
  if (t.workspace) return manifest;
  const explicit = t.package?.workspace;
  if (typeof explicit === "string") {
    const cand = resolve(dirname(manifest), explicit, "Cargo.toml");
    if (existsSync(cand)) return cand;
  }
  let d = dirname(dirname(manifest));
  for (;;) {
    const cand = join(d, "Cargo.toml");
    if (existsSync(cand)) { const c = toml(cand); if (c?.workspace) return cand; }
    if (d === ROOT || d.length <= ROOT.length) break;
    d = dirname(d);
  }
  return null;
}

for (const m of touchedManifests) if (existsSync(m)) { const w = findWorkspaceOf(m); if (w) wsRoots.add(w); }

function expandMembers(wdir: string, members: string[]): string[] {
  const out: string[] = [];
  for (const m of members) {
    if (m.endsWith("/*")) {
      const base = resolve(wdir, m.slice(0, -2));
      if (existsSync(base)) for (const e of readdirSync(base, { withFileTypes: true })) if (e.isDirectory() && existsSync(join(base, e.name, "Cargo.toml"))) out.push(join(base, e.name));
    } else out.push(resolve(wdir, m));
  }
  return out;
}

function addPkg(dir: string, wsManifest: string) {
  if (pkgByDir.has(dir)) return;
  const manifest = join(dir, "Cargo.toml");
  const t = toml(manifest);
  if (!t?.package?.name) return;
  const hasLib = !!t.lib || existsSync(join(dir, "src/lib.rs"));
  const explicit = typeof t.package.workspace === "string" ? resolve(dir, t.package.workspace, "Cargo.toml") : "";
  const p: Pkg = { name: t.package.name, dir, ws: explicit && existsSync(explicit) ? explicit : wsManifest, manifest, raw: t, hasLib, touched: new Set() };
  pkgByDir.set(dir, p);
  const a = pkgsByName.get(p.name) ?? [];
  a.push(p);
  pkgsByName.set(p.name, a);
}

for (const w of wsRoots) {
  const t = toml(w);
  if (!t?.workspace) continue;
  const wdir = dirname(w);
  const members = expandMembers(wdir, t.workspace.members ?? []);
  wsInfo.set(w, { dir: wdir, deps: t.workspace.dependencies ?? {}, members });
  if (t.package) addPkg(wdir, w);
  for (const d of members) addPkg(d, w);
}

const moduleDirOf = (d: string) => { const i = d.indexOf("/📦️packages/"); return i >= 0 ? d.slice(0, i) : d; };
const byModule = new Map<string, Pkg[]>();
for (const p of pkgByDir.values()) { const k = moduleDirOf(p.dir); const a = byModule.get(k) ?? []; a.push(p); byModule.set(k, a); }

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

for (const m of man1019) touch(pkgByDir.get(dirname(m)), "manifest-1019");
for (const m of man1020) touch(pkgByDir.get(dirname(m)), "manifest-1020");
for (const f of rsTouched) for (const p of pkgsOfFile(f)) touch(p, "rs-1019");
for (const sc of ["rs-sweep-run/scope.txt", "rs-sweep4-run/scope.txt"]) for (const l of lines(`${C}/🗑️generated/${sc}`)) touch(pkgByDir.get(dirname(resolve(ROOT, l))), "sweep-scope");

const resultFiles = readdirSync(C).filter((f) => /^result-(value-fix|cc2|lockref|rg-.*|split-.*|rs-sweep.*|tco2|fixups.*|fem-fix|course-img)\.md$/.test(f) && !/files\.md$/.test(f));
const mentioned = new Set<string>();
for (const f of resultFiles) for (const tok of readFileSync(`${C}/${f}`, "utf8").match(/[a-z][a-z0-9]*(?:-[a-z0-9]+)+/g) ?? []) if (pkgsByName.has(tok)) mentioned.add(tok);
for (const n of mentioned) for (const p of pkgsByName.get(n)!) touch(p, "result-mention");

const wsToday = new Set<string>();
for (const m of man1020) { const w = findWorkspaceOf(m); if (w) wsToday.add(w); }

const wsRank = (w: string) => {
  const r = rel(w);
  if (r === "🧰️framework/Cargo.toml") return 0;
  if (r === "Cargo.toml") return 1;
  if (r === "🌎️hub/Cargo.toml") return 2;
  if (r === "🎓️teaching/Cargo.toml") return 3;
  return 4;
};
const seedOrder = new Map<string, number>(); [...lines(`${C}/🗑️generated/lockref/order.txt`)].forEach((m, i) => seedOrder.set(resolve(ROOT, m), i));
const wsCmp = (a: string, b: string) => wsRank(a) - wsRank(b) || (seedOrder.get(a) ?? 1e9) - (seedOrder.get(b) ?? 1e9) || a.localeCompare(b);

const depPkgs = (p: Pkg): Pkg[] => {
  const out: Pkg[] = [];
  const info = wsInfo.get(p.ws);
  for (const sec of ["dependencies", "dev-dependencies", "build-dependencies"]) {
    for (const [k, v] of Object.entries<any>(p.raw[sec] ?? {})) {
      let path: string | undefined = typeof v === "object" ? v.path : undefined;
      let name = typeof v === "object" && v.package ? v.package : k;
      if (typeof v === "object" && v.workspace && info) {
        const wd = info.deps[k];
        if (wd && typeof wd === "object") { path = wd.path ?? path; name = wd.package ?? name; }
      }
      let q: Pkg | undefined;
      if (path) q = pkgByDir.get(resolve(path.startsWith(".") && typeof v === "object" && v.workspace && info ? info.dir : p.dir, path));
      if (!q) q = (pkgsByName.get(name) ?? []).find((x) => x.ws === p.ws);
      if (q && q !== p) out.push(q);
    }
  }
  return out;
};

type Step = { id: string; group: string; kind: "lock" | "cargo"; alarm: number; manifest: string; sub: string; flags: string; target: string; pkgs: string[]; est: number; label: string };
const steps: Step[] = [];
const scheduled = new Set<Pkg>();
let seq = 0;
const nid = (g: string) => `tmp-${++seq}-${g}`;

const flagsOf = (hasLib: boolean, sub: string) => sub === "test" ? "--no-run --lib --bins --tests --keep-going --message-format=short" : hasLib ? "--lib --bins --tests --keep-going --message-format=short" : "--bins --tests --keep-going --message-format=short";

function emit(group: string, label: string, list: Pkg[], opts: { sub?: string; target?: string; est?: number; flagsOverride?: string; dedupe?: boolean } = {}) {
  const sub = opts.sub ?? "check";
  const uniq: Pkg[] = [];
  const seen = new Set<Pkg>();
  for (const p of list) { if (seen.has(p) || (opts.dedupe !== false && scheduled.has(p))) continue; seen.add(p); uniq.push(p); }
  const byWs = new Map<string, Pkg[]>();
  for (const p of uniq) { const a = byWs.get(p.ws) ?? []; a.push(p); byWs.set(p.ws, a); }
  const order = [...byWs.keys()].sort((a, b) => list.indexOf(byWs.get(a)![0]) - list.indexOf(byWs.get(b)![0]));
  for (const w of order) {
    for (const hasLib of [true, false]) {
      const part = byWs.get(w)!.filter((p) => p.hasLib === hasLib);
      for (let i = 0; i < part.length; i += MAXP) {
        const chunk = part.slice(i, i + MAXP);
        steps.push({ id: nid(group), group, kind: "cargo", alarm: ALARM, manifest: rel(w), sub, flags: opts.flagsOverride ?? flagsOf(hasLib, sub), target: opts.target ?? "", pkgs: chunk.map((p) => p.name), est: opts.est ?? 6, label });
        chunk.forEach((p) => scheduled.add(p));
      }
    }
  }
}

const missingNames = new Set<string>();
const named = (names: string[]): Pkg[] => names.flatMap((n) => { const a = pkgsByName.get(n); if (!a) missingNames.add(n); return a ?? []; });
const pre = (s: string) => `semio-framework-${s}`;

//1 lock refresh
const lockWs: { w: string; why: string }[] = [];
for (const w of [...wsRoots].sort(wsCmp)) {
  const info = wsInfo.get(w);
  if (!info) continue;
  const lock = join(dirname(w), "Cargo.lock");
  const newest = Math.max(mt(w), ...info.members.map((d) => mt(join(d, "Cargo.toml"))));
  const why = wsToday.has(w) ? "manifest-today" : newest >= DAY0 && newest > mt(lock) ? "lock-older-than-manifest" : "";
  if (why) lockWs.push({ w, why });
}
const lockMembers: string[] = [];
for (const { w } of lockWs) { const info = wsInfo.get(w)!; lockMembers.push(`${rel(w)}\t${rel(w)}`); for (const d of info.members) lockMembers.push(`${rel(w)}\t${rel(join(d, "Cargo.toml"))}`); }
writeFileSync(`${T}/lock-members.tsv`, lockMembers.join("\n") + "\n");
const staleNow = lockWs.filter(({ w }) => { const lock = join(dirname(w), "Cargo.lock"); const info = wsInfo.get(w)!; return Math.max(mt(w), ...info.members.map((d) => mt(join(d, "Cargo.toml")))) > mt(lock); }).length;

//2 value family
const fam = lines(`${T}/value-family.txt`)[0].split(/\s+/).filter(Boolean);
const f2 = (s: string) => pre(s);
const g21 = [f2("value-retained-map"), f2("value-retained-clone"), f2("value-snapshot-read"), f2("value-persistent"), f2("value-registry"), f2("value-schema")];
const g22 = [f2("value-native-owned"), f2("value-edit"), f2("value-native-decoding"), f2("value-native-decoding-tests"), f2("value-derive"), f2("value-derive-expansion")];
const g23 = [f2("pack-json"), f2("pack-json-float"), f2("pack-json-source-sink"), f2("tool-run"), f2("tool-run-tick-wire"), f2("replication"), f2("replication-mutation-retained"), f2("replication-mutation-messages"), f2("job"), f2("job-payload")];
const rest = fam.filter((n) => ![...g21, ...g22, ...g23, f2("value")].includes(n));
emit("value", "value family 1 (value-fix 5)", named(g21), { est: 18 });
emit("value", "value family 2 (native, derive)", named(g22), { est: 12 });
emit("value", "value family rest + facade", named([...rest, f2("value")]), { est: 10 });
emit("value", "rg-core downstream (pack-json, tool-run, replication, job)", named(g23), { est: 12 });
emit("value", "root-workspace rg-core crates (pets, store testing)", named([pre("pets-staging-draft"), pre("pets-staging"), pre("os-store-testing-prelude"), pre("os-store-testing-fixtures")]), { est: 8 });

//3 proctor + course
const teaching = [...pkgByDir.values()].filter((p) => rel(p.ws) === "🎓️teaching/Cargo.toml");
const proctor = teaching.filter((p) => /proctor/.test(p.name)).sort((a, b) => depPkgs(a).length - depPkgs(b).length || a.name.localeCompare(b.name));
const course = [...pkgByDir.values()].filter((p) => /^semio-framework-course(-|$)/.test(p.name));
emit("course", "course crates (root workspace)", course, { est: 10 });
emit("proctor", "teaching proctor (check lib+bins+tests)", proctor, { est: 15 });
emit("proctor", "teaching proctor (test --no-run)", proctor.filter((p) => p.name === "teaching-proctor" || p.name === "teaching-proctor-base"), { sub: "test", est: 15, dedupe: false });

//4 activation closure
const lane = JSON.parse(readFileSync(`${T}/lane-roots.json`, "utf8"));
const hubNames: string[] = lane.hubRoots.map((r: any) => r.name);
const extraDirs: string[] = lane.extraRoots;
const extraAll = extraDirs.map((d) => pkgByDir.get(resolve(ROOT, d)));
const unresolvedExtra = extraDirs.filter((d, i) => !extraAll[i]);
const extra = extraAll.filter(Boolean) as Pkg[];
const extraByDir = (frag: string) => extra.filter((p) => rel(p.dir).includes(frag));
emit("activation", "engine and font roots (surface, editor, node-graph, flow browser, describe, infinite, canvas, wgpu, puzzle)", extra.filter((p) => !/^semio-hub-/.test(p.name)), { est: 30 });
emit("activation", "hub lane roots", named(hubNames), { est: 25 });
emit("activation", "hub puzzle engine", extra.filter((p) => /^semio-hub-/.test(p.name)), { est: 10 });

//5 all other touched
const libDeps = (p: Pkg): Pkg[] => {
  const out: Pkg[] = [];
  const info = wsInfo.get(p.ws);
  for (const sec of ["dependencies", "build-dependencies"]) {
    for (const [k, v] of Object.entries<any>(p.raw[sec] ?? {})) {
      let path: string | undefined = typeof v === "object" ? v.path : undefined;
      let name = typeof v === "object" && v.package ? v.package : k;
      let base = p.dir;
      if (typeof v === "object" && v.workspace && info) { const wd = info.deps[k]; if (wd && typeof wd === "object") { path = wd.path ?? path; name = wd.package ?? name; base = info.dir; } }
      let q: Pkg | undefined;
      if (path) q = pkgByDir.get(resolve(base, path));
      if (!q) q = (pkgsByName.get(name) ?? []).find((x) => x.ws === p.ws);
      if (q && q !== p) out.push(q);
    }
  }
  return out;
};
const libGraph = new Map<Pkg, Pkg[]>();
const gdeps = (p: Pkg) => { let a = libGraph.get(p); if (!a) { a = libDeps(p); libGraph.set(p, a); } return a; };
const closureOf = (roots: Iterable<Pkg>) => { const seen = new Set<Pkg>(); const st = [...roots]; while (st.length) { const p = st.pop()!; for (const q of gdeps(p)) if (!seen.has(q)) { seen.add(q); st.push(q); } } return seen; };
const covered = closureOf(scheduled);
const rest5 = [...pkgByDir.values()].filter((p) => p.touched.size && !scheduled.has(p) && !p.dir.includes("/.🧬semio/"));
const coveredByOther = new Map<Pkg, Pkg>();
for (const p of rest5) for (const q of gdeps(p)) if (!coveredByOther.has(q)) coveredByOther.set(q, p);
const reach = closureOf(rest5);
const roots5 = rest5.filter((p) => !reach.has(p) && !covered.has(p));
const libCovered5 = rest5.filter((p) => !roots5.includes(p));
const inSet = new Set(roots5);
const depth = new Map<Pkg, number>();
const dfs = (p: Pkg, stack: Set<Pkg>): number => {
  if (depth.has(p)) return depth.get(p)!;
  if (stack.has(p)) return 0;
  stack.add(p);
  let d = 0;
  for (const q of depPkgs(p)) if (inSet.has(q) && q.ws === p.ws) d = Math.max(d, 1 + dfs(q, stack));
  stack.delete(p);
  depth.set(p, d);
  return d;
};
for (const p of roots5) dfs(p, new Set());
const sorted5 = roots5.sort((a, b) => wsCmp(a.ws, b.ws) || depth.get(a)! - depth.get(b)! || a.name.localeCompare(b.name));
emit("touched", "touched crate roots (lib closure covers their touched dependencies)", sorted5, { est: 8 });
const named5 = libCovered5.filter((p) => p.touched.has("result-mention"));
const namedSet = new Set(named5);
const tierB = libCovered5.filter((p) => !namedSet.has(p)).sort((a, b) => wsCmp(a.ws, b.ws) || a.name.localeCompare(b.name));
for (const p of named5) dfs(p, new Set());
inSet.clear(); for (const p of named5) inSet.add(p);
depth.clear(); for (const p of named5) dfs(p, new Set());
emit("named", "crates named in executor result files, already lib-covered by tier A (lib + tests)", named5.sort((a, b) => wsCmp(a.ws, b.ws) || depth.get(a)! - depth.get(b)! || a.name.localeCompare(b.name)), { est: 6 });

//6 wasm
const wasmPk = [...extraByDir("🗺️surface"), ...extraByDir("✍️editor"), ...extraByDir("NodeGraph"), ...extraByDir("🧩️puzzle"), ...named(["semio-framework-os-flow-wasm"]), ...extraByDir("🧊️wgpu")];
emit("wasm", "wasm32-unknown-unknown engines", wasmPk, { target: "wasm32-unknown-unknown", flagsOverride: "--lib --keep-going --message-format=short", dedupe: false, est: 20 });
const canary = named(["semio-hub-lowpoly-component"]);
emit("wasm", "wasm32-wasip2 canary", canary, { target: "wasm32-wasip2", flagsOverride: "--lib --keep-going --message-format=short", dedupe: false, est: 15 });
emit("wasm", "wasm32-wasip2 lane components", named(hubNames).filter((p) => !canary.includes(p)), { target: "wasm32-wasip2", flagsOverride: "--lib --keep-going --message-format=short", dedupe: false, est: 15 });

emit("covered", "tier B: own tests of touched crates already lib-covered by tier A (optional, TIER_B=1)", tierB, { flagsOverride: "--tests --keep-going --message-format=short", est: 6, dedupe: false });

const cargoSteps = steps.filter((s) => s.group !== "covered");
const coveredSteps = steps.filter((s) => s.group === "covered");
const usedWs = new Set(cargoSteps.map((s) => s.manifest));
const lockReq = lockWs.filter(({ w }) => usedWs.has(rel(w)));
const lockExtra = lockWs.filter(({ w }) => !usedWs.has(rel(w)));
const lockSteps = (list: typeof lockWs, group: string): Step[] => {
  const out: Step[] = [];
  for (let i = 0; i < list.length; i += MAXP) {
    const chunk = list.slice(i, i + MAXP);
    out.push({ id: "", group, kind: "lock", alarm: LOCK_ALARM, manifest: "", sub: "metadata", flags: "--locked --format-version 1", target: "", pkgs: chunk.map((c) => rel(c.w)), est: chunk.length * 2, label: group === "lock" ? "lock refresh of workspaces used by the window (metadata --locked, update --workspace on refusal)" : "lock refresh of the remaining workspaces whose manifests changed (optional, TIER_B=1)" });
  }
  return out;
};
const allSteps: Step[] = [...lockSteps(lockReq, "lock"), ...cargoSteps, ...lockSteps(lockExtra, "lock-extra"), ...coveredSteps];
const estOf = (s: Step) => (s.kind === "cargo" && (s.group === "touched" || s.group === "covered") ? 3 + 0.8 * s.pkgs.length : s.kind === "cargo" && s.group === "named" ? 2 + 0.6 * s.pkgs.length : s.est);
allSteps.forEach((s, i) => { s.id = `${String(i + 1).padStart(3, "0")}-${s.group}`; s.est = Math.round(estOf(s)); });
const tsv = allSteps.map((s) => [s.id, s.group, s.kind, s.alarm, s.manifest || "-", s.sub, s.flags, s.target || "-", s.pkgs.join(" "), s.est, s.label].join("\t")).join("\n") + "\n";
writeFileSync(`${T}/steps.tsv`, tsv);
const LOGDIR = rel(C) + "/🗑️generated/cargo-window";
const cmdOf = (s: Step) => s.kind === "lock" ? `lock refresh of ${s.pkgs.length} workspace(s): per workspace [skip if Cargo.lock newer than all member manifests] bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- metadata --locked --format-version 1 --manifest-path <ws>; on lock refusal -- update --workspace --manifest-path <ws>; then metadata again` : `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- ${s.sub} --manifest-path ${s.manifest} ${s.pkgs.map((p) => "-p " + p).join(" ")}${s.target ? " --target " + s.target : ""} ${s.flags}`;
writeFileSync(`${T}/plan-commands.txt`, allSteps.map((s) => `[${s.id}] ${s.group}${/^(lock-extra|covered)$/.test(s.group) ? " (optional, TIER_B=1)" : ""} | alarm ${s.alarm}s | est ${s.est}m | collision: wait while any foreign bun repo dependencies|show projects|cargo runs | log ${LOGDIR}/${s.id}.log | marker ${LOGDIR}/${s.id}.done\n    ${cmdOf(s)}${s.kind === "lock" ? "\n    workspaces: " + s.pkgs.join(" ") : ""}`).join("\n") + "\n");

const byGroup: Record<string, { steps: number; pkgs: number; est: number }> = {};
for (const s of allSteps) { const g = (byGroup[s.group] ??= { steps: 0, pkgs: 0, est: 0 }); g.steps++; g.pkgs += s.pkgs.length; g.est += s.est; }
const distinct = new Set(allSteps.filter((s) => s.kind === "cargo").flatMap((s) => s.pkgs.map((p) => `${s.manifest}::${p}`)));
const touchedAll = [...pkgByDir.values()].filter((p) => p.touched.size && !p.dir.includes("/.🧬semio/"));
const srcCount: Record<string, number> = {};
for (const p of touchedAll) for (const s of p.touched) srcCount[s] = (srcCount[s] ?? 0) + 1;
const wsCount = new Set(allSteps.filter((s) => s.kind === "cargo").map((s) => s.manifest));
const summary = { missingNames: [...missingNames], lockStaleNow: staleNow, rootsTierA: roots5.length, namedPromoted: named5.length, tierBCoveredCrates: tierB.length, unresolvedExtra, steps: allSteps.length, requiredSteps: allSteps.filter((s) => !/^(lock-extra|covered)$/.test(s.group)).length, lockReqWs: lockReq.length, lockExtraWs: lockExtra.length, byGroup, distinctCargoPackages: distinct.size, cargoWorkspaces: wsCount.size, lockWorkspaces: lockWs.length, lockWhy: lockWs.map((l) => `${rel(l.w)}\t${l.why}`), touchedPackages: touchedAll.length, touchedBySource: srcCount, workspacesKnown: wsInfo.size, packagesKnown: pkgByDir.size, mentionedNames: mentioned.size, valueFamily: fam.length, estRequiredMinutes: allSteps.filter((s) => !/^(lock-extra|covered)$/.test(s.group)).reduce((a, s) => a + s.est, 0), estOptionalMinutes: allSteps.filter((s) => /^(lock-extra|covered)$/.test(s.group)).reduce((a, s) => a + s.est, 0), notScheduledTouched: touchedAll.filter((p) => !scheduled.has(p)).map((p) => p.name) };
writeFileSync(`${T}/summary.json`, JSON.stringify(summary, null, 1));
writeFileSync(`${T}/touched.tsv`, touchedAll.map((p) => `${rel(p.ws)}\t${p.name}\t${[...p.touched].join(",")}`).join("\n") + "\n");
console.log(JSON.stringify({ ...summary, lockWhy: undefined, notScheduledTouched: summary.notScheduledTouched.length }, null, 1));
