// #region 🔖Header
// [🧰repo🖱️vscode💻extension](repo://p/i/repo/b/u/vscode/f/extension.ts)

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

// VS Code extension providing monorepo navigation, analysis and commands.

// #endregion 🔖Header

// #region 🔖Imports
// [🧰repo🖱️vscode💻extension🔖imports](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Imports)
// Imports MUST include VS Code API, Node.js utilities, and semio validation.

// import { deserializeKit, Problem, validateKit } from "@semio/js/semio";
import { exec, execFile } from "child_process";
import * as fs from "fs";
import * as path from "path";
import { promisify } from "util";
import * as vscode from "vscode";

/**
 * [🧰repo🖱️vscode💻extension🔖imports🪨execasync](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Imports/d/i/execAsync)
 * execAsync holds the data fields for a execAsync record.
 **/
const execAsync = promisify(exec);
/**
 * [🧰repo🖱️vscode💻extension🔖imports🪨execfileasync](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Imports/d/i/execFileAsync)
 * execFileAsync holds the data fields for a execFileAsync record.
 **/
const execFileAsync = promisify(execFile);

// [🧰repo🖱️vscode💻extension🔖imports✂️problem](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Imports/d/i/Problem)
// Problem holds the data fields for a Problem record.
type Problem = { message: string };

// [🧰repo🖱️vscode💻extension🔖imports🛠️deserializekit](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Imports/d/i/deserializeKit)
// deserializeKit holds the data fields for a deserializeKit record.
function deserializeKit(text: string): unknown {
  return JSON.parse(text);
}

// validateKit holds the data fields for a validateKit record.
// [🧰repo🖱️vscode💻extension🔖imports🛠️validatekit](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Imports/d/i/validateKit)
function validateKit(_kit: unknown): { problems: Problem[] } {
  return { problems: [] };
}

/**
 * Structured event emitted by the repo CLI binary.
 * [🧰repo🖱️vscode💻extension🔖imports🛠️repoevent](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Imports/d/i/RepoEvent)
 **/
export type RepoEvent = {
  kind: string;
  data?: unknown;
  result?: unknown;
  error?: { message?: string; fatal?: boolean };
  done?: { exit_code?: number };
};
// #endregion 🔖Imports

// #region 🔖Constants
// [🧰repo🖱️vscode💻extension🔖constants](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Constants)
// Constants MUST define static configuration for diagnostics and UI strings.

/**
 * [🧰repo🖱️vscode💻extension🔖constants🪨semiokitlanguage](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Constants/d/i/SEMIO_KIT_LANGUAGE)
 * SEMIO_KIT_LANGUAGE holds the data fields for a SEMIO_KIT_LANGUAGE record.
 **/
const SEMIO_KIT_LANGUAGE = "json";
/**
 * [🧰repo🖱️vscode💻extension🔖constants🪨diagnosticsource](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Constants/d/i/DIAGNOSTIC_SOURCE)
 * DIAGNOSTIC_SOURCE holds the data fields for a DIAGNOSTIC_SOURCE record.
 **/
const DIAGNOSTIC_SOURCE = "semio";

/**
 * [🧰repo🖱️vscode💻extension🔖constants🪨uistrings](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Constants/d/i/UI_STRINGS)
 * UI_STRINGS holds the data fields for a UI_STRINGS record.
 **/
const UI_STRINGS = {
  en: {
    sectionsEmpty: "No sections found",
    sectionsNoActiveFile: "No active file",
    sectionsRenamePrompt: "Enter new section name",
    sectionsCreateChildPrompt: "Enter child section name",
    sectionsDeleteConfirm: "Enter section path to delete",
  },
  de: {
    sectionsRenamePrompt: "Neuen Abschnittsnamen eingeben",
    sectionsCreateChildPrompt: "Name des Unterabschnitts eingeben",
    sectionsDeleteConfirm: "Abschnittspfad zum Löschen eingeben",
  },
};

// #region 🔖Entity Emoji Registry
// [🧰repo🖱️vscode💻extension🔖constants🔖entityemojiregistry](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Constants/s/Entity%20Emoji%20Registry)
// Entity Emoji Registry MUST contain all entity-identifying emojis used in IDs.
// This registry drives CodeLens detection, gutter decorations, and ID parsing.
// It MUST be kept in sync with the CLI AllEntityEmojis() function.

/**
 * Complete set of entity-identifying emojis that appear as kind prefixes in entity IDs.
 * Each entry maps an emoji (after VS16 normalization) to its entity kind name.
 * This is the single source of truth — regex patterns are derived from it.
 * [🧰repo🖱️vscode💻extension🔖constants🔖entityemojiregistry🪨entityemojis](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Constants/s/Entity%20Emoji%20Registry/d/i/ENTITY_EMOJIS)
 **/
export const ENTITY_EMOJIS: ReadonlyMap<string, string> = new Map([
  ["👤", "technology-user"],
  ["🧰", "technology-infrastructure"],
  ["🔬", "technology-research"],
  ["🌱", "technology-mono"],

  ["📚", "bundle-library"],
  ["🛂", "bundle-schema"],
  ["⌨️", "bundle-binary"],
  ["🖱️", "bundle-ui"],
  ["📔", "bundle-example"],
  ["🌐", "bundle-site"],
  ["🏪", "bundle-assets"],
  ["🪆", "bundle-repo"],

  ["🗃️", "folder-organization"],
  ["🛅", "folder-required"],

  ["💻", "file-code"],
  ["🥼", "file-lab"],
  ["📜", "file-script"],
  ["📃", "file-docs"],
  ["⚙️", "file-config"],
  ["💾", "file-resource"],
  ["📋", "file-template"],
  ["⚖️", "file-license"],

  ["📌", "line"],

  ["🔖", "section"],

  ["🛠️", "definition-implementation"],
  ["✂️", "definition-interface"],
  ["🪨", "definition-constant"],
  ["🧪", "definition-test"],

  ["🎆", "year"],
  ["🌙", "month"],
  ["☀️", "day"],
  ["⏰", "hour"],
  ["⌚", "minute"],
  ["⏱️", "second"],

  ["🎯", "goal"],
  ["🎫", "ticket"],
  ["📝", "draft"],

  ["👮", "policy"],
  ["🚫", "breach"],
  ["🔍", "breach-scope"],

  ["🧑‍💻", "contributor"],

  ["🔀", "checkpoint"],

  ["✏️", "interaction-edited"],
  ["✅", "interaction-finished"],
  ["🔁", "interaction-restarted"],
  ["🗑️", "interaction-deleted"],

  ["⚪", "session"],
  ["🟡", "session-running"],
  ["🟢", "session-completed"],
  ["🔴", "session-interrupted"],

  ["🖥️", "codebase"],
  ["🏗️", "technologies"],
  ["📦", "bundles"],
  ["📁", "folders"],
  ["📄", "files"],
  ["🏷️", "definitions"],
]);

/**
 * Escapes a string for safe use inside a regular expression character class or alternation.
 * [🧰repo🖱️vscode💻extension🔖constants🔖entityemojiregistry🛠️escaperegex](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Constants/s/Entity%20Emoji%20Registry/d/i/escapeRegex)
 **/
