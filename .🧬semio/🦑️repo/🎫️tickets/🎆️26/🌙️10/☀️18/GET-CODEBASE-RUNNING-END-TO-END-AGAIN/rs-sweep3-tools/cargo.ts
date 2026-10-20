import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join, resolve, basename } from "node:path";
import { readText } from "./io.ts";

export type Finding = {
  root?: string;
  pkg?: string;
  dep?: string;
  alias?: string;
  aliasOf?: string;
  ambiguous?: string[];
  envs?: string;
  crate: string;
  kind: string;
  sev: "error" | "warn" | "info";
  file: string;
  line: number;
  msg: string;
  fix?: string;
};

export type DepSpec = {
  key: string;
  section: "dependencies" | "dev-dependencies" | "build-dependencies";
  target: string | null;
  workspace: boolean;
  pkg: string;
  path: string | null;
  optional: boolean;
  features: string[];
  defaultFeatures: boolean;
  version: string | null;
  inherited: boolean;
};

export type TargetDecl = {
  kind: "lib" | "bin" | "test" | "bench" | "example" | "build";
  name: string;
  path: string | null;
  declared: boolean;
  requiredFeatures: string[];
};

export type Workspace = { dir: string; toml: any; deps: Record<string, any> };

export type Package = {
  name: string;
  dir: string;
  manifest: string;
  toml: any;
  edition: string;
  libName: string | null;
  procMacro: boolean;
  targets: TargetDecl[];
  deps: DepSpec[];
  features: Record<string, string[]>;
  ws: Workspace | null;
  findings: Finding[];
};

const tomlCache = new Map<string, any>();
const pkgCache = new Map<string, Package | null>();
const wsCache = new Map<string, Workspace | null>();

/** 📖 Parses a TOML file once; returns the error text on failure. */
export function readToml(file: string): { ok: true; v: any } | { ok: false; err: string } {
  if (tomlCache.has(file)) {
    const v = tomlCache.get(file);
    return v instanceof Error ? { ok: false, err: v.message } : { ok: true, v };
  }
  try {
    const v = (Bun as any).TOML.parse(readText(file));
    tomlCache.set(file, v);
    return { ok: true, v };
  } catch (e) {
    const err = e instanceof Error ? e : new Error(String(e));
    tomlCache.set(file, err);
    return { ok: false, err: err.message.split("\n")[0] };
  }
}

function findWorkspace(startDir: string, explicit: string | undefined): Workspace | null {
  const dir = explicit ? resolve(startDir, explicit) : startDir;
  const key = `${startDir}|${explicit ?? ""}`;
  if (wsCache.has(key)) return wsCache.get(key)!;
  let out: Workspace | null = null;
  const tryDir = (d: string): Workspace | null => {
    const m = join(d, "Cargo.toml");
    if (!existsSync(m)) return null;
    const t = readToml(m);
    if (!t.ok || !t.v.workspace) return null;
    return { dir: d, toml: t.v, deps: t.v.workspace.dependencies ?? {} };
  };
  if (explicit) out = tryDir(dir);
  else {
    let d = startDir;
    for (let i = 0; i < 40; i++) {
      out = tryDir(d);
      if (out) break;
      const p = dirname(d);
      if (p === d) break;
      d = p;
    }
  }
  wsCache.set(key, out);
  return out;
}

function inheritField(v: any, ws: Workspace | null, field: string): any {
  if (v && typeof v === "object" && v.workspace === true) return ws?.toml?.workspace?.package?.[field];
  return v;
}

function depSpecs(toml: any, ws: Workspace | null, dir: string): DepSpec[] {
  const out: DepSpec[] = [];
  const secs: Array<["dependencies" | "dev-dependencies" | "build-dependencies", string | null, any]> = [];
  for (const s of ["dependencies", "dev-dependencies", "build-dependencies"] as const) {
    if (toml[s]) secs.push([s, null, toml[s]]);
    for (const [tk, tv] of Object.entries<any>(toml.target ?? {})) if (tv?.[s]) secs.push([s, tk, tv[s]]);
  }
  for (const [section, target, table] of secs) {
    for (const [key, raw] of Object.entries<any>(table)) {
      let spec: any = typeof raw === "string" ? { version: raw } : raw;
      let inherited = false;
      let base = dir;
      let wsSpec: any = null;
      if (spec.workspace === true) {
        inherited = true;
        wsSpec = ws?.deps?.[key] ?? null;
        const w = typeof wsSpec === "string" ? { version: wsSpec } : wsSpec ?? {};
        spec = { ...w, ...spec, features: [...(w.features ?? []), ...(spec.features ?? [])], optional: spec.optional ?? false };
        if (w.path) base = ws!.dir;
        if (spec.path === undefined && w.path !== undefined) spec.path = w.path;
        if (w.package && !spec.package) spec.package = w.package;
      }
      out.push({
        key,
        section,
        target,
        workspace: inherited,
        pkg: spec.package ?? key,
        path: spec.path ? resolve(inherited && wsSpec?.path ? ws!.dir : dir, spec.path) : null,
        optional: spec.optional === true,
        features: spec.features ?? [],
        defaultFeatures: spec["default-features"] !== false && spec.default_features !== false,
        version: spec.version ?? null,
        inherited,
      });
      if (inherited && ws && wsSpec === null) out[out.length - 1].version = "__missing_workspace_entry__";
    }
  }
  return out;
}

