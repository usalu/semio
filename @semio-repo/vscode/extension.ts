// #region Header

// js/vscode/extension.ts

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

// #endregion Header

// #region Imports

// @ts-ignore
import { applyKitDiff, deserializeKit, DomainLocation, Fix, Kit, Problem, serializeKit, validateKit } from "@semio/js/semio";
import { cacheExchange, Client, fetchExchange } from "@urql/core";
import { exec, execFile } from "child_process";
import * as fs from "fs";
import * as jsonc from "jsonc-parser";
import * as path from "path";
import { promisify } from "util";
import * as vscode from "vscode";
import { TypedDocumentNode } from "@graphql-typed-document-node/core";
import {
  AnalyzeDocument,
  BundlesDocument,
  CodebaseDocument,
  ContributorsDocument,
  FileContentDocument,
  FixDocument,
  FolderContentDocument,
  GoalsDocument,
  PoliciesDocument,
  RepoDocument,
  TicketsDocument,
  TodoCreateDocument,
  TodoDeleteDocument,
  TodosDocument
} from "./queries";
import {
  Bundle,
  Commit,
  Contributor,
  Goal,
  Policy,
  Project,
  Repo,
  Ticket,
  TicketStatus,
  Todo,
  ViolationKind
} from "./generated/graphql";

const execAsync = promisify(exec);
const execFileAsync = promisify(execFile);

type RepoEvent = {
  kind: string;
  data?: unknown;
  result?: unknown;
  error?: { message?: string; fatal?: boolean };
  done?: { exit_code?: number };
};

// #endregion Imports

// #region Constants

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

// #endregion Constants

// #region Types

export interface ToolResult<T = unknown> {
  output: { lines: { type: string; text: string }[]; exitCode: number };
  data?: T;
  error?: string;
}