function escapeRegex(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/**
 * Builds a regex pattern that matches any entity emoji from the registry.
 * Longer emojis are tried first to prevent partial matches (e.g. 🧑‍💻 before 🧑).
 * [🧰repo🖱️vscode💻extension🔖constants🔖entityemojiregistry🛠️buildentityemojipattern](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Constants/s/Entity%20Emoji%20Registry/d/i/buildEntityEmojiPattern)
 **/
export function buildEntityEmojiPattern(): string {
  const emojis = Array.from(ENTITY_EMOJIS.keys());

  emojis.sort((a, b) => b.length - a.length);
  return emojis.map(escapeRegex).join("|");
}

/**
 * Regex that matches entity IDs in text.
 * Matches either:
 * 1. Markdown link: [<emoji-id>](repo://...)
 * 2. Bare reference: <emoji-id> (emoji followed by non-whitespace, non-delimiter characters)
 * Built dynamically from the ENTITY_EMOJIS registry.
 **/
export function buildEntityIdRegex(): RegExp {
  const emojiAlt = buildEntityEmojiPattern();

  //  Group 1+2: Markdown link [<id>](repo://uri) — group(1)=id, group(2)=uri
  //  Group 3: Bare emoji-prefixed ID (emoji followed by non-delimiter text)
  return new RegExp("(?:\\[((?:" + emojiAlt + ")[^\\]]+)\\]\\((repo:\\/\\/[^)]+)\\)|((?:" + emojiAlt + ")[^\\s/\"'\\[\\]()]+))", "gu");
}

/**
 * Compiled entity ID regex, built once from the registry.
 * [🧰repo🖱️vscode💻extension🔖constants🔖entityemojiregistry🪨entityidregex](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Constants/s/Entity%20Emoji%20Registry/d/i/ENTITY_ID_REGEX)
 **/
export const ENTITY_ID_REGEX = buildEntityIdRegex();

// #endregion 🔖Entity Emoji Registry

// #endregion 🔖Constants

// #region 🔖Types
// [🧰repo🖱️vscode💻extension🔖types](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types)
// Types MUST define interfaces for repo events, tool results, and data models.

/**
 * Structured output from a repo CLI tool invocation.
 * [🧰repo🖱️vscode💻extension🔖types🛠️toolresult](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/ToolResult)
 **/
export interface ToolResult<T = unknown> {
  output: { lines: { type: string; text: string }[]; exitCode: number };
  data?: T;
  error?: string;
}

/**
 * NX technology metadata for a workspace package.
 * [🧰repo🖱️vscode💻extension🔖types🛠️technologydata](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/TechnologyData)
 **/
export interface TechnologyData {
  name: string;
  kind?: string;
  root: string;
  sourceRoot?: string;
  projectType?: string;
  tags?: string[];
}

/**
 * Code policy configuration with id, name, and description.
 * [🧰repo🖱️vscode💻extension🔖types🛠️policydata](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/PolicyData)
 **/
export interface PolicyData {
  id: string;
  name: string;
  description: string;
}

/**
 * YAML frontmatter fields parsed from a ticket markdown file.
 * [🧰repo🖱️vscode💻extension🔖types🛠️ticketfrontmatter](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/TicketFrontmatter)
 **/
export interface TicketFrontmatter {
  status: string;
  prompt: string;
  summary?: string;
  author?: string;
  checkpoint?: string;
  started?: string;
  finished?: string;
  ignore?: boolean;
}

/**
 * Single interaction record within a ticket lifecycle.
 * [🧰repo🖱️vscode💻extension🔖types🛠️ticketinteraction](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/TicketInteraction)
 **/
export interface TicketInteraction {
  prompt: string;
  llm: string;
  client: string;
  author: string;
  date: string;
  checkpoint: string;
}

/**
 * Full ticket data including date, slug, frontmatter, and interactions.
 * [🧰repo🖱️vscode💻extension🔖types🛠️ticketdata](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/TicketData)
 **/
export interface TicketData {
  year: number;
  month: number;
  day: number;
  slug: string;
  frontmatter: TicketFrontmatter;
  folderPath: string;
  interactions?: TicketInteraction[];
}

/**
 * Line-level contribution metrics for added and removed lines.
 * [🧰repo🖱️vscode💻extension🔖types🛠️contributorlinemetrics](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/ContributorLineMetrics)
 **/
export interface ContributorLineMetrics {
  added: number;
  removed: number;
}

/**
 * Contributor metrics scoped to a single definition.
 * [🧰repo🖱️vscode💻extension🔖types🛠️contributordefinitiondata](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/ContributorDefinitionData)
 **/
export interface ContributorDefinitionData {
  name: string;
  lines: ContributorLineMetrics;
}

/**
 * Contributor metrics scoped to a file section and its definitions.
 * [🧰repo🖱️vscode💻extension🔖types🛠️contributorsectiondata](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/ContributorSectionData)
 **/
export interface ContributorSectionData {
  name: string;
  lines: ContributorLineMetrics;
  definitions: ContributorDefinitionData[];
}

/**
 * Contributor metrics scoped to a single file and its sections.
 * [🧰repo🖱️vscode💻extension🔖types🛠️contributorfiledata](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/ContributorFileData)
 **/
export interface ContributorFileData {
  name: string;
  lines: ContributorLineMetrics;
  sections: ContributorSectionData[];
}

/**
 * Contributor metrics scoped to a folder and its files.
 * [🧰repo🖱️vscode💻extension🔖types🛠️contributorfolderdata](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/ContributorFolderData)
 **/
export interface ContributorFolderData {
  name: string;
  lines: ContributorLineMetrics;
  files: ContributorFileData[];
}

/**
 * Contributor metrics scoped to a bundle and its folders.
 * [🧰repo🖱️vscode💻extension🔖types🛠️contributorbundledata](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/ContributorBundleData)
 **/
export interface ContributorBundleData {
  name: string;
  lines: ContributorLineMetrics;
  folders: ContributorFolderData[];
}

/**
 * Ticket metadata associated with a contributor.
 * [🧰repo🖱️vscode💻extension🔖types🛠️contributorticketdata](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/ContributorTicketData)
 **/
export interface ContributorTicketData {
  year: number;
  month: number;
  day: number;
  slug: string;
  status: string;
  title: string;
  summary: string;
  folderPath?: string;
}

/**
 * Checkpoint metadata associated with a contributor.
 * [🧰repo🖱️vscode💻extension🔖types🛠️contributorcheckpointdata](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/ContributorCheckpointData)
 **/
export interface ContributorCheckpointData {
  title: string;
  sha: string;
}

/**
 * Full contributor profile with contributions across bundles, tickets, and checkpoints.
 * [🧰repo🖱️vscode💻extension🔖types🛠️contributordata](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/ContributorData)
 **/
export interface ContributorData {
  github: string;
  name?: string;
  emails?: string[];
  links?: Record<string, string>;
  contributions?: {
    checkpoints: ContributorCheckpointData[];
    tickets: ContributorTicketData[];
    bundles: ContributorBundleData[];
  };
}

/**
 * [🧰repo🖱️vscode💻extension🔖types✂️textedit](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/TextEdit)
 * TextEdit holds the data fields for a TextEdit record.
 **/
interface TextEdit {
  start: number;
  end: number;
  newText: string;
}

/**
 * [🧰repo🖱️vscode💻extension🔖types✂️autofix](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/AutoFix)
 * AutoFix holds the data fields for a AutoFix record.
 **/
interface AutoFix {
  description: string;
  edits: Record<string, TextEdit[]>;
}

/**
 * [🧰repo🖱️vscode💻extension🔖types✂️breach](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/Breach)
 * Breach holds the data fields for a Breach record.
 **/
interface Breach {
  id: string;
  summary: string;
  kind: { id: string };
  scope: string;
  line?: number;
  column?: number;
  excerpt?: string;
  autofix?: AutoFix;
}

/**
 * AnalyzeReport holds the data fields for a AnalyzeReport record.
 * [🧰repo🖱️vscode💻extension🔖types✂️analyzereport](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/AnalyzeReport)
 **/
interface AnalyzeReport {
  timestamp: string;
  scope: string;
  breachs: Breach[];
}

/**
 * [🧰repo🖱️vscode💻extension🔖types✂️sectioninfo](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/SectionInfo)
 * SectionInfo holds the data fields for a SectionInfo record.
 **/
interface SectionInfo {
  name: string;
  kind: string;
  startLine: number;
  endLine: number;
  startIndex: number;
  endIndex: number;
  children: SectionInfo[];
}

/**
 * [🧰repo🖱️vscode💻extension🔖types✂️definitioninfo](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/DefinitionInfo)
 * DefinitionInfo holds the data fields for a DefinitionInfo record.
 **/
interface DefinitionInfo {
  name: string;
  startLine: number;
  endLine: number;
  endIndex: number;
}

/**
 * [🧰repo🖱️vscode💻extension🔖types✂️graphqlsectionrange](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/GraphqlSectionRange)
 * GraphqlSectionRange holds the data fields for a GraphqlSectionRange record.
 **/
interface GraphqlSectionRange {
  start?: number;
  end?: number;
}

/**
 * [🧰repo🖱️vscode💻extension🔖types✂️graphqlsection](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Types/d/i/GraphqlSection)
 * GraphqlSection holds the data fields for a GraphqlSection record.
 **/
interface GraphqlSection {
  name: string;
  __typename?: string;
  children?: GraphqlSection[] | null;
}

// #endregion 🔖Types

// #region 🔖Globals
// [🧰repo🖱️vscode💻extension🔖globals](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Globals)
// Globals MUST hold module-level state for output channel, diagnostics, caches, and providers.
let outputChannel: vscode.OutputChannel;
/**
 * [🧰repo🖱️vscode💻extension🔖globals🪨repodiagnosticcollection](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Globals/d/i/repoDiagnosticCollection)
 * repoDiagnosticCollection holds the data fields for a repoDiagnosticCollection record.
 **/
let repoDiagnosticCollection: vscode.DiagnosticCollection;
/**
 * [🧰repo🖱️vscode💻extension🔖globals🪨kitdiagnosticcollection](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Globals/d/i/kitDiagnosticCollection)
 * kitDiagnosticCollection holds the data fields for a kitDiagnosticCollection record.
 **/
let kitDiagnosticCollection: vscode.DiagnosticCollection;
/**
 * [🧰repo🖱️vscode💻extension🔖globals🪨filebreachsmap](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Globals/d/i/fileBreachsMap)
 * fileBreachsMap holds the data fields for a fileBreachsMap record.
 **/
const fileBreachsMap = new Map<string, Breach[]>();
/**
 * [🧰repo🖱️vscode💻extension🔖globals✂️bundleinfo](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Globals/d/i/BundleInfo)
 * BundleInfo holds the data fields for a BundleInfo record.
 **/
interface BundleInfo {
  id: string;
  root: string;
}
/**
 * [🧰repo🖱️vscode💻extension🔖globals🪨bundlecache](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Globals/d/i/bundleCache)
 * bundleCache holds the data fields for a bundleCache record.
 **/
let bundleCache: BundleInfo[] = [];
/**
 * [🧰repo🖱️vscode💻extension🔖globals🪨cachedrepobaseurl](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Globals/d/i/cachedRepoBaseUrl)
 * cachedRepoBaseUrl holds the data fields for a cachedRepoBaseUrl record.
 **/
let cachedRepoBaseUrl: string | undefined = undefined;
/**
 * [🧰repo🖱️vscode💻extension🔖globals🪨runningprocesses](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Globals/d/i/runningProcesses)
 * runningProcesses holds the data fields for a runningProcesses record.
 **/
const runningProcesses = new Map<string, AbortController>();

// [🧰repo🖱️vscode💻extension🔖globals🪨cliconcurrencylimit](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Globals/d/i/cliConcurrencyLimit)
// Maximum number of concurrent CLI subprocess spawns to prevent system overload.
const CLI_CONCURRENCY_LIMIT = 2;
let cliActiveCount = 0;
const cliWaitQueue: (() => void)[] = [];

function acquireCliSlot(): Promise<void> {
  if (cliActiveCount < CLI_CONCURRENCY_LIMIT) {
    cliActiveCount++;
    return Promise.resolve();
  }
  return new Promise<void>((resolve) => {
    cliWaitQueue.push(() => {
      cliActiveCount++;
      resolve();
    });
  });
}

function releaseCliSlot(): void {
  cliActiveCount--;
  const next = cliWaitQueue.shift();
  if (next) next();
}

/**
 * [🧰repo🖱️vscode💻extension🔖globals🪨filterprovider](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Globals/d/i/filterProvider)
 * filterProvider holds the data fields for a filterProvider record.
 **/
let filterProvider: FilterTreeDataProvider | undefined;
/**
 * [🧰repo🖱️vscode💻extension🔖globals🪨monorepoprovider](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Globals/d/i/monorepoProvider)
 * monorepoProvider holds the data fields for a monorepoProvider record.
 **/
let monorepoProvider: MonorepoTreeDataProvider | undefined;
// #endregion 🔖Globals

// #region 🔖Utilities

// [🧰repo🖱️vscode💻extension🔖utilities](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities)
// Utilities MUST provide shared functions for logging, shell execution, and binary resolution.

/**
 * [🧰repo🖱️vscode💻extension🔖utilities🛠️writelog](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/writeLog)
 * [🧰repo🖱️vscode💻extension🔖utilities🪨writelog](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/writeLog)
 * writeLog holds the data fields for a writeLog record.
 **/
function writeLog(level: string, args: any[]): void {
  const message = args.map((a) => (typeof a === "object" ? JSON.stringify(a, null, 2) : String(a))).join(" ");
  const prefix = level === "ERROR" ? "[ERROR] " : "";
  outputChannel?.appendLine(prefix + message);
  try {
    const logPath = path.join(getWorkspaceRoot() || "", "activation.log");
    fs.appendFileSync(logPath, `[${level}] ${message}\n`);
  } catch (e) { }
}

/**
 * [🧰repo🖱️vscode💻extension🔖utilities🛠️log](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/log)
 * log holds the data fields for a log record.
 **/
function log(...args: any[]): void {
  writeLog("LOG", args);
}

/**
 * logError holds the data fields for a logError record.
 * [🧰repo🖱️vscode💻extension🔖utilities🛠️logerror](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/logError)
 **/
function logError(...args: any[]): void {
  writeLog("ERROR", args);
}

/**
 * getWorkspaceRoot holds the data fields for a getWorkspaceRoot record.
 * [🧰repo🖱️vscode💻extension🔖utilities🛠️getworkspaceroot](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/getWorkspaceRoot)
 **/
function getWorkspaceRoot(): string | undefined {
  return vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
}

/** getRepoBinaryPath holds the data fields for a getRepoBinaryPath record.
 * [🧰repo🖱️vscode💻extension🔖utilities🛠️getrepobinarypath](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/getRepoBinaryPath)
 * [🧰repo🖱️vscode💻extension🔖utilities🪨getrepobinarypath](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/getRepoBinaryPath)
 **/
function getRepoBinaryPath(): string | undefined {
  const root = getWorkspaceRoot();
  if (!root) return undefined;
  const ext = process.platform === "win32" ? ".exe" : "";
  const candidate = path.join(root, "repo", "cli", `cli${ext}`);
  return fs.existsSync(candidate) ? candidate : undefined;
}

/**
 * execShell holds the data fields for a execShell record.
 * [🧰repo🖱️vscode💻extension🔖utilities🛠️execshell](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/execShell)
 **/
function execShell(cmd: string, cwd: string | undefined): Promise<string> {
  return new Promise((resolve, reject) => {
    exec(cmd, { cwd, maxBuffer: 1024 * 1024 * 10 }, (err, stdout, stderr) => {
      if (err) return reject(err);
      resolve(stdout);
    });
  });
}

/** getRepoCommand holds the data fields for a getRepoCommand record.
 * [🧰repo🖱️vscode💻extension🔖utilities🛠️getrepocommand](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/getRepoCommand)
 * [🧰repo🖱️vscode💻extension🔖utilities🪨getrepocommand](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/getRepoCommand)
 **/
function getRepoCommand(): string {
  const binaryPath = getRepoBinaryPath();
  return binaryPath ?? "";
}

/**
 * hasRepoAccess holds the data fields for a hasRepoAccess record.
 * [🧰repo🖱️vscode💻extension🔖utilities🛠️hasrepoaccess](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/hasRepoAccess)
 **/
export function hasRepoAccess(): boolean {
  return getRepoCommand() !== "";
}

/**
 * [🧰repo🖱️vscode💻extension🔖utilities🛠️getuistring](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/getUiString)
 * [🧰repo🖱️vscode💻extension🔖utilities🪨getuistring](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/getUiString)
 * getUiString holds the data fields for a getUiString record.
 **/
function getUiString(key: keyof typeof UI_STRINGS.en): string {
  const language = vscode.env.language.split("-")[0];
  const bundle = UI_STRINGS[language as keyof typeof UI_STRINGS] ?? UI_STRINGS.en;
  return (bundle as any)[key];
}

/**
 * [🧰repo🖱️vscode💻extension🔖utilities🛠️resolvecheckpointsha](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/resolveCheckpointSha)
 * resolveCheckpointSha holds the data fields for a resolveCheckpointSha record.
 **/
function resolveCheckpointSha(checkpoint: string | { sha?: string } | undefined): string | undefined {
  if (!checkpoint) return undefined;
  if (typeof checkpoint === "string") return checkpoint;
  return checkpoint.sha;
}

/** getGitHubRepoBaseUrl holds the data fields for a getGitHubRepoBaseUrl record.
 * [🧰repo🖱️vscode💻extension🔖utilities🛠️getgithubrepobaseurl](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/getGitHubRepoBaseUrl)
 * [🧰repo🖱️vscode💻extension🔖utilities🪨getgithubrepobaseurl](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/getGitHubRepoBaseUrl)
 **/
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

/**
 * [🧰repo🖱️vscode💻extension🔖utilities🛠️runrepocommand](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/runRepoCommand)
 * [🧰repo🖱️vscode💻extension🔖utilities🪨runrepocommand](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/runRepoCommand)
 * runRepoCommand holds the data fields for a runRepoCommand record.
 **/
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

/**
 * [🧰repo🖱️vscode💻extension🔖utilities🛠️runrepocommandjson](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/runRepoCommandJson)
 * [🧰repo🖱️vscode💻extension🔖utilities🪨runrepocommandjson](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/runRepoCommandJson)
 * runRepoCommandJson holds the data fields for a runRepoCommandJson record.
 **/
async function runRepoCommandJson<T>(args: string): Promise<T | null> {
  const root = getWorkspaceRoot();
  if (!root || !hasRepoAccess()) return null;
  const command = getRepoCommand();
  if (!command) return null;
  const fullCommand = `"${command}" --json ${args}`;
  await acquireCliSlot();
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
  } finally {
    releaseCliSlot();
  }
}

