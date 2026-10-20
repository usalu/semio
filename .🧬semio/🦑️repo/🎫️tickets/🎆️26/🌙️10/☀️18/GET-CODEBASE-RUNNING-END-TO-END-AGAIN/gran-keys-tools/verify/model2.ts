import {readFileSync, existsSync, writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {cargoLockClosure} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔒️lock/🟨️.mjs";
import {readCargoResolutionPolicyV1, createCargoResolutionContextV1, owningWorkspaceV1, cargoDependenciesV1, crateIdentityV1, toolchainIdentityV1} from "/private/tmp/gk/ov/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🦀️resolution/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const policy = readCargoResolutionPolicyV1(JSON.parse(readFileSync("/private/tmp/gk/policy.json", "utf8")));
const authored: Record<string, {cached: number; cargoToolchainRefs: number; definesCargoToolchain: boolean}> = JSON.parse(readFileSync("/private/tmp/gk/authored.json", "utf8"));
const base = new Map<string, any>();
const readBase = (path: string): any | null => {
  if (base.has(path)) return base.get(path);
  const file = `${ROOT}/${path}`;
  let value: any = null;
  if (existsSync(file)) value = Bun.TOML.parse(readFileSync(file, "utf8"));
  base.set(path, value);
  return value;
};
const hostWith = (edits: Map<string, (value: any) => void>) => ({
  read: async (path: string) => {
    const value = readBase(path);
    const edit = edits.get(path);
    if (!edit || value === null) return value;
    const copy = structuredClone(value);
    edit(copy);
    return copy;
  },
});
const at = (d: string, n: string) => (d === "" ? n : `${d}/${n}`);
const norm = (b: string, r: string): string | undefined => {
  const parts = b === "" ? [] : b.split("/");
  for (const p of r.split("/")) { if (p === "" || p === ".") continue; if (p === "..") { if (!parts.length) return undefined; parts.pop(); } else parts.push(p); }
  return parts.join("/");
};

const workspaces = new Map<string, string[]>();
const queue = [""];
const crateSet = new Set<string>();
while (queue.length) {
  const root = queue.shift()!;
  if (workspaces.has(root)) continue;
  const manifest = readBase(at(root, "Cargo.toml"));
  if (!manifest?.workspace) continue;
  const members: string[] = [];
  for (const m of manifest.workspace.members ?? []) { const dir = norm(root, m); if (dir !== undefined) members.push(dir); }
  workspaces.set(root, members);
  members.forEach((m) => crateSet.add(m));
  if (manifest.package) crateSet.add(root);
  for (const e of manifest.workspace.exclude ?? []) {
    if (/[*?[]/u.test(e)) continue;
    const dir = norm(root, e);
    if (dir === undefined || workspaces.has(dir)) continue;
    if (readBase(at(dir, "Cargo.toml"))?.workspace) queue.push(dir);
  }
}
const crates = [...crateSet].filter((d) => readBase(at(d, "Cargo.toml"))?.package).sort();
console.error("workspaces", workspaces.size, "crates", crates.length);

const baseline = hostWith(new Map());
const ctx0 = createCargoResolutionContextV1(baseline, policy);
const names: string[] = [], owner: (string | null)[] = [], closure: number[][] = [], index = new Map<string, number>();
crates.forEach((d, i) => index.set(d, i));
for (const d of crates) {
  const m = readBase(at(d, "Cargo.toml"));
  names.push(m.package.name);
  const ws = await owningWorkspaceV1(baseline, d);
  owner.push(ws ? ws.root : null);
}
for (const d of crates) {
  const seen = new Set<string>();
  const visit = async (x: string, t: boolean) => {
    const m = readBase(at(x, "Cargo.toml"));
    if (!m) return;
    const ws = await owningWorkspaceV1(baseline, x);
    for (const r of cargoDependenciesV1(x, m, ws, t)) if (r.path !== undefined && !seen.has(r.path)) { seen.add(r.path); await visit(r.path, false); }
  };
  await visit(d, true);
  closure.push([...seen].filter((p) => index.has(p)).map((p) => index.get(p)!).sort((a, b) => a - b));
}
console.error("closures done; mean closure", closure.reduce((a, c) => a + c.length, 0) / crates.length);

const identities = async (edits: Map<string, (v: any) => void>) => {
  const ctx = createCargoResolutionContextV1(hostWith(edits), policy);
  const tool = await toolchainIdentityV1(ctx);
  const rows: string[] = [];
  for (const d of crates) rows.push(await crateIdentityV1(ctx, d, true));
  return {rows, tool};
};
const t0 = performance.now();
const baseId = await identities(new Map());
console.error("identity seconds", (performance.now() - t0) / 1000);

const targetsOf = (d: string): number => {
  const a = authored[d];
  return 3 + (a ? a.cached : 0);
};
const cargoToolchainTargets = (d: string) => authored[d]?.cargoToolchainRefs ?? 0;

const wsRootOf = (d: string): string | null => owner[index.get(d)!] ?? null;
const rootTable = (v: any) => v.workspace;
// scenarios: edits to parsed manifests
type Scenario = {id: string; description: string; path: string; edit: (v: any) => void; before: "all" | "none"; kind: "manifest" | "toolchain" | "config"};
const scenarios: Scenario[] = [];
const firstDep = (ws: any, pick: (n: string) => boolean) => Object.keys(ws.dependencies ?? {}).find(pick);
const addScenario = (s: Scenario) => scenarios.push(s);
const root = readBase("Cargo.toml");
const fw = readBase("🧰️framework/Cargo.toml");
// find the inherited root workspace dependency used by the most crates / the fewest
const usage = new Map<string, number>();
for (let i = 0; i < crates.length; i++) {
  const d = crates[i]!, m = readBase(at(d, "Cargo.toml"));
  if (owner[i] !== "") continue;
  for (const k of ["dependencies", "build-dependencies", "dev-dependencies"]) for (const [alias, decl] of Object.entries<any>(m[k] ?? {})) if (decl && typeof decl === "object" && decl.workspace === true) usage.set(alias, (usage.get(alias) ?? 0) + 1);
}
const ranked = [...usage.entries()].sort((a, b) => b[1] - a[1]);
const busy = ranked[0]!, median = ranked[Math.floor(ranked.length / 2)]!, rare = ranked.filter(([k]) => typeof root.workspace.dependencies[k] === "object" || typeof root.workspace.dependencies[k] === "string").at(-1)!;
const bump = (name: string) => (v: any) => { const e = v.workspace.dependencies[name]; if (typeof e === "string") v.workspace.dependencies[name] = e + "-edited"; else e.version = (e.version ?? "0") + "-edited"; };
addScenario({id: "root-members", description: "root Cargo.toml: append one member to [workspace].members (a new crate is added)", path: "Cargo.toml", edit: (v) => { v.workspace.members.push("new/crate"); }, before: "all", kind: "manifest"});
addScenario({id: "root-comment", description: "root Cargo.toml: comment-only edit (no parsed difference)", path: "Cargo.toml", edit: () => {}, before: "all", kind: "manifest"});
addScenario({id: "root-dep-busy", description: `root Cargo.toml: edit [workspace.dependencies].${busy[0]} (used by ${busy[1]} root crates)`, path: "Cargo.toml", edit: bump(busy[0]), before: "all", kind: "manifest"});
addScenario({id: "root-dep-median", description: `root Cargo.toml: edit [workspace.dependencies].${median[0]} (used by ${median[1]} root crates)`, path: "Cargo.toml", edit: bump(median[0]), before: "all", kind: "manifest"});
addScenario({id: "root-dep-rare", description: `root Cargo.toml: edit [workspace.dependencies].${rare[0]} (used by ${rare[1]} root crate)`, path: "Cargo.toml", edit: bump(rare[0]), before: "all", kind: "manifest"});
addScenario({id: "root-lints", description: "root Cargo.toml: edit [workspace.lints]", path: "Cargo.toml", edit: (v) => { v.workspace.lints.rust.unused_lifetimes = "deny"; }, before: "all", kind: "manifest"});
addScenario({id: "root-package", description: "root Cargo.toml: edit [workspace.package].rust-version", path: "Cargo.toml", edit: (v) => { v.workspace.package["rust-version"] = "1.96"; }, before: "all", kind: "manifest"});
addScenario({id: "root-profile", description: "root Cargo.toml: add a [profile.dev] table", path: "Cargo.toml", edit: (v) => { v.profile = {dev: {"opt-level": 1}}; }, before: "all", kind: "manifest"});
addScenario({id: "framework-lints", description: "nested 🧰️framework/Cargo.toml: edit [workspace.lints] (not in any key today)", path: "🧰️framework/Cargo.toml", edit: (v) => { v.workspace.lints.rust.unused_lifetimes = "deny"; }, before: "none", kind: "manifest"});
addScenario({id: "framework-dep", description: "nested 🧰️framework/Cargo.toml: edit one [workspace.dependencies] entry (not in any key today)", path: "🧰️framework/Cargo.toml", edit: (v) => { const k = Object.keys(v.workspace.dependencies)[0]!; bump(k)(v); }, before: "none", kind: "manifest"});
addScenario({id: "config-comment", description: ".cargo/config.toml: comment-only edit", path: ".cargo/config.toml", edit: () => {}, before: "all", kind: "config"});
addScenario({id: "config-target-dir", description: ".cargo/config.toml: edit [build].target-dir only", path: ".cargo/config.toml", edit: (v) => { v.build["target-dir"] += "-x"; }, before: "all", kind: "config"});
addScenario({id: "config-flags", description: ".cargo/config.toml: edit [build].rustflags", path: ".cargo/config.toml", edit: (v) => { v.build.rustflags.push("-Zfoo"); }, before: "all", kind: "config"});
addScenario({id: "toolchain-comment", description: "rust-toolchain.toml: comment-only edit", path: "rust-toolchain.toml", edit: () => {}, before: "all", kind: "toolchain"});
addScenario({id: "toolchain-channel", description: "rust-toolchain.toml: channel bump", path: "rust-toolchain.toml", edit: (v) => { v.toolchain.channel = "nightly-2026-08-01"; }, before: "all", kind: "toolchain"});

const result: any[] = [];
const totalTargets = crates.reduce((a, d) => a + targetsOf(d), 0);
for (const s of scenarios) {
  const edits = new Map<string, (v: any) => void>([[s.path, s.edit]]);
  const after = await identities(edits);
  const changed: number[] = [];
  for (let i = 0; i < crates.length; i++) if (after.rows[i] !== baseId.rows[i]) changed.push(i);
  const toolChanged = after.tool.toolchain !== baseId.tool.toolchain || after.tool.config !== baseId.tool.config;
  const afterCrates = toolChanged ? crates.length : changed.length;
  const afterTargets = toolChanged ? totalTargets : changed.reduce((a, i) => a + targetsOf(crates[i]!), 0);
  const beforeCrates = s.before === "all" ? crates.length : 0;
  const beforeTargets = s.before === "all" ? totalTargets : 0;
  result.push({id: s.id, description: s.description, beforeCrates, beforeTargets, afterCrates, afterTargets, toolChanged});
}

// lock scenarios: per workspace lock, package-level closure membership
const lockStats: Record<string, {crates: number; packages: number}> = {};
const perCrateLock: {digest: string; ids: number[]; ws: string | null}[] = new Array(crates.length);
const lockNames = new Map<string, string[]>();
const byWs = new Map<string, number[]>();
crates.forEach((d, i) => { const w = owner[i]; if (w === null) return; (byWs.get(w) ?? byWs.set(w, []).get(w)!).push(i); });
let stale = 0, missing = 0;
const nearestLock = (dir: string): string | null => { for (let d = dir; ; d = d.includes("/") ? d.slice(0, d.lastIndexOf("/")) : "") { if (existsSync(`${ROOT}/${at(d, "Cargo.lock")}`)) return d; if (d === "") return null; } };
let nearestMismatch: string[] = [];
for (const [wsRoot, members] of byWs) {
  const file = `${ROOT}/${at(wsRoot, "Cargo.lock")}`;
  if (!existsSync(file)) { missing += members.length; continue; }
  const lock: any = Bun.TOML.parse(readFileSync(file, "utf8"));
  const wsManifest = readBase(at(wsRoot, "Cargo.toml"));
  const pkgs = (lock.package ?? []).map((p: any) => `${p.name}@${p.version}`);
  lockNames.set(wsRoot, pkgs);
  const idOf = new Map<string, number>();
  (lock.package ?? []).forEach((p: any, i: number) => idOf.set(`${p.name}@${p.version}${p.source ? "|" + p.source : ""}`, i));
  for (const i of members) {
    const m = readBase(at(crates[i]!, "Cargo.toml"));
    try {
      const version = m.package.version?.workspace ? wsManifest.workspace?.package?.version : m.package.version;
      const c = cargoLockClosure(lock, [{name: m.package.name, version}]);
      const ids = c.packages.map((p: any) => idOf.get(`${p.name}@${p.version}${p.source ? "|" + p.source : ""}`) ?? -1).sort((a: number, b: number) => a - b);
      perCrateLock[i] = {digest: createHash("sha256").update(JSON.stringify(c)).digest("hex"), ids, ws: wsRoot};
    } catch { stale++; perCrateLock[i] = {digest: "stale", ids: [], ws: wsRoot}; }
    const nl = nearestLock(crates[i]!);
    if (nl !== wsRoot) nearestMismatch.push(crates[i]!);
  }
}
console.error("lock: stale", stale, "missing lock", missing, "nearest-lock != workspace lock", nearestMismatch.length);

// pick packages in the root lock by closure frequency
const rootNames = lockNames.get("")!;
const freq = new Int32Array(rootNames.length);
for (let i = 0; i < crates.length; i++) if (perCrateLock[i]?.ws === "") for (const id of perCrateLock[i]!.ids) if (id >= 0) freq[id]++;
const order = [...freq.keys()].filter((k) => freq[k]! > 0).sort((a, b) => freq[b]! - freq[a]! || a - b);
const picks = [order[0]!, order[Math.floor(order.length / 2)]!, order[order.length - 1]!];
const rootMembers = byWs.get("")!.length;
const lockScenarios = picks.map((id) => {
  const hit = crates.map((_, i) => i).filter((i) => perCrateLock[i]?.ws === "" && perCrateLock[i]!.ids.includes(id));
  const nxBefore = new Set<number>(hit);
  let authoredExtra = 0, authoredCrates = 0;
  crates.forEach((d, i) => { if (cargoToolchainTargets(d) > 0) { authoredCrates++; authoredExtra += cargoToolchainTargets(d); } });
  const hitTargets = hit.reduce((a, i) => a + targetsOf(crates[i]!), 0);
  const authoredOutside = crates.reduce((a, d, i) => a + (nxBefore.has(i) ? 0 : cargoToolchainTargets(d)), 0);
  const rootWsCrates = byWs.get("")!;
  return {package: rootNames[id], closureCrates: hit.length, afterCrates: hit.length, afterTargets: hitTargets, nxBeforeTargets: hitTargets + authoredOutside, nxBeforeCrates: new Set([...hit, ...crates.map((d, i) => i).filter((i) => cargoToolchainTargets(crates[i]!) > 0)]).size, bazelBeforeCrates: rootWsCrates.length, bazelBeforeTargets: rootWsCrates.reduce((a, i) => a + targetsOf(crates[i]!), 0) + authoredOutside};
});

const fan = (() => {
  const dependents: number[][] = crates.map(() => []);
  closure.forEach((c, i) => c.forEach((j) => dependents[j]!.push(i)));
  return dependents;
})();
const dependents = fan.map((a) => a.length);
const sortedDeps = [...dependents.keys()].sort((a, b) => dependents[a]! - dependents[b]!);
const pickSrc = (q: number) => sortedDeps[Math.floor((sortedDeps.length - 1) * q)]!;
const sourceEdits = [0, 0.5, 0.9, 1].map((q) => { const i = pickSrc(q); const set = [i, ...fan[i]!]; return {crate: crates[i], name: names[i], dependents: fan[i]!.length, crates: set.length, targets: set.reduce((a, j) => a + targetsOf(crates[j]!), 0)}; });
const meanDependents = dependents.reduce((a, b) => a + b, 0) / dependents.length;

writeFileSync("/private/tmp/gk/model2.json", JSON.stringify({workspaces: workspaces.size, crates: crates.length, totalTargets, scenarios: result, lock: {stale, missing, nearestMismatch: nearestMismatch.length, nearestMismatchSample: nearestMismatch.slice(0, 5), rootMembers, picks: lockScenarios}, sourceEdits, meanDependents, authoredCargoToolchainTargets: crates.reduce((a, d) => a + cargoToolchainTargets(d), 0), authoredCargoToolchainProjects: Object.values(authored).filter((v) => v.definesCargoToolchain).length, busy, median, rare}, null, 1));
console.error("done");
