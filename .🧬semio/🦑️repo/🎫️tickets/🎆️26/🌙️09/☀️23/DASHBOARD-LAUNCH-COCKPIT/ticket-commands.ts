#!/usr/bin/env bun
/**
 * 🎮️ Ticket command manifests: moves the ticket-scoped rows of `.vscode/launch.json` into one
 * `🎮️commands.json` per owning open ticket (fleet-plan.md §2.1/§2.2), drops dead rows and proves that every
 * written tool re-expands to the argv, cwd and meaningful environment of the launch row it replaces.
 *
 * Ticket input file of DASHBOARD-LAUNCH-COCKPIT (slice M-1b). It is not part of the codebase and nothing in
 * the codebase may reference it. Re-runnable against the live launch files (repository root):
 *
 *   bun "<ticket>/ticket-commands.ts"            analyse, prove, validate, write manifests + coverage log
 *   bun "<ticket>/ticket-commands.ts" --check      the same without touching any ticket folder (manifests go to the preview folder)
 *   bun "<ticket>/ticket-commands.ts" --self-test  checks the shell reader, the proof, the schema and the merge against known cases
 *
 * Exit code 0 only when every written tool is proven equivalent and every manifest validates.
 *
 * @see ./fleet-plan.md
 * @see ./launch-inventory.md
 * @see ./m1b-ticket-commands.md
 */
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

//#region constants
export const TICKET_DIR = import.meta.dirname;
export const ROOT = resolve(TICKET_DIR, "../../../../../../..");
export const GEN = join(TICKET_DIR, "🗑️generated", "ticket-commands");
export const VS16 = "\uFE0F";
export const WS_VAR = "${workspaceFolder}";
export const WS_TOKEN = "{workspace}";
export const MANIFEST_NAME = "🎮️commands.json";
export const VARIANT = "variant";
export const FAMILY_MINIMUM = 3;
/** 🗂️ Verb list of the dashboard command tree (`🌳️command-tree/🦀️.rs` `VERBS`), in launcher order. */
export const VERBS = ["setup", "start", "dev", "serve", "watch", "activate", "prepare", "run", "build", "package", "test", "smoke", "check", "typecheck", "verify", "gate", "lint", "format", "generate", "publish", "deploy", "preview", "bench", "clean"];
export const FALLBACK_VERB = "task";
/** 🔇️ Runner noise the `bun nx` wrapper already defaults (launch-inventory.md §6); meaningful only where the wrapper is bypassed. */
export const NOISE_ENV = new Set(["NX_DAEMON", "NX_ISOLATE_PLUGINS", "NX_TUI", "FORCE_COLOR", "NX_CACHE_PROJECT_GRAPH"]);
/** 📤️ Environment values that name a location the probe writes to; they need not exist before the run. */
export const OUTPUT_ENV = new Set(["NX_CACHE_DIRECTORY", "NX_WORKSPACE_DATA_DIRECTORY", "SEMIO_TEST_ARTIFACT_DIR", "SEMIO_TEST_ARTIFACTS_DIR", "SEMIO_TEST_ARTIFACTS_ROOT", "CARGO_TARGET_DIR", "CARGO_BUILD_BUILD_DIR", "PLAYWRIGHT_BROWSERS_PATH", "SEMIO_TEST_OUTPUT_SCOPE"]);
/** 📤️ Flags whose value is a location the probe writes to. */
export const OUTPUT_FLAG = /^--?(?:out|output|out-dir|outDir|output-dir|artifacts?|report|receipts?|log)$/;
export type Json = any;
//#endregion

//#region text helpers
export const readText = (path: string): string => readFileSync(path, "utf8");
export const sha256 = (text: string): string => createHash("sha256").update(text).digest("hex");
export const uniq = <T>(items: readonly T[]): T[] => [...new Set(items)];
export const compare = (a: string, b: string): number => (a < b ? -1 : a > b ? 1 : 0);
export const tally = (items: readonly string[]): Record<string, number> => { const counts: Record<string, number> = {}; for (const item of items) counts[item] = (counts[item] ?? 0) + 1; return Object.fromEntries(Object.entries(counts).sort((a, b) => b[1] - a[1] || compare(a[0], b[0]))); };
/** 🧼️ Emoji directory names are written with and without U+FE0F; comparisons ignore it. */
export const bare = (text: string): string => text.replaceAll(VS16, "");
export const kebab = (text: string): string => text.replace(/([a-z0-9])([A-Z])/g, "$1-$2").toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "");
export const relRoot = (path: string): string => (path === ROOT ? "" : path.startsWith(ROOT + "/") ? path.slice(ROOT.length + 1) : path);
const escapeRegExp = (text: string): string => text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
/** 🧭️ Resolves a path below `root` by listing, so a typed emoji segment matches its on-disk spelling. */
export function find(root: string, segments: string[]): string | null {
  let level = root;
  for (const segment of segments) {
    const entry = existsSync(level) ? readdirSync(level).find((name) => bare(name) === bare(segment)) : undefined;
    if (!entry) return null;
    level = join(level, entry);
  }
  return level;
}
//#endregion

//#region shell reading
export type Word = { text: string; raw: string; hazards: string[] };
export type Redirect = { fd: number; operator: string; target: string; duplicate?: number };
export type Shell = { assignments: [string, string][]; argv: string[]; redirects: Redirect[]; unsupported: string[] };
const CONTROL_OPERATORS = ["&&", "||", ";;", ";", "|&", "|", "&", "(", ")"];
/**
 * 🐚️ Reads one simple POSIX shell command (assignments, words, redirections) without executing anything.
 * Everything a shell would expand or chain (`$`, backticks, globs, braces, tilde, comments, history, control
 * operators) is reported as unsupported instead of being interpreted; `/bin/sh` and `/bin/zsh` confirm the
 * reading in {@link oracle}.
 */
export function readShell(source: string): Shell {
  const words: (Word | { redirect: string; fd?: number })[] = [];
  const unsupported: string[] = [];
  const n = source.length;
  let i = 0;
  while (i < n) {
    while (i < n && /\s/.test(source[i]!)) i++;
    if (i >= n) break;
    if (source.startsWith("&>", i)) { unsupported.push("operator &>"); i += 2; continue; }
    const redirect = source.slice(i).match(/^(\d*)(>>|>&|<&|>\||<>|>|<)/);
    if (redirect) { words.push({ redirect: redirect[2]!, fd: redirect[1] === "" ? undefined : Number(redirect[1]) }); i += redirect[0].length; continue; }
    const control = CONTROL_OPERATORS.find((operator) => source.startsWith(operator, i));
    if (control) { unsupported.push(`operator ${control}`); i += control.length; continue; }
    const word: Word = { text: "", raw: "", hazards: [] };
    const start = i;
    while (i < n && !/\s/.test(source[i]!)) {
      const c = source[i]!;
      if (c === "'") {
        const end = source.indexOf("'", i + 1);
        if (end === -1) { unsupported.push("unterminated single quote"); i = n; break; }
        word.text += source.slice(i + 1, end); i = end + 1;
      } else if (c === '"') {
        i++;
        while (i < n && source[i] !== '"') {
          const d = source[i]!;
          if (d === "\\" && i + 1 < n && '"\\$`\n'.includes(source[i + 1]!)) { if (source[i + 1] !== "\n") word.text += source[i + 1]; i += 2; continue; }
          if (d === "$" || d === "`") word.hazards.push(`expansion ${d} in double quotes`);
          if (d === "!") word.hazards.push("history ! in double quotes");
          word.text += d; i++;
        }
        if (i >= n) unsupported.push("unterminated double quote");
        i++;
      } else if (c === "\\") {
        if (i + 1 >= n) { unsupported.push("trailing backslash"); i++; break; }
        if (source[i + 1] !== "\n") word.text += source[i + 1];
        i += 2;
      } else if (/[<>|&;()]/.test(c)) break;
      else {
        if (c === "$" || c === "`") word.hazards.push(`expansion ${c}`);
        if ("*?[]".includes(c)) word.hazards.push(`glob ${c}`);
        if ("{}".includes(c)) word.hazards.push(`brace ${c}`);
        if (c === "!" || c === "^") word.hazards.push(`history ${c}`);
        if ((c === "~" || c === "#" || c === "=") && i === start) word.hazards.push(`leading ${c}`);
        word.text += c; i++;
      }
    }
    word.raw = source.slice(start, i);
    unsupported.push(...word.hazards);
    words.push(word);
  }
  const shell: Shell = { assignments: [], argv: [], redirects: [], unsupported };
  const isWord = (item: (typeof words)[number] | undefined): item is Word => !!item && "text" in item;
  let k = 0;
  for (; k < words.length; k++) {
    const item = words[k];
    const name = isWord(item) ? item.raw.match(/^([A-Za-z_][A-Za-z0-9_]*)=/)?.[1] : undefined;
    if (!isWord(item) || !name) break;
    shell.assignments.push([name, item.text.slice(name.length + 1)]);
  }
  for (; k < words.length; k++) {
    const item = words[k]!;
    if (isWord(item)) { shell.argv.push(item.text); continue; }
    const target = words[k + 1];
    if (!isWord(target)) { unsupported.push(`redirection ${item.redirect} without target`); continue; }
    k++;
    const fd = item.fd ?? (item.redirect.startsWith("<") ? 0 : 1);
    if (item.redirect === ">&" || item.redirect === "<&") {
      if (/^\d+$/.test(target.text)) shell.redirects.push({ fd, operator: item.redirect, target: "", duplicate: Number(target.text) });
      else unsupported.push(`redirection ${item.redirect}${target.text}`);
    } else shell.redirects.push({ fd, operator: item.redirect, target: target.text });
  }
  return shell;
}
//#endregion

