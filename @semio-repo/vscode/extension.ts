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

import { applyKitDiff, deserializeKit, DomainLocation, Fix, Kit, Problem, serializeKit, validateKit } from "@semio/js/semio";
import { cacheExchange, Client, fetchExchange } from "@urql/core";
import { exec, execFile } from "child_process";
import * as fs from "fs";
import * as jsonc from "jsonc-parser";
import * as path from "path";
import { promisify } from "util";
import * as vscode from "vscode";
import { DocumentType, graphql } from "./generated/gql";
import { TicketStatus, Todo } from "./generated/graphql";

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

// #region urql Client

let urqlClient: Client | null = null;

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
    if (!root) logError("[getUrqlClient] No workspace root found");
    if (!command) logError("[getUrqlClient] No repo command found");
    return null;
  }

  log(`[getUrqlClient] Initializing URQL client with command: ${command}`);
  urqlClient = new Client({
    url: "local://graphql",
    exchanges: [cacheExchange, fetchExchange],
    fetch: async (_input: RequestInfo | URL, init?: RequestInit) => {
      log("[urql] fetch triggered");
      try {
        const body = init?.body ? JSON.parse(init.body as string) : {};
        const query = (body.query as string) || "";
        const variables = body.variables || {};
        const variablesJson = JSON.stringify(variables);
        log(`[urql] executing graphql via execFile. Variables: ${variablesJson.length > 100 ? variablesJson.substring(0, 100) + '...' : variablesJson}`);
        const repoPath = getRepoCommand();
        if (!repoPath) throw new Error("Repo command not found");
        const repoArgs = ["--json", "graphql", query];
        if (Object.keys(variables).length > 0) {
          repoArgs.push("-v", variablesJson);
        }
        const start = Date.now();
        const { stdout, stderr } = await execFileAsync(repoPath, repoArgs, {
          cwd: root,
          timeout: 45000,
          maxBuffer: 500 * 1024 * 1024,
        });
        const duration = Date.now() - start;
        log(`[urql] CLI execution successful in ${duration}ms, stdout size: ${stdout.length}`);
        if (stderr) {
          log("[urql] CLI stderr (first 100):", stderr.substring(0, 100));
        }
        if (!stdout.trim()) {
          logError("[urql] CLI returned empty stdout");
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
        logError("[urql] fetch execution error:", error);
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

// #endregion urql Client

// #region GraphQL Documents

const RepoDocument = graphql(`
  query Repo {
    repo {
      id
      name
      path
      projects {
        id
        name
        kind
        root
        bundles {
          id
          name
          root
          sourceRoot
          projectType
          tags
          uri
        }
      }
      commits(limit: 100) {
        id
        sha
        title
        date
      }
      bundles {
        id
        name
        root
        sourceRoot
        projectType
        tags
        uri
      }
      tickets {
        id
        year
        month
        day
        slug
        path
        uri
        prompt
        summary
        status
        commit
        goal
        author {
          name
          github
        }
      }
      goals {
        id
        title
        description
        status
      }
      policies {
        id
        name
        description
        scopes
        violationKinds {
          id
          priority
          autofixable
          reason
          solution
        }
      }
      contributors {
        id
        github
        name
        emails
        links {
          name
          url
        }
        contributions {
           commits {
             id 
             sha
             title
             date
           }
        }
      }
    }
  }
`);

const FolderContentDocument = graphql(`
  query FolderContent($path: String!) {
    folder(path: $path) {
      children {
        path
        name
        uri
      }
      files {
        path
        name
        uri
      }
    }
  }
`);

const BundlesDocument = graphql(`
  query Bundles {
    repo {
      bundles { id name root sourceRoot projectType tags uri }
    }
  }
`);

const TicketsDocument = graphql(`
  query Tickets($year: Int, $month: Int, $day: Int, $status: TicketStatus) {
    repo {
      tickets(year: $year, month: $month, day: $day, status: $status) {
        id year month day slug path uri prompt summary status
        author { github name }
        llm commit
        dates { started finished }
        iterations {
          prompt
          plan
          llm
          ui
          author { github name }
          started
          finished
          commit
        }
      }
    }
  }
`);

const PoliciesDocument = graphql(`
  query Policies {
    repo {
      policies { id name description scopes violationKinds { id priority autofixable reason solution } }
    }
  }
`);

const ContributorsDocument = graphql(`
  query Contributors {
    repo {
      contributors {
        id github name emails
        links { name url }
        icons { avatar avatarRound github }
        contributions {
          commits {
            id sha title
          }
          tickets {
            slug year month day title summary status
          }
          bundles {
            name
            lines { added removed }
            folders {
              name
              lines { added removed }
              files {
                name
                lines { added removed }
                sections {
                  name
                  lines { added removed }
                  definitions {
                    name
                    lines { added removed }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
`);

const AnalyzeDocument = graphql(`
  query Analyze($scope: String) {
    analyze(scope: $scope) {
      violations {
        id summary priority autofixable scope line column excerpt
        kind { id policy { id name } reason solution }
      }
      metrics { total byPriority { high medium low } autofixable }
    }
  }
`);

const FixDocument = graphql(`
  mutation Fix($scope: String) {
    fix(scope: $scope) {
      fixed remaining
      violations { id summary priority scope }
    }
  }
`);

const CodebaseDocument = graphql(`
  query Codebase {
    repo {
      id name path
      bundles {
        id name root sourceRoot projectType tags uri
      }
      folders {
        id path uri
      }
      files {
        id path uri
        sections {
          id name path
          range { start end }
        }
        definitions {
          id name kind
          range { start end }
        }
      }
      contributors {
        id github name emails
        links { name url }
      }
      tickets {
        id year month day slug path uri prompt summary status commit
        author { github name }
      }
      policies {
        id name description scopes
        violationKinds { id priority autofixable reason solution }
      }
    }
  }
`);

const FileContentDocument = graphql(`
  query FileContent($path: String!) {
    file(path: $path) {
      path
      name
      uri
      sections {
        id
        name
        range { start end }
        parent { id }
      }
      definitions {
        id
        name
        kind
        range { start end }
        section { id }
      }
    }
  }
`);

const TodosDocument = graphql(`
  query Todos($filter: FilterInput) {
    todos(filter: $filter) {
      id
      text
      location {
        file
        line
        column
      }
    }
  }
`);

const TodoCreateDocument = graphql(`
  mutation TodoCreate($input: TodoCreateInput!) {
    todoCreate(input: $input) {
      id
    }
  }
`);

const TodoDeleteDocument = graphql(`
  mutation TodoDelete($id: ID!) {
    todoDelete(id: $id)
  }
`);

const GoalsDocument = graphql(`
  query Goals {
    repo {
      goals {
        id
        title
        description
        prompt
        status
        dueDate
        ui
        llm
        milestone
      }
    }
  }
`);

// #endregion GraphQL Documents

// #region GraphQL Types

type Repo = DocumentType<typeof RepoDocument>["repo"];
type Bundle = Repo["bundles"][number];

type Ticket = DocumentType<typeof TicketsDocument>["repo"]["tickets"][number];

type Policy = DocumentType<typeof PoliciesDocument>["repo"]["policies"][number];
type ViolationKind = Policy["violationKinds"][number];

type Contributor = DocumentType<typeof ContributorsDocument>["repo"]["contributors"][number];

type AnalyzeResult = DocumentType<typeof AnalyzeDocument>["analyze"];
type GqlViolation = AnalyzeResult["violations"][number];

type FixResult = DocumentType<typeof FixDocument>["fix"];

type GqlCodebase = DocumentType<typeof CodebaseDocument>["repo"];
type CodebaseBundle = GqlCodebase["bundles"][number];
type CodebaseFolder = GqlCodebase["folders"][number];
type CodebaseFile = GqlCodebase["files"][number];
type CodebaseSection = CodebaseFile["sections"][number];
type CodebaseDefinition = CodebaseFile["definitions"][number];
type CodebaseContributor = GqlCodebase["contributors"][number];
type CodebaseTicket = GqlCodebase["tickets"][number];
type CodebasePolicy = GqlCodebase["policies"][number];

type TreeNodeKind = "root" | "repo" | "bundle" | "folder" | "file" | "section" | "definition";

interface TreeNodeMap {
  [key: string]: TreeNodeEntry;
}

interface TreeNodeEntry {
  kind: TreeNodeKind;
  children?: TreeNodeMap;
  filePath?: string;
}

interface Codebase extends GqlCodebase {
  tree: TreeNodeMap;
}

// #endregion GraphQL Types

// #region GraphQL Helpers

const RepoRootQuery = `
  query RepoRoot {
    repo {
      id
      name
      path
      bundles {
        id
        name
        root
        sourceRoot
        projectType
        tags
        uri
      }
    }
  }
`;

async function fetchRepoViaGraphQL(): Promise<Repo | null> {
  const client = getUrqlClient();
  if (!client) return null;
  // Use explicit string query instead of RepoDocument to avoid fetching tickets/contributors/policies
  // This drastically reduces payload size and improve startup performance.
  // We use 'as any' because the project usually uses TypedDocumentNode, but we want a lightweight ad-hoc query here.
  const result = await client.query(RepoRootQuery as any, {});
  if (result.error) {
    logError("[GraphQL] fetchRepoViaGraphQL error:", result.error);
    return null;
  }
  // Data will lack tickets/contributors/policies, but consumers (loadCodebase) only strictly need bundles.
  return result.data?.repo as unknown as Repo ?? null;
}

async function fetchBundlesViaGraphQL(): Promise<Bundle[]> {
  const client = getUrqlClient();
  if (!client) return [];
  const result = await client.query(BundlesDocument, {});
  if (result.error) {
    logError("[GraphQL] fetchBundlesViaGraphQL error:", result.error);
    return [];
  }
  return result.data?.repo?.bundles ?? [];
}

async function fetchFolderContent(path: string): Promise<DocumentType<typeof FolderContentDocument>["folder"] | null> {
  const client = getUrqlClient();
  if (!client) return null;
  const result = await client.query(FolderContentDocument, { path });
  if (result.error) {
    logError("[GraphQL] fetchFolderContent error:", result.error);
    return null;
  }
  return result.data?.folder ?? null;
}

async function fetchTicketsViaGraphQL(year?: number, month?: number, day?: number, status?: TicketStatus): Promise<Ticket[]> {
  const client = getUrqlClient();
  if (!client) return [];
  const result = await client.query(TicketsDocument, { year, month, day, status });
  if (result.error) {
    logError("[GraphQL] fetchTicketsViaGraphQL error:", result.error);
    return [];
  }
  return result.data?.repo?.tickets ?? [];
}

async function fetchTodosViaGraphQL(): Promise<Todo[]> {
  const client = getUrqlClient();
  if (!client) return [];
  const filter = {};
  const result = await client.query(TodosDocument, { filter });
  if (result.error) {
    logError("[GraphQL] fetchTodosViaGraphQL error:", result.error);
    return [];
  }
  return result.data?.todos ?? [];
}

async function fetchPoliciesViaGraphQL(): Promise<Policy[]> {
  const client = getUrqlClient();
  if (!client) return [];
  const result = await client.query(PoliciesDocument, {});
  if (result.error) {
    logError("[GraphQL] fetchPoliciesViaGraphQL error:", result.error);
    return [];
  }
  return result.data?.repo?.policies ?? [];
}

async function fetchContributorsViaGraphQL(): Promise<Contributor[]> {
  const client = getUrqlClient();
  if (!client) return [];
  const result = await client.query(ContributorsDocument, {});
  if (result.error) {
    logError("[GraphQL] fetchContributorsViaGraphQL error:", result.error);
    return [];
  }
  return result.data?.repo?.contributors ?? [];
}

async function analyzeViaGraphQL(scope?: string): Promise<AnalyzeResult | null> {
  const client = getUrqlClient();
  if (!client) return null;
  const result = await client.query(AnalyzeDocument, { scope });
  if (result.error) {
    logError("[GraphQL] analyzeViaGraphQL error:", result.error);
    return null;
  }
  return result.data?.analyze ?? null;
}

async function fixViaGraphQL(scope?: string): Promise<FixResult | null> {
  const client = getUrqlClient();
  if (!client) return null;
  const result = await client.mutation(FixDocument, { scope });
  if (result.error) {
    logError("[GraphQL] fixViaGraphQL error:", result.error);
    return null;
  }
  return result.data?.fix ?? null;
}

// #endregion GraphQL Helpers

// #region Constants

const runningProcesses = new Map<string, AbortController>();

const SEMIO_KIT_LANGUAGE = "json";
const DIAGNOSTIC_SOURCE = "semio";

let cachedCodebase: Codebase | null = null;
let codebaseLoadPromise: Promise<Codebase | null> | null = null;
let cachedProjects: ProjectData[] | null = null;
let cachedRepoBaseUrl: string | undefined = undefined;
const Clientient_STRINGS = {
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

interface TextEdit {
  start: number;
  end: number;
  newText: string;
}

interface AutoFix {
  description: string;
  edits: Record<string, TextEdit[]>;
}

// Removed duplicate ViolationKind interface

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

// #endregion Types

// #region Utilities

function getWorkspaceRoot(): string | undefined {
  return vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
}

function getRepoBinaryPath(): string | undefined {
  const root = getWorkspaceRoot();
  if (!root) return undefined;
  const isWindows = process.platform === "win32";
  const binaryName = isWindows ? "repo.exe" : "repo";
  const binaryPath = path.join(root, "go", "repo", binaryName);
  log("getRepoBinaryPath:", binaryPath, "exists:", fs.existsSync(binaryPath));
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
  if (!root) {
    logError("[runRepoCommandJson] no workspace root");
    return null;
  }
  const command = getRepoCommand();
  if (!command) {
    logError("[runRepoCommandJson] no repo command found");
    return null;
  }
  // Add --json flag to get JSONL output
  const fullCommand = `"${command}" --json ${args}`;
  log("[runRepoCommandJson] executing:", fullCommand, "cwd:", root);
  try {
    const { stdout, stderr } = await execAsync(fullCommand, { cwd: root, timeout: 60000, maxBuffer: 10 * 1024 * 1024 });
    if (stderr) {
      log("[runRepoCommandJson] stderr:", stderr.substring(0, 500));
    }
    if (stdout.length === 0) {
      logError("[runRepoCommandJson] stdout is empty!");
      return null;
    }
    // Parse JSONL output - extract result from event stream
    const events = parseRepoEvents(stdout);
    const result = extractRepoResult(events);
    if (result && "data" in result) {
      return { data: result.data, output: { exitCode: 0, lines: [] } } as any;
    }
    return result as T;
  } catch (error) {
    logError("[runRepoCommandJson] error:", error);
    if (error instanceof Error) {
      logError("[runRepoCommandJson] error message:", error.message);
      logError("[runRepoCommandJson] error stack:", error.stack);
    }
    return null;
  }
}

function normalizeSectionTree(sections: (GraphqlSection | any)[]): SectionInfo[] {
  return sections.map((sectionOrWrapper) => {
    // Handle CLI wrapper { section: { ... } }
    const section = sectionOrWrapper.section ?? sectionOrWrapper;

    return {
      name: section.name,
      kind: section.__typename ?? "Section",
      // Use optional chaining carefully, section might be a raw JSON object from CLI
      startLine: section.range?.start?.line ?? section.startLine ?? 0,
      endLine: section.range?.end?.line ?? section.endLine ?? 0,
      startIndex: section.startIndex ?? 0,
      endIndex: section.endIndex ?? 0,
      children: normalizeSectionTree(section.children ?? []),
    };
  });
}

function extractSections(result: any): SectionInfo[] {
  if (!result) {
    log("[extractSections] result is null/undefined");
    return [];
  }
  log("[extractSections] result keys:", Object.keys(result));
  if ("data" in result) {
    const data = result.data;
    log("[extractSections] data:", typeof data, Array.isArray(data) ? "isArray" : "notArray");
    if (Array.isArray(data)) return data;
    if (data && typeof data === "object") {
      log("[extractSections] data keys:", Object.keys(data));
      if ("sections" in data && Array.isArray(data.sections)) {
        log("[extractSections] found", data.sections.length, "sections in data.sections");
        return normalizeSectionTree(data.sections);
      }
      // Handle result.data.file.sections structure
      if ("file" in data && data.file?.sections) {
        log("[extractSections] found", data.file.sections.length, "sections in data.file.sections");
        return normalizeSectionTree(data.file.sections);
      }
    }
  }
  if ("file" in result) {
    log("[extractSections] found sections in result.file, count:", result.file?.sections?.length ?? 0);
    return normalizeSectionTree(result.file?.sections ?? []);
  }
  log("[extractSections] no sections found, returning empty array");
  return [];
}

function extractDefinitions(result: any): DefinitionInfo[] {
  if (!result) return [];
  if ("data" in result) {
    const data = result.data;
    if (Array.isArray(data)) return data;
    if (data && typeof data === "object") {
      if ("definitions" in data && Array.isArray(data.definitions)) {
        return data.definitions;
      }
      if ("file" in data && data.file?.definitions) {
        return data.file.definitions;
      }
    }
  }
  if ("file" in result && result.file?.definitions) {
    return result.file.definitions;
  }
  return [];
}

async function getSectionListForFile(filePath: string): Promise<SectionInfo[]> {
  log("[getSectionListForFile] fetching sections for:", filePath);
  const result = await runRepoCommandJson<ToolResult<SectionInfo[]> | { file?: { sections?: GraphqlSection[] | null } | null }>(`section list --file "${filePath}"`);
  log("[getSectionListForFile] result:", result ? "received" : "null", result ? JSON.stringify(result).substring(0, 200) : "");
  const sections = extractSections(result);
  log("[getSectionListForFile] extracted sections count:", sections.length);
  return sections;
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

    // Initialize missing arrays if any
    const gqlRepo = repo as any;
    if (!gqlRepo.files) gqlRepo.files = [];
    if (!gqlRepo.folders) gqlRepo.folders = [];

    const tree: TreeNodeMap = {};

    // Initialize tree with bundles as root items
    for (const bundle of repo.bundles) {
      tree[bundle.id] = { kind: "bundle", children: {} };
    }

    const codebase: Codebase = { ...gqlRepo, tree };
    cachedCodebase = codebase;
    codebaseLoadPromise = null;

    if (cachedCodebase) {
      log(`[loadCodebase] Loaded ${cachedCodebase.bundles.length} bundles, ${cachedCodebase.tickets.length} tickets, ${cachedCodebase.contributors.length} contributors`);
      cachedProjects = cachedCodebase.bundles.map((b) => ({
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

function refreshCodebase(): void {
  cachedCodebase = null;
  codebaseLoadPromise = null;
  cachedProjects = null;
}

async function getCodebase(): Promise<Codebase | null> {
  return loadCodebase();
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

function resolveCommitSha(commit: string | { sha?: string } | undefined): string | undefined {
  if (!commit) return undefined;
  if (typeof commit === "string") return commit;
  return commit.sha;
}

function getUiString(key: keyof typeof Client_STRINGS.en): string {
  const language = vscode.env.language.split("-")[0];
  const bundle = UI_STRINGS[language as keyof typeof UI_STRINGS] ?? UI_STRINGS.en;
  return bundle[key];
}

interface ToolResult<T = unknown> {
  output: { lines: { type: string; text: string }[]; exitCode: number };
  data?: T;
  error?: string;
}

interface LineMetrics {
  added: number;
  removed: number;
}

interface SectionStats {
  definitions?: string[];
  lines?: LineMetrics;
}

interface FileStats {
  sections?: Record<string, SectionStats>;
}

interface BundleStats {
  files?: Record<string, FileStats>;
}

type TicketBundles = Record<string, BundleStats>;

interface TicketFrontmatter {
  status: string;
  prompt: string;
  summary?: string;
  author?: string;
  commit?: string;
  started?: string;
  finished?: string;
  ignore?: boolean;
}

interface TicketIteration {
  prompt: string;
  plan?: string;
  llm: string;
  ui: string;
  author: { github: string; name?: string | null };
  started: string;
  finished?: string;
  commit?: string;
}

interface TicketData {
  year: number;
  month: number;
  day: number;
  slug: string;
  frontmatter: TicketFrontmatter;
  folderPath: string;
  filePath: string;
  iterations?: TicketIteration[];
}

interface PolicyData {
  id: string;
  name: string;
  description: string;
}

interface ProjectData {
  name: string;
  root: string;
  sourceRoot?: string;
  projectType?: string;
  tags?: string[];
}

async function pickTicket(statusFilter?: "open" | "closed"): Promise<TicketData | undefined> {
  const result = await runRepoCommandJson<ToolResult<TicketData[]>>("ticket list");
  log("pickTicket result:", result ? `data length: ${result.data?.length}` : "null");
  if (!result) {
    vscode.window.showWarningMessage("Failed to run ticket list command");
    return undefined;
  }
  if (!result.data || result.data.length === 0) {
    vscode.window.showWarningMessage("No tickets found");
    return undefined;
  }
  let tickets = result.data;
  if (statusFilter) {
    tickets = tickets.filter((t) => t.frontmatter.status === statusFilter);
    if (tickets.length === 0) {
      vscode.window.showWarningMessage(`No ${statusFilter} tickets found`);
      return undefined;
    }
  }
  const items = tickets.map((t) => ({
    label: `${t.year}/${String(t.month).padStart(2, "0")}/${String(t.day).padStart(2, "0")}/${t.slug}`,
    description: t.frontmatter.status,
    detail: t.frontmatter.summary || t.frontmatter.prompt,
    ticket: t,
  }));
  const picked = await vscode.window.showQuickPick(items, { placeHolder: "Select a ticket" });
  return picked?.ticket;
}

async function pickPolicy(): Promise<PolicyData | undefined> {
  const result = await runRepoCommandJson<ToolResult<PolicyData[]>>("policy list");
  if (!result?.data || result.data.length === 0) {
    vscode.window.showWarningMessage("No policies found");
    return undefined;
  }
  const items = result.data.map((p) => ({
    label: p.id,
    description: p.name,
    detail: p.description,
    policy: p,
  }));
  const picked = await vscode.window.showQuickPick(items, { placeHolder: "Select a policy" });
  return picked?.policy;
}

async function pickFiles(preselectedFiles?: string[]): Promise<string[] | undefined> {
  const root = getWorkspaceRoot();
  if (!root) {
    vscode.window.showErrorMessage("No workspace folder open");
    return undefined;
  }
  const files = await vscode.window.showOpenDialog({
    canSelectFiles: true,
    canSelectFolders: false,
    canSelectMany: true,
    defaultUri: preselectedFiles?.length ? vscode.Uri.file(path.join(root, preselectedFiles[0])) : vscode.Uri.file(root),
    openLabel: "Select Files",
    title: "Select files to include in ticket (at least one required)",
  });
  if (!files || files.length === 0) return undefined;
  return files.map((f) => vscode.workspace.asRelativePath(f));
}

function getActiveFileRelativePath(): string | undefined {
  const editor = vscode.window.activeTextEditor;
  if (!editor) return undefined;
  return vscode.workspace.asRelativePath(editor.document.uri);
}

function pinDiagnosticPreview(editor?: vscode.TextEditor): void {
  if (!editor) return;
  // Use setTimeout to allow VSCode to settle after opening the file from Problems panel
  // This ensures the tab state and diagnostics are properly updated before checking
  setTimeout(() => {
    const currentEditor = vscode.window.activeTextEditor;
    if (!currentEditor) return;
    const activeTab = vscode.window.tabGroups?.activeTabGroup?.activeTab;
    if (!activeTab || !activeTab.isPreview) return;
    if (!vscode.languages.getDiagnostics(currentEditor.document.uri).some((d) => d.source === DIAGNOSTIC_SOURCE)) return;
    vscode.commands.executeCommand("workbench.action.keepEditor");
  }, 50);
}

// #endregion Utilities

// #region File Analysis

let repoDiagnosticCollection: vscode.DiagnosticCollection;
const fileViolationsMap = new Map<string, Violation[]>();
let bundleCache: Bundle[] = [];

async function updateBundleCache() {
  const bundles = await fetchBundlesViaGraphQL();
  if (bundles.length > 0) {
    bundleCache = bundles;
  }
}

function extractFilePathFromScope(scope: string): string | undefined {
  let cleanScope = scope;
  if (cleanScope.startsWith("@semio/violations/")) {
    cleanScope = cleanScope.replace("@semio/violations/", "");
  }

  // Handle hierarchical IDs: BUNDLE/RELATIVEPATH
  // We find the longest matching bundle ID
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

    // Clean up trailing slash if it was just the root
    return filePath.endsWith("/") ? filePath.slice(0, -1) : filePath;
  }

  // Fallback to legacy guessing logic if bundle not found or cache empty
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
  log("[extractFilePathFromScope] could not identify file path from scope:", scope, "cleanScope:", cleanScope, "candidate:", filePath);
  return undefined;
}

// Directories that should never be analyzed (matching repo binary behavior)
const ignoredDirectories = new Set([
  "node_modules",
  "venv",
  "dist",
  "build",
  "out",
  "__pycache__",
  "coverage",
  "site-packages",
  "eggs",
  "wheels",
  "htmlcov",
  "target",
  "artifacts",
  "vendor",
]);

// Dot-prefixed directories that ARE allowed for analysis
const allowedDotDirectories = new Set([".github", ".devcontainer", ".semio-repo"]);

function isInIgnoredDirectory(relativePath: string): boolean {
  const segments = relativePath.split("/");
  return segments.some((segment) => {
    // Check explicit ignore list
    if (ignoredDirectories.has(segment)) return true;
    // Skip dot-prefixed directories unless explicitly allowed (matches repo binary behavior)
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

  // Ensure bundle cache is populated
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

    log("[analyzeFile] result for", relativePath, ":", result ? `data: ${result.data ? "present" : "missing"}` : "null");

    if (result?.data?.violations) {
      log("[analyzeFile] found", result.data.violations.length, "violations");
      fileViolationsMap.set(fileUri.toString(), result.data.violations);
      updateFileDiagnostics(document, result.data.violations);
    } else {
      log("[analyzeFile] no violations found or result format unexpected");
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

  // Always initialize target document with empty array to ensure it's cleared if no violations found for it
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

class RepoCodeActionProvider implements vscode.CodeActionProvider {
  provideCodeActions(document: vscode.TextDocument, range: vscode.Range | vscode.Selection, context: vscode.CodeActionContext): vscode.CodeAction[] | undefined {
    const repoDiagnostics = context.diagnostics.filter((d) => d.source === DIAGNOSTIC_SOURCE);
    if (repoDiagnostics.length === 0) return undefined;
    const root = getWorkspaceRoot();
    if (!root) return undefined;
    if (document.uri.scheme !== "file") return undefined;
    const relativePath = path.relative(root, document.uri.fsPath).replace(/\\/g, "/");
    if (relativePath.startsWith("..")) return undefined;
    const fileUri = vscode.Uri.file(path.join(root, relativePath));
    const violations = fileViolationsMap.get(fileUri.toString()) || [];
    const actions: vscode.CodeAction[] = [];
    for (const diagnostic of repoDiagnostics) {
      const diagnosticLine = diagnostic.range.start.line + 1;
      const policyId = typeof diagnostic.code === "object" && diagnostic.code !== null ? (diagnostic.code as { value: string }).value : diagnostic.code;
      const violation = violations.find((v) => {
        let vId = v.kind.id;
        if (vId.startsWith("@semio/policies//violations/")) {
          vId = vId.replace("@semio/policies//violations/", "");
        }
        return vId === policyId && (v.line ?? 1) === diagnosticLine;
      });
      if (!violation) continue;
      const action = createRepoCodeAction(document, diagnostic, violation);
      if (action) actions.push(action);
    }
    return actions;
  }
}

function createRepoCodeAction(document: vscode.TextDocument, diagnostic: vscode.Diagnostic, violation: Violation): vscode.CodeAction | undefined {
  let kindId = violation.kind.id;
  if (kindId.startsWith("@semio/policies//violations/")) {
    kindId = kindId.replace("@semio/policies//violations/", "");
  }
  const [, violationName] = kindId.split(":");
  const action = new vscode.CodeAction(`Fix: ${violationName || kindId}`, vscode.CodeActionKind.QuickFix);
  action.diagnostics = [diagnostic];
  action.isPreferred = true;
  if (violation.kind.autofixable) {
    const root = getWorkspaceRoot();
    if (root) {
      const relativePath = path.relative(root, document.uri.fsPath).replace(/\\/g, "/");
      action.command = {
        command: "semio.fixViolation",
        title: "Fix Violation",
        arguments: [relativePath],
      };
    }
  }
  return action;
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

// #endregion File Analysis

// #region Kit Validation

let kitDiagnosticCollection: vscode.DiagnosticCollection;

function isKitDocument(document: vscode.TextDocument): boolean {
  if (document.languageId !== SEMIO_KIT_LANGUAGE) return false;
  const basename = document.uri.path.split("/").pop()?.toLowerCase() || "";
  return basename.startsWith("kit_") || basename.includes("_kit") || basename === "kit.json";
}

function problemToDiagnostic(document: vscode.TextDocument, problem: Problem): vscode.Diagnostic {
  const range = locationToRange(document, problem.location);
  const diagnostic = new vscode.Diagnostic(range, problem.message, vscode.DiagnosticSeverity.Error);
  diagnostic.source = DIAGNOSTIC_SOURCE;
  // Use object form with target to ensure clicking opens the file at the correct location
  const line = range.start.line + 1;
  diagnostic.code = { value: problem.constraintId, target: document.uri.with({ fragment: `L${line}` }) };
  if (problem.relatedGuids && problem.relatedGuids.length > 1) {
    diagnostic.relatedInformation = problem.relatedGuids.slice(1).map((guid) => {
      const relatedRange = findGuidRange(document, guid);
      return new vscode.DiagnosticRelatedInformation(new vscode.Location(document.uri, relatedRange), `Related entity: ${guid}`);
    });
  }
  return diagnostic;
}

function locationToRange(document: vscode.TextDocument, location: DomainLocation): vscode.Range {
  if (!location.entityGuid) return new vscode.Range(0, 0, 0, 0);
  const text = document.getText();
  const tree = jsonc.parseTree(text);
  if (!tree) return new vscode.Range(0, 0, 0, 0);
  const entityNode = findEntityNode(tree, location);
  if (!entityNode) return new vscode.Range(0, 0, 0, 0);
  const startPos = document.positionAt(entityNode.offset);
  const endPos = document.positionAt(entityNode.offset + entityNode.length);
  return new vscode.Range(startPos, endPos);
}

function findEntityNode(tree: jsonc.Node, location: DomainLocation): jsonc.Node | undefined {
  const entityKindToArrayName: Record<string, string> = {
    Type: "types",
    Design: "designs",
    Quality: "qualities",
    Port: "ports",
    File: "files",
    Folder: "folders",
    Piece: "pieces",
    Connection: "connections",
    Stat: "stats",
    Llm: "llms",
    Connector: "connectors",
    Layer: "layers",
  };
  const arrayName = entityKindToArrayName[location.entityKind];
  if (!arrayName) return undefined;
  const arrayNode = jsonc.findNodeAtLocation(tree, [arrayName]);
  if (!arrayNode || arrayNode.type !== "array") return undefined;
  for (const child of arrayNode.children || []) {
    const guidNode = jsonc.findNodeAtLocation(child, ["guid"]);
    if (guidNode?.type === "string" && guidNode.value === location.entityGuid) {
      if (location.field) {
        const fieldNode = jsonc.findNodeAtLocation(child, [location.field]);
        return fieldNode || child;
      }
      return child;
    }
  }
  if (location.entityKind === "Piece" || location.entityKind === "Connection" || location.entityKind === "Stat" || location.entityKind === "Layer") {
    const designsNode = jsonc.findNodeAtLocation(tree, ["designs"]);
    if (designsNode && designsNode.type === "array") {
      for (const designNode of designsNode.children || []) {
        const subArrayNode = jsonc.findNodeAtLocation(designNode, [arrayName]);
        if (subArrayNode && subArrayNode.type === "array") {
          for (const child of subArrayNode.children || []) {
            const guidNode = jsonc.findNodeAtLocation(child, ["guid"]);
            if (guidNode?.type === "string" && guidNode.value === location.entityGuid) {
              if (location.field) {
                const fieldNode = jsonc.findNodeAtLocation(child, [location.field]);
                return fieldNode || child;
              }
              return child;
            }
          }
        }
      }
    }
  }
  if ((location.entityKind as string) === "Llm" || location.entityKind === "Connector") {
    const typesNode = jsonc.findNodeAtLocation(tree, ["types"]);
    if (typesNode && typesNode.type === "array") {
      for (const typeNode of typesNode.children || []) {
        const subArrayNode = jsonc.findNodeAtLocation(typeNode, [arrayName]);
        if (subArrayNode && subArrayNode.type === "array") {
          for (const child of subArrayNode.children || []) {
            const guidNode = jsonc.findNodeAtLocation(child, ["guid"]);
            if (guidNode?.type === "string" && guidNode.value === location.entityGuid) {
              if (location.field) {
                const fieldNode = jsonc.findNodeAtLocation(child, [location.field]);
                return fieldNode || child;
              }
              return child;
            }
          }
        }
      }
    }
  }
  return undefined;
}

function findGuidRange(document: vscode.TextDocument, guid: string): vscode.Range {
  const text = document.getText();
  const tree = jsonc.parseTree(text);
  if (!tree) return new vscode.Range(0, 0, 0, 0);
  const node = findNodeByGuid(tree, guid);
  if (!node) return new vscode.Range(0, 0, 0, 0);
  const startPos = document.positionAt(node.offset);
  const endPos = document.positionAt(node.offset + node.length);
  return new vscode.Range(startPos, endPos);
}

function findNodeByGuid(node: jsonc.Node, guid: string): jsonc.Node | undefined {
  if (node.type === "object") {
    const guidNode = jsonc.findNodeAtLocation(node, ["guid"]);
    if (guidNode?.type === "string" && guidNode.value === guid) return node;
  }
  if (node.type === "array" || node.type === "object") {
    for (const child of node.children || []) {
      const result = findNodeByGuid(child, guid);
      if (result) return result;
    }
  }
  return undefined;
}

function validateKitDocument(document: vscode.TextDocument): void {
  if (!isKitDocument(document)) return;
  try {
    const text = document.getText();
    const kit = deserializeKit(text);
    const result = validateKit(kit);
    const diagnostics = result.problems.map((problem) => problemToDiagnostic(document, problem));
    kitDiagnosticCollection.set(document.uri, diagnostics);
  } catch (error) {
    logError("Failed to validate semio kit:", error);
    kitDiagnosticCollection.delete(document.uri);
  }
}

class KitCodeActionProvider implements vscode.CodeActionProvider {
  provideCodeActions(document: vscode.TextDocument, range: vscode.Range | vscode.Selection, context: vscode.CodeActionContext): vscode.CodeAction[] | undefined {
    const kitDiagnostics = context.diagnostics.filter((d) => d.source === DIAGNOSTIC_SOURCE);
    if (kitDiagnostics.length === 0) return undefined;
    const actions: vscode.CodeAction[] = [];
    for (const diagnostic of kitDiagnostics) {
      try {
        const text = document.getText();
        const kit = deserializeKit(text);
        const result = validateKit(kit);
        const diagnosticCode = typeof diagnostic.code === "object" && diagnostic.code !== null ? (diagnostic.code as { value: string }).value : diagnostic.code;
        const problem = result.problems.find((i) => i.message === diagnostic.message && i.constraintId === diagnosticCode);
        if (!problem) continue;
        for (const fix of problem.fixes) {
          const action = createKitCodeAction(document, diagnostic, fix, kit);
          if (action) actions.push(action);
        }
      } catch (error) {
        logError("Failed to generate code actions:", error);
      }
    }
    return actions;
  }
}

function createKitCodeAction(document: vscode.TextDocument, diagnostic: vscode.Diagnostic, fix: Fix, kit: Kit): vscode.CodeAction | undefined {
  try {
    const fixedKit = applyKitDiff(kit, fix.diff);
    const fixedJson = serializeKit(fixedKit);
    const action = new vscode.CodeAction(fix.title, vscode.CodeActionKind.QuickFix);
    action.diagnostics = [diagnostic];
    action.isPreferred = true;
    const edit = new vscode.WorkspaceEdit();
    const fullRange = new vscode.Range(document.positionAt(0), document.positionAt(document.getText().length));
    edit.replace(document.uri, fullRange, fixedJson);
    action.edit = edit;
    return action;
  } catch (error) {
    logError("Failed to create code action:", error);
    return undefined;
  }
}

// #endregion Kit Validation

// #region Sidebar Views

let globalSearchQuery = "";
let globalMatchCase = false;
let globalMatchWholeWord = false;
let globalUseRegex = false;

function matchesSearchText(text: string, query: string = ""): boolean {
  // Use passed query or global fallback (from filter provider if initialized)
  const currentQuery = query || (filterProvider ? filterProvider.globalSearchQuery : "");
  if (!currentQuery) return true;

  try {
    if (globalUseRegex) {
      const flags = globalMatchCase ? "" : "i";
      const pattern = globalMatchWholeWord ? `\\b${currentQuery}\\b` : currentQuery;
      const regex = new RegExp(pattern, flags);
      return regex.test(text);
    } else {
      const q = globalMatchCase ? currentQuery : currentQuery.toLowerCase();
      const target = globalMatchCase ? text : text.toLowerCase();
      if (globalMatchWholeWord) {
        const wordRegex = new RegExp(`\\b${q.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\\b`, globalMatchCase ? "" : "i");
        return wordRegex.test(text);
      }
      return target.includes(q);
    }
  } catch {
    return text.toLowerCase().includes(currentQuery.toLowerCase());
  }
}

class SearchViewProvider implements vscode.WebviewViewProvider {
  public static readonly viewType = "semio.search";
  private _view?: vscode.WebviewView;

  constructor(private readonly _extensionUri: vscode.Uri) { }

  public resolveWebviewView(webviewView: vscode.WebviewView): void {
    this._view = webviewView;
    webviewView.webview.options = { enableScripts: true };
    webviewView.webview.html = this._getHtmlForWebview();
    webviewView.webview.onDidReceiveMessage((data) => {
      switch (data.type) {
        case "search":
          globalSearchQuery = data.query;
          globalMatchCase = data.matchCase;
          globalMatchWholeWord = data.matchWholeWord;
          globalUseRegex = data.useRegex;
          if (filterProvider) {
            filterProvider.setSearchQuery(data.query, data.matchCase, data.matchWholeWord, data.useRegex);
          }
          ticketsProvider.refresh();
          policiesProvider.refresh();
          contributorsProvider.refresh();
          commandsProvider.refresh();
          break;
      }
    });
  }

  private _getHtmlForWebview(): string {
    return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <style>
    * { box-sizing: border-box; margin: 0; padding: 0; }
    body { padding: 8px; font-family: var(--vscode-font-family); font-size: var(--vscode-font-size); }
    .search-box {
      display: flex;
      align-items: center;
      background: var(--vscode-input-background);
      border: 1px solid var(--vscode-input-border, transparent);
      border-radius: 2px;
    }
    .search-box:focus-within { border-color: var(--vscode-focusBorder); }
    input[type="text"] {
      flex: 1;
      min-width: 0;
      padding: 3px 4px 3px 6px;
      border: none;
      background: transparent;
      color: var(--vscode-input-foreground);
      outline: none;
      font-size: 13px;
      line-height: 18px;
    }
    input[type="text"]::placeholder { color: var(--vscode-input-placeholderForeground); }
    .toggles { display: flex; padding: 0 2px; gap: 1px; }
    .toggle-btn {
      width: 20px;
      height: 20px;
      border: 1px solid transparent;
      background: transparent;
      color: var(--vscode-foreground);
      cursor: pointer;
      display: flex;
      align-items: center;
      justify-content: center;
      opacity: 0.7;
      border-radius: 3px;
      margin: 2px 0;
    }
    .toggle-btn:hover { background: var(--vscode-inputOption-hoverBackground, rgba(90, 93, 94, 0.31)); opacity: 1; }
    .toggle-btn.active {
      background: var(--vscode-inputOption-activeBackground, rgba(0, 127, 212, 0.4));
      border-color: var(--vscode-inputOption-activeBorder, var(--vscode-focusBorder));
      color: var(--vscode-inputOption-activeForeground, #fff);
      opacity: 1;
    }
    .toggle-btn svg { width: 14px; height: 14px; fill: currentColor; }
  </style>
</head>
<body>
  <div class="search-box">
    <input type="text" id="searchInput" placeholder="Search" />
    <div class="toggles">
      <button class="toggle-btn" id="matchCase" title="Match Case (Alt+C)">
        <svg viewBox="0 0 16 16"><path d="M8.854 11.702h-1l-.816-2.159H3.772l-.768 2.16H2L5.086 4h.822l2.946 7.702zm-2.242-2.91L5.364 5.055l-1.26 3.737h2.508zm4.673-5.001h.723v.682h.012c.238-.46.792-.769 1.373-.769.859 0 1.393.453 1.393 1.208v4.79h-.723V5.49c0-.578-.338-.918-.937-.918-.665 0-1.118.498-1.118 1.197v3.933h-.723V3.791z"/></svg>
      </button>
      <button class="toggle-btn" id="matchWholeWord" title="Match Whole Word (Alt+W)">
        <svg viewBox="0 0 16 16"><path fill-rule="evenodd" clip-rule="evenodd" d="M0 11H1V13H15V11H16V14H15H1H0V11Z"/><path d="M6.84048 11H5.95963V10.1406H5.93814C5.555 10.7995 4.99104 11.1289 4.24625 11.1289C3.69839 11.1289 3.26871 10.9839 2.95718 10.6938C2.64924 10.4038 2.49527 10.0189 2.49527 9.53906C2.49527 8.51139 3.10041 7.91341 4.3107 7.74512L5.95963 7.51855C5.95963 6.91341 5.5903 6.61084 4.85163 6.61084C4.22476 6.61084 3.65003 6.81104 3.12741 7.21143V6.30371C3.71895 5.99577 4.36606 5.8418 5.06861 5.8418C6.24119 5.8418 6.84048 6.40592 6.84048 7.53418V11ZM5.95963 8.21631L4.63297 8.40625C4.22022 8.46224 3.9183 8.55794 3.72721 8.69336C3.53613 8.82878 3.44059 9.0485 3.44059 9.35254C3.44059 9.58073 3.52555 9.76986 3.69548 9.91992C3.869 10.0664 4.09864 10.1396 4.38444 10.1396C4.78076 10.1396 5.1048 10.0007 5.35656 9.72266C5.60832 9.44108 5.73421 9.08073 5.73421 8.64258V8.21631H5.95963Z"/><path d="M9.3475 10.2051H9.32601V11H8.44515V2.85742H9.32601V6.4668H9.3475C9.78076 5.72559 10.4146 5.35498 11.2489 5.35498C11.9264 5.35498 12.4674 5.61198 12.8708 6.12598C13.2743 6.63997 13.476 7.32389 13.476 8.17773C13.476 9.13818 13.2429 9.89616 12.7768 10.4517C12.3107 11.0073 11.6883 11.2852 10.9098 11.2852C10.2004 11.2852 9.67057 10.9254 9.3475 10.2051ZM9.32601 8.07129V8.64258C9.32601 9.09831 9.46683 9.4834 9.74847 9.79785C10.0337 10.1087 10.3988 10.2642 10.8438 10.2642C11.3559 10.2642 11.7521 10.0605 12.0319 9.65332C12.3153 9.24609 12.457 8.68376 12.457 7.96631C12.457 7.35059 12.3224 6.86914 12.0533 6.52197C11.7878 6.1748 11.4191 6.00122 10.9473 6.00122C10.4541 6.00122 10.0625 6.17969 9.77274 6.53662C9.48299 6.89355 9.33455 7.34473 9.32601 7.89014V8.07129Z"/></svg>
      </button>
      <button class="toggle-btn" id="useRegex" title="Use Regular Expression (Alt+R)">
        <svg viewBox="0 0 16 16"><path fill-rule="evenodd" clip-rule="evenodd" d="M10.012 2H11.012V4.219L12.963 2.994L13.463 3.863L11.512 5.088L13.463 6.314L12.963 7.182L11.012 5.957V8.176H10.012V5.957L8.061 7.182L7.561 6.314L9.512 5.088L7.561 3.863L8.061 2.994L10.012 4.219V2ZM2 10H6V14H2V10Z"/></svg>
      </button>
    </div>
  </div>
  <script>
    const vscode = acquireVsCodeApi();
    const searchInput = document.getElementById('searchInput');
    const matchCaseBtn = document.getElementById('matchCase');
    const matchWholeWordBtn = document.getElementById('matchWholeWord');
    const useRegexBtn = document.getElementById('useRegex');
    let matchCase = false, matchWholeWord = false, useRegex = false;
    function sendSearch() {
      vscode.postMessage({ type: 'search', query: searchInput.value, matchCase, matchWholeWord, useRegex });
    }
    searchInput.addEventListener('input', sendSearch);
    matchCaseBtn.addEventListener('click', () => { matchCase = !matchCase; matchCaseBtn.classList.toggle('active', matchCase); sendSearch(); });
    matchWholeWordBtn.addEventListener('click', () => { matchWholeWord = !matchWholeWord; matchWholeWordBtn.classList.toggle('active', matchWholeWord); sendSearch(); });
    useRegexBtn.addEventListener('click', () => { useRegex = !useRegex; useRegexBtn.classList.toggle('active', useRegex); sendSearch(); });
    searchInput.addEventListener('keydown', (e) => {
      if (e.altKey && e.key === 'c') { matchCase = !matchCase; matchCaseBtn.classList.toggle('active', matchCase); sendSearch(); e.preventDefault(); }
      if (e.altKey && e.key === 'w') { matchWholeWord = !matchWholeWord; matchWholeWordBtn.classList.toggle('active', matchWholeWord); sendSearch(); e.preventDefault(); }
      if (e.altKey && e.key === 'r') { useRegex = !useRegex; useRegexBtn.classList.toggle('active', useRegex); sendSearch(); e.preventDefault(); }
    });
  </script>
</body>
</html>`;
  }
}

type TicketFilter = "all" | "open" | "closed";

type TicketTreeItem = TicketYearItem | TicketMonthItem | TicketDayItem | TicketItem | TicketIterationItem | TicketAuthorItem | TicketCommitsItem | TicketCommitItem | TicketDateItem;

class TicketIterationItem extends vscode.TreeItem {
  constructor(public readonly iteration: TicketIteration, public readonly index: number, public readonly ticket: TicketData) {
    super(`Iteration ${index + 1}`, vscode.TreeItemCollapsibleState.Collapsed);
    this.description = iteration.prompt;
    this.tooltip = iteration.prompt;
    this.iconPath = iteration.finished ? new vscode.ThemeIcon("check") : new vscode.ThemeIcon("play");
  }
}

class TicketDateItem extends vscode.TreeItem {
  constructor(public readonly label: string, public readonly date: string) {
    super(label, vscode.TreeItemCollapsibleState.None);
    this.description = new Date(date).toLocaleString();
    this.iconPath = new vscode.ThemeIcon("calendar");
  }
}

class TicketYearItem extends vscode.TreeItem {
  constructor(public readonly year: number) {
    super(String(year), vscode.TreeItemCollapsibleState.Expanded);
    this.iconPath = new vscode.ThemeIcon("calendar");
    this.contextValue = "ticketYear";
  }
}

class TicketMonthItem extends vscode.TreeItem {
  constructor(
    public readonly year: number,
    public readonly month: number,
  ) {
    super(String(month).padStart(2, "0"), vscode.TreeItemCollapsibleState.Expanded);
    this.iconPath = new vscode.ThemeIcon("calendar");
    this.contextValue = "ticketMonth";
  }
}

class TicketDayItem extends vscode.TreeItem {
  constructor(
    public readonly year: number,
    public readonly month: number,
    public readonly day: number,
  ) {
    super(String(day).padStart(2, "0"), vscode.TreeItemCollapsibleState.Expanded);
    this.iconPath = new vscode.ThemeIcon("calendar");
    this.contextValue = "ticketDay";
  }
}

class TicketItem extends vscode.TreeItem {
  constructor(public readonly ticket: TicketData) {
    super(ticket.slug, vscode.TreeItemCollapsibleState.Collapsed);
    this.tooltip = ticket.frontmatter.summary || ticket.frontmatter.prompt;
    this.description = ticket.frontmatter.status;
    this.iconPath = new vscode.ThemeIcon(ticket.frontmatter.status === "open" ? "issue-opened" : "issue-closed");
    this.contextValue = ticket.frontmatter.status === "open" ? "ticketOpen" : "ticketClosed";
    this.command = { command: "semio.openTicket", title: "Open Ticket", arguments: [ticket] };
  }
}

class TicketAuthorItem extends vscode.TreeItem {
  constructor(
    public readonly author: string,
    public readonly ticket: TicketData,
  ) {
    super(author, vscode.TreeItemCollapsibleState.None);
    this.iconPath = new vscode.ThemeIcon("person");
    this.contextValue = "ticketAuthor";
    this.description = "author";
  }
}

class TicketCommitsItem extends vscode.TreeItem {
  constructor(public readonly commits: string[]) {
    super("commits", vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon("git-commit");
    this.contextValue = "ticketCommits";
  }
}

class TicketCommitItem extends vscode.TreeItem {
  constructor(public readonly commit: string) {
    super(commit.substring(0, 7), vscode.TreeItemCollapsibleState.None);
    this.description = commit;
    this.tooltip = commit;
    this.iconPath = new vscode.ThemeIcon("git-commit");
    this.contextValue = "ticketCommit";
  }
}

class TicketsProvider implements vscode.TreeDataProvider<TicketTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<TicketTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;
  private filter: TicketFilter = "all";
  private cachedTickets: TicketData[] = [];

  refresh(): void {
    this.cachedTickets = [];
    this._onDidChangeTreeData.fire();
  }

  toggleFilter(): void {
    const filters: TicketFilter[] = ["all", "open", "closed"];
    const currentIndex = filters.indexOf(this.filter);
    this.filter = filters[(currentIndex + 1) % filters.length];
    this.refresh();
    vscode.window.showInformationMessage(`Ticket filter: ${this.filter}`);
  }

  getFilter(): TicketFilter {
    return this.filter;
  }

  getTreeItem(element: TicketTreeItem): vscode.TreeItem {
    return element;
  }

  private matchesSearch(ticket: TicketData): boolean {
    if (!globalSearchQuery) return true;
    const searchable = [ticket.slug, ticket.frontmatter.summary || "", ticket.frontmatter.prompt || "", ticket.frontmatter.author || ""].join(" ");
    return matchesSearchText(searchable);
  }

  async getChildren(element?: TicketTreeItem): Promise<TicketTreeItem[]> {
    log("[TicketsProvider.getChildren] called, element:", element?.constructor.name ?? "root");
    log("[TicketsProvider.getChildren] cachedTickets.length:", this.cachedTickets.length);

    if (this.cachedTickets.length === 0) {
      log("[TicketsProvider.getChildren] cache empty, fetching tickets via GraphQL...");
      const tickets = await fetchTicketsViaGraphQL();
      log("[TicketsProvider.getChildren] GraphQL tickets:", tickets.length);
      this.cachedTickets = tickets.map((t) => ({
        year: t.year,
        month: t.month,
        day: t.day,
        slug: t.slug,
        folderPath: t.path.replace(/[/\\]ticket\.md$/, ""),
        filePath: t.path,
        frontmatter: {
          status: t.status.toLowerCase(),
          prompt: t.prompt,
          summary: t.summary ?? undefined,
          author: t.author?.github,
          commit: t.commit ?? undefined,
          started: t.dates.started,
          finished: t.dates.finished ?? undefined,
        },
        iterations: t.iterations?.map((it) => ({
          prompt: it.prompt,
          plan: it.plan ?? undefined,
          llm: it.llm,
          ui: it.ui,
          author: it.author,
          started: it.started,
          finished: it.finished ?? undefined,
          commit: it.commit ?? undefined,
        })),
      }));
      log("[TicketsProvider.getChildren] cachedTickets.length after fetch:", this.cachedTickets.length);
    }

    let tickets = this.cachedTickets;
    log("[TicketsProvider.getChildren] tickets before filter:", tickets.length);

    if (this.filter === "open") tickets = tickets.filter((t) => t.frontmatter.status === "open");
    else if (this.filter === "closed") tickets = tickets.filter((t) => t.frontmatter.status === "closed");

    log("[TicketsProvider.getChildren] tickets after status filter:", tickets.length);

    tickets = tickets.filter((t) => this.matchesSearch(t));

    log("[TicketsProvider.getChildren] tickets after search filter:", tickets.length);

    if (filterProvider) {
      const timeFilter = filterProvider.getTimeFilter();
      if (timeFilter.excludeYears.length > 0) {
        tickets = tickets.filter((t) => !timeFilter.excludeYears.includes(t.year));
      }
      if (timeFilter.excludeMonths.length > 0) {
        tickets = tickets.filter((t) => !timeFilter.excludeMonths.includes(t.month));
      }
      if (timeFilter.excludeDays.length > 0) {
        tickets = tickets.filter((t) => !timeFilter.excludeDays.includes(t.day));
      }
      log("[TicketsProvider.getChildren] tickets after time filter:", tickets.length);
    }

    if (!element) {
      if (tickets.length === 0) {
        log("[TicketsProvider.getChildren] no tickets, returning empty array");
        return [];
      }
      const years = [...new Set(tickets.map((t) => t.year))].sort((a, b) => b - a);
      log("[TicketsProvider.getChildren] returning", years.length, "year items");
      return years.map((year) => new TicketYearItem(year));
    }
    if (element instanceof TicketYearItem) {
      const yearTickets = tickets.filter((t) => t.year === element.year);
      const months = [...new Set(yearTickets.map((t) => t.month))].sort((a, b) => b - a);
      return months.map((month) => new TicketMonthItem(element.year, month));
    }
    if (element instanceof TicketMonthItem) {
      const monthTickets = tickets.filter((t) => t.year === element.year && t.month === element.month);
      const days = [...new Set(monthTickets.map((t) => t.day))].sort((a, b) => b - a);
      return days.map((day) => new TicketDayItem(element.year, element.month, day));
    }
    if (element instanceof TicketDayItem) {
      const dayTickets = tickets.filter((t) => t.year === element.year && t.month === element.month && t.day === element.day);
      return dayTickets.map((ticket) => new TicketItem(ticket));
    }
    if (element instanceof TicketItem) {
      const children: TicketTreeItem[] = [];
      if (element.ticket.iterations) {
        element.ticket.iterations.forEach((it, i) => {
          children.push(new TicketIterationItem(it, i, element.ticket));
        });
      } else {
        const commits: string[] = [];
        if (element.ticket.frontmatter.commit) {
          commits.push(element.ticket.frontmatter.commit);
        }
        if (element.ticket.frontmatter.author) {
          children.push(new TicketAuthorItem(element.ticket.frontmatter.author, element.ticket));
        }
        if (commits.length > 0) {
          children.push(new TicketCommitsItem(commits));
        }
      }
      return children;
    }
    if (element instanceof TicketIterationItem) {
      const children: TicketTreeItem[] = [];
      children.push(new TicketAuthorItem(element.iteration.author.github || element.iteration.author.name || "unknown", element.ticket));
      children.push(new TicketDateItem("Started", element.iteration.started));
      if (element.iteration.finished) {
        children.push(new TicketDateItem("Finished", element.iteration.finished));
      }
      if (element.iteration.commit) {
        children.push(new TicketCommitsItem([element.iteration.commit]));
      }
      return children;
    }
    if (element instanceof TicketCommitsItem) {
      return element.commits.map((commit) => new TicketCommitItem(commit));
    }
    return [];
  }
}

type PolicyTreeItem = PolicyItem | ViolationKindGroupItem | ViolationKindItem;

class PolicyItem extends vscode.TreeItem {
  constructor(
    public readonly policy: PolicyData,
    public readonly lineNumber?: number,
  ) {
    super(`${policy.name} - ${policy.description}`, vscode.TreeItemCollapsibleState.Collapsed);
    this.tooltip = `${policy.id}\n${policy.name}\n${policy.description}`;
    this.iconPath = new vscode.ThemeIcon("shield");
    this.contextValue = "policy";
    this.command = { command: "semio.openPolicy", title: "Open Policy", arguments: [policy, lineNumber] };
  }
}

class ViolationKindGroupItem extends vscode.TreeItem {
  constructor(
    public readonly groupPath: string,
    public readonly policyId: string,
    public readonly children: string[],
  ) {
    const segments = groupPath.split(":");
    const name = segments[segments.length - 1];
    super(name, vscode.TreeItemCollapsibleState.Collapsed);
    this.tooltip = `Violation group: ${groupPath}`;
    this.iconPath = new vscode.ThemeIcon("folder");
    this.contextValue = "violationKindGroup";
  }
}

class ViolationKindItem extends vscode.TreeItem {
  constructor(
    public readonly kind: string,
    public readonly policyId: string,
  ) {
    const segments = kind.split(":");
    const name = segments[segments.length - 1];
    super(name, vscode.TreeItemCollapsibleState.None);
    this.description = kind;
    this.tooltip = `Violation kind: ${kind}`;
    this.iconPath = new vscode.ThemeIcon("warning");
    this.contextValue = "violationKind";
  }
}

class PoliciesProvider implements vscode.TreeDataProvider<PolicyTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<PolicyTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;
  private cachedPolicies: PolicyData[] = [];
  private cachedViolationKinds = new Map<string, string[]>();

  refresh(): void {
    this.cachedPolicies = [];
    this.cachedViolationKinds.clear();
    this._onDidChangeTreeData.fire();
  }

  private matchesPolicySearch(policy: PolicyData): boolean {
    const searchable = [policy.id, policy.name, policy.description].join(" ");
    return matchesSearchText(searchable);
  }

  private matchesViolationKindSearch(kind: string): boolean {
    return matchesSearchText(kind);
  }

  private async getViolationKinds(policyId: string): Promise<string[]> {
    if (!this.cachedViolationKinds.has(policyId)) {
      const result = await runRepoCommandJson<ToolResult<string[]>>(`policy violation list ${policyId}`);
      this.cachedViolationKinds.set(policyId, result?.data ?? []);
    }
    return this.cachedViolationKinds.get(policyId) ?? [];
  }

  private buildViolationTree(kinds: string[], policyId: string, prefix: string): PolicyTreeItem[] {
    const groups = new Map<string, string[]>();
    const leafKinds: string[] = [];
    for (const kind of kinds) {
      if (!kind.startsWith(prefix)) continue;
      const rest = prefix ? kind.slice(prefix.length + 1) : kind;
      const colonIndex = rest.indexOf(":");
      if (colonIndex === -1) {
        leafKinds.push(kind);
      } else {
        const groupName = rest.slice(0, colonIndex);
        const groupPath = prefix ? `${prefix}:${groupName}` : groupName;
        if (!groups.has(groupPath)) groups.set(groupPath, []);
        groups.get(groupPath)!.push(kind);
      }
    }
    const items: PolicyTreeItem[] = [];
    for (const [groupPath, children] of groups) {
      items.push(new ViolationKindGroupItem(groupPath, policyId, children));
    }
    for (const kind of leafKinds) {
      items.push(new ViolationKindItem(kind, policyId));
    }
    return items;
  }

  getTreeItem(element: PolicyTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: PolicyTreeItem): Promise<PolicyTreeItem[]> {
    if (!element) {
      if (this.cachedPolicies.length === 0) {
        log("[PoliciesProvider.getChildren] cache empty, fetching policies via GraphQL...");
        const policies = await fetchPoliciesViaGraphQL();
        log("[PoliciesProvider.getChildren] GraphQL policies:", policies.length);
        this.cachedPolicies = policies.map((p) => ({
          id: p.id,
          name: p.name,
          description: p.description ?? "",
        }));
        for (const policy of policies) {
          this.cachedViolationKinds.set(policy.id, policy.violationKinds.map((vk) => vk.id));
        }
      }

      let policies = this.cachedPolicies;

      if (filterProvider) {
        const policyFilter = filterProvider.getPolicyFilter();
        if (policyFilter.excludePolicies.length > 0) {
          policies = policies.filter((p) => !policyFilter.excludePolicies.includes(p.id));
        }
      }

      if (!globalSearchQuery) {
        return policies.map((policy) => new PolicyItem(policy));
      }
      const matchingPolicies: PolicyItem[] = [];
      for (const policy of policies) {
        if (this.matchesPolicySearch(policy)) {
          matchingPolicies.push(new PolicyItem(policy));
        } else {
          const kinds = await this.getViolationKinds(policy.id);
          if (kinds.some((k) => this.matchesViolationKindSearch(k))) {
            matchingPolicies.push(new PolicyItem(policy));
          }
        }
      }
      return matchingPolicies;
    }
    if (element instanceof PolicyItem) {
      let kinds = await this.getViolationKinds(element.policy.id);

      if (filterProvider) {
        const violationFilter = filterProvider.getViolationFilter();
        if (violationFilter.excludeViolations.length > 0) {
          kinds = kinds.filter((k) => !violationFilter.excludeViolations.some((v) => k.startsWith(v)));
        }
      }

      const filtered = globalSearchQuery ? kinds.filter((k) => this.matchesViolationKindSearch(k) || this.matchesPolicySearch(element.policy)) : kinds;
      return this.buildViolationTree(filtered, element.policy.id, "");
    }
    if (element instanceof ViolationKindGroupItem) {
      let children = element.children;

      if (filterProvider) {
        const violationFilter = filterProvider.getViolationFilter();
        if (violationFilter.excludeViolations.length > 0) {
          children = children.filter((k) => !violationFilter.excludeViolations.some((v) => k.startsWith(v)));
        }
      }

      return this.buildViolationTree(children, element.policyId, element.groupPath);
    }
    return [];
  }
}

interface ContributorData {
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

interface ContributorBundleData {
  name: string;
  lines: ContributorLineMetrics;
  folders: ContributorFolderData[];
}
interface ContributorFolderData {
  name: string;
  lines: ContributorLineMetrics;
  files: ContributorFileData[];
}

interface ContributorFileData {
  name: string;
  lines: ContributorLineMetrics;
  sections: ContributorSectionData[];
}

interface ContributorSectionData {
  name: string;
  lines: ContributorLineMetrics;
  definitions: ContributorDefinitionData[];
}

interface ContributorDefinitionData {
  name: string;
  lines: ContributorLineMetrics;
}

interface ContributorTicketData {
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

interface ContributorCommitData {
  title: string;
  sha: string;
}

interface ContributorLineMetrics {
  added: number;
  removed: number;
}

type ContributorTreeItem =
  | ContributorItem
  | ContributorEmailsItem
  | ContributorEmailItem
  | ContributorLinksItem
  | ContributorLinkItem
  | ContributorContributionsItem
  | ContributorProjectsItem
  | ContributorBundleItem
  | ContributorFolderItem
  | ContributorFileItem
  | ContributorSectionItem
  | ContributorDefinitionItem
  | ContributorTicketsItem
  | ContributorTicketYearItem
  | ContributorTicketMonthItem
  | ContributorTicketDayItem
  | ContributorTicketItem
  | ContributorCommitsItem
  | ContributorCommitItem;

class ContributorItem extends vscode.TreeItem {
  constructor(
    public readonly contributor: ContributorData,
    avatarPath?: string,
  ) {
    const displayName = contributor.name ? `${contributor.name} - ${contributor.github}` : contributor.github;
    super(displayName, vscode.TreeItemCollapsibleState.Collapsed);
    this.tooltip = `${contributor.github}${contributor.contributions?.tickets ? `\nTickets: ${contributor.contributions.tickets.length}` : ""}${contributor.contributions?.bundles ? `\nProjects: ${contributor.contributions.bundles.map((b) => b.name).join(", ")}` : ""}`;
    if (avatarPath && fs.existsSync(avatarPath)) {
      this.iconPath = vscode.Uri.file(avatarPath);
    } else {
      this.iconPath = new vscode.ThemeIcon("person");
    }
    this.contextValue = "contributor";
    this.command = { command: "semio.openContributor", title: "Open Contributor", arguments: [contributor] };
  }
}

class ContributorEmailsItem extends vscode.TreeItem {
  constructor(public readonly contributor: ContributorData) {
    super("emails", vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon("mail");
    this.contextValue = "contributorEmails";
  }
}

class ContributorEmailItem extends vscode.TreeItem {
  constructor(public readonly email: string) {
    super(email, vscode.TreeItemCollapsibleState.None);
    this.iconPath = new vscode.ThemeIcon("mail");
    this.contextValue = "contributorEmail";
    this.command = { command: "vscode.open", title: "MailTo", arguments: [vscode.Uri.parse(`mailto:${email}`)] };
  }
}

class ContributorLinksItem extends vscode.TreeItem {
  constructor(public readonly contributor: ContributorData) {
    super("links", vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon("link");
    this.contextValue = "contributorLinks";
  }
}

class ContributorLinkItem extends vscode.TreeItem {
  constructor(
    public readonly kind: string,
    public readonly url: string,
  ) {
    super(kind, vscode.TreeItemCollapsibleState.None);
    this.description = url;
    this.iconPath = new vscode.ThemeIcon("link-external");
    this.contextValue = "contributorLink";
    this.command = { command: "vscode.open", title: "Open Link", arguments: [vscode.Uri.parse(url)] };
  }
}

class ContributorContributionsItem extends vscode.TreeItem {
  constructor(public readonly contributor: ContributorData) {
    super("contributions", vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon("git-commit");
    this.contextValue = "contributorContributions";
  }
}

class ContributorProjectsItem extends vscode.TreeItem {
  constructor(public readonly contributor: ContributorData, count: number) {
    super("bundles", vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon("package");
    this.contextValue = "contributorProjects";
    this.description = String(count);
  }
}

class ContributorBundleItem extends vscode.TreeItem {
  constructor(public readonly bundle: ContributorBundleData) {
    super(bundle.name, vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon("package");
    this.contextValue = "contributorBundle";
    this.description = `(${bundle.folders.length}) +${bundle.lines.added} -${bundle.lines.removed}`;
  }
}

class ContributorFolderItem extends vscode.TreeItem {
  constructor(public readonly folder: ContributorFolderData) {
    super(folder.name, vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon("folder");
    this.contextValue = "contributorFolder";
    this.description = `(${folder.files.length}) +${folder.lines.added} -${folder.lines.removed}`;
  }
}

class ContributorFileItem extends vscode.TreeItem {
  constructor(public readonly file: ContributorFileData) {
    super(file.name, vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon("file");
    this.contextValue = "contributorFile";
    this.description = `(${file.sections.length}) +${file.lines.added} -${file.lines.removed}`;
  }
}

class ContributorSectionItem extends vscode.TreeItem {
  constructor(public readonly section: ContributorSectionData) {
    super(section.name, vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon("symbol-class");
    this.contextValue = "contributorSection";
    this.description = `(${section.definitions.length}) +${section.lines.added} -${section.lines.removed}`;
  }
}

class ContributorDefinitionItem extends vscode.TreeItem {
  constructor(public readonly definition: ContributorDefinitionData) {
    super(definition.name, vscode.TreeItemCollapsibleState.None);
    this.iconPath = new vscode.ThemeIcon("symbol-method");
    this.contextValue = "contributorDefinition";
    this.description = `+${definition.lines.added} -${definition.lines.removed}`;
  }
}

class ContributorTicketsItem extends vscode.TreeItem {
  constructor(public readonly contributor: ContributorData, count: number) {
    super("tickets", vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon("issue-opened");
    this.contextValue = "contributorTickets";
    this.description = String(count);
  }
}

class ContributorTicketYearItem extends vscode.TreeItem {
  constructor(public readonly year: number, public readonly tickets: ContributorTicketData[]) {
    super(String(year), vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon("calendar");
    this.contextValue = "contributorTicketYear";
  }
}

class ContributorTicketMonthItem extends vscode.TreeItem {
  constructor(public readonly year: number, public readonly month: number, public readonly tickets: ContributorTicketData[]) {
    super(String(month).padStart(2, "0"), vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon("calendar");
    this.contextValue = "contributorTicketMonth";
  }
}

class ContributorTicketDayItem extends vscode.TreeItem {
  constructor(
    public readonly year: number,
    public readonly month: number,
    public readonly day: number,
    public readonly tickets: ContributorTicketData[],
  ) {
    super(String(day).padStart(2, "0"), vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon("calendar");
    this.contextValue = "contributorTicketDay";
  }
}

class ContributorTicketItem extends vscode.TreeItem {
  constructor(public readonly ticket: ContributorTicketData) {
    super(ticket.slug, vscode.TreeItemCollapsibleState.None);
    this.iconPath = new vscode.ThemeIcon(ticket.status === "open" ? "issue-opened" : "issue-closed");
    this.contextValue = ticket.status === "open" ? "ticketOpen" : "ticketClosed";
    this.command = { command: "semio.openTicket", title: "Open Ticket", arguments: [ticket] };
  }
}

class ContributorCommitsItem extends vscode.TreeItem {
  constructor(public readonly contributor: ContributorData, count: number) {
    super("commits", vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon("git-commit");
    this.contextValue = "contributorCommits";
    this.description = String(count);
  }
}

class ContributorCommitItem extends vscode.TreeItem {
  constructor(
    public readonly title: string,
    public readonly sha: string,
  ) {
    super(`${title} - ${sha.substring(0, 7)}`, vscode.TreeItemCollapsibleState.None);
    this.tooltip = `${title}\n${sha}`;
    this.description = sha;
    this.iconPath = new vscode.ThemeIcon("git-commit");
    this.contextValue = "contributorCommit";
    this.command = { command: "git.showCommit", title: "Open Commit", arguments: [sha] };
  }
}

class TodosProvider implements vscode.TreeDataProvider<TodoTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<TodoTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;
  private cachedTodos: Todo[] = [];

  refresh(): void {
    this.cachedTodos = [];
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(element: TodoTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: TodoTreeItem): Promise<TodoTreeItem[]> {
    if (element) return [];

    if (this.cachedTodos.length === 0) {
      this.cachedTodos = await fetchTodosViaGraphQL();
    }

    return this.cachedTodos.map((t) => new TodoTreeItem(t));
  }
}

class TodoTreeItem extends vscode.TreeItem {
  constructor(public readonly todo: Todo) {
    super(todo.text, vscode.TreeItemCollapsibleState.None);
    this.description = todo.location ? `${todo.location.file}:${todo.location.line}` : "";
    this.contextValue = "todo";
    this.iconPath = new vscode.ThemeIcon("checklist");
    if (todo.location) {
      this.command = {
        command: "vscode.open",
        title: "Open Todo",
        arguments: [vscode.Uri.file(path.join(getWorkspaceRoot() || "", todo.location.file)), { selection: new vscode.Range(todo.location.line - 1, (todo.location.column || 1) - 1, todo.location.line - 1, (todo.location.column || 1) - 1) }]
      };
    }
  }
}

class ContributorsProvider implements vscode.TreeDataProvider<ContributorTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<ContributorTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;
  private cachedContributors: ContributorData[] = [];

  refresh(): void {
    this.cachedContributors = [];
    cachedProjects = null;
    this._onDidChangeTreeData.fire();
  }

  private matchesSearch(contributor: ContributorData): boolean {
    if (!globalSearchQuery) return true;
    const searchable = [contributor.github, contributor.name || "", ...(contributor.emails || [])].join(" ");
    return matchesSearchText(searchable);
  }

  getTreeItem(element: ContributorTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: ContributorTreeItem): Promise<ContributorTreeItem[]> {
    if (!element) {
      if (this.cachedContributors.length === 0) {
        log("[ContributorsProvider.getChildren] cache empty, fetching contributors via GraphQL...");
        const contributors = await fetchContributorsViaGraphQL();
        log("[ContributorsProvider.getChildren] GraphQL contributors:", contributors.length);
        this.cachedContributors = contributors.map((c) => ({
          github: c.github,
          name: c.name ?? undefined,
          emails: c.emails,
          links: c.links?.reduce((acc: Record<string, string>, l) => ({ ...acc, [l.name]: l.url }), {}),
          contributions: {
            commits: (c.contributions.commits || []).map((commit) => ({
              id: commit.id,
              sha: commit.sha,
              title: commit.title,
            })),
            tickets: (c.contributions.tickets || []).map((ticket) => ({
              slug: ticket.slug,
              year: ticket.year,
              month: ticket.month,
              day: ticket.day,
              title: ticket.title ?? "Untitled",
              summary: ticket.summary ?? "",
              status: ticket.status.toLowerCase(),
            })),
            bundles: (c.contributions.bundles || []).map((bundle) => ({
              name: bundle.name,
              lines: bundle.lines,
              folders: (bundle.folders || []).map((folder) => ({
                name: folder.name,
                lines: folder.lines,
                files: (folder.files || []).map((file) => ({
                  name: file.name,
                  lines: file.lines,
                  sections: (file.sections || []).map((section) => ({
                    name: section.name,
                    lines: section.lines,
                    definitions: (section.definitions || []).map((definition) => ({
                      name: definition.name,
                      lines: definition.lines,
                    })),
                  })),
                })),
              })),
            })),
          },
        }));
      }
      const root = getWorkspaceRoot();
      let contributors = this.cachedContributors.filter((c) => this.matchesSearch(c));

      if (filterProvider) {
        const contributorFilter = filterProvider.getContributorFilter();
        if (contributorFilter.excludeContributors.length > 0) {
          contributors = contributors.filter((c) => !contributorFilter.excludeContributors.includes(c.name || c.github));
        }
      }

      return contributors.map((contributor) => {
        const avatarPath = root ? path.join(root, ".semio-repo", "contributors", contributor.github, "avatar-round-90x90.png") : undefined;
        return new ContributorItem(contributor, avatarPath);
      });
    }
    if (element instanceof ContributorItem) {
      const children: ContributorTreeItem[] = [];
      const c = element.contributor;
      if (c.emails && c.emails.length > 0) children.push(new ContributorEmailsItem(c));
      if (c.links && Object.keys(c.links).length > 0) children.push(new ContributorLinksItem(c));

      // Flattened contributions
      if (c.contributions?.commits?.length) children.push(new ContributorCommitsItem(c, c.contributions.commits.length));
      if (c.contributions?.tickets?.length) children.push(new ContributorTicketsItem(c, c.contributions.tickets.length));
      if (c.contributions?.bundles?.length) children.push(new ContributorProjectsItem(c, c.contributions.bundles.length));

      return children;
    }
    if (element instanceof ContributorEmailsItem) {
      return (element.contributor.emails || []).map((email) => new ContributorEmailItem(email));
    }
    if (element instanceof ContributorLinksItem) {
      return Object.entries(element.contributor.links || {}).map(([kind, url]) => new ContributorLinkItem(kind, url));
    }

    if (element instanceof ContributorProjectsItem) {
      return (element.contributor.contributions?.bundles || []).map((p) => new ContributorBundleItem(p));
    }
    if (element instanceof ContributorBundleItem) {
      return (element.bundle.folders || []).map((f) => new ContributorFolderItem(f));
    }
    if (element instanceof ContributorFolderItem) {
      return (element.folder.files || []).map((f) => new ContributorFileItem(f));
    }
    if (element instanceof ContributorFileItem) {
      return (element.file.sections || []).map((s) => new ContributorSectionItem(s));
    }
    if (element instanceof ContributorSectionItem) {
      return (element.section.definitions || []).map((d) => new ContributorDefinitionItem(d));
    }
    if (element instanceof ContributorCommitsItem) {
      return (element.contributor.contributions?.commits || []).map((c) => new ContributorCommitItem(c.title, c.sha));
    }
    if (element instanceof ContributorTicketsItem) {
      const tickets = element.contributor.contributions?.tickets || [];
      const years = [...new Set(tickets.map((t) => t.year))].sort((a, b) => b - a);
      return years.map((year) => new ContributorTicketYearItem(year, tickets.filter((t) => t.year === year)));
    }
    if (element instanceof ContributorTicketYearItem) {
      const months = [...new Set((element.tickets || []).map((t) => t.month))].sort((a, b) => b - a);
      return months.map((month) => new ContributorTicketMonthItem(element.year, month, (element.tickets || []).filter((t) => t.month === month)));
    }
    if (element instanceof ContributorTicketMonthItem) {
      const days = [...new Set((element.tickets || []).map((t) => t.day))].sort((a, b) => b - a);
      return days.map((day) => new ContributorTicketDayItem(element.year, element.month, day, (element.tickets || []).filter((t) => t.day === day)));
    }
    if (element instanceof ContributorTicketDayItem) {
      return (element.tickets || [])
        .filter((t) => t.year === element.year && t.month === element.month && t.day === element.day)
        .sort((a, b) => a.slug.localeCompare(b.slug))
        .map((t) => new ContributorTicketItem(t));
    }
    return [];
  }
}

type SectionTreeItem = SectionItem | SectionDefinitionItem | SectionStatusItem;

class SectionStatusItem extends vscode.TreeItem {
  constructor(label: string) {
    super(label, vscode.TreeItemCollapsibleState.None);
    this.contextValue = "sectionStatus";
  }
}

class SectionItem extends vscode.TreeItem {
  constructor(
    public readonly section: SectionInfo,
    public readonly sectionPath: string,
    public readonly hasChildren: boolean,
    public readonly filePath: string,
    public readonly fileUri: string,
  ) {
    super(section.name, hasChildren ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None);
    this.contextValue = "section";
    this.tooltip = section.name;
    this.iconPath = new vscode.ThemeIcon("symbol-namespace");
    this.command = { command: "semio.sectionOpen", title: "Open Section", arguments: [this] };
  }
}

class SectionDefinitionItem extends vscode.TreeItem {
  constructor(
    public readonly definition: DefinitionInfo,
    public readonly fileUri: string,
  ) {
    const name = definition.name;
    super(name, vscode.TreeItemCollapsibleState.None);
    this.iconPath = new vscode.ThemeIcon("symbol-function");
    this.contextValue = "sectionDefinition";
    this.tooltip = definition.name;
    this.command = { command: "semio.navigateToDefinition", title: "Navigate to Definition", arguments: [fileUri, definition.startLine] };
  }
}

class SectionsProvider implements vscode.TreeDataProvider<SectionTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<SectionTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;
  private cachedSections: SectionInfo[] = [];
  private cachedDefinitions: DefinitionInfo[] = [];
  private cachedFilePath: string = "";
  private cachedFileUri: string = "";

  refresh(): void {
    this.cachedSections = [];
    this.cachedDefinitions = [];
    this.cachedFilePath = "";
    this.cachedFileUri = "";
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(element: SectionTreeItem): vscode.TreeItem {
    return element;
  }

  private isDefinitionInSection(def: DefinitionInfo, section: SectionInfo): boolean {
    return def.startLine >= section.startLine && def.startLine <= section.endLine;
  }

  private getDefinitionsForSection(section: SectionInfo): DefinitionInfo[] {
    return this.cachedDefinitions.filter((d) => this.isDefinitionInSection(d, section));
  }

  private sectionHasChildren(section: SectionInfo): boolean {
    return section.children.length > 0 || this.getDefinitionsForSection(section).length > 0;
  }

  private buildFileChildren(): SectionTreeItem[] {
    const items: SectionTreeItem[] = [];

    // Root sections
    for (const section of this.cachedSections) {
      items.push(new SectionItem(section, section.name, this.sectionHasChildren(section), this.cachedFilePath, this.cachedFileUri));
    }

    // File-level definitions (not inside any root section)
    const fileDefs = this.cachedDefinitions.filter((d) => {
      return !this.cachedSections.some((s) => this.isDefinitionInSection(d, s));
    });
    for (const def of fileDefs) {
      items.push(new SectionDefinitionItem(def, this.cachedFileUri));
    }

    return this.sortItemsByLine(items);
  }

  private buildSectionChildren(section: SectionInfo, parentPath: string): SectionTreeItem[] {
    const items: SectionTreeItem[] = [];

    // Child sections
    for (const child of section.children) {
      const childPath = `${parentPath}/${child.name}`;
      items.push(new SectionItem(child, childPath, this.sectionHasChildren(child), this.cachedFilePath, this.cachedFileUri));
    }

    // Definitions directly in this section but NOT in any child section
    const allDefs = this.getDefinitionsForSection(section);
    const directDefs = allDefs.filter((d) => {
      return !section.children.some((child) => this.isDefinitionInSection(d, child));
    });
    for (const def of directDefs) {
      items.push(new SectionDefinitionItem(def, this.cachedFileUri));
    }

    return this.sortItemsByLine(items);
  }

  private sortItemsByLine(items: SectionTreeItem[]): SectionTreeItem[] {
    return items.sort((a, b) => {
      const lineA = this.getItemStartLine(a);
      const lineB = this.getItemStartLine(b);
      return lineA - lineB;
    });
  }

  private getItemStartLine(item: SectionTreeItem): number {
    if (item instanceof SectionItem) return item.section.startLine || 0;
    if (item instanceof SectionDefinitionItem) return item.definition.startLine || 0;
    return 0;
  }

  async getChildren(element?: SectionTreeItem): Promise<SectionTreeItem[]> {
    log("[SectionsProvider.getChildren] called, element:", element?.constructor.name ?? "root");

    if (element instanceof SectionItem) {
      log("[SectionsProvider.getChildren] returning children of section:", element.sectionPath);
      return this.buildSectionChildren(element.section, element.sectionPath);
    }

    if (element instanceof SectionDefinitionItem || element instanceof SectionStatusItem) {
      return [];
    }

    const editor = vscode.window.activeTextEditor;
    if (!editor) {
      log("[SectionsProvider.getChildren] no active editor");
      return [new SectionStatusItem(getUiString("sectionsNoActiveFile"))];
    }
    if (!hasRepoAccess()) {
      log("[SectionsProvider.getChildren] no repo access");
      return [];
    }

    const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
    const fileUri = editor.document.uri.toString();
    log("[SectionsProvider.getChildren] fetching sections and definitions for:", relativePath);

    // Fetch sections
    const sections = await getSectionListForFile(relativePath);
    log("[SectionsProvider.getChildren] got sections:", sections.length);

    // Fetch definitions
    let definitions: DefinitionInfo[] = [];
    try {
      const defResult = await runRepoCommandJson<ToolResult<DefinitionInfo[]>>(`definition list --file "${relativePath}"`);
      definitions = extractDefinitions(defResult);
      log("[SectionsProvider.getChildren] got definitions:", definitions.length);
    } catch (e) {
      logError("[SectionsProvider.getChildren] failed to fetch definitions", e);
    }

    if (sections.length === 0 && definitions.length === 0) {
      log("[SectionsProvider.getChildren] returning empty status");
      return [new SectionStatusItem(getUiString("sectionsEmpty"))];
    }

    // Cache for child lookups
    this.cachedSections = sections;
    this.cachedDefinitions = definitions;
    this.cachedFilePath = relativePath;
    this.cachedFileUri = fileUri;

    return this.buildFileChildren();
  }
}

class SectionsDragAndDropController implements vscode.TreeDragAndDropController<SectionTreeItem> {
  readonly dragMimeTypes = ["application/vnd.semio.section"];
  readonly dropMimeTypes = ["application/vnd.semio.section"];

  handleDrag(source: readonly SectionTreeItem[], dataTransfer: vscode.DataTransfer): void {
    if (source.length === 0) return;
    const items = source.filter((s): s is SectionItem => s instanceof SectionItem);
    if (items.length === 0) return;
    dataTransfer.set("application/vnd.semio.section", new vscode.DataTransferItem(JSON.stringify(items.map((item) => ({ path: item.sectionPath, name: item.section.name })))));
  }

  async handleDrop(target: SectionTreeItem | undefined, dataTransfer: vscode.DataTransfer): Promise<void> {
    if (!target || !(target instanceof SectionItem)) return;
    if (!hasRepoAccess()) return;
    const editor = vscode.window.activeTextEditor;
    if (!editor) return;
    const item = dataTransfer.get("application/vnd.semio.section");
    if (!item) return;
    const raw = await item.asString();
    const parsed = JSON.parse(raw) as { path: string; name: string }[];
    if (!Array.isArray(parsed) || parsed.length === 0) return;
    const sourcePath = parsed[0].path;
    const targetPath = `${target.sectionPath}/${parsed[0].name}`;
    if (sourcePath === targetPath) return;
    const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
    runRepoCommand(`section move ${relativePath} ${sourcePath} ${targetPath}`);
    sectionsProvider.refresh();
  }
}

type CodebaseTreeItem =
  | CodebaseRepoItem
  | CodebaseBundleItem
  | CodebaseFolderItem
  | CodebaseFileItem
  | CodebaseSectionItem
  | CodebaseDefinitionItem;

type FileKind = "code" | "script" | "config" | "test" | "docs" | "resource" | "license";
type BundleKind = "library" | "schema" | "binary" | "ui" | "site" | "assets";
type FolderKind = "organization" | "required";
type DefinitionKind = "implementation" | "interface" | "constant";

interface CodebaseFilter {
  code: boolean;
  script: boolean;
  config: boolean;
  test: boolean;
  docs: boolean;
  resource: boolean;
  license: boolean;
}

interface BundleFilter {
  library: boolean;
  schema: boolean;
  binary: boolean;
  ui: boolean;
  site: boolean;
  assets: boolean;
}

interface FolderFilter {
  organization: boolean;
  required: boolean;
}

interface DefinitionFilter {
  [key: string]: boolean;
  implementation: boolean;
  interface: boolean;
  constant: boolean;
}

const DEFAULT_FILTER: CodebaseFilter = {
  code: true,
  script: true,
  config: true,
  test: true,
  docs: true,
  resource: true,
  license: true,
};

const DEFAULT_BUNDLE_FILTER: BundleFilter = {
  library: true,
  schema: true,
  binary: true,
  ui: true,
  site: true,
  assets: true,
};

const DEFAULT_FOLDER_FILTER: FolderFilter = {
  organization: true,
  required: true,
};

const DEFAULT_DEFINITION_FILTER: DefinitionFilter = {
  implementation: true,
  interface: true,
  constant: true,
};

const FILTER_EXTENSIONS: Record<FileKind, Set<string>> = {
  code: new Set(["ts", "js", "py", "cs", "go", "rs", "tsx", "jsx", "c", "cpp", "h", "hpp", "java", "kt", "swift", "php", "rb", "pl"]),
  script: new Set(["sh", "ps1", "bat", "cmd", "bash", "zsh"]),
  config: new Set([
    "json",
    "yaml",
    "yml",
    "toml",
    "xml",
    "ini",
    "env",
    "conf",
    "config",
    "properties",
    "gitignore",
    "dockerignore",
    "editorconfig",
    "prettierrc",
    "eslintrc",
  ]),
  test: new Set([]),
  docs: new Set(["md", "txt", "rst", "adoc", "html", "css", "scss", "less"]),
  resource: new Set([
    "png",
    "jpg",
    "jpeg",
    "gif",
    "svg",
    "ico",
    "webp",
    "mp4",
    "webm",
    "mp3",
    "wav",
    "ogg",
    "glb",
    "gltf",
    "fbx",
    "obj",
    "stl",
    "ply",
    "dae",
    "3ds",
    "3mf",
    "usdz",
    "vrm",
    "ifc",
    "pdf",
    "zip",
    "tar",
    "gz",
  ]),
  license: new Set([]),
};



interface TimeFilter {
  excludeYears: number[];
  includeYears: number[];
  excludeMonths: number[];
  includeMonths: number[];
  excludeDays: number[];
  includeDays: number[];
}

interface ContributorFilter {
  excludeContributors: string[];
  includeContributors: string[];
}

interface PolicyFilter {
  excludePolicies: string[];
  includePolicies: string[];
}

interface ViolationFilter {
  excludeViolations: string[];
  includeViolations: string[];
}

interface SectionFilter {
  none: boolean;
  all: boolean;
}

const DEFAULT_TIME_FILTER: TimeFilter = {
  excludeYears: [],
  includeYears: [],
  excludeMonths: [],
  includeMonths: [],
  excludeDays: [],
  includeDays: [],
};

const DEFAULT_CONTRIBUTOR_FILTER: ContributorFilter = {
  excludeContributors: [],
  includeContributors: [],
};

const DEFAULT_POLICY_FILTER: PolicyFilter = {
  excludePolicies: [],
  includePolicies: [],
};

const DEFAULT_VIOLATION_FILTER: ViolationFilter = {
  excludeViolations: [],
  includeViolations: [],
};

const DEFAULT_SECTION_FILTER: SectionFilter = {
  none: false,
  all: true,
};

export type FilterItemType = "root" | "search" | "bundle" | "folder" | "section" | "definition" | "time" | "contributors" | "policy" | "violation" | "timeYear" | "timeMonth" | "timeDay" | "contributor" | "action";

export class FilterItem extends vscode.TreeItem {
  constructor(
    public readonly label: string,
    public readonly collapsibleState: vscode.TreeItemCollapsibleState,
    public readonly itemType: FilterItemType,
    public readonly timeValue?: number,
    public readonly secondaryTimeValue?: number,
    public readonly tertiaryTimeValue?: number,
    public readonly contributorValue?: string,
    public readonly policyValue?: string,
    public readonly violationValue?: string
  ) {
    super(label, collapsibleState);
    this.contextValue = "filterItem";

    if (itemType === "search") this.iconPath = new vscode.ThemeIcon("search");
    else if (itemType === "bundle") this.iconPath = new vscode.ThemeIcon("package");
    else if (itemType === "folder") this.iconPath = new vscode.ThemeIcon("folder");
    else if (itemType === "section") this.iconPath = new vscode.ThemeIcon("symbol-class");
    else if (itemType === "definition") this.iconPath = new vscode.ThemeIcon("symbol-method");
    else if (itemType === "time") this.iconPath = new vscode.ThemeIcon("calendar");
    else if (itemType === "contributors") this.iconPath = new vscode.ThemeIcon("organization");
    else if (itemType === "policy") this.iconPath = new vscode.ThemeIcon("shield");
    else if (itemType === "violation") this.iconPath = new vscode.ThemeIcon("warning");
  }
}

export class FilterProvider implements vscode.TreeDataProvider<FilterItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<FilterItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  public globalSearchQuery: string = "";
  public globalMatchCase: boolean = false;
  public globalMatchWholeWord: boolean = false;
  public globalUseRegex: boolean = false;

  public availableYears: number[] = [];
  public availableMonths: number[] = [];
  public availableDays: number[] = [];
  public availableContributors: string[] = [];
  public availablePolicies: string[] = [];
  public availableViolations: string[] = [];

  private filters: CodebaseFilter = { ...DEFAULT_FILTER };
  private bundleFilters: BundleFilter = { ...DEFAULT_BUNDLE_FILTER };
  private folderFilters: FolderFilter = { ...DEFAULT_FOLDER_FILTER };
  private definitionFilters: DefinitionFilter = { ...DEFAULT_DEFINITION_FILTER };
  private timeFilter: TimeFilter = { ...DEFAULT_TIME_FILTER };
  private contributorFilter: ContributorFilter = { ...DEFAULT_CONTRIBUTOR_FILTER };
  private policyFilter: PolicyFilter = { ...DEFAULT_POLICY_FILTER };
  private violationFilter: ViolationFilter = { ...DEFAULT_VIOLATION_FILTER };
  private sectionFilter: SectionFilter = { ...DEFAULT_SECTION_FILTER };

  constructor(private readonly workspaceRoot: string) { }

  private refreshAllViews(): void {
    if (codebaseProvider) codebaseProvider.refresh();
    if (ticketsProvider) ticketsProvider.refresh();
    if (contributorsProvider) contributorsProvider.refresh();
    if (policiesProvider) policiesProvider.refresh();
    if (commandsProvider) commandsProvider.refresh();
    if (sectionsProvider) sectionsProvider.refresh();
  }

  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  setSearchQuery(query: string, matchCase: boolean = false, matchWholeWord: boolean = false, useRegex: boolean = false): void {
    this.globalSearchQuery = query;
    this.globalMatchCase = matchCase;
    this.globalMatchWholeWord = matchWholeWord;
    this.globalUseRegex = useRegex;
    this.refresh();
    this.refreshAllViews();
  }

  toggle(kind: keyof CodebaseFilter): void {
    this.filters[kind] = !this.filters[kind];
    this.refresh();
    this.refreshAllViews();
  }

  toggleBundleKind(kind: keyof BundleFilter): void {
    this.bundleFilters[kind] = !this.bundleFilters[kind];
    this.refresh();
    this.refreshAllViews();
  }

  toggleFolderKind(kind: keyof FolderFilter): void {
    this.folderFilters[kind] = !this.folderFilters[kind];
    this.refresh();
    this.refreshAllViews();
  }

  toggleDefinitionKind(kind: keyof DefinitionFilter): void {
    this.definitionFilters[kind] = !this.definitionFilters[kind];
    this.refresh();
    this.refreshAllViews();
  }

  toggleSectionFilter(mode: "none" | "all"): void {
    if (mode === "none") {
      this.sectionFilter.none = !this.sectionFilter.none;
      if (this.sectionFilter.none) this.sectionFilter.all = false;
    } else {
      this.sectionFilter.all = !this.sectionFilter.all;
      if (this.sectionFilter.all) this.sectionFilter.none = false;
    }
    this.refresh();
    this.refreshAllViews();
  }

  toggleYear(year: number): void {
    const idx = this.timeFilter.excludeYears.indexOf(year);
    if (idx >= 0) this.timeFilter.excludeYears.splice(idx, 1);
    else this.timeFilter.excludeYears.push(year);
    this.refresh();
    this.refreshAllViews();
  }

  toggleMonth(month: number): void {
    const idx = this.timeFilter.excludeMonths.indexOf(month);
    if (idx >= 0) this.timeFilter.excludeMonths.splice(idx, 1);
    else this.timeFilter.excludeMonths.push(month);
    this.refresh();
    this.refreshAllViews();
  }

  toggleDay(day: number): void {
    const idx = this.timeFilter.excludeDays.indexOf(day);
    if (idx >= 0) this.timeFilter.excludeDays.splice(idx, 1);
    else this.timeFilter.excludeDays.push(day);
    this.refresh();
    this.refreshAllViews();
  }

  toggleContributor(contributor: string): void {
    const idx = this.contributorFilter.excludeContributors.indexOf(contributor);
    if (idx >= 0) this.contributorFilter.excludeContributors.splice(idx, 1);
    else this.contributorFilter.excludeContributors.push(contributor);
    this.refresh();
    this.refreshAllViews();
  }

  togglePolicy(policy: string): void {
    const idx = this.policyFilter.excludePolicies.indexOf(policy);
    if (idx >= 0) this.policyFilter.excludePolicies.splice(idx, 1);
    else this.policyFilter.excludePolicies.push(policy);
    this.refresh();
    this.refreshAllViews();
  }

  toggleViolation(violation: string): void {
    const idx = this.violationFilter.excludeViolations.indexOf(violation);
    if (idx >= 0) this.violationFilter.excludeViolations.splice(idx, 1);
    else this.violationFilter.excludeViolations.push(violation);
    this.refresh();
    this.refreshAllViews();
  }

  showAll(): void {
    this.filters = { code: true, script: true, config: true, test: true, docs: true, resource: true, license: true };
    this.bundleFilters = { ...DEFAULT_BUNDLE_FILTER };
    this.folderFilters = { ...DEFAULT_FOLDER_FILTER };
    this.definitionFilters = { ...DEFAULT_DEFINITION_FILTER };
    this.sectionFilter = { none: false, all: true };
    this.timeFilter = { ...DEFAULT_TIME_FILTER };
    this.contributorFilter = { ...DEFAULT_CONTRIBUTOR_FILTER };
    this.policyFilter = { ...DEFAULT_POLICY_FILTER };
    this.violationFilter = { ...DEFAULT_VIOLATION_FILTER };
    this.refresh();
    this.refreshAllViews();
  }

  showNone(): void {
    this.filters = { code: false, script: false, config: false, test: false, docs: false, resource: false, license: false };
    this.bundleFilters = { library: false, schema: false, binary: false, ui: false, site: false, assets: false };
    this.folderFilters = { organization: false, required: false };
    this.definitionFilters = { implementation: false, interface: false, constant: false };
    this.sectionFilter = { none: true, all: false };
    this.refresh();
    this.refreshAllViews();
  }

  showDefault(): void {
    this.filters = { ...DEFAULT_FILTER };
    this.bundleFilters = { ...DEFAULT_BUNDLE_FILTER };
    this.folderFilters = { ...DEFAULT_FOLDER_FILTER };
    this.definitionFilters = { ...DEFAULT_DEFINITION_FILTER };
    this.sectionFilter = { ...DEFAULT_SECTION_FILTER };
    this.timeFilter = { ...DEFAULT_TIME_FILTER };
    this.contributorFilter = { ...DEFAULT_CONTRIBUTOR_FILTER };
    this.policyFilter = { ...DEFAULT_POLICY_FILTER };
    this.violationFilter = { ...DEFAULT_VIOLATION_FILTER };
    this.refresh();
    this.refreshAllViews();
  }

  getFilters(): CodebaseFilter { return this.filters; }
  getBundleFilters(): BundleFilter { return this.bundleFilters; }
  getFolderFilters(): FolderFilter { return this.folderFilters; }
  getDefinitionFilters(): DefinitionFilter { return this.definitionFilters; }
  getSectionFilter(): SectionFilter { return this.sectionFilter; }

  getIncludeKinds(): FileKind[] { return (Object.keys(this.filters) as FileKind[]).filter(k => this.filters[k]); }
  getExcludeKinds(): FileKind[] { return (Object.keys(this.filters) as FileKind[]).filter(k => !this.filters[k]); }
  getIncludeBundleKinds(): BundleKind[] { return (Object.keys(this.bundleFilters) as BundleKind[]).filter(k => this.bundleFilters[k]); }
  getExcludeBundleKinds(): BundleKind[] { return (Object.keys(this.bundleFilters) as BundleKind[]).filter(k => !this.bundleFilters[k]); }
  getIncludeFolderKinds(): FolderKind[] { return (Object.keys(this.folderFilters) as FolderKind[]).filter(k => this.folderFilters[k]); }
  getExcludeFolderKinds(): FolderKind[] { return (Object.keys(this.folderFilters) as FolderKind[]).filter(k => !this.folderFilters[k]); }
  getIncludeDefinitionKinds(): DefinitionKind[] { return (Object.keys(this.definitionFilters) as DefinitionKind[]).filter(k => this.definitionFilters[k]); }
  getExcludeDefinitionKinds(): DefinitionKind[] { return (Object.keys(this.definitionFilters) as DefinitionKind[]).filter(k => !this.definitionFilters[k]); }

  getTimeFilter(): TimeFilter { return this.timeFilter; }
  getContributorFilter(): ContributorFilter { return this.contributorFilter; }
  getPolicyFilter(): PolicyFilter { return this.policyFilter; }
  getViolationFilter(): ViolationFilter { return this.violationFilter; }

  getTreeItem(element: FilterItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: FilterItem): Promise<FilterItem[]> {
    if (!element) {
      return [
        new FilterItem("Search", vscode.TreeItemCollapsibleState.None, "search"),
        new FilterItem("Bundles", vscode.TreeItemCollapsibleState.Collapsed, "bundle"),
        new FilterItem("Folders", vscode.TreeItemCollapsibleState.Collapsed, "folder"),
        new FilterItem("Sections", vscode.TreeItemCollapsibleState.Collapsed, "section"),
        new FilterItem("Definitions", vscode.TreeItemCollapsibleState.Collapsed, "definition"),
        new FilterItem("Time", vscode.TreeItemCollapsibleState.Collapsed, "time"),
        new FilterItem("Contributors", vscode.TreeItemCollapsibleState.Collapsed, "contributors"),
        new FilterItem("Policies", vscode.TreeItemCollapsibleState.Collapsed, "policy"),
        new FilterItem("Violations", vscode.TreeItemCollapsibleState.Collapsed, "violation")
      ];
    }

    if (element.itemType === "time") {
      return this.availableYears.map(year => {
        const excluded = this.timeFilter.excludeYears.includes(year);
        const item = new FilterItem(`${year}`, vscode.TreeItemCollapsibleState.Collapsed, "timeYear", year);
        item.iconPath = new vscode.ThemeIcon(excluded ? "circle-slash" : "check");
        item.contextValue = "filterTimeYear";
        return item;
      });
    }

    if (element.itemType === "timeYear" && element.timeValue !== undefined) {
      const months = this.availableMonths.length > 0 ? this.availableMonths : [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
      return months.map(month => {
        const excluded = this.timeFilter.excludeMonths.includes(month);
        const date = new Date(2000, month - 1, 1);
        const monthName = date.toLocaleString('default', { month: 'long' });
        const item = new FilterItem(monthName, vscode.TreeItemCollapsibleState.Collapsed, "timeMonth", element.timeValue, month);
        item.iconPath = new vscode.ThemeIcon(excluded ? "circle-slash" : "check");
        item.contextValue = "filterTimeMonth";
        return item;
      });
    }

    if (element.itemType === "timeMonth" && element.secondaryTimeValue !== undefined) {
      const days = this.availableDays.length > 0 ? this.availableDays : Array.from({ length: 31 }, (_, i) => i + 1);
      return days.map(day => {
        const excluded = this.timeFilter.excludeDays.includes(day);
        const item = new FilterItem(`${day}`, vscode.TreeItemCollapsibleState.None, "timeDay", element.timeValue, element.secondaryTimeValue, day);
        item.iconPath = new vscode.ThemeIcon(excluded ? "circle-slash" : "check");
        item.contextValue = "filterTimeDay";
        return item;
      });
    }

    if (element.itemType === "contributors") {
      return this.availableContributors.map(c => {
        const excluded = this.contributorFilter.excludeContributors.includes(c);
        const item = new FilterItem(c, vscode.TreeItemCollapsibleState.None, "contributor", undefined, undefined, undefined, c);
        item.iconPath = new vscode.ThemeIcon(excluded ? "circle-slash" : "check");
        item.contextValue = "filterContributor";
        return item;
      });
    }

    if (element.itemType === "bundle") {
      return Object.keys(this.bundleFilters).map(k => {
        const item = new FilterItem(k, vscode.TreeItemCollapsibleState.None, "action");
        item.contextValue = "filterBundleKind";
        item.command = { command: "semio.toggleBundleFilter", title: "Toggle", arguments: [k] };
        item.iconPath = new vscode.ThemeIcon((this.bundleFilters as any)[k] ? "check" : "circle-slash");
        return item;
      });
    }

    if (element.itemType === "folder") {
      return Object.keys(this.folderFilters).map(k => {
        const item = new FilterItem(k, vscode.TreeItemCollapsibleState.None, "action");
        item.contextValue = "filterFolderKind";
        item.command = { command: "semio.toggleFolderFilter", title: "Toggle", arguments: [k] };
        item.iconPath = new vscode.ThemeIcon((this.folderFilters as any)[k] ? "check" : "circle-slash");
        return item;
      });
    }

    if (element.itemType === "definition") {
      return Object.keys(this.definitionFilters).map(k => {
        const item = new FilterItem(k, vscode.TreeItemCollapsibleState.None, "action");
        item.contextValue = "filterDefinitionKind";
        item.command = { command: "semio.toggleDefinitionFilter", title: "Toggle", arguments: [k] };
        item.iconPath = new vscode.ThemeIcon((this.definitionFilters as any)[k] ? "check" : "circle-slash");
        return item;
      });
    }

    if (element.itemType === "section") {
      const items: FilterItem[] = [];
      const noneItem = new FilterItem("None", vscode.TreeItemCollapsibleState.None, "action");
      noneItem.command = { command: "semio.toggleSectionFilter", title: "Toggle", arguments: ["none"] };
      noneItem.iconPath = new vscode.ThemeIcon(this.sectionFilter.none ? "check" : "circle-slash");
      items.push(noneItem);

      const allItem = new FilterItem("All", vscode.TreeItemCollapsibleState.None, "action");
      allItem.command = { command: "semio.toggleSectionFilter", title: "Toggle", arguments: ["all"] };
      allItem.iconPath = new vscode.ThemeIcon(this.sectionFilter.all ? "check" : "circle-slash");
      items.push(allItem);
      return items;
    }

    if (element.itemType === "policy") {
      return this.availablePolicies.map(p => {
        const excluded = this.policyFilter.excludePolicies.includes(p);
        const item = new FilterItem(p, vscode.TreeItemCollapsibleState.None, "action", undefined, undefined, undefined, undefined, p, undefined);
        item.iconPath = new vscode.ThemeIcon(excluded ? "circle-slash" : "check");
        item.contextValue = "filterPolicy";
        item.command = { command: "semio.togglePolicyFilter", title: "Toggle", arguments: [p] };
        return item;
      });
    }

    if (element.itemType === "violation") {
      return this.availableViolations.map(v => {
        const excluded = this.violationFilter.excludeViolations.includes(v);
        const item = new FilterItem(v, vscode.TreeItemCollapsibleState.None, "action", undefined, undefined, undefined, undefined, undefined, v);
        item.iconPath = new vscode.ThemeIcon(excluded ? "circle-slash" : "check");
        item.contextValue = "filterViolation";
        item.command = { command: "semio.toggleViolationFilter", title: "Toggle", arguments: [v] };
        return item;
      });
    }

    // Default empty
    return [];
  }

  updateAvailableFilters(data: { years?: number[], contributors?: string[], policies?: string[], violations?: string[] }): void {
    if (data.years) this.availableYears = data.years;
    if (data.contributors) this.availableContributors = data.contributors;
    if (data.policies) this.availablePolicies = data.policies;
    if (data.violations) this.availableViolations = data.violations;
    this.refresh();
  }

  shouldInclude(filename: string): boolean {
    if (this.globalSearchQuery && !matchesSearchText(filename, this.globalSearchQuery)) return false;

    if (filename.startsWith(".")) return this.filters.config;
    const ext = path.extname(filename).toLowerCase().replace(".", "");
    if (!ext) return this.filters.config;

    if (FILTER_EXTENSIONS.code.has(ext)) return this.filters.code;
    if (FILTER_EXTENSIONS.script.has(ext)) return this.filters.script;
    if (FILTER_EXTENSIONS.config.has(ext)) return this.filters.config;
    if (FILTER_EXTENSIONS.docs.has(ext)) return this.filters.docs;
    if (FILTER_EXTENSIONS.resource.has(ext)) return this.filters.resource;

    return true;
  }
}

class CodebaseRepoItem extends vscode.TreeItem {
  constructor(public readonly treeChildren: TreeNodeMap) {
    super("@semio", vscode.TreeItemCollapsibleState.Expanded);
    this.iconPath = new vscode.ThemeIcon("repo");
    this.contextValue = "codebaseRepo";
    this.command = { command: "semio.navigateToRepo", title: "Navigate to Repo", arguments: [] };
  }
}

class CodebaseBundleItem extends vscode.TreeItem {
  constructor(
    public readonly bundle: CodebaseBundle,
    public readonly childNodes: TreeNodeMap,
  ) {
    // Use bundle.name instead of bundle.id to avoid "bundle:" prefix
    super(bundle.name, vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon("package");
    this.contextValue = "codebaseBundle";
    this.description = bundle.projectType || "";
    this.tooltip = `${bundle.name} (${bundle.root})`;
    this.command = { command: "semio.navigateToBundle", title: "Navigate to Bundle", arguments: [bundle.root] };
  }
}

class CodebaseFolderItem extends vscode.TreeItem {
  constructor(
    public readonly folderPath: string,
    public readonly displayName: string,
    public readonly childNodes: TreeNodeMap,
  ) {
    super(displayName, vscode.TreeItemCollapsibleState.Collapsed);
    this.iconPath = new vscode.ThemeIcon("folder");
    this.contextValue = "codebaseFolder";
    this.command = { command: "semio.navigateToFolder", title: "Navigate to Folder", arguments: [folderPath] };
  }
}

class CodebaseFileItem extends vscode.TreeItem {
  constructor(
    public readonly file: CodebaseFile,
    public readonly displayName: string,
    public readonly hasSectionsOrDefs: boolean,
  ) {
    super(displayName, hasSectionsOrDefs ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None);
    this.iconPath = new vscode.ThemeIcon("file-code");
    this.contextValue = "codebaseFile";
    this.tooltip = file.path;
    this.command = { command: "semio.navigateToFile", title: "Navigate to File", arguments: [file.path] };
  }
}

class CodebaseSectionItem extends vscode.TreeItem {
  constructor(
    public readonly section: CodebaseSection,
    public readonly file: CodebaseFile,
    public readonly displayName: string,
    public readonly hasChildren: boolean,
  ) {
    super(displayName, hasChildren ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None);
    this.iconPath = new vscode.ThemeIcon("symbol-namespace");
    this.contextValue = "codebaseSection";
    this.tooltip = section.name;
    this.command = { command: "semio.navigateToSection", title: "Navigate to Section", arguments: [file.uri, section.range?.start] };
  }
}

class CodebaseDefinitionItem extends vscode.TreeItem {
  constructor(
    public readonly definition: CodebaseDefinition,
    public readonly fileUri: string,
  ) {
    const name = definition.name || definition.id.split("§").pop() || definition.id;
    super(name, vscode.TreeItemCollapsibleState.None);
    this.iconPath = new vscode.ThemeIcon("symbol-function");
    this.contextValue = "codebaseDefinition";
    this.tooltip = definition.name;
    this.command = { command: "semio.navigateToDefinition", title: "Navigate to Definition", arguments: [fileUri, definition.range?.start] };
  }
}

class CodebaseProvider implements vscode.TreeDataProvider<CodebaseTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<CodebaseTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;
  private cachedBundles: Bundle[] | null = null;

  refresh(): void {
    this.cachedBundles = null;
    refreshCodebase();
    resetUrqlClient();
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(element: CodebaseTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: CodebaseTreeItem): Promise<CodebaseTreeItem[]> {
    // Root level: fetch only bundles (lazy loading)
    if (!element) {
      log("[CodebaseProvider] Fetching bundles for root level...");
      if (!this.cachedBundles) {
        this.cachedBundles = await fetchBundlesViaGraphQL();
      }
      if (!this.cachedBundles || this.cachedBundles.length === 0) {
        log("[CodebaseProvider] No bundles found");
        return [];
      }
      log(`[CodebaseProvider] Found ${this.cachedBundles.length} bundles`);
      // Sort bundles by name
      const sortedBundles = [...this.cachedBundles].sort((a, b) => a.name.localeCompare(b.name));
      return sortedBundles.map((bundle) => new CodebaseBundleItem(bundle, {}));
    }

    if (element instanceof CodebaseRepoItem) {
      // Legacy support - shouldn't be needed with new lazy loading
      return [];
    }

    // Bundle expanded: fetch folder content
    if (element instanceof CodebaseBundleItem) {
      log(`[CodebaseProvider] Fetching content for bundle: ${element.bundle.root}`);
      const content = await fetchFolderContent(element.bundle.root);
      if (content) {
        return this.buildChildrenFromContent(content);
      }
      return [];
    }

    // Folder expanded: fetch folder content
    if (element instanceof CodebaseFolderItem) {
      log(`[CodebaseProvider] Fetching content for folder: ${element.folderPath}`);
      const content = await fetchFolderContent(element.folderPath);
      if (content) {
        return this.buildChildrenFromContent(content);
      }
      return [];
    }

    // File expanded: fetch file content via CLI
    if (element instanceof CodebaseFileItem) {
      log(`[CodebaseProvider] Fetching content for file: ${element.file.path}`);
      await this.loadFileContent(element.file);
      return this.buildFileChildren(element.file);
    }

    // Section expanded: show child sections and definitions
    if (element instanceof CodebaseSectionItem) {
      return this.buildSectionChildren(element.section, element.file);
    }

    return [];
  }

  private buildChildrenFromContent(content: NonNullable<DocumentType<typeof FolderContentDocument>["folder"]>): CodebaseTreeItem[] {
    const items: CodebaseTreeItem[] = [];

    for (const folder of content.children) {
      items.push(new CodebaseFolderItem(folder.path, folder.name, {}));
    }

    for (const file of content.files) {
      if (filterProvider && !filterProvider.shouldInclude(file.name)) continue;
      const fileObj: CodebaseFile = {
        id: file.path,
        path: file.path,
        uri: file.uri,
        sections: [],
        definitions: [],
      } as any;
      // We assume it might have sections/definitions, so we allow expansion
      items.push(new CodebaseFileItem(fileObj, file.name, true));
    }

    // Sort items: folders first, then files
    items.sort((a, b) => {
      if (a instanceof CodebaseFolderItem && b instanceof CodebaseFileItem) return -1;
      if (a instanceof CodebaseFileItem && b instanceof CodebaseFolderItem) return 1;
      return (a.label as string).localeCompare(b.label as string);
    });

    return items;
  }

  private async loadFileContent(file: CodebaseFile): Promise<void> {
    if (file.sections && file.sections.length > 0) return;

    try {
      // Load sections
      const secResult = await runRepoCommandJson<ToolResult<SectionInfo[]>>(`section list --file "${file.path}"`);
      const sections = extractSections(secResult);
      file.sections = this.flattenSections(sections, file.path) as any;

      // Load definitions
      const defResult = await runRepoCommandJson<ToolResult<DefinitionInfo[]>>(`definition list --file "${file.path}"`);
      const definitions = extractDefinitions(defResult);
      if (definitions.length > 0) {
        file.definitions = definitions.map((d) => ({
          id: file.path + "§" + d.name,
          name: d.name,
          kind: d.kind,
          range: { start: d.startLine, end: d.endLine },
        })) as any;
      }
    } catch (e) {
      logError("loadFileContent error", e);
    }
  }

  private flattenSections(sections: SectionInfo[], parentPath: string): any[] {
    let result: any[] = [];
    for (const s of sections) {
      const myPath = parentPath + "#" + s.name;
      const gqlS = {
        id: myPath,
        name: s.name,
        path: myPath,
        range: { start: s.startLine, end: s.endLine },
      };
      result.push(gqlS);
      result = result.concat(this.flattenSections(s.children, myPath));
    }
    return result;
  }

  private buildFileChildren(file: CodebaseFile): CodebaseTreeItem[] {
    const items: CodebaseTreeItem[] = [];

    // Root sections (depth 1 - file#section)
    const rootSections = file.sections.filter((s) => {
      const parts = s.path.split("#");
      return parts.length === 2;
    });

    for (const section of rootSections) {
      const hasSubSections = file.sections.some(s => s.path.startsWith(section.path + "#") && s.path !== section.path);
      const hasDefinitions = this.getDefinitionsForSection(file, section).length > 0;
      items.push(new CodebaseSectionItem(section, file, section.name, hasSubSections || hasDefinitions));
    }

    // File-level definitions (not in any section)
    const fileDefinitions = file.definitions.filter((d) => {
      return !rootSections.some(s => this.isDefinitionInSection(d, s));
    });
    for (const def of fileDefinitions) {
      items.push(new CodebaseDefinitionItem(def, file.uri));
    }

    return this.sortItemsByLine(items);
  }

  private buildSectionChildren(section: CodebaseSection, file: CodebaseFile): CodebaseTreeItem[] {
    const items: CodebaseTreeItem[] = [];
    const sectionPathWithHash = section.path + "#";

    const childSections = file.sections.filter((s) => {
      if (!s.path.startsWith(sectionPathWithHash)) return false;
      const remainder = s.path.substring(sectionPathWithHash.length);
      return !remainder.includes("#");
    });

    const allSectionDefs = this.getDefinitionsForSection(file, section);
    // Definitions directly in this section but NOT in any child section
    const directDefs = allSectionDefs.filter((d) => {
      return !childSections.some(child => this.isDefinitionInSection(d, child));
    });

    for (const child of childSections) {
      const hasGrand = file.sections.some((s) => s.path.startsWith(child.path + "#") && s.path !== child.path);
      // Ensure recursive definitions check enables collapsing
      const hasChildDefs = this.getDefinitionsForSection(file, child).length > 0;
      items.push(new CodebaseSectionItem(child, file, child.name, hasGrand || hasChildDefs));
    }

    for (const def of directDefs) {
      items.push(new CodebaseDefinitionItem(def, file.uri));
    }

    return this.sortItemsByLine(items);
  }

  private sortItemsByLine(items: CodebaseTreeItem[]): CodebaseTreeItem[] {
    return items.sort((a, b) => {
      const lineA = this.getItemStartLine(a);
      const lineB = this.getItemStartLine(b);
      return lineA - lineB;
    });
  }

  private getItemStartLine(item: CodebaseTreeItem): number {
    if (item instanceof CodebaseSectionItem) return (item.section.range?.start as any) || 0;
    if (item instanceof CodebaseDefinitionItem) return (item.definition.range?.start as any) || 0;
    return 0;
  }

  private isDefinitionInSection(def: CodebaseDefinition, section: CodebaseSection): boolean {
    if (!def.range || !section.range) return false;
    const dStart = def.range.start as unknown as number;
    const sStart = section.range.start as unknown as number;
    const sEnd = section.range.end as unknown as number;

    return dStart >= sStart && dStart <= sEnd;
  }

  private getDefinitionsForSection(file: CodebaseFile, section: CodebaseSection): CodebaseDefinitionItem["definition"][] {
    return file.definitions.filter((d) => this.isDefinitionInSection(d, section));
  }
}

interface CommandInfo {
  id: string;
  title: string;
}

interface CommandNode {
  name: string;
  children: Map<string, CommandNode>;
  commands: CommandInfo[];
}

type CommandTreeItem = CommandGroupItem | CommandItem;

class CommandGroupItem extends vscode.TreeItem {
  constructor(public readonly node: CommandNode) {
    super(node.name, node.children.size > 0 || node.commands.length > 0 ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None);
    this.iconPath = new vscode.ThemeIcon("folder");
    this.contextValue = "commandGroup";
  }
}

class CommandItem extends vscode.TreeItem {
  constructor(public readonly cmd: CommandInfo) {
    super(cmd.title.replace("semio: ", ""), vscode.TreeItemCollapsibleState.None);
    this.tooltip = cmd.id;
    this.description = cmd.id;
    this.iconPath = new vscode.ThemeIcon("terminal");
    this.contextValue = "command";
    this.command = { command: "semio.openCommand", title: "Open Command", arguments: [cmd.id] };
  }
}

const SIDEBAR_COMMANDS: CommandInfo[] = [
  { id: "semio.analyze", title: "Analyze Codebase" },
  { id: "semio.analyzeFile", title: "Analyze Current File" },
  { id: "semio.fix", title: "Fix Codebase Problems" },
  { id: "semio.fixFile", title: "Fix Current File Problems" },
  { id: "semio.ticketOpen", title: "Create Ticket" },
  { id: "semio.ticketClose", title: "Finish Ticket" },
  { id: "semio.folderTree", title: "Show Folder Tree" },
  { id: "semio.folderCreate", title: "Create Folder" },
  { id: "semio.fileCreate", title: "Create File" },
  { id: "semio.sectionTree", title: "Show Section Tree" },
  { id: "semio.definitionList", title: "List Definitions" },
  { id: "semio.export", title: "Export Repo to SQLite" },
  { id: "semio.refreshDiagnostics", title: "Refresh Diagnostics" },
];

class CommandsProvider implements vscode.TreeDataProvider<CommandTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<CommandTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;
  private treeRoot = this.buildCommandTree(SIDEBAR_COMMANDS);

  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  private getCommandSegments(commandId: string): string[] {
    const segments: string[] = [];
    for (const part of (commandId.startsWith("semio.") ? commandId.slice(6) : commandId).split(".")) {
      for (const piece of part.split(/(?=[A-Z])/)) {
        const lower = piece.toLowerCase();
        if (lower) segments.push(lower);
      }
    }
    return segments;
  }

  private buildCommandTree(commands: CommandInfo[]): CommandNode {
    const root: CommandNode = { name: "", children: new Map(), commands: [] };
    for (const command of commands) {
      const segments = this.getCommandSegments(command.id);
      let node = root;
      for (const segment of segments.length > 1 ? segments.slice(0, -1) : segments) {
        if (!node.children.has(segment)) {
          node.children.set(segment, { name: segment, children: new Map(), commands: [] });
        }
        node = node.children.get(segment)!;
      }
      node.commands.push(command);
    }
    return root;
  }

  private matchesSearch(cmd: CommandInfo): boolean {
    if (!globalSearchQuery) return true;
    const segments = this.getCommandSegments(cmd.id);
    const searchable = [cmd.id, cmd.title, ...segments].join(" ");
    return matchesSearchText(searchable);
  }

  private matchesGroupSearch(name: string): boolean {
    if (!globalSearchQuery) return true;
    return matchesSearchText(name);
  }

  private buildTreeItems(node: CommandNode, includeAll: boolean): CommandTreeItem[] {
    const items: CommandTreeItem[] = [];
    for (const child of [...node.children.values()].sort((a, b) => a.name.localeCompare(b.name))) {
      const groupMatches = includeAll || this.matchesGroupSearch(child.name);
      if (groupMatches || this.buildTreeItems(child, groupMatches).length > 0) {
        items.push(new CommandGroupItem(child));
      }
    }
    for (const command of includeAll || !globalSearchQuery ? node.commands : node.commands.filter((cmd) => this.matchesSearch(cmd))) {
      items.push(new CommandItem(command));
    }
    return items;
  }

  getTreeItem(element: CommandTreeItem): vscode.TreeItem {
    return element;
  }

  getChildren(element?: CommandTreeItem): CommandTreeItem[] {
    const node = element instanceof CommandGroupItem ? element.node : this.treeRoot;
    return this.buildTreeItems(node, element instanceof CommandGroupItem ? this.matchesGroupSearch(node.name) : false);
  }
}


class CodebaseDragAndDropController implements vscode.TreeDragAndDropController<CodebaseTreeItem> {
  dropMimeTypes = ["application/vnd.code.tree.semio.codebase"];
  dragMimeTypes = ["application/vnd.code.tree.semio.codebase"];

  handleDrag(source: readonly CodebaseTreeItem[], dataTransfer: vscode.DataTransfer, token: vscode.CancellationToken): void | Thenable<void> {
    dataTransfer.set("application/vnd.code.tree.semio.codebase", new vscode.DataTransferItem(source));
    // Also support dragging sections as text for other views if needed, but primary use case is internal.
  }

  async handleDrop(target: CodebaseTreeItem | undefined, dataTransfer: vscode.DataTransfer, token: vscode.CancellationToken): Promise<void> {
    const transferItem = dataTransfer.get("application/vnd.code.tree.semio.codebase");
    if (!transferItem) return;

    const sources = transferItem.value as CodebaseTreeItem[];
    if (sources.length === 0) return;
    const source = sources[0];

    // Case 1: Integrate (File -> Section)
    if (source instanceof CodebaseFileItem && target instanceof CodebaseSectionItem) {
      const sourcePath = source.file.path;
      const targetFile = target.file.path;
      const targetSectionName = target.section.name;

      try {
        log(`[CodebaseDragAndDrop] Integrating ${sourcePath} into ${targetFile} section ${targetSectionName}`);
        await runRepoCommandJson(`section integrate "${sourcePath}" "${targetSectionName}" "${targetFile}"`);
        vscode.window.showInformationMessage(`Integrated ${path.basename(sourcePath)} into ${targetSectionName}`);
        codebaseProvider.refresh();
      } catch (e) {
        logError("Integrate failed", e);
        vscode.window.showErrorMessage("Integrate failed: " + (e instanceof Error ? e.message : String(e)));
      }
      return;
    }

    // Case 2: Extract (Section -> Folder/Bundle)
    if (source instanceof CodebaseSectionItem && (target instanceof CodebaseFolderItem || target instanceof CodebaseBundleItem)) {
      const sourceFile = source.file.path;
      const sourceSection = source.section.name;

      let targetDir = "";
      if (target instanceof CodebaseFolderItem) targetDir = target.folderPath;
      if (target instanceof CodebaseBundleItem) targetDir = target.bundle.root;

      // Suggest filename based on section name + extension of source file
      const originalExt = path.extname(sourceFile);
      const defaultName = sourceSection + originalExt;

      const filename = await vscode.window.showInputBox({
        title: "Extract Section",
        prompt: `Enter filename for extracted section "${sourceSection}"`,
        value: defaultName,
        placeHolder: "filename.ext"
      });

      if (!filename) return;

      const targetFile = path.join(targetDir, filename);

      try {
        log(`[CodebaseDragAndDrop] Extracting ${sourceSection} from ${sourceFile} to ${targetFile}`);
        await runRepoCommandJson(`section extract "${sourceFile}" "${sourceSection}" "${targetFile}"`);
        vscode.window.showInformationMessage(`Extracted ${sourceSection} to ${filename}`);
        codebaseProvider.refresh();
      } catch (e) {
        logError("Extract failed", e);
        vscode.window.showErrorMessage("Extract failed: " + (e instanceof Error ? e.message : String(e)));
      }
      return;
    }
  }
}

let ticketsProvider: TicketsProvider;
let todosProvider: TodosProvider;
let contributorsProvider: ContributorsProvider;
let policiesProvider: PoliciesProvider;
let commandsProvider: CommandsProvider;
let sectionsProvider: SectionsProvider;
let codebaseProvider: CodebaseProvider;
let filterProvider: FilterProvider;

function registerSidebarViews(context: vscode.ExtensionContext): void {
  try {
    log("[registerSidebarViews] Initializing providers...");
    filterProvider = new FilterProvider(getWorkspaceRoot() || "");
    ticketsProvider = new TicketsProvider();
    todosProvider = new TodosProvider();
    contributorsProvider = new ContributorsProvider();
    policiesProvider = new PoliciesProvider();
    commandsProvider = new CommandsProvider();
    sectionsProvider = new SectionsProvider();
    codebaseProvider = new CodebaseProvider();

    log("[registerSidebarViews] Registering semio.filter tree view...");
    context.subscriptions.push(vscode.window.createTreeView("semio.filter", { treeDataProvider: filterProvider }));

    log("[registerSidebarViews] Creating semio.codebase tree view...");
    context.subscriptions.push(vscode.window.createTreeView("semio.codebase", { treeDataProvider: codebaseProvider, showCollapseAll: true, dragAndDropController: new CodebaseDragAndDropController() }));

    log("[registerSidebarViews] Creating semio.tickets tree view...");
    context.subscriptions.push(vscode.window.createTreeView("semio.tickets", { treeDataProvider: ticketsProvider, showCollapseAll: true }));

    log("[registerSidebarViews] Creating semio.todos tree view...");
    context.subscriptions.push(vscode.window.createTreeView("semio.todos", { treeDataProvider: todosProvider, showCollapseAll: true }));

    log("[registerSidebarViews] Creating semio.contributors tree view...");
    context.subscriptions.push(vscode.window.createTreeView("semio.contributors", { treeDataProvider: contributorsProvider, showCollapseAll: true }));

    log("[registerSidebarViews] Creating semio.policies tree view...");
    context.subscriptions.push(vscode.window.createTreeView("semio.policies", { treeDataProvider: policiesProvider, showCollapseAll: true }));

    log("[registerSidebarViews] Creating semio.commands tree view...");
    context.subscriptions.push(vscode.window.createTreeView("semio.commands", { treeDataProvider: commandsProvider }));

    log("[registerSidebarViews] Creating semio.sections tree view...");
    context.subscriptions.push(vscode.window.createTreeView("semio.sections", { treeDataProvider: sectionsProvider, dragAndDropController: new SectionsDragAndDropController() }));

    log("[registerSidebarViews] Registering refresh commands...");
    context.subscriptions.push(
      vscode.commands.registerCommand("semio.refreshCodebase", () => codebaseProvider.refresh()),
      vscode.commands.registerCommand("semio.refreshTickets", () => ticketsProvider.refresh()),
      vscode.commands.registerCommand("semio.refreshContributors", () => contributorsProvider.refresh()),
      vscode.commands.registerCommand("semio.refreshPolicies", () => policiesProvider.refresh()),
    );

    loadAvailableFilterValues();

    log("[registerSidebarViews] Sidebar views registration complete.");
  } catch (error) {
    logError("[registerSidebarViews] CRASH during registration:", error);
    throw error;
  }
}

async function loadAvailableFilterValues(): Promise<void> {
  try {
    const root = getWorkspaceRoot();
    if (!root) return;

    const years: Set<number> = new Set();
    const contributors: Set<string> = new Set();
    const policies: Set<string> = new Set();
    const violations: Set<string> = new Set();

    const tickets = await fetchTicketsViaGraphQL();
    for (const ticket of tickets) {
      years.add(ticket.year);
    }

    const contributorsList = await fetchContributorsViaGraphQL();
    for (const contributor of contributorsList) {
      contributors.add(contributor.name || contributor.id);
    }

    const policiesList = await fetchPoliciesViaGraphQL();
    for (const policy of policiesList) {
      policies.add(policy.id);
      if (policy.violationKinds) {
        for (const violation of policy.violationKinds) {
          violations.add(violation.id);
        }
      }
    }

    if (filterProvider) {
      filterProvider.updateAvailableFilters({
        years: Array.from(years).sort((a, b) => b - a),
        contributors: Array.from(contributors).sort(),
        policies: Array.from(policies).sort(),
        violations: Array.from(violations).sort(),
      });
    }
  } catch (error) {
    logError("[loadAvailableFilterValues] Failed to load filter values:", error);
  }
}


// #region Sidebar Views Registration

let monorepoProvider: MonorepoTreeDataProvider | undefined;
let filterProvider: FilterTreeDataProvider | undefined;

function registerSidebarViews(context: vscode.ExtensionContext): void {
  monorepoProvider = new MonorepoTreeDataProvider(context);
  vscode.window.registerTreeDataProvider("semio.monorepo", monorepoProvider);

  filterProvider = new FilterTreeDataProvider();
  vscode.window.registerTreeDataProvider("semio.filter", filterProvider);
}

// #endregion Sidebar Views Registration

// #endregion Sidebar Views


// #region Commands

// #region Smart Wizards

const LLMS = ["opus-4-5", "sonnet-4-5", "haiku-4-5", "gemini-3-pro", "gemini-3-flash", "gpt-5-2-codex", "gpt-5-mini", "swe-1-5"];
const ClientS = ["copilot-chat", "windsurf-chat", "claude-code", "codex", "cursor-chat", "antigravity-chat", "droid"];

async function pickLLM(): Promise<string | undefined> {
  return await vscode.window.showQuickPick(LLMS, { placeHolder: "Select LLM (default: opus-4-5)" });
}

async function pickClient(): Promise<string | undefined> {
  return await vscode.window.showQuickPick(ClientS, { placeHolder: "Select Client (default: copilot-chat)" });
}

interface GoalItem extends vscode.QuickPickItem {
  goalId: string;
  isLeaf: boolean;
  children: GoalItem[];
}

async function pickGoal(): Promise<string | undefined> {
  const client = getUrqlClient();
  if (!client) {
    vscode.window.showErrorMessage("Repo client not available");
    return undefined;
  }

  const result = await client.query(GoalsDocument, {}).toPromise();
  if (result.error) {
    vscode.window.showErrorMessage("Failed to fetch goals: " + result.error.message);
    return undefined;
  }
  const goals = result.data?.repo?.goals ?? [];
  if (goals.length === 0) return ""; // No goals available, return empty string which means root/no-goal

  // Normalize and sort
  goals.sort((a, b) => a.id.localeCompare(b.id));

  const getChildren = (parentId: string | null): typeof goals => {
    return goals.filter(g => {
      if (parentId === null) {
        return !g.id.includes("/");
      }
      return g.id.startsWith(parentId + "/") && g.id.split("/").length === parentId.split("/").length + 1;
    });
  };

  let currentParentId: string | null = null;

  while (true) {
    const children = getChildren(currentParentId);
    const items: vscode.QuickPickItem[] = children.map(g => ({
      label: g.title || g.id,
      description: g.id,
      detail: g.description || undefined
    }));

    const CONFIRM = "$(check) Confirm " + (currentParentId ? `"${currentParentId}"` : "None");
    const UP = "$(arrow-up) Up";

    const specialItems: vscode.QuickPickItem[] = [];
    specialItems.push({ label: CONFIRM, description: "Select this goal" });
    if (currentParentId !== null) {
      specialItems.push({ label: UP, description: "Go to parent" });
    }

    const selection = await vscode.window.showQuickPick([...specialItems, ...items], {
      placeHolder: currentParentId ? `Select sub-goal of ${currentParentId}` : "Select goal",
      ignoreFocusOut: true
    });

    if (!selection) return undefined; // Cancelled

    if (selection.label.startsWith("$(check) Confirm")) {
      return currentParentId || "";
    }
    if (selection.label.startsWith("$(arrow-up) Up")) {
      const parts: string[] = currentParentId!.split("/");
      parts.pop();
      currentParentId = parts.length > 0 ? parts.join("/") : null;
      continue;
    }

    // Selected a child
    const selectedGoal = goals.find(g => (g.title || g.id) === selection.label && g.id === selection.description);
    if (selectedGoal) {
      const grandChildren = getChildren(selectedGoal.id);
      if (grandChildren.length > 0) {
        // Has children, dive in
        currentParentId = selectedGoal.id;
        continue;
      } else {
        // Leaf node. User logic "show me... then confirm".
        // Maybe if leaf, we can just return it?
        // Or should we allow user to stay on leaf and see "Confirm"?
        // If I assume selection of leaf is final:
        return selectedGoal.id;
      }
    }
  }
}

async function pickTicketGql(): Promise<string | undefined> {
  const client = getUrqlClient();
  if (!client) return undefined;

  const result = await client.query(TicketsDocument, {}).toPromise();
  if (result.error) return undefined;
  const tickets = result.data?.repo?.tickets ?? [];

  const years = [...new Set(tickets.map(t => t.year))].sort((a, b) => b - a);
  if (years.length === 0) return undefined;

  const yearPick = await vscode.window.showQuickPick(years.map(y => y.toString()), {
    placeHolder: "Select Year",
    ignoreFocusOut: true
  });
  if (!yearPick) return undefined;
  const selectedYear = parseInt(yearPick);

  const months = [...new Set(tickets.filter(t => t.year === selectedYear).map(t => t.month))].sort((a, b) => b - a);
  const monthPick = await vscode.window.showQuickPick(months.map(m => m.toString()), {
    placeHolder: "Select Month",
  });
  if (!monthPick) return undefined;
  const selectedMonth = parseInt(monthPick);

  const monthTickets = tickets.filter(t => t.year === selectedYear && t.month === selectedMonth);
  // Sort by day desc
  monthTickets.sort((a, b) => b.day - a.day);

  const ticketItems = monthTickets.map(t => ({
    label: `${t.year}/${t.month.toString().padStart(2, '0')}/${t.day.toString().padStart(2, '0')}/${t.slug}`,
    description: t.status,
    detail: t.prompt,
    ticket: t
  }));

  const selection = await vscode.window.showQuickPick(ticketItems, {
    placeHolder: "Select Ticket",
    ignoreFocusOut: true
  });
  if (!selection) return undefined;

  const t = selection.ticket;
  return `${t.year}/${t.month.toString().padStart(2, '0')}/${t.day.toString().padStart(2, '0')}/${t.slug}`;
}

// #endregion Smart Wizards

function registerCommands(context: vscode.ExtensionContext): void {
  context.subscriptions.push(
    vscode.commands.registerCommand("semio.toggleFilter", (kind: keyof CodebaseFilter) => {
      if (filterProvider) filterProvider.toggle(kind);
    }),
    vscode.commands.registerCommand("semio.toggleBundleFilter", (kind: keyof BundleFilter) => {
      if (filterProvider) filterProvider.toggleBundleKind(kind);
    }),
    vscode.commands.registerCommand("semio.toggleFolderFilter", (kind: keyof FolderFilter) => {
      if (filterProvider) filterProvider.toggleFolderKind(kind);
    }),
    vscode.commands.registerCommand("semio.toggleDefinitionFilter", (kind: keyof DefinitionFilter) => {
      if (filterProvider) filterProvider.toggleDefinitionKind(kind);
    }),
    vscode.commands.registerCommand("semio.toggleYearFilter", (year: number) => {
      if (filterProvider) filterProvider.toggleYear(year);
    }),
    vscode.commands.registerCommand("semio.toggleMonthFilter", (month: number) => {
      if (filterProvider) filterProvider.toggleMonth(month);
    }),
    vscode.commands.registerCommand("semio.toggleDayFilter", (day: number) => {
      if (filterProvider) filterProvider.toggleDay(day);
    }),
    vscode.commands.registerCommand("semio.toggleContributorFilter", (contributor: string) => {
      if (filterProvider) filterProvider.toggleContributor(contributor);
    }),
    vscode.commands.registerCommand("semio.togglePolicyFilter", (policy: string) => {
      if (filterProvider) filterProvider.togglePolicy(policy);
    }),
    vscode.commands.registerCommand("semio.toggleViolationFilter", (violation: string) => {
      if (filterProvider) filterProvider.toggleViolation(violation);
    }),
    vscode.commands.registerCommand("semio.filterAction", (action: string) => {
      if (!filterProvider) return;
      switch (action) {
        case "showAll":
          filterProvider.showAll();
          break;
        case "showNone":
          filterProvider.showNone();
          break;
        case "showDefault":
          filterProvider.showDefault();
          break;
      }
    }),
    vscode.commands.registerCommand("semio.toggleTicketFilter", () => ticketsProvider.toggleFilter()),
    vscode.commands.registerCommand("semio.openTicket", async (ticket: TicketData | ContributorTicketData | { ticket: TicketData | ContributorTicketData }) => {
      const resolvedTicket = resolveTicketData(ticket);
      if (!resolvedTicket) return;
      const filePath = resolveTicketPath(resolvedTicket);
      if (!filePath) return;
      const root = getWorkspaceRoot();
      const resolvedPath = path.isAbsolute(filePath) ? filePath : root ? path.join(root, filePath) : filePath;
      const uri = vscode.Uri.file(resolvedPath);
      await vscode.commands.executeCommand("markdown.showPreview", uri);
    }),
    vscode.commands.registerCommand("semio.ticketRead", async (ticket: TicketData | ContributorTicketData | { ticket: TicketData | ContributorTicketData }) => {
      await vscode.commands.executeCommand("semio.openTicket", ticket);
    }),
    vscode.commands.registerCommand("semio.openTicketPlan", async (ticket: TicketData | ContributorTicketData | { ticket: TicketData | ContributorTicketData }) => {
      const resolvedTicket = resolveTicketData(ticket);
      if (!resolvedTicket) return;
      const root = getWorkspaceRoot();
      if (!root) return;

      const relPath = path.join(String(resolvedTicket.year), String(resolvedTicket.month).padStart(2, "0"), String(resolvedTicket.day).padStart(2, "0"), resolvedTicket.slug, "plan.md");
      const metaPath = path.join(root, ".semio-repo", "tickets", relPath);
      let filePath = metaPath;
      // Fallback to old location or repo/tickets location if needed, 
      // but logic says we stick to .semio-repo or tickets/
      if (!fs.existsSync(metaPath)) {
        filePath = path.join(root, "tickets", relPath);
      }

      if (!fs.existsSync(filePath)) {
        // Try without plan.md to see if folder exists? 
        // Assuming plan.md is the name.
        vscode.window.showErrorMessage(`Plan file not found: ${filePath}`);
        return;
      }

      const uri = vscode.Uri.file(filePath);
      await vscode.window.showTextDocument(uri);
    }),
    vscode.commands.registerCommand("semio.openPolicy", async (policy: PolicyData, lineNumber?: number) => {
      if (!policy?.id) return;
      const root = getWorkspaceRoot();
      if (!root) return;
      const repoFilePath = path.join(root, "go", "repo", "main.go");
      if (fs.existsSync(repoFilePath)) {
        const uri = vscode.Uri.file(repoFilePath);
        const doc = await vscode.workspace.openTextDocument(uri);
        const content = doc.getText();
        const functionPattern = new RegExp(`^func ${policy.id}Policy\\(`, "m");
        const match = functionPattern.exec(content);
        if (match && match.index !== undefined) {
          const position = doc.positionAt(match.index);
          await vscode.window.showTextDocument(doc, { selection: new vscode.Range(position, position) });
        } else {
          await vscode.window.showTextDocument(doc);
        }
      }
    }),
    vscode.commands.registerCommand("semio.openContributor", async (contributor: ContributorData) => {
      if (!contributor?.github) return;
      const root = getWorkspaceRoot();
      if (!root) return;
      const contributorPath = path.join(root, "contributors", contributor.github, "contributor.json");
      if (fs.existsSync(contributorPath)) {
        const uri = vscode.Uri.file(contributorPath);
        await vscode.window.showTextDocument(uri);
      }
    }),
    vscode.commands.registerCommand("semio.openCommand", async (commandId: string) => {
      const root = getWorkspaceRoot();
      if (!root) return;
      const extensionPath = path.join(root, "js", "vscode", "extension.ts");
      if (!fs.existsSync(extensionPath)) return;
      const doc = await vscode.workspace.openTextDocument(extensionPath);
      const content = doc.getText();
      const commandPattern = new RegExp(`vscode\\.commands\\.registerCommand\\("${commandId.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}"`, "m");
      const match = commandPattern.exec(content);
      if (match && match.index !== undefined) {
        const position = doc.positionAt(match.index);
        await vscode.window.showTextDocument(doc, { selection: new vscode.Range(position, position) });
      } else {
        await vscode.window.showTextDocument(doc);
      }
    }),
    vscode.commands.registerCommand("semio.createPolicy", async () => {
      vscode.window.showInformationMessage("Policies are defined in go/repo/main.go - open the file to add a new policy");
      const root = getWorkspaceRoot();
      if (!root) return;
      const repoFilePath = path.join(root, "go", "repo", "main.go");
      if (fs.existsSync(repoFilePath)) {
        const uri = vscode.Uri.file(repoFilePath);
        const doc = await vscode.workspace.openTextDocument(uri);
        const content = doc.getText();
        const match = content.match(/var policyMetas = \[\]PolicyMeta\{/);
        if (match && match.index !== undefined) {
          const position = doc.positionAt(match.index);
          await vscode.window.showTextDocument(doc, { selection: new vscode.Range(position, position) });
        } else {
          await vscode.window.showTextDocument(doc);
        }
      }
    }),
    vscode.commands.registerCommand("semio.createContributor", async () => {
      await vscode.commands.executeCommand("semio.contributorAdd");
    }),
    vscode.commands.registerCommand("semio.openProject", async (projectName: string) => {
      if (!projectName) return;
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const root = getWorkspaceRoot();
      if (!root) return;
      const bundles = await getProjectList();
      const bundle = bundles.find((p) => p.name === projectName);
      if (!bundle) return;
      const projectRoot = path.join(root, bundle.root);
      const projectJson = path.join(projectRoot, "bundle.json");
      const packageJson = path.join(projectRoot, "package.json");
      if (fs.existsSync(projectJson)) {
        await vscode.window.showTextDocument(vscode.Uri.file(projectJson));
        return;
      }
      if (fs.existsSync(packageJson)) {
        await vscode.window.showTextDocument(vscode.Uri.file(packageJson));
        return;
      }
      await vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(projectRoot));
    }),
    vscode.commands.registerCommand("semio.openFolder", async (folderPath: string) => {
      if (!folderPath) return;
      const root = getWorkspaceRoot();
      if (!root) return;
      const fullPath = path.join(root, folderPath);
      await vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(fullPath));
    }),
    vscode.commands.registerCommand("semio.openSection", async (filePath: string, sectionPath: string) => {
      if (!filePath || !sectionPath) return;
      const root = getWorkspaceRoot();
      if (!root) return;
      const fullPath = path.join(root, filePath);
      const sections = await getSectionListForFile(filePath);
      const sectionName = sectionPath.split("/").pop() || sectionPath;
      const findSection = (secs: SectionInfo[], name: string): SectionInfo | undefined => {
        for (const sec of secs) {
          if (sec.name === name) return sec;
          const found = findSection(sec.children || [], name);
          if (found) return found;
        }
        return undefined;
      };
      const section = findSection(sections, sectionName);
      if (section) {
        const doc = await vscode.workspace.openTextDocument(fullPath);
        const editor = await vscode.window.showTextDocument(doc);
        const position = new vscode.Position(section.startLine - 1, 0);
        editor.selection = new vscode.Selection(position, position);
        editor.revealRange(new vscode.Range(position, position), vscode.TextEditorRevealType.InCenter);
      }
    }),
    vscode.commands.registerCommand("semio.openDefinition", async (filePath: string, definitionName: string) => {
      if (!filePath || !definitionName) return;
      const root = getWorkspaceRoot();
      if (!root) return;
      const fullPath = path.join(root, filePath);
      const result = await runRepoCommandJson<ToolResult<DefinitionInfo[]>>(`definition list "${filePath}"`);
      const definitions = result?.data || [];
      const definition = definitions.find((d) => d.name === definitionName);
      if (definition) {
        const doc = await vscode.workspace.openTextDocument(fullPath);
        const editor = await vscode.window.showTextDocument(doc);
        const position = new vscode.Position(definition.startLine - 1, 0);
        editor.selection = new vscode.Selection(position, position);
        editor.revealRange(new vscode.Range(position, position), vscode.TextEditorRevealType.InCenter);
      }
    }),
    vscode.commands.registerCommand("semio.copyCommitSha", async (commit: string | { sha?: string }) => {
      const sha = resolveCommitSha(commit);
      if (!sha) return;
      await vscode.env.clipboard.writeText(sha);
    }),
    vscode.commands.registerCommand("semio.openCommitInGitHub", async (commit: string | { sha?: string }) => {
      const sha = resolveCommitSha(commit);
      if (!sha) return;
      const baseUrl = getGitHubRepoBaseUrl();
      if (!baseUrl) return;
      await vscode.env.openExternal(vscode.Uri.parse(`${baseUrl}/commit/${sha}`));
    }),
    vscode.commands.registerCommand("semio.checkPolicy", async (policy: PolicyItem) => {
      if (!policy?.policy?.id) return;
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      runRepoCommand(`policy check ${policy.policy.id}`);
    }),
    vscode.commands.registerCommand("semio.runCommand", async (commandId: string) => {
      if (!commandId) return;
      await vscode.commands.executeCommand(commandId);
    }),
    vscode.commands.registerCommand("semio.navigateToRepo", async () => {
      const root = getWorkspaceRoot();
      if (!root) return;
      const uri = vscode.Uri.file(root);
      await vscode.commands.executeCommand("revealInExplorer", uri);
    }),
    vscode.commands.registerCommand("semio.navigateToBundle", async (bundleRoot: string) => {
      const root = getWorkspaceRoot();
      if (!root || !bundleRoot) return;
      const uri = vscode.Uri.file(path.join(root, bundleRoot));
      await vscode.commands.executeCommand("revealInExplorer", uri);
    }),
    vscode.commands.registerCommand("semio.navigateToFolder", async (folderPath: string) => {
      const root = getWorkspaceRoot();
      if (!root || !folderPath) return;
      const uri = vscode.Uri.file(path.join(root, folderPath));
      await vscode.commands.executeCommand("revealInExplorer", uri);
    }),
    vscode.commands.registerCommand("semio.navigateToFile", async (filePath: string) => {
      const root = getWorkspaceRoot();
      if (!root || !filePath) return;
      const uri = vscode.Uri.file(path.join(root, filePath));
      await vscode.window.showTextDocument(uri);
    }),
    vscode.commands.registerCommand("semio.navigateToSection", async (fileUri: string, startLine?: number) => {
      if (!fileUri) return;
      const uri = vscode.Uri.parse(fileUri);
      const doc = await vscode.workspace.openTextDocument(uri);
      const editor = await vscode.window.showTextDocument(doc);
      if (startLine != null && startLine > 0) {
        const position = new vscode.Position(startLine - 1, 0);
        editor.selection = new vscode.Selection(position, position);
        editor.revealRange(new vscode.Range(position, position), vscode.TextEditorRevealType.InCenter);
      }
    }),
    vscode.commands.registerCommand("semio.navigateToDefinition", async (fileUri: string, startLine?: number) => {
      if (!fileUri) return;
      const uri = vscode.Uri.parse(fileUri);
      const doc = await vscode.workspace.openTextDocument(uri);
      const editor = await vscode.window.showTextDocument(doc);
      if (startLine != null && startLine > 0) {
        const position = new vscode.Position(startLine - 1, 0);
        editor.selection = new vscode.Selection(position, position);
        editor.revealRange(new vscode.Range(position, position), vscode.TextEditorRevealType.InCenter);
      }
    }),
    vscode.commands.registerCommand("semio.fixViolation", fixViolation),
    vscode.commands.registerCommand("semio.analyze", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      runRepoCommand("analyze @semio");
      vscode.window.showInformationMessage("Running semio analyze...");
    }),
    vscode.commands.registerCommand("semio.analyzeFile", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      runRepoCommand(`analyze ${relativePath}`);
    }),
    vscode.commands.registerCommand("semio.fix", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      runRepoCommand("fix @semio");
      vscode.window.showInformationMessage("Running semio fix...");
    }),
    vscode.commands.registerCommand("semio.fixFile", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      runRepoCommand(`fix ${relativePath}`);
    }),
    vscode.commands.registerCommand("semio.policyList", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      runRepoCommand("policy list");
    }),
    vscode.commands.registerCommand("semio.ticketOpen", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }

      // Smart wizard: pick goal first
      const goal = await pickGoal();
      if (goal === undefined) return;

      const title = await vscode.window.showInputBox({
        prompt: "Enter a titleized title for the ticket (e.g. \"Some Title on Something\"): ",
        placeHolder: "Some Title on Something",
      });
      if (!title) return;

      const prompt = await vscode.window.showInputBox({
        prompt: "Enter ticket prompt: ",
        placeHolder: "Describe the task...",
        value: title,
      });
      if (!prompt) return;

      const ui = await pickClient();
      if (!ui) return;

      const llm = await pickLLM();
      if (!llm) return;

      let cmd = `ticket open "${title.replace(/"/g, '\\"')}" "${prompt.replace(/"/g, '\\"')}" ${ui} ${llm}`;

      if (goal) {
        cmd += ` --goal "${goal.replace(/"/g, '\\"')}"`;
      }
      runRepoCommand(cmd);
    }),
    vscode.commands.registerCommand("semio.goalOpen", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const title = await vscode.window.showInputBox({
        prompt: "Enter goal title",
        placeHolder: "Goal Title",
      });
      if (!title) return;

      const description = await vscode.window.showInputBox({
        prompt: "Enter goal description",
        placeHolder: "Goal Description",
      });
      if (!description) return;

      const prompt = await vscode.window.showInputBox({
        prompt: "Enter goal prompt",
        placeHolder: "Goal Prompt",
      });
      if (!prompt) return;

      const ui = await pickClient();
      if (!ui) return;

      const llm = await pickLLM();
      if (!llm) return;

      const dueDate = await vscode.window.showInputBox({
        prompt: "Enter goal due date (YYYY-MM-DD) (optional)",
        placeHolder: "2026-02-15",
      });

      let cmd = `goal open "${title.replace(/"/g, '\\"')}" "${description.replace(/"/g, '\\"')}" "${prompt.replace(/"/g, '\\"')}" ${ui} ${llm}`;
      if (dueDate) {
        cmd += ` --due "${dueDate.replace(/"/g, '\\"')}"`;
      }
      runRepoCommand(cmd);
    }),
    vscode.commands.registerCommand("semio.goalList", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      runRepoCommand("goal list");
    }),
    vscode.commands.registerCommand("semio.ticketList", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      runRepoCommand("ticket list");
    }),
    vscode.commands.registerCommand("semio.ticketClose", async (ticketItem?: TicketItem | TicketData | ContributorTicketItem | ContributorTicketData) => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const resolvedTicket = resolveTicketData(ticketItem) ?? (await pickTicket("open"));
      if (!resolvedTicket) return;
      const summary = await vscode.window.showInputBox({
        prompt: "Enter a summary for this ticket",
        placeHolder: "Summary of the work completed",
      });
      if (!summary) {
        vscode.window.showWarningMessage("Summary is required to finish a ticket");
        return;
      }

      const activeFile = getActiveFileRelativePath();
      const files = await pickFiles(activeFile ? [activeFile] : undefined);

      if (!files || files.length === 0) {
        vscode.window.showWarningMessage("At least one file is required to finish a ticket");
        return;
      }
      runRepoCommand(
        `ticket close ${resolvedTicket.year}/${resolvedTicket.month}/${resolvedTicket.day}/${resolvedTicket.slug} "${summary.replace(/"/g, '\\"')}" ${files
          .map((f) => `"${f.replace(/"/g, '\\"')}"`)
          .join(" ")}`
      );
    }),
    vscode.commands.registerCommand("semio.ticketReopen", async (ticketItem?: TicketItem | TicketData | ContributorTicketItem | ContributorTicketData) => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const resolvedTicket = resolveTicketData(ticketItem) ?? (await pickTicket("closed"));
      if (!resolvedTicket) return;
      const prompt = await vscode.window.showInputBox({
        prompt: "Enter ticket prompt: ",
        placeHolder: "Describe the task...",
        value: ("frontmatter" in resolvedTicket ? resolvedTicket.frontmatter.prompt : undefined) ?? resolvedTicket.slug,
      });
      if (!prompt) return;

      const ui = await pickClient();
      if (!ui) return;

      const llm = await pickLLM();
      if (!llm) return;

      const cmd = `ticket reopen ${resolvedTicket.year}/${resolvedTicket.month}/${resolvedTicket.day}/${resolvedTicket.slug} "${prompt.replace(/"/g, '\\"')}" ${ui} ${llm}`;
      runRepoCommand(cmd);
    }),
    vscode.commands.registerCommand("semio.goalClose", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const goal = await pickGoal();
      if (!goal) return;

      const summary = await vscode.window.showInputBox({
        prompt: "Enter summary",
        placeHolder: "Reason for closing...",
      });
      if (!summary) return;

      runRepoCommand(`goal close "${goal.replace(/"/g, '\\"')}" "${summary.replace(/"/g, '\\"')}"`);
    }),
    vscode.commands.registerCommand("semio.goalReopen", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const goal = await pickGoal();
      if (!goal) return;

      const prompt = await vscode.window.showInputBox({
        prompt: "Enter prompt",
        placeHolder: "Reason for reopening...",
      });
      if (!prompt) return;

      const ui = await pickClient();
      if (!ui) return;

      const llm = await pickLLM();
      if (!llm) return;

      runRepoCommand(`goal reopen "${goal.replace(/"/g, '\\"')}" "${prompt.replace(/"/g, '\\"')}" ${ui} ${llm}`);
    }),
    vscode.commands.registerCommand("semio.ticketTree", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      runRepoCommand("ticket tree");
    }),
    vscode.commands.registerCommand("semio.projectList", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      runRepoCommand("bundle list");
    }),
    vscode.commands.registerCommand("semio.contributorAdd", async () => {
      const github = await vscode.window.showInputBox({ prompt: "GitHub username" });
      if (!github) return;
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      runRepoCommand(`contributor add ${github}`);
    }),
    vscode.commands.registerCommand("semio.contributorList", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      runRepoCommand("contributor list");
    }),
    vscode.commands.registerCommand("semio.contributorRemove", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const contributors = await fetchContributorsViaGraphQL();
      const items = contributors
        .filter((c: any) => c.github)
        .map((c: any) => ({
          label: c.github,
          description: c.email,
          detail: c.name
        }));

      const selected = await vscode.window.showQuickPick(items, {
        placeHolder: "Select contributor to remove"
      });
      if (!selected) return;

      runRepoCommand(`contributor remove ${selected.label}`);
    }),
    vscode.commands.registerCommand("semio.sectionTree", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      runRepoCommand(`section tree ${relativePath}`);
    }),
    vscode.commands.registerCommand("semio.sectionOpen", async (item?: SectionItem) => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || !item) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      const position = new vscode.Position(Math.max(0, item.section.startLine - 1), 0);
      const shownEditor = await vscode.window.showTextDocument(editor.document, { selection: new vscode.Range(position, position) });
      shownEditor.revealRange(new vscode.Range(position, position), vscode.TextEditorRevealType.InCenter);
    }),
    vscode.commands.registerCommand("semio.sectionRename", async (item?: SectionItem) => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || !item) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const newName = await vscode.window.showInputBox({
        prompt: getUiString("sectionsRenamePrompt"),
        value: item.section.name,
      });
      if (!newName) return;
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      const parts = item.sectionPath.split("/").filter(Boolean);
      parts[parts.length - 1] = newName;
      runRepoCommand(`section move ${relativePath} ${item.sectionPath} ${parts.join("/")}`);
      sectionsProvider.refresh();
    }),
    vscode.commands.registerCommand("semio.sectionCreateChild", async (item?: SectionItem) => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || !item) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const childName = await vscode.window.showInputBox({
        prompt: getUiString("sectionsCreateChildPrompt"),
      });
      if (!childName) return;
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      runRepoCommand(`section create ${relativePath} ${item.sectionPath}/${childName}`);
      sectionsProvider.refresh();
    }),
    vscode.commands.registerCommand("semio.sectionRemove", async (item?: SectionItem) => {
      const editor = vscode.window.activeTextEditor;
      if (!editor || !item) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const confirmPath = await vscode.window.showInputBox({
        prompt: getUiString("sectionsDeleteConfirm"),
        value: item.sectionPath,
      });
      if (!confirmPath) return;
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      runRepoCommand(`section delete ${relativePath} ${confirmPath}`);
      sectionsProvider.refresh();
    }),
    vscode.commands.registerCommand("semio.definitionList", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      runRepoCommand(`definition list ${relativePath}`);
    }),
    vscode.commands.registerCommand("semio.folderTree", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const folderUri = await vscode.window.showOpenDialog({
        canSelectFiles: false,
        canSelectFolders: true,
        canSelectMany: false,
        openLabel: "Select Folder",
        title: "Select folder to show tree",
      });
      if (!folderUri || folderUri.length === 0) return;
      const root = getWorkspaceRoot();
      if (!root) return;
      const folderPath = path.relative(root, folderUri[0].fsPath).replace(/\\/g, "/");
      runRepoCommand(`folder tree ${folderPath}`);
    }),
    vscode.commands.registerCommand("semio.folderCreate", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const folderPath = await vscode.window.showInputBox({
        prompt: "Enter folder path to create (relative to workspace root)",
        placeHolder: "js/js/new-folder",
      });
      if (!folderPath) return;
      runRepoCommand(`folder create ${folderPath}`);
    }),
    vscode.commands.registerCommand("semio.folderMove", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const sourceUri = await vscode.window.showOpenDialog({
        canSelectFiles: false,
        canSelectFolders: true,
        canSelectMany: false,
        openLabel: "Select Source Folder",
        title: "Select source folder to move/rename",
      });
      if (!sourceUri || sourceUri.length === 0) return;
      const root = getWorkspaceRoot();
      if (!root) return;
      const sourcePath = path.relative(root, sourceUri[0].fsPath).replace(/\\/g, "/");

      const targetPath = await vscode.window.showInputBox({
        prompt: "Enter target folder path",
        placeHolder: "js/js/new-folder",
        value: sourcePath,
      });
      if (!targetPath) return;
      runRepoCommand(`folder move ${sourcePath} ${targetPath}`);
    }),
    vscode.commands.registerCommand("semio.folderDelete", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const folderUri = await vscode.window.showOpenDialog({
        canSelectFiles: false,
        canSelectFolders: true,
        canSelectMany: false,
        openLabel: "Select Folder",
        title: "Select folder to delete",
      });
      if (!folderUri || folderUri.length === 0) return;
      const root = getWorkspaceRoot();
      if (!root) return;
      const folderPath = path.relative(root, folderUri[0].fsPath).replace(/\\/g, "/");
      runRepoCommand(`folder delete ${folderPath}`);
    }),
    vscode.commands.registerCommand("semio.folderList", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const folderUri = await vscode.window.showOpenDialog({
        canSelectFiles: false,
        canSelectFolders: true,
        canSelectMany: false,
        openLabel: "Select Folder",
        title: "Select folder to list",
      });
      if (!folderUri || folderUri.length === 0) return;
      const root = getWorkspaceRoot();
      if (!root) return;
      const folderPath = path.relative(root, folderUri[0].fsPath).replace(/\\/g, "/");
      runRepoCommand(`folder list ${folderPath}`);
    }),
    vscode.commands.registerCommand("semio.fileCreate", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const filePath = await vscode.window.showInputBox({
        prompt: "Enter file path to create (relative to workspace root)",
        placeHolder: "js/js/new-file.ts",
      });
      if (!filePath) return;
      runRepoCommand(`file create ${filePath}`);
    }),
    vscode.commands.registerCommand("semio.fileMove", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const sourceUri = await vscode.window.showOpenDialog({
        canSelectFiles: true,
        canSelectFolders: false,
        canSelectMany: false,
        openLabel: "Select Source File",
        title: "Select source file to move/rename",
      });
      if (!sourceUri || sourceUri.length === 0) return;
      const root = getWorkspaceRoot();
      if (!root) return;
      const sourcePath = path.relative(root, sourceUri[0].fsPath).replace(/\\/g, "/");

      const targetPath = await vscode.window.showInputBox({
        prompt: "Enter target file path",
        placeHolder: "js/js/new-file.ts",
        value: sourcePath,
      });
      if (!targetPath) return;
      runRepoCommand(`file move ${sourcePath} ${targetPath}`);
    }),
    vscode.commands.registerCommand("semio.fileDelete", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const fileUri = await vscode.window.showOpenDialog({
        canSelectFiles: true,
        canSelectFolders: false,
        canSelectMany: false,
        openLabel: "Select File",
        title: "Select file to delete",
      });
      if (!fileUri || fileUri.length === 0) return;
      const root = getWorkspaceRoot();
      if (!root) return;
      const filePath = path.relative(root, fileUri[0].fsPath).replace(/\\/g, "/");
      runRepoCommand(`file delete ${filePath}`);
    }),
    vscode.commands.registerCommand("semio.fileList", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const folderUri = await vscode.window.showOpenDialog({
        canSelectFiles: false,
        canSelectFolders: true,
        canSelectMany: false,
        openLabel: "Select Folder",
        title: "Select folder to list files from",
      });
      if (!folderUri || folderUri.length === 0) return;
      const root = getWorkspaceRoot();
      if (!root) return;
      const folderPath = path.relative(root, folderUri[0].fsPath).replace(/\\/g, "/");
      runRepoCommand(`file list ${folderPath}`);
    }),
    vscode.commands.registerCommand("semio.fileTree", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const folderUri = await vscode.window.showOpenDialog({
        canSelectFiles: false,
        canSelectFolders: true,
        canSelectMany: false,
        openLabel: "Select Folder",
        title: "Select folder to show file tree",
      });
      if (!folderUri || folderUri.length === 0) return;
      const root = getWorkspaceRoot();
      if (!root) return;
      const folderPath = path.relative(root, folderUri[0].fsPath).replace(/\\/g, "/");
      runRepoCommand(`file tree ${folderPath}`);
    }),
    vscode.commands.registerCommand("semio.sectionCreate", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      const sectionPath = await vscode.window.showInputBox({
        prompt: "Enter section path (e.g., MySection/SubSection)",
        placeHolder: "MySection",
      });
      if (!sectionPath) return;
      runRepoCommand(`section create ${relativePath} ${sectionPath}`);
    }),
    vscode.commands.registerCommand("semio.sectionMove", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      const sourcePath = await vscode.window.showInputBox({
        prompt: "Enter source section path",
        placeHolder: "OldSection",
      });
      if (!sourcePath) return;
      const targetPath = await vscode.window.showInputBox({
        prompt: "Enter target section path",
        placeHolder: "NewSection",
      });
      if (!targetPath) return;
      runRepoCommand(`section move ${relativePath} ${sourcePath} ${targetPath}`);
    }),
    vscode.commands.registerCommand("semio.sectionDelete", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      const sectionPath = await vscode.window.showInputBox({
        prompt: "Enter section path to delete",
        placeHolder: "SectionToDelete",
      });
      if (!sectionPath) return;
      runRepoCommand(`section delete ${relativePath} ${sectionPath}`);
    }),
    vscode.commands.registerCommand("semio.sectionIntegrate", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const source = await vscode.window.showInputBox({
        prompt: "Enter source file path",
        placeHolder: "path/to/source/file",
      });
      if (!source) return;

      const targetSection = await vscode.window.showInputBox({
        prompt: "Enter target section name",
        placeHolder: "TargetSectionName",
      });
      if (!targetSection) return;

      const targetFile = await vscode.window.showInputBox({
        prompt: "Enter target file path",
        placeHolder: "path/to/target/file",
        value: getActiveFileRelativePath() || "",
      });
      if (!targetFile) return;

      const targetParent = await vscode.window.showInputBox({
        prompt: "Enter target parent section name (optional)",
        placeHolder: "ParentSectionName",
      });

      const cmd = targetParent
        ? `section integrate "${source}" "${targetSection}" "${targetFile}" "${targetParent}"`
        : `section integrate "${source}" "${targetSection}" "${targetFile}"`;
      runRepoCommand(cmd);
    }),
    vscode.commands.registerCommand("semio.sectionList", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      runRepoCommand(`section list --file "${relativePath}"`);
    }),
    vscode.commands.registerCommand("semio.definitionTree", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      runRepoCommand(`definition list --file "${relativePath}"`);
    }),
    vscode.commands.registerCommand("semio.projectTree", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      runRepoCommand("bundle tree");
    }),
    vscode.commands.registerCommand("semio.policyCheck", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary not found in go/repo/");
        return;
      }
      const policy = await pickPolicy();
      if (!policy) return;
      runRepoCommand(`policy check ${policy.id}`);
    }),
    vscode.commands.registerCommand("semio.refreshDiagnostics", async () => {
      vscode.workspace.textDocuments.forEach(validateKitDocument);
      await Promise.all(vscode.workspace.textDocuments.map(analyzeFile));
      vscode.window.showInformationMessage("semio diagnostics refreshed");
    }),
  );
}

