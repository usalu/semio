import { existsSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { Env, active, compatible, cfgOf, CfgExpr, evalCfg } from "./cfg.ts";
import { CrateTree, Entry, Mod, M, T, V } from "./tree.ts";
import { Ctx, Look, isActive, modActive, resolvePath, externs, libTree } from "./resolve.ts";
import { Ref, UseLeaf } from "./parse.ts";
import { Finding, Package, depTarget, loadPackage, workspaceMembers, wrongWorkspace } from "./cargo.ts";

export const ROOT = "/Users/ueli/Documents/semio";
export const rel = (p: string): string => relative(ROOT, p);

export type Stats = { unknown: number; checked: number; unknownSamples: string[] };

/** 🌐 Lib-name index of every loaded workspace package, used to hint at missing dependencies. */
export const libIndex = new Map<string, Package[]>();
export const regIndex = new Map<string, Map<string, number>>();
export const aliasIndex = new Map<string, Map<string, number>>();

export type ErrInfo = { tree: CrateTree; mod: Mod; env: Env; what: string; segs: string[]; err: NonNullable<Look["err"]>; line: number; ref?: Ref; leaf?: UseLeaf; entry?: Entry; src?: string; envs: string };
export let errorHook: ((i: ErrInfo) => void) | null = null;

/** 🪝 Installs a callback that receives every definite resolution error with its source context. */
export function setErrorHook(h: ((i: ErrInfo) => void) | null): void {
  errorHook = h;
}

export let dupHook: ((tree: CrateTree, mod: Mod, env: Env, a: Entry, b: Entry) => void) | null = null;

/** 🪝 Installs a callback that receives every duplicate-name conflict between two entries. */
export function setDupHook(h: typeof dupHook): void {
  dupHook = h;
}

const seenKeys = new Set<string>();

function emit(out: Finding[], f: Finding): void {
  const k = `${f.crate}|${f.kind}|${f.file}|${f.line}|${f.msg}`;
  if (seenKeys.has(k)) return;
  seenKeys.add(k);
  out.push(f);
}

const HOST: Env = { test: false, wasm: false };
const WASM: Env = { test: false, wasm: true };

function wasmOnly(text: string): boolean {
  const e = cfgOf(text);
  return !!e && evalCfg(e, WASM) === true && evalCfg(e, HOST) === false;
}

/** 🕸️ A crate is built for wasm only when some source cfg or target-specific dependency is wasm-only. */
export function wasmRelevant(tree: CrateTree): boolean {
  const c = (tree as any)._wr as boolean | undefined;
  if (c !== undefined) return c;
  let r = false;
  for (const d of tree.pkg.deps) {
    const m = d.target ? /^cfg\((.*)\)$/s.exec(d.target.trim()) : null;
    if (m ? wasmOnly(m[1]) : !!d.target && /^wasm32/.test(d.target)) r = true;
  }
  if (!r) outer: for (const m of tree.mods) {
    for (const ch of m.cfgChain) if (wasmOnly(ch)) { r = true; break outer; }
    for (const e of m.entries) for (const ch of e.cfg) if (wasmOnly(ch)) { r = true; break outer; }
  }
  (tree as any)._wr = r;
  return r;
}

export function envsFor(tree: CrateTree): Env[] {
  const k = tree.target.kind;
  if (k === "lib") {
    const base = [{ test: false, wasm: false }, { test: true, wasm: false }];
    return tree.pkg.procMacro || !wasmRelevant(tree) ? base : [...base, { test: false, wasm: true }];
  }
  if (k === "bin") return [{ test: false, wasm: false }, { test: true, wasm: false }];
  if (k === "test") return [{ test: true, wasm: false }];
  return [{ test: false, wasm: false }];
}

const envName = (e: Env): string => `${e.test ? "test" : "lib"}${e.wasm ? "+wasm" : ""}`;

function nameIndex(tree: CrateTree): Map<string, Mod[]> {
  const c = (tree as any)._ni as Map<string, Mod[]> | undefined;
  if (c) return c;
  const idx = new Map<string, Mod[]>();
  for (const m of tree.mods) for (const [n, es] of m.idx) if (es.some((e) => e.kind !== "import")) {
    const l = idx.get(n);
    if (l) l.push(m);
    else idx.set(n, [m]);
  }
  (tree as any)._ni = idx;
  return idx;
}

function hint(tree: CrateTree, name: string, mod: Mod): string {
  const here = nameIndex(tree).get(name) ?? [];
  const parts = here.slice(0, 3).map((m) => m.path());
  const elsewhere: string[] = [];
  for (const d of tree.pkg.deps) {
    const t = depTarget(d).pkg;
    if (!t) continue;
    const lt = libTree(t);
    if (lt && nameIndex(lt).has(name)) elsewhere.push(`${t.libName}::${(nameIndex(lt).get(name) ?? [])[0].path().replace(/^crate/, "")}`.replace(/::$/, ""));
    if (elsewhere.length >= 2) break;
  }
  const all = [...parts, ...elsewhere];
  return all.length ? ` (defined as: ${all.join(", ")})` : "";
}

function classify(tree: CrateTree, mod: Mod, line: number, segs: string[], r: Look, what: string, envs: string, out: Finding[], stats: Stats, src?: string, extra?: { env: Env; ref?: Ref; leaf?: UseLeaf; entry?: Entry }): void {
  if (!r.err) {
    stats.checked++;
    return;
  }
  const e = r.err;
  if (errorHook && extra && e.kind !== "unknown") errorHook({ tree, mod, env: extra.env, what, segs, err: e, line, ref: extra.ref, leaf: extra.leaf, entry: extra.entry, src, envs });
  const full = segs.join("::").replace(/^::::/, "::");
  const base = { crate: tree.id, file: rel(src ?? mod.file), line };
  if (e.kind === "unknown") {
    stats.unknown++;
    if (stats.unknownSamples.length < 400) stats.unknownSamples.push(`${tree.id} ${rel(mod.file)}:${line} ${full}`);
    return;
  }
  if (e.kind === "missing" && e.seg === 0) {
    const cands = libIndex.get(e.name.replace(/-/g, "_")) ?? [];
    const reg = regIndex.get(e.name);
    const ali = aliasIndex.get(e.name);
    const rb = { root: rel(tree.root.file), pkg: tree.pkg.name, envs };
    if (cands.length) {
      emit(out, { ...base, ...rb, dep: cands[0].name, ambiguous: cands.length > 1 ? cands.map((c) => c.name) : undefined, kind: "undeclared-dependency", sev: "error", msg: `${what} \`${full}\`: crate \`${e.name}\` (package ${cands.map((c) => c.name).join("|")}) is not a dependency of ${tree.pkg.name} [${envs}]`, fix: `add ${cands[0].name} to ${rel(tree.pkg.manifest)}` });
    } else if (ali && e.name !== "self") {
      const best = [...ali].sort((a, b) => b[1] - a[1])[0][0];
      emit(out, { ...base, ...rb, alias: e.name, aliasOf: best, kind: "missing-extern-alias", sev: "error", msg: `${what} \`${full}\`: \`${e.name}\` is not in scope; the repo convention is \`extern crate ${best} as ${e.name};\` at the crate root [${envs}]`, fix: `extern crate ${best} as ${e.name}` });
    } else if (reg) {
      emit(out, { ...base, ...rb, dep: [...reg].sort((a, b) => b[1] - a[1])[0][0], kind: "undeclared-dependency", sev: "error", msg: `${what} \`${full}\`: registry crate \`${e.name}\` is not a dependency of ${tree.pkg.name} [${envs}]`, fix: `add ${[...reg][0][0]} to ${rel(tree.pkg.manifest)}` });
    } else {
      emit(out, { ...base, kind: what === "use" ? "unresolved-import" : extra?.ref?.bare ? "unresolved-type" : "unresolved-qualifier", sev: "error", msg: `${what} \`${full}\`: first segment \`${e.name}\` is not in module scope, extern prelude or std prelude [${envs}]` });
    }
    return;
  }
  if (e.kind === "no-lib") {
    emit(out, { ...base, kind: "dep-no-lib", sev: "error", msg: `${what} \`${full}\`: dependency \`${e.name}\` has no library target [${envs}]` });
    return;
  }
  const where = e.at ? e.at.path() + (e.at.tree !== tree ? ` (${e.at.tree.pkg.name})` : "") : "?";
  if (e.kind === "private") {
    emit(out, { ...base, kind: "private-item", sev: "error", msg: `${what} \`${full}\`: \`${e.name}\` in ${where} is ${e.entry?.vis.k === "priv" ? "private" : "not visible"}${e.entry?.kind === "import" ? " (private import)" : ""} [${envs}]`, fix: "make it pub / pub use at the root cause" });
    return;
  }
  if (e.kind === "not-module") {
    emit(out, { ...base, kind: "unresolved-import", sev: "error", msg: `${what} \`${full}\`: segment \`${e.name}\` follows a non-module item [${envs}]` });
    return;
  }
  emit(out, { ...base, kind: what === "use" ? "unresolved-import" : "unresolved-path", sev: "error", msg: `${what} \`${full}\`: \`${e.name}\` not found in ${where} [${envs}]${hint(tree, e.name, mod)}` });
}

function checkEnv(tree: CrateTree, env: Env, out: Finding[], stats: Stats): void {
  const ctx: Ctx = { env, home: tree };
  const en = envName(env);
  for (const m of tree.mods) {
    if (m.isEnum || !modActive(m, env)) continue;
    for (const e of m.entries) {
      if (e.kind !== "import" && e.kind !== "extern_crate") continue;
      if (!isActive(e, env)) continue;
      if (e.kind === "extern_crate") {
        const nm = e.leaf!.segs[0];
        if (nm === "self") continue;
        const r = resolvePath(m, ["::", nm], "use", ctx);
        classify(tree, m, e.line, ["extern crate", nm], r, "use", en, out, stats, e.file);
        continue;
      }
      const r = resolvePath(m, e.leaf!.segs, e.leaf!.glob ? "glob" : "use", ctx);
      classify(tree, m, e.leaf!.line, e.leaf!.segs.concat(e.leaf!.glob ? ["*"] : []), r, "use", en, out, stats, e.file, { env, leaf: e.leaf, entry: e });
    }
    for (const ref of m.refs) {
      if (!active(ref.cfg, env)) continue;
      if (ref.kind === "path") {
        if (ref.segs[0] === "Self" || ref.segs[0] === "$crate") continue;
        classify(tree, m, ref.line, ref.segs, resolvePath(m, ref.segs, "code", ctx), "path", en, out, stats, ref.src, { env, ref });
      } else if (ref.kind === "macro") {
        classify(tree, m, ref.line, ref.segs, resolvePath(m, ref.segs, "macro", ctx), "macro", en, out, stats, ref.src, { env, ref });
      } else if (ref.kind === "use") {
        for (const l of ref.leaves ?? []) {
          if (l.segs[0] === "Self") continue;
          classify(tree, m, l.line, l.segs.concat(l.glob ? ["*"] : []), resolvePath(m, l.segs, l.glob ? "glob" : "use", ctx), "use", en, out, stats, ref.src, { env, ref, leaf: l });
        }
      }
    }
  }
}

function checkIncludes(tree: CrateTree, out: Finding[]): void {
  for (const m of tree.mods) for (const ref of m.refs) {
    if (ref.kind !== "include" || !ref.file) continue;
    const nm = ref.segs[0];
    if (nm.startsWith("fixture_")) continue;
    const p = resolve(dirname(ref.src ?? m.file), ref.file);
    if (!existsSync(p)) emit(out, { crate: tree.id, kind: "include-missing", sev: "error", file: rel(ref.src ?? m.file), line: ref.line, msg: `${nm}!(${JSON.stringify(ref.file)}) -> ${rel(p)} does not exist` });
  }
}

const TRANSPARENT = new Set<string>(["impl"]);

function nsOfImport(e: Entry, env: Env, ctx: Ctx): number {
  const r = resolvePath(e.owner, e.leaf!.segs, "use", ctx);
  if (r.err) return 0;
  let ns = 0;
  for (const d of r.defs) {
    if (d.kind === "ext") {
      const last = e.leaf!.segs[e.leaf!.segs.length - 1];
      ns |= /^[A-Z]/.test(last) ? T : 0;
    } else ns |= d.ns;
  }
  return ns;
}

function checkDups(tree: CrateTree, env: Env, out: Finding[]): void {
  const ctx: Ctx = { env, home: tree };
  const en = envName(env);
  for (const m of tree.mods) {
    if (m.isEnum || m.suspect || !modActive(m, env)) continue;
    for (const [name, es] of m.idx) {
      if (es.length < 2 || name === "<impl>") continue;
      const act = es.filter((e) => isActive(e, env) && e.kind !== "macro" && !e.item?.fromMacro);
      if (act.length < 2) continue;
      const ns = act.map((e) => (e.kind === "import" ? nsOfImport(e, env, ctx) : e.ns));
      for (let i = 0; i < act.length; i++) for (let j = i + 1; j < act.length; j++) {
        if (!(ns[i] & ns[j])) continue;
        if (act[i].kind === "import" && act[j].kind === "import" && act[i].leaf!.segs.join("::") === act[j].leaf!.segs.join("::") && act[i].line === act[j].line) continue;
        const ci = [...act[i].cfg, ...m.cfgChain];
        const cj = [...act[j].cfg, ...m.cfgChain];
        if (!compatible(ci, cj, env)) continue;
        const a = act[i];
        const b = act[j];
        if (dupHook) dupHook(tree, m, env, a, b);
        emit(out, { crate: tree.id, kind: "duplicate-name", sev: "error", file: rel(b.file), line: b.line, msg: `\`${name}\` is defined twice in ${m.path()}: ${a.kind}@${rel(a.file).split("/").slice(-3).join("/")}:${a.line} and ${b.kind}@${b.line} [${en}]`, fix: "remove the stale duplicate or the redundant import" });
      }
    }
  }
}

function checkImplDups(tree: CrateTree, env: Env, out: Finding[]): void {
  for (const m of tree.mods) {
    if (m.isEnum || m.suspect || !modActive(m, env)) continue;
    const impls = m.entries.filter((e) => e.item?.kind === "impl" && isActive(e, env));
    for (let i = 0; i < impls.length; i++) for (let j = i + 1; j < impls.length; j++) {
      const a = impls[i].item!;
      const b = impls[j].item!;
      if (a.implKey !== b.implKey || !!a.implTrait !== !!b.implTrait || a.name !== b.name) continue;
      if (!compatible([...impls[i].cfg, ...m.cfgChain], [...impls[j].cfg, ...m.cfgChain], env)) continue;
      if (a.implTrait) {
        if (dupHook) dupHook(tree, m, env, impls[i], impls[j]);
        emit(out, { crate: tree.id, kind: "duplicate-impl", sev: "error", file: rel(impls[j].file), line: b.line, msg: `conflicting \`impl ${a.name} for ${a.implKey}\` at ${rel(impls[i].file)}:${a.line} and ${rel(impls[j].file)}:${b.line} in ${m.path()}` });
        continue;
      }
      for (const x of a.methods ?? []) for (const y of b.methods ?? []) {
        if (x.name === y.name && compatible(x.cfg, y.cfg, env)) emit(out, { crate: tree.id, kind: "duplicate-method", sev: "error", file: rel(impls[j].file), line: y.line, msg: `\`${a.implKey}::${x.name}\` defined at ${rel(impls[i].file)}:${x.line} and ${rel(impls[j].file)}:${y.line} in ${m.path()}` });
      }
    }
  }
}

/** 🔬 Runs every source-level check of one crate target over its environments. */
export function checkSource(tree: CrateTree, out: Finding[], stats: Stats): void {
  for (const f of tree.findings) emit(out, { ...f, file: f.file.startsWith("/") ? rel(f.file) : f.file });
  checkIncludes(tree, out);
  for (const env of envsFor(tree)) {
    checkEnv(tree, env, out, stats);
    checkDups(tree, env, out);
    checkImplDups(tree, env, out);
  }
  const declared = new Set<string>([...Object.keys(tree.pkg.features), ...tree.pkg.deps.filter((d) => d.optional).map((d) => d.key)]);
  const seen = new Set<string>();
  const scan = (chain: readonly string[], m: Mod, line: number) => {
    for (const c of chain) {
      const x = cfgOf(c);
      if (!x) continue;
      const feats = new Set<string>();
      collect(x, feats);
      for (const f of feats) if (!declared.has(f) && !seen.has(f + m.file)) {
        seen.add(f + m.file);
        emit(out, { crate: tree.id, kind: "cfg-feature-undeclared", sev: "warn", file: rel(m.file), line, msg: `cfg(feature = "${f}") but ${tree.pkg.name} declares no such feature` });
      }
    }
  };
  for (const m of tree.mods) for (const e of m.entries) scan(e.cfg, m, e.line);
}

function collect(e: CfgExpr, out: Set<string>): void {
  if (e.k === "atom") {
    if (e.key === "feature" && e.val !== null) out.add(e.val);
  } else if (e.k === "not") collect(e.x, out);
  else for (const x of e.xs) collect(x, out);
}

const wsMemberCache = new Map<string, Set<string>>();

function wsMembers(wsDir: string, ws: NonNullable<Package["ws"]>): Set<string> {
  const hit = wsMemberCache.get(wsDir);
  if (hit) return hit;
  const set = new Set<string>(workspaceMembers(ws).map((p) => resolve(p)));
  const queue = [...set];
  const seen = new Set<string>();
  while (queue.length) {
    const d = queue.pop()!;
    if (seen.has(d)) continue;
    seen.add(d);
    const p = loadPackage(d);
    if (!p) continue;
    for (const dep of p.deps) {
      if (!dep.path || dep.section === "build-dependencies" && false) continue;
      const dd = resolve(dep.path);
      if (dd.startsWith(wsDir + "/") && !set.has(dd)) {
        set.add(dd);
        queue.push(dd);
      }
    }
  }
  const excl = new Set<string>((ws.toml.workspace.exclude ?? []).map((p: string) => resolve(ws.dir, p)));
  for (const x of excl) set.delete(x);
  wsMemberCache.set(wsDir, set);
  return set;
}

/** 📜 Checks one package manifest: targets, dependencies, features and workspace membership. */
export function checkManifest(pkg: Package, out: Finding[]): void {
  const mf = rel(pkg.manifest);
  const c = pkg.name;
  const err = (kind: string, msg: string, sev: Finding["sev"] = "error", fix?: string) => emit(out, { crate: c, kind, sev, file: mf, line: 0, msg, fix });
  for (const f of pkg.findings) emit(out, { ...f, file: rel(f.file) });
  const names = new Map<string, string>();
  for (const t of pkg.targets) {
    if (t.path && !existsSync(t.path)) err("target-path-missing", `[${t.kind}] \`${t.name}\` path ${rel(t.path)} does not exist`);
    if (t.kind !== "lib" && t.kind !== "build") {
      const k = `${t.kind}:${t.name}`;
      if (names.has(k)) err("target-name-dup", `duplicate ${t.kind} target name \`${t.name}\``);
      names.set(k, t.path ?? "");
    }
    for (const rf of t.requiredFeatures) if (!pkg.features[rf] && !pkg.deps.some((d) => d.optional && d.key === rf)) err("required-feature-missing", `[${t.kind}] \`${t.name}\` requires undeclared feature \`${rf}\``);
  }
  const libTargets = pkg.targets.filter((t) => t.kind === "lib");
  if (!pkg.targets.some((t) => t.kind !== "build")) err("no-targets", `package ${c} has no lib, bin, test, bench or example target`);
  if (libTargets.length === 0 && pkg.toml.lib) err("target-path-missing", "[lib] declared without a resolvable path");
  const optional = new Map(pkg.deps.filter((d) => d.optional).map((d) => [d.key, d]));
  const allDeps = new Map<string, Package["deps"][number]>();
  for (const d of pkg.deps) {
    if (d.version === "__missing_workspace_entry__") err("ws-dep-missing", `dependency \`${d.key}\` uses workspace = true but the workspace has no [workspace.dependencies.${d.key}] entry`, "error", `add ${d.key} to ${pkg.ws ? rel(join(pkg.ws.dir, "Cargo.toml")) : "the workspace manifest"}`);
    if (d.section !== "dev-dependencies" || true) allDeps.set(d.key, d);
    if (!d.path) continue;
    if (!existsSync(join(d.path, "Cargo.toml"))) {
      err("dep-path-missing", `dependency \`${d.key}\` path ${rel(d.path)} has no Cargo.toml`);
      continue;
    }
    const t = depTarget(d).pkg;
    if (!t) {
      err("dep-path-missing", `dependency \`${d.key}\` path ${rel(d.path)} is not a package`);
      continue;
    }
    if (wrongWorkspace(pkg, t)) err("dep-wrong-workspace", `dependency \`${d.key}\` (${t.name}) lies inside workspace ${rel(pkg.ws.dir)} but is a member of ${rel(t.ws.dir)} (cargo: member of the wrong workspace)`, "error", "remove the dependency or move the code that needs it to a crate of the right workspace");
    if (t.name !== d.pkg) err("dep-name-mismatch", `dependency \`${d.key}\` expects package \`${d.pkg}\` but ${rel(d.path)} is package \`${t.name}\``, "error", `rename the key (or set package = "${t.name}")`);
    if (!t.libName && !t.procMacro) err("dep-no-lib", `dependency \`${d.key}\` (${t.name}) has no library target`);
    for (const f of d.features) if (!t.features[f] && !t.deps.some((x) => x.optional && x.key === f) && !f.includes("/")) err("dep-feature-missing", `dependency \`${d.key}\` requests feature \`${f}\` that ${t.name} does not declare`);
  }
  const depColon = new Set<string>();
  for (const vs of Object.values(pkg.features)) for (const v of vs) if (v.startsWith("dep:")) depColon.add(v.slice(4));
  const implicit = new Set<string>([...optional.keys()].filter((k) => !depColon.has(k)));
  for (const [f, vs] of Object.entries(pkg.features)) {
    for (const v of vs) {
      if (v.startsWith("dep:")) {
        const k = v.slice(4);
        if (!optional.has(k)) err("feature-dep-undeclared", `feature \`${f}\` uses \`${v}\` but \`${k}\` is not a declared optional dependency`);
      } else if (v.includes("/")) {
        const [k0, ff] = v.split("/");
        const k = k0.replace(/\?$/, "");
        const d = pkg.deps.find((x) => x.key === k && x.section !== "dev-dependencies");
        if (!d) {
          err("feature-dep-undeclared", `feature \`${f}\` references \`${v}\` but \`${k}\` is not a declared dependency`);
          continue;
        }
        const t = d.path ? depTarget(d).pkg : null;
        if (t && !t.features[ff] && !t.deps.some((x) => x.optional && x.key === ff)) err("feature-dep-feature-missing", `feature \`${f}\` references \`${v}\`: ${t.name} declares no feature \`${ff}\``);
      } else if (!pkg.features[v] && !implicit.has(v)) err("feature-undeclared", `feature \`${f}\` enables unknown feature \`${v}\``);
    }
  }
  const dir = pkg.dir;
  const wsDir = pkg.ws?.dir;
  if (pkg.toml.package.workspace && pkg.ws && wsDir !== dir) {
    if (!wsMembers(wsDir!, pkg.ws).has(dir)) err("ws-member-unlisted", `package.workspace = ${JSON.stringify(pkg.toml.package.workspace)} but ${rel(dir)} is not a member of ${rel(join(wsDir!, "Cargo.toml"))} (cargo: "believes it's in a workspace when it's not")`, "error", `add the dir to [workspace].members of ${rel(join(wsDir!, "Cargo.toml"))}`);
  } else if (!pkg.toml.package.workspace && pkg.ws && wsDir !== dir) {
    if (!wsMembers(wsDir!, pkg.ws).has(dir)) err("ws-member-unlisted", `${rel(dir)} lies under workspace ${rel(join(wsDir!, "Cargo.toml"))} but is not a member (add it to members/exclude)`, "error");
  }
  if (!pkg.ws && pkg.toml.package.workspace) err("ws-root-missing", `package.workspace points to ${pkg.toml.package.workspace}, which has no [workspace] manifest`);
}

/** 🗂️ Checks a workspace root: literal members exist, unique package names, workspace.dependencies targets. */
export function checkWorkspace(wsDir: string, out: Finding[]): void {
  const mfPath = join(wsDir, "Cargo.toml");
  const mf = rel(mfPath);
  if (!existsSync(mfPath)) return;
  const tomlFile = (Bun as any).TOML.parse(require("node:fs").readFileSync(mfPath, "utf8"));
  const w = tomlFile.workspace;
  if (!w) return;
  const err = (kind: string, msg: string) => emit(out, { crate: `workspace:${mf}`, kind, sev: "error", file: mf, line: 0, msg });
  const names = new Map<string, string>();
  for (const m of w.members ?? []) {
    if (m.includes("*")) {
      const parent = resolve(wsDir, m.slice(0, m.indexOf("*")));
      if (!existsSync(parent)) err("ws-member-missing", `member glob ${m} has no parent directory`);
      continue;
    }
    const d = resolve(wsDir, m);
    const p = loadPackage(d);
    if (!existsSync(join(d, "Cargo.toml"))) {
      err("ws-member-missing", `member ${m} has no Cargo.toml`);
      continue;
    }
    if (!p) {
      err("ws-member-not-package", `member ${m} has no [package]`);
      continue;
    }
    const prev = names.get(p.name);
    if (prev) err("ws-duplicate-name", `package name \`${p.name}\` appears twice in members: ${prev} and ${m}`);
    names.set(p.name, m);
  }
  for (const [k, v] of Object.entries<any>(w.dependencies ?? {})) {
    if (typeof v !== "object" || !v.path) continue;
    const d = resolve(wsDir, v.path);
    const p = loadPackage(d);
    if (!existsSync(join(d, "Cargo.toml"))) err("ws-dep-path-missing", `[workspace.dependencies.${k}] path ${v.path} has no Cargo.toml`);
    else if (!p) err("ws-dep-path-missing", `[workspace.dependencies.${k}] path ${v.path} is not a package`);
    else if (p.name !== (v.package ?? k)) err("dep-name-mismatch", `[workspace.dependencies.${k}] path ${v.path} is package \`${p.name}\`, key expects \`${v.package ?? k}\``);
  }
}
