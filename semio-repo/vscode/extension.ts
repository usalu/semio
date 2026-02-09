// #region 🔖Header

// 💻 semio-repo/vscode/extension.ts

// 2025 Ueli Saluz <ueli@semio-tech.com>

// #region 🔖License

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.


// #endregion 🔖License

// #region 🔖Specs
// #endregion 🔖Specs

// #endregion 🔖Header

// #region 🔖Imports

import { deserializeKit, Problem, validateKit } from "@semio/js/semio";
import { exec, execFile } from "child_process";
import * as fs from "fs";
import * as path from "path";
import { promisify } from "util";
import * as vscode from "vscode";

const execAsync = promisify(exec);
const execFileAsync = promisify(execFile);

export type RepoEvent = {
  kind: string;
  data?: unknown;
  result?: unknown;
  error?: { message?: string; fatal?: boolean };
  done?: { exit_code?: number };
};

// #endregion 🔖Imports

// #region 🔖Constants

const SEMIO_KIT_LANGUAGE = "json";
const DIAGNOSTIC_SOURCE = "semio";

const UI_STRINGS = {
  en: {
    sectionsEmpty: "No sections found",
    sectionsNoActiveFile: "No active file",
    sectionsRenamePrompt: "Enter new section name",
    sectionsCreateChildPrompt: "Enter child section name",
    sectionsDeleteConfirm: "Enter section path to delete",
  },
  de: {
    sectionsEmpty: "Keine Abschnitte gefunden",
    sectionsNoActiveFile: "Keine aktive Datei",
    sectionsRenamePrompt: "Neuen Abschnittsnamen eingeben",
    sectionsCreateChildPrompt: "Name des Unterabschnitts eingeben",
    sectionsDeleteConfirm: "Abschnittspfad zum Löschen eingeben",
  },
};

// #endregion 🔖Constants

// #region 🔖Types

export interface ToolResult<T = unknown> {
  output: { lines: { type: string; text: string }[]; exitCode: number };
  data?: T;
  error?: string;
}

export interface ProjectData {
  name: string;
  kind?: string;
  root: string;
  sourceRoot?: string;
  projectType?: string;
  tags?: string[];
}

export interface PolicyData {
  id: string;
  name: string;
  description: string;
}

export interface TicketFrontmatter {
  status: string;
  prompt: string;
  summary?: string;
  author?: string;
  commit?: string;
  started?: string;
  finished?: string;
  ignore?: boolean;
}

export interface TicketInteraction {
  prompt: string;
  llm: string;
  client: string;
  author: string;
  dates: { started: string; finished?: string };
  commit: string;
}

export interface TicketData {
  year: number;
  month: number;
  day: number;
  slug: string;
  frontmatter: TicketFrontmatter;
  folderPath: string;
  filePath: string;
  interactions?: TicketInteraction[];
}

export interface ContributorLineMetrics {
  added: number;
  removed: number;
}

export interface ContributorDefinitionData {
  name: string;
  lines: ContributorLineMetrics;
}

export interface ContributorSectionData {
  name: string;
  lines: ContributorLineMetrics;
  definitions: ContributorDefinitionData[];
}

export interface ContributorFileData {
  name: string;
  lines: ContributorLineMetrics;
  sections: ContributorSectionData[];
}

export interface ContributorFolderData {
  name: string;
  lines: ContributorLineMetrics;
  files: ContributorFileData[];
}

export interface ContributorBundleData {
  name: string;
  lines: ContributorLineMetrics;
  folders: ContributorFolderData[];
}

export interface ContributorTicketData {
  year: number;
  month: number;
  day: number;
  slug: string;
  status: string;
  title: string;
  summary: string;
  folderPath?: string;
  filePath?: string;
}

export interface ContributorCommitData {
  title: string;
  sha: string;
}

export interface ContributorData {
  github: string;
  name?: string;
  emails?: string[];
  links?: Record<string, string>;
  contributions?: {
    commits: ContributorCommitData[];
    tickets: ContributorTicketData[];
    bundles: ContributorBundleData[];
  };
}

interface TextEdit {
  start: number;
  end: number;
  newText: string;
}

interface AutoFix {
  description: string;
  edits: Record<string, TextEdit[]>;
}

interface Violation {
  id: string;
  summary: string;
  kind: { id: string };
  scope: string;
  line?: number;
  column?: number;
  excerpt?: string;
  autofix?: AutoFix;
}

interface AnalyzeReport {
  timestamp: string;
  scope: string;
  violations: Violation[];
}

interface SectionInfo {
  name: string;
  kind: string;
  startLine: number;
  endLine: number;
  startIndex: number;
  endIndex: number;
  children: SectionInfo[];
}

interface DefinitionInfo {
  name: string;
  kind: string;
  startLine: number;
  endLine: number;
  startIndex: number;
  endIndex: number;
}

interface GraphqlSectionRange {
  start?: number;
  end?: number;
}

interface GraphqlSection {
  name: string;
  __typename?: string;
  range?: GraphqlSectionRange | null;
  children?: GraphqlSection[] | null;
}

// #endregion 🔖Types

// #region 🔖Globals

let outputChannel: vscode.OutputChannel;
let repoDiagnosticCollection: vscode.DiagnosticCollection;
let kitDiagnosticCollection: vscode.DiagnosticCollection;
const fileViolationsMap = new Map<string, Violation[]>();
interface BundleInfo { id: string; root: string; }
let bundleCache: BundleInfo[] = [];
let cachedRepoBaseUrl: string | undefined = undefined;
const runningProcesses = new Map<string, AbortController>();

let filterProvider: FilterTreeDataProvider | undefined;
let monorepoProvider: MonorepoTreeDataProvider | undefined;

// #endregion 🔖Globals

// #region 🔖Utilities

function writeLog(level: string, args: any[]): void {
  const message = args.map(a => typeof a === 'object' ? JSON.stringify(a, null, 2) : String(a)).join(' ');
  const prefix = level === 'ERROR' ? '[ERROR] ' : '';
  outputChannel?.appendLine(prefix + message);
  try {
    const logPath = path.join(getWorkspaceRoot() || "", "activation.log");
    fs.appendFileSync(logPath, `[${level}] ${message}\n`);
  } catch (e) { }
}

function log(...args: any[]): void {
  writeLog('LOG', args);
}

function logError(...args: any[]): void {
  writeLog('ERROR', args);
}

function getWorkspaceRoot(): string | undefined {
  return vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
}

function getRepoBinaryPath(): string | undefined {
  const root = getWorkspaceRoot();
  if (!root) return undefined;
  const ext = process.platform === "win32" ? ".exe" : "";
  const candidate = path.join(root, "semio-repo", "cli", `cli${ext}`);
  return fs.existsSync(candidate) ? candidate : undefined;
}

function execShell(cmd: string, cwd: string | undefined): Promise<string> {
  return new Promise((resolve, reject) => {
    exec(cmd, { cwd, maxBuffer: 1024 * 1024 * 10 }, (err, stdout, stderr) => {
      if (err) return reject(err);
      resolve(stdout);
    });
  });
}

function getRepoCommand(): string {
  const binaryPath = getRepoBinaryPath();
  return binaryPath ?? "";
}

function hasRepoAccess(): boolean {
  return getRepoCommand() !== "";
}

function getUiString(key: keyof typeof UI_STRINGS.en): string {
  const language = vscode.env.language.split("-")[0];
  const bundle = UI_STRINGS[language as keyof typeof UI_STRINGS] ?? UI_STRINGS.en;
  return bundle[key];
}

function resolveCommitSha(commit: string | { sha?: string } | undefined): string | undefined {
  if (!commit) return undefined;
  if (typeof commit === "string") return commit;
  return commit.sha;
}

