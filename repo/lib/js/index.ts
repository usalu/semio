//#region 🧲Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — @semio-tech/repo-lib/js: bundle scripts, policy runner, linters, dependency-boundary lint.
//#endregion 🧲Header

//#region 🔌Adapters
import { execFileSync, spawn, spawnSync } from "node:child_process";
import { chmodSync, existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, normalize, relative, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { createHash } from "node:crypto";
//#endregion 🔌Adapters

import type { PlaygroundBuildTarget as PlaygroundVariant } from "../../../framework/plugin/registry/generated/playgrounds.ts";

export type PlaygroundHostKind = string;

//#region 🔖breach
/** 🚫BreachRecord is one policy lint finding from `script.ts` (serialized to cache JSON). */
export type BreachPriority = "high" | "medium" | "low";

export type BreachRecord = {
  id: string;
  summary: string;
  /** Statute-style rule id (path or slug) for GraphQL `kindId` / cache round-trip. */
  kind: string;
  scope: string;
  line?: number;
  column?: number;
  excerpt?: string;
  priority?: BreachPriority;
  autofixable?: boolean;
  reason?: string;
  solution?: string;
};
//#endregion 🔖breach

//#region 🔖cli
/** 🔎Resolves monorepo root (directory containing root package.json named `compose`). */
export function getWorkspaceRoot(): string {
  const fromEnv = process.env.REPO_ROOT?.trim();
  if (fromEnv) return resolve(fromEnv);
  let dir = process.cwd();
  for (let i = 0; i < 30; i++) {
    const pkg = join(dir, "package.json");
    if (existsSync(pkg)) {
      try {
        const j = JSON.parse(readFileSync(pkg, "utf8")) as { name?: string };
        if (j.name === "compose") return dir;
      } catch {
        /* ignore */
      }
    }
    const up = dirname(dir);
    if (up === dir) break;
    dir = up;
  }
  return process.cwd();
}

/** @emoji 📎 Reads `{ hash, items }` collection blocks from kit snapshot JSON. */
export function fixtureItemsOf<T = Record<string, unknown>>(node: unknown): readonly T[] {
  if (node && typeof node === "object" && Array.isArray((node as { items?: unknown[] }).items)) {
    return (node as { items: T[] }).items;
  }
  return [];
}

function defaultCliBin(root: string): string {
  const win = process.platform === "win32";
  return join(root, "repo", "client", win ? "client.exe" : "client");
}

export function resolveCliBin(root = getWorkspaceRoot()): string {
  const fromEnv = process.env.REPO_CLI_BIN?.trim();
  if (fromEnv) return resolve(fromEnv);
  return defaultCliBin(root);
}

/** 📡Runs repo client with `--json` and returns parsed GraphQL payload (`data` object). */
export function runCliGraphql(query: string, variables: Record<string, unknown> = {}, options?: { cwd?: string; repoRoot?: string }): unknown {
  const root = options?.repoRoot ?? getWorkspaceRoot();
  const cwd = options?.cwd ?? root;
  const bin = resolveCliBin(root);
  const vars = JSON.stringify(variables ?? {});
  const args = ["--repo", root, "--json", "graphql", "--query", query, "-v", vars];
  let stdout: string;
  try {
    stdout = execFileSync(bin, args, {
      cwd,
      encoding: "utf8",
      maxBuffer: 64 * 1024 * 1024,
    });
  } catch (e: unknown) {
    const err = e as { stderr?: Buffer; status?: number; message?: string };
    const msg = err.stderr?.toString?.() ?? err.message ?? String(e);
    throw new Error(`[repo/cli] exit ${err.status ?? "?"}: ${msg}`);
  }
  const lines = stdout
    .split(/\r?\n/)
    .map((l) => l.trim())
    .filter(Boolean);
  let last: unknown;
  for (const line of lines) {
    try {
      last = JSON.parse(line) as { data?: unknown; errors?: { message: string }[] };
    } catch {
      continue;
    }
  }
  if (!last || typeof last !== "object") {
    throw new Error(`[repo/cli] no JSON lines in stdout: ${stdout.slice(0, 500)}`);
  }
  const payload = last as { data?: unknown; errors?: { message: string }[] };
  if (payload.errors?.length) {
    throw new Error(`[repo/cli] graphql errors: ${payload.errors.map((x) => x.message).join("; ")}`);
  }
  return payload.data ?? payload;
}

const __dirname = dirname(fileURLToPath(import.meta.url));

/** 🧭Package dir for `@semio-tech/repo-lib` (…/repo/lib/js). */
export function getLibRoot(): string {
  return resolve(__dirname, "..");
}
//#endregion 🔖cli

//#region 🔖linter
const NODE_QUERY = `
query NodeQ($id: ID!) {
  node(id: $id) {
    __typename
    ... on File {
      id path name extension fileKind: kind
      sections { id name path range { start end } }
      definitions { id name kind range { start end } }
    }
    ... on Folder {
      id path name
    }
    ... on Bundle {
      id name root bundleKind: kind
    }
    ... on Section {
      id name path
      range { start end }
      sectionFile: file { path }
      definitions { id name kind range { start end } }
    }
    ... on Definition {
      id name defKind: kind
      range { start end }
      defFile: file { path }
    }
  }
}
`;

export type GraphNode = Record<string, unknown> & { __typename?: string };

/** 🧷BaseLinter holds shared repo-root + graphql helpers for lint scripts. */
export abstract class BaseLinter {
  constructor(
    readonly entityId: string,
    protected readonly repoRoot: string = getWorkspaceRoot(),
  ) {}

  protected gql<T = unknown>(query: string, variables: Record<string, unknown> = {}): T {
    return runCliGraphql(query, variables, { repoRoot: this.repoRoot }) as T;
  }

  protected loadNode(): GraphNode {
    const data = this.gql<{ node: GraphNode | null }>(NODE_QUERY, { id: this.entityId });
    const n = data.node;
    if (!n || !n.__typename) {
      throw new Error(`[linter] node not found for id ${this.entityId}`);
    }
    return n;
  }

  /** 🚫Builds a breach with default scope = entity id. */
  breach(p: Omit<BreachRecord, "scope"> & { scope?: string }): BreachRecord {
    const { scope, ...rest } = p;
    return {
      ...rest,
      scope: scope ?? this.entityId,
    };
  }
}

/** 🏗️TechnologyLinter queries a technology node by id. */
export class TechnologyLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) {
      const data = this.gql<{ technologies: GraphNode[] }>(`query T { technologies { id name kind root } }`);
      const found = (data.technologies ?? []).find((t) => String(t.id) === this.entityId);
      if (!found) {
        throw new Error(`[TechnologyLinter] technology not found for id ${this.entityId}`);
      }
      this.node = { ...found, __typename: "Technology" };
    }
    return this.node;
  }

  name(): string {
    return String(this.load().name ?? "");
  }

  kind(): string {
    return String(this.load().kind ?? "");
  }

  root(): string {
    return String(this.load().root ?? "");
  }

  /** 📦Lists bundle rows for this technology. */
  bundles(): GraphNode[] {
    const data = this.gql<{ bundles: GraphNode[] }>(`query B { bundles { id name root kind } }`);
    const tech = this.name();
    return (data.bundles ?? []).filter((b) => String(b.name ?? "").split("/")[0] === tech);
  }
}

/** 📦BundleLinter queries a bundle node by id. */
export class BundleLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "Bundle") {
      throw new Error(`[BundleLinter] expected Bundle, got ${this.node.__typename}`);
    }
    return this.node;
  }

  name(): string {
    return String(this.load().name ?? "");
  }

  root(): string {
    return String(this.load().root ?? "");
  }

  kind(): string {
    return String(this.load().bundleKind ?? "");
  }

  technologyName(): string {
    return this.name().split("/")[0] ?? "";
  }
}

/** 📁FolderLinter queries a folder node by id. */
export class FolderLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "Folder") {
      throw new Error(`[FolderLinter] expected Folder, got ${this.node.__typename}`);
    }
    return this.node;
  }

  path(): string {
    return String(this.load().path ?? "").replaceAll("\\", "/");
  }

  name(): string {
    return String(this.load().name ?? "");
  }
}

/** 📄FileLinter queries a file node by id and reads bytes from disk. */
export class FileLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "File") {
      throw new Error(`[FileLinter] expected File, got ${this.node.__typename}`);
    }
    return this.node;
  }

  path(): string {
    return String(this.load().path ?? "").replaceAll("\\", "/");
  }

  ext(): string {
    return String(this.load().extension ?? "");
  }

  kind(): string {
    return String(this.load().fileKind ?? "");
  }

  content(): string {
    const p = this.path();
    return readFileSync(join(this.repoRoot, p), "utf8");
  }

  lines(): string[] {
    return this.content().split(/\r?\n/);
  }

  sections(): GraphNode[] {
    return (this.load().sections as GraphNode[]) ?? [];
  }

  definitions(): GraphNode[] {
    return (this.load().definitions as GraphNode[]) ?? [];
  }
}

/** 🔖SectionLinter queries a section node by id. */
export class SectionLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "Section") {
      throw new Error(`[SectionLinter] expected Section, got ${this.node.__typename}`);
    }
    return this.node;
  }

  filePath(): string {
    const f = this.load().sectionFile as GraphNode | undefined;
    return String(f?.path ?? "").replaceAll("\\", "/");
  }

  sectionPath(): string {
    return String(this.load().path ?? "");
  }

  startLine(): number {
    const r = this.load().range as GraphNode | undefined;
    return Number(r?.start ?? 0);
  }

  endLine(): number {
    const r = this.load().range as GraphNode | undefined;
    return Number(r?.end ?? 0);
  }

  content(): string {
    const full = readFileSync(join(this.repoRoot, this.filePath()), "utf8");
    const lines = full.split(/\r?\n/);
    const s = this.startLine();
    const e = this.endLine();
    if (s <= 0 || e < s) return "";
    return lines.slice(s - 1, e).join("\n");
  }

  definitions(): GraphNode[] {
    return (this.load().definitions as GraphNode[]) ?? [];
  }
}

/** 🏷️DefinitionLinter queries a definition node by id. */
export class DefinitionLinter extends BaseLinter {
  private node: GraphNode | undefined;

  private load(): GraphNode {
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "Definition") {
      throw new Error(`[DefinitionLinter] expected Definition, got ${this.node.__typename}`);
    }
    return this.node;
  }

  filePath(): string {
    const f = this.load().defFile as GraphNode | undefined;
    return String(f?.path ?? "").replaceAll("\\", "/");
  }

  name(): string {
    return String(this.load().name ?? "");
  }

  kind(): string {
    return String(this.load().defKind ?? "");
  }

  startLine(): number {
    const r = this.load().range as GraphNode | undefined;
    return Number(r?.start ?? 0);
  }

  endLine(): number {
    const r = this.load().range as GraphNode | undefined;
    return Number(r?.end ?? 0);
  }

  content(): string {
    const full = readFileSync(join(this.repoRoot, this.filePath()), "utf8");
    const lines = full.split(/\r?\n/);
    const s = this.startLine();
    const e = this.endLine();
    if (s <= 0 || e < s) return "";
    return lines.slice(s - 1, e).join("\n");
  }
}

/** 🔎Resolves folder path to folder graphql row (for script.ts policy placement). */
export function resolveFolderByPath(repoRoot: string, folderPath: string): GraphNode {
  const rel = folderPath.replaceAll("\\", "/").replace(/^\/+/, "");
  const data = runCliGraphql(`query F($p: String!) { folder(path: $p) { __typename id path name } }`, { p: rel }, { repoRoot }) as { folder: GraphNode };
  if (!data.folder?.id) throw new Error(`[linter] folder not found for path ${rel}`);
  return data.folder;
}

/** 🔎Resolves bundle name like `repo/client` to bundle id. */
export function resolveBundleByName(repoRoot: string, name: string): GraphNode {
  const data = runCliGraphql(`query B($n: String!) { bundle(name: $n) { __typename id name root kind } }`, { n: name }, { repoRoot }) as { bundle: GraphNode };
  if (!data.bundle?.id) throw new Error(`[linter] bundle not found for name ${name}`);
  return data.bundle;
}

/** 🔎Resolves technology folder name (e.g. `repo`) to technology id. */
export function resolveTechnologyByName(repoRoot: string, name: string): GraphNode {
  const data = runCliGraphql(`query T { technologies { id name root kind } }`, {}, { repoRoot }) as { technologies: GraphNode[] };
  const hit = (data.technologies ?? []).find((t) => String(t.name ?? "") === name);
  if (!hit?.id) throw new Error(`[linter] technology not found for name ${name}`);
  return hit;
}
//#endregion 🔖linter

//#region 🔖script
export type LintFn<T extends BaseLinter> = (linter: T) => BreachRecord[] | Promise<BreachRecord[]>;

/** 📜Tags a lint callback for tooling (runner unwraps default export). */
export function defineLint<T extends BaseLinter>(_tag: string, fn: LintFn<T>): LintFn<T> {
  return fn;
}
//#endregion 🔖script

//#region 🔖dependency-boundary
const ADAPTER_MARKERS = [
  "//#region 🔌adapter",
  "// #region 🔌adapter",
  "# #region 🔌adapter",
  "#region 🔌adapter",
  "//#region 🔌adapters",
  "// #region 🔌adapters",
  "# #region 🔌adapters",
  "//#region 🌐rswasmtransport",
  "// #region 🌐rswasmtransport",
  "pub mod adapters",
  "mod adapters ",
];

const INTERNAL_PREFIXES = ["@compose/", "@ui/", "@cad/", "@puzzle/", "@framework/", "@repo/", "@coda/"];

/** 🔌Returns true when the file path or content marks an adapter boundary. */
export function isAdapterBoundaryFile(filePath: string, content: string): boolean {
  const n = normalize(filePath).replaceAll("\\", "/").toLowerCase();
  if (n.includes("/adapters/") || n.includes("/external_adapters")) return true;
  if (n.includes("-transport.") || n.endsWith(".worker.ts") || n.includes("kit-store.worker")) return true;
  if (n.includes("adapter")) return true;
  const lower = content.toLowerCase();
  return ADAPTER_MARKERS.some((m) => lower.includes(m));
}

/** 🔌Skips generated, test, and non-source paths for dependency-boundary lint. */
export function shouldSkipDependencyBoundaryFile(filePath: string): boolean {
  const n = normalize(filePath).replaceAll("\\", "/");
  if (n.includes("/node_modules/") || n.includes("/.repo/") || n.includes("/dist/") || n.includes("/target/")) {
    return true;
  }
  const base = n.split("/").pop() ?? n;
  if (base.endsWith(".gen.ts")) return true;
  if (base.includes(".test.") || base.endsWith(".spec.ts")) return true;
  return /\.(json|md|ya?ml|lock|svg|png|jpe?g|woff2?)$/i.test(base);
}

function isInternalPackage(name: string, version: string): boolean {
  if (INTERNAL_PREFIXES.some((p) => name.startsWith(p))) return true;
  return version.startsWith("workspace:") || version.startsWith("file:") || version.startsWith("link:");
}

/** 🔌Loads third-party package names from the nearest manifest walking up from filePath. */
export function loadThirdPartyDeps(repoRoot: string, filePath: string): Set<string> {
  const deps = new Set<string>();
  let dir = normalize(dirname(join(repoRoot, filePath))).replaceAll("\\", "/");
  const root = normalize(repoRoot).replaceAll("\\", "/");
  while (dir.startsWith(root) || dir === root) {
    const pkg = join(dir, "package.json");
    if (existsSync(pkg)) {
      mergePackageJson(pkg, deps);
      break;
    }
    const parent = dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  return deps;
}

function mergePackageJson(path: string, deps: Set<string>): void {
  const raw = JSON.parse(readFileSync(path, "utf8")) as Record<string, Record<string, string> | undefined>;
  for (const key of ["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"] as const) {
    const block = raw[key];
    if (!block) continue;
    for (const [name, ver] of Object.entries(block)) {
      if (isInternalPackage(name, ver)) continue;
      deps.add(name);
    }
  }
}

/** 🔌Parses import specifiers from a single import line (TS/JS). */
export function parseTsImportSpecs(line: string): string[] {
  const specs: string[] = [];
  const re = /(?:from|import)\s+['"]([^'"]+)['"]/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(line)) !== null) {
    specs.push(m[1]);
  }
  return specs;
}

function isThirdPartySpec(spec: string, deps: Set<string>): boolean {
  if (!spec || spec.startsWith(".") || spec.startsWith("/")) return false;
  if (deps.has(spec)) return true;
  if (spec.startsWith("@")) {
    const parts = spec.split("/");
    if (parts.length >= 2 && deps.has(`${parts[0]}/${parts[1]}`)) return true;
  }
  for (const dep of deps) {
    if (spec === dep || spec.startsWith(`${dep}/`)) return true;
  }
  return false;
}

/** 🔌Builds breach records for direct third-party imports outside adapter boundaries. */
export function dependencyBoundaryBreachesForFile(repoRoot: string, filePath: string, content: string, scope: string): BreachRecord[] {
  if (shouldSkipDependencyBoundaryFile(filePath)) return [];
  if (isAdapterBoundaryFile(filePath, content)) return [];
  const deps = loadThirdPartyDeps(repoRoot, filePath);
  if (deps.size === 0) return [];
  const breachs: BreachRecord[] = [];
  const lines = content.split(/\r?\n/);
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    if (!line.includes("import ")) continue;
    for (const spec of parseTsImportSpecs(line)) {
      if (!isThirdPartySpec(spec, deps)) continue;
      breachs.push({
        id: `dep-boundary-${spec}-${i + 1}`,
        summary: `Direct import of third-party "${spec}" must live in an adapter module`,
        kind: "dependency-boundary/import/direct-third-party",
        priority: "high",
        reason: "Third-party packages must only be imported in adapter regions or adapter paths",
        solution: "Move the import into a //#region 🔌Adapter (or /adapters/) module and depend on a first-party port interface elsewhere",
        scope,
      });
    }
  }
  return breachs;
}

/** 🔌Aggregates dependency-boundary breaches for every TS/TSX file under a bundle root (repo-relative). */
export function dependencyBoundaryBreachesForBundleDir(repoRoot: string, bundleRootRel: string): BreachRecord[] {
  const rootAbs = join(repoRoot, bundleRootRel);
  if (!existsSync(rootAbs)) return [];
  const breachs: BreachRecord[] = [];
  const walk = (dir: string): void => {
    for (const ent of readdirSync(dir, { withFileTypes: true })) {
      if (ent.name === "node_modules" || ent.name === "dist" || ent.name === ".repo") continue;
      const abs = join(dir, ent.name);
      if (ent.isDirectory()) {
        walk(abs);
        continue;
      }
      if (!/\.tsx?$/i.test(ent.name)) continue;
      const rel = relative(repoRoot, abs).replaceAll("\\", "/");
      breachs.push(...dependencyBoundaryBreachesForFile(repoRoot, rel, readFileSync(abs, "utf8"), rel));
    }
  };
  walk(rootAbs);
  return breachs;
}
//#endregion 🔖dependency-boundary

//#region 🔖policy-runner
export type LintScriptModule = {
  policy?: LintFn<never>;
};

/** 🔎True when `script.ts` exports a repo policy lint callback. */
export function scriptExportsPolicy(scriptPath: string): boolean {
  const text = readFileSync(scriptPath, "utf8");
  return /\bexport\s+(const|function)\s+policy\b/.test(text);
}

function parsePolicyFileExport(scriptPath: string): string | undefined {
  const text = readFileSync(scriptPath, "utf8");
  const m = text.match(/export\s+const\s+policyFile\s*=\s*["']([^"']+)["']/);
  return m?.[1];
}

function fileEntityId(repoRoot: string, fileRel: string): string {
  const rel = relative(repoRoot, fileRel).replaceAll("\\", "/");
  const data = runCliGraphql(`query F($p: String!) { file(path: $p) { id } }`, { p: rel }, { repoRoot }) as {
    file: { id?: string };
  };
  if (!data.file?.id) throw new Error(`[policy-runner] file id not found for ${rel}`);
  return data.file.id;
}

function norm(p: string): string {
  return p.replaceAll("\\", "/").replace(/\/+$/, "");
}

export type ResolvedLintEntity = { kind: "file"; id: string; path: string } | { kind: "folder"; id: string } | { kind: "bundle"; id: string } | { kind: "technology"; id: string };

/** 🔎Maps `script.ts` directory to bundle, technology, or folder entity id. */
export function resolvePolicyScriptEntity(repoRoot: string, scriptPath: string): ResolvedLintEntity {
  const dir = dirname(scriptPath);
  const relDir = norm(relative(repoRoot, dir));
  const folder = runCliGraphql(`query Fo($p: String!) { folder(path: $p) { id path } }`, { p: relDir }, { repoRoot }) as {
    folder: { id?: string; path?: string };
  };
  // 🌱 Workspace root: `folder.id` is `""` (falsy but valid — relative(repoRoot, repoRoot) === "").
  if (folder.folder?.id === undefined) throw new Error(`[policy-runner] folder not resolved for ${relDir}`);

  const meta = runCliGraphql(`query M { bundles { id root name } technologies { id root name } }`, {}, { repoRoot }) as {
    bundles: GraphNode[];
    technologies: GraphNode[];
  };
  const d = norm(relDir);
  for (const b of meta.bundles ?? []) {
    if (norm(String(b.root ?? "")) === d) {
      return { kind: "bundle", id: String(b.id) };
    }
  }
  for (const t of meta.technologies ?? []) {
    // 🌱 Technology `root` echoes the absolute `--repo` path (unlike bundle `root`, which is repo-relative) —
    // the workspace-root script.ts's relDir is "", so match against the absolute repoRoot too.
    const tRoot = norm(String(t.root ?? ""));
    if (tRoot === d || tRoot === norm(repoRoot)) {
      return { kind: "technology", id: String(t.id) };
    }
  }
  return { kind: "folder", id: String(folder.folder.id) };
}

export async function runPolicyScript(
  scriptPath: string,
  repoRoot = getWorkspaceRoot(),
): Promise<{
  entityId: string;
  breachs: BreachRecord[];
  cachePath: string;
}> {
  console.log("[DEBUG] runPolicyScript starting for", scriptPath);
  const absScript = scriptPath.includes(":") || scriptPath.startsWith("/") || /^[A-Za-z]:\\/.test(scriptPath) ? scriptPath : join(repoRoot, scriptPath);
  const base = basename(absScript);
  if (base !== "script.ts") {
    throw new Error(`[policy-runner] expected script.ts, got ${base}`);
  }

  console.log("[DEBUG] runPolicyScript parsing policy file export");
  const policyFile = parsePolicyFileExport(absScript);
  let entity: ResolvedLintEntity;
  if (policyFile) {
    console.log("[DEBUG] runPolicyScript resolving file entity for", policyFile);
    const target = join(dirname(absScript), policyFile).replaceAll("\\", "/");
    entity = { kind: "file", id: fileEntityId(repoRoot, target), path: target };
  } else {
    console.log("[DEBUG] runPolicyScript resolving folder/bundle entity");
    entity = resolvePolicyScriptEntity(repoRoot, absScript);
  }

  console.log("[DEBUG] runPolicyScript importing module dynamically from url", absScript);
  const href = pathToFileURL(absScript).href;
  const mod = (await import(href)) as LintScriptModule;
  console.log("[DEBUG] runPolicyScript imported module successfully");
  const fn = mod.policy;
  if (typeof fn !== "function") {
    throw new Error(`[policy-runner] ${absScript} must export const policy = defineLint(...)`);
  }

  console.log("[DEBUG] runPolicyScript invoking policy function for kind", entity.kind);
  let breachs: BreachRecord[];
  switch (entity.kind) {
    case "file":
      breachs = await fn(new FileLinter(entity.id, repoRoot) as never);
      break;
    case "folder":
      breachs = await fn(new FolderLinter(entity.id, repoRoot) as never);
      break;
    case "bundle":
      breachs = await fn(new BundleLinter(entity.id, repoRoot) as never);
      break;
    case "technology":
      breachs = await fn(new TechnologyLinter(entity.id, repoRoot) as never);
      break;
    default:
      throw new Error("[policy-runner] unreachable");
  }

  const { mkdirSync, writeFileSync } = await import("node:fs");
  const sanitizeCacheKey = (id: string) => id.replace(/[^\w.-]+/g, "_").slice(0, 200);
  const cacheDir = join(repoRoot, ".repo", "cache", "breaches");
  mkdirSync(cacheDir, { recursive: true });
  const cacheName = `${sanitizeCacheKey(entity.id)}.json`;
  const cachePath = join(cacheDir, cacheName);
  const payload = {
    entityId: entity.id,
    script: relative(repoRoot, absScript).replaceAll("\\", "/"),
    breachs,
  };
  writeFileSync(cachePath, JSON.stringify(payload, null, 2), "utf8");
  return { entityId: entity.id, breachs, cachePath };
}

