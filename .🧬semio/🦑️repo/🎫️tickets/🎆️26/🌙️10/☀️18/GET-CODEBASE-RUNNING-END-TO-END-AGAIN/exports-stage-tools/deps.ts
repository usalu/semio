/**
 * 📚 Dependency declaration gaps: third-party packages a source file cannot resolve from its binding manifest, workspace packages
 * a binding manifest does not declare (requested, observed, adopted by the export batch). Cycle-checked against the workspace graph.
 *
 *   bun deps.ts   (reads generated/candidates.json + moves.json, writes generated/deps.json)
 */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { GENERATED, ROOT } from "./lib.ts";
import { lane, model } from "./model.ts";
import { loadPolicy, ownerOfFile } from "./rules.ts";
import { scan } from "./scan.ts";

type Entry = { packageJson: string; manifest: string; field: "dependencies" | "devDependencies"; name: string; version: string; tier: "tst6" | "requested" | "physical" | "hygiene" | "adoption" | "observed" | "third-party" | "types"; importers: string[]; sources: string[]; note?: string; blocked?: string };
const lock = Bun.JSONC.parse(readFileSync(resolve(ROOT, "bun.lock"), "utf8")) as { packages: Record<string, string[]> };
const lockVersion = (name: string): string | undefined => { const e = lock.packages[name]; return e?.[0] ? e[0].slice(name.length + 1) : undefined; };

const dirs = new Map(model.all.map(p => [p.dir, p]));
const bound = (dir: string): boolean => existsSync(resolve(ROOT, dir, "node_modules"));
export const bindingOf = (file: string) => { let d = file; while (d.includes("/")) { d = d.slice(0, d.lastIndexOf("/")); const p = dirs.get(d); if (p && bound(d)) return p; } return dirs.get(".")!; };
const declares = (m: { deps: Record<string, string>; dev: Record<string, string>; peer: Record<string, string>; json: any }, name: string): boolean => name in m.deps || name in m.dev || name in m.peer || name in (m.json.optionalDependencies ?? {});

const graph = new Map<string, Set<string>>();
for (const p of model.all) {
  const set = graph.get(p.name) ?? graph.set(p.name, new Set()).get(p.name)!;
  for (const f of ["dependencies", "peerDependencies", "optionalDependencies"] as const) for (const [k, v] of Object.entries<string>(p.json[f] ?? {})) if (String(v).startsWith("workspace:") && model.byName.has(k)) set.add(k);
}
/** 🔁 Only the production graph (dependencies, peers, optionals) decides cycles: a devDependency of a test, story or builder never closes one (cycle-break ruling). */
const reaches = (from: string, to: string, extra: Map<string, Set<string>>): boolean => {
  const seen = new Set<string>(); const stack = [from];
  while (stack.length) { const n = stack.pop()!; if (n === to) return true; if (seen.has(n)) continue; seen.add(n); for (const m of graph.get(n) ?? []) stack.push(m); for (const m of extra.get(n) ?? []) stack.push(m); }
  return false;
};
const fieldFor = (m: { deps: Record<string, string>; dev: Record<string, string> }): "dependencies" | "devDependencies" => {
  const ws = (o: Record<string, string>) => Object.values(o).filter(v => String(v).startsWith("workspace:")).length;
  return ws(m.dev) >= ws(m.deps) && (ws(m.dev) > 0 || Object.keys(m.dev).length >= Object.keys(m.deps).length) ? "devDependencies" : "dependencies";
};