function getGitHubRepoBaseUrl(): string | undefined {
  if (cachedRepoBaseUrl !== undefined) return cachedRepoBaseUrl;
  const root = getWorkspaceRoot();
  if (!root) return (cachedRepoBaseUrl = undefined);
  const packagePath = path.join(root, "package.json");
  if (!fs.existsSync(packagePath)) return (cachedRepoBaseUrl = undefined);
  const raw = fs.readFileSync(packagePath, "utf8");
  const parsed = JSON.parse(raw) as { repository?: { url?: string } | string };
  const repoUrl = typeof parsed.repository === "string" ? parsed.repository : parsed.repository?.url;
  if (!repoUrl) return (cachedRepoBaseUrl = undefined);
  let cleaned = repoUrl.replace(/^git\+/, "").replace(/\.git$/, "");
  if (cleaned.startsWith("git@")) {
    const match = cleaned.match(/^git@([^:]+):(.+)$/);
    if (match) cleaned = `https://${match[1]}/${match[2]}`;
  }
  cachedRepoBaseUrl = cleaned.startsWith("http://") || cleaned.startsWith("https://") ? cleaned : undefined;
  return cachedRepoBaseUrl;
}

function runRepoCommand(args: string): void {
  const command = getRepoCommand();
  if (!command) {
    vscode.window.showErrorMessage("repo binary not found");
    return;
  }
  const root = getWorkspaceRoot();
  if (!root) {
    vscode.window.showErrorMessage("No workspace folder open");
    return;
  }
  const fullCommand = `"${command}" ${args}`;
  log("runRepoCommand:", fullCommand, "cwd:", root);
  const terminal = vscode.window.createTerminal({ name: "semio", cwd: root });
  terminal.show();
  terminal.sendText(fullCommand);
}

async function runRepoCommandJson<T>(args: string): Promise<T | null> {
  const root = getWorkspaceRoot();
  if (!root || !hasRepoAccess()) return null;
  const command = getRepoCommand();
  const fullCommand = `"${command}" --json ${args}`;
  try {
    const { stdout } = await execAsync(fullCommand, { cwd: root, timeout: 60000, maxBuffer: 10 * 1024 * 1024 });
    if (!stdout.trim()) return null;
    const events = parseRepoEvents(stdout);
    const result = extractRepoResult(events);
    if (result && "data" in result) {
      return { data: result.data, output: { exitCode: 0, lines: [] } } as any;
    }
    return result as T;
  } catch (error) {
    logError("[runRepoCommandJson] error:", error);
    return null;
  }
}

export function parseRepoEvents(output: string): RepoEvent[] {
  const lines = output.split("\n").map((line) => line.trim()).filter((line) => line.length > 0);
  return lines.map((line) => JSON.parse(line) as RepoEvent);
}

export function extractRepoResult(events: RepoEvent[]): Record<string, unknown> {
  const results: unknown[] = [];
  const controlKinds = new Set(["start", "progress", "log", "done"]);

  for (const event of events) {
    if (event.kind === "error" && event.error?.fatal) {
      throw new Error(event.error.message ?? "Repo command failed");
    }
    if (event.kind === "result") {
      results.push(event.result ?? event.data ?? null);
    } else if (!event.kind || !controlKinds.has(event.kind)) {
      results.push(event);
    }
  }

  if (results.length > 0 && results.some(r => r && typeof r === 'object' && 'section' in r)) {
    const sections = results.map(r => (r as any).section).filter(s => s);
    if (sections.length > 0) {
      return { data: { sections } };
    }
  }

  let lastResult = results.length > 0 ? results[results.length - 1] : null;

  if (lastResult && typeof lastResult === "object" && !Array.isArray(lastResult)) {
    const res = lastResult as Record<string, unknown>;
    if ("data" in res || "errors" in res) {
      return res;
    }
  }
  return { data: lastResult };
}

// #endregion 🔖Utilities

// #region 🔖URI Resolution

export interface TreeNodeData {
  Kind: string;
  ID: string;
  Label: string;
  URI: string;
  SubKind?: string;
  Description?: string;
  Year?: number;
  Month?: number;
  Day?: number;
  Status?: string;
  Contributor?: string;
  Data?: Record<string, any>;
  Children?: TreeNodeData[];
}

let treeNodeCache: Map<string, TreeNodeData> | null = null;
let treeRootCache: TreeNodeData | null = null;
let treeNodeCacheTime = 0;
const TREE_CACHE_TTL = 30000;

export function extractLeadingEmoji(text: string): string {
  const match = text.match(/^[\p{Emoji_Presentation}\p{Extended_Pictographic}][\u{FE0E}\u{FE0F}\u{200D}\p{Emoji_Component}]*/u);
  return match ? match[0] : "";
}

export function treeNodeDisplayLabel(node: TreeNodeData): string {
  if (node.Kind === "category") return node.Label;
  const emoji = extractLeadingEmoji(node.ID);
  let statusIcon = "";
  if (node.Status === "open") statusIcon = "🔵";
  else if (node.Status === "closed") statusIcon = "🟢";
  const fallbackEmojis: Record<string, string> = {
    contributor: "👤", commit: "🔀", policy: "🛡", violationKind: "⚠",
  };
  const prefix = emoji || fallbackEmojis[node.Kind] || "";
  return `${prefix}${statusIcon}${node.Label}`;
}

export function treeNodeContextValue(node: TreeNodeData): string {
  if (node.Kind === "ticket") return node.Status === "open" ? "ticketOpen" : "ticketClosed";
  return node.Kind;
}

export function treeNodeCommand(node: TreeNodeData): vscode.Command | undefined {
  if (node.Kind === "category") return undefined;
  if (node.URI) return { command: "semio.navigate", title: "Navigate", arguments: [node.URI] };
  return undefined;
}

export function buildCliTreeArgs(fp?: FilterTreeDataProvider): string[] {
  const args: string[] = [];
  if (!fp) return args;
  const query = fp.searchQuery?.trim();
  if (query) args.push(query);
  const ff = fp.filters.file;
  if (!ff.code) args.push("--no-code");
  if (!ff.script) args.push("--no-script");
  if (!ff.config) args.push("--no-config");
  if (!ff.test) args.push("--no-test");
  if (!ff.docs) args.push("--no-docs");
  if (!ff.resource) args.push("--no-resource");
  if (!ff.license) args.push("--no-license");
  if (!fp.filters.section.all) {
    args.push("--no-section", "--no-definition");
  } else {
    const df = fp.filters.definition;
    if (!df.implementation) args.push("--no-implementation");
    if (!df.interface) args.push("--no-interface");
    if (!df.constant) args.push("--no-constant");
  }
  const fo = fp.filters.folder;
  if (!fo.organization && !fo.required) args.push("--no-folder");
  else if (!fo.organization && fo.required) args.push("--only-required");
  else if (fo.organization && !fo.required) args.push("--only-organization");
  const bf = fp.filters.bundle;
  if (!bf.library) args.push("--no-library");
  if (!bf.schema) args.push("--no-schema");
  if (!bf.binary) args.push("--no-binary");
  if (!bf.ui) args.push("--no-client");
  if (!bf.site) args.push("--no-site");
  if (!bf.assets) args.push("--no-assets");
  const gf = fp.filters.goal;
  const tf = fp.filters.ticket;
  if (!gf.open && !gf.closed) args.push("--no-goal");
  if (!tf.open && !tf.closed) args.push("--no-ticket");
  if (gf.open && !gf.closed && tf.open && !tf.closed) args.push("--only-open");
  else if (!gf.open && gf.closed && !tf.open && tf.closed) args.push("--only-closed");
  for (const year of fp.excludedYears) args.push("--no-year", String(year));
  for (const month of fp.excludedMonths) args.push("--no-month", String(month));
  for (const day of fp.excludedDays) args.push("--no-day", String(day));
  if (Object.values(ff).every(v => !v)) args.push("--no-file");
  if (!fp.filters.policy.all) args.push("--no-policy");
  if (!fp.filters.contributor.all) args.push("--no-contributor");
  if (!fp.filters.commit.all) args.push("--no-commit");
  const pf = fp.filters.project;
  if (!pf.user && !pf.infrastructure && !pf.research) args.push("--no-project");
  return args;
}