/** 🚪Runs `policy` on this `script.ts` and exits 1 when any high-priority breach exists. */
export async function runPolicyExit(scriptPath: string): Promise<void> {
  const { breachs } = await runPolicyScript(scriptPath);
  if (breachs.some((b) => b.priority === "high")) process.exit(1);
}
//#endregion 🔖policy-runner

//#region 🔖runner
export { resolvePolicyScriptEntity as resolveLintScriptEntity, runPolicyScript as runLintScript };
//#endregion 🔖runner

//#region 🔖policy-cli
/** 🚪When argv contains `policy`, runs this bundle's policy lint and exits. */
export async function dispatchPolicyArgv(segments: string[], scriptUrl: string): Promise<boolean> {
  if (segments[0] !== "policy") return false;
  setTimeout(async () => {
    try {
      await runPolicyExit(fileURLToPath(scriptUrl));
    } catch (e) {
      console.error(e);
      process.exit(1);
    }
  }, 0);
  return true;
}
//#endregion 🔖policy-cli

//#region 🔖bundle-script
//#region 🔖Script
/** 🧭Bundle command; `run` receives argv segments after the subcommand (e.g. `dev mcp` → `["mcp"]`). */
export abstract class Script {
  constructor(
    protected readonly root: string,
    protected readonly repoRoot: string,
  ) {}
  abstract run(segments: string[]): void | Promise<void>;
}

/** 📦Bundle-scoped command with `root` at the package directory. */
export abstract class BundleScript extends Script {
  constructor(bundleRoot: string, repoRoot?: string) {
    super(bundleRoot, repoRoot ?? findRepoRoot(bundleRoot));
  }
}
//#endregion 🔖Script

//#region 🔖Router
export type ScriptCommand = new (root: string, repoRoot: string) => Script;

/** 🧭Declarative subcommand registry for a single `script.ts`. */
export class ScriptRouter {
  private readonly commands = new Map<string, ScriptCommand>();

  constructor(
    readonly bundleRoot: string,
    readonly repoRoot: string = findRepoRoot(bundleRoot),
  ) {}

  /** 📌Registers a subcommand implemented by a `Script` subclass. */
  register(name: string, Command: ScriptCommand): this {
    this.commands.set(name, Command);
    return this;
  }

  /** 📋Human-readable usage line for this router. */
  usage(): string {
    const names = [...this.commands.keys()];
    if (names.length === 0) return "bun ./script.ts policy";
    return `bun ./script.ts <${names.join("|")}> [args…]`;
  }

  /** 📊Whether any subcommands are registered (policy-only bundles may have none). */
  hasCommands(): boolean {
    return this.commands.size > 0;
  }

  /** ▶️Dispatches `segments[0]` to a registered command class. */
  async run(segments: string[]): Promise<void> {
    const name = segments[0];
    if (!name) {
      console.error(`usage: ${this.usage()}`);
      process.exit(1);
    }
    const Command = this.commands.get(name);
    if (!Command) {
      console.error(`unknown command ${JSON.stringify(name)}`);
      console.error(`usage: ${this.usage()}`);
      process.exit(1);
    }
    await Promise.resolve(new Command(this.bundleRoot, this.repoRoot).run(segments.slice(1)));
  }
}

export type RunBundleScriptMainOptions = {
  defaultCommand?: string;
};

/** 🚪Policy-only bundle entry when no other subcommands are registered. */
export async function runPolicyOnlyMain(scriptUrl: string): Promise<void> {
  const segments = process.argv.slice(2);
  if (await dispatchPolicyArgv(segments, scriptUrl)) return;
  console.error("usage: bun ./script.ts policy");
  process.exit(1);
}

/**
 * 🚪Bundle `script.ts` entry: handles optional `policy`, then routes remaining argv through `router`.
 * Export `policy` / `policyFile` from the same file when policy lint applies.
 */
export async function runBundleScriptMain(router: ScriptRouter, scriptUrl: string, opts: RunBundleScriptMainOptions = {}): Promise<void> {
  let segments = process.argv.slice(2);
  if (await dispatchPolicyArgv(segments, scriptUrl)) return;
  if (opts.defaultCommand && segments.length === 0) {
    segments = [opts.defaultCommand];
  }
  if (!router.hasCommands()) {
    console.error(`usage: ${router.usage()}`);
    process.exit(1);
  }
  await router.run(segments);
}

/** 🚪Workspace root `script.ts` entry (no policy dispatch). */
export async function runWorkspaceScriptMain(router: ScriptRouter): Promise<void> {
  await router.run(process.argv.slice(2));
}

/**
 * 🧭Nested subcommand dispatch inside a `Script.run` implementation.
 * `handlers` keys are the first argv segment; `defaultKey` runs when argv is empty.
 */
export function dispatchSubcommand(segments: string[], handlers: Record<string, (rest: string[]) => void | Promise<void>>, usage: string, defaultKey?: string): void | Promise<void> {
  const key = segments[0] ?? defaultKey;
  const handler = key ? handlers[key] : undefined;
  if (!handler) {
    console.error(`usage: ${usage}`);
    process.exit(1);
  }
  return handler(segments.slice(1));
}