export interface ProjectData {
  name: string;
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

export interface TicketIteration {
  prompt: string;
  plan?: string;
  llm: string;
  ui: string;
  author: { github: string; name?: string | null };
  started: string;
  finished?: string;
  commit?: string;
}

export interface TicketData {
  year: number;
  month: number;
  day: number;
  slug: string;
  frontmatter: TicketFrontmatter;
  folderPath: string;
  filePath: string;
  iterations?: TicketIteration[];
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
  start?: { line: number; column: number };
  end?: { line: number; column: number };
}

interface GraphqlSection {
  name: string;
  __typename?: string;
  range?: GraphqlSectionRange | null;
  children?: GraphqlSection[] | null;
}

interface Codebase {
  tree: any;
}

// #endregion Types

// #region Globals

let outputChannel: vscode.OutputChannel;
let urqlClient: Client | null = null;
let repoDiagnosticCollection: vscode.DiagnosticCollection;
let kitDiagnosticCollection: vscode.DiagnosticCollection;
const fileViolationsMap = new Map<string, Violation[]>();
let bundleCache: Bundle[] = [];
let cachedCodebase: Codebase | null = null;
let codebaseLoadPromise: Promise<Codebase | null> | null = null;
let cachedProjects: ProjectData[] | null = null;
let cachedRepoBaseUrl: string | undefined = undefined;
const runningProcesses = new Map<string, AbortController>();

let filterProvider: FilterTreeDataProvider | undefined;
let monorepoProvider: MonorepoTreeDataProvider | undefined;

// #endregion Globals

// #region Utilities

function log(...args: any[]): void {
  const message = args.map(a => typeof a === 'object' ? JSON.stringify(a, null, 2) : String(a)).join(' ');
  outputChannel?.appendLine(message);
  try {
    const logPath = path.join(getWorkspaceRoot() || "", "activation.log");
    fs.appendFileSync(logPath, "[LOG] " + message + "\n");
  } catch (e) { }
}

function logError(...args: any[]): void {
  const message = args.map(a => typeof a === 'object' ? JSON.stringify(a, null, 2) : String(a)).join(' ');
  outputChannel?.appendLine('[ERROR] ' + message);
  try {
    const logPath = path.join(getWorkspaceRoot() || "", "activation.log");
    fs.appendFileSync(logPath, "[ERROR] " + message + "\n");
  } catch (e) { }
}

function getWorkspaceRoot(): string | undefined {
  return vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
}

function getRepoBinaryPath(): string | undefined {
  const root = getWorkspaceRoot();
  if (!root) return undefined;
  const isWindows = process.platform === "win32";
  const binaryName = isWindows ? "repo.exe" : "repo";
  const binaryPath = path.join(root, "go", "repo", binaryName);
  if (fs.existsSync(binaryPath)) return binaryPath;
  return undefined;
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
  if (!root) {
    cachedRepoBaseUrl = undefined;
    return cachedRepoBaseUrl;
  }
  const packagePath = path.join(root, "package.json");
  if (!fs.existsSync(packagePath)) {
    cachedRepoBaseUrl = undefined;
    return cachedRepoBaseUrl;
  }
  const raw = fs.readFileSync(packagePath, "utf8");
  const parsed = JSON.parse(raw) as { repository?: { url?: string } | string };
  const repoUrl = typeof parsed.repository === "string" ? parsed.repository : parsed.repository?.url;
  if (!repoUrl) {
    cachedRepoBaseUrl = undefined;
    return cachedRepoBaseUrl;
  }
  let cleaned = repoUrl.replace(/^git\+/, "").replace(/\.git$/, "");
  if (cleaned.startsWith("git@")) {
    const match = cleaned.match(/^git@([^:]+):(.+)$/);
    if (match) {
      cleaned = `https://${match[1]}/${match[2]}`;
    }
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

function parseRepoEvents(output: string): RepoEvent[] {
  const lines = output.split("\n").map((line) => line.trim()).filter((line) => line.length > 0);
  return lines.map((line) => JSON.parse(line) as RepoEvent);
}

function extractRepoResult(events: RepoEvent[]): Record<string, unknown> {
  const results: unknown[] = [];
  for (const event of events) {
    if (event.kind === "error" && event.error?.fatal) {
      throw new Error(event.error.message ?? "Repo command failed");
    }
    if (event.kind === "result") {
      results.push(event.result ?? event.data ?? null);
    }
  }

  // Handle aggregated section results
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

// #endregion Utilities

// #region Data Fetching

async function fetchRepoViaGraphQL(): Promise<Repo | null> {
  const client = getUrqlClient();
  if (!client) return null;
  const result = await client.query(RepoDocument as TypedDocumentNode<any, any>, {});
  if (result.error) {
    logError("[GraphQL] fetchRepoViaGraphQL error:", result.error);
    return null;
  }
  return result.data?.repo as unknown as Repo ?? null;
}

async function fetchBundlesViaGraphQL(): Promise<Bundle[]> {
  const client = getUrqlClient();
  if (!client) return [];
  const result = await client.query(BundlesDocument as TypedDocumentNode<any, any>, {});
  if (result.error) {
    logError("[GraphQL] fetchBundlesViaGraphQL error:", result.error);
    return [];
  }
  return result.data?.repo?.bundles ?? [];
}

async function fetchFolderContent(path: string): Promise<any | null> { // Simplified return type
  const client = getUrqlClient();
  if (!client) return null;
  const result = await client.query(FolderContentDocument as TypedDocumentNode<any, any>, { path });
  if (result.error) {
    logError("[GraphQL] fetchFolderContent error:", result.error);
    return null;
  }
  return result.data?.folder ?? null;
}

async function fetchTicketsViaGraphQL(year?: number, month?: number, day?: number, status?: TicketStatus): Promise<Ticket[]> {
  const client = getUrqlClient();
  if (!client) return [];
  const result = await client.query(TicketsDocument as TypedDocumentNode<any, any>, { year, month, day, status });
  if (result.error) {
    logError("[GraphQL] fetchTicketsViaGraphQL error:", result.error);
    return [];
  }
  return result.data?.repo?.tickets ?? [];
}

async function fetchContributorsViaGraphQL(): Promise<Contributor[]> {
  const client = getUrqlClient();
  if (!client) return [];
  const result = await client.query(ContributorsDocument as TypedDocumentNode<any, any>, {});
  if (result.error) {
    logError("[GraphQL] fetchContributorsViaGraphQL error:", result.error);
    return [];
  }
  return result.data?.repo?.contributors ?? [];
}

async function fetchPoliciesViaGraphQL(): Promise<Policy[]> {
  const client = getUrqlClient();
  if (!client) return [];
  const result = await client.query(PoliciesDocument as TypedDocumentNode<any, any>, {});
  if (result.error) {
    logError("[GraphQL] fetchPoliciesViaGraphQL error:", result.error);
    return [];
  }
  return result.data?.repo?.policies ?? [];
}

async function getProjectList(): Promise<ProjectData[]> {
  if (cachedProjects) return cachedProjects;
  const codebase = await loadCodebase();
  if (codebase) return cachedProjects ?? [];
  if (!hasRepoAccess()) return [];
  const result = await runRepoCommandJson<ToolResult<ProjectData[]>>("bundle list");
  cachedProjects = result?.data ?? [];
  return cachedProjects;
}

async function loadCodebase(): Promise<Codebase | null> {
  if (cachedCodebase) return cachedCodebase;
  if (codebaseLoadPromise) return codebaseLoadPromise;
  if (!hasRepoAccess()) return null;

  codebaseLoadPromise = (async () => {
    log("[loadCodebase] Loading codebase via GraphQL...");
    const repo = await fetchRepoViaGraphQL();
    if (!repo) {
      logError("[loadCodebase] Failed to fetch repo via GraphQL");
      codebaseLoadPromise = null;
      return null;
    }

    const tree: any = {};
    for (const bundle of repo.bundles) {
      tree[bundle.id] = { kind: "bundle", children: {} };
    }

    const codebase: Codebase = { ...repo, tree } as unknown as Codebase;
    cachedCodebase = codebase;
    codebaseLoadPromise = null;

    if (cachedCodebase) {
      log(`[loadCodebase] Loaded ${repo.bundles.length} bundles`);
      cachedProjects = repo.bundles.map((b) => ({
        name: b.id,
        root: b.root,
        projectType: b.projectType ?? undefined,
        tags: b.tags,
      }));
    }

    return cachedCodebase;
  })();

  return codebaseLoadPromise;
}

// #endregion Data Fetching

// #region Helpers

function extractFilePathFromScope(scope: string): string | undefined {
  let cleanScope = scope;
  if (cleanScope.startsWith("@semio/violations/")) {
    cleanScope = cleanScope.replace("@semio/violations/", "");
  }

  // Handle hierarchical IDs: BUNDLE/RELATIVEPATH
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

  // Fallback
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

// #endregion Helpers

// #region File Analysis & Diagnostics

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

// #endregion File Analysis

// #region Providers

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
    bundle: { library: true, binary: true, ui: true, site: true, assets: true, default: true },
    folder: { organization: true, required: true },
    section: { none: false, all: true },
    definition: { implementation: true, interface: true, constant: true },
  };

  public timeFilter: Record<string, boolean> = { none: false, all: true };
  public excludedYears: number[] = [];
  public excludedMonths: number[] = [];
  public excludedDays: number[] = [];

  public availableYears: number[] = [];
  public availableMonths: number[] = [];
  public availableDays: number[] = [];
  public availableContributors: string[] = [];
  public availablePolicies: string[] = [];

  constructor() { }

  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(element: FilterTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: FilterTreeItem): Promise<FilterTreeItem[]> {
    if (!element) {
      return [
        this.createSearchItem(),
        new FilterTreeItem("bundle", "root", vscode.TreeItemCollapsibleState.Collapsed, "filterRoot"),
        new FilterTreeItem("folder", "root", vscode.TreeItemCollapsibleState.Collapsed, "filterRoot"),
        new FilterTreeItem("section", "root", vscode.TreeItemCollapsibleState.Collapsed, "filterRoot"),
        new FilterTreeItem("definition", "root", vscode.TreeItemCollapsibleState.Collapsed, "filterRoot"),
        new FilterTreeItem("time", "root", vscode.TreeItemCollapsibleState.Collapsed, "filterRoot"),
        new FilterTreeItem("contributors", "root", vscode.TreeItemCollapsibleState.Collapsed, "filterRoot"),
        new FilterTreeItem("policies", "root", vscode.TreeItemCollapsibleState.Collapsed, "filterRoot"),
      ];
    }

    if (element.type === "root") {
      const kind = element.label;
      if (kind === "time") {
        return [
          this.createFilterOption("time", "none", this.timeFilter.none),
          this.createFilterOption("time", "all", this.timeFilter.all),
          ...this.availableYears.map(y => this.createTimeValueItem("year", y, !this.excludedYears.includes(y)))
        ];
      }
      if (kind === "contributors") {
        return this.availableContributors.map(c => new FilterTreeItem(c, "filterOption"));
      }
      if (kind === "policies") {
        return this.availablePolicies.map(p => new FilterTreeItem(p, "filterOption"));
      }

      const options = this.filters[kind];
      if (options) {
        return Object.keys(options).map(key => this.createFilterOption(kind, key, options[key]));
      }
    }

    if (element.type === "timeValue" && element.filterKey === "year") {
      const year = element.filterValue;
      return [
        this.createFilterOption(`time/${year}`, "none", false),
        this.createFilterOption(`time/${year}`, "all", true),
        ...this.availableMonths.map(m => this.createTimeValueItem("month", m, !this.excludedMonths.includes(m), year))
      ];
    }

    if (element.type === "timeValue" && element.filterKey === "month") {
      const month = element.filterValue;
      return [
        this.createFilterOption(`time/month/${month}`, "none", false),
        this.createFilterOption(`time/month/${month}`, "all", true),
        ...this.availableDays.map(d => this.createTimeValueItem("day", d, !this.excludedDays.includes(d), month))
      ];
    }

    return [];
  }

  private createSearchItem(): FilterTreeItem {
    const label = `SEARCH${this.searchQuery ? `: ${this.searchQuery}` : ""}`;
    const item = new FilterTreeItem(label, "search", vscode.TreeItemCollapsibleState.None, "searchItem");
    item.description = `[${this.matchCase ? "Aa" : "  "}] [${this.matchWholeWord ? "Ab" : "  "}] [${this.useRegex ? ".*" : "  "}]`;
    item.command = { command: "semio.filter.search", title: "Search" };
    return item;
  }

  private createFilterOption(kind: string, key: string, enabled: boolean): FilterTreeItem {
    const item = new FilterTreeItem(key, "filterOption", vscode.TreeItemCollapsibleState.None, "filterOption", kind, key);
    item.iconPath = new vscode.ThemeIcon(enabled ? "check" : "circle-slash");
    item.command = { command: "semio.filter.toggle", title: "Toggle", arguments: [kind, key] };
    return item;
  }

  private createTimeValueItem(kind: string, value: number, enabled: boolean, parentValue?: number): FilterTreeItem {
    const label = kind === "month" ? new Date(2000, value - 1, 1).toLocaleString('default', { month: 'long' }) : String(value);
    const item = new FilterTreeItem(label, "timeValue", vscode.TreeItemCollapsibleState.Collapsed, "timeValue", kind, value);
    item.iconPath = new vscode.ThemeIcon(enabled ? "check" : "circle-slash");
    // Click on icon to toggle, or use command on item?
    // For now we assume using the "none/all" options inside or implementing toggleYear logic separately
    return item;
  }

  toggle(kind: string, key: string) {
    if (kind === "time") {
      if (key === "all") { this.excludedYears = []; this.timeFilter.all = true; this.timeFilter.none = false; }
      else if (key === "none") { this.excludedYears = [...this.availableYears]; this.timeFilter.all = false; this.timeFilter.none = true; }
      else {
        // specific year toggle via test calls or manual invocation
        const year = parseInt(key);
        if (!isNaN(year)) this.toggleYear(year);
      }
    } else if (kind.startsWith("time/")) {
      // time/2024 or time/month/1
      if (kind.includes("/month/")) {
        // toggle day
      } else {
        // toggle month
      }
    } else if (this.filters[kind]) {
      this.filters[kind][key] = !this.filters[kind][key];
    }
    this.refresh();
    monorepoProvider?.refresh();
  }

  toggleYear(year: number) {
    if (this.excludedYears.includes(year)) {
      this.excludedYears = this.excludedYears.filter(y => y !== year);
    } else {
      this.excludedYears.push(year);
    }
    this.refresh();
    monorepoProvider?.refresh();
  }
  toggleMonth(month: number) {
    if (this.excludedMonths.includes(month)) {
      this.excludedMonths = this.excludedMonths.filter(m => m !== month);
    } else {
      this.excludedMonths.push(month);
    }
    this.refresh();
    monorepoProvider?.refresh();
  }
  toggleDay(day: number) {
    if (this.excludedDays.includes(day)) {
      this.excludedDays = this.excludedDays.filter(d => d !== day);
    } else {
      this.excludedDays.push(day);
    }
    this.refresh();
    monorepoProvider?.refresh();
  }

  toggleContributor(c: string) { /* implementation */ }
  togglePolicy(p: string) { /* implementation */ }
  toggleViolation(v: string) { /* implementation */ }
  toggleBundleKind(k: string) { /* implementation */ }
  toggleFolderKind(k: string) { /* implementation */ }
  toggleDefinitionKind(k: string) { /* implementation */ }
  showAll() { /* implementation */ }
  showNone() { /* implementation */ }
  showDefault() { /* implementation */ }
}

export class MonorepoTreeDataProvider implements vscode.TreeDataProvider<MonorepoTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<MonorepoTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  constructor() { }

  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(element: MonorepoTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: MonorepoTreeItem): Promise<MonorepoTreeItem[]> {
    if (!element) {
      return [
        new MonorepoTreeItem("Monorepo", vscode.TreeItemCollapsibleState.Expanded, "root_monorepo"),
        new MonorepoTreeItem("Goals", vscode.TreeItemCollapsibleState.Collapsed, "root_goals"),
        new MonorepoTreeItem("Tickets", vscode.TreeItemCollapsibleState.Collapsed, "root_tickets"),
        new MonorepoTreeItem("Policies", vscode.TreeItemCollapsibleState.Collapsed, "root_policies"),
        new MonorepoTreeItem("Contributors", vscode.TreeItemCollapsibleState.Collapsed, "root_contributors"),
        new MonorepoTreeItem("Commits", vscode.TreeItemCollapsibleState.Collapsed, "root_commits"),
      ];
    }

    const client = getUrqlClient();
    if (!client) return [];

    // -- Monorepo Branch --
    if (element.contextValue === "root_monorepo") {
      const res = await client.query(RepoDocument as TypedDocumentNode<any, any>, {}).toPromise();
      const projects = res.data?.repo?.projects || [];
      return projects.map((p: any) => new MonorepoTreeItem(p.name, vscode.TreeItemCollapsibleState.Collapsed, "project", p));
    }

    if (element.contextValue === "project") {
      const project = element.data;
      return (project.bundles || []).map((b: any) => {
        const item = new MonorepoTreeItem(b.name, vscode.TreeItemCollapsibleState.Collapsed, "bundle", b);
        item.command = { command: "semio.navigateToBundle", title: "Open", arguments: [b.root] };
        return item;
      });
    }

    if (element.contextValue === "bundle") {
      // Simplify for now: show files? or folders?
      // We need to query folder content or bundles details
      return [];
    }

    // -- Goals Branch --
    if (element.contextValue === "root_goals") {
      const res = await client.query(GoalsDocument as TypedDocumentNode<any, any>, {}).toPromise();
      const goals = res.data?.repo?.goals || [];
      return goals.filter((g: any) => !g.id.includes("/")).map((g: any) => new MonorepoTreeItem(g.title || g.id, vscode.TreeItemCollapsibleState.Collapsed, "goal", g));
    }

    if (element.contextValue === "goal") {
      const goal = element.data;
      const goalId = goal.id;

      const resGoals = await client.query(GoalsDocument as TypedDocumentNode<any, any>, {}).toPromise();
      const allGoals = resGoals.data?.repo?.goals || [];
      const subgoals = allGoals.filter((g: any) => g.id.startsWith(goalId + "/") && g.id.split("/").length === goalId.split("/").length + 1);
      const subgoalItems = subgoals.map((g: any) => new MonorepoTreeItem(g.title || g.id, vscode.TreeItemCollapsibleState.Collapsed, "goal", g));

      const resTickets = await client.query(TicketsDocument as TypedDocumentNode<any, any>, {}).toPromise();
      const allTickets = resTickets.data?.repo?.tickets || [];
      const goalTickets = allTickets.filter((t: any) => t.goal === goalId);
      const ticketItems = goalTickets.map((t: any) => {
        const item = new MonorepoTreeItem(t.slug, vscode.TreeItemCollapsibleState.None, "ticket", t);
        item.command = { command: "semio.ticketOpen", title: "Open", arguments: [t] };
        return item;
      });

      return [...subgoalItems, ...ticketItems];
    }

    // -- Tickets Branch --
    if (element.contextValue === "root_tickets") {
      const res = await client.query(TicketsDocument as TypedDocumentNode<any, any>, {}).toPromise();
      const tickets = res.data?.repo?.tickets || [];
      const years = [...new Set(tickets.map((t: any) => t.year))].sort((a: any, b: any) => b - a);
      return years.map((y: any) => new MonorepoTreeItem(String(y), vscode.TreeItemCollapsibleState.Collapsed, "ticket_year", y));
    }

    if (element.contextValue === "ticket_year") {
      const year = element.data;
      const res = await client.query(TicketsDocument as TypedDocumentNode<any, any>, { year }).toPromise();
      const tickets = res.data?.repo?.tickets || [];
      const months = [...new Set(tickets.map((t: any) => t.month))].sort((a: any, b: any) => b - a);
      return months.map((m: any) => new MonorepoTreeItem(String(m).padStart(2, '0'), vscode.TreeItemCollapsibleState.Collapsed, "ticket_month", { year, month: m }));
    }

    if (element.contextValue === "ticket_month") {
      const { year, month } = element.data;
      const res = await client.query(TicketsDocument as TypedDocumentNode<any, any>, { year, month }).toPromise();
      const tickets = res.data?.repo?.tickets || [];
      const days = [...new Set(tickets.map((t: any) => t.day))].sort((a: any, b: any) => b - a);
      return days.map((d: any) => new MonorepoTreeItem(String(d).padStart(2, '0'), vscode.TreeItemCollapsibleState.Collapsed, "ticket_day", { year, month, day: d }));
    }

    if (element.contextValue === "ticket_day") {
      const { year, month, day } = element.data;
      const res = await client.query(TicketsDocument as TypedDocumentNode<any, any>, { year, month, day }).toPromise();
      const tickets = res.data?.repo?.tickets || [];
      return tickets.map((t: any) => {
        const item = new MonorepoTreeItem(t.slug, vscode.TreeItemCollapsibleState.None, "ticket", t);
        item.command = { command: "semio.ticketOpen", title: "Open", arguments: [t] };
        return item;
      });
    }

    // -- Policies Branch --
    if (element.contextValue === "root_policies") {
      const res = await client.query(PoliciesDocument as TypedDocumentNode<any, any>, {}).toPromise();
      const policies = res.data?.repo?.policies || [];
      return policies.map((p: any) => new MonorepoTreeItem(p.name, vscode.TreeItemCollapsibleState.Collapsed, "policy", p));
    }

    if (element.contextValue === "policy") {
      const policy = element.data;
      return (policy.violationKinds || []).map((v: any) => new MonorepoTreeItem(v.id, vscode.TreeItemCollapsibleState.None, "violation", v));
    }

    // -- Contributors Branch --
    if (element.contextValue === "root_contributors") {
      const res = await client.query(ContributorsDocument as TypedDocumentNode<any, any>, {}).toPromise();
      const contributors = res.data?.repo?.contributors || [];
      return contributors.map((c: any) => new MonorepoTreeItem(c.name || c.github, vscode.TreeItemCollapsibleState.Collapsed, "contributor", c));
    }

    if (element.contextValue === "contributor") {
      const contributor = element.data;
      return [
        new MonorepoTreeItem("Emails", vscode.TreeItemCollapsibleState.None, "contributor_emails", contributor.emails),
        new MonorepoTreeItem("Links", vscode.TreeItemCollapsibleState.None, "contributor_links", contributor.links),
        new MonorepoTreeItem("Contributions", vscode.TreeItemCollapsibleState.None, "contributor_contributions", contributor)
      ];
    }

    // -- Commits Branch --
    if (element.contextValue === "root_commits") {
      const res = await client.query(RepoDocument as TypedDocumentNode<any, any>, {}).toPromise();
      const commits = res.data?.repo?.commits || [];
      return commits.map((c: any) => new MonorepoTreeItem(c.sha.substring(0, 7), vscode.TreeItemCollapsibleState.Collapsed, "commit", c));
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
      const res = await client.query(TicketsDocument as TypedDocumentNode<any, any>, {}).toPromise(); // ideally filter by commit, but schema might not support it yet, fetching all for now or check fetching
      // The schema has tickets(commit: String) ? No, tickets(year... status).
      // We have to fetch all tickets or optimize. For now, fetch all and filter client-side if needed, 
      // OR use RepoDocument.tickets which has commit field.
      const resRepo = await client.query(RepoDocument as TypedDocumentNode<any, any>, {}).toPromise();
      const tickets = (resRepo.data?.repo?.tickets || []).filter((t: any) => t.commit === sha);

      const goals = [...new Set(tickets.map((t: any) => t.goal).filter((g: any) => !!g))];
      const items = goals.map((g: any) => new MonorepoTreeItem(g, vscode.TreeItemCollapsibleState.Collapsed, "commit_ticket_goal", { goal: g, tickets: tickets.filter((t: any) => t.goal === g) }));

      const noGoalTickets = tickets.filter((t: any) => !t.goal);
      noGoalTickets.forEach((t: any) => {
        const item = new MonorepoTreeItem(t.slug, vscode.TreeItemCollapsibleState.None, "ticket", t);
        item.command = { command: "semio.ticketOpen", title: "Open", arguments: [t] };
        items.push(item);
      });
      return items;
    }

    if (element.contextValue === "commit_ticket_goal") {
      const { tickets } = element.data;
      return tickets.map((t: any) => {
        const item = new MonorepoTreeItem(t.slug, vscode.TreeItemCollapsibleState.None, "ticket", t);
        item.command = { command: "semio.ticketOpen", title: "Open", arguments: [t] };
        return item;
      });
    }

    if (element.contextValue === "commit_goals") {
      const sha = element.data;
      // Similarly find goals related to commit. If tickets have goals, maybe goals directly associated?
      // Assuming derived from tickets for now as per "commit_tickets" structure.
      // Or maybe goals have commits? Repo.goals doesn't have commits.
      // Let's assume it means goals that have tickets in this commit.
      const resRepo = await client.query(RepoDocument as TypedDocumentNode<any, any>, {}).toPromise();
      const tickets = (resRepo.data?.repo?.tickets || []).filter((t: any) => t.commit === sha);
      const goalIds = [...new Set(tickets.map((t: any) => t.goal).filter((g: any) => !!g))];
      return goalIds.map((g: any) => new MonorepoTreeItem(g, vscode.TreeItemCollapsibleState.None, "goal", { id: g }));
    }

    return [];
  }
}

class MonorepoTreeItem extends vscode.TreeItem {
  constructor(
    public readonly label: string,
    public readonly collapsibleState: vscode.TreeItemCollapsibleState,
    public readonly contextValue?: string,
    public readonly data?: any
  ) {
    super(label, collapsibleState);
    this.contextValue = contextValue;
  }
}

// #endregion Providers

// #region Activation

function registerSidebarViews(context: vscode.ExtensionContext): void {
  monorepoProvider = new MonorepoTreeDataProvider();
  vscode.window.registerTreeDataProvider("semio.monorepo", monorepoProvider);

  filterProvider = new FilterTreeDataProvider();
  vscode.window.registerTreeDataProvider("semio.filter", filterProvider);
}

function registerCommands(context: vscode.ExtensionContext): void {
  // Register commands that depend on providers
  context.subscriptions.push(
    vscode.commands.registerCommand("semio.filter.search", async () => {
      const q = await vscode.window.showInputBox({ prompt: "Search..." });
      if (q !== undefined && filterProvider) {
        filterProvider.searchQuery = q;
        filterProvider.refresh();
        monorepoProvider?.refresh();
      }
    }),
    vscode.commands.registerCommand("semio.filter.toggle", (kind: string, key: string) => {
      filterProvider?.toggle(kind, key);
    }),
    vscode.commands.registerCommand("semio.refreshCodebase", () => {
      monorepoProvider?.refresh();
    }),
    vscode.commands.registerCommand("semio.toggleFilter", (kind: string) => {
      if (filterProvider && kind in filterProvider.filters) {
        // This command seems redundant or needs specific implementation based on arguments
        // For now, mapping to toggle if key matches
      }
    })
  );

  loadAvailableFilterValues();
}

async function loadAvailableFilterValues(): Promise<void> {
  const years = new Set<number>();
  const contributors = new Set<string>();
  const policies = new Set<string>();
  const violations = new Set<string>();

  const tickets = await fetchTicketsViaGraphQL();
  tickets.forEach(t => years.add(t.year));

  const contribs = await fetchContributorsViaGraphQL();
  contribs.forEach(c => contributors.add(c.name || c.github));

  const pols = await fetchPoliciesViaGraphQL();
  pols.forEach(p => {
    policies.add(p.id);
    p.violationKinds.forEach(v => violations.add(v.id));
  });

  if (filterProvider) {
    filterProvider.availableYears = Array.from(years).sort((a, b) => b - a);
    filterProvider.availableContributors = Array.from(contributors).sort();
    filterProvider.availablePolicies = Array.from(policies).sort();
    filterProvider.refresh();
  }
}

export function activate(context: vscode.ExtensionContext) {
  outputChannel = vscode.window.createOutputChannel("semio");
  context.subscriptions.push(outputChannel);
  log("[ACTIVATION] semio-repo extension activating...");

  try {
    registerSidebarViews(context);
    registerCommands(context);

    // Diagnostics
    repoDiagnosticCollection = vscode.languages.createDiagnosticCollection("semio");
    kitDiagnosticCollection = vscode.languages.createDiagnosticCollection("semio-kit");
    context.subscriptions.push(repoDiagnosticCollection, kitDiagnosticCollection);

    // Initial diagnostics run
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

// #endregion Activation