export function slugify(text: string): string {
  return text.toUpperCase().replace(/[^A-Z0-9]+/g, "-").replace(/^-|-$/g, "");
}

function flattenTree(node: TreeNodeData, result: Map<string, TreeNodeData>): void {
  if (node.URI) {
    result.set(node.URI, node);
  }
  if (node.Children) {
    for (const child of node.Children) {
      flattenTree(child, result);
    }
  }
}

async function getTreeNodeCache(): Promise<Map<string, TreeNodeData>> {
  const now = Date.now();
  if (treeNodeCache && (now - treeNodeCacheTime) < TREE_CACHE_TTL) {
    return treeNodeCache;
  }
  const root = getWorkspaceRoot();
  const command = getRepoCommand();
  if (!root || !command) return new Map();
  try {
    const { stdout } = await execAsync(`"${command}" --json tree`, { cwd: root, timeout: 60000, maxBuffer: 50 * 1024 * 1024 });
    if (!stdout.trim()) return treeNodeCache ?? new Map();
    const events = parseRepoEvents(stdout);
    const result = extractRepoResult(events);
    const tree = result.data as TreeNodeData | undefined;
    if (tree) {
      const cache = new Map<string, TreeNodeData>();
      flattenTree(tree, cache);
      treeNodeCache = cache;
      treeRootCache = tree;
      treeNodeCacheTime = now;
      return cache;
    }
  } catch (error) {
    logError("[getTreeNodeCache] error:", error);
  }
  return treeNodeCache ?? new Map();
}

async function getTreeRoot(): Promise<TreeNodeData | null> {
  await getTreeNodeCache();
  return treeRootCache;
}

async function fetchTreeWithArgs(args: string[]): Promise<TreeNodeData | null> {
  const root = getWorkspaceRoot();
  const command = getRepoCommand();
  if (!root || !command) return null;
  try {
    const fullArgs = ["--json", "tree", ...args];
    const { stdout } = await execFileAsync(command, fullArgs, { cwd: root, timeout: 60000, maxBuffer: 50 * 1024 * 1024 });
    if (!stdout.trim()) return null;
    const events = parseRepoEvents(stdout);
    const result = extractRepoResult(events);
    return result.data as TreeNodeData | null;
  } catch (error) {
    logError("[fetchTreeWithArgs] error:", error);
    return null;
  }
}

export function invalidateTreeNodeCache(): void {
  treeNodeCache = null;
  treeRootCache = null;
  treeNodeCacheTime = 0;
}

export function parseUri(uri: string): { type: string; path: string } | null {
  const match = uri.match(/^semiorepo:\/\/([a-zA-Z]+)\/(.*)/);
  if (!match) return null;
  return { type: match[1], path: match[2] };
}

async function navigateToUri(uri: string): Promise<void> {
  const wsRoot = getWorkspaceRoot();
  if (!wsRoot) return;
  const parsed = parseUri(uri);
  if (!parsed) return;

  switch (parsed.type) {
    case "ticket": {
      const ticketMdPath = path.join(wsRoot, ".semio-repo", "tickets", parsed.path, "ticket.md");
      if (fs.existsSync(ticketMdPath)) {
        return vscode.commands.executeCommand("semio.navigateToFile", ticketMdPath) as any;
      }
      break;
    }
    case "goal": {
      const goalJsonPath = path.join(wsRoot, ".semio-repo", "goals", parsed.path, "goal.json");
      if (fs.existsSync(goalJsonPath)) {
        return vscode.commands.executeCommand("semio.navigateToFile", goalJsonPath) as any;
      }
      break;
    }
    case "draft": {
      const draftPath = path.join(wsRoot, ".semio-repo", "drafts", parsed.path);
      if (fs.existsSync(draftPath)) {
        return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(draftPath)) as any;
      }
      break;
    }
    case "contributor": {
      const github = parsed.path.toLowerCase().replace(/-/g, "");
      return vscode.env.openExternal(vscode.Uri.parse(`https://github.com/${github}`)) as any;
    }
    case "commit": {
      const sha = parsed.path.toLowerCase().replace(/-/g, "");
      const baseUrl = getGitHubRepoBaseUrl();
      if (baseUrl) {
        return vscode.env.openExternal(vscode.Uri.parse(`${baseUrl}/commit/${sha}`)) as any;
      }
      break;
    }
    default: {
      const cache = await getTreeNodeCache();
      const node = cache.get(uri);
      if (!node) break;

      switch (node.Kind) {
        case "project": {
          const name = node.Data?.name || node.Label;
          const abs = path.join(wsRoot, name);
          if (fs.existsSync(abs)) {
            return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(abs)) as any;
          }
          break;
        }
        case "bundle": {
          const root = node.Data?.root;
          if (root) {
            const abs = path.join(wsRoot, root);
            if (fs.existsSync(abs)) {
              return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(abs)) as any;
            }
          }
          break;
        }
        case "folder": {
          const folderPath = node.Data?.path || node.Label;
          const abs = path.join(wsRoot, folderPath);
          if (fs.existsSync(abs)) {
            return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(abs)) as any;
          }
          break;
        }
        case "file": {
          const filePath = node.Data?.path || node.Label;
          return vscode.commands.executeCommand("semio.navigateToFile", filePath) as any;
        }
        case "section": {
          const sectionId = node.ID.replace(/^section:/, "");
          const hashIdx = sectionId.indexOf("#");
          if (hashIdx >= 0) {
            const filePath = sectionId.substring(0, hashIdx);
            const sectionPath = sectionId.substring(hashIdx + 1);
            const binaryPath = getRepoBinaryPath();
            if (binaryPath) {
              try {
                const { stdout } = await execAsync(`"${binaryPath}" --json section list --file "${filePath}"`, { cwd: wsRoot, timeout: 15000 });
                const events = parseRepoEvents(stdout);
                for (const event of events) {
                  const section = (event as any).section;
                  if (section) {
                    if (findSectionByPath(section, sectionPath)) {
                      const found = findSectionByPath(section, sectionPath)!;
                      return openFileAtLine(filePath, found.startLine, found.endLine);
                    }
                  }
                }
              } catch { /* fall through */ }
            }
            return vscode.commands.executeCommand("semio.navigateToFile", filePath) as any;
          }
          break;
        }
        case "definition": {
          const defId = node.ID.replace(/^definition:/, "");
          const sepIdx = defId.indexOf("§");
          if (sepIdx >= 0) {
            const fileSection = defId.substring(0, sepIdx);
            const hashIdx = fileSection.indexOf("#");
            const filePath = hashIdx >= 0 ? fileSection.substring(0, hashIdx) : fileSection;
            const binaryPath = getRepoBinaryPath();
            if (binaryPath) {
              try {
                const { stdout } = await execAsync(`"${binaryPath}" --json definition list --file "${filePath}"`, { cwd: wsRoot, timeout: 15000 });
                const events = parseRepoEvents(stdout);
                for (const event of events) {
                  const def = (event as any).definition;
                  if (def && def.name === node.Label && def.startLine) {
                    return openFileAtLine(filePath, def.startLine, def.endLine);
                  }
                }
              } catch { /* fall through */ }
            }
            return vscode.commands.executeCommand("semio.navigateToFile", filePath) as any;
          }
          break;
        }
        case "policy": {
          vscode.window.showInformationMessage(`Policy: ${node.Label}${node.Description ? " - " + node.Description : ""}`);
          break;
        }
        case "violationKind": {
          vscode.window.showInformationMessage(`Violation Kind: ${node.Label}${node.Description ? " - " + node.Description : ""}`);
          break;
        }
      }
      break;
    }
  }
}