/**
 * Parses raw CLI output into structured repo events.
 * Implementations MUST split output by newlines and parse each non-empty line as JSON.
 * [🧰repo🖱️vscode💻extension🔖utilities🛠️parserepoevents](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/parseRepoEvents)
 **/
export function parseRepoEvents(output: string): RepoEvent[] {
  const lines = output
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.length > 0);
  return lines.map((line) => JSON.parse(line) as RepoEvent);
}

/**
 * Extracts the final result payload from a sequence of repo events.
 * Implementations MUST throw on fatal errors and return the last meaningful result.
 * [🧰repo🖱️vscode💻extension🔖utilities🛠️extractreporesult](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Utilities/d/i/extractRepoResult)
 **/
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

  if (results.length > 0 && results.some((r) => r && typeof r === "object" && "section" in r)) {
    const sections = results.map((r) => (r as any).section).filter((s) => s);
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
    return { data: res };
  }
  return { data: lastResult };
}

// #endregion 🔖Utilities

// #region 🔖URI Resolution

// [🧰repo🖱️vscode💻extension🔖uriresolution](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution)
// URI Resolution MUST handle parsing, tree node caching, and repo URI navigation.

/**
 * Tree node data structure representing a monorepo artifact in the sidebar tree.
 * [🧰repo🖱️vscode💻extension🔖uriresolution🛠️treenodedata](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/TreeNodeData)
 **/
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

/**
 * [🧰repo🖱️vscode💻extension🔖uriresolution🪨treenodecache](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/treeNodeCache)
 * treeNodeCache holds the data fields for a treeNodeCache record.
 **/
let treeNodeCache: Map<string, TreeNodeData> | null = null;
/**
 * [🧰repo🖱️vscode💻extension🔖uriresolution🪨treerootcache](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/treeRootCache)
 * treeRootCache holds the data fields for a treeRootCache record.
 **/
let treeRootCache: TreeNodeData | null = null;
/**
 * treeNodeCacheTime holds the data fields for a treeNodeCacheTime record.
 * [🧰repo🖱️vscode💻extension🔖uriresolution🪨treenodecachetime](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/treeNodeCacheTime)
 **/
let treeNodeCacheTime = 0;
/**
 * [🧰repo🖱️vscode💻extension🔖uriresolution🪨treecachettl](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/TREE_CACHE_TTL)
 * TREE_CACHE_TTL holds the data fields for a TREE_CACHE_TTL record.
 **/
const TREE_CACHE_TTL = 30000;

/**
 * Extracts the leading emoji characters from a text string.
 * Implementations MUST use Unicode emoji properties to detect the prefix.
 * [🧰repo🖱️vscode💻extension🔖uriresolution🛠️extractleadingemoji](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/extractLeadingEmoji)
 **/
export function extractLeadingEmoji(text: string): string {
  const match = text.match(/^[\p{Emoji_Presentation}\p{Extended_Pictographic}][\u{FE0E}\u{FE0F}\u{200D}\p{Emoji_Component}]*/u);
  return match ? match[0] : "";
}

/**
 * Computes the display label for a tree node including emoji prefix and status icon.
 * Implementations MUST prepend the node emoji and status indicator to the label.
 * [🧰repo🖱️vscode💻extension🔖uriresolution🛠️treenodedisplaylabel](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/treeNodeDisplayLabel)
 **/
export function treeNodeDisplayLabel(node: TreeNodeData): string {
  if (node.Kind === "category") return node.Label;
  const emoji = extractLeadingEmoji(node.ID);
  let statusIcon = "";
  if (node.Status === "open") statusIcon = "🔵";
  else if (node.Status === "closed") statusIcon = "🟢";
  const fallbackEmojis: Record<string, string> = {
    contributor: "🧑‍💻",
    checkpoint: "🔀",
    policy: "👮",
    statute: "⚠",
  };
  const prefix = emoji || fallbackEmojis[node.Kind] || "";
  let label = node.Label;
  if (prefix && label.startsWith(prefix)) {
    label = label.substring(prefix.length);
  }
  return `${prefix}${statusIcon}${label}`;
}

/**
 * Returns the VS Code context value for a tree node based on its kind and status.
 * Implementations MUST distinguish open and closed tickets.
 * [🧰repo🖱️vscode💻extension🔖uriresolution🛠️treenodecontextvalue](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/treeNodeContextValue)
 **/
export function treeNodeContextValue(node: TreeNodeData): string {
  if (node.Kind === "ticket") return node.Status === "open" ? "ticketOpen" : "ticketClosed";
  return node.Kind;
}

/**
 * Returns the VS Code command to execute when a tree node is clicked.
 * Implementations MUST return undefined for category nodes and navigate for others.
 * [🧰repo🖱️vscode💻extension🔖uriresolution🛠️treenodecommand](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/treeNodeCommand)
 **/
export function treeNodeCommand(node: TreeNodeData): vscode.Command | undefined {
  if (node.Kind === "category") return undefined;
  if (node.URI) return { command: "semio.navigate", title: "Navigate", arguments: [node.URI] };
  return undefined;
}

/**
 * Builds CLI tree command arguments from the current filter provider state.
 * Implementations MUST translate each filter toggle into the corresponding CLI flag.
 * [🧰repo🖱️vscode💻extension🔖uriresolution🛠️buildclitreeargs](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/buildCliTreeArgs)
 **/
export function buildCliTreeArgs(fp?: FilterTreeDataProvider): string[] {
  const args: string[] = [];
  if (!fp) return args;
  const query = fp.searchQuery?.trim();
  if (query) args.push(query);
  const ff = fp.filters.file;
  if (!ff.code) args.push("--no-code");
  if (!ff.script) args.push("--no-script");
  if (!ff.config) args.push("--no-config");
  if (!ff.lab) args.push("--no-lab");
  if (!ff.docs) args.push("--no-docs");
  if (!ff.resource) args.push("--no-resource");
  if (!ff.template) args.push("--no-template");
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
  if (Object.values(ff).every((v) => !v)) args.push("--no-file");
  if (!fp.filters.policy.all) args.push("--no-policy");
  if (!fp.filters.contributor.all) args.push("--no-contributor");
  if (!fp.filters.checkpoint.all) args.push("--no-checkpoint");
  const pf = fp.filters.technology;
  if (!pf.user && !pf.infrastructure && !pf.research) args.push("--no-technology");
  return args;
}

/**
 * Converts text to an uppercase slug with non-alphanumeric characters replaced by hyphens.
 * Implementations MUST uppercase the input and strip leading and trailing hyphens.
 * [🧰repo🖱️vscode💻extension🔖uriresolution🛠️slugify](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/slugify)
 **/