if (import.meta.main) {
  const { bare, third } = scan();
  const entries = new Map<string, Entry>();
  const put = (e: Omit<Entry, "importers" | "sources"> & { importer?: string; source: string }): void => {
    const k = e.packageJson + "\0" + e.name;
    const cur = entries.get(k) ?? entries.set(k, { ...e, importers: [], sources: [] } as Entry).get(k)!;
    if (e.importer && !cur.importers.includes(e.importer)) cur.importers.push(e.importer);
    if (!cur.sources.includes(e.source)) cur.sources.push(e.source);
    const rank = { tst6: 0, requested: 0, adoption: 1, "third-party": 2, types: 2, observed: 3 } as const;
    if (rank[e.tier] < rank[cur.tier]) cur.tier = e.tier;
  };

  const declaredWhere = new Map<string, Map<string, string[]>>();
  for (const p of model.all) for (const f of ["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"] as const) for (const [k, v] of Object.entries<string>(p.json[f] ?? {})) { const m = declaredWhere.get(k) ?? declaredWhere.set(k, new Map()).get(k)!; (m.get(v) ?? m.set(v, []).get(v)!).push(p.packageJson); }
  const cache = new Map<string, boolean>();
  const has = (dir: string, pkg: string): boolean => { const k = dir + "\0" + pkg; let v = cache.get(k); if (v === undefined) { v = existsSync(resolve(ROOT, dir, "node_modules", pkg)); cache.set(k, v); } return v; };
  const resolves = (file: string, pkg: string): boolean => { let d = file; while (d.includes("/")) { d = d.slice(0, d.lastIndexOf("/")); if (has(d, pkg)) return true; } return has(".", pkg); };
  const hasOwnTypes = (name: string): boolean => {
    for (const manifests of declaredWhere.get(name)?.values() ?? []) for (const pj of manifests) {
      const dir = pj.slice(0, pj.lastIndexOf("/")) || ".";
      const f = resolve(ROOT, dir, "node_modules", name, "package.json");
      if (existsSync(f)) { try { const j = JSON.parse(readFileSync(f, "utf8")); return !!(j.types || j.typings || (j.exports && JSON.stringify(j.exports).includes('"types"'))); } catch { /* next */ } }
    }
    return false;
  };
  const pickVersion = (name: string): { version: string; from: string[] } | null => {
    const m = declaredWhere.get(name);
    if (m && m.size) { const best = [...m].sort((a, b) => b[1].length - a[1].length)[0]!; return { version: best[0], from: best[1] }; }
    const v = lockVersion(name);
    return v ? { version: v, from: ["bun.lock"] } : null;
  };

  const notes: string[] = [];
  const resolvedDir = (file: string, pkg: string): string | null => { let d = file; while (d.includes("/")) { d = d.slice(0, d.lastIndexOf("/")); if (has(d, pkg)) return d; } return has(".", pkg) ? "." : null; };
  const typesOf = new Map<string, boolean>();
  const bundledTypes = (dir: string, pkg: string): boolean => {
    const k = dir + "\0" + pkg; let v = typesOf.get(k);
    if (v === undefined) { try { const j = JSON.parse(readFileSync(resolve(ROOT, dir, "node_modules", pkg, "package.json"), "utf8")); v = !!(j.types || j.typings || (j.exports && JSON.stringify(j.exports).includes('"types"')) || existsSync(resolve(ROOT, dir, "node_modules", pkg, "index.d.ts"))); } catch { v = true; } typesOf.set(k, v); }
    return v;
  };
  const SKIP_THIRD = new Set(["dotenv"]);
  for (const t of third) {
    if (SKIP_THIRD.has(t.pkg)) continue;
    if (resolves(t.file, t.pkg)) {
      if (!/\.(ts|tsx|mts|cts)$/u.test(t.file) || t.pkg.startsWith("@types/")) continue;
      const dir = resolvedDir(t.file, t.pkg)!;
      if (bundledTypes(dir, t.pkg)) continue;
      const tn = t.pkg.startsWith("@") ? "@types/" + t.pkg.slice(1).replace("/", "__") : "@types/" + t.pkg;
      if (resolves(t.file, tn)) continue;
      const tv = pickVersion(tn);
      const m = bindingOf(t.file);
      if (tv && !declares(m, tn)) put({ packageJson: m.packageJson, manifest: m.name, field: "devDependencies", name: tn, version: tv.version, tier: "types", importer: t.file, source: "types of resolved untyped package " + t.pkg + "; " + tv.from.slice(0, 1).join() });
      continue;
    }
    const pv = pickVersion(t.pkg);
    if (!pv) continue;
    const m = bindingOf(t.file);
    if (declares(m, t.pkg)) continue;
    put({ packageJson: m.packageJson, manifest: m.name, field: "devDependencies", name: t.pkg, version: pv.version, tier: "third-party", importer: t.file, source: "tsc5/glue 7.1 owner scan; declared in " + pv.from.slice(0, 2).join(", ") });
    const isTs = /\.(ts|tsx|mts|cts)$/u.test(t.file);
    if (isTs && !t.pkg.startsWith("@types/") && !hasOwnTypes(t.pkg)) {
      const tn = t.pkg.startsWith("@") ? "@types/" + t.pkg.slice(1).replace("/", "__") : "@types/" + t.pkg;
      const tv = pickVersion(tn);
      if (tv && !declares(m, tn)) put({ packageJson: m.packageJson, manifest: m.name, field: "devDependencies", name: tn, version: tv.version, tier: "types", importer: t.file, source: "types of " + t.pkg + "; " + tv.from.slice(0, 1).join() });
    }
  }

  const wsDeclared = (m: ReturnType<typeof bindingOf>, name: string): boolean => declares(m, name);
  for (const b of bare) {
    const pkg = model.byName.get(b.pkg);
    if (!pkg) continue;
    const m = bindingOf(b.file);
    if (m.packageJson === "package.json" || m.name === b.pkg || wsDeclared(m, b.pkg)) continue;
    put({ packageJson: m.packageJson, manifest: m.name, field: fieldFor(m), name: b.pkg, version: "workspace:*", tier: "observed", importer: b.file, source: "current bare import" });
  }

  const cands = JSON.parse(readFileSync(resolve(GENERATED, "candidates.json"), "utf8")).candidates as { pkg: string; importers: string[] }[];
  for (const c of cands) for (const f of c.importers) {
    const m = bindingOf(f);
    if (m.packageJson === "package.json" || m.name === c.pkg || wsDeclared(m, c.pkg)) continue;
    put({ packageJson: m.packageJson, manifest: m.name, field: fieldFor(m), name: c.pkg, version: "workspace:*", tier: "adoption", importer: f, source: "export batch candidate " + c.pkg });
  }
  const moves = JSON.parse(readFileSync(resolve(GENERATED, "moves.json"), "utf8")) as { pkg: string; rewrites: { file: string }[] }[];
  for (const mv of moves) for (const r of mv.rewrites) {
    const m = bindingOf(r.file);
    if (m.packageJson === "package.json" || m.name === mv.pkg || wsDeclared(m, mv.pkg)) continue;
    put({ packageJson: m.packageJson, manifest: m.name, field: fieldFor(m), name: mv.pkg, version: "workspace:*", tier: "adoption", importer: r.file, source: "moved entry destination " + mv.pkg });
  }

  const requested: [string, string, string][] = [["🌎️hub/package.json", "@semio-tech/framework-renderer-wgpu", "tax2b: hub imports it through the renderer-wgpu entries"], ["🌎️hub/package.json", "@semio-tech/puzzle-js", "tax2b: hub imports it through the puzzle-js entries"], ["♻️mit-bestand/package.json", "@semio-tech/print", "tax2b: mit-bestand imports it through the print entries"]];
  for (const [pj, name, note] of requested) {
    const m = model.byJson.get(pj)!;
    if (declares(m, name)) continue;
    const users = bare.filter(b => b.pkg === name && bindingOf(b.file).packageJson === pj).map(b => b.file);
    put({ packageJson: pj, manifest: m.name, field: fieldFor(m), name, version: "workspace:*", tier: "requested", importer: users[0], source: note });
    const e = entries.get(pj + "\0" + name)!;
    for (const u of users) if (!e.importers.includes(u)) e.importers.push(u);
  }

  const policy = loadPolicy();
  const ownerJson = (file: string): string => { const sel = ownerOfFile(policy, file); const p = sel ? lane(sel.owner) : undefined; if (!p) throw Error("no owning package for " + file); return p.packageJson; };
  const L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
  const tst6: { name: string; version?: string; at: string[]; field?: "dependencies" | "devDependencies"; note: string }[] = [
    { name: "@semio-tech/plugin-registry", version: "workspace:*", field: "dependencies", at: [ownerJson("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts")], note: "wgpu sources import the plugin-registry entries (12 files)" },
    { name: "d3-regression", at: ["🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/package.json"], note: "print inferences probes import it; declared only by the print package" },
    { name: "tar-stream", at: ["🎓️teaching/package.json"], note: "course image test extracts the tar through tar-stream; version as already declared in the repo (2.2.0); bun.lock also holds 3.2.2 (npm latest), both expose extract()" },
    { name: "scheduler", at: ["✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/package.json"], note: "draw script uses scheduler/unstable_mock through createRequire" },
    { name: "cross-spawn", at: [ownerJson("🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/🟦️.ts"), "package.json"], note: "owned-execution capture suites; passed only under the emulated hoist" },
    { name: "lru-cache", at: [ownerJson("🧰️framework/🔨️modules/🫧️transient/🧪️tests/🟦️.ts"), "package.json"], note: "transient suite; passed only under the emulated hoist" },
    { name: "semver", at: [ownerJson("🧰️framework/🔨️modules/🎠️kernel/🧪️tests/✅️satisfy-version-requirements/🟦️.ts"), ownerJson(L + "/🧪️tests/🟦️.ts"), "package.json"], note: "kernel version-requirement suite and the dependency-storage test adapter" },
    { name: "dotenv", at: [ownerJson(L + "/🏗️build/🛠️cmake/🧪️tests/🟦️.ts"), "package.json"], note: "cmake law oracle; tsc-stage patch 14 replaces it with node:util parseEnv, declaring it stays harmless" },
    { name: "safe-regex", at: [ownerJson(L + "/🧪️tests/🟦️.ts"), "package.json"], note: "two pattern-safety suites" },
    { name: "@bytecodealliance/jco-transpile", version: "0.20.0", at: [ownerJson("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts"), "package.json"], note: "browser-bundle script; 0.20.0 is the version jco 1.39.0 (declared at the root) depends on, already in bun.lock" },
    { name: "@iarna/toml", at: [ownerJson(L + "/🧪️tests/🟦️.ts"), "package.json", L + "/⚡️caching/🚀️bootstrap/🛠️tools/package.json"], note: "process-budgets oracle (tst6 root-cause fix) and the rustup installation test" },
    { name: "iconv-lite", at: [L + "/⚡️caching/🚀️bootstrap/🛠️tools/package.json"], note: "bootstrap tools test" },
  ];
  for (const x of tst6) {
    const pv = x.version ? { version: x.version, from: ["npm metadata / bun.lock"] } : pickVersion(x.name);
    if (!pv) { console.log("[tst6] no version for", x.name); continue; }
    for (const pj of [...new Set(x.at)]) {
      const m = model.byJson.get(pj);
      if (!m) { console.log("[tst6] no manifest", pj); continue; }
      if (m.name === x.name || declares(m, x.name)) continue;
      put({ packageJson: pj, manifest: m.name, field: x.field ?? "devDependencies", name: x.name, version: pv.version, tier: "tst6", importer: undefined, source: "result-tst6.md / gaps-undeclared.json: " + x.note + "; version from " + pv.from[0] });
    }
  }
  for (const e of entries.values()) {
    if (e.version !== "workspace:*" || e.tier === "requested" || e.tier === "tst6") continue;
    e.tier = e.importers.some(f => !resolves(f, e.name)) ? "physical" : "hygiene";
    if (e.tier === "hygiene") e.note = "resolves today through the auto-linked workspace members of the binding root; declaration only keeps the manifest honest";
  }
  const extra = new Map<string, Set<string>>();
  const ordered = [...entries.values()].filter(e => e.version === "workspace:*").sort((a, b) => ["tst6", "requested", "physical", "hygiene"].indexOf(a.tier) - ["tst6", "requested", "physical", "hygiene"].indexOf(b.tier));
  for (const e of ordered) {
    const from = e.manifest;
    if (e.field !== "dependencies") continue;
    if (reaches(e.name, from, extra)) { e.blocked = `cycle: ${e.name} already depends on ${from}`; continue; }
    (extra.get(from) ?? extra.set(from, new Set()).get(from)!).add(e.name);
  }
  const out = [...entries.values()];
  writeFileSync(resolve(GENERATED, "deps.json"), JSON.stringify(out));
  const by = new Map<string, number>(); for (const e of out) by.set(e.tier + (e.blocked ? "/blocked" : ""), (by.get(e.tier + (e.blocked ? "/blocked" : "")) ?? 0) + 1);
  console.log("dependency entries", out.length, [...by].map(x => x.join("=")).join(" "), "manifests", new Set(out.map(e => e.packageJson)).size);
  for (const e of out.filter(e => e.tier === "third-party" || e.tier === "types" || e.tier === "requested" || e.blocked)) console.log(e.tier.padEnd(11), e.packageJson.slice(0, 40).padEnd(40), e.name, e.version, e.blocked ?? "", e.importers.length);
  void notes;
}