function findSectionByPath(section: any, sectionPath: string): any | null {
  const parts = sectionPath.split("/");
  if (slugify(section.name) === slugify(parts[0]) || section.name === parts[0]) {
    if (parts.length === 1) return section;
    const rest = parts.slice(1).join("/");
    for (const child of section.children || []) {
      const found = findSectionByPath(child, rest);
      if (found) return found;
    }
  }
  return null;
}

// #endregion 🔖URI Resolution

// #region 🔖Helpers

function extractFilePathFromScope(scope: string): string | undefined {
  let cleanScope = scope;
  if (cleanScope.startsWith("@semio/violations/")) {
    cleanScope = cleanScope.replace("@semio/violations/", "");
  }

  let bestBundle: BundleInfo | undefined;
  for (const b of bundleCache) {
    if (cleanScope.startsWith(b.id + "/")) {
      if (!bestBundle || b.id.length > bestBundle.id.length) {
        bestBundle = b;
      }
    } else if (cleanScope === b.id) {
      if (!bestBundle || b.id.length > bestBundle.id.length) {
        bestBundle = b;
      }
    }
  }

  if (bestBundle) {
    const relPath = cleanScope === bestBundle.id ? "" : cleanScope.substring(bestBundle.id.length + 1);
    const parts = relPath.split(/[#§:]/);
    const fileName = parts[0];
    const root = bestBundle.root === "." ? "" : (bestBundle.root.endsWith("/") ? bestBundle.root : bestBundle.root + "/");
    const filePath = root + fileName;
    return filePath.endsWith("/") ? filePath.slice(0, -1) : filePath;
  }

  if (cleanScope.startsWith("@semio/") || cleanScope.startsWith("@semio-repo/")) {
    const parts = cleanScope.split("/");
    if (parts.length > 2) {
      cleanScope = parts.slice(2).join("/");
    }
  }

  const parts = cleanScope.split(/[#§:]/);
  const filePath = parts[0];
  if (filePath.endsWith(".ts") || filePath.endsWith(".tsx") || filePath.endsWith(".js") || filePath.endsWith(".json") || filePath.endsWith(".py") || filePath.endsWith(".cs") || filePath.endsWith(".go") || filePath.endsWith(".sh")) {
    return filePath;
  }
  return undefined;
}

function resolveTicketPath(ticket: { year: number; month: number; day: number; slug: string; filePath?: string }): string | undefined {
  if (ticket.filePath) return ticket.filePath;
  const root = getWorkspaceRoot();
  if (!root) return undefined;

  const relPath = path.join(String(ticket.year), String(ticket.month).padStart(2, "0"), String(ticket.day).padStart(2, "0"), ticket.slug, "ticket.md");
  const metaPath = path.join(root, ".semio-repo", "tickets", relPath);
  if (fs.existsSync(metaPath)) {
    return metaPath;
  }
  return path.join(root, "tickets", relPath);
}

async function openFileAtLine(filePath: string, startLine: number, endLine?: number): Promise<void> {
  const root = getWorkspaceRoot();
  if (!root) return;
  const abs = path.isAbsolute(filePath) ? filePath : path.join(root, filePath);
  const uri = vscode.Uri.file(abs);
  const doc = await vscode.workspace.openTextDocument(uri);
  const editor = await vscode.window.showTextDocument(doc);
  const startPos = new vscode.Position(Math.max(0, startLine - 1), 0);
  const endPos = typeof endLine === "number" ? new vscode.Position(Math.max(0, endLine - 1), 0) : startPos;
  const range = new vscode.Range(startPos, endPos);
  editor.selection = new vscode.Selection(startPos, startPos);
  editor.revealRange(range, vscode.TextEditorRevealType.InCenter);
}

// #endregion 🔖Helpers

// #region 🔖File Analysis & Diagnostics

async function updateBundleCache() {
  const root = await getTreeRoot();
  if (!root) return;
  const bundles: BundleInfo[] = [];
  function walk(node: TreeNodeData) {
    if (node.Kind === "bundle" && node.Data) {
      bundles.push({ id: node.Data.name || node.Label, root: node.Data.root || "" });
    }
    for (const child of node.Children || []) walk(child);
  }
  walk(root);
  if (bundles.length > 0) bundleCache = bundles;
}

const ignoredDirectories = new Set([
  "node_modules", "venv", "dist", "build", "out", "__pycache__", "coverage", "site-packages", "eggs", "wheels", "htmlcov", "target", "artifacts", "vendor"
]);
const allowedDotDirectories = new Set([".github", ".devcontainer", ".semio-repo"]);

function isInIgnoredDirectory(relativePath: string): boolean {
  const segments = relativePath.split("/");
  return segments.some((segment) => {
    if (ignoredDirectories.has(segment)) return true;
    if (segment.startsWith(".") && !allowedDotDirectories.has(segment)) return true;
    return false;
  });
}

function shouldAnalyzeFile(document: vscode.TextDocument): boolean {
  const supportedLanguages = ["typescript", "javascript", "typescriptreact", "javascriptreact", "json", "python", "csharp", "go", "shellscript"];
  return supportedLanguages.includes(document.languageId);
}

async function analyzeFile(document: vscode.TextDocument): Promise<void> {
  if (!shouldAnalyzeFile(document)) return;
  if (document.uri.scheme !== "file") return;
  const root = getWorkspaceRoot();
  if (!root) return;

  if (bundleCache.length === 0) {
    await updateBundleCache();
  }

  const relativePath = path.relative(root, document.uri.fsPath).replace(/\\/g, "/");
  if (relativePath.startsWith("..")) return;
  if (isInIgnoredDirectory(relativePath)) return;
  const fileUri = vscode.Uri.file(path.join(root, relativePath));
  const processKey = `analyze:${relativePath}`;

  if (runningProcesses.has(processKey)) {
    runningProcesses.get(processKey)?.abort();
    runningProcesses.delete(processKey);
  }

  const controller = new AbortController();
  runningProcesses.set(processKey, controller);

  try {
    const result = await runRepoCommandJson<ToolResult<AnalyzeReport>>(`analyze "${relativePath}"`);
    if (controller.signal.aborted) return;

    if (result?.data?.violations) {
      fileViolationsMap.set(fileUri.toString(), result.data.violations);
      updateFileDiagnostics(document, result.data.violations);
    } else {
      repoDiagnosticCollection.delete(fileUri);
    }
  } catch (error) {
    if (!controller.signal.aborted) {
      logError("Error analyzing file:", error);
    }
  } finally {
    runningProcesses.delete(processKey);
  }
}

function updateFileDiagnostics(document: vscode.TextDocument, violations: Violation[]): void {
  const root = getWorkspaceRoot();
  if (!root) return;
  const diagnosticsByUri = new Map<string, { uri: vscode.Uri; diagnostics: vscode.Diagnostic[] }>();

  diagnosticsByUri.set(document.uri.toString(), { uri: document.uri, diagnostics: [] });

  for (const violation of violations) {
    const filePath = extractFilePathFromScope(violation.scope);
    if (!filePath) continue;
    const absPath = path.join(root, filePath);
    const fileUri = vscode.Uri.file(absPath);
    const uriKey = fileUri.toString();
    if (!diagnosticsByUri.has(uriKey)) {
      diagnosticsByUri.set(uriKey, { uri: fileUri, diagnostics: [] });
    }
    const line = Math.max(0, (violation.line ?? 1) - 1);
    const column = Math.max(0, (violation.column ?? 1) - 1);
    const endColumn = violation.excerpt ? column + violation.excerpt.length : column + 1;
    const range = new vscode.Range(line, column, line, endColumn);
    const severity = vscode.DiagnosticSeverity.Warning;
    let kindId = violation.kind.id;
    if (kindId.startsWith("@semio/policies//violations/")) {
      kindId = kindId.replace("@semio/policies//violations/", "");
    }
    const diagnostic = new vscode.Diagnostic(range, violation.summary, severity);
    diagnostic.source = DIAGNOSTIC_SOURCE;
    diagnostic.code = { value: kindId, target: fileUri.with({ fragment: `L${line + 1}` }) };
    diagnosticsByUri.get(uriKey)!.diagnostics.push(diagnostic);
  }
  for (const { uri, diagnostics } of diagnosticsByUri.values()) {
    repoDiagnosticCollection.set(uri, diagnostics);
  }
}

async function fixViolation(relativePath: string): Promise<void> {
  const root = getWorkspaceRoot();
  if (!root) return;
  if (!hasRepoAccess()) {
    vscode.window.showErrorMessage("repo binary not found in go/repo/");
    return;
  }
  const command = getRepoCommand();
  try {
    await vscode.window.withProgress({ location: vscode.ProgressLocation.Notification, title: "Fixing violation..." }, async () => {
      const { stderr } = await execAsync(`"${command}" fix "${relativePath}"`, { cwd: root, timeout: 30000 });
      if (stderr) log("Fix stderr:", stderr);
      const absPath = path.join(root, relativePath);
      const uri = vscode.Uri.file(absPath);
      const openDoc = vscode.workspace.textDocuments.find((d) => d.uri.fsPath === absPath);
      if (openDoc) {
        const newContent = fs.readFileSync(absPath, "utf-8");
        const edit = new vscode.WorkspaceEdit();
        const fullRange = new vscode.Range(openDoc.positionAt(0), openDoc.positionAt(openDoc.getText().length));
        edit.replace(uri, fullRange, newContent);
        await vscode.workspace.applyEdit(edit);
        await analyzeFile(openDoc);
      }
    });
    vscode.window.showInformationMessage(`Fixed: ${relativePath}`);
  } catch (error) {
    logError("Failed to run fix:", error);
    vscode.window.showErrorMessage(`Failed to fix violation: ${error}`);
  }
}

function isKitDocument(document: vscode.TextDocument): boolean {
  if (document.languageId !== SEMIO_KIT_LANGUAGE) return false;
  const basename = document.uri.path.split("/").pop()?.toLowerCase() || "";
  return basename.startsWith("kit_") || basename.includes("_kit") || basename === "kit.json";
}

function validateKitDocument(document: vscode.TextDocument): void {
  if (!isKitDocument(document)) return;
  try {
    const text = document.getText();
    const kit = deserializeKit(text);
    const result = validateKit(kit);
    const diagnostics = result.problems.map((problem: Problem) => {
      return new vscode.Diagnostic(new vscode.Range(0, 0, 0, 0), problem.message);
    });
    kitDiagnosticCollection.set(document.uri, diagnostics);
  } catch (error) {
    logError("Failed to validate semio kit:", error);
    kitDiagnosticCollection.delete(document.uri);
  }
}

// #endregion 🔖File Analysis & Diagnostics

// #region 🔖Providers

export class FilterTreeItem extends vscode.TreeItem {
  constructor(
    public readonly label: string,
    public readonly type: "root" | "search" | "filter" | "time" | "filterOption" | "timeValue",
    public readonly collapsibleState: vscode.TreeItemCollapsibleState = vscode.TreeItemCollapsibleState.None,
    public readonly contextValue?: string,
    public readonly filterKey?: string,
    public readonly filterValue?: any
  ) {
    super(label, collapsibleState);
    this.contextValue = contextValue;
  }
}

export class FilterTreeDataProvider implements vscode.TreeDataProvider<FilterTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<FilterTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  public searchQuery: string = "";
  public matchCase: boolean = false;
  public matchWholeWord: boolean = false;
  public useRegex: boolean = false;

  public filters: Record<string, Record<string, boolean>> = {
    project: { user: true, infrastructure: true, research: true },
    bundle: { library: true, binary: true, ui: true, site: true, assets: true, schema: true, default: true },
    folder: { organization: true, required: true },
    file: { code: true, script: true, config: true, test: true, docs: true, resource: true, license: true },
    section: { none: false, all: true },
    definition: { implementation: true, interface: true, constant: true },
    goal: { open: true, closed: true },
    ticket: { open: true, closed: true },
    policy: { all: true },
    contributor: { all: true },
    commit: { all: true },
  };

  public timeFilter: Record<string, boolean> = { none: false, all: true };
  public excludedYears: number[] = [];
  public excludedMonths: number[] = [];
  public excludedDays: number[] = [];

  constructor() {
    this.updateContextKeys();
  }

  refresh(): void {
    this.updateContextKeys();
    this._onDidChangeTreeData.fire();
  }

  updateContextKeys(): void {
    for (const [kind, values] of Object.entries(this.filters)) {
      for (const [key, enabled] of Object.entries(values)) {
        vscode.commands.executeCommand("setContext", `semio.filter.${kind}.${key}`, enabled);
      }
    }
  }

  public availableYears: number[] = [];
  public availableMonths: number[] = [];
  public availableDays: number[] = [];
  public availableContributors: string[] = [];
  public availablePolicies: string[] = [];

  getTreeItem(element: FilterTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: FilterTreeItem): Promise<FilterTreeItem[]> {
    if (!element) {
      return [
        this.createSearchItem(),
        this.createFilterItem("🏗️Projects", "filter_project", "Projects filter"),
        this.createFilterItem("📦Bundles", "filter_bundle", "Bundles filter"),
        this.createFilterItem("📂Folders", "filter_folder", "Folders filter"),
        this.createFilterItem("📄Files", "filter_file", "Files filter"),
        this.createFilterItem("🔖Sections", "filter_section", "Sections filter"),
        this.createFilterItem("🏷Definitions", "filter_definition", "Definitions filter"),
        this.createFilterItem("🎯Goals", "filter_goal", "Goals filter"),
        this.createFilterItem("🎫Tickets", "filter_ticket", "Tickets filter"),
        this.createFilterItem("🎫Dates", "filter_time", "Dates filter", vscode.TreeItemCollapsibleState.Collapsed),
        this.createFilterItem("🛡Policies", "filter_policy", "Policies filter"),
        this.createFilterItem("👤Contributors", "filter_contributor", "Contributors filter"),
        this.createFilterItem("🔄Commits", "filter_commit", "Commits filter"),
      ];
    }

    if (element.contextValue === "filter_time") {
      return this.availableYears.map(y => {
        const excluded = this.excludedYears.includes(y);
        const item = new FilterTreeItem(
          String(y), "timeValue", vscode.TreeItemCollapsibleState.Collapsed, "filter_time_year", "year", y
        );
        item.tooltip = excluded ? `Excluded year ${y}` : `Included year ${y}`;
        item.command = { command: "semio.filter.toggleYear", title: "Toggle Year", arguments: [y] };
        return item;
      });
    }

    if (element.contextValue === "filter_time_year") {
      const year = element.filterValue;
      return this.availableMonths.map(m => {
        const excluded = this.excludedMonths.includes(m);
        const label = new Date(2000, m - 1, 1).toLocaleString("default", { month: "long" });
        const item = new FilterTreeItem(
          label, "timeValue", vscode.TreeItemCollapsibleState.Collapsed, "filter_time_month", "month", m
        );
        item.tooltip = excluded ? `Excluded month ${label}` : `Included month ${label}`;
        item.command = { command: "semio.filter.toggleMonth", title: "Toggle Month", arguments: [m] };
        return item;
      });
    }

    if (element.contextValue === "filter_time_month") {
      return this.availableDays.map(d => {
        const excluded = this.excludedDays.includes(d);
        const item = new FilterTreeItem(
          String(d).padStart(2, "0"), "timeValue", vscode.TreeItemCollapsibleState.None, "filter_time_day", "day", d
        );
        item.tooltip = excluded ? `Excluded day ${d}` : `Included day ${d}`;
        item.command = { command: "semio.filter.toggleDay", title: "Toggle Day", arguments: [d] };
        return item;
      });
    }

    return [];
  }

  private createSearchItem(): FilterTreeItem {
    const item = new FilterTreeItem("🔍Search", "search", vscode.TreeItemCollapsibleState.None, "filter_search");
    const details = [
      this.searchQuery ? `Query: ${this.searchQuery}` : "No query set",
      this.matchCase ? "Match case on" : "Match case off",
      this.matchWholeWord ? "Whole word on" : "Whole word off",
      this.useRegex ? "Regex on" : "Regex off",
    ];
    item.tooltip = `Search filter\n${details.join("\n")}`;
    item.command = { command: "semio.filter.search", title: "Search" };
    return item;
  }

  private createFilterItem(
    label: string,
    contextValue: string,
    tooltip: string,
    collapsibleState: vscode.TreeItemCollapsibleState = vscode.TreeItemCollapsibleState.None
  ): FilterTreeItem {
    const item = new FilterTreeItem(label, "filter", collapsibleState, contextValue);
    item.tooltip = tooltip;
    return item;
  }

  toggle(kind: string, key: string) {
    const filterKeys = this.filters[kind] ? Object.keys(this.filters[kind]) : [];
    const hasRealKeys = filterKeys.some(k => k !== "none" && k !== "all");
    if ((key === "none" || key === "all") && this.filters[kind] && hasRealKeys) {
      for (const k of Object.keys(this.filters[kind])) this.filters[kind][k] = key === "all";
      this.refresh();
      monorepoProvider?.refresh();
      return;
    }
    if (kind === "time") {
      if (key === "all") {
        this.excludedYears = [];
        this.excludedMonths = [];
        this.excludedDays = [];
        this.timeFilter.all = true;
        this.timeFilter.none = false;
      }
      else if (key === "none") {
        this.excludedYears = [...this.availableYears];
        this.excludedMonths = [...this.availableMonths];
        this.excludedDays = [...this.availableDays];
        this.timeFilter.all = false;
        this.timeFilter.none = true;
      }
    } else if (this.filters[kind]) {
      this.filters[kind][key] = !this.filters[kind][key];
    }
    this.refresh();
    monorepoProvider?.refresh();
  }

  setTimeMode(kind: "year" | "month" | "day", mode: "all" | "none") {
    if (kind === "year") this.excludedYears = mode === "all" ? [] : [...this.availableYears];
    if (kind === "month") this.excludedMonths = mode === "all" ? [] : [...this.availableMonths];
    if (kind === "day") this.excludedDays = mode === "all" ? [] : [...this.availableDays];
    this.refresh();
    monorepoProvider?.refresh();
  }

  toggleYear(year: number) {
    if (this.excludedYears.includes(year)) this.excludedYears = this.excludedYears.filter(y => y !== year);
    else this.excludedYears.push(year);
    this.refresh();
    monorepoProvider?.refresh();
  }

  toggleMonth(month: number) {
    if (this.excludedMonths.includes(month)) this.excludedMonths = this.excludedMonths.filter(m => m !== month);
    else this.excludedMonths.push(month);
    this.refresh();
    monorepoProvider?.refresh();
  }

  toggleDay(day: number) {
    if (this.excludedDays.includes(day)) this.excludedDays = this.excludedDays.filter(d => d !== day);
    else this.excludedDays.push(day);
    this.refresh();
    monorepoProvider?.refresh();
  }
}

export class MonorepoTreeItem extends vscode.TreeItem {
  constructor(
    public readonly label: string,
    public readonly collapsibleState: vscode.TreeItemCollapsibleState,
    public readonly contextValue?: string,
    public readonly data?: any,
    public readonly nodeId?: string
  ) {
    super(label, collapsibleState);
    this.contextValue = contextValue;
    if (nodeId) this.tooltip = nodeId;
  }
}

function treeNodeToItem(node: TreeNodeData): MonorepoTreeItem {
  const label = treeNodeDisplayLabel(node);
  const hasChildren = (node.Children && node.Children.length > 0);
  const collapsible = hasChildren ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None;
  const ctx = treeNodeContextValue(node);
  const item = new MonorepoTreeItem(label, collapsible, ctx, node, node.ID || undefined);
  item.command = treeNodeCommand(node);
  if (node.Description) item.tooltip = node.Description;
  if (node.Kind === "commit" && node.Data?.sha) item.description = node.Data.sha.substring(0, 7);
  if (node.Kind === "violationKind") {
    item.description = node.Data?.autofixable ? "🔧" : "";
    if (node.Description) item.tooltip = node.Description;
  }
  return item;
}

export class MonorepoTreeDataProvider implements vscode.TreeDataProvider<MonorepoTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<MonorepoTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  constructor(public filterProvider?: FilterTreeDataProvider) { }

  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  refreshItem(item?: MonorepoTreeItem): void {
    this._onDidChangeTreeData.fire(item);
  }

  getTreeItem(element: MonorepoTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: MonorepoTreeItem): Promise<MonorepoTreeItem[]> {
    if (!element) {
      const args = buildCliTreeArgs(this.filterProvider);
      const tree = await fetchTreeWithArgs(args);
      if (!tree?.Children) return [];
      return tree.Children.map(treeNodeToItem);
    }
    const node = element.data as TreeNodeData;
    if (!node?.Children) return [];
    return node.Children.map(treeNodeToItem);
  }

}

class SectionTreeItem extends vscode.TreeItem {
  constructor(
    public section: SectionInfo,
    public filePath: string
  ) {
    super(
      section.name,
      (section.children && section.children.length > 0)
        ? vscode.TreeItemCollapsibleState.Collapsed
        : vscode.TreeItemCollapsibleState.None
    );
    this.contextValue = "section";
    this.iconPath = new vscode.ThemeIcon("bookmark");
    this.tooltip = `Section: ${section.name}`;
    const start = section.startLine - 1;
    this.command = {
      command: "vscode.open",
      title: "Open Section",
      arguments: [
        vscode.Uri.file(path.join(getWorkspaceRoot() || "", filePath)),
        { selection: new vscode.Range(start, 0, start, 0) }
      ]
    };
  }
}

export class SectionsTreeDataProvider implements vscode.TreeDataProvider<SectionTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<SectionTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;
  private activeEditor: vscode.TextEditor | undefined;

  constructor(private context: vscode.ExtensionContext) {
    this.activeEditor = vscode.window.activeTextEditor;
    vscode.window.onDidChangeActiveTextEditor(editor => {
      this.activeEditor = editor;
      this.refresh();
    });
    vscode.workspace.onDidChangeTextDocument(e => {
      if (this.activeEditor && e.document.uri.toString() === this.activeEditor.document.uri.toString()) {
        this.refresh();
      }
    });
  }

  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(element: SectionTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: SectionTreeItem): Promise<SectionTreeItem[]> {
    if (!this.activeEditor) return [];

    // Use relative path for repo commands
    const root = getWorkspaceRoot();
    if (!root) return [];

    const uri = this.activeEditor.document.uri;
    const filePath = path.relative(root, uri.fsPath);
    if (!filePath || filePath.startsWith("..")) return [];

    if (element) {
      return this.createSectionItems(element.section.children || [], filePath);
    } else {
      const binaryPath = getRepoBinaryPath();
      if (!binaryPath) return [];

      try {
        const output = await execShell(`"${binaryPath}" section list --file "${filePath}" --json`, root);

        const sections: SectionInfo[] = [];
        const lines = output.split("\n");
        for (const line of lines) {
          if (!line.trim()) continue;
          try {
            const parsed = JSON.parse(line);
            if (parsed.section) {
              sections.push(parsed.section);
            }
          } catch (e) {
            // Ignore parse errors
          }
        }
        return this.createSectionItems(sections, filePath);
      } catch (e) {
        console.error("Failed to fetch sections:", e);
        return [];
      }
    }
  }

  private createSectionItems(sections: SectionInfo[], filePath: string): SectionTreeItem[] {
    return sections.map(s => {
      const item = new SectionTreeItem(s, filePath);
      // Map JSON keys as sections effectively
      // The CLI output already includes children recursively in the result event if filtered? 
      // No, CLI section list returns roots.
      return item;
    });
  }
}

// #endregion 🔖Providers

// #region 🔖Activation

function registerSidebarViews(context: vscode.ExtensionContext): void {
  filterProvider = new FilterTreeDataProvider();
  vscode.window.registerTreeDataProvider("semio.filter", filterProvider);

  monorepoProvider = new MonorepoTreeDataProvider(filterProvider);
  vscode.window.registerTreeDataProvider("semio.monorepo", monorepoProvider);

  const sectionsProvider = new SectionsTreeDataProvider(context);
  vscode.window.registerTreeDataProvider("semio.sections", sectionsProvider);
}

function registerCommands(context: vscode.ExtensionContext): void {
  const registered = new Set<string>();
  const register = (command: string, handler: (...args: any[]) => any): void => {
    if (registered.has(command)) return;
    registered.add(command);
    context.subscriptions.push(vscode.commands.registerCommand(command, handler));
  };

  register("semio.copyId", (item: MonorepoTreeItem) => {
    const id = item?.nodeId || (typeof item?.label === "string" ? item.label : "");
    if (id) {
      vscode.env.clipboard.writeText(id);
      vscode.window.showInformationMessage(`Copied: ${id}`);
    }
  });

  register("semio.mailto", (email: string) => {
    if (email) vscode.env.openExternal(vscode.Uri.parse(`mailto:${email}`));
  });

  register("semio.openLink", (url: string) => {
    if (url) vscode.env.openExternal(vscode.Uri.parse(url));
  });

  register("semio.refreshMonorepo", () => {
    monorepoProvider?.refresh();
  });

  register("semio.refreshCodebase", () => {
    filterProvider?.refresh();
    monorepoProvider?.refresh();
  });

  register("semio.refreshItem", (item: MonorepoTreeItem) => {
    monorepoProvider?.refreshItem(item);
  });

  register("semio.filter.search", async () => {
    const q = await vscode.window.showInputBox({ prompt: "Search..." });
    if (q !== undefined && filterProvider) {
      filterProvider.searchQuery = q;
      filterProvider.refresh();
      monorepoProvider?.refresh();
    }
  });

  register("semio.filter.toggle", (kind: string, key: string) => {
    filterProvider?.toggle(kind, key);
  });

  const filterToggleEntries: Record<string, string[]> = {
    bundle: ["library", "binary", "ui", "site", "assets", "schema", "default", "none", "all"],
    project: ["user", "infrastructure", "research", "none", "all"],
    folder: ["organization", "required", "none", "all"],
    file: ["code", "script", "config", "test", "docs", "resource", "license", "none", "all"],
    section: ["none", "all"],
    definition: ["implementation", "interface", "constant", "none", "all"],
    goal: ["open", "closed", "none", "all"],
    ticket: ["open", "closed", "none", "all"],
    policy: ["none", "all"],
    contributor: ["none", "all"],
    commit: ["none", "all"],
    time: ["none", "all"],
  };
  for (const [kind, keys] of Object.entries(filterToggleEntries)) {
    for (const key of keys) {
      register(`semio.filter.toggle.${kind}.${key}`, () => filterProvider?.toggle(kind, key));
    }
  }

  const timeModes: Array<["year" | "month" | "day", "none" | "all"]> = [
    ["year", "none"], ["year", "all"], ["month", "none"], ["month", "all"], ["day", "none"], ["day", "all"],
  ];
  for (const [unit, mode] of timeModes) {
    register(`semio.filter.time.${unit}.${mode}`, () => filterProvider?.setTimeMode(unit, mode));
  }

  register("semio.filter.toggleYear", (year: number) => filterProvider?.toggleYear(year));
  register("semio.filter.toggleMonth", (month: number) => filterProvider?.toggleMonth(month));
  register("semio.filter.toggleDay", (day: number) => filterProvider?.toggleDay(day));

  const searchToggles: Array<[string, keyof FilterTreeDataProvider]> = [
    ["semio.filter.search.matchCase", "matchCase"],
    ["semio.filter.search.wholeWord", "matchWholeWord"],
    ["semio.filter.search.regex", "useRegex"],
  ];
  for (const [cmd, prop] of searchToggles) {
    register(cmd, () => {
      if (filterProvider) {
        (filterProvider as any)[prop] = !(filterProvider as any)[prop];
        filterProvider.refresh();
        monorepoProvider?.refresh();
      }
    });
  }

  const revealInExplorer = (targetPath: string) => {
    const wsRoot = getWorkspaceRoot();
    if (!wsRoot) return;
    const abs = path.isAbsolute(targetPath) ? targetPath : path.join(wsRoot, targetPath);
    return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(abs));
  };
  register("semio.navigateToBundle", revealInExplorer);
  register("semio.navigateToFolder", revealInExplorer);

  register("semio.navigateToFile", async (filePath: string) => {
    const root = getWorkspaceRoot();
    if (root) {
      const abs = path.isAbsolute(filePath) ? filePath : path.join(root, filePath);
      const uri = vscode.Uri.file(abs);
      try {
        const doc = await vscode.workspace.openTextDocument(uri);
        await vscode.window.showTextDocument(doc);
      } catch (e) {
        vscode.window.showErrorMessage(`Failed to open file: ${filePath}`);
      }
    }
  });

  const navigateToRangedItem = (payload: any, rangeKey: string) => {
    const filePath = payload?.filePath;
    const item = payload?.[rangeKey];
    if (!filePath || typeof item?.range?.start !== "number") return;
    return openFileAtLine(filePath, item.range.start, item.range.end ?? undefined);
  };
  register("semio.navigateToSection", (s: any) => navigateToRangedItem(s, "section"));
  register("semio.navigateToDefinition", (d: any) => navigateToRangedItem(d, "definition"));

  register("semio.navigate", async (target: string) => {
    if (!target) return;
    if (target.startsWith("semiorepo://")) {
      return navigateToUri(target);
    }
    const cache = await getTreeNodeCache();
    for (const [uri, node] of cache) {
      if (node.ID === target || node.Label === target || slugify(node.Label) === slugify(target)) {
        return navigateToUri(uri);
      }
    }
  });

  register("semio.navigateTo", async () => {
    const cache = await getTreeNodeCache();
    const items: vscode.QuickPickItem[] = [];
    for (const [uri, node] of cache) {
      if (node.Kind === "category") continue;
      items.push({ label: node.Label, description: node.Kind, detail: uri });
    }
    const picked = await vscode.window.showQuickPick(items, { placeHolder: "Navigate to..." });
    if (picked?.detail) {
      return navigateToUri(picked.detail);
    }
  });

  register("semio.ticketOpen", (item: MonorepoTreeItem) => {
    const node = item?.data as TreeNodeData | undefined;
    if (!node) return;
    const year = node.Year ?? node.Data?.year;
    const month = node.Month ?? node.Data?.month;
    const day = node.Day ?? node.Data?.day;
    const slug = node.Data?.slug ?? node.Label;
    if (!year || !month || !day || !slug) return;
    const t = { year, month, day, slug, filePath: undefined as string | undefined };
    const p = resolveTicketPath(t);
    if (!p) return;
    return vscode.commands.executeCommand("semio.navigateToFile", p);
  });

  register("semio.ticketClose", (item: MonorepoTreeItem) => {
    const node = item?.data as TreeNodeData | undefined;
    if (!node) return;
    const year = node.Year ?? node.Data?.year;
    const month = node.Month ?? node.Data?.month;
    const day = node.Day ?? node.Data?.day;
    const slug = node.Data?.slug ?? node.Label;
    if (!year || !month || !day || !slug) return;
    const ticketId = `${year}/${String(month).padStart(2, "0")}/${String(day).padStart(2, "0")}/${slug}`;
    return vscode.window.showInformationMessage(`Close ticket: ${ticketId}?`, "Yes", "No").then(answer => {
      if (answer === "Yes") {
        const binaryPath = getRepoBinaryPath();
        if (!binaryPath) return;
        const cp = require("child_process");
        cp.execSync(`${binaryPath} ticket close ${ticketId} "Closed via VS Code" .`, { cwd: getWorkspaceRoot() });
        monorepoProvider?.refresh();
      }
    });
  });

  register("semio.ticketReopen", (item: MonorepoTreeItem) => {
    const node = item?.data as TreeNodeData | undefined;
    if (!node) return;
    const year = node.Year ?? node.Data?.year;
    const month = node.Month ?? node.Data?.month;
    const day = node.Day ?? node.Data?.day;
    const slug = node.Data?.slug ?? node.Label;
    if (!year || !month || !day || !slug) return;
    const ticketId = `${year}/${String(month).padStart(2, "0")}/${String(day).padStart(2, "0")}/${slug}`;
    return vscode.window.showInputBox({ prompt: "Reopen prompt" }).then(prompt => {
      if (!prompt) return;
      const binaryPath = getRepoBinaryPath();
      if (!binaryPath) return;
      const cp = require("child_process");
      cp.execSync(`${binaryPath} ticket reopen ${ticketId} "${prompt}" copilot-chat`, { cwd: getWorkspaceRoot() });
      monorepoProvider?.refresh();
    });
  });

  register("semio.copyCommitSha", (item: MonorepoTreeItem) => {
    const node = item?.data as TreeNodeData | undefined;
    const sha = node?.Data?.sha;
    if (sha) {
      vscode.env.clipboard.writeText(sha);
      vscode.window.showInformationMessage(`Copied SHA: ${sha.substring(0, 7)}`);
    }
  });

  register("semio.openCommitInGitHub", (item: MonorepoTreeItem) => {
    const node = item?.data as TreeNodeData | undefined;
    const sha = node?.Data?.sha;
    if (sha) vscode.env.openExternal(vscode.Uri.parse(`https://github.com/usalu/semio/commit/${sha}`));
  });

  register("semio.policyCheck", (item: MonorepoTreeItem) => {
    const node = item?.data as TreeNodeData | undefined;
    const policyId = node?.Data?.id || node?.Label;
    if (!policyId) return;
    const binaryPath = getRepoBinaryPath();
    if (!binaryPath) return;
    const cp = require("child_process");
    cp.execSync(`${binaryPath} policy check ${policyId}`, { cwd: getWorkspaceRoot() });
  });

  const contributedCommands: string[] = [
    "semio.analyze", "semio.analyzeFile", "semio.fix", "semio.fixFile",
    "semio.policyList", "semio.policyTree", "semio.ticketList", "semio.ticketRead", "semio.ticketTree",
    "semio.projectList", "semio.projectTree",
    "semio.contributorAdd", "semio.contributorList", "semio.contributorRemove",
    "semio.sectionTree", "semio.sectionList", "semio.sectionCreate", "semio.sectionMove",
    "semio.sectionDelete", "semio.sectionOpen", "semio.sectionRename",
    "semio.sectionCreateChild", "semio.sectionRemove", "semio.sectionIntegrate",
    "semio.definitionList", "semio.definitionTree",
    "semio.folderTree", "semio.folderCreate", "semio.folderMove", "semio.folderDelete", "semio.folderList",
    "semio.fileCreate", "semio.fileMove", "semio.fileDelete", "semio.fileList", "semio.fileTree",
    "semio.refreshDiagnostics", "semio.fixViolation",
    "semio.navigateToRepo", "semio.navigateTo", "semio.goalOpen", "semio.goalList",
  ];

  for (const command of contributedCommands) {
    if (registered.has(command)) continue;
    register(command, (..._args: unknown[]) => undefined);
  }

  loadAvailableFilterValues();
}