export function slugify(text: string): string {
  return text
    .toUpperCase()
    .replace(/[^A-Z0-9]+/g, "-")
    .replace(/^-|-$/g, "");
}

/**
 * [🧰repo🖱️vscode💻extension🔖uriresolution🛠️flattentree](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/flattenTree)
 * flattenTree holds the data fields for a flattenTree record.
 **/
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

/**
 * [🧰repo🖱️vscode💻extension🔖uriresolution🛠️gettreenodecache](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/getTreeNodeCache)
 * [🧰repo🖱️vscode💻extension🔖uriresolution🪨gettreenodecache](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/getTreeNodeCache)
 * getTreeNodeCache holds the data fields for a getTreeNodeCache record.
 **/
async function getTreeNodeCache(): Promise<Map<string, TreeNodeData>> {
  const now = Date.now();
  if (treeNodeCache && now - treeNodeCacheTime < TREE_CACHE_TTL) {
    return treeNodeCache;
  }
  const root = getWorkspaceRoot();
  const command = getRepoCommand();
  if (!root || !command) return new Map();
  await acquireCliSlot();
  try {
    const { stdout } = await execAsync(`"${command}" --json search`, { cwd: root, timeout: 60000, maxBuffer: 50 * 1024 * 1024 });
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
  } finally {
    releaseCliSlot();
  }
  return treeNodeCache ?? new Map();
}

/**
 * [🧰repo🖱️vscode💻extension🔖uriresolution🛠️gettreeroot](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/getTreeRoot)
 * getTreeRoot holds the data fields for a getTreeRoot record.
 **/
async function getTreeRoot(): Promise<TreeNodeData | null> {
  await getTreeNodeCache();
  return treeRootCache;
}

/**
 * [🧰repo🖱️vscode💻extension🔖uriresolution🛠️fetchtreewithargs](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/fetchTreeWithArgs)
 * [🧰repo🖱️vscode💻extension🔖uriresolution🪨fetchtreewithargs](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/fetchTreeWithArgs)
 * fetchTreeWithArgs holds the data fields for a fetchTreeWithArgs record.
 **/
async function fetchTreeWithArgs(args: string[]): Promise<TreeNodeData | null> {
  const root = getWorkspaceRoot();
  const command = getRepoCommand();
  if (!root || !command) return null;
  try {
    const fullArgs = ["--json", "search", ...args];
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

/**
 * Clears the cached tree node data forcing a fresh fetch on next access.
 * Implementations MUST reset all cache fields and the timestamp.
 * [🧰repo🖱️vscode💻extension🔖uriresolution🛠️invalidatetreenodecache](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/invalidateTreeNodeCache)
 **/
export function invalidateTreeNodeCache(): void {
  treeNodeCache = null;
  treeRootCache = null;
  treeNodeCacheTime = 0;
}

/**
 * Parses a repo URI into its type and path components.
 * Implementations MUST return null for URIs that do not match the repo scheme.
 * [🧰repo🖱️vscode💻extension🔖uriresolution🛠️parseuri](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/parseUri)
 **/
export function parseUri(uri: string): { type: string; path: string } | null {
  const match = uri.match(/^repo:\/\/([a-zA-Z]+)(?:\/(.*)?)?$/);
  if (!match) return null;
  return { type: match[1], path: match[2] ? decodeURIComponent(match[2]) : "" };
}

/**
 * [🧰repo🖱️vscode💻extension🔖uriresolution🛠️navigatetouri](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/navigateToUri)
 * [🧰repo🖱️vscode💻extension🔖uriresolution🪨navigatetouri](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/navigateToUri)
 * navigateToUri holds the data fields for a navigateToUri record.
 **/
async function navigateToUri(uri: string): Promise<void> {
  const wsRoot = getWorkspaceRoot();
  if (!wsRoot) return;
  const parsed = parseUri(uri);
  if (!parsed) return;

  const cache = await getTreeNodeCache();
  const node = cache.get(uri);

  switch (parsed.type) {
    case "repo": {
      return vscode.commands.executeCommand("semio.monorepo.focus") as any;
    }
    case "cb":
    case "technologies":
    case "bundles":
    case "tickets":
    case "goals":
    case "drafts":
    case "todos":
    case "policies":
    case "statutes":
    case "contributors":
    case "checkpoints":
    case "folders":
    case "files":
    case "sections":
    case "definitions": {
      return vscode.commands.executeCommand("semio.monorepo.focus") as any;
    }
    case "ticket": {
      let ticketPath = "";
      if (node && node.Year && node.Month && node.Day && node.ID) {
        const slug = node.ID.replace(/^[^\w]+/, "");
        const year = String(node.Year).padStart(2, "0");
        const month = String(node.Month).padStart(2, "0");
        const day = String(node.Day).padStart(2, "0");
        ticketPath = path.join(wsRoot, ".repo", "🎫", year, month, day, slug);
      } else {
        if (parsed.path.match(/^\d+\/\d+\/\d+\/.+/)) {
          ticketPath = path.join(wsRoot, ".repo", "🎫", parsed.path);
        } else {
          const slug = path.basename(parsed.path);
        }
      }

      if (ticketPath && fs.existsSync(ticketPath)) {
        return vscode.commands.executeCommand("semio.navigateToFile", ticketPath) as any;
      }
      break;
    }
    case "goal": {
      const goalJsonPath = path.join(wsRoot, ".repo", "🎯", parsed.path, "goal.json");
      if (fs.existsSync(goalJsonPath)) {
        return vscode.commands.executeCommand("semio.navigateToFile", goalJsonPath) as any;
      }
      break;
    }
    case "draft": {
      const slug = path.basename(parsed.path);
      const draftPath = path.join(wsRoot, ".repo", "✍️", slug);
      if (fs.existsSync(draftPath)) {
        return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(draftPath)) as any;
      }
      break;
    }
    case "todo": {
      const slug = path.basename(parsed.path);
      const todoPath = path.join(wsRoot, ".repo", "todos", slug);
      if (fs.existsSync(todoPath)) {
        return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(todoPath)) as any;
      }
      break;
    }
    case "contributor": {
      const github = path.basename(parsed.path);
      return vscode.env.openExternal(vscode.Uri.parse(`https://github.com/${github}`)) as any;
    }
    case "checkpoint": {
      const sha = path.basename(parsed.path);
      const baseUrl = getGitHubRepoBaseUrl();
      if (baseUrl) {
        return vscode.env.openExternal(vscode.Uri.parse(`${baseUrl}/commit/${sha}`)) as any;
      }
      break;
    }
    case "technology": {
      const abs = path.join(wsRoot, parsed.path);
      if (fs.existsSync(abs)) {
        return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(abs)) as any;
      }
      break;
    }
    case "bundle": {
      if (node?.Data?.root) {
        const abs = path.join(wsRoot, node.Data.root);
        if (fs.existsSync(abs)) {
          return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(abs)) as any;
        }
      }

      const parts = parsed.path.split("/");
      if (parts.length >= 2) {
      }
      break;
    }
    case "folder": {
      const abs = path.join(wsRoot, parsed.path);
      if (fs.existsSync(abs)) {
        return vscode.commands.executeCommand("revealInExplorer", vscode.Uri.file(abs)) as any;
      }
      break;
    }
    case "file": {
      const abs = path.join(wsRoot, parsed.path);
      if (fs.existsSync(abs)) {
        return vscode.commands.executeCommand("semio.navigateToFile", parsed.path) as any;
      }
      break;
    }
    case "section": {
      const parts = parsed.path.split("/");
      const filePathParts: string[] = [];
      const sectionParts: string[] = [];
      let foundFile = false;
      for (const part of parts) {
        if (!foundFile) {
          filePathParts.push(part);
          const candidatePath = filePathParts.join("/");
          const abs = path.join(wsRoot, candidatePath);
          if (fs.existsSync(abs) && fs.statSync(abs).isFile()) {
            foundFile = true;
          }
        } else {
          sectionParts.push(part);
        }
      }
      const filePath = filePathParts.join("/");
      const sectionPath = sectionParts.join("/");
      const binaryPath = getRepoBinaryPath();
      if (binaryPath) {
        await acquireCliSlot();
        let sectionResult: { filePath: string; startLine: number; endLine?: number } | undefined;
        try {
          const { stdout } = await execAsync(`"${binaryPath}" --json section list --file "${filePath}"`, { cwd: wsRoot, timeout: 15000 });
          const events = parseRepoEvents(stdout);
          for (const event of events) {
            const section = (event as any).section;
            if (section) {
              const found = findSectionByPath(section, sectionPath);
              if (found) {
                sectionResult = { filePath, startLine: found.startLine, endLine: found.endLine };
                break;
              }
            }
          }
        } catch {
        } finally {
          releaseCliSlot();
        }
        if (sectionResult) return openFileAtLine(sectionResult.filePath, sectionResult.startLine, sectionResult.endLine);
      }
      return vscode.commands.executeCommand("semio.navigateToFile", filePath) as any;
    }
    case "definition": {
      const parts = parsed.path.split("/");
      const filePathParts: string[] = [];
      let foundFile = false;
      let defName = "";
      for (const part of parts) {
        if (!foundFile) {
          filePathParts.push(part);
          const candidatePath = filePathParts.join("/");
          const abs = path.join(wsRoot, candidatePath);
          if (fs.existsSync(abs) && fs.statSync(abs).isFile()) {
            foundFile = true;
          }
        } else {
          defName = part;
        }
      }
      const filePath = filePathParts.join("/");
      if (defName) {
        const binaryPath = getRepoBinaryPath();
        if (binaryPath) {
          await acquireCliSlot();
          let defResult: { filePath: string; startLine: number; endLine?: number } | undefined;
          try {
            const { stdout } = await execAsync(`"${binaryPath}" --json definition list --file "${filePath}"`, { cwd: wsRoot, timeout: 15000 });
            const events = parseRepoEvents(stdout);
            for (const event of events) {
              const def = (event as any).definition;
              if (def && slugify(def.name) === slugify(defName) && def.startLine) {
                defResult = { filePath, startLine: def.startLine, endLine: def.endLine };
                break;
              }
            }
          } catch {
          } finally {
            releaseCliSlot();
          }
          if (defResult) return openFileAtLine(defResult.filePath, defResult.startLine, defResult.endLine);
        }
      }
      return vscode.commands.executeCommand("semio.navigateToFile", filePath) as any;
    }
    case "policy": {
      if (node) {
        vscode.window.showInformationMessage(`Policy: ${node.Label}${node.Description ? " - " + node.Description : ""}`);
      } else {
        vscode.window.showInformationMessage(`Policy: ${path.basename(parsed.path)}`);
      }
      break;
    }
    case "statute": {
      if (node) {
        vscode.window.showInformationMessage(`Breach Kind: ${node.Label}${node.Description ? " - " + node.Description : ""}`);
      } else {
        vscode.window.showInformationMessage(`Breach Kind: ${path.basename(parsed.path)}`);
      }
      break;
    }
  }
}