/** 📁Walks parents until the monorepo root (`nx.json` + workspace `package.json`). */
export function findRepoRoot(start: string): string {
  let dir = start?.trim() ? start : getWorkspaceRoot();
  for (let i = 0; i < 32; i++) {
    if (existsSync(join(dir, "nx.json")) && existsSync(join(dir, "package.json"))) return dir;
    const parent = dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  return getWorkspaceRoot();
}
//#endregion 🔖Router

//#region ⏱️Budget
/** ⏱️Ordered test levels — every test belongs to exactly one; running level L runs all levels ≤ L, budgeted at L's limit. */
export const TEST_LEVELS = ["fundamental", "quick", "long", "exhaustive"] as const;
export type TestLevel = (typeof TEST_LEVELS)[number];

/** ⏱️Hard wall-clock budget (ms) per test level. */
export const TEST_LEVEL_BUDGET_MS: Record<TestLevel, number> = {
  fundamental: 15_000,
  quick: 30_000,
  long: 300_000,
  exhaustive: 900_000,
};

/** ⏱️Deprecated alias for the fundamental-level budget; kept for straggling call sites during the leveled-test migration. */
export const DEFAULT_TEST_BUDGET_MS = TEST_LEVEL_BUDGET_MS.fundamental;

function isTestLevel(value: string | undefined): value is TestLevel {
  return !!value && (TEST_LEVELS as readonly string[]).includes(value);
}

/** ⏱️Reads the active test level (`SEMIO_TEST_LEVEL`, defaulting to `fundamental`) — set by [[resolveTestLevel]]. */
function activeTestLevel(): TestLevel {
  return isTestLevel(process.env.SEMIO_TEST_LEVEL) ? (process.env.SEMIO_TEST_LEVEL as TestLevel) : "fundamental";
}

/**
 * 🎚️Resolves the test level from `segments[0]` (if it names a level) or `SEMIO_TEST_LEVEL`, else `fundamental`.
 * Sets `process.env.SEMIO_TEST_LEVEL` so every child process spawned afterwards (vitest, cargo, go, pytest,
 * dotnet) inherits it without explicit plumbing. Returns the remaining segments.
 */
export function resolveTestLevel(segments: string[]): { level: TestLevel; rest: string[] } {
  const [first, ...restIfLevel] = segments;
  const level = isTestLevel(first) ? first : activeTestLevel();
  process.env.SEMIO_TEST_LEVEL = level;
  if (level === "exhaustive" && process.env.SEMIO_COVERAGE === undefined) process.env.SEMIO_COVERAGE = "1";
  return { level, rest: isTestLevel(first) ? restIfLevel : segments };
}

/** 🎚️Numeric rank of a test level (0=fundamental..3=exhaustive), for `if (testLevelRank() >= testLevelRank("long"))`-style gating in test files. */
export function testLevelRank(level: string | undefined = process.env.SEMIO_TEST_LEVEL): number {
  const idx = TEST_LEVELS.indexOf((isTestLevel(level) ? level : "fundamental") as TestLevel);
  return idx === -1 ? 0 : idx;
}

function levelsAbove(level: TestLevel): readonly TestLevel[] {
  return TEST_LEVELS.slice(TEST_LEVELS.indexOf(level) + 1);
}

/** ⏱️Cumulative `go test` args for the active level: keeps `-short` through `quick`, adds `-skip` for `Test<Level>`-prefixed tests above it. */
export function goLevelTestArgs(level: TestLevel = activeTestLevel()): string[] {
  const args: string[] = [];
  if (TEST_LEVELS.indexOf(level) <= TEST_LEVELS.indexOf("quick")) args.push("-short");
  const skipped = levelsAbove(level).map((l) => l[0]!.toUpperCase() + l.slice(1));
  if (skipped.length) args.push("-skip", `^Test(${skipped.join("|")})`);
  return args;
}

/** ⏱️Cumulative pytest `-m` marker expression for the active level: excludes markers registered for levels above it. */
export function pytestLevelArgs(level: TestLevel = activeTestLevel()): string[] {
  const excluded = levelsAbove(level);
  return excluded.length ? ["-m", excluded.map((l) => `not ${l}`).join(" and ")] : [];
}

/** ⏱️Cumulative dotnet xunit `--filter` expression for the active level: excludes `Category` traits above it (an absent trait counts as `fundamental`). */
export function dotnetLevelArgs(level: TestLevel = activeTestLevel()): string[] {
  const excluded = levelsAbove(level);
  return excluded.length ? ["--filter", excluded.map((l) => `Category!=${l}`).join("&")] : [];
}

/**
 * ⏱️SIGKILLs `pid`'s whole process tree, not just `pid` — a timed-out `spawnSync`/`execFileSync` only signals the
 * direct child, leaking forked worker pools (e.g. vitest's `workers/forks.js`) that keep burning CPU indefinitely.
 * Requires the child to have been spawned with `detached: true` on POSIX so `pid` is its own process-group leader.
 */
function killBudgetTree(pid: number): void {
  if (process.platform === "win32") {
    spawnSync("taskkill", ["/pid", String(pid), "/T", "/F"], { stdio: "ignore" });
    return;
  }
  try {
    process.kill(-pid, "SIGKILL");
  } catch {
    /* group already gone */
  }
}

/** ⏱️Hard ceiling (ms) for a warm-build step preceding a test run, and for any other cargo build/clippy/check/wasm invocation — compile time isn't billed against the test-level budget, but a stuck build (e.g. shared cargo target-dir lock contention) must never hang a command forever. Overridable via `SEMIO_BUILD_BUDGET_MS`. */
export const BUILD_BUDGET_MS = 1_200_000;

/** ⏱️Resolves the active build-class budget: `SEMIO_BUILD_BUDGET_MS` env override, else [[BUILD_BUDGET_MS]]. */
export function buildBudgetMs(): number {
  return Number(process.env.SEMIO_BUILD_BUDGET_MS ?? BUILD_BUDGET_MS);
}

/** ⏱️Default hard wall-clock budget (ms) for a generic spawned command — the [[runCmd]]/[[runCmdStatus]] default for anything that isn't a `cargo` invocation. Overridable via `SEMIO_CMD_BUDGET_MS`. */
export const CMD_BUDGET_MS = 600_000;

/** ⏱️Resolves the active generic-command budget: `SEMIO_CMD_BUDGET_MS` env override, else [[CMD_BUDGET_MS]]. */
export function cmdBudgetMs(): number {
  return Number(process.env.SEMIO_CMD_BUDGET_MS ?? CMD_BUDGET_MS);
}

/** ⏱️The default budget class for `cmd`: `cargo` invocations (build/clippy/check/install) default to the longer [[buildBudgetMs]] since compiles routinely exceed the generic command budget; everything else defaults to [[cmdBudgetMs]]. */
function defaultBudgetMs(cmd: string): number {
  return cmd === "cargo" ? buildBudgetMs() : cmdBudgetMs();
}

/** ⏱️Timeout hint for a budget-exceeded message; `cargo` commands default to the shared target-dir lock-contention hint (by far the most common real cause), everything else to a generic budget-tuning hint. An explicit `override` always wins. */
export function budgetTimeoutHint(cmd: string, override?: string): string {
  if (override) return override;
  return cmd === "cargo"
    ? "Likely shared cargo target-dir lock contention from another concurrent session — investigate before retrying."
    : "Trim it, or raise its budget (`budgetMs`, `SEMIO_CMD_BUDGET_MS`, `SEMIO_BUILD_BUDGET_MS`).";
}

/**
 * ⏱️Runs a command under a hard wall-clock budget; SIGKILLs the whole process tree and fails loudly past it.
 * Deliberately async: Bun's `spawnSync`/`execFileSync` `detached` option does not put the child in its own
 * process group (verified — only the async `spawn` does), so tree-killing on timeout requires the async form.
 * Callers may fire-and-forget this from a synchronous `void`-returning context — the process stays alive on
 * the pending child/timer handles regardless, and the eventual `process.exit()` below still takes effect.
 */
export async function runTestBudgeted(cmd: string, args: string[], opts: { cwd?: string; env?: NodeJS.ProcessEnv; budgetMs?: number; onTimeoutHint?: string } = {}): Promise<void> {
  const budgetMs = opts.budgetMs ?? Number(process.env.SEMIO_TEST_BUDGET_MS ?? TEST_LEVEL_BUDGET_MS[activeTestLevel()]);
  const child = spawn(cmd, args, { stdio: "inherit", cwd: opts.cwd, env: opts.env ?? process.env, detached: process.platform !== "win32" });
  let timedOut = false;
  const timer = setTimeout(() => {
    timedOut = true;
    if (child.pid) killBudgetTree(child.pid);
  }, budgetMs);
  const { code, signal } = await new Promise<{ code: number | null; signal: NodeJS.Signals | null }>((resolveExit, rejectExit) => {
    child.on("error", rejectExit);
    child.on("exit", (exitCode, exitSignal) => resolveExit({ code: exitCode, signal: exitSignal }));
  }).finally(() => clearTimeout(timer));
  if (timedOut) {
    const hint = opts.onTimeoutHint ?? "Trim it, or assign it to a higher level (quick/long/exhaustive).";
    console.error(`[budget] ${cmd} ${args.join(" ")} exceeded ${budgetMs}ms — killed. ${hint}`);
    process.exit(1);
  }
  if (signal || code !== 0) process.exit(code ?? 1);
}

/**
 * 🦀Warm-builds test binaries — bounded by [[buildBudgetMs]], NOT the test-level budget, but never unbounded —
 * then runs `cargo test` under the active level's budget, appending cumulative `--skip <level>::` filters for every
 * level above it (tests live in `mod quick`/`mod long`/`mod exhaustive` submodules inside `mod tests`; unscoped
 * tests are `fundamental`). Splits `extraArgs` on an existing `--` so callers passing their own libtest args (e.g.
 * `--nocapture`) still compose correctly.
 */
export async function runCargoTestBudgeted(packages: string[], cwd: string, extraArgs: string[] = [], env: NodeJS.ProcessEnv = process.env): Promise<void> {
  const packageArgs = packages.flatMap((pkg) => ["-p", pkg]);
  const dashIdx = extraArgs.indexOf("--");
  const cargoArgs = dashIdx === -1 ? extraArgs : extraArgs.slice(0, dashIdx);
  const libtestArgs = dashIdx === -1 ? [] : extraArgs.slice(dashIdx + 1);
  const level = isTestLevel(env.SEMIO_TEST_LEVEL) ? (env.SEMIO_TEST_LEVEL as TestLevel) : activeTestLevel();
  const skipArgs = levelsAbove(level).flatMap((l) => ["--skip", `${l}::`]);

  if (coverageEnabled()) {
    // 🦀`cargo-llvm-cov` has no build/run split (unlike plain `cargo test --no-run`) — the combined budget
    // below covers both the instrumented compile and the test run; report generation gets its own build-class
    // budget since it only reads existing profraw data and is comparatively fast even for large crates.
    // `--release`: LLVM's source-based coverage (`-C instrument-coverage`) stays accurate under optimization
    // (unlike legacy gcov-style coverage) — building unoptimized (`cargo-llvm-cov`'s default) makes CPU-heavy
    // numeric algorithms (e.g. algebraic-number/root-isolation code) 10-100x slower than their normal
    // `--release` runtime, which blew the test budget for real crates before this flag was added.
    const testBudgetMs = Number(env.SEMIO_TEST_BUDGET_MS ?? TEST_LEVEL_BUDGET_MS[level]);
    await runTestBudgeted("cargo", ["llvm-cov", "test", "--release", "--no-report", ...packageArgs, ...cargoArgs, "--", ...libtestArgs, ...skipArgs], {
      cwd,
      env,
      budgetMs: buildBudgetMs() + testBudgetMs,
      onTimeoutHint: budgetTimeoutHint("cargo"),
    });
    // Slug by package name(s), not `cwd` — many crates' `script.ts` invoke cargo from `this.repoRoot` rather
    // than their own bundle dir (workspace-root builds), which would otherwise collapse dozens of unrelated
    // crates onto the same `coverageSlug(cwd)` filename and silently overwrite each other's coverage report.
    const lcovPath = join(coverageDir(findRepoRoot(cwd), "rust"), `${coverageSlug(packages.join("_"))}.lcov`);
    await runTestBudgeted("cargo", ["llvm-cov", "report", "--release", "--lcov", ...packageArgs, "--output-path", lcovPath], {
      cwd,
      env,
      budgetMs: buildBudgetMs(),
      onTimeoutHint: budgetTimeoutHint("cargo"),
    });
    return;
  }

  await runTestBudgeted("cargo", ["build", "--tests", ...packageArgs], {
    cwd,
    env,
    budgetMs: buildBudgetMs(),
    onTimeoutHint: budgetTimeoutHint("cargo"),
  });
  await runTestBudgeted("cargo", ["test", ...packageArgs, ...cargoArgs, "--", ...libtestArgs, ...skipArgs], { cwd, env });
}

export interface RunCmdOpts {
  cwd?: string;
  env?: NodeJS.ProcessEnv;
  /** ⏱️Wall-clock budget (ms). `null` exempts the command entirely — ONLY for dev servers, interactive
   *  apps, and orchestrators whose own children are individually budgeted. Default: [[defaultBudgetMs]]. */
  budgetMs?: number | null;
  onTimeoutHint?: string;
}

/** ⏱️Shared `spawnSync` core for [[runCmd]]/[[runCmdStatus]]: throws on spawn error, budget timeout, or signal kill (printing `[budget]` first on timeout); otherwise returns the exit status. */
function runCmdInternal(cmd: string, args: string[], opts: RunCmdOpts): number {
  const budgetMs = opts.budgetMs === null ? undefined : (opts.budgetMs ?? defaultBudgetMs(cmd));
  const result = spawnSync(cmd, args, {
    stdio: "inherit",
    cwd: opts.cwd,
    env: opts.env ?? process.env,
    timeout: budgetMs,
    killSignal: "SIGKILL",
  });
  if (result.error) {
    if ((result.error as NodeJS.ErrnoException).code === "ETIMEDOUT") {
      console.error(`[budget] ${cmd} ${args.join(" ")} exceeded ${budgetMs}ms — killed. ${budgetTimeoutHint(cmd, opts.onTimeoutHint)}`);
    }
    throw result.error;
  }
  if (result.signal) throw new Error(`${cmd} ${args.join(" ")} killed by signal ${result.signal}`);
  return result.status ?? 1;
}

/**
 * 🏃Runs a subprocess with inherited stdio under a hard wall-clock budget (default [[defaultBudgetMs]],
 * `null` to exempt); throws on non-zero exit, signal, or budget exceed (the `[budget]` line is printed
 * to stderr first so it survives a caller's try/catch, e.g. [[tryRun]]).
 */
export function runCmd(cmd: string, args: string[], opts: RunCmdOpts = {}): void {
  const status = runCmdInternal(cmd, args, opts);
  if (status !== 0) throw new Error(`${cmd} ${args.join(" ")} exited with status ${status}`);
}

/** 🏃Like [[runCmd]] but returns the exit status instead of throwing on non-zero exit — for call sites
 *  that branch on it. Budget exceed still prints `[budget]` and throws (never silently returns a status). */
export function runCmdStatus(cmd: string, args: string[], opts: RunCmdOpts = {}): number {
  return runCmdInternal(cmd, args, opts);
}

/** 🏃Like [[runCmd]] but ignores failures — including a budget kill, which is the desired never-hang behavior for optional commands. */
export function tryRun(cmd: string, args: string[], opts: RunCmdOpts = {}): void {
  try {
    runCmd(cmd, args, opts);
  } catch {
    /* optional */
  }
}
//#endregion ⏱️Budget

//#region 📊Coverage
/** 📊Whether instrumented coverage collection is on — auto-set by [[resolveTestLevel]] at the `exhaustive` level; `SEMIO_COVERAGE=0` opts out. */
export function coverageEnabled(): boolean {
  return process.env.SEMIO_COVERAGE === "1";
}

export type CoverageKind = "js" | "rust" | "go" | "py" | "dotnet";

/** 📊Per-toolchain lcov output directory under `.repo/coverage`, created on demand. */
export function coverageDir(repoRoot: string, kind: CoverageKind): string {
  const dir = join(repoRoot, ".repo", "coverage", kind);
  mkdirSync(dir, { recursive: true });
  return dir;
}

/** 📊Filesystem-safe unique slug for a project's coverage output filename (mirrors the path-slugging idiom used for nx cache keys). */
export function coverageSlug(bundleRoot: string): string {
  return bundleRoot.replace(/[^a-zA-Z0-9_-]+/g, "_");
}

/**
 * 📊Central, authoritative exclusion list applied at repo-wide aggregation — generated code, vendored/emitted
 * assets, and paths that are GPU-only or Electron-shell and thus unmeasurable by a headless line-coverage
 * runner. Per-tool excludes (vitest `coverage.exclude`, coverage.py `omit`, …) may mirror this for speed, but
 * this list is what decides what counts toward the repo-wide percentage. Every entry has a reason in
 * [[COVERAGE_EXCLUDE_REASONS]] so exclusions stay auditable rather than a silent denominator shrink.
 */
export const COVERAGE_EXCLUDE_GLOBS: readonly string[] = [
  "**/generated/**",
  "asset/metabolism/icon/generated/**",
  "**/pkg/**",
  ".storybook/**",
  "**/*.stories.*",
  "**/*.spec.ts",
  "**/*.test.ts",
  "**/*.tex",
  "**/*.wgsl",
  "**/*.svg",
  "elements/client/lib/geometry/topologic/**",
  "framework/renderer/wgpu/**",
  "ui/wgpu/**",
  "**/dist/**",
  "**/node_modules/**",
  "**/target/**",
];

/** 📊One-line rationale per [[COVERAGE_EXCLUDE_GLOBS]] entry — printed alongside the coverage summary. */
export const COVERAGE_EXCLUDE_REASONS: Readonly<Record<string, string>> = {
  "**/generated/**": "Emitted lookup tables/codegen output, not hand-authored logic.",
  "asset/metabolism/icon/generated/**": "~22k LOC generated icon table.",
  "**/pkg/**": "wasm-bindgen build output.",
  ".storybook/**": "Covered by Playwright specs, not unit line coverage.",
  "**/*.stories.*": "Storybook fixtures, exercised visually not via line coverage.",
  "**/*.spec.ts": "Playwright specs — browser-process coverage is a separate mechanism.",
  "**/*.test.ts": "Standalone test files measure themselves, not the source under test.",
  "**/*.tex": "LaTeX templates — not executable.",
  "**/*.wgsl": "GPU shader source — not measurable by CPU line coverage.",
  "**/*.svg": "Vector assets — not executable.",
  "elements/client/lib/geometry/topologic/**": "Vendored third-party C++.",
  "framework/renderer/wgpu/**": "GPU rendering internals — smoke-testable only; a headless-GPU harness is out of scope here.",
  "ui/wgpu/**": "GPU rendering internals — smoke-testable only; a headless-GPU harness is out of scope here.",
  "**/dist/**": "Build output.",
  "**/node_modules/**": "Third-party dependency code.",
  "**/target/**": "Cargo build output.",
};

function coverageGlobToRegExp(glob: string): RegExp {
  const escaped = glob
    .replace(/[.+^${}()|[\]\\]/g, "\\$&")
    .replace(/\*\*/g, " ")
    .replace(/\*/g, "[^/]*")
    .replace(/ /g, ".*");
  return new RegExp(`^${escaped}$`);
}

/** 📊Whether a repo-relative path (forward-slash, no leading `./`) matches any [[COVERAGE_EXCLUDE_GLOBS]] entry. */
export function isCoverageExcluded(relPath: string): boolean {
  const normalized = relPath.replace(/\\/g, "/").replace(/^\.\//, "");
  return COVERAGE_EXCLUDE_GLOBS.some((glob) => coverageGlobToRegExp(glob).test(normalized));
}

/** 📊`go test` coverage args, appended alongside `goLevelTestArgs` when [[coverageEnabled]]; the text profile is converted to LCOV via [[goProfileToLcov]] at aggregation. */
export function goCoverageArgs(repoRoot: string, moduleLabel: string): string[] {
  if (!coverageEnabled()) return [];
  const file = join(coverageDir(repoRoot, "go"), `${coverageSlug(moduleLabel)}.cover`);
  return ["-covermode=atomic", "-coverpkg=./...", `-coverprofile=${file}`];
}

/** 📊pytest coverage args, appended alongside `pytestLevelArgs` when [[coverageEnabled]]; `pytest-cov`'s `lcov` reporter writes LCOV directly, no conversion needed. */
export function pytestCoverageArgs(repoRoot: string, moduleLabel: string): string[] {
  if (!coverageEnabled()) return [];
  const file = join(coverageDir(repoRoot, "py"), `${coverageSlug(moduleLabel)}.lcov`);
  return ["--cov", `--cov-report=lcov:${file}`];
}

/** 📊dotnet test coverage args (coverlet via `--collect`), appended when [[coverageEnabled]]; each project gets its own results-directory subfolder so concurrent projects don't clobber one another's report file. */
export function dotnetCoverageArgs(repoRoot: string, moduleLabel: string): string[] {
  if (!coverageEnabled()) return [];
  const dir = join(coverageDir(repoRoot, "dotnet"), coverageSlug(moduleLabel));
  mkdirSync(dir, { recursive: true });
  return ["--collect", "XPlat Code Coverage;Format=lcov", "--results-directory", dir];
}

//#region 🗄️Lcov
export type LcovFileRecord = { path: string; lines: Map<number, number> };

/** 📊Parses one LCOV text blob into per-file line-hit maps (only `SF:`/`DA:`/`end_of_record` — the subset every toolchain here emits). */
export function parseLcov(text: string): LcovFileRecord[] {
  const records: LcovFileRecord[] = [];
  let current: LcovFileRecord | undefined;
  for (const rawLine of text.split(/\r?\n/)) {
    const line = rawLine.trim();
    if (line.startsWith("SF:")) {
      current = { path: line.slice(3), lines: new Map() };
      records.push(current);
    } else if (line.startsWith("DA:") && current) {
      const [lineNoStr, hitsStr] = line.slice(3).split(",");
      const lineNo = Number(lineNoStr);
      const hits = Number(hitsStr);
      current.lines.set(lineNo, (current.lines.get(lineNo) ?? 0) + hits);
    } else if (line === "end_of_record") {
      current = undefined;
    }
  }
  return records;
}

/** 📊Merges LCOV records from multiple toolchain runs — line-hit counts sum, covered line numbers union. */
export function mergeLcov(recordSets: LcovFileRecord[][]): Map<string, Map<number, number>> {
  const merged = new Map<string, Map<number, number>>();
  for (const records of recordSets) {
    for (const record of records) {
      const target = merged.get(record.path) ?? new Map<number, number>();
      for (const [lineNo, hits] of record.lines) target.set(lineNo, (target.get(lineNo) ?? 0) + hits);
      merged.set(record.path, target);
    }
  }
  return merged;
}

/** 📊Renders a merged coverage map back to LCOV text (one `SF:`/`DA:`×N/`end_of_record` block per file, lines sorted ascending). */
export function renderLcov(merged: Map<string, Map<number, number>>): string {
  const chunks: string[] = [];
  for (const [path, lines] of merged) {
    chunks.push(`SF:${path}`);
    for (const lineNo of [...lines.keys()].sort((a, b) => a - b)) chunks.push(`DA:${lineNo},${lines.get(lineNo)}`);
    chunks.push("end_of_record", "");
  }
  return chunks.join("\n");
}

/** 📊Expands a `go test -coverprofile` text block (`mode: <mode>` header, then `file:startLine.startCol,endLine.endCol numStmt count` records) into per-line LCOV records — every line in a statement's range is marked hit iff `count > 0`. */
export function goProfileToLcov(text: string): LcovFileRecord[] {
  const byFile = new Map<string, Map<number, number>>();
  for (const rawLine of text.split(/\r?\n/)) {
    const line = rawLine.trim();
    if (!line || line.startsWith("mode:")) continue;
    const match = /^(.+):(\d+)\.\d+,(\d+)\.\d+ \d+ (\d+)$/.exec(line);
    if (!match) continue;
    const [, file, startStr, endStr, countStr] = match;
    const start = Number(startStr);
    const end = Number(endStr);
    const hits = Number(countStr);
    const lines = byFile.get(file) ?? new Map<number, number>();
    for (let lineNo = start; lineNo <= end; lineNo++) lines.set(lineNo, (lines.get(lineNo) ?? 0) + hits);
    byFile.set(file, lines);
  }
  return [...byFile.entries()].map(([path, lines]) => ({ path, lines }));
}

export type CoverageSummary = {
  linesFound: number;
  linesHit: number;
  pct: number;
  perFile: { path: string; linesFound: number; linesHit: number; pct: number }[];
};

/** 📊Reduces a merged coverage map to a repo-wide percentage plus a worst-offenders table, after dropping [[isCoverageExcluded]] paths. */
export function summarizeCoverage(merged: Map<string, Map<number, number>>): CoverageSummary {
  let linesFound = 0;
  let linesHit = 0;
  const perFile: CoverageSummary["perFile"] = [];
  for (const [path, lines] of merged) {
    if (isCoverageExcluded(path)) continue;
    const found = lines.size;
    const hit = [...lines.values()].filter((count) => count > 0).length;
    linesFound += found;
    linesHit += hit;
    perFile.push({ path, linesFound: found, linesHit: hit, pct: found === 0 ? 100 : (hit / found) * 100 });
  }
  perFile.sort((a, b) => a.pct - b.pct || b.linesFound - a.linesFound);
  return { linesFound, linesHit, pct: linesFound === 0 ? 0 : (linesHit / linesFound) * 100, perFile };
}

/** 📊Prints the worst-covered files and hard-fails (`process.exit(1)`) below `thresholdPct`. */
export function enforceCoverageThreshold(summary: CoverageSummary, thresholdPct: number, worstCount = 25): void {
  console.log(`[coverage] ${summary.linesHit}/${summary.linesFound} lines (${summary.pct.toFixed(2)}%), threshold ${thresholdPct}%.`);
  if (summary.pct < thresholdPct) {
    console.log(`[coverage] worst ${Math.min(worstCount, summary.perFile.length)} files:`);
    for (const file of summary.perFile.slice(0, worstCount)) {
      console.log(`  ${file.pct.toFixed(1)}% (${file.linesHit}/${file.linesFound})  ${file.path}`);
    }
    console.error(`[coverage] ${summary.pct.toFixed(2)}% < ${thresholdPct}% — exhaustive gate failed.`);
    process.exit(1);
  }
}
//#endregion 🗄️Lcov
//#endregion 📊Coverage

//#region 🧹CargoLint
/**
 * 🧹Zero-warning gate for a crate: `cargo clippy -p <pkg> --all-targets -- -D warnings`.
 * Deny-on-warnings lives ONLY in this trailing clippy arg — never in RUSTFLAGS, which
 * would replace (not merge with) `.cargo/config.toml`'s rustflags (`-Z threads=8`, the
 * wasm32 `getrandom_backend` cfg, mold) and break every wasm build. See
 * `[workspace.lints]` in the root `Cargo.toml` for the shared lint baseline this checks.
 */
export function runCargoLint(packages: string[], cwd: string, extraArgs: string[] = [], env: NodeJS.ProcessEnv = process.env): void {
  const packageArgs = packages.flatMap((pkg) => ["-p", pkg]);
  runCmd("cargo", ["clippy", ...packageArgs, "--all-targets", ...extraArgs, "--", "-D", "warnings"], { cwd, env, budgetMs: buildBudgetMs() });
}
//#endregion 🧹CargoLint

/** 🧰Dev tooling env without IDE-injected node options. */
export function devToolingEnv(extra: NodeJS.ProcessEnv = {}): NodeJS.ProcessEnv {
  const env = { ...process.env, ...extra };
  delete env.NODE_OPTIONS;
  delete env.VSCODE_INSPECTOR_OPTIONS;
  env.NX_NATIVE_COMMAND_RUNNER ??= "false";
  env.NX_TASKS_RUNNER_DYNAMIC_OUTPUT ??= "false";
  env.NX_TUI ??= "false";
  return env;
}

/** 🥖Runs `bun` with inherited stdio in `cwd`. */
export function runBun(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): void {
  runCmd(process.execPath, args, { cwd, env });
}

/** 🥖Runs `bunx` synchronously in `cwd`. */
export function runBunx(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): void {
  const result = spawnSync(process.execPath, ["x", ...args], { cwd, env, shell: false, stdio: "inherit" });
  if (result.error) {
    console.error(result.error);
    process.exit(1);
  }
  if (result.status !== 0) process.exit(result.status ?? 1);
}

/** 🥖Spawns `bunx` asynchronously; exits with child code. */
export function spawnBunx(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): void {
  const child = spawn(process.execPath, ["x", ...args], { cwd, env, shell: false, stdio: "inherit" });
  child.on("exit", (code) => process.exit(code ?? 0));
  child.on("error", (error) => {
    console.error(error);
    process.exit(1);
  });
}

/** 🥖Spawns `bun` asynchronously; exits with child code. */
export function spawnBun(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): void {
  const child = spawn(process.execPath, args, { cwd, env, shell: true, stdio: "inherit" });
  child.on("exit", (code) => process.exit(code ?? 0));
  child.on("error", (error) => {
    console.error(error);
    process.exit(1);
  });
}

/** ▶️Vite dev server with polling-friendly env defaults. */
export function runViteDev(bundleRoot: string, segments: string[], opts: { config: string; portEnv?: string; defaultPort?: string }): void {
  const env = playPollingEnv();
  const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
  const port = process.env[opts.portEnv ?? "VITE_PORT"] ?? opts.defaultPort ?? "5173";
  spawnBun(["run", "vite", "--config", opts.config, "--host", host, "--port", port, ...segments], bundleRoot, env);
}

/** ▶️Vite production build. */
export function runViteBuild(bundleRoot: string, segments: string[], config: string): void {
  runBun(["run", "vite", "build", "--config", config, ...segments], bundleRoot, devToolingEnv());
}

/**
 * ▶️Vitest run in bundle directory, under the [[runTestBudgeted]] wall-clock budget. Appends v8 lcov coverage
 * flags when [[coverageEnabled]]. Invokes the workspace's own `node_modules/vitest/vitest.mjs` directly rather
 * than `bun x vitest` — `bunx` resolves its own globally cached vitest version, which can silently drift from
 * the workspace's pinned version (observed: a cached 3.x core paired with a locally installed 4.x
 * `@vitest/coverage-v8` crashes the coverage provider on an undefined `reportsDirectory`). Coverage runs use
 * plain `node`, not bun, as the runtime — `@vitest/coverage-v8` drives V8 coverage via `node:inspector`'s
 * `Profiler.startPreciseCoverage`, which Bun's `node:inspector` shim doesn't implement (observed: "Coverage
 * APIs are not supported"); non-coverage runs keep using bun for its faster startup.
 */
export async function runVitest(bundleRoot: string, segments: string[], config = "vitest.config.ts"): Promise<void> {
  const collectingCoverage = coverageEnabled();
  const coverageArgs = collectingCoverage
    ? ["--coverage.enabled", "--coverage.provider=v8", "--coverage.reporter=lcovonly", `--coverage.reportsDirectory=${join(coverageDir(findRepoRoot(bundleRoot), "js"), coverageSlug(bundleRoot))}`]
    : [];
  const vitestBin = join(findRepoRoot(bundleRoot), "node_modules", "vitest", "vitest.mjs");
  const runtime = collectingCoverage ? "node" : process.execPath;
  await runTestBudgeted(runtime, [vitestBin, "run", "--config", config, "--passWithNoTests", ...coverageArgs, ...segments], { cwd: bundleRoot, env: devToolingEnv() });
}

//#region 🔌PlaygroundDevPorts
type PlaygroundPortSpec = {
  readonly dev: number;
  readonly test?: number;
  readonly env: string;
};

/** @emoji 🔌 Builds playground port table from semio.app manifests plus non-app hosts. */
function buildPlaygroundPortsFromManifests(): Record<string, PlaygroundPortSpec> {
  return {
    storybook: { dev: 6010, env: "STORYBOOK_PORT" },
  };
}

let playgroundPortsCache: Record<string, PlaygroundPortSpec> | undefined;

function resolvePlaygroundPorts(): Record<string, PlaygroundPortSpec> {
  playgroundPortsCache ??= buildPlaygroundPortsFromManifests();
  return playgroundPortsCache;
}

export const PLAYGROUND_PORTS: Record<string, PlaygroundPortSpec> = new Proxy({} as Record<string, PlaygroundPortSpec>, {
  get(_target, prop: string) {
    return resolvePlaygroundPorts()[prop];
  },
  ownKeys() {
    return Reflect.ownKeys(resolvePlaygroundPorts());
  },
  getOwnPropertyDescriptor(_target, prop) {
    const value = resolvePlaygroundPorts()[prop as string];
    if (value === undefined) return undefined;
    return { configurable: true, enumerable: true, value };
  },
});

/** @emoji 🔌 Local dev port for a playground host. */
export function playgroundDevPort(kind: PlaygroundHostKind): number {
  const spec = resolvePlaygroundPorts()[kind];
  if (!spec) throw new Error(`unknown playground host kind: ${kind}`);
  return spec.dev;
}

/** @emoji 🔌 String dev port (vite `--port`, nx `env`). */
export function playgroundDevPortString(kind: PlaygroundHostKind): string {
  return String(playgroundDevPort(kind));
}

/** @emoji 🧪 Vitest/playwright port when set; otherwise `undefined`. */
export function playgroundTestPort(kind: PlaygroundHostKind): number | undefined {
  return resolvePlaygroundPorts()[kind]?.test;
}

/** @emoji 🧪 String test port for nx `env` / playwright. */
export function playgroundTestPortString(kind: PlaygroundHostKind): string | undefined {
  const port = playgroundTestPort(kind);
  return port === undefined ? undefined : String(port);
}

/** @emoji 🔌 Process env var holding the dev port override. */
export function playgroundPortEnv(kind: PlaygroundHostKind): string {
  const spec = resolvePlaygroundPorts()[kind];
  if (!spec) throw new Error(`unknown playground host kind: ${kind}`);
  return spec.env;
}

/** @emoji 🚧 Every assigned playground dev + test port (for strict binding). */
export function allPlaygroundReservedPorts(): ReadonlySet<number> {
  const ports = new Set<number>();
  for (const spec of Object.values(resolvePlaygroundPorts())) {
    ports.add(spec.dev);
    if (spec.test !== undefined) ports.add(spec.test);
  }
  return ports;
}

/** @emoji 🔌 OS hub service dev port. */
export const OS_HUB_PORT = 6070;

/** @emoji 🔌 Process env var for {@link OS_HUB_PORT}. */
export const OS_HUB_PORT_ENV = "OS_HUB_PORT";

/** @emoji 🌐 Subset used by iframe static-site embed URLs (derived from manifest `site.embedKind`). */
export type PlaygroundEmbedSiteKind = string;

let playgroundEmbedSiteDevPortsCache: Readonly<Record<string, string>> | undefined;

/** @emoji 🌐 Builds embed-site dev port map from semio.app manifest `site.embedKind`. */
function buildPlaygroundEmbedSiteDevPortsFromManifests(): Readonly<Record<string, string>> {
  return {};
}

/** @emoji 🌐 Resolves iframe embed dev ports after manifest scan is available. */
export function resolvePlaygroundEmbedSiteDevPorts(): Readonly<Record<string, string>> {
  playgroundEmbedSiteDevPortsCache ??= buildPlaygroundEmbedSiteDevPortsFromManifests();
  return playgroundEmbedSiteDevPortsCache;
}

export const PLAYGROUND_EMBED_SITE_DEV_PORTS: Readonly<Record<string, string>> = new Proxy({} as Record<string, string>, {
  get(_target, prop: string) {
    return resolvePlaygroundEmbedSiteDevPorts()[prop];
  },
});

export type PlaygroundSiteKind = string;

/** @emoji 🔒 Process env var locking a playground to one example (hides navbar dropdown). */
export const PLAYGROUND_LOCKED_EXAMPLE_ENV = "PLAYGROUND_LOCKED_EXAMPLE_ID";

/** @emoji 🔒 Locked example id from process env, if any. */
export function playgroundLockedExampleIdFromEnv(env: NodeJS.ProcessEnv = process.env): string | undefined {
  const raw = env[PLAYGROUND_LOCKED_EXAMPLE_ENV]?.trim();
  return raw || undefined;
}

//#region 🔒FrameworkOsLocks
/** @emoji 🔒 Process env vars locking one shell preference to a single boot-time value. */
export const SEMIO_LOCKED_LOCALE_ENV = "SEMIO_LOCKED_LOCALE";
export const SEMIO_LOCKED_TERMINOLOGY_ENV = "SEMIO_LOCKED_TERMINOLOGY";
export const SEMIO_LOCKED_THEME_ENV = "SEMIO_LOCKED_THEME";
export const SEMIO_LOCKED_APPEARANCE_ENV = "SEMIO_LOCKED_APPEARANCE";

/** @emoji 🏷️ Process env var selecting the shell brand a standalone artifact ships as. */
export const SEMIO_BRAND_ENV = "SEMIO_BRAND";

/** @emoji 🎛️ Process env var seeding the boot example without locking it (switcher stays visible). */
export const SEMIO_DEFAULT_EXAMPLE_ENV = "SEMIO_DEFAULT_EXAMPLE";

/**
 * @emoji 🔌 `VITE_`-prefixed env for every set `SEMIO_LOCKED_*`/`SEMIO_BRAND`/`SEMIO_DEFAULT_EXAMPLE`/
 * `PLAYGROUND_LOCKED_EXAMPLE_ID` var, so vite exposes it on `import.meta.env` with no `define` needed.
 * Values are forwarded verbatim — the browser-side `resolveShellLocks`/`resolveShellBrandById` are the
 * single validation authority, so CLI and direct-vite launches behave identically.
 */
export function frameworkOsLockedPrefsEnv(env: NodeJS.ProcessEnv = process.env): NodeJS.ProcessEnv {
  const pairs: [string, string][] = [
    ["VITE_SEMIO_LOCKED_EXAMPLE", PLAYGROUND_LOCKED_EXAMPLE_ENV],
    ["VITE_SEMIO_LOCKED_LOCALE", SEMIO_LOCKED_LOCALE_ENV],
    ["VITE_SEMIO_LOCKED_TERMINOLOGY", SEMIO_LOCKED_TERMINOLOGY_ENV],
    ["VITE_SEMIO_LOCKED_THEME", SEMIO_LOCKED_THEME_ENV],
    ["VITE_SEMIO_LOCKED_APPEARANCE", SEMIO_LOCKED_APPEARANCE_ENV],
    ["VITE_SEMIO_BRAND", SEMIO_BRAND_ENV],
    ["VITE_SEMIO_DEFAULT_EXAMPLE", SEMIO_DEFAULT_EXAMPLE_ENV],
  ];
  const out: NodeJS.ProcessEnv = {};
  for (const [viteKey, sourceKey] of pairs) {
    const raw = env[sourceKey]?.trim();
    if (raw) out[viteKey] = raw;
  }
  return out;
}
//#endregion 🔒FrameworkOsLocks

/** @emoji 🔌 Vite `define` entries for playground play bundles. */
export function playgroundPlayViteDefine(extra: Record<string, string> = {}): Record<string, string> {
  return {
    "import.meta.env.PLAYGROUND_LOCKED_EXAMPLE_ID": JSON.stringify(playgroundLockedExampleIdFromEnv() ?? ""),
    "import.meta.vitest": "undefined",
    ...extra,
  };
}

let playgroundSiteHostsCache: Readonly<Record<string, string>> | undefined;

/** @emoji 🌐 Builds production iframe hostnames from semio.app manifest `site.host`. */
function buildPlaygroundSiteHostsFromManifests(): Readonly<Record<string, string>> {
  return {};
}

function resolvePlaygroundSiteHosts(): Readonly<Record<string, string>> {
  playgroundSiteHostsCache ??= buildPlaygroundSiteHostsFromManifests();
  return playgroundSiteHostsCache;
}

/** @emoji 🌐 Latest-only GitHub Pages hostnames for iframe-embeddable playground static sites. */
export const PLAYGROUND_SITE_HOSTS: Readonly<Record<string, string>> = new Proxy({} as Record<string, string>, {
  get(_target, prop: string) {
    return resolvePlaygroundSiteHosts()[prop];
  },
});

/** @emoji 🔌 Local dev ports for iframe-embeddable playground static sites (from `playground-dev-ports.ts`). */
export const PLAYGROUND_SITE_DEV_PORTS = PLAYGROUND_EMBED_SITE_DEV_PORTS;

/** @emoji 🌐 Playground iframe URL: localhost in dev, canonical host in production builds. */
export function playgroundEmbedUrl(kind: PlaygroundSiteKind, isDev: boolean): string {
  if (isDev) {
    return `http://localhost:${PLAYGROUND_SITE_DEV_PORTS[kind]}`;
  }
  return `https://${PLAYGROUND_SITE_HOSTS[kind]}`;
}
//#endregion 🔌PlaygroundDevPorts

//#region 🖥️FrameworkOsPlaygroundDev
/**
 * 📚 Loads the generated framework OS playground catalog (variant/plugin/aliases/ports rows).
 * Reads `framework/plugin/registry/generated/playgrounds.json` directly (rather than a static
 * TS import of the gitignored generated module) so this shared kernel never fails to load on a
 * fresh clone before `bun nx run @semio-tech/plugin-registry:generate` has ever run — callers get
 * an empty catalog in that case instead of a hard module-resolution error.
 */
export function loadFrameworkOsPlaygroundCatalog(): readonly PlaygroundVariant[] {
  const catalogPath = join(getWorkspaceRoot(), "framework", "plugin", "registry", "generated", "playgrounds.json");
  if (!existsSync(catalogPath)) return [];
  return JSON.parse(readFileSync(catalogPath, "utf8")) as readonly PlaygroundVariant[];
}

/** @emoji 🔌 Local dev-time asset server port for wgpu Trunk/native playgrounds (Trunk forwards
 * route-scoped requests — e.g. `/osm`, `/vt`, `/dem` — here; driven by each playground's declared
 * `[[package.metadata.semio.assets]]` rows, not any one app's routes). */
export const SEMIO_ASSET_SERVER_PORT = 6141;

/** @emoji 🔌 Process env for the absolute asset server URL base (native-bin wgpu route-relative fetches). */
export const SEMIO_ASSET_BASE_URL_ENV = "SEMIO_ASSET_BASE_URL";

/** @emoji 🔌 Resolves the default dev port for a given catalog variant and renderer. */
export function frameworkOsPlaygroundDefaultPort(catalog: readonly PlaygroundVariant[], variant: string, renderer: string): number {
  const row = catalog.find((r) => r.variant === variant);
  if (!row) return 6066;
  return renderer === "wgpu" ? row.ports.wgpu : row.ports.react;
}

/** @emoji 🎯 Resolves `bun ./script.ts dev …` segments to a framework OS plugin filter via the catalog. */
export function resolveFrameworkOsPlaygroundPlugin(catalog: readonly PlaygroundVariant[], segments: readonly string[]): { readonly plugin: string; readonly rest: readonly string[] } | null {
  if (segments.length === 0) return null;
  for (let len = segments.length; len >= 1; len--) {
    const alias = segments.slice(0, len).join(" ");
    const row = catalog.find((r) => r.variant === alias || r.aliases.includes(alias));
    if (row) {
      return { plugin: row.variant, rest: segments.slice(len) };
    }
  }
  return null;
}

/** @emoji 🧊 Env for `@semio-tech/framework-os-dev:dev` with wgpu renderer and plugin filter. */
export function frameworkOsPlaygroundDevEnv(catalog: readonly PlaygroundVariant[], plugin: string, extra: NodeJS.ProcessEnv = {}, env: NodeJS.ProcessEnv = process.env): NodeJS.ProcessEnv {
  const renderer = env.SEMIO_RENDERER ?? "wgpu";
  const defaultPort = frameworkOsPlaygroundDefaultPort(catalog, plugin, renderer);
  const portVal = env.S_OS_PORT || String(defaultPort);
  return devToolingEnv({
    SEMIO_PLUGIN: plugin,
    SEMIO_RENDERER: renderer,
    S_OS_PORT: portVal,
    ...extra,
  });
}
//#endregion 🖥️FrameworkOsPlaygroundDev

/** 🧰Play/vite dev env with optional file-watcher polling defaults. */
export function playPollingEnv(extra: NodeJS.ProcessEnv = {}): NodeJS.ProcessEnv {
  return devToolingEnv({
    ...(process.env.WATCHPACK_POLLING !== undefined ? {} : { WATCHPACK_POLLING: "true", CHOKIDAR_USEPOLLING: "true" }),
    ...extra,
  });
}

/** @emoji 🔒 Parses optional `example <id>` argv prefix for playground play scripts. */
export function consumePlaygroundExampleArgv(segments: string[], resolveExampleId: (slug: string) => string | undefined): { readonly segments: string[]; readonly exampleEnv: NodeJS.ProcessEnv } {
  if (segments[0] !== "example" || !segments[1]) {
    return { segments, exampleEnv: {} };
  }
  const exampleId = resolveExampleId(segments[1]);
  if (!exampleId) {
    console.error(`[play] unknown example ${JSON.stringify(segments[1])}`);
    process.exit(1);
  }
  return {
    segments: segments.slice(2),
    exampleEnv: { PLAYGROUND_LOCKED_EXAMPLE_ID: exampleId },
  };
}

/** ▶️Playwright test run in bundle directory. */
export function runPlaywright(bundleRoot: string, config: string, segments: string[] = []): void {
  runBunx(["playwright", "test", "--config", config, ...segments], bundleRoot, playPollingEnv());
}

/** @emoji 🔌 True when host:port already accepts TCP (existing dev server). */
export function isDevPortInUse(host: string, port: number): boolean {
  const probe = `
import { createConnection } from "node:net";
const socket = createConnection({ host: ${JSON.stringify(host)}, port: ${port} });
socket.setTimeout(300);
socket.once("connect", () => process.exit(0));
socket.once("timeout", () => process.exit(1));
socket.once("error", () => process.exit(1));
`;
  const result = spawnSync(process.execPath, ["--input-type=module", "-e", probe], { timeout: 500 });
  return result.status === 0;
}

/** @emoji 🌐 Loopback URL for a dev server bound to `host`/`port`. */
export function devServerUrl(host: string, port: number): string {
  const probeHost = host === "0.0.0.0" ? "127.0.0.1" : host;
  return `http://${probeHost}:${port}/`;
}

/** @emoji 🧊 Legacy trunk entry paths still seen on long-running dev servers. */
export const WGPU_DEV_LEGACY_ENTRY_PATH = "/renderer-modules/wgpu/";

/** @emoji 🧊 Play URL for a wgpu trunk entry path and plugin filter. */
export function wgpuDevPlayUrl(host: string, port: number, plugin: string, entryPath = "/"): string {
  const probeHost = host === "0.0.0.0" ? "127.0.0.1" : host;
  const base = entryPath.endsWith("/") ? entryPath : `${entryPath}/`;
  return `http://${probeHost}:${port}${base}?plugin=${encodeURIComponent(plugin)}`;
}

/** @emoji 🧊 Probes which wgpu trunk entry path responds on `port`, if any. */
export function probeWgpuDevPort(host: string, port: number): { entryPath: string } | null {
  const probeHost = host === "0.0.0.0" ? "127.0.0.1" : host;
  for (const entryPath of ["/", WGPU_DEV_LEGACY_ENTRY_PATH] as const) {
    const url = `http://${probeHost}:${port}${entryPath}`;
    const probe = `const res = await fetch(${JSON.stringify(url)}, { signal: AbortSignal.timeout(2000) });
process.exit(res.ok ? 0 : 1);`;
    const result = spawnSync(process.execPath, ["--input-type=module", "-e", probe], { timeout: 3000 });
    if (result.status === 0) return { entryPath };
  }
  return null;
}

/** @emoji 🛑 Stops a trunk listener on `port` when it is the sole occupant. */
export function stopTrunkDevPort(port: number): boolean {
  const occupant = describeDevPortOccupant(port);
  if (!occupant?.startsWith("trunk")) return false;
  const pid = Number(occupant.match(/PID (\d+)/)?.[1]);
  if (!Number.isFinite(pid)) return false;
  try {
    process.kill(pid, "SIGTERM");
    return true;
  } catch {
    return false;
  }
}

/** @emoji 🎯 Reads `import.meta.env.PLAYGROUND_APP_KIND` baked into a running playground dev server. */
export function devServerPlayEntry(host: string, port: number): string | undefined {
  const url = `${devServerUrl(host, port)}index.ts`;
  const probe = `const res = await fetch(${JSON.stringify(url)}, { signal: AbortSignal.timeout(2000) });
if (!res.ok) process.exit(1);
const text = await res.text();
const match = text.match(/PLAYGROUND_APP_KIND\\":\\s*\\"([^\\"]+)\\"/);
process.stdout.write(match?.[1] ?? "");
`;
  const result = spawnSync(process.execPath, ["--input-type=module", "-e", probe], { encoding: "utf8", timeout: 3000 });
  const entry = result.stdout?.trim();
  return entry || undefined;
}

/** @emoji 🔎 Best-effort description of the process listening on `port` (Unix/macOS/Linux). */
export function describeDevPortOccupant(port: number): string | undefined {
  if (process.platform === "win32") return undefined;
  const result = spawnSync("lsof", ["-nP", `-iTCP:${port}`, "-sTCP:LISTEN"], { encoding: "utf8" });
  const line = result.stdout
    ?.trim()
    .split("\n")
    .find((row, index) => index > 0 && row.trim().length > 0);
  if (!line) return undefined;
  const parts = line.trim().split(/\s+/);
  return parts.length >= 2 ? `${parts[0]} (PID ${parts[1]})` : line.trim();
}

/** @emoji ♻️ True when an HTTP server on `port` already responds successfully (reuse existing dev). */
export function canReuseDevPort(host: string, port: number, expectedPlayEntry?: string): boolean {
  const url = devServerUrl(host, port);
  const probe = `const res = await fetch(${JSON.stringify(url)}, { signal: AbortSignal.timeout(2000) });
process.exit(res.ok ? 0 : 1);`;
  const result = spawnSync(process.execPath, ["--input-type=module", "-e", probe], { timeout: 3000 });
  if (result.status !== 0) return false;
  if (!expectedPlayEntry) return true;
  return devServerPlayEntry(host, port) === expectedPlayEntry;
}

/** @emoji 🔌 First free TCP port at or after `preferredPort` (up to `maxAttempts`), skipping `skipPorts`. */
export function resolveDevPort(host: string, preferredPort: number, maxAttempts = 20, skipPorts: ReadonlySet<number> = new Set()): number {
  for (let offset = 0; offset < maxAttempts; offset++) {
    const port = preferredPort + offset;
    if (skipPorts.has(port)) continue;
    if (!isDevPortInUse(host, port)) return port;
  }
  console.error(`[dev] No free port found in range ${preferredPort}-${preferredPort + maxAttempts - 1}.`);
  process.exit(1);
}

/** ▶️Vite dev via `bunx` with root-level `vite.config.ts`. */
export function runViteBunxDev(
  bundleRoot: string,
  segments: string[],
  opts: {
    portEnv?: string;
    defaultPort?: string;
    clearViteCache?: boolean;
    strictPort?: boolean;
    /** When true, bind only `defaultPort` / env — never bump into another playground port. */
    fixedPort?: boolean;
    reservedPorts?: ReadonlySet<number>;
    env?: NodeJS.ProcessEnv;
    /** When set with `fixedPort`, only reuse an existing listener serving this play entry. */
    expectedPlayEntry?: string;
  } = {},
): void {
  const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
  const preferredPort = Number(process.env[opts.portEnv ?? "VITE_PORT"] ?? opts.defaultPort ?? "5173");
  const spawnEnv = playPollingEnv(opts.env);
  if (opts.fixedPort && isDevPortInUse(host, preferredPort)) {
    const url = devServerUrl(host, preferredPort);
    if (canReuseDevPort(host, preferredPort, opts.expectedPlayEntry)) {
      console.log(`[dev] Port ${preferredPort} is already in use — dev server appears to be running at ${url}`);
      return;
    }
    const occupant = describeDevPortOccupant(preferredPort);
    const servedEntry = devServerPlayEntry(host, preferredPort);
    if (servedEntry && opts.expectedPlayEntry && servedEntry !== opts.expectedPlayEntry) {
      console.error(`[dev] Port ${preferredPort} is serving play entry "${servedEntry}" but "${opts.expectedPlayEntry}" was requested. Stop that process or set ${opts.portEnv ?? "VITE_PORT"}.`);
      process.exit(1);
    }
    console.error(`[dev] Port ${preferredPort} is already in use${occupant ? ` by ${occupant}` : ""}. Stop that process or set ${opts.portEnv ?? "VITE_PORT"}.`);
    process.exit(1);
  }
  const port = (() => {
    if (opts.fixedPort) {
      return preferredPort;
    }
    const skip = opts.reservedPorts ?? new Set<number>();
    const resolved = resolveDevPort(host, preferredPort, 20, skip);
    if (resolved !== preferredPort) {
      console.warn(`[dev] Port ${preferredPort} is already in use — starting on ${resolved} instead.`);
      if (opts.portEnv) process.env[opts.portEnv] = String(resolved);
    }
    return resolved;
  })();
  if (opts.clearViteCache) {
    const viteCache = join(bundleRoot, "node_modules", ".vite");
    if (existsSync(viteCache)) rmSync(viteCache, { recursive: true, force: true });
  }
  const wantStrictPort = opts.strictPort ?? true;
  const viteArgs = ["vite", "--config", "vite.config.ts", "--host", host, "--port", String(port)];
  if (wantStrictPort && !segments.includes("--strictPort") && !segments.includes("--no-strictPort")) {
    viteArgs.push("--strictPort");
  }
  spawnBunx([...viteArgs, ...segments], bundleRoot, spawnEnv);
}

/** ▶️Vite dev via `bunx` without a fixed config path (extra args only). */
export function runViteBunxDevPlain(bundleRoot: string, segments: string[]): void {
  const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
  spawnBunx(["vite", "--host", host, ...segments], bundleRoot, playPollingEnv());
}

/** 🦀Runs `cargo` with inherited stdio. */
export function runCargo(args: string[], cwd: string, env: NodeJS.ProcessEnv = process.env): void {
  runCmd("cargo", args, { cwd, env });
}

export type WasmPackWebPkg = {
  name: string;
  version?: string;
  files: string[];
  main: string;
  module: string;
  types: string;
  sideEffects?: string[];
};

/** 📦Resolve wasm-bindgen CLI installed by wasm-pack. */
function resolveWasmBindgenBin(): string {
  const cacheRoot = join(process.env.HOME ?? "", "Library/Caches/.wasm-pack");
  if (existsSync(cacheRoot)) {
    const entries = readdirSync(cacheRoot)
      .filter((name) => name.startsWith("wasm-bindgen-cargo-install-"))
      .map((name) => join(cacheRoot, name, "wasm-bindgen"))
      .filter((path) => existsSync(path));
    if (entries.length > 0) return entries[entries.length - 1]!;
  }
  return "wasm-bindgen";
}

/** 📦Collect wasm-bindgen snippet paths produced by threaded builds. */
function wasmPackSnippetFiles(pkgDir: string): string[] {
  const snippetsDir = join(pkgDir, "snippets");
  if (!existsSync(snippetsDir)) return [];
  const out: string[] = [];
  const walk = (dir: string, prefix: string) => {
    for (const entry of readdirSync(dir)) {
      const rel = prefix ? `${prefix}/${entry}` : entry;
      const abs = join(dir, entry);
      if (statSync(abs).isDirectory()) {
        walk(abs, rel);
      } else {
        out.push(`snippets/${rel}`);
      }
    }
  };
  walk(snippetsDir, "");
  return out;
}

/** 📦Collects Rust sources beneath a crate without descending into generated output. */
function rustSourceInputs(root: string): string[] {
  const out: string[] = [];
  const walk = (dir: string) => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      if (entry.isDirectory() && !["target", "pkg", ".git"].includes(entry.name)) walk(join(dir, entry.name));
      else if (entry.isFile() && entry.name.endsWith(".rs")) out.push(join(dir, entry.name));
    }
  };
  walk(root);
  return out;
}