async function loadAvailableFilterValues(): Promise<void> {
  const years = new Set<number>();
  const months = new Set<number>();
  const days = new Set<number>();
  const contributors = new Set<string>();
  const policies = new Set<string>();
  const violations = new Set<string>();

  const tree = await getTreeRoot();
  if (tree?.Children) {
    const walk = (nodes: TreeNodeData[]) => {
      for (const n of nodes) {
        if (n.Kind === "ticket") {
          if (n.Year) years.add(n.Year);
          if (n.Month) months.add(n.Month);
          if (n.Day) days.add(n.Day);
        }
        if (n.Kind === "contributor") contributors.add(n.Label || "");
        if (n.Kind === "policy") policies.add(n.Data?.id || n.Label || "");
        if (n.Kind === "violationKind") violations.add(n.Data?.id || n.ID || "");
        if (n.Children) walk(n.Children);
      }
    };
    walk(tree.Children);
  }

  if (filterProvider) {
    filterProvider.availableYears = Array.from(years).sort((a, b) => b - a);
    filterProvider.availableMonths = Array.from(months).sort((a, b) => a - b);
    filterProvider.availableDays = Array.from(days).sort((a, b) => a - b);
    filterProvider.availableContributors = Array.from(contributors).sort();
    filterProvider.availablePolicies = Array.from(policies).sort();
    filterProvider.refresh();
  }
}