//#region sources
export type RowText = { command: string; cwd: string; env: Record<string, string> };
export type LaunchRow = RowText & { name: string; group: string | null; ready: Json | null; origin: "launch" | "seed-only"; index: number; original: RowText; rewritten: boolean };
export type Sources = { launchPath: string; seedPath: string | null; launchSha: string; seedSha: string | null; rows: LaunchRow[]; inputs: Map<string, Json>; compounds: Json[]; launchRows: number; seedOnlyRows: number };
const ROOT_LITERAL = new RegExp(`(?<![A-Za-z0-9_.\\-/])${escapeRegExp(ROOT)}(?=[/"'\\s]|$)`, "g");
/** 🧳️ A hard-coded workspace root is the same directory VS Code substitutes for `${workspaceFolder}`. */
export const portable = (text: string): string => text.replace(ROOT_LITERAL, WS_VAR);
export const seedFile = (directory: string): string | null => { const name = existsSync(directory) ? readdirSync(directory).find((entry) => bare(entry).endsWith("launch.seed.jsonc")) : undefined; return name ? join(directory, name) : null; };
function toRow(row: Json, index: number, origin: LaunchRow["origin"]): LaunchRow {
  const original: RowText = { command: String(row.command ?? ""), cwd: typeof row.cwd === "string" ? row.cwd : WS_VAR, env: Object.fromEntries(Object.entries<Json>(row.env ?? {}).map(([key, value]) => [key, String(value)])) };
  const text: RowText = { command: portable(original.command), cwd: portable(original.cwd), env: Object.fromEntries(Object.entries(original.env).map(([key, value]) => [key, portable(value)])) };
  return { ...text, name: String(row.name), group: row.presentation?.group ?? null, ready: row.serverReadyAction ?? null, origin, index, original, rewritten: JSON.stringify(text) !== JSON.stringify(original) };
}
/** 📥️ Reads the launch file and its seed; seed rows that the launch file does not carry yet are appended. */
export function readSources(launchPath: string, seedPath: string | null): Sources {
  const launchText = readText(launchPath);
  const launch: Json = Bun.JSONC.parse(launchText);
  const rows: LaunchRow[] = (launch.configurations as Json[]).filter((row) => row && typeof row === "object" && row.type === "node-terminal").map((row, index) => toRow(row, index, "launch"));
  const inputs = new Map<string, Json>(((launch.inputs as Json[]) ?? []).map((input) => [input.id, input]));
  const names = new Set(rows.map((row) => row.name));
  let seedSha: string | null = null, seedOnlyRows = 0;
  if (seedPath && existsSync(seedPath)) {
    const seedText = readText(seedPath);
    seedSha = sha256(seedText);
    const seed: Json = Bun.JSONC.parse(seedText);
    for (const input of (seed.inputs as Json[]) ?? []) if (!inputs.has(input.id)) inputs.set(input.id, input);
    for (const row of (seed.configurations as Json[]).filter((item) => item && typeof item === "object" && item.type === "node-terminal")) {
      if (names.has(row.name)) continue;
      rows.push(toRow(row, rows.length, "seed-only"));
      names.add(row.name);
      seedOnlyRows++;
    }
  }
  return { launchPath, seedPath, launchSha: sha256(launchText), seedSha, rows, inputs, compounds: launch.compounds ?? [], launchRows: rows.length - seedOnlyRows, seedOnlyRows };
}
//#endregion