/** 📦Collects sources and manifests from transitive `[dependencies]` path crates. */
function wasmPackPathDependencyInputs(rsDir: string, visited = new Set<string>()): string[] {
  const cargoToml = join(rsDir, "Cargo.toml");
  const root = resolve(rsDir);
  if (!existsSync(cargoToml) || visited.has(root)) return [];
  visited.add(root);
  const out: string[] = [];
  for (const m of readFileSync(cargoToml, "utf8").matchAll(/path\s*=\s*"([^"]+)"/gu)) {
    const depRoot = resolve(rsDir, m[1]!);
    const depCargo = join(depRoot, "Cargo.toml");
    if (!existsSync(depCargo)) continue;
    out.push(depCargo, ...rustSourceInputs(depRoot), ...wasmPackPathDependencyInputs(depRoot, visited));
  }
  return [...new Set(out)];
}

/** 📦True when any wasm-pack input is newer than the built `.wasm` artifact. */
function wasmPackInputsStale(rsDir: string, wasmPath: string): boolean {
  if (!existsSync(wasmPath)) return true;
  const wasmMtime = statSync(wasmPath).mtimeMs;
  const repoRoot = getWorkspaceRoot();
  const inputs = [...rustSourceInputs(rsDir), join(rsDir, "Cargo.toml"), join(rsDir, "Cargo.lock"), join(repoRoot, "Cargo.toml"), join(repoRoot, "Cargo.lock"), ...wasmPackPathDependencyInputs(rsDir)];
  for (const input of inputs) {
    if (existsSync(input) && statSync(input).mtimeMs > wasmMtime) return true;
  }
  return false;
}

/** 📦`wasm-pack build` for `--target web`, restores `pkg/package.json`, verifies wasm output. */
export function runWasmPackWebBuild(opts: {
  rsDir: string;
  skipEnvVar: string;
  logPrefix: string;
  pkg: WasmPackWebPkg;
  wasmBaseName: string;
  /** When true, build with atomics + `-Z build-std` for wasm-bindgen-rayon thread pools. */
  threads?: boolean;
  /** Optional Cargo feature flags passed to wasm-pack / cargo build. */
  cargoFeatures?: readonly string[];
}): void {
  const { rsDir, skipEnvVar, logPrefix, pkg, wasmBaseName, threads = false, cargoFeatures = [] } = opts;
  const pkgDir = join(rsDir, "pkg");
  const wasmPath = join(pkgDir, `${wasmBaseName}_bg.wasm`);
  if (process.env[skipEnvVar] === "1") {
    console.log(`[${logPrefix}] ${skipEnvVar}=1 → skipping wasm-pack build`);
    return;
  }
  if (!wasmPackInputsStale(rsDir, wasmPath)) {
    console.log(`[${logPrefix}] pkg/${wasmBaseName}_bg.wasm up to date → skipping wasm-pack build`);
    if (!existsSync(pkgDir)) mkdirSync(pkgDir, { recursive: true });
    const snippetFiles = wasmPackSnippetFiles(pkgDir);
    const pkgJson = {
      type: "module",
      version: pkg.version ?? "0.1.0",
      sideEffects: pkg.sideEffects ?? ["./snippets/*"],
      ...pkg,
      files: [...new Set([...pkg.files, ...snippetFiles])],
    };
    writeFileSync(join(pkgDir, "package.json"), `${JSON.stringify(pkgJson, null, 2)}\n`, "utf8");
    return;
  }
  const buildLabel = threads ? "cargo build (threaded) + wasm-bindgen" : "wasm-pack build";
  console.log(`[${logPrefix}] ${buildLabel} --release --target web --out-dir pkg --no-pack`);
  const t0 = Date.now();
  let status: number;
  if (threads) {
    const repoRoot = getWorkspaceRoot();
    const crateName = readFileSync(join(rsDir, "Cargo.toml"), "utf8").match(/^name\s*=\s*"([^"]+)"/m)?.[1];
    if (!crateName) {
      console.error(`[${logPrefix}] missing package name in Cargo.toml`);
      process.exit(1);
    }
    const cargoWasm = join(repoRoot, "target/wasm32-unknown-unknown/release", `${crateName.replace(/-/g, "_")}.wasm`);
    const threadedCargoArgs = ["build", "--release", "--target", "wasm32-unknown-unknown", "-Z", "build-std=std,panic_abort", ...cargoFeatures.flatMap((feature) => ["--features", feature])];
    status = runCmdStatus("cargo", threadedCargoArgs, { cwd: rsDir, env: { ...process.env }, budgetMs: buildBudgetMs() });
    if (status !== 0) {
      console.error(`[${logPrefix}] cargo threaded build failed`);
      process.exit(status);
    }
    if (!existsSync(pkgDir)) mkdirSync(pkgDir, { recursive: true });
    status = runCmdStatus(resolveWasmBindgenBin(), [cargoWasm, "--out-dir", "pkg", "--typescript", "--target", "web", "--out-name", wasmBaseName], { cwd: rsDir, env: { ...process.env }, budgetMs: buildBudgetMs() });
  } else {
    const wasmPackArgs = ["x", "wasm-pack", "build", "--release", "--target", "web", "--out-dir", "pkg", "--no-pack", ...cargoFeatures.flatMap((feature) => ["--", "--features", feature])];
    status = runCmdStatus("bun", wasmPackArgs, { cwd: rsDir, env: { ...process.env }, budgetMs: buildBudgetMs() });
  }
  if (status !== 0) {
    console.error(`[${logPrefix}] wasm build failed`);
    process.exit(status);
  }
  console.log(`[${logPrefix}] wasm build done in ${((Date.now() - t0) / 1000).toFixed(1)}s`);

  if (!existsSync(pkgDir)) mkdirSync(pkgDir, { recursive: true });
  const snippetFiles = wasmPackSnippetFiles(pkgDir);
  const pkgJson = {
    type: "module",
    version: pkg.version ?? "0.1.0",
    sideEffects: pkg.sideEffects ?? ["./snippets/*"],
    ...pkg,
    files: [...new Set([...pkg.files, ...snippetFiles])],
  };
  writeFileSync(join(pkgDir, "package.json"), `${JSON.stringify(pkgJson, null, 2)}\n`, "utf8");

  if (existsSync(wasmPath)) {
    const sz = (statSync(wasmPath).size / (1024 * 1024)).toFixed(2);
    console.log(`[${logPrefix}] pkg/${wasmBaseName}_bg.wasm ready (${sz} MiB) + pkg/package.json restored`);
  } else {
    console.error(`[${logPrefix}] expected wasm output missing: ${wasmPath}`);
    process.exit(1);
  }
}

/** 🔗Resolves `import.meta.url` of the bundle `script.ts`. */
export function scriptPathFromUrl(scriptUrl: string): string {
  return fileURLToPath(scriptUrl);
}
//#endregion 🔖Process
//#endregion 🔖bundle-script

//#region 🔖uloc-metrics

//#region Types
export type MicroCommitLangMetrics = {
  lang: string;
  emoji: string;
  code: number;
  edited: number;
  added: number;
  removed: number;
};

export type UlocByLanguage = Record<string, number>;

/** 📊Repo-wide unified LOC per language (tracked git files; JSON uses key count). */
export type UlocRunner = {
  countRepoByLanguage(root: string): UlocByLanguage;
};
//#endregion Types

//#region Constants
const METRICS_LOCK_FILES = new Set(["package-lock.json", "yarn.lock", "pnpm-lock.yaml", "go.sum", "uv.lock", "bun.lockb", "cargo.lock"]);

const METRICS_LICENSE_TEMPLATE_BASENAMES = new Set(["LICENSE", "LICENSE.md", "LICENSE.txt", "COPYING", "COPYING.md", "NOTICE", "NOTICE.md", "UNLICENSE", "UNLICENSE.md"]);

const LANG_EMOJI: Record<string, string> = {
  TypeScript: "🟦",
  JavaScript: "🟨",
  Go: "🔵",
  "C#": "🟣",
  Python: "🐍",
  Rust: "🦀",
  Shell: "🐚",
  Dockerfile: "🐳",
  Makefile: "🔧",
  CSS: "🎨",
  SQL: "🛢️",
  HTML: "🌐",
  Markdown: "📝",
  TeX: "📐",
  JSON: "🧾",
  YAML: "📋",
  TOML: "⚙️",
  XML: "📄",
  CSV: "📑",
  "Bourne Shell": "🐚",
  "Bourne Again Shell": "🐚",
  PowerShell: "💠",
  Docker: "🐳",
};

const ULOC_EXCLUDE_DIRS = [".repo", "node_modules", "dist", "build", "target", ".git", ".nx", "coverage", ".cache", ".turbo", ".next", "out", "vendor", "third_party", "Carthage"];

const MAX_METRICS_FILE_BYTES = 8 * 1024 * 1024;
//#endregion Constants