/**
 * [🧰repo🖱️vscode💻extension🔖uriresolution🛠️findsectionbypath](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/findSectionByPath)
 * [🧰repo🖱️vscode💻extension🔖uriresolution🪨findsectionbypath](repo://p/i/repo/b/u/vscode/f/extension.ts/s/URI%20Resolution/d/i/findSectionByPath)
 * findSectionByPath holds the data fields for a findSectionByPath record.
 **/
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
}

// #endregion 🔖URI Resolution

// #region 🔖Helpers

// [🧰repo🖱️vscode💻extension🔖helpers](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Helpers)
// Helpers MUST provide file path extraction, ticket path resolution, and editor navigation.

/**
 * [🧰repo🖱️vscode💻extension🔖helpers🛠️extractfilepathfromscope](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Helpers/d/i/extractFilePathFromScope)
 * [🧰repo🖱️vscode💻extension🔖helpers🪨extractfilepathfromscope](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Helpers/d/i/extractFilePathFromScope)
 * extractFilePathFromScope holds the data fields for a extractFilePathFromScope record.
 **/
function extractFilePathFromScope(scope: string): string | undefined {
  let cleanScope = scope;
  if (cleanScope.startsWith("@semio/breachs/")) {
    cleanScope = cleanScope.replace("@semio/breachs/", "");
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
    const root = bestBundle.root === "." ? "" : bestBundle.root.endsWith("/") ? bestBundle.root : bestBundle.root + "/";
    const filePath = root + fileName;
    return filePath.endsWith("/") ? filePath.slice(0, -1) : filePath;
  }

  if (cleanScope.startsWith("@semio/") || cleanScope.startsWith("@repo/")) {
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

/**
 * [🧰repo🖱️vscode💻extension🔖helpers🛠️resolveticketpath](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Helpers/d/i/resolveTicketPath)
 * resolveTicketPath holds the data fields for a resolveTicketPath record.
 **/
function resolveTicketPath(ticket: { year: number; month: number; day: number; slug: string; folderPath?: string }): string | undefined {
  if (ticket.folderPath) return ticket.folderPath;
  const root = getWorkspaceRoot();
  if (!root) return undefined;
  const relPath = path.join(String(ticket.year).padStart(2, "0"), String(ticket.month).padStart(2, "0"), String(ticket.day).padStart(2, "0"), ticket.slug);
  return path.join(root, ".repo", "🎫", relPath);
}

/**
 * [🧰repo🖱️vscode💻extension🔖helpers🛠️openfileatline](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Helpers/d/i/openFileAtLine)
 * [🧰repo🖱️vscode💻extension🔖helpers🪨openfileatline](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Helpers/d/i/openFileAtLine)
 * openFileAtLine holds the data fields for a openFileAtLine record.
 **/
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
}

// #endregion 🔖Helpers

// #region 🔖File Analysis & Diagnostics

// [🧰repo🖱️vscode💻extension🔖fileanalysisdiagnostics](repo://p/i/repo/b/u/vscode/f/extension.ts/s/File%20Analysis%20&%20Diagnostics)
// File Analysis & Diagnostics MUST handle analysis, breach diagnostics, bundle caching, and kit validation.

/** updateBundleCache holds the data fields for a updateBundleCache record.
 * [🧰repo🖱️vscode💻extension🔖fileanalysisdiagnostics🛠️updatebundlecache](repo://p/i/repo/b/u/vscode/f/extension.ts/s/File%20Analysis%20&%20Diagnostics/d/i/updateBundleCache)
 * [🧰repo🖱️vscode💻extension🔖fileanalysisdiagnostics🪨updatebundlecache](repo://p/i/repo/b/u/vscode/f/extension.ts/s/File%20Analysis%20&%20Diagnostics/d/i/updateBundleCache)
 **/
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

/**
 * [🧰repo🖱️vscode💻extension🔖fileanalysisdiagnostics🪨ignoreddirectories](repo://p/i/repo/b/u/vscode/f/extension.ts/s/File%20Analysis%20&%20Diagnostics/d/i/ignoredDirectories)
 * ignoredDirectories holds the data fields for a ignoredDirectories record.
 **/
const ignoredDirectories = new Set(["node_modules", "venv", "dist", "build", "out", "__pycache__", "coverage", "site-packages", "eggs", "wheels", "htmlcov", "target", "artifacts", "vendor"]);
/**
 * [🧰repo🖱️vscode💻extension🔖fileanalysisdiagnostics🪨alloweddotdirectories](repo://p/i/repo/b/u/vscode/f/extension.ts/s/File%20Analysis%20&%20Diagnostics/d/i/allowedDotDirectories)
 * allowedDotDirectories holds the data fields for a allowedDotDirectories record.
 **/
const allowedDotDirectories = new Set([".github", ".devcontainer", ".repo"]);

/**
 * [🧰repo🖱️vscode💻extension🔖fileanalysisdiagnostics🛠️isinignoreddirectory](repo://p/i/repo/b/u/vscode/f/extension.ts/s/File%20Analysis%20&%20Diagnostics/d/i/isInIgnoredDirectory)
 * [🧰repo🖱️vscode💻extension🔖fileanalysisdiagnostics🪨isinignoreddirectory](repo://p/i/repo/b/u/vscode/f/extension.ts/s/File%20Analysis%20&%20Diagnostics/d/i/isInIgnoredDirectory)
 * isInIgnoredDirectory holds the data fields for a isInIgnoredDirectory record.
 **/
function isInIgnoredDirectory(relativePath: string): boolean {
  const segments = relativePath.split("/");
  return segments.some((segment) => {
    if (ignoredDirectories.has(segment)) return true;
    if (segment.startsWith(".") && !allowedDotDirectories.has(segment)) return true;
    return false;
  });
}

/** shouldAnalyzeFile holds the data fields for a shouldAnalyzeFile record.
 * [🧰repo🖱️vscode💻extension🔖fileanalysisdiagnostics🛠️shouldanalyzefile](repo://p/i/repo/b/u/vscode/f/extension.ts/s/File%20Analysis%20&%20Diagnostics/d/i/shouldAnalyzeFile)
 * [🧰repo🖱️vscode💻extension🔖fileanalysisdiagnostics🪨shouldanalyzefile](repo://p/i/repo/b/u/vscode/f/extension.ts/s/File%20Analysis%20&%20Diagnostics/d/i/shouldAnalyzeFile)
 **/
function shouldAnalyzeFile(document: vscode.TextDocument): boolean {
  const supportedLanguages = ["typescript", "javascript", "typescriptreact", "javascriptreact", "json", "python", "csharp", "go", "shellscript"];
  return supportedLanguages.includes(document.languageId);
}

/**
 * analyzeFile holds the data fields for a analyzeFile record.
 * [🧰repo🖱️vscode💻extension🔖fileanalysisdiagnostics🛠️analyzefile](repo://p/i/repo/b/u/vscode/f/extension.ts/s/File%20Analysis%20&%20Diagnostics/d/i/analyzeFile)
 **/
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
    const result = await runRepoCommandJson<ToolResult<{ analyze: AnalyzeReport }>>(`analyze "${relativePath}"`);

    const breachs = result?.data?.analyze?.breachs;
    if (breachs && breachs.length > 0) {
      fileBreachsMap.set(fileUri.toString(), breachs);
      updateFileDiagnostics(document, breachs);
    } else {
      fileBreachsMap.delete(fileUri.toString());
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

/**
 * [🧰repo🖱️vscode💻extension🔖fileanalysisdiagnostics🛠️updatefilediagnostics](repo://p/i/repo/b/u/vscode/f/extension.ts/s/File%20Analysis%20&%20Diagnostics/d/i/updateFileDiagnostics)
 * [🧰repo🖱️vscode💻extension🔖fileanalysisdiagnostics🪨updatefilediagnostics](repo://p/i/repo/b/u/vscode/f/extension.ts/s/File%20Analysis%20&%20Diagnostics/d/i/updateFileDiagnostics)
 * updateFileDiagnostics holds the data fields for a updateFileDiagnostics record.
 **/
function updateFileDiagnostics(document: vscode.TextDocument, breachs: Breach[]): void {
  const root = getWorkspaceRoot();
  if (!root) return;
  const diagnosticsByUri = new Map<string, { uri: vscode.Uri; diagnostics: vscode.Diagnostic[] }>();

  diagnosticsByUri.set(document.uri.toString(), { uri: document.uri, diagnostics: [] });

  for (const breach of breachs) {
    const filePath = extractFilePathFromScope(breach.scope);
    if (!filePath) continue;
    const absPath = path.join(root, filePath);
    const fileUri = vscode.Uri.file(absPath);
    const uriKey = fileUri.toString();
    if (!diagnosticsByUri.has(uriKey)) {
      diagnosticsByUri.set(uriKey, { uri: fileUri, diagnostics: [] });
    }
    const line = Math.max(0, (breach.line ?? 1) - 1);
    const column = Math.max(0, (breach.column ?? 1) - 1);
    const endColumn = breach.excerpt ? column + breach.excerpt.length : column + 1;
    const range = new vscode.Range(line, column, line, endColumn);
    const severity = vscode.DiagnosticSeverity.Warning;
    let kindId = breach.kind.id;
    if (kindId.startsWith("@semio/policies//breachs/")) {
      kindId = kindId.replace("@semio/policies//breachs/", "");
    }
    const diagnostic = new vscode.Diagnostic(range, breach.summary, severity);
    diagnostic.source = DIAGNOSTIC_SOURCE;
    diagnostic.code = { value: kindId, target: fileUri.with({ fragment: `L${line + 1}` }) };
    diagnosticsByUri.get(uriKey)!.diagnostics.push(diagnostic);
  }
  for (const { uri, diagnostics } of diagnosticsByUri.values()) {
    repoDiagnosticCollection.set(uri, diagnostics);
  }
}

/** autofixBreach holds the data fields for a autofixBreach record.
 * [🧰repo🖱️vscode💻extension🔖fileanalysisdiagnostics🛠️autofixbreach](repo://p/i/repo/b/u/vscode/f/extension.ts/s/File%20Analysis%20&%20Diagnostics/d/i/autofixBreach)
 * [🧰repo🖱️vscode💻extension🔖fileanalysisdiagnostics🪨autofixbreach](repo://p/i/repo/b/u/vscode/f/extension.ts/s/File%20Analysis%20&%20Diagnostics/d/i/autofixBreach)
 **/
async function autofixBreach(relativePath: string): Promise<void> {
  const root = getWorkspaceRoot();
  if (!root) return;
  if (!hasRepoAccess()) {
    vscode.window.showErrorMessage("repo binary not found in go/repo/");
    return;
  }
  const command = getRepoCommand();
  try {
    await vscode.window.withProgress({ location: vscode.ProgressLocation.Notification, title: "Autofixing breach..." }, async () => {
      const { stderr } = await execAsync(`"${command}" autofix "${relativePath}"`, { cwd: root, timeout: 30000 });
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
    vscode.window.showInformationMessage(`Autofixed: ${relativePath}`);
  } catch (error) {
    logError("Failed to run autofix:", error);
    vscode.window.showErrorMessage(`Failed to autofix breach: ${error}`);
  }
}

/**
 * [🧰repo🖱️vscode💻extension🔖fileanalysisdiagnostics🛠️iskitdocument](repo://p/i/repo/b/u/vscode/f/extension.ts/s/File%20Analysis%20&%20Diagnostics/d/i/isKitDocument)
 * isKitDocument holds the data fields for a isKitDocument record.
 **/
function isKitDocument(document: vscode.TextDocument): boolean {
  if (document.languageId !== SEMIO_KIT_LANGUAGE) return false;
  const basename = document.uri.path.split("/").pop()?.toLowerCase() || "";
  return basename.startsWith("kit_") || basename.includes("_kit") || basename === "kit.json";
}

/**
 * [🧰repo🖱️vscode💻extension🔖fileanalysisdiagnostics🛠️validatekitdocument](repo://p/i/repo/b/u/vscode/f/extension.ts/s/File%20Analysis%20&%20Diagnostics/d/i/validateKitDocument)
 * validateKitDocument holds the data fields for a validateKitDocument record.
 **/
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

// [🧰repo🖱️vscode💻extension🔖providers](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Providers)
// Providers MUST implement VS Code tree data providers for filter, monorepo, and sections views.

/**
 * Tree item representing a filter option in the filter sidebar view.
 *Implementations MUST extend vscode.TreeItem and expose filter metadata.
 * [🧰repo🖱️vscode💻extension🔖providers🛠️filtertreeitem](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Providers/d/i/FilterTreeItem)
 **/
export class FilterTreeItem extends vscode.TreeItem {
  constructor(
    public readonly label: string,
    public readonly type: "root" | "search" | "filter" | "time" | "filterOption" | "timeValue",
    public readonly collapsibleState: vscode.TreeItemCollapsibleState = vscode.TreeItemCollapsibleState.None,
    public readonly contextValue?: string,
    public readonly filterKey?: string,
    public readonly filterValue?: any,
  ) {
    super(label, collapsibleState);
    this.contextValue = contextValue;
  }
}

/**
 * Provides the tree data for the filter sidebar view with search and toggle state.
 *Implementations MUST implement vscode.TreeDataProvider and emit change events on toggle.
 * [🧰repo🖱️vscode💻extension🔖providers🛠️filtertreedataprovider](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Providers/d/i/FilterTreeDataProvider)
 **/
export class FilterTreeDataProvider implements vscode.TreeDataProvider<FilterTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<FilterTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  public searchQuery: string = "";
  public matchCase: boolean = false;
  public matchWholeWord: boolean = false;
  public useRegex: boolean = false;

  public filters: Record<string, Record<string, boolean>> = {
    technology: { user: true, infrastructure: true, research: true },
    bundle: { library: true, binary: true, ui: true, site: true, assets: true, schema: true, default: true },
    folder: { organization: true, required: true },
    file: { code: true, script: true, config: true, lab: true, docs: true, resource: true, template: true, license: true },
    section: { none: false, all: true },
    definition: { implementation: true, interface: true, constant: true },
    goal: { open: true, closed: true },
    ticket: { open: true, closed: true },
    policy: { all: true },
    contributor: { all: true },
    checkpoint: { all: true },
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
        this.createFilterItem("🏗️Technologies", "filter_technology", "Technologies filter"),
        this.createFilterItem("📦Bundles", "filter_bundle", "Bundles filter"),
        this.createFilterItem("📁Folders", "filter_folder", "Folders filter"),
        this.createFilterItem("📄Files", "filter_file", "Files filter"),
        this.createFilterItem("🔖Sections", "filter_section", "Sections filter"),
        this.createFilterItem("🏷️Definitions", "filter_definition", "Definitions filter"),
        this.createFilterItem("🎯Goals", "filter_goal", "Goals filter"),
        this.createFilterItem("🎫Tickets", "filter_ticket", "Tickets filter"),
        this.createFilterItem("🎫Dates", "filter_time", "Dates filter", vscode.TreeItemCollapsibleState.Collapsed),
        this.createFilterItem("👮Policies", "filter_policy", "Policies filter"),
        this.createFilterItem("🧑‍💻Contributors", "filter_contributor", "Contributors filter"),
        this.createFilterItem("🔄Checkpoints", "filter_checkpoint", "Checkpoints filter"),
      ];
    }

    if (element.contextValue === "filter_time") {
      return this.availableYears.map((y) => {
        const excluded = this.excludedYears.includes(y);
        const item = new FilterTreeItem(String(y), "timeValue", vscode.TreeItemCollapsibleState.Collapsed, "filter_time_year", "year", y);
        item.tooltip = excluded ? `Excluded year ${y}` : `Included year ${y}`;
        item.command = { command: "semio.filter.toggleYear", title: "Toggle Year", arguments: [y] };
        return item;
      });
    }

    if (element.contextValue === "filter_time_year") {
      const year = element.filterValue;
      return this.availableMonths.map((m) => {
        const excluded = this.excludedMonths.includes(m);
        const label = new Date(2000, m - 1, 1).toLocaleString("default", { month: "long" });
        const item = new FilterTreeItem(label, "timeValue", vscode.TreeItemCollapsibleState.Collapsed, "filter_time_month", "month", m);
        item.tooltip = excluded ? `Excluded month ${label}` : `Included month ${label}`;
        item.command = { command: "semio.filter.toggleMonth", title: "Toggle Month", arguments: [m] };
        return item;
      });
    }

    if (element.contextValue === "filter_time_month") {
      return this.availableDays.map((d) => {
        const excluded = this.excludedDays.includes(d);
        const item = new FilterTreeItem(String(d).padStart(2, "0"), "timeValue", vscode.TreeItemCollapsibleState.None, "filter_time_day", "day", d);
        item.tooltip = excluded ? `Excluded day ${d}` : `Included day ${d}`;
        item.command = { command: "semio.filter.toggleDay", title: "Toggle Day", arguments: [d] };
        return item;
      });
    }

    return [];
  }

  private createSearchItem(): FilterTreeItem {
    const item = new FilterTreeItem("🔍Search", "search", vscode.TreeItemCollapsibleState.None, "filter_search");
    const details = [this.searchQuery ? `Query: ${this.searchQuery}` : "No query set", this.matchCase ? "Match case on" : "Match case off", this.matchWholeWord ? "Whole word on" : "Whole word off", this.useRegex ? "Regex on" : "Regex off"];
    item.tooltip = `Search filter\n${details.join("\n")}`;
    item.command = { command: "semio.filter.search", title: "Search" };
    return item;
  }

  private createFilterItem(label: string, contextValue: string, tooltip: string, collapsibleState: vscode.TreeItemCollapsibleState = vscode.TreeItemCollapsibleState.None): FilterTreeItem {
    const item = new FilterTreeItem(label, "filter", collapsibleState, contextValue);
    item.tooltip = tooltip;
    return item;
  }

  toggle(kind: string, key: string) {
    const filterKeys = this.filters[kind] ? Object.keys(this.filters[kind]) : [];
    const hasRealKeys = filterKeys.some((k) => k !== "none" && k !== "all");
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
      } else if (key === "none") {
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
    if (this.excludedYears.includes(year)) this.excludedYears = this.excludedYears.filter((y) => y !== year);
    else this.excludedYears.push(year);
    this.refresh();
    monorepoProvider?.refresh();
  }

  toggleMonth(month: number) {
    if (this.excludedMonths.includes(month)) this.excludedMonths = this.excludedMonths.filter((m) => m !== month);
    else this.excludedMonths.push(month);
    this.refresh();
    monorepoProvider?.refresh();
  }

  toggleDay(day: number) {
    if (this.excludedDays.includes(day)) this.excludedDays = this.excludedDays.filter((d) => d !== day);
    else this.excludedDays.push(day);
    this.refresh();
    monorepoProvider?.refresh();
  }
}

/**
 * Tree item representing a monorepo artifact in the sidebar tree.
 *Implementations MUST extend vscode.TreeItem and carry the original node data.
 * [🧰repo🖱️vscode💻extension🔖providers🛠️monorepotreeitem](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Providers/d/i/MonorepoTreeItem)
 **/
export class MonorepoTreeItem extends vscode.TreeItem {
  constructor(
    public readonly label: string,
    public readonly collapsibleState: vscode.TreeItemCollapsibleState,
    public readonly contextValue?: string,
    public readonly data?: any,
    public readonly nodeId?: string,
  ) {
    super(label, collapsibleState);
    this.contextValue = contextValue;
    if (nodeId) this.tooltip = nodeId;
  }
}

/**
 * Converts a TreeNodeData to a VS Code MonorepoTreeItem for the sidebar.
 *Implementations MUST set label, description, tooltip, and command from node data.
 * [🧰repo🖱️vscode💻extension🔖providers🛠️treenodetoitem](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Providers/d/i/treeNodeToItem)
 **/
export function treeNodeToItem(node: TreeNodeData): MonorepoTreeItem {
  const label = treeNodeDisplayLabel(node);
  const hasChildren = node.Children && node.Children.length > 0;
  const collapsible = hasChildren ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None;
  const ctx = treeNodeContextValue(node);
  const item = new MonorepoTreeItem(label, collapsible, ctx, node, node.ID || undefined);
  item.command = treeNodeCommand(node);
  if (node.Description) item.tooltip = node.Description;
  if (node.Kind === "checkpoint" && node.Data?.sha) item.description = node.Data.sha.substring(0, 7);
  if (node.Kind === "statute") {
    item.description = node.Data?.autofixable ? "🔧" : "";
    if (node.Description) item.tooltip = node.Description;
  }
  return item;
}

/**
 * Provides the tree data for the monorepo sidebar view using CLI tree output.
 *Implementations MUST implement vscode.TreeDataProvider and fetch data via CLI.
 * [🧰repo🖱️vscode💻extension🔖providers🛠️monorepotreedataprovider](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Providers/d/i/MonorepoTreeDataProvider)
 **/
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

/**
 * [🧰repo🖱️vscode💻extension🔖providers🛠️sectiontreeitem](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Providers/d/i/SectionTreeItem)
 * SectionTreeItem holds the data fields for a SectionTreeItem record.
 **/
class SectionTreeItem extends vscode.TreeItem {
  constructor(
    public section: SectionInfo,
    public filePath: string,
  ) {
    super(section.name, section.children && section.children.length > 0 ? vscode.TreeItemCollapsibleState.Collapsed : vscode.TreeItemCollapsibleState.None);
    this.contextValue = "section";
    this.iconPath = new vscode.ThemeIcon("bookmark");
    this.tooltip = `Section: ${section.name}`;
    const start = section.startLine - 1;
    this.command = {
      command: "vscode.open",
      title: "Open Section",
      arguments: [vscode.Uri.file(path.join(getWorkspaceRoot() || "", filePath)), { selection: new vscode.Range(start, 0, start, 0) }],
    };
  }
}

/**
 * Provides the tree data for the sections sidebar view of the active file.
 *Implementations MUST refresh when the active editor changes or the document is edited.
 * [🧰repo🖱️vscode💻extension🔖providers🛠️sectionstreedataprovider](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Providers/d/i/SectionsTreeDataProvider)
 **/
export class SectionsTreeDataProvider implements vscode.TreeDataProvider<SectionTreeItem> {
  private _onDidChangeTreeData = new vscode.EventEmitter<SectionTreeItem | undefined | null | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;
  private activeEditor: vscode.TextEditor | undefined;

  constructor(private context: vscode.ExtensionContext) {
    this.activeEditor = vscode.window.activeTextEditor;
    vscode.window.onDidChangeActiveTextEditor((editor) => {
      this.refresh();
    });
    vscode.workspace.onDidChangeTextDocument((e) => {
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
          } catch (e) { }
        }
        return this.createSectionItems(sections, filePath);
      } catch (e) {
        console.error("Failed to fetch sections:", e);
        return [];
      }
    }
  }

  private createSectionItems(sections: SectionInfo[], filePath: string): SectionTreeItem[] {
    return sections.map((s) => {
      const item = new SectionTreeItem(s, filePath);

      return item;
    });
  }
}