/** 📦 Loads a package manifest into the typed package model (null if there is no [package]). */
export function loadPackage(dir: string): Package | null {
  const cached = pkgCache.get(dir);
  if (cached !== undefined) return cached;
  const manifest = join(dir, "Cargo.toml");
  let pkg: Package | null = null;
  if (existsSync(manifest)) {
    const t = readToml(manifest);
    const findings: Finding[] = [];
    if (!t.ok) {
      findings.push({ crate: dir, kind: "toml-parse", sev: "error", file: manifest, line: 0, msg: t.err });
      pkg = { name: basename(dir), dir, manifest, toml: {}, edition: "2021", libName: null, procMacro: false, targets: [], deps: [], features: {}, ws: null, findings };
    } else if (t.v.package) {
      const ws = findWorkspace(dir, t.v.package.workspace);
      const edition = inheritField(t.v.package.edition, ws, "edition") ?? "2021";
      const name = t.v.package.name as string;
      const lib = t.v.lib;
      const targets: TargetDecl[] = [];
      const libDefault = join(dir, "src/lib.rs");
      let libName: string | null = null;
      if (lib || existsSync(libDefault)) {
        libName = lib?.name ?? name.replace(/-/g, "_");
        targets.push({ kind: "lib", name: libName!, path: join(dir, lib?.path ?? "src/lib.rs"), declared: !!lib?.path, requiredFeatures: [] });
      }
      const decl = (arr: any[] | undefined, kind: TargetDecl["kind"], def: (n: string) => string) => {
        for (const b of arr ?? []) {
          const n = b.name ?? name;
          targets.push({ kind, name: n, path: join(dir, b.path ?? def(n)), declared: !!b.path, requiredFeatures: b["required-features"] ?? [] });
        }
      };
      decl(t.v.bin, "bin", (n) => `src/bin/${n}.rs`);
      decl(t.v.test, "test", (n) => `tests/${n}.rs`);
      decl(t.v.bench, "bench", (n) => `benches/${n}.rs`);
      decl(t.v.example, "example", (n) => `examples/${n}.rs`);
      if (!(t.v.bin ?? []).length && existsSync(join(dir, "src/main.rs"))) targets.push({ kind: "bin", name, path: join(dir, "src/main.rs"), declared: false, requiredFeatures: [] });
      const auto = (sub: string, kind: TargetDecl["kind"], flag: string) => {
        if (t.v.package[flag] === false) return;
        const d = join(dir, sub);
        if (!existsSync(d)) return;
        const known = new Set(targets.filter((x) => x.kind === kind).map((x) => x.path));
        for (const f of readdirSync(d)) if (f.endsWith(".rs") && !known.has(join(d, f))) targets.push({ kind, name: f.slice(0, -3), path: join(d, f), declared: false, requiredFeatures: [] });
      };
      auto("tests", "test", "autotests");
      auto("examples", "example", "autoexamples");
      auto("benches", "bench", "autobenches");
      if (t.v.package.build !== false) {
        const b = typeof t.v.package.build === "string" ? t.v.package.build : "build.rs";
        if (typeof t.v.package.build === "string" || existsSync(join(dir, "build.rs"))) targets.push({ kind: "build", name: "build", path: join(dir, b), declared: typeof t.v.package.build === "string", requiredFeatures: [] });
      }
      pkg = {
        name,
        dir,
        manifest,
        toml: t.v,
        edition,
        libName,
        procMacro: lib?.["proc-macro"] === true || lib?.proc_macro === true,
        targets,
        deps: depSpecs(t.v, ws, dir),
        features: t.v.features ?? {},
        ws,
        findings,
      };
    }
  }
  pkgCache.set(dir, pkg);
  return pkg;
}

/** 🧭 Lists workspace member manifests (globs unsupported: members are literal in this repo). */
export function workspaceMembers(ws: Workspace): string[] {
  return (ws.toml.workspace.members ?? []).map((m: string) => resolve(ws.dir, m));
}

/** 🔗 Resolves a dependency spec to the package it points at (null for registry dependencies). */
export function depTarget(d: DepSpec): { pkg: Package | null; missing: boolean } {
  if (!d.path) return { pkg: null, missing: false };
  const p = loadPackage(d.path);
  return { pkg: p, missing: p === null };
}

/** 📏 Is `dir` a cargo package directory whose manifest exists? */
export function isDir(p: string): boolean {
  try {
    return statSync(p).isDirectory();
  } catch {
    return false;
  }
}

export function externName(d: DepSpec, target: Package | null): string {
  if (d.pkg !== d.key) return d.key.replace(/-/g, "_");
  if (target?.libName) return target.libName;
  return d.key.replace(/-/g, "_");
}

/** 🚧 Cargo refuses a path dependency inside a workspace root that is a member of another workspace unless excluded. */
export function wrongWorkspace(pkg: Package, t: Package): boolean {
  if (!pkg.ws || !t.ws || t.ws.dir === pkg.ws.dir || !t.dir.startsWith(pkg.ws.dir + "/")) return false;
  for (const x of pkg.ws.toml.workspace.exclude ?? []) {
    const e = resolve(pkg.ws.dir, x);
    if (t.dir === e || t.dir.startsWith(e + "/")) return false;
  }
  return true;
}