//#region Path rules
function normalizeRepoPath(path: string): string {
  return path.replace(/\\/g, "/").replace(/^\.\//, "");
}

function metricsPathBasename(rel: string): string {
  return rel.slice(rel.lastIndexOf("/") + 1);
}

function hasHiddenDotPathSegment(rel: string): boolean {
  for (const seg of rel.split("/")) {
    if (!seg || seg === "." || seg === "..") continue;
    if (seg.startsWith(".")) return true;
  }
  return false;
}

function isMetricsLockOrGenerated(rel: string): boolean {
  const base = metricsPathBasename(rel);
  if (METRICS_LOCK_FILES.has(base)) return true;
  if (base.endsWith(".generated.go") || base.endsWith(".pb.go")) return true;
  return false;
}

function isMetricsLicenseTemplateFile(rel: string): boolean {
  const base = metricsPathBasename(rel);
  if (METRICS_LICENSE_TEMPLATE_BASENAMES.has(base)) return true;
  if (base.startsWith("LICENSE.")) return true;
  return false;
}

/** 🗂️Whether paths must be excluded from uloc/metrics (dot paths, license templates, vendor, lockfiles). */
export function shouldSkipPathForUloc(root: string, relPath: string): boolean {
  const rel = normalizeRepoPath(relPath);
  if (!rel || rel === ".repo" || rel.startsWith(".repo/")) return true;
  if (hasHiddenDotPathSegment(rel)) return true;
  if (isMetricsLicenseTemplateFile(rel)) return true;
  if (isMetricsLockOrGenerated(rel)) return true;
  const ignored = spawnSync("git", ["check-ignore", "-q", "--", rel], { cwd: gitRepoRoot(root) });
  if (ignored.status === 0) return true;
  for (const dir of ULOC_EXCLUDE_DIRS) {
    if (rel === dir || rel.startsWith(`${dir}/`)) return true;
  }
  return false;
}

/** 🏷️Maps a repo path to a metrics language bucket (code langs + JSON/YAML/… formats). */
export function classifyPathForMetrics(path: string): string {
  const rel = normalizeRepoPath(path);
  const base = rel.slice(rel.lastIndexOf("/") + 1).toLowerCase();
  if (base === "dockerfile" || base.startsWith("dockerfile.")) return "Dockerfile";
  if (base === "makefile" || base === "justfile") return "Makefile";
  const ext = rel.slice(rel.lastIndexOf(".")).toLowerCase();
  switch (ext) {
    case ".ts":
    case ".tsx":
    case ".cts":
    case ".mts":
    case ".mtsx":
      return "TypeScript";
    case ".js":
    case ".mjs":
    case ".cjs":
      return "JavaScript";
    case ".go":
      return "Go";
    case ".cs":
      return "C#";
    case ".py":
      return "Python";
    case ".rs":
      return "Rust";
    case ".sh":
    case ".bash":
    case ".zsh":
      return "Shell";
    case ".ps1":
      return "PowerShell";
    case ".css":
    case ".scss":
    case ".sass":
      return "CSS";
    case ".sql":
      return "SQL";
    case ".html":
    case ".htm":
    case ".xhtml":
      return "HTML";
    case ".md":
    case ".markdown":
    case ".mdown":
    case ".mkd":
    case ".mdx":
    case ".mdc":
    case ".svx":
      return "Markdown";
    case ".tex":
    case ".sty":
    case ".cls":
    case ".ltx":
    case ".bib":
      return "TeX";
    case ".json":
    case ".jsonc":
      return "JSON";
    case ".yaml":
    case ".yml":
      return "YAML";
    case ".toml":
      return "TOML";
    case ".csv":
      return "CSV";
    case ".xml":
      return "XML";
    default:
      return "";
  }
}

/** 🎨Emoji for a metrics language row. */
export function langMetricsEmoji(lang: string): string {
  return LANG_EMOJI[lang] ?? "📎";
}

/** 🌳Resolves the git worktree root (never a subdirectory of the monorepo). */
export function gitRepoRoot(start: string): string {
  const r = spawnSync("git", ["rev-parse", "--show-toplevel"], { cwd: start, encoding: "utf8" });
  if (r.status === 0) {
    const top = (r.stdout ?? "").trim();
    if (top) return top;
  }
  return start;
}

function gitTrackedPaths(root: string): string[] {
  const repoRoot = gitRepoRoot(root);
  const r = spawnSync("git", ["ls-files", "-z"], { cwd: repoRoot, maxBuffer: 256 * 1024 * 1024 });
  if (r.status !== 0) return [];
  return (r.stdout ?? Buffer.alloc(0)).toString("utf8").split("\0").filter(Boolean);
}
//#endregion Path rules

//#region Uloc counting
function countJsonKeysValue(v: unknown): number {
  if (v === null || typeof v !== "object") return 0;
  if (Array.isArray(v)) {
    let n = 0;
    for (const item of v) n += countJsonKeysValue(item);
    return n;
  }
  const o = v as Record<string, unknown>;
  let n = Object.keys(o).length;
  for (const vv of Object.values(o)) n += countJsonKeysValue(vv);
  return n;
}

/** 🧮Counts JSON object keys recursively (aligned with repo `loc` Data JSON rules). */
export function countJsonKeys(text: string): number {
  const t = text.trim();
  if (!t) return 0;
  try {
    return countJsonKeysValue(JSON.parse(t));
  } catch {
    return 0;
  }
}

function physicalLineCount(text: string): number {
  if (!text.length) return 0;
  return text.split(/\r?\n/).length;
}

/** 📏LOC for one tracked file body (JSON key count when applicable). */
export function countUnifiedLocForFile(rel: string, data: string): number {
  const ext = rel.slice(rel.lastIndexOf(".")).toLowerCase();
  if (ext === ".json" || ext === ".jsonc") {
    const keys = countJsonKeys(data);
    if (keys > 0) return keys;
    if (!data.trim()) return 0;
    return physicalLineCount(data);
  }
  return physicalLineCount(data);
}

function gitDir(root: string): string {
  const repoRoot = gitRepoRoot(root);
  const r = spawnSync("git", ["rev-parse", "--git-dir"], { cwd: repoRoot, encoding: "utf8" });
  if (r.status !== 0) return join(repoRoot, ".git");
  const dir = (r.stdout ?? "").trim();
  return dir.startsWith("/") ? dir : join(repoRoot, dir);
}

const ULOC_CACHE_VERSION = 4;

type UlocCacheFile = {
  version: number;
  head: string;
  trackedFiles: number;
  totalLoc: number;
  langCount: number;
  counts: UlocByLanguage;
};

function ulocCachePath(root: string): string {
  return join(gitDir(root), "compose-uloc-cache.json");
}

function ulocCacheStats(counts: UlocByLanguage): { totalLoc: number; langCount: number } {
  let totalLoc = 0;
  let langCount = 0;
  for (const n of Object.values(counts)) {
    if (n > 0) {
      totalLoc += n;
      langCount++;
    }
  }
  return { totalLoc, langCount };
}

/** 🧪Whether cached uloc looks like a complete repo scan (rejects partial/stale caches). */
export function isUlocCachePlausible(root: string, counts: UlocByLanguage): boolean {
  const { totalLoc, langCount } = ulocCacheStats(counts);
  if (totalLoc <= 0 || langCount === 0) return false;
  const tracked = gitTrackedPaths(root).length;
  if (tracked === 0) return true;
  if (tracked < 100) return totalLoc > 0 && (langCount >= 2 || totalLoc >= 50);
  if (langCount < 6 && tracked >= 300) return false;
  if (totalLoc < tracked * 3) return false;
  return true;
}

function readUlocCache(root: string): UlocByLanguage | null {
  const head = spawnSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" });
  if (head.status !== 0) return null;
  const h = (head.stdout ?? "").trim();
  if (!h) return null;
  try {
    const raw = readFileSync(ulocCachePath(root), "utf8");
    const parsed = JSON.parse(raw) as Partial<UlocCacheFile>;
    if (parsed.version !== ULOC_CACHE_VERSION) return null;
    if (parsed.head !== h || !parsed.counts || typeof parsed.counts !== "object") return null;
    const tracked = gitTrackedPaths(root).length;
    if (typeof parsed.trackedFiles === "number" && Math.abs(tracked - parsed.trackedFiles) > 50) return null;
    if (!isUlocCachePlausible(root, parsed.counts)) return null;
    const stats = ulocCacheStats(parsed.counts);
    if (typeof parsed.totalLoc === "number" && parsed.totalLoc !== stats.totalLoc) return null;
    if (typeof parsed.langCount === "number" && parsed.langCount !== stats.langCount) return null;
    return parsed.counts;
  } catch {
    return null;
  }
}

function writeUlocCache(root: string, counts: UlocByLanguage): void {
  const head = spawnSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" });
  if (head.status !== 0) return;
  const h = (head.stdout ?? "").trim();
  if (!h) return;
  const { totalLoc, langCount } = ulocCacheStats(counts);
  const payload: UlocCacheFile = {
    version: ULOC_CACHE_VERSION,
    head: h,
    trackedFiles: gitTrackedPaths(root).length,
    totalLoc,
    langCount,
    counts,
  };
  writeFileSync(ulocCachePath(root), JSON.stringify(payload));
}

/** 📊Scans all git-tracked paths and sums unified LOC per language bucket. */
export function scanRepoUnifiedLocUncached(root: string): UlocByLanguage {
  const repoRoot = gitRepoRoot(root);
  const out: UlocByLanguage = {};
  for (const rel of gitTrackedPaths(root)) {
    if (shouldSkipPathForUloc(repoRoot, rel)) continue;
    const lang = classifyPathForMetrics(rel);
    if (!lang) continue;
    const fp = join(repoRoot, rel);
    if (!existsSync(fp)) continue;
    try {
      const st = statSync(fp);
      if (!st.isFile() || st.size > MAX_METRICS_FILE_BYTES) continue;
    } catch {
      continue;
    }
    let data: string;
    try {
      const buf = readFileSync(fp);
      if (buf.includes(0)) continue;
      data = buf.toString("utf8");
    } catch {
      continue;
    }
    const loc = countUnifiedLocForFile(rel, data);
    if (loc <= 0) continue;
    out[lang] = (out[lang] ?? 0) + loc;
  }
  return out;
}

/** 📊Repo uloc with per-HEAD cache under `.git/compose-uloc-cache.json`. */
export function scanRepoUnifiedLoc(root: string): UlocByLanguage {
  const cached = readUlocCache(root);
  if (cached) return cached;
  const counts = scanRepoUnifiedLocUncached(root);
  writeUlocCache(root, counts);
  return counts;
}

/** 📊Default uloc runner (tracked-file unified scan). */
export function createDefaultUlocRunner(): UlocRunner {
  return { countRepoByLanguage: scanRepoUnifiedLoc };
}
//#endregion Uloc counting

//#region Git deltas
/** ✂️Splits git numstat into replaced (edited), net added, and net removed lines. */
export function splitGitNumstatDelta(added: number, removed: number): { edited: number; added: number; removed: number } {
  const a = Math.max(0, added);
  const r = Math.max(0, removed);
  const edited = Math.min(a, r);
  return { edited, added: a - edited, removed: r - edited };
}

function parseGitNumstatZ(stdout: Buffer | string): { path: string; added: number; removed: number }[] {
  const raw = typeof stdout === "string" ? stdout : stdout.toString("utf8");
  if (!raw) return [];
  const out: { path: string; added: number; removed: number }[] = [];
  for (const entry of raw.split("\0")) {
    if (!entry) continue;
    const parts = entry.split("\t");
    if (parts.length < 3) continue;
    const added = parts[0] === "-" ? 0 : Number(parts[0]) || 0;
    const removed = parts[1] === "-" ? 0 : Number(parts[1]) || 0;
    out.push({ path: parts.slice(2).join("\t"), added, removed });
  }
  return out;
}

function gitCachedNumstat(root: string): { path: string; added: number; removed: number }[] {
  const r = spawnSync("git", ["diff", "--cached", "--numstat", "-z"], { cwd: gitRepoRoot(root) });
  if (r.status !== 0) return [];
  return parseGitNumstatZ(r.stdout ?? Buffer.alloc(0));
}

/** 📂Whether a repo-relative path lies under any normalized prefix. */
export function pathUnderPrefixes(rel: string, prefixes: string[]): boolean {
  if (prefixes.length === 0) return true;
  const n = normalizeRepoPath(rel);
  for (const raw of prefixes) {
    const p = normalizeRepoPath(raw).replace(/\/$/, "");
    if (!p) continue;
    if (n === p || n.startsWith(`${p}/`)) return true;
  }
  return false;
}

/** 📈Git numstat between two revisions. */
export function gitRangeNumstat(root: string, base: string, head: string): { path: string; added: number; removed: number }[] {
  const repoRoot = gitRepoRoot(root);
  const r = spawnSync("git", ["diff", "--numstat", "-z", `${base}..${head}`], {
    cwd: repoRoot,
    maxBuffer: 256 * 1024 * 1024,
  });
  if (r.status !== 0) return [];
  return parseGitNumstatZ(r.stdout ?? Buffer.alloc(0));
}

/** ➕Accumulates per-language git deltas from numstat rows (optional path prefixes). */
export function accumulateGitDeltasFromNumstat(root: string, rows: { path: string; added: number; removed: number }[], pathPrefixes?: string[]): Map<string, { added: number; removed: number; edited: number }> {
  const m = new Map<string, { added: number; removed: number; edited: number }>();
  for (const { path, added, removed } of rows) {
    if (shouldSkipPathForUloc(root, path)) continue;
    if (pathPrefixes && !pathUnderPrefixes(path, pathPrefixes)) continue;
    const lang = classifyPathForMetrics(path);
    if (!lang) continue;
    const d = splitGitNumstatDelta(added, removed);
    const row = m.get(lang) ?? { added: 0, removed: 0, edited: 0 };
    row.edited += d.edited;
    row.added += d.added;
    row.removed += d.removed;
    m.set(lang, row);
  }
  return m;
}

function accumulateGitDeltas(root: string): Map<string, { added: number; removed: number; edited: number }> {
  return accumulateGitDeltasFromNumstat(root, gitCachedNumstat(root));
}

/** ➕Sums git delta counters across all languages. */
export function sumGitLangDeltas(deltas: Map<string, { added: number; removed: number; edited: number }>): {
  added: number;
  removed: number;
  edited: number;
} {
  let added = 0;
  let removed = 0;
  let edited = 0;
  for (const d of deltas.values()) {
    added += d.added;
    removed += d.removed;
    edited += d.edited;
  }
  return { added, removed, edited };
}

/** 🟰Sum of net added, edited (replaced), and removed line counts from git numstat. */
export function gitDeltaLineTotal(d: { added: number; removed: number; edited: number }): number {
  return d.added + d.edited + d.removed;
}

/** 📊Appends `➕` `✏️` `➖` and total `🟰` (sum of the three) when non-zero. */
export function appendGitDeltaSuffix(line: string, d: { added: number; removed: number; edited: number }): string {
  if (d.added > 0) line += `➕${d.added}`;
  if (d.edited > 0) line += `✏️${d.edited}`;
  if (d.removed > 0) line += `➖${d.removed}`;
  const sum = gitDeltaLineTotal(d);
  if (sum > 0) line += `🟰${sum}`;
  return line;
}

/** 📊Compact `📊uloc➕…✏️…➖…🟰…` suffix from git deltas (bundle header). */
export function formatBundleUlocSuffix(d: { added: number; removed: number; edited: number }): string {
  return appendGitDeltaSuffix("📊uloc", d);
}
//#endregion Git deltas

//#region Format
/** 🔢Formats uncommented LOC counts (e.g. 200000 → 200k). */
export function formatMetricLocCount(n: number): string {
  const v = Math.max(0, Math.round(n));
  if (v >= 1000) return `${Math.round(v / 1000)}k`;
  return String(v);
}

function sortMetricLanguages(codeByLang: UlocByLanguage, deltas: Map<string, { edited: number }>): string[] {
  const langs = new Set<string>();
  for (const [lang, n] of Object.entries(codeByLang)) {
    if (n > 0) langs.add(lang);
  }
  for (const lang of deltas.keys()) langs.add(lang);
  return [...langs].sort((a, b) => (codeByLang[b] ?? 0) - (codeByLang[a] ?? 0) || a.localeCompare(b));
}

function buildMicroCommitMetricsFromDeltas(codeByLang: UlocByLanguage, deltas: Map<string, { added: number; removed: number; edited: number }>): MicroCommitLangMetrics[] {
  const rows: MicroCommitLangMetrics[] = [];
  for (const lang of sortMetricLanguages(codeByLang, deltas)) {
    const d = deltas.get(lang) ?? { added: 0, removed: 0, edited: 0 };
    const code = codeByLang[lang] ?? 0;
    if (code === 0 && d.edited === 0 && d.added === 0 && d.removed === 0) continue;
    rows.push({
      lang,
      emoji: langMetricsEmoji(lang),
      code,
      edited: d.edited,
      added: d.added,
      removed: d.removed,
    });
  }
  return rows;
}

/** 📊Builds micro-commit metrics: repo uloc per language + staged git deltas. */
export function buildMicroCommitMetrics(root: string, ulocRunner: UlocRunner = createDefaultUlocRunner()): MicroCommitLangMetrics[] {
  const repoRoot = gitRepoRoot(root);
  const deltas = accumulateGitDeltas(repoRoot);
  const codeByLang = ulocRunner.countRepoByLanguage(repoRoot);
  return buildMicroCommitMetricsFromDeltas(codeByLang, deltas);
}

/** 📊Builds uloc metrics for a git revision range (optional path prefixes). */
export function buildMicroCommitMetricsForRange(root: string, base: string, head = "HEAD", pathPrefixes?: string[], ulocRunner: UlocRunner = createDefaultUlocRunner()): MicroCommitLangMetrics[] {
  const repoRoot = gitRepoRoot(root);
  const deltas = accumulateGitDeltasFromNumstat(repoRoot, gitRangeNumstat(repoRoot, base, head), pathPrefixes);
  const codeByLang = ulocRunner.countRepoByLanguage(repoRoot);
  return buildMicroCommitMetricsFromDeltas(codeByLang, deltas);
}

/** 📊Micro-commit metrics block header (unified repo LOC). */
export const MICRO_COMMIT_ULOC_HEADER = "📊uloc";

/** 🔢Emoji for the aggregate total row (first line after the header). */
export const MICRO_COMMIT_ULOC_TOTAL_EMOJI = "🔢";

/** 📊Sums per-language uloc rows into one total. */
export function sumMicroCommitLangMetrics(metrics: MicroCommitLangMetrics[]): MicroCommitLangMetrics {
  const total: MicroCommitLangMetrics = {
    lang: "Total",
    emoji: MICRO_COMMIT_ULOC_TOTAL_EMOJI,
    code: 0,
    edited: 0,
    added: 0,
    removed: 0,
  };
  for (const m of metrics) {
    total.code += m.code;
    total.edited += m.edited;
    total.added += m.added;
    total.removed += m.removed;
  }
  return total;
}

/** 📊Formats one uloc row; omits zero ➕✏️➖; appends 🟰 sum when any delta is non-zero. */
export function formatMicroCommitMetricLine(m: MicroCommitLangMetrics): string {
  return appendGitDeltaSuffix(`${m.emoji}${formatMetricLocCount(m.code)}`, m);
}

/** 📊Renders the unified LOC block: `📊uloc➕…✏️…➖…🟰…` total, then per-language rows. */
export function formatMicroCommitMetricsLines(metrics: MicroCommitLangMetrics[]): string[] {
  if (metrics.length === 0) return [];
  const total = sumMicroCommitLangMetrics(metrics);
  const header = formatBundleUlocSuffix({ added: total.added, edited: total.edited, removed: total.removed });
  return [header, ...metrics.map(formatMicroCommitMetricLine)];
}

/** ✅Whether two git delta sums match on ➕ ✏️ ➖ (🟰 follows). */
export function gitDeltaSumsEqual(a: { added: number; removed: number; edited: number }, b: { added: number; removed: number; edited: number }): boolean {
  return a.added === b.added && a.edited === b.edited && a.removed === b.removed;
}

/** 🚫Per-language ➕✏️➖ must sum to the footer `📊uloc` total line. */
export function validateMicroCommitLangMetricsDeltaSum(metrics: MicroCommitLangMetrics[]): void {
  if (metrics.length === 0) return;
  const total = sumMicroCommitLangMetrics(metrics);
  let added = 0;
  let edited = 0;
  let removed = 0;
  for (const m of metrics) {
    added += m.added;
    edited += m.edited;
    removed += m.removed;
  }
  if (!gitDeltaSumsEqual({ added, edited, removed }, total)) {
    throw new Error(
      `commit: per-language 📊uloc deltas do not sum to the footer total — languages ➕${added}✏️${edited}➖${removed}🟰${gitDeltaLineTotal({ added, edited, removed })} vs footer ➕${total.added}✏️${total.edited}➖${total.removed}🟰${gitDeltaLineTotal(total)}`,
    );
  }
}
//#endregion Format

//#endregion 🔖uloc-metrics

//#region 🔖micro-commit

export type MicroCommitLevel = "prepare-only" | "prepare-and-commit" | "prepare-and-commit-and-push";

type Contributor = { alias: string; emoji: string; name: string; email: string; emails?: string[] };

const COUNTER_RE = /^(.+🎆\d{2}🌙\d{2}☀️\d{2})🚩(\d+)$/;
const BUNDLE_TAG_RE = /^(.+🎆\d{2}🌙\d{2}☀️\d{2})🚩$/;
const NUMERIC_COUNTER_RE = /^(\d+)$/;
const TICKET_JSON_RE = /^\.repo\/🎫\/.+\/ticket\.json$/;
export function digestMicroCommitMessage(message: string): string {
  return createHash("sha256").update(message.replace(/\r\n/g, "\n").trimEnd()).digest("hex");
}

function preparedDigestPath(root: string): string {
  return join(gitDir(root), "compose-micro-commit-digest");
}

function preparedActivePath(root: string): string {
  return join(gitDir(root), "compose-micro-commit-active");
}

function markPrepareActive(root: string): void {
  writeFileSync(preparedActivePath(root), "1\n");
}

function isPrepareActive(root: string): boolean {
  return existsSync(preparedActivePath(root));
}

const GK_TEMPLATE_BASENAME = "gkcommittemplate";
const GK_COMMIT_TEMPLATE_FILE = `${GK_TEMPLATE_BASENAME}.txt`;

const MICRO_COMMIT_POST_WIPE_HOOKS = ["post-commit", "post-checkout", "post-merge", "post-rewrite"] as const;

function git(root: string, args: string[]): { ok: boolean; out: string } {
  const r = spawnSync("git", args, { cwd: root, encoding: "utf8" });
  if (r.status !== 0) return { ok: false, out: (r.stderr ?? r.stdout ?? "").trim() };
  return { ok: true, out: (r.stdout ?? "").trim() };
}

function gitCachedNames(root: string, extra: string[] = []): string[] {
  const r = spawnSync("git", ["diff", "--cached", "--name-only", "-z", ...extra], { cwd: root });
  if (r.status !== 0) return [];
  const raw = (r.stdout ?? Buffer.alloc(0)).toString("utf8");
  if (!raw) return [];
  return raw.split("\0").filter(Boolean);
}

function branchAllowed(root: string): boolean {
  const b = git(root, ["branch", "--show-current"]).out;
  return b.includes("⛳wip") || b.includes("🏗️dev");
}

function gitEmail(root: string): string {
  return git(root, ["config", "user.email"]).out;
}

function findContributor(root: string): Contributor | null {
  const email = gitEmail(root).toLowerCase();
  if (!email) return null;
  const dir = join(root, ".repo", "🧑‍💻");
  if (!existsSync(dir)) return null;
  for (const name of readdirSync(dir, { withFileTypes: true })) {
    if (!name.isDirectory()) continue;
    const path = join(dir, name.name, "contributor.json");
    if (!existsSync(path)) continue;
    const c = JSON.parse(readFileSync(path, "utf8")) as Contributor & { emails?: string[] };
    const emails = [c.email, ...(c.emails ?? [])].filter((e): e is string => typeof e === "string" && e.length > 0).map((e) => e.toLowerCase());
    if (emails.includes(email)) return c;
  }
  return null;
}

function loadLevel(root: string, contributor: Contributor, segments: string[]): MicroCommitLevel {
  const token = segments.join(" ").toLowerCase();
  if (/\b(gp|gpush|push!|\+push)\b/.test(token)) return "prepare-and-commit-and-push";
  if (/\b(gc|commit!|\+commit)\b/.test(token)) return "prepare-and-commit";
  if (/\b(g\.|gprepare|prepare!|\+prepare)\b/.test(token)) return "prepare-only";
  const path = join(root, ".repo", "🧑‍💻", contributor.alias, "micro-commit.json");
  if (existsSync(path)) {
    const j = JSON.parse(readFileSync(path, "utf8")) as { level?: string };
    if (j.level === "prepare-and-commit" || j.level === "prepare-and-commit-and-push" || j.level === "prepare-only") {
      return j.level;
    }
  }
  return "prepare-only";
}

function pad2(n: number): string {
  return String(n).padStart(2, "0");
}

function pad3(n: number): string {
  return String(n).padStart(3, "0");
}

const COUNTER_LOG_DEPTH = 40;

/** 🔢Reads micro-commit counter from subject line `…🚩NNN`. */
export function extractCounterFromSubject(subject: string): { nnn: number; line1Base: string } | null {
  const s = subject.trim();
  const formatted = COUNTER_RE.exec(s);
  if (!formatted) return null;
  return { nnn: Number.parseInt(formatted[2], 10), line1Base: formatted[1] };
}

/** 🔢Reads GitKraken numeric-only subjects (`152`, `299`, …). */
export function extractNumericCounterFromSubject(subject: string): number | null {
  const m = NUMERIC_COUNTER_RE.exec(subject.trim());
  if (!m) return null;
  const n = Number.parseInt(m[1]!, 10);
  return Number.isFinite(n) && n > 0 ? n : null;
}

/** 🏷️Reads WIP epoch base from a bundle squash tag (`…🎆YY🌙MM☀️DD🚩`). */
export function line1BaseFromBundleTag(tag: string): string | null {
  const m = BUNDLE_TAG_RE.exec(tag.trim());
  return m ? m[1]! : null;
}

/** 🎆Resolves the persisted WIP epoch for line 1 from formatted history or the latest bundle tag. */
export function contributorWipLine1Base(root: string, contributor: Contributor): string | null {
  const prefix = `${contributor.emoji}${contributor.alias}`;
  const log = git(root, ["log", "--format=%s", "-1000"]).out;
  for (const subject of log ? log.split("\n") : []) {
    const hit = extractCounterFromSubject(subject);
    if (hit?.line1Base.startsWith(prefix)) return hit.line1Base;
  }
  const tags = git(root, ["tag", "-l", `${prefix}*`, "--sort=-creatordate"]).out;
  for (const tag of tags ? tags.split("\n") : []) {
    const base = line1BaseFromBundleTag(tag);
    if (base?.startsWith(prefix)) return base;
  }
  return null;
}

/** 🎆Bumps counter from recent `…🚩NNN` or numeric GitKraken subjects (newest first). */
export function bumpCounterFromHistory(
  subjectsNewestFirst: string[],
  contributor: Contributor,
  now = new Date(),
  wipLine1Base: string | null = null,
): { line1Base: string; nnn: string } {
  const yy = pad2(now.getFullYear() % 100);
  const mm = pad2(now.getMonth() + 1);
  const dd = pad2(now.getDate());
  const fresh = `${contributor.emoji}${contributor.alias}🎆${yy}🌙${mm}☀️${dd}`;
  let max = 0;
  let line1Base: string | null = null;
  for (const subject of subjectsNewestFirst) {
    const hit = extractCounterFromSubject(subject);
    if (hit) {
      max = Math.max(max, hit.nnn);
      if (!line1Base) line1Base = hit.line1Base;
      continue;
    }
    const numeric = extractNumericCounterFromSubject(subject);
    if (numeric !== null) max = Math.max(max, numeric);
  }
  const epoch = line1Base ?? wipLine1Base ?? fresh;
  if (max > 0) return { line1Base: epoch, nnn: pad3(max + 1) };
  return { line1Base: epoch, nnn: "001" };
}

export function bumpCounterFromSubject(subject: string, contributor: Contributor, now = new Date()): { line1Base: string; nnn: string } {
  return bumpCounterFromHistory([subject], contributor, now);
}

function nextCounter(root: string, contributor: Contributor): { line1Base: string; nnn: string } {
  const log = git(root, ["log", "--format=%s", `-${COUNTER_LOG_DEPTH}`]).out;
  const subjects = log ? log.split("\n").filter(Boolean) : [];
  return bumpCounterFromHistory(subjects, contributor, new Date(), contributorWipLine1Base(root, contributor));
}

function formatSecond(now: Date): string {
  const yy = pad2(now.getFullYear() % 100);
  const mm = pad2(now.getMonth() + 1);
  const dd = pad2(now.getDate());
  const hh = pad2(now.getHours());
  const min = pad2(now.getMinutes());
  const ss = pad2(now.getSeconds());
  return `🎆${yy}🌙${mm}☀️${dd}⏰${hh}⌚${min}⏱️${ss}`;
}

function preparedBulletsPath(root: string): string {
  return join(gitDir(root), "compose-micro-commit-bullets");
}

const EMOJI_LEAD_RE = /^((?:\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*)+)/u;
const MICRO_COMMIT_BULLET_RE = /^(?:\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*)+\S/u;
const TIMESTAMP_LINE_RE = /^🎆\d{2}🌙\d{2}☀️\d{2}/u;
const RESERVED_BULLET_LEAD_EMOJIS = new Set(["🎆", "📊", "🔢", "🚩"]);

/** 🏷️Leading emoji grapheme on a bullet line, or "" if none. */
export function bulletLeadEmoji(line: string): string {
  const m = EMOJI_LEAD_RE.exec(line.trim());
  return m?.[1] ?? "";
}

/** 📝Formats one bullet as `{emoji}{description}` (line starts with emoji, no leading `-`). */
export function formatMicroCommitBulletLine(line: string): string {
  let body = line.trim().replace(/^-+\s*/, "");
  return body.replace(new RegExp(`^${EMOJI_LEAD_RE.source}\\s+`, "u"), "$1");
}

const MICRO_COMMIT_ULOC_ROW_RE = /^[\p{Extended_Pictographic}][\dk]+(?:➕\d+)?(?:✏️\d+)?(?:➖\d+)?(?:🟰\d+)?$/u;

function isMicroCommitUlocLine(line: string): boolean {
  const t = line.trim();
  if (t.startsWith(MICRO_COMMIT_ULOC_HEADER)) return true;
  return MICRO_COMMIT_ULOC_ROW_RE.test(t);
}

/** 📝Normalizes LLM-authored bullet lines to `{emoji}{description}`. */
export function normalizeBulletLines(text: string): string[] {
  return text
    .split("\n")
    .map((l) => l.trim())
    .filter((l) => l.length > 0 && !l.startsWith("#") && !isMicroCommitUlocLine(l))
    .map(formatMicroCommitBulletLine)
    .filter((l) => l.length > 1)
    .slice(0, 8);
}

function validateBulletSpacing(bullets: string[]): void {
  for (const b of bullets) {
    if (MICRO_COMMIT_BULLET_RE.test(b)) continue;
    console.error(`micro-commit: bullet must start with {emoji} then description (no '-' prefix, no space after emoji): ${b}`);
    process.exit(1);
  }
}

/** 🚫Returns an error when a bullet uses reserved or timestamp emojis. */
export function bulletEmojiValidationError(bullets: string[]): string | null {
  for (const b of bullets) {
    const lead = bulletLeadEmoji(b);
    if (RESERVED_BULLET_LEAD_EMOJIS.has(lead)) {
      return `micro-commit: ${lead} is reserved for subject/timestamp/uloc — start each bullet with the emoji that best matches that line's description`;
    }
    if (TIMESTAMP_LINE_RE.test(b.trim())) {
      return "micro-commit: bullet must not copy the 🎆YY🌙MM☀️DD timestamp pattern — use one leading emoji that fits the change, not the calendar line";
    }
  }
  return null;
}

function validateBulletEmojis(bullets: string[]): void {
  const err = bulletEmojiValidationError(bullets);
  if (err) {
    console.error(err);
    process.exit(1);
  }
}

function writePreparedBullets(root: string, bullets: string[]): void {
  writeFileSync(preparedBulletsPath(root), `${bullets.join("\n")}\n`);
}

function readPreparedBullets(root: string): string[] {
  const path = preparedBulletsPath(root);
  if (!existsSync(path)) return [];
  return normalizeBulletLines(readFileSync(path, "utf8"));
}

const GIT_COMMIT_DRAFT_FILES = ["COMMIT_EDITMSG", "MERGE_MSG", "SQUASH_MSG"] as const;

const STAGED_CHANGE_AREAS = [
  { id: ".cursor/plans", match: (p: string) => p.startsWith(".cursor/plans/"), keywords: ["plan"] },
  { id: ".agents", match: (p: string) => p.startsWith(".agents/") && !p.endsWith("SKILL.md"), keywords: ["skill", "agent"] },
  { id: "repo", match: (p: string) => p.startsWith("repo/"), keywords: ["hook", "micro-commit"] },
  { id: ".devcontainer", match: (p: string) => p.startsWith(".devcontainer/"), keywords: ["devcontainer"] },
  {
    id: "product",
    match: (p: string) => /^(framework|puzzle|compose|cad|ui|mathematical|infinite|elements|coda|reuse)\//.test(p),
    keywords: [],
  },
] as const;

function isInsignificantStagedPath(path: string): boolean {
  return /\/micro-commit\.ts$/.test(path) || /\/index\.test\.ts$/.test(path) || path.endsWith("SKILL.md");
}

/** 🔤Path tokens used to check whether bullets mention a staged file. */
export function pathTokensForBulletCoverage(filePath: string): string[] {
  return [
    ...new Set(
      filePath
        .toLowerCase()
        .split(/[/._-]+/)
        .filter((s) => s.length >= 4),
    ),
  ];
}

function bulletsMentionPathTokens(text: string, paths: string[]): boolean {
  const tokens = paths.flatMap(pathTokensForBulletCoverage);
  return tokens.some((t) => text.includes(t));
}

function bulletsCoverArea(text: string, paths: string[], keywords: readonly string[]): boolean {
  if (keywords.some((k) => text.includes(k))) return true;
  return bulletsMentionPathTokens(text, paths);
}

/** 🧪Returns staged area ids not reflected in bullets (empty = ok). */
export function uncoveredStagedAreas(bullets: string[], staged: string[]): string[] {
  const significant = staged.filter((p) => !isInsignificantStagedPath(p));
  if (significant.length === 0) return [];
  const text = bullets.join("\n").toLowerCase();
  const missed: string[] = [];
  const matched = new Set<string>();
  for (const area of STAGED_CHANGE_AREAS) {
    const files = significant.filter((p) => {
      if (!area.match(p)) return false;
      matched.add(p);
      return true;
    });
    if (files.length === 0) continue;
    if (!bulletsCoverArea(text, files, area.keywords)) missed.push(area.id);
  }
  const other = significant.filter((p) => !matched.has(p));
  if (other.length > 0 && !bulletsMentionPathTokens(text, other)) missed.push("other staged paths");
  return missed;
}

function validateBulletsAgainstStaged(bullets: string[], staged: string[]): void {
  const missed = uncoveredStagedAreas(bullets, staged);
  if (missed.length === 0) return;
  console.error(`micro-commit: bullets must cover every staged area — missing: ${missed.join(", ")}`);
  console.error("micro-commit: read `micro-commit diff` again (include .cursor/plans, product code, repo, …)");
  for (const p of staged) console.error(`  ${p}`);
  process.exit(1);
}

function readDiffBulletsInput(root: string, bulletsFile: string | null): string[] {
  if (bulletsFile) {
    const path = bulletsFile.startsWith("/") ? bulletsFile : join(root, bulletsFile);
    return normalizeBulletLines(readFileSync(path, "utf8"));
  }
  if (!process.stdin.isTTY) {
    return normalizeBulletLines(readFileSync(0, "utf8"));
  }
  return [];
}

function listCachedPaths(root: string): string[] {
  return gitCachedNames(root);
}

function listAddedTicketPaths(root: string): string[] {
  return gitCachedNames(root, ["--diff-filter=A"]).filter((p) => TICKET_JSON_RE.test(p));
}

function ticketBullets(root: string): string[] {
  const bullets: string[] = [];
  for (const rel of listAddedTicketPaths(root)) {
    const path = join(root, rel);
    const t = JSON.parse(readFileSync(path, "utf8")) as { emoji?: string; title?: string };
    if (!t.emoji || !t.title) continue;
    bullets.push(`${t.emoji}${t.title}`);
  }
  return bullets;
}

export function buildMicroCommitMessage(root: string, contributor: Contributor, diffBullets: string[] = [], ulocRunner?: UlocRunner): string {
  root = gitRepoRoot(root);
  const { line1Base, nnn } = nextCounter(root, contributor);
  const now = new Date();
  const authored = diffBullets.length > 0 ? normalizeBulletLines(diffBullets.join("\n")) : readPreparedBullets(root);
  const tickets = ticketBullets(root);
  const authoredLower = new Set(authored.map((b) => b.toLowerCase()));
  const bullets = [...authored, ...tickets.filter((t) => !authoredLower.has(t.toLowerCase()))].slice(0, 8);
  if (bullets.length === 0) {
    throw new Error("micro-commit: at least one description bullet is required");
  }
  const metrics = formatMicroCommitMetricsLines(buildMicroCommitMetrics(root, ulocRunner));
  const lines = [`${line1Base}🚩${nnn}`, formatSecond(now), ...bullets];
  if (metrics.length > 0) lines.push("", ...metrics);
  lines.push("", `Signed-off-by: ${contributor.name} <${contributor.email}>`);
  return `${lines.join("\n")}\n`;
}

export function writeMicroCommitTemplates(root: string, message: string): void {
  const dir = gitDir(root);
  const gkCommitTemplate = join(dir, GK_COMMIT_TEMPLATE_FILE);
  removeGitKrakenTemplateFiles(root);
  writeFileSync(gkCommitTemplate, message);
  for (const name of GIT_COMMIT_DRAFT_FILES) {
    writeFileSync(join(dir, name), message);
  }
  git(root, ["config", "--local", "commit.template", gkCommitTemplate]);
  writeFileSync(preparedDigestPath(root), `${digestMicroCommitMessage(message)}\n`);
  markPrepareActive(root);
}

export function shouldRefreshPreparedCommitMessage(current: string, preparedDigest: string | null): boolean {
  const trimmed = current.trim();
  if (!trimmed) return true;
  if (!preparedDigest) return false;
  return digestMicroCommitMessage(current) === preparedDigest.trim();
}

function removeGitDirPrefixed(root: string, prefix: string): void {
  const dir = gitDir(root);
  for (const name of readdirSync(dir)) {
    if (name.startsWith(prefix)) {
      try {
        rmSync(join(dir, name), { force: true });
      } catch {
        /* ignore */
      }
    }
  }
}

function removeGitKrakenTemplateFiles(root: string): void {
  const dir = gitDir(root);
  for (const name of readdirSync(dir)) {
    if (!name.startsWith(GK_TEMPLATE_BASENAME)) continue;
    try {
      rmSync(join(dir, name), { force: true });
    } catch {
      /* ignore */
    }
  }
}

function resetGitCommitTemplateState(root: string): void {
  const dir = gitDir(root);
  removeGitKrakenTemplateFiles(root);
  const gkCommitTemplate = join(dir, GK_COMMIT_TEMPLATE_FILE);
  writeFileSync(gkCommitTemplate, "");
  git(root, ["config", "--local", "commit.template", gkCommitTemplate]);
}

/** 🧹Clears GitKraken templates, git draft messages, and micro-commit prepare state. */
export function clearGitCommitDraftState(root: string): void {
  const dir = gitDir(root);
  resetGitCommitTemplateState(root);
  for (const name of GIT_COMMIT_DRAFT_FILES) {
    try {
      writeFileSync(join(dir, name), "");
    } catch {
      /* ignore */
    }
  }
  removeGitDirPrefixed(root, "compose-micro-commit");
}

/** 🧹Resets GK/git commit-template state without wiping the active draft message file (COMMIT_EDITMSG/MERGE_MSG/SQUASH_MSG). */
export function clearMicroCommitTemplatesOnly(root: string): void {
  resetGitCommitTemplateState(root);
  removeGitDirPrefixed(root, "compose-micro-commit");
}

function clearStaleTemplatesBeforePrepare(root: string): void {
  if (!isPrepareActive(root)) clearGitCommitDraftState(root);
}

/** 🧹Removes prepare state and resets GK/git templates to empty after a commit. */
export function wipeAfterCommit(root: string): void {
  clearGitCommitDraftState(root);
}

export function handlePrepareCommitMsg(root: string, msgFile: string, source: string): void {
  if (!isPrepareActive(root)) {
    clearMicroCommitTemplatesOnly(root);
    return;
  }
  if (!branchAllowed(root)) return;
  const contributor = findContributor(root);
  if (!contributor) return;
  if (source === "merge" || source === "squash") return;
  const preparedBullets = readPreparedBullets(root);
  const newTickets = listAddedTicketPaths(root);
  if (preparedBullets.length === 0 && newTickets.length === 0) {
    const current = existsSync(msgFile) ? readFileSync(msgFile, "utf8") : "";
    if (current.trim()) return;
    return;
  }
  const digestPath = preparedDigestPath(root);
  const preparedDigest = existsSync(digestPath) ? readFileSync(digestPath, "utf8") : null;
  const current = existsSync(msgFile) ? readFileSync(msgFile, "utf8") : "";
  if (!shouldRefreshPreparedCommitMessage(current, preparedDigest)) return;
  const message = buildMicroCommitMessage(root, contributor, preparedBullets);
  writeFileSync(msgFile, message);
  writeMicroCommitTemplates(root, message);
}

const MICRO_COMMIT_BUN_PIN = "compose-micro-commit-bun";

/** 🥖Resolves the Bun executable for git hooks (GUI git often has a minimal PATH). */
export function resolveMicroCommitBunBin(root: string): string {
  const fromEnv = process.env.COMPOSE_BUN?.trim();
  if (fromEnv) return fromEnv;
  const argv0 = process.argv[0] ?? "";
  if (/bun(\.exe)?$/i.test(argv0)) return argv0;
  const win = process.platform === "win32";
  const home = process.env.HOME ?? process.env.USERPROFILE ?? "";
  const bunInstall = process.env.BUN_INSTALL ?? join(home, ".bun");
  const candidates = [join(root, "node_modules", ".bin", win ? "bun.cmd" : "bun"), join(root, "node_modules", ".bin", "bun.exe"), join(bunInstall, "bin", win ? "bun.exe" : "bun"), join(bunInstall, "bin", "bun")];
  for (const c of candidates) {
    if (c && existsSync(c)) return c;
  }
  const which = spawnSync(win ? "where" : "which", ["bun"], { encoding: "utf8", shell: win });
  if (which.status === 0) {
    const first = (which.stdout ?? "")
      .split(/\r?\n/)
      .map((l) => l.trim())
      .find(Boolean);
    if (first && existsSync(first)) return first;
  }
  return win ? "bun.exe" : "bun";
}

const MICRO_COMMIT_SEED_EMPTY_GK_SH = `compose_micro_commit_seed_empty_gk() {
  GIT_DIR=$(git rev-parse --git-dir 2>/dev/null) || return 0
  GK_TEMPLATE="$GIT_DIR/${GK_COMMIT_TEMPLATE_FILE}"
  if [ -d "$GIT_DIR" ]; then
    for f in "$GIT_DIR"/gkcommittemplate*; do
      [ -e "$f" ] || continue
      rm -f "$f" 2>/dev/null || true
    done
  fi
  : >"$GK_TEMPLATE" 2>/dev/null || true
  git config --local commit.template "$GK_TEMPLATE" 2>/dev/null || true
}`;

const MICRO_COMMIT_WIPE_FULL_SH = `${MICRO_COMMIT_SEED_EMPTY_GK_SH}
compose_micro_commit_wipe() {
  GIT_DIR=$(git rev-parse --git-dir 2>/dev/null) || return 0
  compose_micro_commit_seed_empty_gk
  for msg in COMMIT_EDITMSG MERGE_MSG SQUASH_MSG; do
    if [ -f "$GIT_DIR/$msg" ]; then
      : >"$GIT_DIR/$msg" 2>/dev/null || true
    fi
  done
  if [ -d "$GIT_DIR" ]; then
    for f in "$GIT_DIR"/compose-micro-commit-*; do
      [ -e "$f" ] || continue
      rm -f "$f" 2>/dev/null || true
    done
  fi
}`;

const MICRO_COMMIT_RESOLVE_BUN_SH = `compose_resolve_bun() {
  ROOT="$1"
  if [ -n "$COMPOSE_BUN" ] && [ -x "$COMPOSE_BUN" ]; then
    echo "$COMPOSE_BUN"
    return
  fi
  if [ -f "$ROOT/.repo/${MICRO_COMMIT_BUN_PIN}" ]; then
    B=$(head -n 1 "$ROOT/.repo/${MICRO_COMMIT_BUN_PIN}" | tr -d '\\r')
    if [ -n "$B" ] && [ -x "$B" ]; then
      echo "$B"
      return
    fi
  fi
  if [ -n "$BUN_INSTALL" ] && [ -x "$BUN_INSTALL/bin/bun" ]; then
    echo "$BUN_INSTALL/bin/bun"
    return
  fi
  if [ -n "$BUN_INSTALL" ] && [ -x "$BUN_INSTALL/bin/bun.exe" ]; then
    echo "$BUN_INSTALL/bin/bun.exe"
    return
  fi
  if [ -x "$ROOT/node_modules/.bin/bun" ]; then
    echo "$ROOT/node_modules/.bin/bun"
    return
  fi
  if [ -x "$ROOT/node_modules/.bin/bun.cmd" ]; then
    echo "$ROOT/node_modules/.bin/bun.cmd"
    return
  fi
  if [ -x "$ROOT/node_modules/.bin/bun.exe" ]; then
    echo "$ROOT/node_modules/.bin/bun.exe"
    return
  fi
  if [ -x "$HOME/.bun/bin/bun" ]; then
    echo "$HOME/.bun/bin/bun"
    return
  fi
  if [ -x "$HOME/.bun/bin/bun.exe" ]; then
    echo "$HOME/.bun/bin/bun.exe"
    return
  fi
  B=$(command -v bun 2>/dev/null || true)
  if [ -n "$B" ]; then
    echo "$B"
  fi
}`;

/** 🪝Renders a portable `sh` git hook (LF, inline wipe; Bun only when needed). */
export function renderMicroCommitGitHook(name: "prepare-commit-msg" | (typeof MICRO_COMMIT_POST_WIPE_HOOKS)[number]): string {
  const isPostWipe = (MICRO_COMMIT_POST_WIPE_HOOKS as readonly string[]).includes(name);
  const lines = [
    "#!/usr/bin/env sh",
    isPostWipe ? MICRO_COMMIT_WIPE_FULL_SH : MICRO_COMMIT_SEED_EMPTY_GK_SH,
    "ROOT=$(git rev-parse --show-toplevel 2>/dev/null) || exit 0",
    'cd "$ROOT" || exit 0',
    "GIT_DIR=$(git rev-parse --git-dir 2>/dev/null) || exit 0",
  ];
  if (isPostWipe) {
    lines.push(MICRO_COMMIT_RESOLVE_BUN_SH, 'BUN=$(compose_resolve_bun "$ROOT")', '[ -n "$BUN" ] && "$BUN" ./script.ts micro-commit reset 2>/dev/null || true', "compose_micro_commit_wipe", "exit 0");
  } else {
    lines.push(
      MICRO_COMMIT_RESOLVE_BUN_SH,
      '[ ! -f "$GIT_DIR/compose-micro-commit-active" ] && {',
      "  compose_micro_commit_seed_empty_gk",
      "  exit 0",
      "}",
      'BUN=$(compose_resolve_bun "$ROOT")',
      '[ -z "$BUN" ] && exit 0',
      'exec "$BUN" ./script.ts micro-commit prepare-commit-msg "$1" "$2"',
    );
  }
  return `${lines.join("\n")}\n`;
}

function writeMicroCommitHookFile(path: string, body: string): void {
  writeFileSync(path, body.replace(/\r\n/g, "\n"), "utf8");
  try {
    chmodSync(path, 0o755);
  } catch {
    /* windows */
  }
}

export function installMicroCommitGitHooks(root: string): void {
  const bunBin = resolveMicroCommitBunBin(root).replace(/\r/g, "");
  mkdirSync(join(root, ".repo"), { recursive: true });
  writeFileSync(join(root, ".repo", MICRO_COMMIT_BUN_PIN), `${bunBin}\n`, "utf8");
  const hooksDir = join(root, ".git", "hooks");
  const repoHooksDir = join(root, "repo", "hooks");
  mkdirSync(hooksDir, { recursive: true });
  mkdirSync(repoHooksDir, { recursive: true });
  for (const name of [...MICRO_COMMIT_POST_WIPE_HOOKS, "prepare-commit-msg"] as const) {
    const body = renderMicroCommitGitHook(name);
    writeMicroCommitHookFile(join(repoHooksDir, name), body);
    writeMicroCommitHookFile(join(hooksDir, name), body);
  }
  const stalePreCommit = join(hooksDir, "pre-commit");
  if (existsSync(stalePreCommit)) rmSync(stalePreCommit, { force: true });
  const repoPreCommit = join(repoHooksDir, "pre-commit");
  if (existsSync(repoPreCommit)) rmSync(repoPreCommit, { force: true });
}

export function resetMicroCommitTemplates(root: string): void {
  wipeAfterCommit(root);
}

function emitPrepareStdout(message: string): void {
  process.stdout.write(message.endsWith("\n") ? message : `${message}\n`);
}

export function runMicroCommit(root: string, segments: string[]): void {
  root = gitRepoRoot(root);
  const cmd = segments[0] ?? "prepare";
  if (cmd === "reset") {
    resetMicroCommitTemplates(root);
    process.exit(0);
  }
  if (cmd === "install-hooks") {
    installMicroCommitGitHooks(root);
    process.exit(0);
  }
  if (cmd === "prepare-commit-msg") {
    const msgFile = segments[1];
    if (!msgFile) process.exit(1);
    handlePrepareCommitMsg(root, msgFile, segments[2] ?? "");
    process.exit(0);
  }
  if (!branchAllowed(root)) {
    console.error("micro-commit: branch must contain ⛳wip or 🏗️dev");
    process.exit(1);
  }
  const contributor = findContributor(root);
  if (!contributor) {
    console.error(`micro-commit: no contributor for git user.email ${gitEmail(root) || "(unset)"}`);
    process.exit(1);
  }
  if (cmd === "stage") {
    clearStaleTemplatesBeforePrepare(root);
    const staged = git(root, ["add", "-A"]);
    if (!staged.ok) {
      console.error(staged.out || "git add -A failed");
      process.exit(1);
    }
    process.exit(0);
  }
  if (cmd === "diff") {
    const patch = git(root, ["diff", "--cached"]);
    if (!patch.ok) {
      console.error(patch.out || "git diff --cached failed");
      process.exit(1);
    }
    process.stdout.write(patch.out ? `${patch.out}\n` : "");
    process.exit(0);
  }
  if (cmd !== "prepare") {
    console.error("[micro-commit] usage: bun ./script.ts micro-commit <stage|diff|prepare> [level tokens…] [-- bullets.txt]");
    process.exit(1);
  }

  const dash = segments.indexOf("--");
  const levelSegments = dash >= 0 ? segments.slice(1, dash) : segments.slice(1);
  const bulletsFile = dash >= 0 ? (segments[dash + 1] ?? null) : null;

  const level = loadLevel(root, contributor, levelSegments);
  clearStaleTemplatesBeforePrepare(root);
  const staged = git(root, ["add", "-A"]);
  if (!staged.ok) {
    console.error(staged.out || "git add -A failed");
    process.exit(1);
  }

  const stagedPaths = listCachedPaths(root);
  const diffBullets = readDiffBulletsInput(root, bulletsFile);
  if (diffBullets.length === 0) {
    for (const p of stagedPaths) console.error(p);
    console.error("");
    const patch = git(root, ["diff", "--cached"]);
    if (patch.out) console.error(patch.out);
    console.error("\nmicro-commit: analyze the staged paths and diff above; pass 1–8 bullets on stdin (`{emoji}{description}` — pick the emoji that best matches each line, no leading `-`, no space after emoji; never 🎆 📊 🔢 🚩)");
    process.exit(1);
  }

  validateBulletSpacing(diffBullets);
  validateBulletEmojis(diffBullets);
  validateBulletsAgainstStaged(diffBullets, stagedPaths);
  writePreparedBullets(root, diffBullets);
  let message: string;
  try {
    message = buildMicroCommitMessage(root, contributor, diffBullets);
  } catch (e) {
    console.error(e instanceof Error ? e.message : String(e));
    process.exit(1);
  }
  writeMicroCommitTemplates(root, message);
  emitPrepareStdout(message);

  if (level === "prepare-only") process.exit(0);

  const dir = gitDir(root);
  const commit = spawnSync("git", ["commit", "-S", "-F", join(dir, "COMMIT_EDITMSG")], { cwd: root, encoding: "utf8" });
  if (commit.status !== 0) {
    console.error((commit.stderr ?? commit.stdout ?? "git commit failed").trim());
    process.exit(commit.status ?? 1);
  }
  wipeAfterCommit(root);
  if (level === "prepare-and-commit") process.exit(0);

  const push = spawnSync("git", ["push"], { cwd: root, encoding: "utf8" });
  if (push.status !== 0) {
    console.error((push.stderr ?? push.stdout ?? "git push failed").trim());
    process.exit(push.status ?? 1);
  }
  process.exit(0);
}

//#endregion 🔖micro-commit

//#region 🔖commit
export type CommitLevel = "prepare-only" | "prepare-and-tag" | "prepare-and-tag-and-squash" | "prepare-and-tag-and-squash-and-push";

export type CommitSteps = { tag: boolean; squash: boolean; push: boolean };

export type CommitBundleDateSection = { dateLine: string; bullets: string[] };
export type CommitBundleSection = { label: string; dates: CommitBundleDateSection[] };

export const BUNDLE_WIP_SUBJECT_RE = /^(.+🎆\d{2}🌙\d{2}☀️\d{2})🔀$/u;
export const BUNDLE_DATE_SECTION_RE = /^🎆\d{2}🌙\d{2}☀️\d{2}$/u;
const EMOJI_CLUSTER_RE = /^(\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*)/u;
const BUNDLE_SCOPE_RESERVED_RE = /🔀|🚩|📊uloc|🔢/u;
const LABEL_TOKEN_BLOCKLIST = new Set(["uloc", "repo", "the", "and"]);

/** 🧭Parses explicit `ct` / `cs` / `cp` step flags from argv segments. */
export function parseCommitSteps(segments: string[]): CommitSteps {
  const token = segments.join(" ").toLowerCase();
  return {
    tag: /\b(ct|ctag|tag!|\+tag)\b/.test(token),
    squash: /\b(cs|csquash|squash!|\+squash)\b/.test(token),
    push: /\b(cp|cpush|push!|\+push)\b/.test(token),
  };
}

function commitStepsFromLevel(level: CommitLevel): CommitSteps {
  switch (level) {
    case "prepare-and-tag":
      return { tag: true, squash: false, push: false };
    case "prepare-and-tag-and-squash":
      return { tag: true, squash: true, push: false };
    case "prepare-and-tag-and-squash-and-push":
      return { tag: true, squash: true, push: true };
    default:
      return { tag: false, squash: false, push: false };
  }
}

function loadCommitSteps(root: string, contributor: Contributor, segments: string[]): CommitSteps {
  const explicit = parseCommitSteps(segments);
  if (explicit.tag || explicit.squash || explicit.push) return explicit;
  const path = join(root, ".repo", "🧑‍💻", contributor.alias, "commit.json");
  if (existsSync(path)) {
    const j = JSON.parse(readFileSync(path, "utf8")) as { level?: string };
    const allowed: CommitLevel[] = ["prepare-only", "prepare-and-tag", "prepare-and-tag-and-squash", "prepare-and-tag-and-squash-and-push"];
    if (allowed.includes(j.level as CommitLevel)) return commitStepsFromLevel(j.level as CommitLevel);
  }
  return { tag: false, squash: false, push: false };
}

export function isCommitPrepareOnly(segments: string[]): boolean {
  const token = segments.join(" ").toLowerCase();
  if (/\b(c\.|cprepare|prepare!|\+prepare)\b/.test(token)) return true;
  const steps = parseCommitSteps(segments);
  return !steps.tag && !steps.squash && !steps.push;
}

function shSingleQuote(s: string): string {
  return `'${s.replace(/'/g, `'\"'\"'`)}'`;
}

function formatCommitPrepareCommandBlock(command: string): string {
  return `\`\`\`\n${command}\n\`\`\``;
}

/** 🏷️GPG-signed annotated tag; tag object name and `-m` message are the same (`…🚩`). */
export function formatGitSignedTagCommand(tagName: string, head = "HEAD"): string {
  const q = shSingleQuote(tagName);
  return `git tag -s -m ${q} ${q} ${head}`;
}

/** 📋Four copy-paste ``` blocks: signed tag, squash, push, all-in-one. */
export function formatCommitPrepareCommands(opts: { tagName: string; wipSha: string; messageFile?: string }): string {
  const msg = opts.messageFile ?? ".git/compose-commit-message";
  const tag = formatGitSignedTagCommand(opts.tagName);
  const squash = `git reset --soft ${opts.wipSha} && git commit -S -F ${shSingleQuote(msg)}`;
  const push = `git push --follow-tags`;
  const all = `${tag} && git reset --soft ${opts.wipSha} && git commit -S -F ${shSingleQuote(msg)} && git push --follow-tags`;
  return `${[tag, squash, push, all].map(formatCommitPrepareCommandBlock).join("\n\n")}\n`;
}

/** 📋Prepare-only agent reply: four `git` blocks, then tag name, then full commit message. */
export function formatCommitPrepareAgentReply(opts: { tagName: string; wipSha: string; messageFile?: string; commitMessage: string }): string {
  const commands = formatCommitPrepareCommands({
    tagName: opts.tagName,
    wipSha: opts.wipSha,
    messageFile: opts.messageFile,
  });
  const tagNameBlock = formatCommitPrepareCommandBlock(opts.tagName.trim());
  const messageBlock = formatCommitPrepareCommandBlock(opts.commitMessage.trimEnd());
  return `${commands}${tagNameBlock}\n\n${messageBlock}\n`;
}

/** 🔀Finds the newest commit whose subject is a bundle/WIP marker (`…🔀`). */
export function findLastBundleWipCommit(root: string): { sha: string; subject: string } | null {
  root = gitRepoRoot(root);
  const r = spawnSync("git", ["log", "--format=%H%x00%s%x00", "-n", "500"], {
    cwd: root,
    encoding: "utf8",
    maxBuffer: 16 * 1024 * 1024,
  });
  if (r.status !== 0) return null;
  const parts = (r.stdout ?? "").split("\0").filter(Boolean);
  for (let i = 0; i + 1 < parts.length; i += 2) {
    const sha = parts[i]!.trim();
    const subject = parts[i + 1]!.trim();
    if (BUNDLE_WIP_SUBJECT_RE.test(subject)) return { sha, subject };
  }
  return null;
}

/** 🏷️Signed tag name for the micro-commit tip before squash (`…🚩`). */
export function formatBundleTagName(contributor: Contributor, now = new Date()): string {
  const yy = pad2(now.getFullYear() % 100);
  const mm = pad2(now.getMonth() + 1);
  const dd = pad2(now.getDate());
  return `${contributor.emoji}${contributor.alias}🎆${yy}🌙${mm}☀️${dd}🚩`;
}

/** 🔀Bundle squash commit subject (`…🔀`). */
export function formatBundleSubject(contributor: Contributor, now = new Date()): string {
  const yy = pad2(now.getFullYear() % 100);
  const mm = pad2(now.getMonth() + 1);
  const dd = pad2(now.getDate());
  return `${contributor.emoji}${contributor.alias}🎆${yy}🌙${mm}☀️${dd}🔀`;
}

/** 🔢Leading emoji clusters on a line. */
export function leadingEmojiClusterCount(line: string): number {
  let n = 0;
  let rest = line.trim();
  while (true) {
    const m = EMOJI_CLUSTER_RE.exec(rest);
    if (!m) break;
    n++;
    rest = rest.slice(m[0].length);
  }
  return n;
}

/** 🔢Emoji clusters anywhere on a line (bundle labels interleave emoji + text). */
export function emojiClusterCountInLine(line: string): number {
  const re = /\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*/gu;
  return [...line.matchAll(re)].length;
}

/** 🧹Strips hand-written uloc and reserved bundle/tag emojis from a scope line. */
export function normalizeBundleScopeLabel(line: string): string {
  return line
    .trim()
    .replace(/📊uloc.*$/u, "")
    .replace(/🔀|🚩/gu, "")
    .trim();
}

/** 🏷️ASCII tokens from a bundle emoji label (for matching git paths internally). */
export function labelPathTokens(label: string): string[] {
  const text = normalizeBundleScopeLabel(label)
    .replace(/\p{Extended_Pictographic}/gu, " ")
    .trim()
    .toLowerCase();
  return text.split(/[^a-z0-9]+/).filter((t) => t.length >= 2 && !LABEL_TOKEN_BLOCKLIST.has(t));
}

/** 🚫Validates bundle scope label before parse. */
export function bundleScopeLabelError(line: string): string | null {
  const raw = line.trim();
  if (raw.includes("|") || raw.includes("/")) {
    return "commit: bundle scope must be emoji + area name only — no paths or `|`";
  }
  if (BUNDLE_SCOPE_RESERVED_RE.test(raw)) {
    return "commit: bundle scope must not include 🔀 🚩 📊uloc or 🔢 — script adds subject and uloc";
  }
  const norm = normalizeBundleScopeLabel(raw);
  if (!norm) return "commit: empty bundle scope after removing reserved emojis";
  const tokens = labelPathTokens(norm);
  if (tokens.length === 0) {
    return "commit: bundle scope needs an area name after emojis (e.g. 🏘️compose✍️sketchpad, 🥅framework, 🖱️ui⚛️react)";
  }
  if (!isBundleScopeLine(norm)) {
    return `commit: invalid bundle scope: ${raw}`;
  }
  return null;
}

function changedPathsInRange(root: string, base: string, head: string): string[] {
  const r = git(root, ["diff", "--name-only", `${base}..${head}`]);
  if (!r.ok || !r.out) return [];
  return r.out.split("\n").filter(Boolean);
}

function longestCommonPathPrefix(paths: string[]): string {
  if (paths.length === 0) return "";
  const split = paths.map((p) => p.split("/"));
  const first = split[0]!;
  let len = 0;
  for (let i = 0; i < first.length; i++) {
    const seg = first[i]!;
    if (split.every((parts) => parts[i] === seg)) len = i + 1;
    else break;
  }
  if (len === 0) return paths[0]!.split("/")[0] ?? "";
  return first.slice(0, len).join("/");
}

/** 📂Infers repo path prefixes for a bundle label from the revision range (not shown in messages). */
export function inferPathPrefixesForBundleLabel(root: string, base: string, head: string, label: string, assignedPaths?: string[]): string[] {
  const tokens = labelPathTokens(label);
  if (tokens.length === 0) return [];
  const pool = assignedPaths ?? changedPathsInRange(root, base, head);
  const matched = pool.filter((p) => {
    const pl = p.toLowerCase();
    return tokens.every((t) => pl.includes(t));
  });
  if (matched.length === 0) return [];
  const prefix = longestCommonPathPrefix(matched);
  return prefix ? [prefix] : [];
}

function assignPathToBundleIndex(bundles: CommitBundleSection[], path: string): number {
  const pl = path.toLowerCase();
  let best = -1;
  let bestScore = 0;
  for (let i = 0; i < bundles.length; i++) {
    const tokens = labelPathTokens(bundles[i]!.label);
    if (tokens.length === 0) continue;
    if (!tokens.every((t) => pl.includes(t))) continue;
    const score = tokens.length;
    if (score > bestScore) {
      bestScore = score;
      best = i;
    }
  }
  return best;
}

function assignChangedPathsToBundles(root: string, base: string, head: string, bundles: CommitBundleSection[]): string[][] {
  const paths = changedPathsInRange(root, base, head);
  const assigned = bundles.map(() => [] as string[]);
  for (const p of paths) {
    const bi = assignPathToBundleIndex(bundles, p);
    if (bi >= 0) assigned[bi]!.push(p);
  }
  return assigned;
}

type GitDeltaSum = { added: number; removed: number; edited: number };

function addGitDeltaSums(a: GitDeltaSum, b: GitDeltaSum): GitDeltaSum {
  return { added: a.added + b.added, removed: a.removed + b.removed, edited: a.edited + b.edited };
}

const BUNDLE_COMMIT_TIMESTAMP_LINE_RE = /^🎆\d{2}🌙\d{2}☀️\d{2}⏰/u;

/** 🎆Calendar day from a micro-commit subject (`🎆YY🌙MM☀️DD`). */
export function extractBundleDateLineFromSubject(subject: string): string | null {
  const m = /🎆\d{2}🌙\d{2}☀️\d{2}/u.exec(subject.trim());
  return m?.[0] ?? null;
}

/** 🎆Calendar day from the micro-commit body timestamp line (`🎆YY🌙MM☀️DD⏰…`). */
export function extractBundleDateLineFromCommitBody(body: string): string | null {
  for (const raw of body.split("\n")) {
    const line = raw.trim();
    if (!BUNDLE_COMMIT_TIMESTAMP_LINE_RE.test(line)) continue;
    const m = /^🎆\d{2}🌙\d{2}☀️\d{2}/u.exec(line);
    return m?.[0] ?? null;
  }
  return null;
}

/** 🎆Calendar day for bundle per-day uloc (body timestamp, else subject). */
export function extractBundleDateLineFromCommit(subject: string, body: string): string | null {
  return extractBundleDateLineFromCommitBody(body) ?? extractBundleDateLineFromSubject(subject);
}

/** 📂Paths from one numstat row (rename rows may join old/new with tabs or `=>`). */
export function pathsFromNumstatRow(pathField: string): string[] {
  const parts = pathField
    .split("\t")
    .map((p) => p.trim())
    .filter(Boolean);
  if (parts.length === 0) return [];
  const out = new Set<string>();
  for (const part of parts) {
    const brace = /^(.*)\{(.+?)\s*=>\s*(.+?)\}(.*)$/u.exec(part);
    if (brace) {
      const a = brace[2]!.trim();
      const b = brace[3]!.trim();
      if (a) out.add(a);
      if (b) out.add(b);
      continue;
    }
    const arrow = /\s*=>\s*/u.exec(part);
    if (arrow) {
      const [from, to] = part.split(/\s*=>\s*/u);
      if (from?.trim()) out.add(from.trim());
      if (to?.trim()) out.add(to.trim());
      continue;
    }
    out.add(part);
  }
  return [...out];
}

/** 📂Prefix set for a bundle: all range paths, inferred roots, and token-matching parents (renames). */
export function buildBundlePathPrefixSets(root: string, base: string, head: string, bundles: CommitBundleSection[]): string[][] {
  const assignments = assignChangedPathsToBundles(root, base, head, bundles);
  return bundles.map((bundle, i) => {
    const assigned = assignments[i] ?? [];
    const prefixes = new Set<string>();
    const tokens = labelPathTokens(bundle.label);
    for (const p of assigned) {
      prefixes.add(p);
      if (tokens.length === 0) continue;
      const segments = p.split("/");
      let acc = "";
      for (const seg of segments) {
        acc = acc ? `${acc}/${seg}` : seg;
        const pl = acc.toLowerCase();
        if (tokens.every((t) => pl.includes(t))) prefixes.add(acc);
      }
    }
    return [...prefixes];
  });
}

/** 📂Whether a path belongs to a bundle (assigned path prefixes, else label token scoring). */
export function pathMatchesBundleIndex(path: string, bundleIndex: number, prefixSets: string[][], bundles: CommitBundleSection[]): boolean {
  const prefixes = prefixSets[bundleIndex] ?? [];
  if (prefixes.length > 0) return pathUnderPrefixes(path, prefixes);
  return assignPathToBundleIndex(bundles, path) === bundleIndex;
}

/** 🧹Strips hand-written per-day uloc from a date section line. */
export function normalizeBundleDateLine(line: string): string {
  return line
    .trim()
    .replace(/📊uloc.*$/u, "")
    .trim();
}

/** 🎆Bundle squash only: `🎆YY🌙MM☀️DD` + per-day git delta suffix (micro-commit uses timestamp + footer uloc only). */
export function formatBundleDateLine(dateLine: string, d: GitDeltaSum): string {
  return `${normalizeBundleDateLine(dateLine)}${formatBundleUlocSuffix(d)}`;
}

export type BundleDateDeltasMap = Map<number, Map<string, GitDeltaSum>>;

function gitCommitShasInRange(root: string, base: string, head: string): string[] {
  const r = git(root, ["rev-list", "--reverse", `${base}..${head}`]);
  if (!r.ok || !r.out) return [];
  return r.out.split("\n").filter(Boolean);
}

function addNumstatRowToBundleDateMap(map: BundleDateDeltasMap, row: { path: string; added: number; removed: number }, dateLine: string, bi: number, root: string): void {
  const chunk = sumGitLangDeltas(accumulateGitDeltasFromNumstat(root, [{ path: row.path, added: row.added, removed: row.removed }]));
  if (gitDeltaLineTotal(chunk) === 0) return;
  const bundleMap = map.get(bi)!;
  const prev = bundleMap.get(dateLine) ?? { added: 0, removed: 0, edited: 0 };
  bundleMap.set(dateLine, addGitDeltaSums(prev, chunk));
}

/** 📊Per-bundle per-day git deltas: sum each micro-commit parent..sha row on its body 🎆 day (sums to range partition). */
export function buildBundleDateDeltasMap(root: string, base: string, head: string, bundles: CommitBundleSection[]): BundleDateDeltasMap {
  root = gitRepoRoot(root);
  const map: BundleDateDeltasMap = new Map();
  for (let i = 0; i < bundles.length; i++) map.set(i, new Map());
  const prefixSets = buildBundlePathPrefixSets(root, base, head, bundles);
  for (const sha of gitCommitShasInRange(root, base, head)) {
    const parent = `${sha}^`;
    const subject = git(root, ["log", "-1", "--format=%s", sha]).out;
    const body = git(root, ["log", "-1", "--format=%B", sha]).out;
    const dateLine = extractBundleDateLineFromCommit(subject, body);
    if (!dateLine) continue;
    for (const row of gitRangeNumstat(root, parent, sha)) {
      const rowPaths = pathsFromNumstatRow(row.path);
      if (rowPaths.length === 0 || rowPaths.every((p) => shouldSkipPathForUloc(root, p))) continue;
      const owners = resolveBundleIndicesForNumstatRow(row.path, prefixSets, bundles);
      if (owners.length === 0) {
        throw new Error(`commit: changed path is not attributed to any bundle — ${row.path}; add a bundle scope or fix labels`);
      }
      if (owners.length > 1) {
        const names = owners.map((i) => bundles[i]!.label).join(", ");
        throw new Error(`commit: changed path matches multiple bundles (${names}) — ${row.path}`);
      }
      addNumstatRowToBundleDateMap(map, row, dateLine, owners[0]!, root);
    }
  }
  return map;
}

function bundleGitDeltasForPaths(root: string, base: string, head: string, pathPrefixes: string[]): { added: number; removed: number; edited: number } {
  const rows = gitRangeNumstat(root, base, head);
  const assigned = pathPrefixes.length > 0 ? rows.filter((r) => pathsFromNumstatRow(r.path).some((p) => pathUnderPrefixes(p, pathPrefixes))) : [];
  return sumGitLangDeltas(accumulateGitDeltasFromNumstat(root, assigned));
}

function formatBundleHeaderLine(label: string, total: GitDeltaSum): string {
  return `${normalizeBundleScopeLabel(label)}${formatBundleUlocSuffix(total)}`;
}

/** 📊Orders bundles by descending 🟰 (➕+✏️+➖) from assigned path diffs. */
export function sortCommitBundlesByEditTotal(root: string, base: string, head: string, bundles: CommitBundleSection[], pathAssignments: string[][]): { bundles: CommitBundleSection[]; pathAssignments: string[][]; pathPrefixSets: string[][] } {
  const prefixSets = buildBundlePathPrefixSets(root, base, head, bundles);
  const ranked = bundles.map((bundle, i) => ({
    bundle,
    paths: pathAssignments[i] ?? [],
    prefixes: prefixSets[i] ?? [],
    total: gitDeltaLineTotal(bundleGitDeltasForPaths(root, base, head, prefixSets[i] ?? [])),
  }));
  ranked.sort((a, b) => b.total - a.total);
  return {
    bundles: ranked.map((r) => r.bundle),
    pathAssignments: ranked.map((r) => r.paths),
    pathPrefixSets: ranked.map((r) => r.prefixes),
  };
}

/** 🏷️Bundle scope line: two+ emojis, or one emoji + lowercase technology slug (`🥅framework`). */
export function isBundleScopeLine(line: string): boolean {
  const t = normalizeBundleScopeLabel(line);
  if (!t || BUNDLE_DATE_SECTION_RE.test(t)) return false;
  if (t.includes("/") || t.includes("|")) return false;
  if (BUNDLE_SCOPE_RESERVED_RE.test(t)) return false;
  if (emojiClusterCountInLine(t) >= 2) return true;
  if (emojiClusterCountInLine(t) !== 1) return false;
  const after = t.replace(/^\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*/u, "");
  return /^[a-z][a-z0-9]{2,24}$/u.test(after);
}

/** 🚫Validates stdin is bundle body only, not a full commit message. */
export function commitBundleBodyError(text: string): string | null {
  const first =
    text
      .trim()
      .split("\n")
      .find((l) => l.trim().length > 0)
      ?.trim() ?? "";
  if (BUNDLE_WIP_SUBJECT_RE.test(first)) {
    return "commit: stdin must not include the bundle subject (…🔀) — script adds it";
  }
  if (/^🐙|^🧑/.test(first) && first.includes("🚩")) {
    return "commit: stdin must not include micro-commit subject lines";
  }
  if (/^📊uloc/m.test(text) || /^🔢[\dk]/m.test(text)) {
    return "commit: stdin must not include the 📊uloc footer — script adds it";
  }
  if (/^🎆\d{2}🌙\d{2}☀️\d{2}📊uloc/m.test(text)) {
    return "commit: stdin must not include per-day 📊uloc — script adds it to each 🎆 line";
  }
  return null;
}

function validateBulletLine(b: string): void {
  if (!MICRO_COMMIT_BULLET_RE.test(b)) {
    throw new Error(`commit: bullet must start with {emoji} then description (no space after emoji): ${b}`);
  }
  const err = bulletEmojiValidationError([b]);
  if (err) throw new Error(err.replace(/^micro-commit:/, "commit:"));
  if (BUNDLE_DATE_SECTION_RE.test(b.trim())) {
    throw new Error("commit: use a date section line `🎆YY🌙MM☀️DD` on its own, not as a bullet");
  }
}

/** 📦Parses LLM bundle body (emoji-only scope lines, dates, bullets). */
export function parseCommitBundleBody(text: string): CommitBundleSection[] {
  const bundles: CommitBundleSection[] = [];
  let current: CommitBundleSection | null = null;
  let dateSection: CommitBundleDateSection | null = null;

  for (const raw of text.split("\n")) {
    const line = raw.trim();
    if (!line || line.startsWith("#")) continue;
    if (line.startsWith("📊uloc") || line.startsWith("🔢") || line.startsWith("Signed-off-by:")) continue;

    const dateCandidate = normalizeBundleDateLine(line);
    if (BUNDLE_DATE_SECTION_RE.test(dateCandidate)) {
      if (!current) throw new Error(`commit: date section before bundle scope: ${line}`);
      if (dateSection) current.dates.push(dateSection);
      dateSection = { dateLine: dateCandidate, bullets: [] };
      continue;
    }

    if (dateSection) {
      if (isBundleScopeLine(line)) {
        const scopeErr = bundleScopeLabelError(line);
        if (scopeErr) throw new Error(scopeErr);
        if (current) {
          current.dates.push(dateSection);
          bundles.push(current);
        }
        current = { label: normalizeBundleScopeLabel(line), dates: [] };
        dateSection = null;
        continue;
      }
      validateBulletLine(line);
      dateSection.bullets.push(line);
      continue;
    }

    if (isBundleScopeLine(line)) {
      const scopeErr = bundleScopeLabelError(line);
      if (scopeErr) throw new Error(scopeErr);
      if (current) bundles.push(current);
      current = { label: normalizeBundleScopeLabel(line), dates: [] };
      continue;
    }

    if (!current) throw new Error(`commit: expected emoji bundle scope (two+ emojis, no paths), got: ${line}`);
    throw new Error(`commit: expected 🎆YY🌙MM☀️DD date section before bullet: ${line}`);
  }

  if (current) {
    if (dateSection) current.dates.push(dateSection);
    bundles.push(current);
  }
  if (bundles.length === 0) throw new Error("commit: at least one emoji bundle scope is required");
  for (const b of bundles) {
    if (b.dates.length === 0) throw new Error(`commit: bundle ${b.label} needs at least one date section`);
    for (const d of b.dates) {
      if (d.bullets.length === 0) throw new Error(`commit: ${d.dateLine} in ${b.label} needs at least one bullet`);
    }
  }
  return bundles;
}

function formatGitDeltaSumBrief(d: GitDeltaSum): string {
  return `➕${d.added}✏️${d.edited}➖${d.removed}🟰${gitDeltaLineTotal(d)}`;
}

function assertGitDeltaSumsEqual(a: GitDeltaSum, b: GitDeltaSum, message: string): void {
  if (gitDeltaSumsEqual(a, b)) return;
  throw new Error(`${message} — ${formatGitDeltaSumBrief(a)} vs ${formatGitDeltaSumBrief(b)}`);
}

/** 📂Bundle indices that own a numstat row (0 or 1 after validation). */
export function resolveBundleIndicesForNumstatRow(pathField: string, prefixSets: string[][], bundles: CommitBundleSection[]): number[] {
  const matched = new Set<number>();
  for (const path of pathsFromNumstatRow(pathField)) {
    for (let bi = 0; bi < bundles.length; bi++) {
      if (pathMatchesBundleIndex(path, bi, prefixSets, bundles)) matched.add(bi);
    }
  }
  return [...matched];
}

function rangeGitDeltaTotal(root: string, base: string, head: string): GitDeltaSum {
  return sumGitLangDeltas(accumulateGitDeltasFromNumstat(root, gitRangeNumstat(root, base, head)));
}

/** 📊Partition WIP-range numstat across bundles (one bundle per row). */
export function partitionRangeDeltasByBundle(root: string, base: string, head: string, bundles: CommitBundleSection[], prefixSets: string[][]): { bundleTotals: GitDeltaSum[]; rangeTotal: GitDeltaSum } {
  const bundleTotals = bundles.map(() => ({ added: 0, removed: 0, edited: 0 }));
  let rangeTotal: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
  for (const row of gitRangeNumstat(root, base, head)) {
    const rowPaths = pathsFromNumstatRow(row.path);
    if (rowPaths.length === 0 || rowPaths.every((p) => shouldSkipPathForUloc(root, p))) continue;
    const chunk = sumGitLangDeltas(accumulateGitDeltasFromNumstat(root, [{ path: row.path, added: row.added, removed: row.removed }]));
    if (gitDeltaLineTotal(chunk) === 0) continue;
    rangeTotal = addGitDeltaSums(rangeTotal, chunk);
    const owners = resolveBundleIndicesForNumstatRow(row.path, prefixSets, bundles);
    if (owners.length === 0) {
      throw new Error(`commit: changed path is not attributed to any bundle — ${row.path}; add a bundle scope or fix labels`);
    }
    if (owners.length > 1) {
      const names = owners.map((i) => bundles[i]!.label).join(", ");
      throw new Error(`commit: changed path matches multiple bundles (${names}) — ${row.path}`);
    }
    const bi = owners[0]!;
    bundleTotals[bi] = addGitDeltaSums(bundleTotals[bi]!, chunk);
  }
  return { bundleTotals, rangeTotal };
}

/** 🚫Per-day uloc must sum to bundle header totals; missing/extra dates imply attribution mistakes. */
export function validateBundleDayDeltasAttribution(bundles: CommitBundleSection[], prefixSets: string[][], dateDeltas: BundleDateDeltasMap, bundleTotals: GitDeltaSum[]): void {
  for (let bi = 0; bi < bundles.length; bi++) {
    const bundle = bundles[bi]!;
    const total = bundleTotals[bi] ?? { added: 0, removed: 0, edited: 0 };
    const listedDates = new Set(bundle.dates.map((s) => s.dateLine));
    let daySum: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
    for (const dateLine of listedDates) {
      const d = dateDeltas.get(bi)?.get(dateLine) ?? { added: 0, removed: 0, edited: 0 };
      daySum = addGitDeltaSums(daySum, d);
    }
    const perDay = dateDeltas.get(bi);
    if (perDay) {
      for (const [dateLine, d] of perDay) {
        if (listedDates.has(dateLine)) continue;
        if (gitDeltaLineTotal(d) === 0) continue;
        throw new Error(`commit: ${bundle.label} has micro-commit changes on ${dateLine} (${formatGitDeltaSumBrief(d)}) but that day is missing from your bundle body — add a 🎆 section or fix attribution`);
      }
    }
    if (daySum.added !== total.added || daySum.edited !== total.edited || daySum.removed !== total.removed) {
      throw new Error(`commit: per-day 📊uloc for ${bundle.label} does not add up to the bundle total — days ${formatGitDeltaSumBrief(daySum)} vs bundle ${formatGitDeltaSumBrief(total)}; re-read log + diff and fix bundle/date attribution`);
    }
  }
}

/** 🚫All bundle-commit uloc constraints (days→bundle, bundles→range, languages→range). */
export function validateBundleCommitAttribution(root: string, base: string, head: string, bundles: CommitBundleSection[], ulocRunner?: UlocRunner): void {
  root = gitRepoRoot(root);
  const prefixSets = buildBundlePathPrefixSets(root, base, head, bundles);
  const { bundleTotals: partitioned, rangeTotal } = partitionRangeDeltasByBundle(root, base, head, bundles, prefixSets);
  const dateDeltas = buildBundleDateDeltasMap(root, base, head, bundles);
  validateBundleDayDeltasAttribution(bundles, prefixSets, dateDeltas, partitioned);
  for (let bi = 0; bi < bundles.length; bi++) {
    let allDays: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
    const perDay = dateDeltas.get(bi);
    if (perDay) {
      for (const d of perDay.values()) allDays = addGitDeltaSums(allDays, d);
    }
    assertGitDeltaSumsEqual(allDays, partitioned[bi] ?? { added: 0, removed: 0, edited: 0 }, `commit: all micro-commit days for ${bundles[bi]!.label} do not add up to the bundle total`);
  }
  let bundleSum: GitDeltaSum = { added: 0, removed: 0, edited: 0 };
  for (const t of partitioned) bundleSum = addGitDeltaSums(bundleSum, t);
  assertGitDeltaSumsEqual(bundleSum, rangeTotal, "commit: all bundle header totals do not add up to the WIP range 📊uloc — fix bundle attribution");
  const metrics = buildMicroCommitMetricsForRange(root, base, head, undefined, ulocRunner);
  validateMicroCommitLangMetricsDeltaSum(metrics);
  const langTotal = sumMicroCommitLangMetrics(metrics);
  assertGitDeltaSumsEqual({ added: langTotal.added, edited: langTotal.edited, removed: langTotal.removed }, rangeTotal, "commit: footer per-language 📊uloc does not add up to the WIP range total");
}

export function buildCommitMessage(root: string, contributor: Contributor, bundles: CommitBundleSection[], wipSha: string, head = "HEAD", ulocRunner?: UlocRunner, now = new Date()): string {
  root = gitRepoRoot(root);
  const pathAssignments = assignChangedPathsToBundles(root, wipSha, head, bundles);
  const sorted = sortCommitBundlesByEditTotal(root, wipSha, head, bundles, pathAssignments);
  bundles = sorted.bundles;
  validateBundleCommitAttribution(root, wipSha, head, bundles, ulocRunner);
  const prefixSets = buildBundlePathPrefixSets(root, wipSha, head, bundles);
  const { bundleTotals } = partitionRangeDeltasByBundle(root, wipSha, head, bundles, prefixSets);
  const dateDeltas = buildBundleDateDeltasMap(root, wipSha, head, bundles);
  const lines: string[] = [formatBundleSubject(contributor, now), ""];
  for (let bi = 0; bi < bundles.length; bi++) {
    const bundle = bundles[bi]!;
    lines.push(formatBundleHeaderLine(bundle.label, bundleTotals[bi] ?? { added: 0, removed: 0, edited: 0 }));
    const perDay = dateDeltas.get(bi);
    for (const section of bundle.dates) {
      const dayDelta = perDay?.get(section.dateLine) ?? { added: 0, removed: 0, edited: 0 };
      lines.push(formatBundleDateLine(section.dateLine, dayDelta));
      lines.push(...section.bullets);
    }
    if (bi < bundles.length - 1) lines.push("");
  }
  const metrics = formatMicroCommitMetricsLines(buildMicroCommitMetricsForRange(root, wipSha, head, undefined, ulocRunner));
  if (metrics.length > 0) lines.push(...metrics, "");
  lines.push(`Signed-off-by: ${contributor.name} <${contributor.email}>`);
  return `${lines.join("\n")}\n`;
}

function readBodyInput(root: string, file: string | null): string {
  if (file) {
    const path = file.startsWith("/") ? file : join(root, file);
    return readFileSync(path, "utf8");
  }
  if (process.stdin.isTTY) return "";
  return readFileSync(0, "utf8");
}

function assertCleanWorktree(root: string): void {
  const st = git(root, ["status", "--porcelain"]);
  if (!st.ok) return;
  if (st.out.trim()) {
    console.error("commit: working tree must be clean before tag/squash/push");
    process.exit(1);
  }
}

function emitStdout(message: string): void {
  process.stdout.write(message.endsWith("\n") ? message : `${message}\n`);
}

const COMMIT_DIFF_MAX_BYTES = 1_500_000;

export type CommitBundleRange = { wip: { sha: string; subject: string }; range: string };

/** 🔀Resolves last bundle WIP and revision range for analysis. */
export function resolveCommitBundleRange(root: string): CommitBundleRange | null {
  root = gitRepoRoot(root);
  const wip = findLastBundleWipCommit(root);
  if (!wip) return null;
  return { wip, range: `${wip.sha}..HEAD` };
}

function normalizeCompareLine(s: string): string {
  return s.trim().replace(/\s+/g, " ").toLowerCase();
}

/** 📜Collects comparable lines from commit bodies in range (for copy detection). */
export function commitHistoryCompareLines(root: string, base: string, head: string): Set<string> {
  const r = spawnSync("git", ["log", `--format=%B%x00`, `${base}..${head}`], {
    cwd: gitRepoRoot(root),
    encoding: "utf8",
    maxBuffer: 32 * 1024 * 1024,
  });
  const lines = new Set<string>();
  if (r.status !== 0) return lines;
  for (const body of (r.stdout ?? "").split("\0")) {
    if (!body.trim()) continue;
    for (const raw of body.split("\n")) {
      const line = raw.trim();
      if (line.length < 12) continue;
      if (BUNDLE_WIP_SUBJECT_RE.test(line)) continue;
      if (BUNDLE_DATE_SECTION_RE.test(normalizeBundleDateLine(line))) continue;
      if (line.startsWith("📊uloc") || line.startsWith("🔢")) continue;
      if (line.startsWith("Signed-off-by:")) continue;
      if (/^🎆\d{2}🌙\d{2}☀️\d{2}⏰/u.test(line)) continue;
      lines.add(normalizeCompareLine(line));
    }
  }
  return lines;
}

/** 🚫Whether a bullet verbatim-matches a line from a prior commit body in the range. */
export function bulletMatchesCommitHistory(bullet: string, history: Set<string>): boolean {
  return history.has(normalizeCompareLine(bullet));
}

/** 🚫Ensures bullets are newly written from diff analysis, not pasted from prior commits. */
export function validateBundleBulletsFresh(root: string, base: string, head: string, bundles: CommitBundleSection[]): void {
  const history = commitHistoryCompareLines(root, base, head);
  for (const bundle of bundles) {
    for (const section of bundle.dates) {
      for (const bullet of section.bullets) {
        if (bulletMatchesCommitHistory(bullet, history)) {
          throw new Error(`commit: bullet copies a prior commit message line — rewrite from git diff only: ${bullet}`);
        }
      }
    }
  }
}

function emitCommitBundleAttributionNote(): void {
  console.error("commit: bundles and file→bundle mapping are NOT automatic — folder layout and bundle boundaries change between WIPs");
  console.error("commit: you must (1) read log for last bundle/WIP state, (2) read diff --stat + full diff for every path, (3) decide scopes/dates/bullets, then prepare stdin");
  console.error("commit: script only adds subject, uloc suffixes, sort order, footer, Signed-off-by — never invents bundles or bullets");
  console.error("commit: prepare/check fail unless days→bundle, bundles→range, and languages→range (all ➕✏️➖🟰); run: bun ./script.ts commit check");
}

function emitCommitAnalysisHint(): void {
  console.error("commit: write NEW bundle scopes, dates, and bullets on prepare stdin after log + diff attribution");
}

/** 📜Prior commit messages since last WIP (context for dates only — not for copying bullets). */
export function emitCommitLog(root: string): void {
  root = gitRepoRoot(root);
  const resolved = resolveCommitBundleRange(root);
  if (!resolved) {
    console.error("commit: no prior bundle WIP commit (subject …🔀) found in recent history");
    process.exit(1);
  }
  const { wip, range } = resolved;
  console.error(`Last bundle WIP: ${wip.subject} (${wip.sha.slice(0, 7)})`);
  console.error(`Range: ${range}\n`);
  emitCommitBundleAttributionNote();
  console.error("\n=== prior commit messages (context only — do not copy bullets; old format may be wrong) ===\n");
  const log = git(root, ["log", `--format=commit %h%n%s%n%b%n---`, range]);
  if (log.ok && log.out) console.error(log.out);
  emitCommitAnalysisHint();
}

/** 📊Git diff since last WIP (primary source for new bundle summaries). */
export function emitCommitDiff(root: string): void {
  root = gitRepoRoot(root);
  const resolved = resolveCommitBundleRange(root);
  if (!resolved) {
    console.error("commit: no prior bundle WIP commit (subject …🔀) found in recent history");
    process.exit(1);
  }
  const { wip, range } = resolved;
  console.error(`Last bundle WIP: ${wip.subject} (${wip.sha.slice(0, 7)})`);
  console.error(`Range: ${range}\n`);
  const stat = git(root, ["diff", "--stat", range]);
  emitCommitBundleAttributionNote();
  console.error("\n=== git diff --stat (overview) ===\n");
  if (stat.ok && stat.out) console.error(`${stat.out}\n`);
  const patch = git(root, ["diff", range]);
  console.error("=== git diff (write bullets from this — not from prior commit text) ===\n");
  if (!patch.ok || !patch.out) {
    console.error(patch.out || "(empty diff)\n");
    emitCommitAnalysisHint();
    return;
  }
  const bytes = Buffer.byteLength(patch.out, "utf8");
  if (bytes > COMMIT_DIFF_MAX_BYTES) {
    console.error(patch.out.slice(0, COMMIT_DIFF_MAX_BYTES));
    console.error(`\n[commit diff truncated at ${COMMIT_DIFF_MAX_BYTES} bytes of ${bytes} — inspect locally: git diff ${range}]\n`);
  } else {
    console.error(`${patch.out}\n`);
  }
  emitCommitAnalysisHint();
}

function emitCommitAnalysis(root: string): void {
  emitCommitLog(root);
  console.error("\n");
  emitCommitDiff(root);
}

export function runCommit(root: string, segments: string[]): void {
  root = gitRepoRoot(root);
  const cmd = segments[0] ?? "prepare";
  if (!branchAllowed(root)) {
    console.error("commit: branch must contain ⛳wip or 🏗️dev");
    process.exit(1);
  }
  const contributor = findContributor(root);
  if (!contributor) {
    console.error(`commit: no contributor for git user.email ${gitEmail(root) || "(unset)"}`);
    process.exit(1);
  }

  if (cmd === "log") {
    emitCommitLog(root);
    process.exit(0);
  }

  if (cmd === "diff") {
    emitCommitDiff(root);
    process.exit(0);
  }

  if (cmd === "analyze") {
    emitCommitAnalysis(root);
    process.exit(0);
  }

  if (cmd === "check") {
    const dash = segments.indexOf("--");
    const bodyFile = dash >= 0 ? (segments[dash + 1] ?? null) : null;
    const body = readBodyInput(root, bodyFile);
    if (!body.trim()) {
      console.error("commit check: pass bundle body on stdin or after -- body.txt");
      process.exit(1);
    }
    const bodyErr = commitBundleBodyError(body);
    if (bodyErr) {
      console.error(bodyErr);
      process.exit(1);
    }
    const wip = findLastBundleWipCommit(root);
    if (!wip) {
      console.error("commit: no prior bundle WIP commit (subject …🔀) found");
      process.exit(1);
    }
    const ahead = git(root, ["rev-list", "--count", `${wip.sha}..HEAD`]);
    if (!ahead.ok || Number(ahead.out) === 0) {
      console.error("commit: no commits after last bundle WIP — nothing to check");
      process.exit(1);
    }
    try {
      const bundles = parseCommitBundleBody(body);
      validateBundleBulletsFresh(root, wip.sha, "HEAD", bundles);
      validateBundleCommitAttribution(root, wip.sha, "HEAD", bundles);
    } catch (e) {
      console.error(e instanceof Error ? e.message : String(e));
      process.exit(1);
    }
    const range = rangeGitDeltaTotal(root, wip.sha, "HEAD");
    console.error(`commit check: OK — ${formatGitDeltaSumBrief(range)}`);
    console.error("commit check: per-bundle days → bundle headers → WIP range total; per-language footer → same total");
    process.exit(0);
  }

  if (cmd !== "prepare") {
    console.error("[commit] usage: bun ./script.ts commit <log|diff|analyze|check|prepare> [ct|cs|cp|…] [-- body.txt]");
    process.exit(1);
  }

  const dash = segments.indexOf("--");
  const levelSegments = dash >= 0 ? segments.slice(1, dash) : segments.slice(1);
  const bodyFile = dash >= 0 ? (segments[dash + 1] ?? null) : null;
  const steps = loadCommitSteps(root, contributor, levelSegments);
  const prepareOnly = isCommitPrepareOnly(levelSegments);
  const messagePath = join(gitDir(root), "compose-commit-message");

  const wip = findLastBundleWipCommit(root);
  if (!wip) {
    console.error("commit: no prior bundle WIP commit (subject …🔀) found");
    process.exit(1);
  }

  const ahead = git(root, ["rev-list", "--count", `${wip.sha}..HEAD`]);
  if (!ahead.ok || Number(ahead.out) === 0) {
    console.error("commit: no commits after last bundle WIP — nothing to bundle");
    process.exit(1);
  }

  const body = readBodyInput(root, bodyFile);
  let message: string;
  if (body.trim()) {
    const bodyErr = commitBundleBodyError(body);
    if (bodyErr) {
      console.error(bodyErr);
      process.exit(1);
    }
    let bundles: CommitBundleSection[];
    try {
      bundles = parseCommitBundleBody(body);
      validateBundleBulletsFresh(root, wip.sha, "HEAD", bundles);
    } catch (e) {
      console.error(e instanceof Error ? e.message : String(e));
      process.exit(1);
    }
    try {
      message = buildCommitMessage(root, contributor, bundles, wip.sha);
    } catch (e) {
      console.error(e instanceof Error ? e.message : String(e));
      process.exit(1);
    }
    writeFileSync(messagePath, message);
  } else if (existsSync(messagePath)) {
    message = readFileSync(messagePath, "utf8");
    if (!message.trim()) {
      console.error("commit: compose-commit-message is empty — run prepare with bundle body first");
      process.exit(1);
    }
  } else {
    emitCommitAnalysis(root);
    process.exit(1);
  }

  if (prepareOnly) {
    if (!body.trim()) {
      emitCommitAnalysis(root);
      process.exit(1);
    }
    const tagName = formatBundleTagName(contributor);
    emitStdout(
      formatCommitPrepareAgentReply({
        tagName,
        wipSha: wip.sha,
        messageFile: ".git/compose-commit-message",
        commitMessage: message,
      }),
    );
    process.exit(0);
  }

  if (steps.tag || steps.squash || steps.push) assertCleanWorktree(root);

  if (steps.tag) {
    const tagName = formatBundleTagName(contributor);
    const tag = spawnSync("git", ["tag", "-s", "-m", tagName, tagName, "HEAD"], { cwd: root, encoding: "utf8" });
    if (tag.status !== 0) {
      console.error((tag.stderr ?? tag.stdout ?? "git tag failed").trim());
      process.exit(tag.status ?? 1);
    }
  }

  if (steps.squash) {
    const reset = spawnSync("git", ["reset", "--soft", wip.sha], { cwd: root, encoding: "utf8" });
    if (reset.status !== 0) {
      console.error((reset.stderr ?? reset.stdout ?? "git reset --soft failed").trim());
      process.exit(reset.status ?? 1);
    }
    writeFileSync(join(gitDir(root), "COMMIT_EDITMSG"), message);
    const commit = spawnSync("git", ["commit", "-S", "-F", join(gitDir(root), "COMMIT_EDITMSG")], {
      cwd: root,
      encoding: "utf8",
    });
    if (commit.status !== 0) {
      console.error((commit.stderr ?? commit.stdout ?? "git commit failed").trim());
      process.exit(commit.status ?? 1);
    }
  }

  if (steps.push) {
    const push = spawnSync("git", ["push", "--follow-tags"], { cwd: root, encoding: "utf8" });
    if (push.status !== 0) {
      console.error((push.stderr ?? push.stdout ?? "git push failed").trim());
      process.exit(push.status ?? 1);
    }
  }

  emitStdout(message);
  process.exit(0);
}

//#endregion 🔖commit

//#region 📻SVG Export
/** 📻Exports an animated SVG to MP4 using Playwright and FFmpeg */
export async function exportAnimatedSvgToMp4(inputSvgPath: string, outputMp4Path: string, options?: { fps?: number; durationSeconds?: number; width?: number; height?: number }): Promise<void> {
  const fps = options?.fps ?? 60;
  let durationSeconds = options?.durationSeconds;
  const { readFileSync } = await import("node:fs");
  const { resolve } = await import("node:path");

  if (!durationSeconds) {
    const content = readFileSync(inputSvgPath, "utf-8");
    const durMatch = content.match(/dur="([^"]+)s"/);
    if (durMatch) {
      durationSeconds = parseFloat(durMatch[1]);
    } else {
      durationSeconds = 10;
    }
  }

  const { chromium } = await import("playwright");
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();

  const svgUrl = `file://${resolve(inputSvgPath)}`;
  await page.goto(svgUrl);
  await page.waitForSelector("svg");

  const bbox = await page.evaluate(() => {
    const svg = document.querySelector("svg");
    if (!svg) return { width: 1920, height: 1080 };
    return {
      width: svg.viewBox.baseVal?.width || svg.width.baseVal?.value || 1920,
      height: svg.viewBox.baseVal?.height || svg.height.baseVal?.value || 1080,
    };
  });

  const width = options?.width ?? Math.round(bbox.width);
  const height = options?.height ?? Math.round(bbox.height);

  const w = width % 2 === 0 ? width : width + 1;
  const h = height % 2 === 0 ? height : height + 1;

  await page.setViewportSize({ width: w, height: h });

  await page.evaluate(() => {
    const svg = document.querySelector("svg") as any;
    if (svg && svg.pauseAnimations) svg.pauseAnimations();
  });

  const totalFrames = fps * durationSeconds;

  const { spawn } = await import("node:child_process");
  const ffmpeg = spawn("ffmpeg", ["-y", "-f", "image2pipe", "-vcodec", "png", "-r", fps.toString(), "-i", "-", "-c:v", "libx264", "-pix_fmt", "yuv420p", outputMp4Path]);

  for (let i = 0; i <= totalFrames; i++) {
    const time = i / fps;
    await page.evaluate((t) => {
      const svg = document.querySelector("svg") as any;
      if (svg && svg.setCurrentTime) svg.setCurrentTime(t);
    }, time);
    const buffer = await page.screenshot({ omitBackground: true });
    ffmpeg.stdin.write(buffer);
  }
  ffmpeg.stdin.end();

  await new Promise<void>((resolve, reject) => {
    ffmpeg.on("close", (code) => {
      if (code === 0) resolve();
      else reject(new Error(`ffmpeg exited with code ${code}`));
    });
    ffmpeg.on("error", reject);
  });

  await browser.close();
}
//#endregion 📻SVG Export
