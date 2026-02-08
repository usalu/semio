// #region 🔖Header

// 💻︎ semio-repo/vscode/extension.ts

// 2025 Ueli Saluz <ueli@semio-tech.com>

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

// #endregion 🔖Header

// #region 🔖Imports

// @ts-ignore
import { TypedDocumentNode } from "@graphql-typed-document-node/core";
import { deserializeKit, Problem, validateKit } from "@semio/js/semio";
import { cacheExchange, Client, fetchExchange } from "@urql/core";
import { exec, execFile } from "child_process";
import * as fs from "fs";
import * as path from "path";
import { promisify } from "util";
import * as vscode from "vscode";
import {
  Bundle,
  Contributor,
  Policy,
  Repo,
  Ticket,
  TicketStatus,
  ViolationKind
} from "./generated/graphql";
import {
  BundlesDocument,
  ContributorsDocument,
  FileContentDocument,
  FolderContentDocument,
  GoalsDocument,
  PoliciesDocument,
  RepoCommitsDocument,
  RepoStructureDocument,
  TicketsDocument
} from "./queries";

const execAsync = promisify(exec);
const execFileAsync = promisify(execFile);

type RepoEvent = {
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
  kind: ViolationKind;
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
let urqlClient: Client | null = null;
let repoDiagnosticCollection: vscode.DiagnosticCollection;
let kitDiagnosticCollection: vscode.DiagnosticCollection;
const fileViolationsMap = new Map<string, Violation[]>();
let bundleCache: Bundle[] = [];
let cachedProjects: ProjectData[] | undefined = undefined;
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

function getUrqlClient(): Client | null {
  if (urqlClient) return urqlClient;
  const root = getWorkspaceRoot();
  const command = getRepoCommand();
  if (!root || !command) {
    return null;
  }

  urqlClient = new Client({
    url: "local://graphql",
    exchanges: [cacheExchange, fetchExchange],
    fetch: async (_input: RequestInfo | URL, init?: RequestInit) => {
      try {
        const body = init?.body ? JSON.parse(init.body as string) : {};
        const query = (body.query as string) || "";
        const variables = body.variables || {};
        const variablesJson = JSON.stringify(variables);
        const repoPath = getRepoCommand();
        if (!repoPath) throw new Error("Repo command not found");
        const repoArgs = ["--json", "graphql", query];
        if (Object.keys(variables).length > 0) {
          repoArgs.push("-v", variablesJson);
        }
        const { stdout, stderr } = await execFileAsync(repoPath, repoArgs, {
          cwd: root,
          timeout: 45000,
          maxBuffer: 500 * 1024 * 1024,
        });
        if (!stdout.trim()) {
          throw new Error("Empty output from repo command");
        }
        const events = parseRepoEvents(stdout);
        const payload = extractRepoResult(events);
        const responseBody = JSON.stringify(payload);
        if (typeof Response !== "undefined") {
          return new Response(responseBody, {
            status: 200,
            headers: { "Content-Type": "application/json" },
          });
        }
        return {
          status: 200,
          ok: true,
          headers: {
            get: (name: string) => (name.toLowerCase() === "content-type" ? "application/json" : null),
            has: (name: string) => name.toLowerCase() === "content-type",
            forEach: (cb: any) => cb("application/json", "content-type"),
          },
          json: async () => payload,
          text: async () => responseBody,
        } as any;
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        const errorBody = JSON.stringify({ errors: [{ message: errorMessage }] });

        if (typeof Response !== "undefined") {
          return new Response(errorBody, {
            status: 500,
            headers: { "Content-Type": "application/json" },
          });
        }

        return {
          status: 500,
          ok: false,
          headers: {
            get: (name: string) => (name.toLowerCase() === "content-type" ? "application/json" : null),
            has: (name: string) => name.toLowerCase() === "content-type",
            forEach: (cb: any) => cb("application/json", "content-type"),
          },
          json: async () => ({ errors: [{ message: errorMessage }] }),
          text: async () => errorBody,
        } as any;
      }
    },
  });
  return urqlClient;
}

function resetUrqlClient(): void {
  urqlClient = null;
}

function isSectionNode(value: any): boolean {
  if (!value || typeof value !== "object") return false;
  if (value.__typename === "Section") return true;
  if (typeof value.id === "string" && value.id.startsWith("section:")) return true;
  return false;
}

// #endregion 🔖Utilities

// #region 🔖URI Resolution

interface TreeNodeData {
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
  Children?: TreeNodeData[];
}

let treeNodeCache: Map<string, TreeNodeData> | null = null;
let treeNodeCacheTime = 0;
const TREE_CACHE_TTL = 30000;

export function bundleKindEmoji(kind: string): string {
  switch (kind) {
    case "schema": return "🛂";
    case "binary": return "⌨️";
    case "ui": return "🖱️";
    case "site": return "🌐";
    case "assets": return "🏪";
    case "library": return "📚";
    default: return "📚";
  }
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
      treeNodeCacheTime = now;
      return cache;
    }
  } catch (error) {
    logError("[getTreeNodeCache] error:", error);
  }
  return treeNodeCache ?? new Map();
}