export function activate(context: vscode.ExtensionContext) {
  outputChannel = vscode.window.createOutputChannel("semio-repo");
  context.subscriptions.push(outputChannel);
  log("[ACTIVATION] semio-repo extension activating...");

  try {
    registerSidebarViews(context);
    registerCommands(context);

    repoDiagnosticCollection = vscode.languages.createDiagnosticCollection("semio");
    kitDiagnosticCollection = vscode.languages.createDiagnosticCollection("semio-kit");
    context.subscriptions.push(repoDiagnosticCollection, kitDiagnosticCollection);

    context.subscriptions.push(vscode.workspace.onDidSaveTextDocument(() => {
      invalidateTreeNodeCache();
      monorepoProvider?.refresh();
    }));

    context.subscriptions.push(vscode.window.registerUriHandler({
      handleUri(uri: vscode.Uri) {
        const semiorepoUri = `semiorepo://${uri.authority}${uri.path}`;
        vscode.commands.executeCommand("semio.navigate", semiorepoUri);
      }
    }));

    setTimeout(() => {
      vscode.workspace.textDocuments.forEach((document) => {
        if (shouldAnalyzeFile(document)) {
          analyzeFile(document);
        }
        if (isKitDocument(document)) {
          validateKitDocument(document);
        }
      });
    }, 100);

    log("[ACTIVATION] semio-repo extension activated.");
  } catch (e) {
    logError("[ACTIVATION] Failed to activate extension:", e);
  }
}

export function deactivate() { }

// #endregion 🔖Activation
