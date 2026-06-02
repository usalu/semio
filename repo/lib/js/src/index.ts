//#region 🧲Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — @repo/lib/js: bundle scripts, policy runner, linters, dependency-boundary lint.
//#endregion 🧲Header

//#region 🔌Adapters
import { execFileSync, spawn, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, normalize, relative, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
//#endregion 🔌Adapters

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
/** 🔎Resolves monorepo root (directory containing root package.json named `semio`). */
export function getWorkspaceRoot(): string {
  const fromEnv = process.env.REPO_ROOT?.trim();
  if (fromEnv) return resolve(fromEnv);
  let dir = process.cwd();
  for (let i = 0; i < 30; i++) {
    const pkg = join(dir, "package.json");
    if (existsSync(pkg)) {
      try {
        const j = JSON.parse(readFileSync(pkg, "utf8")) as { name?: string };
        if (j.name === "semio") return dir;
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
export function runCliGraphql(
  query: string,
  variables: Record<string, unknown> = {},
  options?: { cwd?: string; repoRoot?: string },
): unknown {
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

/** 🧭Package dir for `@repo/lib` (…/repo/lib/js). */
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
      id path name extension kind
      sections { id name path range { start end } }
      definitions { id name kind range { start end } }
    }
    ... on Folder {
      id path name
    }
    ... on Bundle {
      id name root kind technologyName
    }
    ... on Technology {
      id name kind root
    }
    ... on Section {
      id name path
      range { start end }
      file { path }
      definitions { id name kind range { start end } }
    }
    ... on Definition {
      id name kind
      range { start end }
      file { path }
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
    if (!this.node) this.node = this.loadNode();
    if (this.node.__typename !== "Technology") {
      throw new Error(`[TechnologyLinter] expected Technology, got ${this.node.__typename}`);
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
    const data = this.gql<{ bundles: GraphNode[] }>(
      `query B { bundles { id name root kind technologyName } }`,
    );
    const tech = this.name();
    return (data.bundles ?? []).filter((b) => String(b.technologyName ?? "") === tech);
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
    return String(this.load().kind ?? "");
  }

  technologyName(): string {
    return String(this.load().technologyName ?? "");
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
    return String(this.load().kind ?? "");
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
    const f = this.load().file as GraphNode | undefined;
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
    const f = this.load().file as GraphNode | undefined;
    return String(f?.path ?? "").replaceAll("\\", "/");
  }

  name(): string {
    return String(this.load().name ?? "");
  }

  kind(): string {
    return String(this.load().kind ?? "");
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
  const data = runCliGraphql(
    `query F($p: String!) { folder(path: $p) { __typename id path name } }`,
    { p: rel },
    { repoRoot },
  ) as { folder: GraphNode };
  if (!data.folder?.id) throw new Error(`[linter] folder not found for path ${rel}`);
  return data.folder;
}

/** 🔎Resolves bundle name like `repo/client` to bundle id. */
export function resolveBundleByName(repoRoot: string, name: string): GraphNode {
  const data = runCliGraphql(
    `query B($n: String!) { bundle(name: $n) { __typename id name root kind technologyName } }`,
    { n: name },
    { repoRoot },
  ) as { bundle: GraphNode };
  if (!data.bundle?.id) throw new Error(`[linter] bundle not found for name ${name}`);
  return data.bundle;
}

/** 🔎Resolves technology folder name (e.g. `repo`) to technology id. */
export function resolveTechnologyByName(repoRoot: string, name: string): GraphNode {
  const data = runCliGraphql(
    `query T { technologies { id name root kind } }`,
    {},
    { repoRoot },
  ) as { technologies: GraphNode[] };
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

const INTERNAL_PREFIXES = [
  "@semio/",
  "@ui/",
  "@cad/",
  "@puzzle/",
  "@framework/",
  "@repo/",
  "@coda/",
];

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
export function dependencyBoundaryBreachesForFile(
  repoRoot: string,
  filePath: string,
  content: string,
  scope: string,
): BreachRecord[] {
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
        solution:
          "Move the import into a //#region 🔌Adapter (or /adapters/) module and depend on a first-party port interface elsewhere",
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

export type ResolvedLintEntity =
  | { kind: "file"; id: string; path: string }
  | { kind: "folder"; id: string }
  | { kind: "bundle"; id: string }
  | { kind: "technology"; id: string };

/** 🔎Maps `script.ts` directory to bundle, technology, or folder entity id. */
export function resolvePolicyScriptEntity(repoRoot: string, scriptPath: string): ResolvedLintEntity {
  const dir = dirname(scriptPath);
  const relDir = norm(relative(repoRoot, dir));
  const folder = runCliGraphql(`query Fo($p: String!) { folder(path: $p) { id path } }`, { p: relDir }, { repoRoot }) as {
    folder: { id?: string; path?: string };
  };
  if (!folder.folder?.id) throw new Error(`[policy-runner] folder not resolved for ${relDir}`);

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
    if (norm(String(t.root ?? "")) === d) {
      return { kind: "technology", id: String(t.id) };
    }
  }
  return { kind: "folder", id: String(folder.folder.id) };
}

export async function runPolicyScript(scriptPath: string, repoRoot = getWorkspaceRoot()): Promise<{
  entityId: string;
  breachs: BreachRecord[];
  cachePath: string;
}> {
  const absScript =
    scriptPath.includes(":") || scriptPath.startsWith("/") || /^[A-Za-z]:\\/.test(scriptPath)
      ? scriptPath
      : join(repoRoot, scriptPath);
  const base = basename(absScript);
  if (base !== "script.ts") {
    throw new Error(`[policy-runner] expected script.ts, got ${base}`);
  }

  const policyFile = parsePolicyFileExport(absScript);
  let entity: ResolvedLintEntity;
  if (policyFile) {
    const target = join(dirname(absScript), policyFile).replaceAll("\\", "/");
    entity = { kind: "file", id: fileEntityId(repoRoot, target), path: target };
  } else {
    entity = resolvePolicyScriptEntity(repoRoot, absScript);
  }

  const href = pathToFileURL(absScript).href;
  const mod = (await import(href)) as LintScriptModule;
  const fn = mod.policy;
  if (typeof fn !== "function") {
    throw new Error(`[policy-runner] ${absScript} must export const policy = defineLint(...)`);
  }

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
  await runPolicyExit(fileURLToPath(scriptUrl));
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
export async function runBundleScriptMain(
  router: ScriptRouter,
  scriptUrl: string,
  opts: RunBundleScriptMainOptions = {},
): Promise<void> {
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
export function dispatchSubcommand(
  segments: string[],
  handlers: Record<string, (rest: string[]) => void | Promise<void>>,
  usage: string,
  defaultKey?: string,
): void | Promise<void> {
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
  let dir = start;
  for (let i = 0; i < 32; i++) {
    if (existsSync(join(dir, "nx.json")) && existsSync(join(dir, "package.json"))) return dir;
    const parent = dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  return getWorkspaceRoot();
}
//#endregion 🔖Router

//#region 🔖Process
/** 🏃Runs a subprocess with inherited stdio; throws on non-zero exit. */
export function runCmd(cmd: string, args: string[], opts: { cwd?: string; env?: NodeJS.ProcessEnv } = {}): void {
  execFileSync(cmd, args, {
    stdio: "inherit",
    cwd: opts.cwd,
    env: opts.env ?? process.env,
  });
}

/** 🏃Like `runCmd` but ignores failures. */
export function tryRun(cmd: string, args: string[], opts: { cwd?: string; env?: NodeJS.ProcessEnv } = {}): void {
  try {
    runCmd(cmd, args, opts);
  } catch {
    /* optional */
  }
}

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
export function runViteDev(
  bundleRoot: string,
  segments: string[],
  opts: { config: string; portEnv?: string; defaultPort?: string },
): void {
  const env = playPollingEnv();
  const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
  const port = process.env[opts.portEnv ?? "VITE_PORT"] ?? opts.defaultPort ?? "5173";
  spawnBun(
    ["run", "vite", "--config", opts.config, "--host", host, "--port", port, ...segments],
    bundleRoot,
    env,
  );
}

/** ▶️Vite production build. */
export function runViteBuild(bundleRoot: string, segments: string[], config: string): void {
  runBun(["run", "vite", "build", "--config", config, ...segments], bundleRoot, devToolingEnv());
}

/** ▶️Vitest run in bundle directory. */
export function runVitest(bundleRoot: string, segments: string[], config = "vitest.config.ts"): void {
  runBunx(["vitest", "run", "--config", config, "--passWithNoTests", ...segments], bundleRoot, devToolingEnv());
}

/** 🧰Play/vite dev env with optional file-watcher polling defaults. */
export function playPollingEnv(extra: NodeJS.ProcessEnv = {}): NodeJS.ProcessEnv {
  return devToolingEnv({
    ...(process.env.WATCHPACK_POLLING !== undefined
      ? {}
      : { WATCHPACK_POLLING: "true", CHOKIDAR_USEPOLLING: "true" }),
    ...extra,
  });
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

/** ▶️Vite dev via `bunx` with root-level `vite.config.ts`. */
export function runViteBunxDev(
  bundleRoot: string,
  segments: string[],
  opts: {
    portEnv?: string;
    defaultPort?: string;
    clearViteCache?: boolean;
    strictPort?: boolean;
  } = {},
): void {
  const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
  const port = process.env[opts.portEnv ?? "VITE_PORT"] ?? opts.defaultPort ?? "5173";
  const portInUse = isDevPortInUse(host, Number(port));
  if (opts.clearViteCache) {
    if (portInUse) {
      console.error(
        `[dev] Port ${port} is already in use; skipping Vite cache clear to avoid 504 Outdated Optimize Dep. Stop the existing dev server and restart.`,
      );
    } else {
      const viteCache = join(bundleRoot, "node_modules", ".vite");
      if (existsSync(viteCache)) rmSync(viteCache, { recursive: true, force: true });
    }
  }
  const wantStrictPort = opts.strictPort ?? true;
  const viteArgs = ["vite", "--config", "vite.config.ts", "--host", host, "--port", port];
  if (wantStrictPort && !segments.includes("--strictPort") && !segments.includes("--no-strictPort")) {
    viteArgs.push("--strictPort");
  }
  spawnBunx([...viteArgs, ...segments], bundleRoot, playPollingEnv());
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

/** 📦`wasm-pack build` for `--target web`, restores `pkg/package.json`, verifies wasm output. */
export function runWasmPackWebBuild(opts: {
  rsDir: string;
  skipEnvVar: string;
  logPrefix: string;
  pkg: WasmPackWebPkg;
  wasmBaseName: string;
}): void {
  const { rsDir, skipEnvVar, logPrefix, pkg, wasmBaseName } = opts;
  if (process.env[skipEnvVar] === "1") {
    console.log(`[${logPrefix}] ${skipEnvVar}=1 → skipping wasm-pack build`);
    return;
  }
  console.log(`[${logPrefix}] wasm-pack build --release --target web --out-dir pkg --no-pack`);
  const t0 = Date.now();
  const res = spawnSync(
    "bun",
    ["x", "wasm-pack", "build", "--release", "--target", "web", "--out-dir", "pkg", "--no-pack"],
    { cwd: rsDir, stdio: "inherit" },
  );
  if (res.status !== 0) {
    console.error(`[${logPrefix}] wasm-pack build failed`);
    process.exit(res.status ?? 1);
  }
  console.log(`[${logPrefix}] wasm-pack build done in ${((Date.now() - t0) / 1000).toFixed(1)}s`);

  const pkgDir = join(rsDir, "pkg");
  if (!existsSync(pkgDir)) mkdirSync(pkgDir, { recursive: true });
  const pkgJson = {
    type: "module",
    version: pkg.version ?? "0.1.0",
    sideEffects: pkg.sideEffects ?? ["./snippets/*"],
    ...pkg,
  };
  writeFileSync(join(pkgDir, "package.json"), `${JSON.stringify(pkgJson, null, 2)}\n`, "utf8");

  const wasmPath = join(pkgDir, `${wasmBaseName}_bg.wasm`);
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

export {
  bumpCounterFromHistory,
  bumpCounterFromSubject,
  buildMicroCommitMessage,
  digestMicroCommitMessage,
  extractCounterFromSubject,
  installMicroCommitGitHooks,
  parseCachedDiff,
  resetMicroCommitTemplates,
  runMicroCommit,
  shouldRefreshPreparedCommitMessage,
  summarizeFileChange,
  writeMicroCommitTemplates,
} from "./micro-commit.ts";
export type { CachedFileDiff } from "./micro-commit.ts";
export type { MicroCommitLevel } from "./micro-commit.ts";