export function invalidateTreeNodeCache(): void {
  treeNodeCache = null;
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
        case "project":
        case "bundle": {
          const repo = await fetchRepoStructureViaGraphQL();
          if (repo) {
            if (node.Kind === "project") {
              const proj = repo.projects?.find((p: any) => p.uri === uri || slugify(p.name) === slugify(node.Label));
              if (proj) {
                const abs = path.join(wsRoot, proj.root);
                if (fs.existsSync(abs)) {
                  return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(abs)) as any;
                }
              }
            } else {
              const bundle = repo.bundles?.find((b: any) => b.uri === uri || slugify(b.name) === slugify(node.Label));
              if (bundle) {
                const abs = path.join(wsRoot, bundle.root);
                if (fs.existsSync(abs)) {
                  return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(abs)) as any;
                }
              }
            }
          }
          break;
        }
        case "folder": {
          const folderPath = node.ID.replace(/^folder:/, "");
          const abs = path.join(wsRoot, folderPath);
          if (fs.existsSync(abs)) {
            return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(abs)) as any;
          }
          break;
        }
        case "file": {
          const filePath = node.ID.replace(/^file:/, "");
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
                  if (event.kind === "result" && (event as any).data?.section) {
                    const section = (event as any).data.section;
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
                  if (event.kind === "result" && (event as any).data?.definition) {
                    const def = (event as any).data.definition;
                    if (def.name === node.Label && def.startLine) {
                      return openFileAtLine(filePath, def.startLine, def.endLine);
                    }
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

// #region 🔖Data Fetching

async function queryGraphQL<T>(doc: TypedDocumentNode<any, any>, vars: Record<string, unknown>, extract: (data: any) => T, fallback: T): Promise<T> {
  const client = getUrqlClient();
  if (!client) return fallback;
  const result = await client.query(doc, vars).toPromise();
  if (result.error) {
    logError("[GraphQL] error:", result.error);
    return fallback;
  }
  return extract(result.data) ?? fallback;
}

async function fetchRepoStructureViaGraphQL(): Promise<Repo | null> {
  return queryGraphQL(RepoStructureDocument as TypedDocumentNode<any, any>, {}, d => d?.repo as unknown as Repo, null);
}

async function fetchBundlesViaGraphQL(): Promise<Bundle[]> {
  return queryGraphQL(BundlesDocument as TypedDocumentNode<any, any>, {}, d => d?.repo?.bundles, []);
}

async function fetchFolderContent(folderPath: string): Promise<any | null> {
  return queryGraphQL(FolderContentDocument as TypedDocumentNode<any, any>, { path: folderPath }, d => d?.folder, null);
}

async function fetchTicketsViaGraphQL(year?: number, month?: number, day?: number, status?: TicketStatus): Promise<Ticket[]> {
  return queryGraphQL(TicketsDocument as TypedDocumentNode<any, any>, { year, month, day, status }, d => d?.repo?.tickets, []);
}

async function fetchContributorsViaGraphQL(): Promise<Contributor[]> {
  return queryGraphQL(ContributorsDocument as TypedDocumentNode<any, any>, {}, d => d?.repo?.contributors, []);
}

async function fetchPoliciesViaGraphQL(): Promise<Policy[]> {
  return queryGraphQL(PoliciesDocument as TypedDocumentNode<any, any>, {}, d => d?.repo?.policies, []);
}

async function fetchGoalsViaGraphQL(): Promise<any[]> {
  return queryGraphQL(GoalsDocument as TypedDocumentNode<any, any>, {}, d => d?.repo?.goals, []);
}

async function getProjectList(): Promise<ProjectData[]> {
  if (cachedProjects) return cachedProjects;
  if (!hasRepoAccess()) return [];
  const repo = await fetchRepoStructureViaGraphQL();
  if (repo && repo.bundles) {
    cachedProjects = repo.bundles.map((b) => ({
      name: b.id,
      kind: (b as any).kind ?? undefined,
      root: b.root,
      projectType: b.projectType ?? undefined,
      tags: b.tags,
    }));
    bundleCache = repo.bundles;
    return cachedProjects;
  }
  return [];
}

async function fetchCommitsViaGraphQL(): Promise<any[]> {
  return queryGraphQL(RepoCommitsDocument as TypedDocumentNode<any, any>, {}, d => d?.repo?.commits, []);
}

// #endregion 🔖Data Fetching

// #region 🔖Helpers

function extractFilePathFromScope(scope: string): string | undefined {
  let cleanScope = scope;
  if (cleanScope.startsWith("@semio/violations/")) {
    cleanScope = cleanScope.replace("@semio/violations/", "");
  }

  let bestBundle: Bundle | undefined;
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

function resolveTicketData(
  ticket?: TicketData | ContributorTicketData | { ticket: TicketData | ContributorTicketData } | undefined,
): TicketData | ContributorTicketData | undefined {
  if (!ticket) return undefined;
  if ("ticket" in ticket) return ticket.ticket;
  return ticket;
}

function resolveTicketPath(ticket: TicketData | ContributorTicketData): string | undefined {
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

export function getFileKindIcon(name: string): string {
  if (name.endsWith(".ts") || name.endsWith(".tsx") || name.endsWith(".js") || name.endsWith(".jsx")) return "📄";
  if (name.endsWith(".py")) return "🐍";
  if (name.endsWith(".go")) return "🔷";
  if (name.endsWith(".cs")) return "🟣";
  if (name.endsWith(".json") || name.endsWith(".yaml") || name.endsWith(".toml")) return "⚙️";
  if (name.endsWith(".md") || name.endsWith(".txt")) return "📝";
  if (name.endsWith(".sh") || name.endsWith(".ps1")) return "🖥️";
  return "📄";
}

// #endregion 🔖Helpers

// #region 🔖File Analysis & Diagnostics

async function updateBundleCache() {
  const bundles = await fetchBundlesViaGraphQL();
  if (bundles.length > 0) {
    bundleCache = bundles;
  }
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
        this.createFilterItem("🏷️Definitions", "filter_definition", "Definitions filter"),
        this.createFilterItem("🎯Goals", "filter_goal", "Goals filter"),
        this.createFilterItem("📅Tickets", "filter_ticket", "Tickets filter"),
        this.createFilterItem("📅Dates", "filter_time", "Dates filter", vscode.TreeItemCollapsibleState.Collapsed),
        this.createFilterItem("🛡️Policies", "filter_policy", "Policies filter"),
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

export class MonorepoTreeDataProvider implements vscode.TreeDataProvider<MonorepoTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<MonorepoTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  constructor(public filterProvider?: FilterTreeDataProvider) { }

  matchesSearch(text: string): boolean {
    const fp = this.filterProvider;
    if (!fp) return true;
    const query = fp.searchQuery || "";
    if (!query.trim()) return true;

    const target = fp.matchCase ? text : text.toLowerCase();
    const raw = fp.matchCase ? query : query.toLowerCase();

    if (fp.useRegex) {
      try {
        return new RegExp(query, fp.matchCase ? "" : "i").test(text);
      } catch {
        return true;
      }
    }

    if (fp.matchWholeWord) {
      const escaped = raw.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
      return new RegExp(`\\b${escaped}\\b`, fp.matchCase ? "" : "i").test(text);
    }

    return target.includes(raw);
  }

  passesTicketFilter(t: any): boolean {
    if (!this.filterProvider) return true;
    if (t.status === "OPEN" && !this.filterProvider.filters.ticket.open) return false;
    if (t.status === "CLOSED" && !this.filterProvider.filters.ticket.closed) return false;
    if (this.filterProvider.excludedYears.includes(t.year)) return false;
    if (this.filterProvider.excludedMonths.includes(t.month)) return false;
    if (this.filterProvider.excludedDays.includes(t.day)) return false;
    return true;
  }

  buildTicketItem(t: any): MonorepoTreeItem {
    const statusIcon = t.status === "OPEN" ? "🔵" : "🟢";
    const ticketId = `${t.year}/${String(t.month).padStart(2, "0")}/${String(t.day).padStart(2, "0")}/${t.slug}`;
    const item = new MonorepoTreeItem(`📅${statusIcon}${t.slug}`, vscode.TreeItemCollapsibleState.None, "ticket", t, ticketId);
    item.command = { command: "semio.ticketOpen", title: "Open", arguments: [t] };
    return item;
  }

  private buildFolderItems(content: any): MonorepoTreeItem[] {
    const items: MonorepoTreeItem[] = [];
    for (const c of content.children || []) {
      if (!this.matchesSearch(c.name)) continue;
      const item = new MonorepoTreeItem(`📂${c.name}`, vscode.TreeItemCollapsibleState.Collapsed, "folder", c, c.path);
      item.command = { command: "semio.navigateToFolder", title: "Open", arguments: [c.path] };
      items.push(item);
    }
    for (const f of content.files || []) {
      if (!this.matchesSearch(f.name)) continue;
      const kindIcon = getFileKindIcon(f.name);
      const item = new MonorepoTreeItem(`${kindIcon}${f.name}`, vscode.TreeItemCollapsibleState.Collapsed, "file", f, f.path);
      item.command = { command: "semio.navigateToFile", title: "Open", arguments: [f.path] };
      items.push(item);
    }
    return items;
  }

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
      return [
        new MonorepoTreeItem("🏗️Projects", vscode.TreeItemCollapsibleState.Collapsed, "root_projects", undefined, "🏗️Projects"),
        new MonorepoTreeItem("🎯Goals", vscode.TreeItemCollapsibleState.Collapsed, "root_goals", undefined, "🎯Goals"),
        new MonorepoTreeItem("📅Tickets", vscode.TreeItemCollapsibleState.Collapsed, "root_tickets", undefined, "📅Tickets"),
        new MonorepoTreeItem("🛡️Policies", vscode.TreeItemCollapsibleState.Collapsed, "root_policies", undefined, "🛡️Policies"),
        new MonorepoTreeItem("👤Contributors", vscode.TreeItemCollapsibleState.Collapsed, "root_contributors", undefined, "👤Contributors"),
        new MonorepoTreeItem("🔀Commits", vscode.TreeItemCollapsibleState.Collapsed, "root_commits", undefined, "🔀Commits"),
      ];
    }

    const client = getUrqlClient();
    if (!client) return [];

    if (element.contextValue === "root_projects") {
      const repo = await fetchRepoStructureViaGraphQL();
      const projects = repo?.projects || [];
      return projects
        .filter((p: any) => this.matchesSearch(p.name))
        .map((p: any) => {
          const kindIcon = p.kind || "🏗️";
          const item = new MonorepoTreeItem(`${kindIcon}${p.name}`, vscode.TreeItemCollapsibleState.Collapsed, "project", p, `${kindIcon}${p.id}`);
          item.command = { command: "semio.navigateToFolder", title: "Open", arguments: [p.root] };
          return item;
        });
    }

    if (element.contextValue === "project") {
      const project = element.data;
      return (project.bundles || [])
        .filter((b: any) => this.matchesSearch(b.name))
        .map((b: any) => {
          const kindIcon = bundleKindEmoji(b.kind);
          const shortName = b.name.includes("/") ? b.name.split("/").pop()! : b.name;
          const item = new MonorepoTreeItem(`${kindIcon}${shortName}`, vscode.TreeItemCollapsibleState.Collapsed, "bundle", b, `${kindIcon}${b.id}`);
          item.command = { command: "semio.navigateToBundle", title: "Open", arguments: [b.root] };
          return item;
        });
    }

    if (element.contextValue === "bundle" || element.contextValue === "folder") {
      const folderPath = element.contextValue === "bundle" ? element.data.root : element.data.path;
      const content = await fetchFolderContent(folderPath);
      return content ? this.buildFolderItems(content) : [];
    }

    if (element.contextValue === "file") {
      const file = element.data;
      const res = await client.query(FileContentDocument as TypedDocumentNode<any, any>, { path: file.path }).toPromise();
      const fileData = res.data?.file;
      if (!fileData) return [];
      const items: MonorepoTreeItem[] = [];
      if (this.filterProvider?.filters.section.all) {
        const sections = fileData.sections || [];
        for (const s of sections) {
          if (!this.matchesSearch(s.name)) continue;
          const hasChildren = (s.children && s.children.length > 0);
          const hasDefs = (fileData.definitions || []).some((d: any) => d.section?.id === s.id);
          const collapsible = (hasChildren || hasDefs) ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None;
          const payload = { filePath: file.path, section: s, definitions: fileData.definitions || [] };
          const item = new MonorepoTreeItem(`🔖${s.name}`, collapsible, "section", payload, `${file.path}#${s.name}`);
          item.command = { command: "semio.navigateToSection", title: "Open", arguments: [payload] };
          items.push(item);
        }
      }
      return items;
    }

    if (element.contextValue === "section") {
      const payload = element.data as { filePath: string; section: any; definitions: any[] };
      const section = payload.section;
      const items: MonorepoTreeItem[] = [];
      const children = (section.children || []).filter((child: any) => isSectionNode(child));
      for (const child of children) {
        if (!this.matchesSearch(child.name)) continue;
        const hasGrandChildren = (child.children || []).some((grandChild: any) => isSectionNode(grandChild));
        const hasDefs = (payload.definitions || []).some((d: any) => d.section?.id === child.id);
        const collapsible = (hasGrandChildren || hasDefs) ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None;
        const childPayload = { filePath: payload.filePath, section: child, definitions: payload.definitions };
        const item = new MonorepoTreeItem(`🔖${child.name}`, collapsible, "section", childPayload, `${payload.filePath}#${child.name}`);
        item.command = { command: "semio.navigateToSection", title: "Open", arguments: [childPayload] };
        items.push(item);
      }
      const defs = (payload.definitions || []).filter((d: any) => d.section?.id === section.id);
      const filteredDefs = defs.filter((d: any) => {
        if (!this.matchesSearch(d.name)) return false;
        if (!this.filterProvider) return true;
        if (d.kind === "IMPLEMENTATION" && !this.filterProvider.filters.definition.implementation) return false;
        if (d.kind === "INTERFACE" && !this.filterProvider.filters.definition.interface) return false;
        if (d.kind === "CONSTANT" && !this.filterProvider.filters.definition.constant) return false;
        return true;
      });
      for (const d of filteredDefs) {
        const kindIcon = d.kind === "IMPLEMENTATION" ? "🛠️" : d.kind === "INTERFACE" ? "✂️" : "🪨";
        const data = { filePath: payload.filePath, definition: d };
        const item = new MonorepoTreeItem(`${kindIcon}${d.name}`, vscode.TreeItemCollapsibleState.None, "definition", data, `${payload.filePath}§${d.name}`);
        item.command = { command: "semio.navigateToDefinition", title: "Open", arguments: [data] };
        items.push(item);
      }
      return items;
    }

    if (element.contextValue === "root_goals") {
      const goals = await fetchGoalsViaGraphQL();
      return goals
        .filter((g: any) => !g.id.includes("/"))
        .filter((g: any) => this.matchesSearch(g.title || g.id))
        .map((g: any) => {
          const item = new MonorepoTreeItem(`🎯${g.title || g.id}`, vscode.TreeItemCollapsibleState.Collapsed, "goal", g, g.id);
          item.command = { command: "semio.navigate", title: "Open", arguments: [`semiorepo://goal/${g.id}`] };
          return item;
        });
    }

    if (element.contextValue === "goal") {
      const goal = element.data;
      const goalId = goal.id;
      const allGoals = await fetchGoalsViaGraphQL();
      const subgoals = allGoals
        .filter((g: any) => g.id.startsWith(goalId + "/") && g.id.split("/").length === goalId.split("/").length + 1)
        .filter((g: any) => this.matchesSearch(g.title || g.id));
      const subgoalItems = subgoals.map((g: any) => {
        const item = new MonorepoTreeItem(`🎯${g.title || g.id}`, vscode.TreeItemCollapsibleState.Collapsed, "goal", g, g.id);
        item.command = { command: "semio.navigate", title: "Open", arguments: [`semiorepo://goal/${g.id}`] };
        return item;
      });
      const goalTickets = await fetchTicketsViaGraphQL();
      const ticketItems = goalTickets
        .filter((t: any) => t.goal === goalId)
        .filter((t: any) => this.passesTicketFilter(t))
        .filter((t: any) => this.matchesSearch(t.slug))
        .map((t: any) => this.buildTicketItem(t));
      return [...subgoalItems, ...ticketItems];
    }

    if (element.contextValue === "root_tickets") {
      const tickets = await fetchTicketsViaGraphQL();
      const filteredTickets = tickets.filter((t: any) => this.passesTicketFilter(t));
      const years = [...new Set(filteredTickets.map((t: any) => t.year))].sort((a: any, b: any) => b - a);
      return years
        .filter((y: any) => this.matchesSearch(String(y)))
        .map((y: any) => new MonorepoTreeItem(String(y), vscode.TreeItemCollapsibleState.Collapsed, "ticket_year", y, String(y)));
    }

    if (element.contextValue === "ticket_year") {
      const year = element.data;
      const tickets = await fetchTicketsViaGraphQL(year);
      const filteredTickets = tickets.filter((t: any) => this.passesTicketFilter(t));
      const months = [...new Set(filteredTickets.map((t: any) => t.month))].sort((a: any, b: any) => b - a);
      return months
        .filter((m: any) => this.matchesSearch(String(m).padStart(2, "0")))
        .map((m: any) => new MonorepoTreeItem(String(m).padStart(2, "0"), vscode.TreeItemCollapsibleState.Collapsed, "ticket_month", { year, month: m }, `${year}/${String(m).padStart(2, "0")}`));
    }

    if (element.contextValue === "ticket_month") {
      const { year, month } = element.data;
      const tickets = await fetchTicketsViaGraphQL(year, month);
      const filteredTickets = tickets.filter((t: any) => this.passesTicketFilter(t));
      const days = [...new Set(filteredTickets.map((t: any) => t.day))].sort((a: any, b: any) => b - a);
      return days
        .filter((d: any) => this.matchesSearch(String(d).padStart(2, "0")))
        .map((d: any) => new MonorepoTreeItem(String(d).padStart(2, "0"), vscode.TreeItemCollapsibleState.Collapsed, "ticket_day", { year, month, day: d }, `${year}/${String(month).padStart(2, "0")}/${String(d).padStart(2, "0")}`));
    }

    if (element.contextValue === "ticket_day") {
      const { year, month, day } = element.data;
      const tickets = await fetchTicketsViaGraphQL(year, month, day);
      return tickets
        .filter((t: any) => this.passesTicketFilter(t))
        .filter((t: any) => this.matchesSearch(t.slug))
        .map((t: any) => this.buildTicketItem(t));
    }

    if (element.contextValue === "root_policies") {
      const policies = await fetchPoliciesViaGraphQL();
      return policies
        .filter((p: any) => this.matchesSearch(p.name))
        .map((p: any) => new MonorepoTreeItem(`🛡️${p.name}`, vscode.TreeItemCollapsibleState.Collapsed, "policy", p, p.id));
    }

    if (element.contextValue === "policy") {
      const policy = element.data;
      return (policy.violationKinds || [])
        .filter((v: any) => this.matchesSearch(v.id))
        .map((v: any) => {
          const priorityIcon = v.priority === "HIGH" ? "🔴" : v.priority === "MEDIUM" ? "🟡" : "🟢";
          const item = new MonorepoTreeItem(`${priorityIcon}${v.id}`, vscode.TreeItemCollapsibleState.None, "violation", v, v.id);
          item.description = v.autofixable ? "🔧" : "";
          item.tooltip = `${v.reason}\n${v.solution}`;
          return item;
        });
    }

    if (element.contextValue === "root_contributors") {
      const contributors = await fetchContributorsViaGraphQL();
      return contributors
        .filter((c: any) => this.matchesSearch(c.name || c.github))
        .map((c: any) => new MonorepoTreeItem(`👤${c.name || c.github}`, vscode.TreeItemCollapsibleState.Collapsed, "contributor", c, c.id || c.github));
    }

    if (element.contextValue === "contributor") {
      const contributor = element.data;
      const items: MonorepoTreeItem[] = [];
      if (contributor.emails && contributor.emails.length > 0) {
        items.push(new MonorepoTreeItem("Emails", vscode.TreeItemCollapsibleState.Collapsed, "contributor_emails_group", contributor.emails));
      }
      if (contributor.links && contributor.links.length > 0) {
        items.push(new MonorepoTreeItem("Links", vscode.TreeItemCollapsibleState.Collapsed, "contributor_links_group", contributor.links));
      }
      items.push(new MonorepoTreeItem("Contributions", vscode.TreeItemCollapsibleState.None, "contributor_contributions", contributor));
      return items;
    }

    if (element.contextValue === "contributor_emails_group") {
      const emails = element.data as string[];
      return emails.map((email: string) => {
        const item = new MonorepoTreeItem(email, vscode.TreeItemCollapsibleState.None, "contributor_email", email, email);
        item.command = { command: "semio.mailto", title: "Send Email", arguments: [email] };
        return item;
      });
    }

    if (element.contextValue === "contributor_links_group") {
      const links = element.data as { name: string; url: string }[];
      return links.map((link: { name: string; url: string }) => {
        const item = new MonorepoTreeItem(link.name, vscode.TreeItemCollapsibleState.None, "contributor_link", link, link.url);
        item.command = { command: "semio.openLink", title: "Open Link", arguments: [link.url] };
        item.description = link.url;
        return item;
      });
    }

    if (element.contextValue === "root_commits") {
      const commits = await fetchCommitsViaGraphQL();
      return commits
        .filter((c: any) => this.matchesSearch(c.title || c.sha))
        .map((c: any) => {
          const title = c.title || c.sha.substring(0, 7);
          const item = new MonorepoTreeItem(title, vscode.TreeItemCollapsibleState.Collapsed, "commit", c, c.sha);
          item.description = c.sha.substring(0, 7);
          return item;
        });
    }

    if (element.contextValue === "commit") {
      const commit = element.data;
      return [
        new MonorepoTreeItem("Tickets", vscode.TreeItemCollapsibleState.Collapsed, "commit_tickets", commit.sha),
        new MonorepoTreeItem("Goals", vscode.TreeItemCollapsibleState.Collapsed, "commit_goals", commit.sha)
      ];
    }

    if (element.contextValue === "commit_tickets") {
      const sha = element.data;
      const tickets = await fetchTicketsViaGraphQL();
      const commitTickets = tickets
        .filter((t: any) => t.commit === sha)
        .filter((t: any) => this.passesTicketFilter(t));
      const goals = [...new Set(commitTickets.map((t: any) => t.goal).filter((g: any) => !!g))];
      const items: MonorepoTreeItem[] = goals
        .filter((g: any) => this.matchesSearch(g))
        .map((g: any) => new MonorepoTreeItem(`🎯${g}`, vscode.TreeItemCollapsibleState.Collapsed, "commit_ticket_goal", { goal: g, tickets: commitTickets.filter((t: any) => t.goal === g) }, g));
      const noGoalTickets = commitTickets.filter((t: any) => !t.goal);
      for (const t of noGoalTickets) {
        if (!this.matchesSearch(t.slug)) continue;
        items.push(this.buildTicketItem(t));
      }
      return items;
    }

    if (element.contextValue === "commit_ticket_goal") {
      const { tickets } = element.data;
      return tickets
        .filter((t: any) => this.matchesSearch(t.slug))
        .map((t: any) => this.buildTicketItem(t));
    }

    if (element.contextValue === "commit_goals") {
      const sha = element.data;
      const allTickets = await fetchTicketsViaGraphQL();
      const tickets = allTickets.filter((t: any) => t.commit === sha);
      const goalIds = [...new Set(tickets.map((t: any) => t.goal).filter((g: any) => !!g))];
      return goalIds
        .filter((g: any) => this.matchesSearch(g))
        .map((g: any) => {
          const item = new MonorepoTreeItem(`🎯${g}`, vscode.TreeItemCollapsibleState.None, "goal", { id: g }, g);
          item.command = { command: "semio.navigate", title: "Open", arguments: [`semiorepo://goal/${g}`] };
          return item;
        });
    }

    return [];
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
        // Use JSON format which returns Events
        // The output contains multiple JSON objects (one per line)
        const output = await execShell(`"${binaryPath}" section list --file "${filePath}" --json`, root);

        const sections: SectionInfo[] = [];
        const lines = output.split("\n");
        for (const line of lines) {
          if (!line.trim()) continue;
          try {
            const event = JSON.parse(line);
            if (event.kind === "result" && event.data && event.data.section) {
              sections.push(event.data.section);
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

  register("semio.ticketOpen", (ticket: any) => {
    const t = resolveTicketData(ticket);
    if (!t) return;
    const p = resolveTicketPath(t);
    if (!p) return;
    return vscode.commands.executeCommand("semio.navigateToFile", p);
  });

  register("semio.ticketClose", (item: MonorepoTreeItem) => {
    const t = item?.data;
    if (!t) return;
    const ticketId = `${t.year}/${String(t.month).padStart(2, "0")}/${String(t.day).padStart(2, "0")}/${t.slug}`;
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
    const t = item?.data;
    if (!t) return;
    const ticketId = `${t.year}/${String(t.month).padStart(2, "0")}/${String(t.day).padStart(2, "0")}/${t.slug}`;
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
    const sha = item?.data?.sha;
    if (sha) {
      vscode.env.clipboard.writeText(sha);
      vscode.window.showInformationMessage(`Copied SHA: ${sha.substring(0, 7)}`);
    }
  });

  register("semio.openCommitInGitHub", (item: MonorepoTreeItem) => {
    const sha = item?.data?.sha;
    if (sha) vscode.env.openExternal(vscode.Uri.parse(`https://github.com/usalu/semio/commit/${sha}`));
  });

  register("semio.policyCheck", (item: MonorepoTreeItem) => {
    const policy = item?.data;
    if (!policy) return;
    const binaryPath = getRepoBinaryPath();
    if (!binaryPath) return;
    const cp = require("child_process");
    cp.execSync(`${binaryPath} policy check ${policy.id}`, { cwd: getWorkspaceRoot() });
  });

  const contributedCommands: string[] = [
    "semio.analyze", "semio.analyzeFile", "semio.fix", "semio.fixFile",
    "semio.policyList", "semio.ticketList", "semio.ticketRead", "semio.ticketTree",
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

  const tickets = await fetchTicketsViaGraphQL();
  tickets.forEach(t => {
    years.add(t.year);
    months.add(t.month);
    days.add(t.day);
  });

  const contribs = await fetchContributorsViaGraphQL();
  contribs.forEach(c => contributors.add(c.name || c.github));

  const pols = await fetchPoliciesViaGraphQL();
  pols.forEach(p => {
    policies.add(p.id);
    p.violationKinds.forEach(v => violations.add(v.id));
  });

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