function isDefinitionEntityId(id: string): boolean {
  for (const [emoji, entityKind] of ENTITY_EMOJIS.entries()) {
    if (id.includes(emoji) && entityKind.startsWith("definition-")) {
      return true;
    }
  }
  return false;
}

const NATIVE_DEFINITION_SYMBOL_KINDS = new Set<vscode.SymbolKind>([
  vscode.SymbolKind.Class,
  vscode.SymbolKind.Constant,
  vscode.SymbolKind.Constructor,
  vscode.SymbolKind.Enum,
  vscode.SymbolKind.EnumMember,
  vscode.SymbolKind.Field,
  vscode.SymbolKind.Function,
  vscode.SymbolKind.Interface,
  vscode.SymbolKind.Method,
  vscode.SymbolKind.Module,
  vscode.SymbolKind.Namespace,
  vscode.SymbolKind.Property,
  vscode.SymbolKind.Struct,
  vscode.SymbolKind.TypeParameter,
  vscode.SymbolKind.Variable,
]);

function getDocumentRelativePath(document: vscode.TextDocument): string | null {
  if (document.uri.scheme !== "file") return null;
  const root = getWorkspaceRoot();
  if (!root) return null;
  const relativePath = path.relative(root, document.uri.fsPath).replace(/\\/g, "/");
  if (!relativePath || relativePath.startsWith("..")) return null;
  return relativePath;
}