// #endregion Commands

// #region Activation

let outputChannel: vscode.OutputChannel;

function log(...args: any[]): void {
  const message = args.map(a => typeof a === 'object' ? JSON.stringify(a, null, 2) : String(a)).join(' ');
  outputChannel?.appendLine(message);
  try {
    fs.appendFileSync("/workspaces/semio/js/vscode/activation.log", "[LOG] " + message + "\n");
  } catch (e) { }
}

function logError(...args: any[]): void {
  const message = args.map(a => typeof a === 'object' ? JSON.stringify(a, null, 2) : String(a)).join(' ');
  outputChannel?.appendLine('[ERROR] ' + message);
  try {
    fs.appendFileSync("/workspaces/semio/js/vscode/activation.log", "[ERROR] " + message + "\n");
  } catch (e) { }
}

export function activate(context: vscode.ExtensionContext) {
  fs.writeFileSync("/workspaces/semio/js/vscode/activation.log", "ACTIVATION STARTED at " + new Date().toISOString() + "\n");
  outputChannel = vscode.window.createOutputChannel("semio");
  context.subscriptions.push(outputChannel);

  log("[ACTIVATION] semio extension activated");
  outputChannel.show(true);

  try {
    log("[ACTIVATION] Registering sidebar views...");
    try {
      registerSidebarViews(context);
    } catch (e) {
      logError("[ACTIVATION] registerSidebarViews failed", e);
    }

    log("[ACTIVATION] Registering commands...");
    try {
      registerCommands(context);
    } catch (e) {
      logError("[ACTIVATION] registerCommands failed", e);
    }

    kitDiagnosticCollection = vscode.languages.createDiagnosticCollection(DIAGNOSTIC_SOURCE);
    repoDiagnosticCollection = vscode.languages.createDiagnosticCollection(DIAGNOSTIC_SOURCE);
    context.subscriptions.push(kitDiagnosticCollection, repoDiagnosticCollection);

    context.subscriptions.push(vscode.window.onDidChangeActiveTextEditor((editor) => pinDiagnosticPreview(editor)));
    pinDiagnosticPreview(vscode.window.activeTextEditor);

    context.subscriptions.push(
      vscode.workspace.onDidOpenTextDocument((document) => {
        if (shouldAnalyzeFile(document)) {
          analyzeFile(document);
        }
        if (isKitDocument(document)) {
          validateKitDocument(document);
        }
      }),
      vscode.workspace.onDidChangeTextDocument((e) => {
        if (isKitDocument(e.document)) {
          validateKitDocument(e.document);
        }
      }),
      vscode.workspace.onDidSaveTextDocument((document) => {
        if (shouldAnalyzeFile(document)) {
          analyzeFile(document);
        }
        if (isKitDocument(document)) {
          validateKitDocument(document);
        }
      }),
      vscode.workspace.onDidCloseTextDocument((doc) => {
        kitDiagnosticCollection.delete(doc.uri);
        const root = getWorkspaceRoot();
        if (!root) return;
        if (doc.uri.scheme !== "file") return;
        const relativePath = path.relative(root, doc.uri.fsPath).replace(/\\/g, "/");
        if (relativePath.startsWith("..")) return;
        const fileUri = vscode.Uri.file(path.join(root, relativePath));
        const processKey = `analyze:${relativePath}`;
        const controller = runningProcesses.get(processKey);
        if (controller) {
          controller.abort();
          runningProcesses.delete(processKey);
        }
        fileViolationsMap.delete(fileUri.toString());
        repoDiagnosticCollection.delete(fileUri);
      }),
    );

    // Initial diagnostics run - DON'T block activation
    setTimeout(() => {
      log("[ACTIVATION] Starting initial diagnostics background task...");
      vscode.workspace.textDocuments.forEach((document) => {
        if (shouldAnalyzeFile(document)) {
          analyzeFile(document);
        }
        if (isKitDocument(document)) {
          validateKitDocument(document);
        }
      });
    }, 100);

    context.subscriptions.push(
      vscode.languages.registerCodeActionsProvider({ language: SEMIO_KIT_LANGUAGE }, new KitCodeActionProvider(), { providedCodeActionKinds: [vscode.CodeActionKind.QuickFix] }),
      vscode.languages.registerCodeActionsProvider("*", new RepoCodeActionProvider(), { providedCodeActionKinds: [vscode.CodeActionKind.QuickFix] }),
    );

    log("[ACTIVATION] Triggering codebase load in background...");
    loadCodebase().then((codebase) => {
      if (codebase) {
        log("[ACTIVATION] Codebase load finished successfully");
      }
    }).catch(err => {
      logError("[ACTIVATION] codebaseLoadPromise REJECTED:", err);
    });

    context.subscriptions.push(
      vscode.window.onDidChangeActiveTextEditor(() => sectionsProvider?.refresh()),
      vscode.workspace.onDidChangeTextDocument((event) => {
        if (vscode.window.activeTextEditor?.document.uri.toString() === event.document.uri.toString()) {
          sectionsProvider?.refresh();
        }
      }),
    );

    log("[ACTIVATION] Activation sequence COMPLETED.");
  } catch (error) {
    logError("[ACTIVATION] FATAL CRASH during activation:", error);
  }
}

export function deactivate() {
  for (const controller of runningProcesses.values()) {
    controller.abort();
  }
  runningProcesses.clear();
}

// #endregion Activation