//#region tickets
export const TICKET_ROOT = resolve(TICKET_DIR, "../../../..");
const TICKET_REF = /tickets\/([^\/\s'"]+)\/([^\/\s'"]+)\/([^\/\s'"]+)\/([^\/\s'"]+)/g;
const digits = (text: string): string => text.match(/\d+/)?.[0] ?? text;
export const ticketIdsIn = (text: string): string[] => uniq([...text.matchAll(TICKET_REF)].map((m) => `${digits(m[1]!)}/${digits(m[2]!)}/${digits(m[3]!)}/${m[4]}`));
export type Ticket = { id: string; folder: string | null; status: string; title: string | null; manifest: string | null };
const ticketCache = new Map<string, Ticket>();
/** 🎫️ Resolves a ticket id to its folder by listing (the emoji segments carry U+FE0F, stray spellings exist) and reads its status. */
export function ticket(id: string): Ticket {
  const hit = ticketCache.get(id);
  if (hit) return hit;
  const [year, month, day, slug] = id.split("/") as [string, string, string, string];
  let levels = [TICKET_ROOT];
  for (const want of [year, month, day]) levels = levels.flatMap((level) => readdirSync(level, { withFileTypes: true }).filter((entry) => entry.isDirectory() && !/^[A-Za-z.]/.test(entry.name) && digits(entry.name) === want).map((entry) => join(level, entry.name)));
  const folder = levels.map((level) => join(level, slug)).filter((path) => existsSync(path)).sort((a, b) => b.split(VS16).length - a.split(VS16).length || compare(a, b))[0] ?? null;
  const info: Ticket = { id, folder: folder ? relRoot(folder) : null, status: "missing", title: null, manifest: null };
  if (folder) {
    const entries = readdirSync(folder);
    const file = entries.find((entry) => bare(entry) === bare("🎫️ticket.json"));
    info.status = "no-ticket-json";
    if (file) try { const json = JSON.parse(readText(join(folder, file))); info.status = String(json.status ?? "unknown"); info.title = json.title ?? null; } catch { info.status = "unreadable"; }
    info.manifest = relRoot(join(folder, entries.find((entry) => bare(entry) === bare(MANIFEST_NAME)) ?? MANIFEST_NAME));
  }
  ticketCache.set(id, info);
  return info;
}
//#endregion

//#region contract
export type Contract = { path: string | null; sha: string | null; forbiddenEnv: Set<string>; idLimit: number; tool: (tool: Json) => string[]; document: (document: Json) => string[] };
/** 📜️ The registry schema authored by slice A-1: what a ticket document may contain. Absent schema: nothing is checked. */
export async function contract(): Promise<Contract> {
  const path = find(ROOT, ["🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🎛️dashboard", "🧬️schema", "🎮️registry", "🔣️.json"]);
  if (!path) return { path: null, sha: null, forbiddenEnv: new Set(), idLimit: Number.POSITIVE_INFINITY, tool: () => [], document: () => [] };
  const text = readText(path);
  const schema = JSON.parse(text);
  const { default: Ajv } = await import("ajv/dist/2020");
  const ajv = new Ajv({ allErrors: true, strict: false });
  const validateDocument = ajv.compile(schema);
  const validateTool = ajv.compile({ $ref: `${schema.$id}#/$defs/Tool` });
  const errors = (validate: Json, value: Json): string[] => (validate(value) ? [] : uniq((validate.errors as Json[]).map((error) => `${error.instancePath || "/"} ${error.message}`)));
  return {
    path: relRoot(path), sha: sha256(text),
    forbiddenEnv: new Set<string>(schema.$defs?.EnvName?.not?.enum ?? []),
    idLimit: schema.$defs?.Slug?.maxLength ?? Number.POSITIVE_INFINITY,
    tool: (tool) => errors(validateTool, tool), document: (document) => errors(validateDocument, document),
  };
}
//#endregion

//#region nx workspaces
export type Project = { name: string; root: string; targets: Map<string, string[]>; cargo: boolean; cargoComponent: boolean; testCommand?: string };
export type Workspace = { root: string; projects: Map<string, Project>; graph: Map<string, { root: string; targets: Map<string, string[]> }>; directories: number; rootCommands: Set<string> };
const WALK_SKIP = new Set(["node_modules", ".git", "target", "dist", "temp", "storybook-static", "pkg", ".nx"]);
const MANIFESTS = new Set([bare("📋️project.json"), "project.json"]);
/** 🚶️ Declared Nx projects below `root`: every `📋️project.json`/`project.json` plus Cargo packages the inference plugin turns into projects. */
export function walkWorkspace(root: string, main: boolean): Workspace {
  const workspace: Workspace = { root, projects: new Map(), graph: new Map(), directories: 0, rootCommands: new Set() };
  const stack = [root];
  const cargo: string[] = [];
  const byDirectory = new Map<string, Project>();
  while (stack.length) {
    const directory = stack.pop()!;
    let entries;
    try { entries = readdirSync(directory, { withFileTypes: true }); } catch { continue; }
    workspace.directories++;
    for (const entry of entries) {
      if (entry.isDirectory() && !entry.isSymbolicLink()) {
        if (!(WALK_SKIP.has(entry.name) || (main && (entry.name.startsWith(".") || ["🤖generated", "🗑generated"].includes(bare(entry.name)))))) stack.push(join(directory, entry.name));
        continue;
      }
      if (!entry.isFile()) continue;
      if (entry.name === "Cargo.toml") cargo.push(directory);
      if (!MANIFESTS.has(bare(entry.name))) continue;
      let json: Json;
      try { json = JSON.parse(readText(join(directory, entry.name))); } catch { continue; }
      if (!json?.name) continue;
      const targets = new Map<string, string[]>(Object.entries<Json>(json.targets ?? {}).map(([name, value]) => [name, Object.keys(value?.configurations ?? {})]));
      const project: Project = { name: json.name, root: directory, targets, cargo: false, cargoComponent: false, testCommand: typeof json.targets?.test?.options?.command === "string" ? json.targets.test.options.command : undefined };
      workspace.projects.set(json.name, project);
      byDirectory.set(directory, project);
    }
  }
  for (const directory of cargo) {
    const text = readText(join(directory, "Cargo.toml"));
    const name = text.match(/^\[package\]\s*$([\s\S]*?)(?=^\[|(?![\s\S]))/m)?.[1]?.match(/^\s*name\s*=\s*"([^"]+)"/m)?.[1];
    if (!name) continue;
    const component = /component-kind\s*=\s*"(?:plugin|extension)"/.test(text) && /^\[package\.metadata\.component\]/m.test(text);
    const owner = byDirectory.get(directory) ?? workspace.projects.get(name);
    if (owner) { owner.cargo = true; owner.cargoComponent ||= component; continue; }
    workspace.projects.set(name, { name, root: directory, targets: new Map(), cargo: true, cargoComponent: component });
  }
  if (main) {
    const graphPath = join(root, ".nx/workspace-data/project-graph.json");
    if (existsSync(graphPath)) try {
      const graph = JSON.parse(readText(graphPath));
      for (const [name, node] of Object.entries<Json>(graph.nodes ?? {})) workspace.graph.set(name, { root: join(root, node.data?.root ?? ""), targets: new Map(Object.entries<Json>(node.data?.targets ?? {}).map(([target, value]) => [target, Object.keys(value?.configurations ?? {})])) });
    } catch {}
    const script = readdirSync(root).find((entry) => bare(entry) === bare("📜️script.ts"));
    if (script) for (const m of readText(join(root, script)).matchAll(/\.register\(\s*"([^"]+)"/g)) workspace.rootCommands.add(m[1]!);
  }
  return workspace;
}
const workspaces = new Map<string, Workspace | null>();
export const workspaceAt = (root: string): Workspace | null => {
  if (!workspaces.has(root)) workspaces.set(root, existsSync(root) ? walkWorkspace(root, root === ROOT) : null);
  return workspaces.get(root)!;
};
/** ⚖️ Whether `project[:target[:configuration]]` resolves in `workspace`: declared manifest, published Nx graph (project folder still present), or a known inference rule. */
export function resolveTarget(workspace: Workspace, name: string, target?: string, configuration?: string): { ok: boolean; how: string } {
  const project = workspace.projects.get(name);
  const node = workspace.graph.get(name);
  const graph = node && existsSync(node.root) ? node.targets : undefined;
  if (!project && !graph) return { ok: false, how: `missing-project ${name}` };
  if (target === undefined) return { ok: true, how: project ? "declared-project" : "graph-project" };
  const joined = configuration ? `${target}:${configuration}` : undefined;
  for (const [how, targets] of [["declared", project?.targets], ["graph", graph]] as const) {
    if (!targets) continue;
    if (joined && targets.has(joined)) return { ok: true, how };
    const configurations = targets.get(target);
    if (configurations && (!configuration || configurations.includes(configuration))) return { ok: true, how };
  }
  if (project && !configuration) {
    if (name === "workspace" && workspace.rootCommands.has(target)) return { ok: true, how: "rule root-command" };
    if (project.cargo && ["build", "check", "test"].includes(target)) return { ok: true, how: "rule cargo-target" };
    if (project.cargoComponent && (target === "describe" || /^(?:component|materialize)-(?:dev|release)$/.test(target))) return { ok: true, how: "rule component-target" };
    if (project.testCommand && !bare(project.testCommand).includes(bare("⚡️caching/🦀️cargo/📜️script.ts")) && /^test-(?:quick|long|exhaustive)$/.test(target)) return { ok: true, how: "rule leveled-test" };
  }
  return { ok: false, how: `missing-target ${name}:${target}${configuration ? ":" + configuration : ""}` };
}
//#endregion

//#region row analysis
const WS_MARK = "\uE000", INPUT_OPEN = "\uE001", INPUT_CLOSE = "\uE002";
const INPUT_VAR = /\$\{input:([^}]+)\}/g;
/** 🧷️ VS Code substitutes its variables before the shell reads the line; they travel through the shell reading as opaque marks. */
export const mark = (text: string): string => text.replaceAll(WS_VAR, WS_MARK).replace(INPUT_VAR, (_, id: string) => `${INPUT_OPEN}${id}${INPUT_CLOSE}`);
export const unmark = (text: string, workspace: string, input: (id: string) => string): string => text.replaceAll(WS_MARK, workspace).replace(/\uE001([^\uE002]*)\uE002/g, (_, id: string) => input(id));
export const inputsIn = (text: string): string[] => uniq([...text.matchAll(INPUT_VAR)].map((m) => m[1]!));
export type Entry = "bun-nx" | "bun-x-nx" | "direct-nx-js" | "bun-script" | "other";
export type Reference = { role: string; path: string; exists: boolean; verdict: "ok" | "dead" | "warn" | "output" };
export type Analysis = {
  row: LaunchRow; shell: Shell; env: Record<string, string>; inputs: string[];
  reasons: string[]; ticketIds: string[]; entry: Entry;
  nx?: { sub: string; spec?: string; project?: string; target?: string; configuration?: string; projects: string[]; command: string[]; root: string; local: boolean };
  references: Reference[]; nxVerdict: string; dead: string[]; warnings: string[]; unsupported: string[];
};
const TICKET_PROJECT = /^(?:ticket-|current-native-origin-)/;
const FILE_LIKE = /\/[^/]*\.(?:tsx?|mjs|cjs|jsx?|json|jsonc|py|feature|rs|md|toml|log|txt|ya?ml|sqlite|db)$/;
/**
 * 🔬️ Reads one launch row: shell words, entry point, ticket ownership and everything it needs to run.
 * Dead = it cannot start as written (cwd, Nx workspace, project, target, script, test file or redirection folder
 * is gone) or it names an input whose folder is gone as well. A missing file in an existing folder may be an
 * output and only warns; output flags and output environment values never count.
 */
export function analyse(row: LaunchRow, inputs: Map<string, Json>): Analysis {
  const shell = readShell(mark(row.command));
  const env: Record<string, string> = { ...Object.fromEntries(Object.entries(row.env).map(([key, value]) => [key, mark(value)])), ...Object.fromEntries(shell.assignments) };
  const blob = [row.command, row.cwd, ...Object.values(row.env)].join("\n");
  const used = inputsIn(blob);
  const absolute = (marked: string): string => unmark(marked, ROOT, (id) => String(inputs.get(id)?.default ?? ""));
  const argv = shell.argv;
  let entry: Entry = "other", at = -1;
  if (argv[0] === "bun") {
    if (argv[1] === "nx") { entry = "bun-nx"; at = 1; }
    else if (argv[1] === "x" && argv[2] === "nx") { entry = "bun-x-nx"; at = 2; }
    else if (/node_modules\/nx\/dist\/bin\/nx\.js$/.test(argv[1] ?? "")) { entry = "direct-nx-js"; at = 1; }
    else if (/\.(?:ts|tsx|js|mjs|cjs)$/.test(argv[1] ?? "")) entry = "bun-script";
  }
  const cwd = absolute(mark(row.cwd));
  const references: Reference[] = [];
  const need = (role: string, path: string, required: boolean | "input"): void => {
    const exists = existsSync(path);
    references.push({ role, path: relRoot(path), exists, verdict: required === false ? "output" : exists ? "ok" : required === true || !existsSync(dirname(path)) ? "dead" : "warn" });
  };
  need("cwd", cwd, true);
  let nx: Analysis["nx"];
  let nxVerdict = "n/a";
  if (at !== -1) {
    const tail = argv.slice(at + 2);
    const dash = tail.indexOf("--");
    const before = dash === -1 ? tail : tail.slice(0, dash);
    const sub = argv[at + 1] ?? "";
    const listed = (names: string[], prefix: string): string[] => before.flatMap((arg, k) => (arg.startsWith(prefix) ? arg.slice(prefix.length).split(",") : names.includes(arg) && before[k + 1] ? before[k + 1]!.split(",") : []));
    const override = env.NX_WORKSPACE_ROOT_PATH ? absolute(env.NX_WORKSPACE_ROOT_PATH) : undefined;
    let root = override ?? cwd;
    if (!override) { while (root.length > ROOT.length && !existsSync(join(root, "nx.json"))) root = dirname(root); if (!root.startsWith(ROOT)) root = ROOT; }
    nx = { sub, projects: listed(["--projects", "-p"], "--projects="), command: dash === -1 ? [] : tail.slice(dash + 1), root, local: root !== ROOT };
    if (override) need("env:NX_WORKSPACE_ROOT_PATH", override, true);
    if (sub === "run") {
      nx.spec = before.find((arg) => !arg.startsWith("-"));
      const m = nx.spec?.match(/^([^:]+):([^:]+)(?::(.+))?$/);
      if (m) { nx.project = m[1]; nx.target = m[2]; nx.configuration = m[3]; nx.projects = [m[1]!]; }
    }
    const workspace = workspaceAt(root);
    if (!workspace) nxVerdict = `workspace-missing ${relRoot(root)}`;
    else if (sub === "run") nxVerdict = nx.project ? (nx.project.includes(INPUT_OPEN) ? "input-selected" : resolveTarget(workspace, nx.project, nx.target, nx.configuration).how) : "missing-spec";
    else if (sub === "exec") nxVerdict = nx.projects.length ? nx.projects.map((name) => resolveTarget(workspace, name).how).find((how) => how.startsWith("missing")) ?? "declared-project" : "missing-projects";
    else if (sub === "run-many") {
      const targets = [...listed(["--targets", "--target", "-t"], "--targets="), ...listed([], "--target=")];
      nxVerdict = nx.projects.flatMap((name) => (targets.length ? targets : [undefined]).map((target) => resolveTarget(workspace, name, target).how)).find((how) => how.startsWith("missing")) ?? (nx.projects.length ? "declared" : "all-projects");
    } else nxVerdict = `nx-${sub}`;
  }
  const projectRoots = nx && workspaceAt(nx.root) ? nx.projects.map((name) => workspaceAt(nx!.root)!.projects.get(name)?.root ?? workspaceAt(nx!.root)!.graph.get(name)?.root).filter((path): path is string => !!path) : [];
  const bases = uniq([...projectRoots, cwd, ...(nx ? [nx.root] : []), ROOT]);
  const locate = (text: string): string => (text.startsWith("/") ? text : bases.map((base) => join(base, text)).find((path) => existsSync(path)) ?? join(bases[0]!, text));
  const program = nx ? nx.command : argv;
  const script = program[0] === "bun" && program[1] !== "test" && /\.(?:ts|tsx|js|mjs|cjs)$/.test(program[1] ?? "") ? program[1] : undefined;
  const words = at === -1 ? argv.slice(1) : argv.slice(at + 2);
  let previous = "", scriptSeen = false;
  for (const raw of words) {
    const flag = raw.startsWith("-") && raw.includes("=") ? raw.slice(0, raw.indexOf("=")) : "";
    const value = flag ? raw.slice(flag.length + 1) : raw;
    const output = OUTPUT_FLAG.test(flag || previous);
    previous = raw.startsWith("-") && !raw.includes("=") ? raw : "";
    if (value.startsWith("-") || value.includes(INPUT_OPEN) || /\s/.test(value) || value === "") continue;
    const text = absolute(value);
    if (text.startsWith("/") && !text.startsWith(ROOT + "/")) continue;
    const first = text.split("/")[0]!;
    const pathLike = text.startsWith("/") || /^\.{1,2}\//.test(text) || bare(text).startsWith(bare(".🧬semio")) || FILE_LIKE.test(text) || (!flag && text.includes("/") && !text.includes(":") && bases.some((base) => existsSync(join(base, first))));
    if (!pathLike) continue;
    const isScript = !scriptSeen && script !== undefined && raw === script;
    if (isScript) scriptSeen = true;
    const isTest = program[0] === "bun" && program[1] === "test" && program.includes(raw) && /\.(?:tsx?|jsx?|mjs)$/.test(text);
    need(isScript ? "script" : isTest ? "test-file" : output ? "output-argument" : "argument", locate(text), isScript || isTest ? true : output ? false : "input");
  }
  for (const [key, value] of Object.entries(env)) if (key !== "NX_WORKSPACE_ROOT_PATH" && value.includes(WS_MARK) && !value.includes(INPUT_OPEN)) need(`env:${key}`, absolute(value), OUTPUT_ENV.has(key) ? false : "input");
  for (const redirect of shell.redirects) if (redirect.target) { const target = locate(absolute(redirect.target)); need("redirect-directory", dirname(target), true); need("redirect-file", target, false); }
  const reasons: string[] = [];
  const direct = ticketIdsIn(blob);
  const viaInput = ticketIdsIn(used.map((id) => JSON.stringify(inputs.get(id) ?? {})).join("\n"));
  if (direct.length) reasons.push("ticket-path");
  if (nx?.projects.some((name) => TICKET_PROJECT.test(name))) reasons.push("ticket-named-project");
  if (bare(blob).includes(bare("🗑️generated"))) reasons.push("generated-dir");
  if (viaInput.length) reasons.push("ticket-input");
  const dead = references.filter((reference) => reference.verdict === "dead").map((reference) => `missing ${reference.role}: ${reference.path}`);
  if (/^(?:workspace-missing|missing)/.test(nxVerdict)) dead.push(nxVerdict);
  const warnings = references.filter((reference) => reference.verdict === "warn").map((reference) => `missing ${reference.role} in an existing folder (may be an output): ${reference.path}`);
  const marked = (text: string): string[] => [...text.matchAll(/\uE001([^\uE002]*)\uE002/g)].map((m) => m[1]!);
  const consumed = new Set<string>([...words.flatMap((raw, k) => (OUTPUT_FLAG.test(raw.includes("=") ? raw.slice(0, raw.indexOf("=")) : words[k - 1] ?? "") ? [] : marked(raw))), ...Object.entries(env).flatMap(([key, value]) => (OUTPUT_ENV.has(key) ? [] : marked(value)))]);
  for (const id of consumed) { const fallback = inputs.get(id)?.default; if (typeof fallback === "string" && ticketIdsIn(fallback).length && !existsSync(join(ROOT, fallback))) warnings.push(`default of parameter ${parameterId(id)} names an input path that does not exist: ${fallback}`); }
  return { row, shell, env, inputs: used, reasons, ticketIds: uniq([...direct, ...viaInput]), entry, nx, references, nxVerdict, dead, warnings, unsupported: [...shell.unsupported] };
}
//#endregion

//#region tools
export type Value = { id: string; env?: Record<string, string>; nxFlags?: string[]; args?: string[] };
export type Parameter = { id: string; kind: "choice" | "text" | "flag"; default?: string; required?: boolean; values?: Value[] };
export type Tool = { id: string; verb: string; command: string[]; cwd: string; env?: Record<string, string>; parameters?: Parameter[] };
export type Draft = {
  analysis: Analysis; ticket: string; tool: Tool; verbFrom: "name" | "group" | "fallback";
  wrapperEnv: Record<string, string>; owedEnv: Record<string, string>; redirect: string | null; descriptions: Record<string, string>;
  problems: string[]; duplicates: Analysis[];
};
const UNITS = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
const TENS = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
/** 🔢️ Spelled-out numbers become digits (`twenty-one` -> `21`), so numbered probe series compare by their number. */
export function digitsOf(words: string[]): string[] {
  const out: string[] = [];
  for (let i = 0; i < words.length; i++) {
    const ten = TENS.indexOf(words[i]!), unit = UNITS.indexOf(words[i + 1] ?? "");
    if (ten >= 2) { if (unit >= 1 && unit <= 9) { out.push(String(ten * 10 + unit)); i++; } else out.push(String(ten * 10)); continue; }
    const single = UNITS.indexOf(words[i]!);
    out.push(single >= 0 ? String(single) : words[i]!);
  }
  return out;
}
/** 🪪️ Tool id of a launch row: its name without emoji and without a leading order number, kebab-case, numbers as digits. */
export const identifier = (name: string): string => digitsOf(kebab(name.replace(/[^\x00-\x7F]+/g, " ").replace(/^\s*\d+(?:\.\d+)*\s+(?=[A-Za-z])/, "")).split("-")).join("-");
export const parameterId = (input: string): string => kebab(input);
const asVerb = (word: string): string => VERBS.find((verb) => verb === word || (word.endsWith("s") && word.slice(0, -1) === verb)) ?? FALLBACK_VERB;
/** 🗂️ Verb of a row name as `verb_of` of the dashboard command tree reads it: the leading ASCII word, plural tolerated. */
export const verbOf = (name: string): string => asVerb(name.toLowerCase().match(/[a-z0-9]+/)?.[0] ?? "");
/** 🗂️ Verb of a launch group: its last ASCII word (`4_gate`, `9_gates`, `repo-gate`, `🧹clean🛡️gates`). */
export const groupVerb = (group: string | null): string => asVerb((group ?? "").toLowerCase().match(/[a-z]+/g)?.at(-1) ?? "");
const tokens = (marked: string): string => unmark(marked, WS_TOKEN, (id) => `{${parameterId(id)}}`);
/**
 * 🧱️ Converts one live row into a tool: argument array, workspace-relative cwd, meaningful environment, one
 * parameter per `${input:…}`. Runner noise is left out where the `bun nx` wrapper defaults it; where the wrapper
 * is bypassed it is kept unless the registry schema forbids the variable (then the dashboard owes it).
 */
export function draft(analysis: Analysis, ticketId: string, inputs: Map<string, Json>, rules: Contract): Draft {
  const { row, shell, env, entry } = analysis;
  const problems = [...analysis.unsupported];
  const fromName = verbOf(row.name);
  const fromGroup = groupVerb(row.group);
  const cwdMarked = mark(row.cwd);
  let cwd = "";
  if (cwdMarked.startsWith(WS_MARK + "/")) cwd = tokens(cwdMarked.slice(2));
  else if (cwdMarked !== WS_MARK) problems.push(`cwd outside the workspace: ${row.cwd}`);
  const kept: Record<string, string> = {}, wrapperEnv: Record<string, string> = {}, owedEnv: Record<string, string> = {};
  for (const key of Object.keys(env).sort(compare)) (entry === "bun-nx" && NOISE_ENV.has(key) ? wrapperEnv : rules.forbiddenEnv.has(key) ? owedEnv : kept)[key] = tokens(env[key]!);
  const literal = [...shell.argv, ...Object.values(env), cwdMarked].find((text) => /[{}]/.test(text));
  if (literal) problems.push(`literal brace collides with the token syntax: ${literal}`);
  if (shell.argv.length === 0) problems.push("empty command");
  const descriptions: Record<string, string> = {};
  const parameters: Parameter[] = analysis.inputs.map((id) => {
    const input = inputs.get(id);
    const parameter: Parameter = { id: parameterId(id), kind: input?.type === "pickString" ? "choice" : "text" };
    if (!input) { problems.push(`undeclared input ${id}`); return { ...parameter, required: true }; }
    if (typeof input.description === "string" && input.description) descriptions[parameter.id] = input.description;
    if (typeof input.default === "string" && input.default !== "") parameter.default = input.default; else parameter.required = true;
    if (input.type === "pickString") parameter.values = (input.options as Json[]).map((option) => ({ id: typeof option === "string" ? option : String(option.value) }));
    else if (input.type !== "promptString") problems.push(`input ${id} has unsupported type ${input.type}`);
    return parameter;
  });
  if (uniq(parameters.map((parameter) => parameter.id)).length !== parameters.length || parameters.some((parameter) => parameter.id === VARIANT || parameter.id === "workspace" || parameter.id === "port")) problems.push("parameter ids collide");
  const redirects = shell.redirects;
  if (redirects.length && !(redirects.length === 2 && redirects[0]!.fd === 1 && redirects[0]!.operator === ">" && redirects[1]!.fd === 2 && redirects[1]!.operator === ">&" && redirects[1]!.duplicate === 1)) problems.push(`unsupported redirection ${JSON.stringify(redirects)}`);
  if (row.ready) problems.push("serverReadyAction on a ticket row has no mapping rule");
  const tool: Tool = { id: identifier(row.name), verb: fromName !== FALLBACK_VERB ? fromName : fromGroup, command: shell.argv.map(tokens), cwd };
  if (!tool.id) problems.push("name without any ASCII word");
  if (Object.keys(kept).length) tool.env = kept;
  if (parameters.length) tool.parameters = parameters;
  return { analysis, ticket: ticketId, tool, verbFrom: fromName !== FALLBACK_VERB ? "name" : fromGroup !== FALLBACK_VERB ? "group" : "fallback", wrapperEnv, owedEnv, redirect: redirects[0]?.target ? tokens(redirects[0].target) : null, descriptions, problems, duplicates: [] };
}
const content = (tool: Tool): string => JSON.stringify({ ...tool, id: "" });
//#endregion

//#region identity
/** ✂️ Ids longer than the schema allows keep their leading and trailing words; the middle ones go first. */
export function shorten(id: string, limit: number): string {
  const words = id.split("-");
  while (words.join("-").length > limit && words.length > 2) words.splice(Math.floor(words.length / 2), 1);
  return words.join("-").slice(0, limit).replace(/-+$/, "");
}
const wordsOf = (tool: Tool): string[] => [...tool.command, ...Object.values(tool.env ?? {})].flatMap((text) => kebab(text.replace(/[^\x00-\x7F]+/g, " ")).split("-")).filter(Boolean);
/**
 * 🪪️ Makes ids unique inside one ticket. Rows with the same id and the same tool are one tool (duplicates).
 * Rows whose names differ only by emoji (or by their order number) get the first word of their command no sibling
 * has, else the first word at which their command departs from every sibling; a content hash is the last resort.
 * Never a running number.
 */
export function settle(drafts: Draft[]): Draft[] {
  const byId = new Map<string, Draft[]>();
  for (const item of drafts) byId.set(item.tool.id, [...(byId.get(item.tool.id) ?? []), item]);
  const out: Draft[] = [];
  for (const group of byId.values()) {
    const distinct: Draft[] = [];
    for (const item of group) {
      const twin = distinct.find((other) => content(other.tool) === content(item.tool) && JSON.stringify([other.wrapperEnv, other.owedEnv, other.redirect]) === JSON.stringify([item.wrapperEnv, item.owedEnv, item.redirect]));
      if (twin) twin.duplicates.push(item.analysis, ...item.duplicates); else distinct.push(item);
    }
    const sequences = distinct.map((item) => wordsOf(item.tool));
    const suffixes = distinct.map((item, at) => {
      const mine = sequences[at]!;
      const others = new Set(sequences.flatMap((sequence, k) => (k === at ? [] : sequence)));
      const own = mine.find((word) => !others.has(word) && !item.tool.id.split("-").includes(word) && !/^\d+$/.test(word));
      const turn = mine.findIndex((word, k) => sequences.every((sequence, other) => other === at || sequence[k] !== word));
      return own ?? (turn > 0 ? (/^\d+$/.test(mine[turn]!) ? `${mine[turn - 1]}-${mine[turn]}` : mine[turn]!) : sha256(content(item.tool)).slice(0, 6));
    });
    if (distinct.length > 1) distinct.forEach((item, at) => { item.tool.id = `${item.tool.id}-${suffixes[at]}`; });
    out.push(...distinct);
  }
  const taken = new Set<string>();
  for (const item of out) { if (taken.has(item.tool.id)) item.tool.id = `${item.tool.id}-${sha256(content(item.tool)).slice(0, 6)}`; taken.add(item.tool.id); }
  return out;
}
//#endregion

//#region families
const ATOM = /[A-Za-z0-9]+|[^A-Za-z0-9]+/g;
const WORD = /^[A-Za-z0-9]/;
type Slot = { kind: "id" | "cwd" | "argument" | "env-value"; text: string };
const slots = (tool: Tool): Slot[] => [{ kind: "id", text: tool.id }, { kind: "cwd", text: tool.cwd }, ...tool.command.map((text): Slot => ({ kind: "argument", text })), ...Object.values(tool.env ?? {}).map((text): Slot => ({ kind: "env-value", text }))];
const fixed = (tool: Tool): string => JSON.stringify([tool.verb, Object.keys(tool.env ?? {}), tool.parameters ?? []]);
export type Family = { members: Draft[]; values: string[]; open: Set<number>; idWords: number; outside: number; numeric: boolean; wholeArgument: boolean };
/**
 * 🧬️ Single-dimension families: tools whose id, cwd, command and environment have the same shape and differ only
 * where one value (the driver) occurs. Every other word is identical across the members.
 */
export function families(drafts: Draft[]): Family[] {
  const groups = new Map<string, { draft: Draft; words: string[]; owner: Slot[] }[]>();
  for (const item of drafts) {
    const parts = slots(item.tool).map((slot) => ({ slot, atoms: slot.text.match(ATOM) ?? [] }));
    const skeleton = fixed(item.tool) + "\u0001" + parts.map((part) => `${part.slot.kind}:${part.atoms.map((atom) => (WORD.test(atom) ? "\u0000" : atom)).join("")}`).join("\u0001");
    const words = parts.flatMap((part) => part.atoms.filter((atom) => WORD.test(atom)));
    const owner = parts.flatMap((part) => part.atoms.filter((atom) => WORD.test(atom)).map(() => part.slot));
    groups.set(skeleton, [...(groups.get(skeleton) ?? []), { draft: item, words, owner }]);
  }
  const found: Family[] = [];
  for (const group of groups.values()) {
    if (group.length < 2) continue;
    const width = group[0]!.words.length;
    const varying = [...Array(width).keys()].filter((column) => group.some((row) => row.words[column] !== group[0]!.words[column]));
    const candidates: { rows: typeof group; driver: number; tied: number[] }[] = [];
    for (const driver of varying) {
      for (const mask of uniq(group.map((row) => varying.filter((column) => column !== driver && row.words[column] === row.words[driver]).join(",")))) {
        const tied = mask === "" ? [] : mask.split(",").map(Number);
        const open = new Set([driver, ...tied]);
        const buckets = new Map<string, typeof group>();
        for (const row of group) {
          if (tied.some((column) => row.words[column] !== row.words[driver])) continue;
          const key = row.words.map((word, column) => (open.has(column) ? "" : word)).join("\u0000");
          buckets.set(key, [...(buckets.get(key) ?? []), row]);
        }
        for (const rows of buckets.values()) if (rows.length > 1 && uniq(rows.map((row) => row.words[driver])).length === rows.length) candidates.push({ rows, driver, tied });
      }
    }
    candidates.sort((a, b) => b.rows.length - a.rows.length || a.driver - b.driver || a.tied.length - b.tied.length);
    const taken = new Set<Draft>();
    for (const candidate of candidates) {
      if (candidate.rows.some((row) => taken.has(row.draft))) continue;
      for (const row of candidate.rows) taken.add(row.draft);
      const open = new Set([candidate.driver, ...candidate.tied]);
      const first = candidate.rows[0]!;
      const idWords = first.owner.filter((slot) => slot.kind === "id").length;
      const outside = [...open].filter((column) => column >= idWords);
      const values = candidate.rows.map((row) => row.words[candidate.driver]!);
      found.push({
        members: candidate.rows.map((row) => row.draft), values, open, idWords, outside: outside.length, numeric: values.every((value) => /^\d+$/.test(value)),
        wholeArgument: outside.length === 1 && first.owner[outside[0]!]!.kind === "argument" && first.owner[outside[0]!]!.text === first.words[outside[0]!],
      });
    }
  }
  return found;
}
export type Command = { ticket: string; tool: Tool; members: { draft: Draft; choices: Record<string, string> }[] };
/**
 * 🧩️ Folds mechanical families into one tool with a `variant` choice: numbered series (at least three members,
 * the number also occurs in the row name) and phase lists (at least three members that differ in exactly one whole
 * argument that the row name repeats). Everything else stays one tool per row.
 */
export function fold(drafts: Draft[]): Command[] {
  const merged = new Map<Draft, Command>();
  const ids = new Set(drafts.map((item) => item.tool.id));
  const eligible = families(drafts).filter((family) => family.members.length >= FAMILY_MINIMUM && family.outside > 0 && [...family.open].some((column) => column < family.idWords) && (family.numeric || family.wholeArgument) && family.members.every((member) => member.problems.length === 0));
  const candidates = eligible.map((family) => {
    const first = family.members[0]!;
    let column = 0;
    const rebuilt = slots(first.tool).map((slot) => (slot.text.match(ATOM) ?? []).map((atom) => (!WORD.test(atom) ? atom : family.open.has(column++) ? (slot.kind === "id" ? "" : `{${VARIANT}}`) : atom)).join(""));
    const id = rebuilt[0]!.replace(/-{2,}/g, "-").replace(/^-+|-+$/g, "");
    const envKeys = Object.keys(first.tool.env ?? {});
    const tool: Tool = { id, verb: first.tool.verb, command: rebuilt.slice(2, 2 + first.tool.command.length), cwd: rebuilt[1]! };
    if (envKeys.length) tool.env = Object.fromEntries(envKeys.map((key, k) => [key, rebuilt[2 + first.tool.command.length + k]!]));
    const values = family.numeric ? [...family.values].sort((a, b) => Number(a) - Number(b) || compare(a, b)) : family.values;
    tool.parameters = [...(first.tool.parameters ?? []), { id: VARIANT, kind: "choice", required: true, values: values.map((value) => ({ id: value })) }];
    return { family, tool };
  }).filter((candidate) => candidate.tool.id !== "").sort((a, b) => b.family.members.length - a.family.members.length || compare(a.tool.id, b.tool.id));
  for (const { family, tool } of candidates) {
    const own = new Set(family.members.map((member) => member.tool.id));
    if (ids.has(tool.id) && !own.has(tool.id)) continue;
    for (const id of own) ids.delete(id);
    ids.add(tool.id);
    const command: Command = { ticket: family.members[0]!.ticket, tool, members: family.members.map((member, k) => ({ draft: member, choices: { [VARIANT]: family.values[k]! } })) };
    for (const member of family.members) merged.set(member, command);
  }
  return uniq(drafts.map((item) => merged.get(item) ?? { ticket: item.ticket, tool: item.tool, members: [{ draft: item, choices: {} }] }));
}
//#endregion

//#region proof
export type Expansion = { argv: string[]; env: Record<string, string>; cwd: string };
/** ▶️ What the dashboard starts for a tool and chosen parameter values (fleet-plan.md §2.2 semantics). */
export function expand(tool: Tool, chosen: Record<string, string>): Expansion {
  const values: Record<string, string> = {};
  const env: Record<string, string> = {};
  const extra: string[] = [];
  for (const parameter of tool.parameters ?? []) {
    const value = chosen[parameter.id] ?? parameter.default;
    if (value === undefined) throw new Error(`parameter ${parameter.id} has no value`);
    if (parameter.kind === "choice") {
      const choice = parameter.values?.find((candidate) => candidate.id === value);
      if (!choice) throw new Error(`parameter ${parameter.id} has no value ${value}`);
      Object.assign(env, choice.env ?? {});
      extra.push(...(choice.args ?? []));
    }
    values[parameter.id] = value;
  }
  const substitute = (text: string): string => text.replace(/\{([a-z0-9]+(?:[._-][a-z0-9]+)*)\}/g, (token, key: string) => { if (key === "workspace") return ROOT; if (!(key in values)) throw new Error(`unknown token ${token}`); return values[key]!; });
  return { argv: [...tool.command, ...extra].map(substitute), env: Object.fromEntries(Object.entries({ ...(tool.env ?? {}), ...env }).map(([key, value]) => [key, substitute(value)])), cwd: resolve(ROOT, substitute(tool.cwd)) };
}
export type Probe = { name: string; text: string; shell: Shell; oracleText: string; values: Record<string, string>; problems: string[] };
const REDIRECT_SUFFIX = /^([\s\S]*?)\s+>\s*("[^"\\$`]*"|'[^']*'|[^\s"'<>|&;()$`\\]+)\s+2>&1\s*$/;
/** 🧪️ The command line VS Code hands to the shell for a row (variables substituted) and the part of it that is safe to give to a real shell. */
export function probe(row: LaunchRow, inputs: Map<string, Json>): Probe {
  const values: Record<string, string> = {};
  const text = row.original.command.replaceAll(WS_VAR, ROOT).replace(INPUT_VAR, (_, id: string) => (values[id] = typeof inputs.get(id)?.default === "string" && inputs.get(id).default !== "" ? inputs.get(id).default : `value-of-${parameterId(id)}`));
  for (const source of [row.original.cwd, ...Object.values(row.original.env)]) for (const id of inputsIn(source)) values[id] ??= typeof inputs.get(id)?.default === "string" && inputs.get(id).default !== "" ? inputs.get(id).default : `value-of-${parameterId(id)}`;
  const shell = readShell(text);
  const problems = [...shell.unsupported];
  const oracleText = shell.redirects.length ? text.match(REDIRECT_SUFFIX)?.[1] ?? "" : text;
  if (shell.redirects.length && !oracleText) problems.push("redirection is not the plain `> file 2>&1` suffix");
  if (/[<>;`$\n\r]|&/.test(oracleText)) problems.push("command text is not safe to hand to a shell oracle");
  if (shell.argv[0] !== "bun") problems.push(`entry point is ${shell.argv[0] ?? "(none)"}, not bun`);
  return { name: row.name, text, shell, oracleText, values, problems };
}
export type Observation = { argv: string[]; env: Record<string, string> };
/**
 * 🔮️ Independent reading of the launch command lines by a real shell. `bun` is a shell function that prints its
 * arguments and the inline assignments; the shell runs with an empty environment and a search path that finds no
 * program, the texts contain no redirection, substitution or command separator, so nothing but that function runs.
 */
export function oracle(shellPath: string, flags: string[], probes: Probe[], keys: string[]): Map<number, Observation> {
  const directory = join(GEN, "oracle");
  mkdirSync(directory, { recursive: true });
  const data = join(directory, "commands.txt");
  const script = join(directory, "oracle.sh");
  writeFileSync(data, probes.map((item) => (item.problems.length ? ":" : item.oracleText)).join("\n") + "\n");
  writeFileSync(script, [
    "bun() {",
    `  printf 'R\\0%s\\0' "$SEMIO_ORACLE_ROW"`,
    ...keys.map((key) => `  printf 'E\\0%s\\0%s\\0' ${key} "\${${key}-__unset__}"`),
    `  for SEMIO_ORACLE_WORD in "$@"; do printf 'A\\0%s\\0' "$SEMIO_ORACLE_WORD"; done`,
    "  printf 'Z\\0'",
    "}",
    "SEMIO_ORACLE_COUNT=0",
    `while IFS= read -r SEMIO_ORACLE_LINE; do ( SEMIO_ORACLE_ROW=$SEMIO_ORACLE_COUNT; eval "$SEMIO_ORACLE_LINE" ); SEMIO_ORACLE_COUNT=$((SEMIO_ORACLE_COUNT+1)); done < "$1"`,
    "",
  ].join("\n"));
  const run = Bun.spawnSync(["/usr/bin/env", "-i", "PATH=/semio-oracle-finds-no-program", shellPath, ...flags, script, data], { cwd: directory, stdout: "pipe", stderr: "pipe" });
  const fields = run.stdout.toString("utf8").split("\0");
  const seen = new Map<number, Observation>();
  let current: Observation | null = null, row = -1;
  for (let i = 0; i < fields.length - 1; i++) {
    const tag = fields[i];
    if (tag === "R") { row = Number(fields[++i]); current = { argv: ["bun"], env: {} }; }
    else if (tag === "E" && current) { const key = fields[++i]!, value = fields[++i]!; if (value !== "__unset__") current.env[key] = value; }
    else if (tag === "A" && current) current.argv.push(fields[++i]!);
    else if (tag === "Z" && current) { seen.set(row, current); current = null; }
  }
  rmSync(directory, { recursive: true, force: true });
  return seen;
}
export type Verdict = { name: string; ticket: string; tool: string; choices: Record<string, string>; failures: string[] };
const same = (a: unknown, b: unknown): boolean => JSON.stringify(a) === JSON.stringify(b);
const sorted = (record: Record<string, string>): Record<string, string> => Object.fromEntries(Object.entries(record).sort((a, b) => compare(a[0], b[0])));
/**
 * ⚖️ Equivalence proof. For every launch row that became (part of) a tool: `/bin/sh` and `/bin/zsh` read the
 * original command line, this script's reader agrees with both, and the tool expanded with the row's parameter
 * values yields that argv, the row's cwd and the row's environment minus the variables recorded as left out.
 */
export function prove(commands: Command[], inputs: Map<string, Json>): { verdicts: Verdict[]; shells: Record<string, number> } {
  const members = commands.flatMap((command) => command.members.flatMap((member) => [member.draft.analysis, ...member.draft.duplicates].map((analysis) => ({ command, member, analysis }))));
  const probes = members.map((item) => probe(item.analysis.row, inputs));
  const keys = uniq(probes.flatMap((item) => item.shell.assignments.map(([key]) => key))).sort(compare);
  const readings = { "/bin/sh": oracle("/bin/sh", [], probes, keys), "/bin/zsh": oracle("/bin/zsh", ["-f"], probes, keys) };
  const verdicts = members.map(({ command, member, analysis }, index): Verdict => {
    const item = probes[index]!;
    const failures = [...item.problems];
    const reader: Observation = { argv: item.shell.argv, env: Object.fromEntries(item.shell.assignments) };
    for (const [shell, reading] of Object.entries(readings)) {
      const observed = reading.get(index);
      if (!observed) failures.push(`${shell} did not run the command`);
      else if (!same(observed.argv, reader.argv)) failures.push(`${shell} reads other words: ${JSON.stringify(observed.argv)} vs ${JSON.stringify(reader.argv)}`);
      else if (!same(sorted(observed.env), sorted(reader.env))) failures.push(`${shell} reads other assignments: ${JSON.stringify(observed.env)} vs ${JSON.stringify(reader.env)}`);
    }
    const substitute = (text: string): string => text.replaceAll(WS_VAR, ROOT).replace(INPUT_VAR, (_, id: string) => item.values[id]!);
    const left = new Set([...Object.keys(member.draft.wrapperEnv), ...Object.keys(member.draft.owedEnv)]);
    const expectedEnv = Object.fromEntries(Object.entries({ ...Object.fromEntries(Object.entries(analysis.row.original.env).map(([key, value]) => [key, substitute(value)])), ...reader.env }).filter(([key]) => !left.has(key)));
    try {
      const actual = expand(command.tool, { ...Object.fromEntries(Object.entries(item.values).map(([id, value]) => [parameterId(id), value])), ...member.choices });
      if (!same(actual.argv, reader.argv)) failures.push(`argv differs: tool ${JSON.stringify(actual.argv)} vs row ${JSON.stringify(reader.argv)}`);
      if (!same(sorted(actual.env), sorted(expectedEnv))) failures.push(`environment differs: tool ${JSON.stringify(sorted(actual.env))} vs row ${JSON.stringify(sorted(expectedEnv))}`);
      if (actual.cwd !== resolve(substitute(analysis.row.original.cwd))) failures.push(`cwd differs: tool ${actual.cwd} vs row ${substitute(analysis.row.original.cwd)}`);
    } catch (error) { failures.push(`tool does not expand: ${(error as Error).message}`); }
    return { name: analysis.row.name, ticket: command.ticket, tool: command.tool.id, choices: member.choices, failures };
  });
  return { verdicts, shells: Object.fromEntries(Object.entries(readings).map(([shell, reading]) => [shell, reading.size])) };
}
//#endregion

//#region manifests
const ordered = (tool: Tool): Tool => {
  const out: Tool = { id: tool.id, verb: tool.verb, command: tool.command, cwd: tool.cwd };
  if (tool.env && Object.keys(tool.env).length) out.env = sorted(tool.env);
  if (tool.parameters?.length) out.parameters = tool.parameters.map((parameter) => { const p: Parameter = { id: parameter.id, kind: parameter.kind }; if (parameter.default !== undefined) p.default = parameter.default; if (parameter.required) p.required = true; if (parameter.values) p.values = parameter.values; return p; });
  return out;
};
const byVerbThenId = (a: Json, b: Json): number => compare(String(a.verb ?? ""), String(b.verb ?? "")) || compare(String(a.id), String(b.id));
export type Merge = { document: Json; preserved: string[]; ownerEdited: string[]; retired: string[]; existed: boolean };
/**
 * 🤝️ Three-way merge with a manifest already in the ticket folder. Tools this script wrote before (hash in the
 * previous coverage log) are replaced or retired; a tool the owner added or edited since is kept as it is.
 */
export function merge(tools: Tool[], existing: Json | null, base: Record<string, string>): Merge {
  const generated = new Map(tools.map((tool) => [tool.id, tool as Json]));
  const preserved: string[] = [], ownerEdited: string[] = [], retired: string[] = [];
  for (const tool of (existing?.tools as Json[]) ?? []) {
    const hash = sha256(JSON.stringify(tool));
    const ours = generated.get(tool.id);
    if (ours && sha256(JSON.stringify(ours)) === hash) continue;
    if (base[tool.id] === hash) { if (!ours) retired.push(tool.id); continue; }
    (tool.id in base ? ownerEdited : preserved).push(tool.id);
    generated.set(tool.id, tool);
  }
  const document: Json = {};
  if (typeof existing?.$schema === "string") document.$schema = existing.$schema;
  document.tools = [...generated.values()].sort(byVerbThenId);
  if (Array.isArray(existing?.compounds) && existing.compounds.length) document.compounds = existing.compounds;
  return { document, preserved, ownerEdited, retired, existed: existing !== null };
}
//#endregion

//#region self test
/**
 * 🧫️ Proves the instruments before they are trusted: the shell reader against /bin/sh and /bin/zsh on quoting
 * cases, the refusal of everything a shell would expand or chain, the proof against deliberately wrong tools, the
 * registry schema against forbidden content and the three-way merge.
 */
export function selfTest(rules: Contract): string[] {
  const failures: string[] = [];
  const expect = (label: string, holds: boolean): void => { if (!holds) failures.push(label); };
  const vectors = [
    String.raw`bun a "b c" 'd e' f\ g "h\"i" 'j"k' "l'm" --x='y z' "" ''`,
    String.raw`A=1 B='two words' C="x y" bun "x" D=word`,
    String.raw`"bun" nx run p:t --args='long -E "test(a) | test(b)"' -- --flag=value`,
    String.raw`bun "C:\Users\Example User\x" "/Users/Example User/y" 'emoji 🧪️ words'`,
    String.raw`bun a\\b "c\\d" 'e\\f' g"h"'i'j k=l`,
    "bun   spaced \t out",
  ];
  const probes: Probe[] = vectors.map((text) => ({ name: text, text, shell: readShell(text), oracleText: text, values: {}, problems: [] }));
  const keys = uniq(probes.flatMap((item) => item.shell.assignments.map(([key]) => key)));
  for (const [shell, flags] of [["/bin/sh", []], ["/bin/zsh", ["-f"]]] as const) {
    const reading = oracle(shell, [...flags], probes, keys);
    probes.forEach((item, index) => {
      expect(`reader has nothing unsupported in: ${item.text}`, item.shell.unsupported.length === 0);
      expect(`${shell} agrees on the words of: ${item.text}`, same(reading.get(index)?.argv, item.shell.argv));
      expect(`${shell} agrees on the assignments of: ${item.text}`, same(sorted(reading.get(index)?.env ?? {}), sorted(Object.fromEntries(item.shell.assignments))));
    });
  }
  for (const text of ["bun $HOME", "bun a; b", "bun *", "bun a | b", "bun ~/x", "bun {a,b}", "bun `x`", 'bun "$(x)"', "bun a && b", "bun a &", 'bun "a$b"', "bun 'open", "bun =x", "bun #c", "bun a &> b", 'bun "wow!"']) expect(`reader refuses: ${text}`, readShell(text).unsupported.length > 0);
  const redirected = readShell(String.raw`bun a > "out file.log" 2>&1`);
  expect("reader separates the redirection", same(redirected.argv, ["bun", "a"]) && same(redirected.redirects, [{ fd: 1, operator: ">", target: "out file.log" }, { fd: 2, operator: ">&", target: "", duplicate: 1 }]) && redirected.unsupported.length === 0);
  expect("identifier drops emoji and the order number and writes numbers as digits", identifier("900.263001 Current Native Origin TwentyOne Source Laws") === "current-native-origin-21-source-laws" && identifier("🧪️test🏢️ifc📕️pdf🦀️assembly") === "test-ifc-pdf-assembly");
  expect("verbs come from the leading word of the name, else the last word of the group", verbOf("🧪️test🏢️ifc") === "test" && verbOf("⚖️gates x") === "gate" && verbOf("🛠️os-common") === FALLBACK_VERB && groupVerb("🧹clean🛡️gates") === "gate" && groupVerb("4_build") === "build" && groupVerb("9_clean_architecture") === FALLBACK_VERB);
  expect("shorten keeps both ends", shorten("a-b-c-d-e", 7) === "a-b-d-e" && shorten("short", 64) === "short");
  const inputs = new Map<string, Json>([["who", { id: "who", type: "promptString", default: "w x" }]]);
  const row = toRow({ name: "🧪️self proof", command: `NX_DAEMON=false bun "${WS_VAR}/x y.ts" run "\${input:who}" ${ROOT}/z`, cwd: `${WS_VAR}/sub`, env: { K: `${WS_VAR}/v`, FORCE_COLOR: "0" } }, 0, "launch");
  const made = draft(analyse(row, inputs), "00/00/00/SELF", inputs, { ...rules, forbiddenEnv: new Set(["NX_DAEMON", "FORCE_COLOR"]) });
  expect("draft yields argv, cwd, environment and parameter", same(made.tool, { id: "self-proof", verb: FALLBACK_VERB, command: ["bun", "{workspace}/x y.ts", "run", "{who}", "{workspace}/z"], cwd: "sub", env: { K: "{workspace}/v" }, parameters: [{ id: "who", kind: "text", default: "w x" }] }) && same(made.owedEnv, { FORCE_COLOR: "0", NX_DAEMON: "false" }) && row.rewritten && made.problems.length === 0);
  const verdict = (tool: Tool): number => prove([{ ticket: made.ticket, tool, members: [{ draft: made, choices: {} }] }], inputs).verdicts[0]!.failures.length;
  expect("proof accepts the faithful tool", verdict(made.tool) === 0);
  expect("proof rejects a dropped argument", verdict({ ...made.tool, command: made.tool.command.slice(0, -1) }) > 0);
  expect("proof rejects a changed argument", verdict({ ...made.tool, command: made.tool.command.map((word) => (word === "run" ? "ran" : word)) }) > 0);
  expect("proof rejects a parameter frozen to another value", verdict({ ...made.tool, command: made.tool.command.map((word) => (word === "{who}" ? "w" : word)) }) > 0);
  expect("proof rejects a changed environment", verdict({ ...made.tool, env: { K: "{workspace}/other" } }) > 0);
  expect("proof rejects an added variable", verdict({ ...made.tool, env: { ...made.tool.env, EXTRA: "1" } }) > 0);
  expect("proof rejects a changed cwd", verdict({ ...made.tool, cwd: "" }) > 0);
  if (rules.path) {
    const valid: Tool = { id: "ok-tool", verb: "gate", command: ["bun", "{workspace}/x.ts"], cwd: "" };
    expect("schema accepts a plain tool", rules.tool(valid).length === 0);
    expect("schema refuses every dashboard-owned variable", rules.forbiddenEnv.size > 0 && [...rules.forbiddenEnv].every((key) => rules.tool({ ...valid, env: { [key]: "false" } }).length > 0));
    expect("schema refuses an absolute path", rules.tool({ ...valid, command: ["bun", "/Users/someone/x.ts"] }).length > 0);
    expect("schema refuses an overlong id", rules.tool({ ...valid, id: "a".repeat(rules.idLimit + 1) }).length > 0);
    expect("schema refuses an unknown member", rules.tool({ ...valid, label: "x" } as Json).length > 0);
    expect("schema accepts a document and refuses a foreign key", rules.document({ tools: [valid] }).length === 0 && rules.document({ tools: [valid], parameters: [] }).length > 0);
  }
  const hash = (tool: Json): string => sha256(JSON.stringify(tool));
  const kept = { id: "kept", command: ["a"] }, edited = { id: "edited", command: ["owner"] }, gone = { id: "gone", command: ["old"] }, foreign = { id: "foreign", command: ["theirs"] };
  const merged = merge([{ id: "kept", verb: "gate", command: ["new"], cwd: "" }, { id: "edited", verb: "gate", command: ["ours"], cwd: "" }], { tools: [kept, edited, gone, foreign], compounds: [{ id: "c", members: [{ run: "a:b" }] }] }, { kept: hash(kept), edited: hash({ id: "edited", command: ["before"] }), gone: hash(gone) });
  expect("merge replaces untouched tools, keeps owner edits and foreign tools, retires stale ones", same(merged.document.tools.map((tool: Json) => [tool.id, tool.command[0]]), [["edited", "owner"], ["foreign", "theirs"], ["kept", "new"]]) && same([merged.preserved, merged.ownerEdited, merged.retired], [["foreign"], ["edited"], ["gone"]]) && merged.document.compounds.length === 1);
  return failures;
}
//#endregion

//#region run
const category = (reason: string): string => (reason.startsWith("missing ") ? reason.slice(0, reason.indexOf(":")) : reason.split(" ")[0]!);
async function main(): Promise<number> {
  const check = process.argv.includes("--check");
  const rules = await contract();
  if (process.argv.includes("--self-test")) { const failures = selfTest(rules); console.log(JSON.stringify({ selfTest: failures.length ? "FAILED" : "passed", failures }, null, 2)); return failures.length ? 1 : 0; }
  if (!existsSync(join(ROOT, ".vscode/launch.json"))) { console.error("No .vscode/launch.json: nothing to convert; the ticket manifests stay as they are."); return 1; }
  const sources = readSources(join(ROOT, ".vscode/launch.json"), seedFile(join(ROOT, ".vscode")));
  const snapshotPath = join(TICKET_DIR, "🗑️generated", "launch-inventory", "snapshot", "launch.json");
  const snapshot = existsSync(snapshotPath) ? new Map<string, string>(((Bun.JSONC.parse(readText(snapshotPath)) as Json).configurations as Json[]).map((row) => [String(row.name), JSON.stringify([row.command, row.cwd, row.env ?? {}])])) : null;
  const since = (row: LaunchRow): "same" | "new" | "changed" | "unknown" => (!snapshot ? "unknown" : !snapshot.has(row.name) ? "new" : snapshot.get(row.name) === JSON.stringify([row.original.command, row.original.cwd, row.original.env]) ? "same" : "changed");
  const scoped = sources.rows.map((row) => analyse(row, sources.inputs)).filter((analysis) => analysis.reasons.length);
  const log: Json[] = [];
  const record = (analysis: Analysis, status: string, extra: Json = {}): void => { log.push({ name: analysis.row.name, ticket: analysis.ticketIds[0] ?? null, origin: analysis.row.origin, sinceSnapshot: since(analysis.row), status, ...extra }); };
  const original = (analysis: Analysis): Json => ({ command: analysis.row.original.command, cwd: analysis.row.original.cwd, env: analysis.row.original.env });
  const drafts = new Map<string, Draft[]>();
  for (const analysis of scoped) {
    const id = analysis.ticketIds[0];
    const owner = id ? ticket(id) : null;
    if (!owner) record(analysis, "unowned", { reason: `ticket-scoped only through ${analysis.reasons.join("+")}; no ticket folder is named`, ...original(analysis) });
    else if (owner.status !== "open") record(analysis, "ticket-not-open", { reason: `ticket status ${owner.status}`, ...original(analysis) });
    else if (analysis.dead.length) record(analysis, "dead", { reasons: analysis.dead, ...original(analysis) });
    else drafts.set(id!, [...(drafts.get(id!) ?? []), draft(analysis, id!, sources.inputs, rules)]);
  }
  const previous: Json = existsSync(join(GEN, "coverage.json")) ? JSON.parse(readText(join(GEN, "coverage.json"))) : {};
  const commands: Command[] = [];
  for (const [id, list] of [...drafts].sort((a, b) => compare(a[0], b[0]))) {
    const convertible = list.filter((item) => { if (item.problems.length) record(item.analysis, "not-carried", { reasons: item.problems, ...original(item.analysis) }); return item.problems.length === 0; });
    const folded = fold(settle(convertible)).sort((a, b) => compare(a.tool.id, b.tool.id));
    const taken = new Set(folded.map((command) => command.tool.id).filter((name) => name.length <= rules.idLimit));
    for (const command of folded) {
      let short = command.tool.id;
      if (short.length > rules.idLimit) { short = shorten(short, rules.idLimit); if (taken.has(short)) short = `${shorten(command.tool.id, rules.idLimit - 7)}-${sha256(command.tool.id).slice(0, 6)}`; taken.add(short); }
      command.ticket = id;
      command.tool = ordered({ ...command.tool, id: short });
      const errors = rules.tool(command.tool);
      if (errors.length) for (const member of command.members) for (const analysis of [member.draft.analysis, ...member.draft.duplicates]) record(analysis, "not-carried", { reasons: errors.map((error) => `registry schema: ${error}`), ...original(analysis) });
      else commands.push(command);
    }
  }
  const proof = prove(commands, sources.inputs);
  const failed = new Set(proof.verdicts.filter((verdict) => verdict.failures.length).map((verdict) => verdict.tool + "\u0000" + verdict.ticket));
  const manifests: Record<string, Json> = {};
  const tickets: Record<string, Json> = {};
  let invalid = 0;
  for (const id of uniq(scoped.map((analysis) => analysis.ticketIds[0]).filter((value): value is string => !!value)).sort(compare)) {
    const owner = ticket(id);
    const mine = commands.filter((command) => command.ticket === id && !failed.has(command.tool.id + "\u0000" + id));
    for (const command of mine) for (const member of command.members) {
      const extra = { tool: command.tool.id, command: `ticket:${id}/${command.tool.id}`, verb: command.tool.verb, verbFrom: member.draft.verbFrom, parameters: member.choices, toolParameters: (command.tool.parameters ?? []).map((parameter) => parameter.id), parameterDescriptions: member.draft.descriptions, runnerNoiseDropped: member.draft.wrapperEnv, owedByDashboard: member.draft.owedEnv, redirectNotCarried: member.draft.redirect, entry: member.draft.analysis.entry, rootRewritten: member.draft.analysis.row.rewritten, warnings: member.draft.analysis.warnings };
      record(member.draft.analysis, command.members.length > 1 ? "merged" : "tool", extra);
      for (const twin of member.draft.duplicates) record(twin, "duplicate", { ...extra, duplicateOf: member.draft.analysis.row.name, rootRewritten: twin.row.rewritten, warnings: twin.warnings });
    }
    for (const command of commands.filter((candidate) => candidate.ticket === id && failed.has(candidate.tool.id + "\u0000" + id))) for (const member of command.members) for (const analysis of [member.draft.analysis, ...member.draft.duplicates]) record(analysis, "not-carried", { reasons: proof.verdicts.filter((verdict) => verdict.ticket === id && verdict.tool === command.tool.id && verdict.failures.length).flatMap((verdict) => verdict.failures.map((failure) => `equivalence: ${failure}`)).slice(0, 4), ...original(analysis) });
    const rows = log.filter((entry) => entry.ticket === id);
    const count = (status: string): number => rows.filter((entry) => entry.status === status).length;
    const summary: Json = {
      folder: owner.folder, status: owner.status, title: owner.title, manifest: owner.manifest,
      rows: rows.length, live: rows.length - count("dead"), dead: count("dead"),
      deadReasons: tally(rows.filter((entry) => entry.status === "dead").map((entry) => category(String(entry.reasons[0])))),
      newSinceSnapshot: rows.filter((entry) => entry.sinceSnapshot === "new").length, changedSinceSnapshot: rows.filter((entry) => entry.sinceSnapshot === "changed").length,
      tools: mine.length, parameterizedTools: mine.filter((command) => command.members.length > 1).length, rowsMergedIntoParameters: count("merged"),
      duplicateRowsFolded: count("duplicate"), notCarried: count("not-carried"),
      toolsWithInputParameters: mine.filter((command) => (command.tool.parameters ?? []).some((parameter) => parameter.id !== VARIANT)).length,
      redirectsNotCarried: rows.filter((entry) => entry.redirectNotCarried).length,
      owedByDashboard: tally(rows.flatMap((entry) => Object.entries(entry.owedByDashboard ?? {}).map(([key, value]) => `${key}=${value}`))),
      runnerNoiseDropped: tally(rows.flatMap((entry) => Object.entries(entry.runnerNoiseDropped ?? {}).map(([key, value]) => `${key}=${value}`))),
      rootRewrittenRows: rows.filter((entry) => entry.rootRewritten).length, warnings: rows.filter((entry) => entry.warnings?.length).length,
      verbs: tally(mine.map((command) => command.tool.verb)),
    };
    if (owner.status === "open" && owner.folder && owner.manifest) {
      const path = join(ROOT, owner.manifest);
      const existing = existsSync(path) ? JSON.parse(readText(path)) : null;
      const merged = merge(mine.map((command) => command.tool), existing, previous.manifests?.[id]?.tools ?? {});
      const errors = rules.document(merged.document);
      invalid += errors.length ? 1 : 0;
      const text = JSON.stringify(merged.document, null, 2) + "\n";
      const write = !check && errors.length === 0 && (merged.document.tools.length > 0 || merged.existed);
      if (write && (!existing || readText(path) !== text)) writeFileSync(path, text);
      if (check) { mkdirSync(join(GEN, "preview"), { recursive: true }); writeFileSync(join(GEN, "preview", `${id.replaceAll("/", "-")}.json`), text); }
      Object.assign(summary, { manifestExisted: merged.existed, preservedForeignTools: merged.preserved, ownerEditedTools: merged.ownerEdited, retiredTools: merged.retired, schemaErrors: errors, written: write, bytes: Buffer.byteLength(text), sha256: sha256(text) });
      if (write) manifests[id] = { path: owner.manifest, sha256: sha256(text), tools: Object.fromEntries(mine.map((command) => [command.tool.id, sha256(JSON.stringify(command.tool))])) };
      else if (previous.manifests?.[id]) manifests[id] = previous.manifests[id];
    }
    tickets[id] = summary;
  }
  const failures = proof.verdicts.filter((verdict) => verdict.failures.length);
  const totals = {
    launchRows: sources.launchRows, seedOnlyRows: sources.seedOnlyRows, ticketScopedRows: scoped.length,
    byStatus: tally(log.map((entry) => entry.status)), tickets: Object.keys(tickets).length,
    tools: Object.values(tickets).reduce((sum, entry) => sum + entry.tools, 0),
    newSinceSnapshot: log.filter((entry) => entry.sinceSnapshot === "new").length, changedSinceSnapshot: log.filter((entry) => entry.sinceSnapshot === "changed").length,
  };
  const coverage = {
    generator: relRoot(join(TICKET_DIR, "ticket-commands.ts")), mode: check ? "check" : "write",
    sources: { launch: { path: relRoot(sources.launchPath), sha256: sources.launchSha, rows: sources.launchRows }, seed: sources.seedPath ? { path: relRoot(sources.seedPath), sha256: sources.seedSha, rowsNotInLaunch: sources.seedOnlyRows } : null, snapshot: snapshot ? relRoot(snapshotPath) : null, schema: rules.path ? { path: rules.path, sha256: rules.sha, forbiddenEnv: [...rules.forbiddenEnv].sort(compare), idLimit: rules.idLimit } : null },
    totals, compoundsWithTicketRows: sources.compounds.filter((compound) => (compound.configurations as string[]).some((member) => scoped.some((analysis) => analysis.row.name === member))).map((compound) => compound.name), proof: { rowsProven: proof.verdicts.length - failures.length, rowsFailed: failures.length, shellsRan: proof.shells, failures: failures.slice(0, 50) },
    validation: rules.path ? { manifestsInvalid: invalid } : "schema absent: WRITTEN BUT UNVERIFIED",
    tickets, manifests, rows: log.sort((a, b) => compare(String(a.ticket), String(b.ticket)) || compare(a.name, b.name)),
  };
  mkdirSync(GEN, { recursive: true });
  writeFileSync(join(GEN, "coverage.json"), JSON.stringify(coverage, null, 2) + "\n");
  console.log(JSON.stringify({ mode: coverage.mode, totals, proof: { ...coverage.proof, failures: failures.slice(0, 5) }, validation: coverage.validation, tickets: Object.fromEntries(Object.entries(tickets).map(([id, entry]) => [id, { rows: entry.rows, live: entry.live, dead: entry.dead, new: entry.newSinceSnapshot, tools: entry.tools, parameterized: entry.parameterizedTools, merged: entry.rowsMergedIntoParameters, duplicates: entry.duplicateRowsFolded, notCarried: entry.notCarried, written: entry.written, schemaErrors: entry.schemaErrors?.length ?? 0 }])) }, null, 2));
  return failures.length || invalid ? 1 : 0;
}
if (import.meta.main) process.exit(await main());
//#endregion