function buildNativeDefinitionScope(relativePath: string, definitionName: string): string {
  return `${relativePath}§${definitionName}`;
}

function isDocumentSymbol(value: vscode.DocumentSymbol | vscode.SymbolInformation): value is vscode.DocumentSymbol {
  return "selectionRange" in value;
}

function collectNativeDefinitionFallbackRanges(document: vscode.TextDocument): Array<{ name: string; range: vscode.Range }> {
  const patternsByLanguage: Partial<Record<string, RegExp[]>> = {
    typescript: [
      /^\s*(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?(?:default\s+)?class\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?interface\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?type\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?enum\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\b/,
    ],
    typescriptreact: [
      /^\s*(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?(?:default\s+)?class\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?interface\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?type\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?enum\s+([A-Za-z_$][\w$]*)\b/,
      /^\s*(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\b/,
    ],
    javascript: [/^\s*(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\b/, /^\s*(?:export\s+)?(?:default\s+)?class\s+([A-Za-z_$][\w$]*)\b/, /^\s*(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\b/],
    javascriptreact: [/^\s*(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\b/, /^\s*(?:export\s+)?(?:default\s+)?class\s+([A-Za-z_$][\w$]*)\b/, /^\s*(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\b/],
    go: [/^\s*func\s+(?:\([^)]+\)\s*)?([A-Za-z_][\w]*)\b/, /^\s*type\s+([A-Za-z_][\w]*)\b/, /^\s*const\s+([A-Za-z_][\w]*)\b/, /^\s*var\s+([A-Za-z_][\w]*)\b/],
  };
  const patterns = patternsByLanguage[document.languageId] ?? [];
  if (patterns.length === 0) return [];
  const results: Array<{ name: string; range: vscode.Range }> = [];
  const seen = new Set<string>();
  for (let lineIndex = 0; lineIndex < document.lineCount; lineIndex++) {
    const textLine = document.lineAt(lineIndex).text;
    for (const pattern of patterns) {
      const match = pattern.exec(textLine);
      if (!match?.[1]) continue;
      const name = match[1];
      const key = `${lineIndex}:${name}`;
      if (seen.has(key)) continue;
      seen.add(key);
      const startCharacter = textLine.indexOf(name);
      if (startCharacter < 0) continue;
      results.push({
        name,
        range: new vscode.Range(lineIndex, startCharacter, lineIndex, startCharacter + name.length),
      });
      break;
    }
  }
  return results;
}

async function collectNativeDefinitionCodeLenses(document: vscode.TextDocument, token: vscode.CancellationToken): Promise<vscode.CodeLens[]> {
  const relativePath = getDocumentRelativePath(document);
  if (!relativePath) return [];
  const shouldDebug = document.uri.fsPath.endsWith("/repo/vscode/extension.ts") || document.uri.fsPath.endsWith("/repo/go/events.go");
  const symbols = (await vscode.commands.executeCommand<Array<vscode.DocumentSymbol | vscode.SymbolInformation>>("vscode.executeDocumentSymbolProvider", document.uri)) ?? [];
  const scopes = new Map<string, vscode.CodeLens>();

  const addLens = (name: string, range: vscode.Range): void => {
    if (!name) return;
    const scope = buildNativeDefinitionScope(relativePath, name);
    if (scopes.has(scope)) return;
    scopes.set(
      scope,
      new vscode.CodeLens(range, {
        title: "Analyze",
        command: "semio.analyze",
        arguments: [scope],
      }),
    );
  };

  const visitDocumentSymbol = (symbol: vscode.DocumentSymbol): void => {
    if (token.isCancellationRequested) return;
    if (NATIVE_DEFINITION_SYMBOL_KINDS.has(symbol.kind)) {
      addLens(symbol.name, symbol.selectionRange);
    }
    for (const child of symbol.children) {
      visitDocumentSymbol(child);
    }
  };

  for (const symbol of symbols) {
    if (token.isCancellationRequested) break;
    if (isDocumentSymbol(symbol)) {
      visitDocumentSymbol(symbol);
    } else if (NATIVE_DEFINITION_SYMBOL_KINDS.has(symbol.kind)) {
      addLens(symbol.name, symbol.location.range);
    }
  }

  if (scopes.size == 0) {
    for (const fallback of collectNativeDefinitionFallbackRanges(document)) {
      addLens(fallback.name, fallback.range);
    }
  }

  return Array.from(scopes.values());
}

/**
 * [🧰repo🖱️vscode💻extension🔖providers🛠️semiocodelensprovider](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Providers/d/i/SemioCodeLensProvider)
 * SemioCodeLensProvider provides Analyze and Navigate to CodeLenses for all entity IDs.
 * It uses the ENTITY_ID_REGEX built dynamically from the ENTITY_EMOJIS registry.
 **/
class SemioCodeLensProvider implements vscode.CodeLensProvider {
  async provideCodeLenses(document: vscode.TextDocument, token: vscode.CancellationToken): Promise<vscode.CodeLens[]> {
    const lenses: vscode.CodeLens[] = [];
    const text = document.getText();
    const regex = buildEntityIdRegex();
    let match;

    while ((match = regex.exec(text)) !== null) {
      if (token.isCancellationRequested) break;
      const id = match[1] || match[3];
      const uri = match[2];

      const startPos = document.positionAt(match.index);
      const endPos = document.positionAt(match.index + match[0].length);
      const range = new vscode.Range(startPos, endPos);

      lenses.push(
        new vscode.CodeLens(range, {
          title: "Analyze",
          command: "semio.analyze",
          arguments: [id],
        }),
      );

      lenses.push(
        new vscode.CodeLens(range, {
          title: "Navigate to",
          command: "semio.navigate",
          arguments: [uri || id],
        }),
      );
    }

    lenses.push(...(await collectNativeDefinitionCodeLenses(document, token)));

    return lenses;
  }
}

/**
 * [🧰repo🖱️vscode💻extension🔖providers🪨semioguttericon](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Providers/d/i/semioGutterIcon)
 * semioGutterIcon holds the data fields for a semioGutterIcon record.
 **/
let semioGutterIcon: vscode.TextEditorDecorationType;

/** updateSemioDecorations holds the data fields for a updateSemioDecorations record.
 * [🧰repo🖱️vscode💻extension🔖providers🛠️updatesemiodecorations](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Providers/d/i/updateSemioDecorations)
 * [🧰repo🖱️vscode💻extension🔖providers🪨updatesemiodecorations](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Providers/d/i/updateSemioDecorations)
 **/
function updateSemioDecorations(editor: vscode.TextEditor) {
  if (!editor || !semioGutterIcon) return;
  const text = editor.document.getText();
  const regex = buildEntityIdRegex();
  const decorations: vscode.DecorationOptions[] = [];
  let match;

  while ((match = regex.exec(text)) !== null) {
    const startPos = editor.document.positionAt(match.index);
    const endPos = editor.document.positionAt(match.index + match[0].length);
    decorations.push({ range: new vscode.Range(startPos, endPos) });
  }

  editor.setDecorations(semioGutterIcon, decorations);
}

// #endregion 🔖Providers

// #region 🔖Activation

// [🧰repo🖱️vscode💻extension🔖activation](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Activation)
// Activation MUST handle extension activation, command registration, and lifecycle management.

/**
 * registerSidebarViews holds the data fields for a registerSidebarViews record.
 * [🧰repo🖱️vscode💻extension🔖activation🛠️registersidebarviews](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Activation/d/i/registerSidebarViews)
 **/
function registerSidebarViews(context: vscode.ExtensionContext): void {
  filterProvider = new FilterTreeDataProvider();
  vscode.window.registerTreeDataProvider("semio.filter", filterProvider);

  monorepoProvider = new MonorepoTreeDataProvider(filterProvider);
  vscode.window.registerTreeDataProvider("semio.monorepo", monorepoProvider);

  const sectionsProvider = new SectionsTreeDataProvider(context);
  vscode.window.registerTreeDataProvider("semio.sections", sectionsProvider);
}

/**
 * [🧰repo🖱️vscode💻extension🔖activation🛠️registercommands](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Activation/d/i/registerCommands)
 * [🧰repo🖱️vscode💻extension🔖activation🪨registercommands](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Activation/d/i/registerCommands)
 * registerCommands holds the data fields for a registerCommands record.
 **/
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
    technology: ["user", "infrastructure", "research", "none", "all"],
    folder: ["organization", "required", "none", "all"],
    file: ["code", "script", "config", "lab", "docs", "resource", "license", "none", "all"],
    section: ["none", "all"],
    definition: ["implementation", "interface", "constant", "none", "all"],
    goal: ["open", "closed", "none", "all"],
    ticket: ["open", "closed", "none", "all"],
    policy: ["none", "all"],
    contributor: ["none", "all"],
    checkpoint: ["none", "all"],
    time: ["none", "all"],
  };
  for (const [kind, keys] of Object.entries(filterToggleEntries)) {
    for (const key of keys) {
      register(`semio.filter.toggle.${kind}.${key}`, () => filterProvider?.toggle(kind, key));
    }
  }

  const timeModes: Array<["year" | "month" | "day", "none" | "all"]> = [
    ["year", "none"],
    ["year", "all"],
    ["month", "none"],
    ["month", "all"],
    ["day", "none"],
    ["day", "all"],
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
    if (target.startsWith("repo://")) {
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
    const t = { year, month, day, slug, folderPath: undefined as string | undefined };
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
    return vscode.window.showInformationMessage(`Close ticket: ${ticketId}?`, "Yes", "No").then((answer) => {
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
    return vscode.window.showInputBox({ prompt: "Reopen prompt" }).then((prompt) => {
      if (!prompt) return;
      const binaryPath = getRepoBinaryPath();
      if (!binaryPath) return;
      const cp = require("child_process");
      cp.execSync(`${binaryPath} ticket reopen ${ticketId} "${prompt}" copilot-chat`, { cwd: getWorkspaceRoot() });
      monorepoProvider?.refresh();
    });
  });

  register("semio.draftCreate", async () => {
    const title = await vscode.window.showInputBox({ prompt: "Draft title" });
    if (!title) return;
    const binaryPath = getRepoBinaryPath();
    if (!binaryPath) return;
    const cp = require("child_process");
    cp.execSync(`${binaryPath} draft create "${title}"`, { cwd: getWorkspaceRoot() });
    monorepoProvider?.refresh();
  });

  register("semio.draftDelete", (item: MonorepoTreeItem) => {
    const node = item?.data as TreeNodeData | undefined;
    const slug = node?.Data?.slug ?? node?.Label;
    if (!slug) return;
    return vscode.window.showInformationMessage(`Delete draft: ${slug}?`, "Yes", "No").then((answer) => {
      if (answer === "Yes") {
        const binaryPath = getRepoBinaryPath();
        if (!binaryPath) return;
        const cp = require("child_process");
        cp.execSync(`${binaryPath} draft delete ${slug}`, { cwd: getWorkspaceRoot() });
        monorepoProvider?.refresh();
      }
    });
  });

  register("semio.copyCheckpointSha", (item: MonorepoTreeItem) => {
    const node = item?.data as TreeNodeData | undefined;
    const sha = node?.Data?.sha;
    if (sha) {
      vscode.env.clipboard.writeText(sha);
      vscode.window.showInformationMessage(`Copied SHA: ${sha.substring(0, 7)}`);
    }
  });

  register("semio.openCheckpointInGitHub", (item: MonorepoTreeItem) => {
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

  register("semio.open", (target: string) => {
    if (target) {
      vscode.commands.executeCommand("semio.navigate", target);
    }
  });

  register("semio.analyze", async (id: string) => {
    if (!id) return;
    const binaryPath = getRepoBinaryPath();
    if (!binaryPath) return;
    const cp = require("child_process");
    try {
      const output = await new Promise<string>((resolve, reject) => {
        cp.exec(`"${binaryPath}" analyze "${id}"`, { cwd: getWorkspaceRoot(), encoding: "utf-8" }, (error: any, stdout: string, stderr: string) => {
          if (error && !stdout && !stderr) reject(error);
          else resolve((stdout || "") + (stderr ? "\n" + stderr : ""));
        });
      });
      const doc = await vscode.workspace.openTextDocument({ content: output, language: "markdown" });
      await vscode.window.showTextDocument(doc, { preview: true, preserveFocus: true, viewColumn: vscode.ViewColumn.Beside });
    } catch (e: any) {
      vscode.window.showErrorMessage(`Failed to analyze ${id}: ${e.message}`);
    }
  });

  const contributedCommands: string[] = [
    "semio.analyze",
    "semio.analyzeFile",
    "semio.autofix",
    "semio.autofixFile",
    "semio.policyList",
    "semio.policyTree",
    "semio.ticketList",
    "semio.ticketRead",
    "semio.ticketTree",
    "semio.technologyList",
    "semio.technologyTree",
    "semio.contributorAdd",
    "semio.contributorList",
    "semio.contributorRemove",
    "semio.sectionTree",
    "semio.sectionList",
    "semio.sectionCreate",
    "semio.sectionMove",
    "semio.sectionDelete",
    "semio.sectionOpen",
    "semio.sectionRename",
    "semio.sectionCreateChild",
    "semio.sectionRemove",
    "semio.sectionIntegrate",
    "semio.definitionList",
    "semio.definitionTree",
    "semio.folderTree",
    "semio.folderCreate",
    "semio.folderMove",
    "semio.folderDelete",
    "semio.folderList",
    "semio.fileCreate",
    "semio.fileMove",
    "semio.fileDelete",
    "semio.fileList",
    "semio.fileTree",
    "semio.refreshDiagnostics",
    "semio.autofixBreach",
    "semio.navigateToRepo",
    "semio.navigateTo",
    "semio.goalOpen",
    "semio.goalList",
  ];

  for (const command of contributedCommands) {
    if (registered.has(command)) continue;
    register(command, (..._args: unknown[]) => undefined);
  }

  loadAvailableFilterValues();
}

/**
 * [🧰repo🖱️vscode💻extension🔖activation🛠️loadavailablefiltervalues](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Activation/d/i/loadAvailableFilterValues)
 * [🧰repo🖱️vscode💻extension🔖activation🪨loadavailablefiltervalues](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Activation/d/i/loadAvailableFilterValues)
 * loadAvailableFilterValues holds the data fields for a loadAvailableFilterValues record.
 **/
async function loadAvailableFilterValues(): Promise<void> {
  const years = new Set<number>();
  const months = new Set<number>();
  const days = new Set<number>();
  const contributors = new Set<string>();
  const policies = new Set<string>();
  const breachs = new Set<string>();

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
        if (n.Kind === "statute") breachs.add(n.Data?.id || n.ID || "");
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

/**
 * Activates the repo VS Code extension and registers all providers and commands.
 *Implementations MUST register sidebar views, commands, diagnostics, and event handlers.
 * [🧰repo🖱️vscode💻extension🔖activation🛠️activate](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Activation/d/i/activate)
 **/
export function activate(context: vscode.ExtensionContext) {
  outputChannel = vscode.window.createOutputChannel("repo");
  context.subscriptions.push(outputChannel);
  log("[ACTIVATION] repo extension activating...");

  try {
    registerSidebarViews(context);
    registerCommands(context);

    repoDiagnosticCollection = vscode.languages.createDiagnosticCollection("semio");
    kitDiagnosticCollection = vscode.languages.createDiagnosticCollection("semio-kit");
    context.subscriptions.push(repoDiagnosticCollection, kitDiagnosticCollection);

    semioGutterIcon = vscode.window.createTextEditorDecorationType({
      gutterIconPath: vscode.Uri.file(context.asAbsolutePath("semio_codeicon.svg")),
      gutterIconSize: "contain",
    });

    context.subscriptions.push(vscode.languages.registerCodeLensProvider({ pattern: "**/*" }, new SemioCodeLensProvider()));

    context.subscriptions.push(
      vscode.window.onDidChangeActiveTextEditor((editor) => {
        if (editor) updateSemioDecorations(editor);
      }),
    );

    if (vscode.window.activeTextEditor) {
      updateSemioDecorations(vscode.window.activeTextEditor);
    }

    context.subscriptions.push(
      vscode.workspace.onDidSaveTextDocument((document) => {
        invalidateTreeNodeCache();
        monorepoProvider?.refresh();
        if (shouldAnalyzeFile(document)) analyzeFile(document);
        if (isKitDocument(document)) validateKitDocument(document);
      }),
    );

    context.subscriptions.push(
      vscode.workspace.onDidOpenTextDocument((document) => {
        if (shouldAnalyzeFile(document)) analyzeFile(document);
        if (isKitDocument(document)) validateKitDocument(document);
      }),
    );

    const analyzeDebounceTimers = new Map<string, ReturnType<typeof setTimeout>>();
    context.subscriptions.push(
      vscode.workspace.onDidChangeTextDocument((event) => {
        if (vscode.window.activeTextEditor && event.document === vscode.window.activeTextEditor.document) {
          updateSemioDecorations(vscode.window.activeTextEditor);
        }
        const document = event.document;
        if (!shouldAnalyzeFile(document) && !isKitDocument(document)) return;
        const key = document.uri.toString();
        const existing = analyzeDebounceTimers.get(key);
        if (existing) clearTimeout(existing);
        analyzeDebounceTimers.set(
          key,
          setTimeout(() => {
            analyzeDebounceTimers.delete(key);
            if (shouldAnalyzeFile(document)) analyzeFile(document);
            if (isKitDocument(document)) validateKitDocument(document);
          }, 1500),
        );
      }),
    );

    context.subscriptions.push(
      vscode.workspace.registerTextDocumentContentProvider("repo", {
        provideTextDocumentContent(uri: vscode.Uri): string {
          const repoUri = `repo://${uri.authority.toLowerCase()}${uri.path.toLowerCase()}`;
          vscode.commands.executeCommand("semio.navigate", repoUri);
          return "";
        },
      }),
    );

    context.subscriptions.push(
      vscode.window.registerUriHandler({
        handleUri(uri: vscode.Uri) {
          const repoUri = `repo://${uri.authority.toLowerCase()}${uri.path.toLowerCase()}`;
          vscode.commands.executeCommand("semio.navigate", repoUri);
        },
      }),
    );

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

    log("[ACTIVATION] repo extension activated.");
  } catch (e) {
    logError("[ACTIVATION] Failed to activate extension:", e);
  }
}

/**
 * Deactivates the repo VS Code extension and releases resources.
 *Implementations MUST clean up any active subscriptions.
 * [🧰repo🖱️vscode💻extension🔖activation🛠️deactivate](repo://p/i/repo/b/u/vscode/f/extension.ts/s/Activation/d/i/deactivate)
 **/
export function deactivate() { }

// #endregion 🔖Activation
