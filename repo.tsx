#!/usr/bin/env tsx
// #region Header

// repo.tsx

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

import { execSync, spawnSync } from "child_process";
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "fs";
import matter from "gray-matter";
import { Box, render, Text, useApp } from "ink";
import { dirname, join, relative, sep } from "path";
import React from "react";
import * as ts from "typescript";
import { fileURLToPath } from "url";

// #endregion Imports

// #region Types

//#region Scope
type ScopeKind = "repo" | "project" | "folder" | "file" | "region" | "definition";

interface Scope {
  raw: string;
  kind: ScopeKind;
  projectName?: string;
  filePath?: string;
  regionPath?: string[];
  definitionName?: string;
}
//#endregion Scope

//#region Issue
type IssuePriority = "high" | "medium" | "low";
type IssueSeverity = "error" | "warning";

interface Issue {
  id: string;
  summary: string;
  kind: string;
  priority: IssuePriority;
  severity: IssueSeverity;
  autofixable: boolean;
  solution: string;
  reason: string;
  scope: string;
  line?: number;
  column?: number;
  excerpt?: string;
  autofix?: Fix;
}
//#endregion Issue

//#region Fix
interface TextEdit {
  start: number;
  end: number;
  newText: string;
}

interface Fix {
  description: string;
  edits: Map<string, TextEdit[]>;
}
//#endregion Fix

//#region Project
interface NxProject {
  name: string;
  root: string;
  sourceRoot?: string;
  projectType?: "library" | "application";
  tags?: string[];
}
//#endregion Project

//#region Region
interface RegionInfo {
  name: string;
  startLine: number;
  endLine: number;
  startIndex: number;
  endIndex: number;
  children: RegionInfo[];
}
//#endregion Region

//#region Definition
interface DefinitionInfo {
  name: string;
  kind: "function" | "class" | "variable" | "interface" | "type" | "enum" | "method" | "property";
  startLine: number;
  endLine: number;
  startIndex: number;
  endIndex: number;
}
//#endregion Definition

//#region Ticket
type TicketStatus = "open" | "closed";

interface TicketIteration {
  prompt: string;
  model?: string;
  date: { started: string; ended?: string };
  author?: string;
  commit?: string;
  files?: { updated?: Array<{ path: string; lines?: { added: number; removed: number } }>; created?: Array<{ path: string; lines?: { added: number; removed: number } }>; removed?: Array<{ path: string; lines?: { added: number; removed: number } }> };
  lines?: { added: number; removed: number };
}

interface TicketFrontmatter {
  slug: string;
  prompt: string;
  summary?: string;
  status: TicketStatus;
  author?: string;
  date: { created: string; finished?: string };
  commit?: string;
  model?: string;
  iterations?: TicketIteration[];
  files?: { updated?: Array<{ path: string; lines?: { added: number; removed: number } }>; created?: string[]; removed?: string[] };
  lines?: { added: number; removed: number };
}

interface Ticket {
  year: number;
  month: number;
  day: number;
  slug: string;
  frontmatter: TicketFrontmatter;
  content: string;
  filePath: string;
}
//#endregion Ticket

//#region Rule
interface RuleMeta {
  id: string;
  name: string;
  description: string;
  scopes: string[];
  priority: IssuePriority;
  autofixable: boolean;
}

interface RuleContext {
  scope: Scope;
  rootDir: string;
  projects: () => NxProject[];
  project: (name: string) => NxProject | undefined;
  files: (pattern?: string) => string[];
  readText: (filePath: string) => string;
  regions: (filePath: string) => RegionInfo[];
  definitions: (filePath: string) => DefinitionInfo[];
  createIssue: (partial: Omit<Issue, "id" | "priority" | "autofixable">) => Issue;
  createFix: (description: string, edits: Map<string, TextEdit[]>) => Fix;
}

type RuleFn = (ctx: RuleContext) => Promise<Issue[]>;

interface RegisteredRule {
  meta: RuleMeta;
  run: RuleFn;
}
//#endregion Rule

//#region Report
interface AnalyzeReport {
  timestamp: string;
  status: "success" | "error" | "warning";
  scope: string;
  summary: { total: number; byPriority: Record<IssuePriority, number>; bySeverity: Record<IssueSeverity, number>; byKind: Record<string, number> };
  issues: Issue[];
}
//#endregion Report

//#region Command
type CommandName = "help" | "analyze" | "fix" | "rule" | "ticket" | "project" | "folder" | "file" | "region" | "definition" | "tool";

interface ParsedCommand {
  name: CommandName;
  subcommand?: string;
  args: string[];
  options: Record<string, string | boolean>;
}
//#endregion Command

// #endregion Types

// #region Constants

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const ROOT_DIR = __dirname;
const REPORTS_DIR = join(ROOT_DIR, "reports");
const TICKETS_DIR = join(ROOT_DIR, "tickets");

const HELP_TEXT = `
repo - Monorepo CLI for Semio

Usage: npx tsx repo.tsx <command> [subcommand] [options]

Commands:
  help                     Show this help message
  analyze [--scope=...]    Analyze codebase for issues
  fix [--scope=...]        Apply autofixes for issues
  rule list                List all registered rules
  rule run <id>            Run a specific rule
  ticket new <title>       Create a new ticket
  ticket list [year/month] List tickets
  ticket read <path>       Read a ticket
  ticket iterate <path>    Run rules and sync issues to ticket
  ticket close <path>      Close a ticket (if valid)
  project list             List Nx projects
  project tree             Show project dependency tree
  folder tree [path]       Show folder structure
  file list [scope]        List files in scope
  region tree <file>       Show region structure of a file
  definition list <file>   List definitions in a file
  tool <name> [args...]    Run an Nx target (e.g., lint, test)

Options:
  --scope=<scope>          Limit operation to scope
  --json                   Output as JSON
  --dry-run                Preview without making changes
  --help, -h               Show help for command

Scope syntax:
  @semio                   Repo scope
  @semio/js                Project scope
  js/js/sketchpad/         Folder scope
  js/js/sketchpad/App.tsx  File scope
  file.tsx#Region          Region scope
  file.tsx§Function        Definition scope
`;

// #endregion Constants

// #region Utilities

//#region Path Utilities
function normalizePathSeparators(p: string): string {
  return p.replace(/\\/g, "/");
}

function ensureDir(dirPath: string): void {
  if (!existsSync(dirPath)) {
    mkdirSync(dirPath, { recursive: true });
  }
}

function getRelativePath(filePath: string): string {
  return normalizePathSeparators(relative(ROOT_DIR, filePath));
}
//#endregion Path Utilities

//#region File Operations
function readTextFile(filePath: string): string {
  return readFileSync(filePath, "utf-8");
}

function writeTextFile(filePath: string, content: string): void {
  ensureDir(dirname(filePath));
  writeFileSync(filePath, content, "utf-8");
}

function writeJsonFile(filePath: string, data: unknown): void {
  writeTextFile(filePath, JSON.stringify(data, null, 2) + "\n");
}

function readJsonFile<T>(filePath: string): T {
  return JSON.parse(readTextFile(filePath)) as T;
}

function simpleGlob(pattern: string, options: { cwd?: string; ignore?: string[]; respectGitignore?: boolean } = {}): string[] {
  const cwd = options.cwd ?? ROOT_DIR;
  const ignore = options.ignore ?? [];
  const respectGitignore = options.respectGitignore ?? true;
  const results: string[] = [];
  const extensions = pattern.match(/\{([^}]+)\}/)?.[1]?.split(",") ?? [];
  const basePath = pattern.replace(/\*\*\/\*\.\{[^}]+\}$/, "").replace(/\*\*\/\*\.[a-z]+$/, "");
  function shouldIgnore(p: string): boolean {
    const normalized = normalizePathSeparators(p);
    return ignore.some((ig) => {
      const normalizedIg = normalizePathSeparators(ig.replace(/\*\*/g, "").replace(/\*/g, ""));
      return normalized.includes(normalizedIg);
    });
  }
  function walkDir(dir: string): void {
    if (!existsSync(dir)) return;
    const entries = readdirSync(dir, { withFileTypes: true });
    const entryPaths = entries.map((e) => normalizePathSeparators(relative(cwd, join(dir, e.name))));
    const gitIgnoredSet = respectGitignore ? getGitIgnoredSet(entryPaths) : new Set<string>();
    for (const entry of entries) {
      const fullPath = join(dir, entry.name);
      const relPath = relative(cwd, fullPath);
      const normalizedRelPath = normalizePathSeparators(relPath);
      if (shouldIgnore(relPath)) continue;
      if (gitIgnoredSet.has(normalizedRelPath) || gitIgnoredSet.has(normalizedRelPath + "/")) continue;
      if (entry.isDirectory()) {
        walkDir(fullPath);
      } else if (entry.isFile()) {
        const ext = entry.name.split(".").pop()?.toLowerCase() ?? "";
        if (extensions.length === 0 || extensions.includes(ext)) {
          results.push(normalizedRelPath);
        }
      }
    }
  }
  const startDir = basePath ? join(cwd, basePath) : cwd;
  walkDir(startDir);
  return results;
}
//#endregion File Operations

//#region Date Utilities
function formatDate(date: Date): { year: number; month: number; day: number } {
  return { year: date.getFullYear(), month: date.getMonth() + 1, day: date.getDate() };
}

function padNumber(n: number, width: number = 2): string {
  return String(n).padStart(width, "0");
}

function isoTimestamp(): string {
  return new Date().toISOString();
}
//#endregion Date Utilities

//#region String Utilities
function slugify(text: string): string {
  return text
    .toUpperCase()
    .replace(/[^A-Z0-9]+/g, "-")
    .replace(/^-|-$/g, "");
}
//#endregion String Utilities

//#region Process Management
function execCommand(command: string, args: string[] = [], options?: { cwd?: string }): { stdout: string; stderr: string; exitCode: number } {
  const result = spawnSync(command, args, { cwd: options?.cwd ?? ROOT_DIR, encoding: "utf-8", shell: true });
  return { stdout: result.stdout ?? "", stderr: result.stderr ?? "", exitCode: result.status ?? 1 };
}
//#endregion Process Management

//#region Git Utilities
function getGitIgnoredSet(paths: string[]): Set<string> {
  if (paths.length === 0) return new Set();
  const result = spawnSync("git", ["check-ignore", ...paths.map(normalizePathSeparators)], { cwd: ROOT_DIR, encoding: "utf-8", shell: true });
  if (result.status === null || result.stdout === null) return new Set();
  return new Set(result.stdout.split("\n").filter(Boolean).map(normalizePathSeparators));
}
//#endregion Git Utilities

// #endregion Utilities

// #region Scope Parser

function parseScope(raw: string): Scope {
  if (!raw || raw === "@semio") {
    return { raw: raw || "@semio", kind: "repo" };
  }
  if (raw.includes("§")) {
    const [filePart, defName] = raw.split("§");
    return { raw, kind: "definition", filePath: filePart, definitionName: defName };
  }
  if (raw.includes("#")) {
    const [filePart, ...regionParts] = raw.split("#");
    return { raw, kind: "region", filePath: filePart, regionPath: regionParts };
  }
  if (raw.startsWith("@semio/")) {
    return { raw, kind: "project", projectName: raw };
  }
  if (raw.endsWith("/")) {
    return { raw, kind: "folder", filePath: raw };
  }
  const ext = raw.split(".").pop()?.toLowerCase() ?? "";
  const codeExtensions = ["ts", "tsx", "js", "jsx", "py", "cs", "json", "md", "yaml", "yml", "sql", "graphql"];
  if (codeExtensions.includes(ext)) {
    return { raw, kind: "file", filePath: raw };
  }
  return { raw, kind: "folder", filePath: raw };
}

function scopeToFiles(scope: Scope, projects: NxProject[]): string[] {
  if (scope.kind === "repo") {
    return simpleGlob("**/*.{ts,tsx,py,cs}", { cwd: ROOT_DIR, ignore: ["node_modules/**", "**/node_modules/**", "**/.venv/**"] });
  }
  if (scope.kind === "project") {
    const project = projects.find((p) => p.name === scope.projectName);
    if (!project) return [];
    return simpleGlob(`${project.root}/**/*.{ts,tsx,py,cs}`, { cwd: ROOT_DIR, ignore: ["**/node_modules/**", "**/.venv/**"] });
  }
  if (scope.kind === "folder" && scope.filePath) {
    return simpleGlob(`${scope.filePath}**/*.{ts,tsx,py,cs}`, { cwd: ROOT_DIR, ignore: ["**/node_modules/**", "**/.venv/**"] });
  }
  if ((scope.kind === "file" || scope.kind === "region" || scope.kind === "definition") && scope.filePath) {
    return [scope.filePath];
  }
  return [];
}

function matchesScope(ruleScopes: string[], targetScope: Scope): boolean {
  for (const pattern of ruleScopes) {
    if (pattern === "*" || pattern === "**/*") return true;
    if (pattern.startsWith("@semio")) {
      if (targetScope.kind === "repo" || (targetScope.kind === "project" && targetScope.projectName?.startsWith(pattern))) return true;
    }
    if (targetScope.filePath) {
      const normalizedTarget = normalizePathSeparators(targetScope.filePath);
      const normalizedPattern = normalizePathSeparators(pattern);
      if (normalizedTarget.includes(normalizedPattern) || normalizedTarget.match(new RegExp(normalizedPattern.replace(/\*/g, ".*")))) return true;
    }
  }
  return false;
}

// #endregion Scope Parser

// #region Nx Adapter

let cachedProjects: NxProject[] | null = null;

function getNxProjects(): NxProject[] {
  if (cachedProjects) return cachedProjects;
  try {
    const result = execCommand("npx", ["nx", "show", "projects", "--json"]);
    if (result.exitCode !== 0) {
      cachedProjects = [];
      return cachedProjects;
    }
    const projectNames: string[] = JSON.parse(result.stdout);
    cachedProjects = projectNames.map((name) => {
      const configResult = execCommand("npx", ["nx", "show", "project", name, "--json"]);
      if (configResult.exitCode === 0) {
        try {
          const config = JSON.parse(configResult.stdout);
          return { name, root: config.root ?? "", sourceRoot: config.sourceRoot, projectType: config.projectType, tags: config.tags };
        } catch {
          return { name, root: "" };
        }
      }
      return { name, root: "" };
    });
    return cachedProjects;
  } catch {
    cachedProjects = [];
    return cachedProjects;
  }
}

function runNxTarget(target: string, projects?: string[], extraArgs: string[] = []): { success: boolean; output: string } {
  const args = ["nx"];
  if (projects && projects.length === 1) {
    args.push("run", `${projects[0]}:${target}`, ...extraArgs);
  } else if (projects && projects.length > 1) {
    args.push("run-many", "-t", target, "-p", projects.join(","), ...extraArgs);
  } else {
    args.push("run-many", "-t", target, ...extraArgs);
  }
  const result = execCommand("npx", args);
  return { success: result.exitCode === 0, output: result.stdout + result.stderr };
}

// #endregion Nx Adapter

// #region Region Parser

function parseRegions(content: string, language: "typescript" | "python" | "csharp"): RegionInfo[] {
  const lines = content.split("\n");
  const stack: RegionInfo[] = [];
  const roots: RegionInfo[] = [];
  const regionStartPatterns: Record<string, RegExp> = { typescript: /^\s*\/\/\s*#region\s+(.+?)\r?$/i, python: /^\s*#\s*region\s+(.+?)\r?$/i, csharp: /^\s*#region\s+(.+?)\r?$/i };
  const regionEndPatterns: Record<string, RegExp> = { typescript: /^\s*\/\/\s*#endregion/i, python: /^\s*#\s*endregion/i, csharp: /^\s*#endregion/i };
  const startPattern = regionStartPatterns[language];
  const endPattern = regionEndPatterns[language];
  let charIndex = 0;
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const lineStart = charIndex;
    const startMatch = line.match(startPattern);
    if (startMatch) {
      const region: RegionInfo = { name: startMatch[1].trim(), startLine: i + 1, endLine: -1, startIndex: lineStart, endIndex: -1, children: [] };
      if (stack.length > 0) {
        stack[stack.length - 1].children.push(region);
      } else {
        roots.push(region);
      }
      stack.push(region);
    } else if (endPattern.test(line)) {
      const region = stack.pop();
      if (region) {
        region.endLine = i + 1;
        region.endIndex = charIndex + line.length;
      }
    }
    charIndex += line.length + 1;
  }
  return roots;
}

function getLanguageFromPath(filePath: string): "typescript" | "python" | "csharp" | null {
  const ext = filePath.split(".").pop()?.toLowerCase();
  if (ext === "ts" || ext === "tsx" || ext === "js" || ext === "jsx") return "typescript";
  if (ext === "py") return "python";
  if (ext === "cs") return "csharp";
  return null;
}

// #endregion Region Parser

// #region Definition Parser

function parseDefinitions(content: string, filePath: string): DefinitionInfo[] {
  const language = getLanguageFromPath(filePath);
  if (language !== "typescript") return [];
  const sourceFile = ts.createSourceFile(filePath, content, ts.ScriptTarget.Latest, true, filePath.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const definitions: DefinitionInfo[] = [];
  function visit(node: ts.Node) {
    let name: string | undefined;
    let kind: DefinitionInfo["kind"] | undefined;
    if (ts.isFunctionDeclaration(node) && node.name) {
      name = node.name.text;
      kind = "function";
    } else if (ts.isClassDeclaration(node) && node.name) {
      name = node.name.text;
      kind = "class";
    } else if (ts.isInterfaceDeclaration(node)) {
      name = node.name.text;
      kind = "interface";
    } else if (ts.isTypeAliasDeclaration(node)) {
      name = node.name.text;
      kind = "type";
    } else if (ts.isEnumDeclaration(node)) {
      name = node.name.text;
      kind = "enum";
    } else if (ts.isVariableStatement(node)) {
      for (const decl of node.declarationList.declarations) {
        if (ts.isIdentifier(decl.name)) {
          const startPos = sourceFile.getLineAndCharacterOfPosition(node.getStart());
          const endPos = sourceFile.getLineAndCharacterOfPosition(node.getEnd());
          definitions.push({ name: decl.name.text, kind: "variable", startLine: startPos.line + 1, endLine: endPos.line + 1, startIndex: node.getStart(), endIndex: node.getEnd() });
        }
      }
      return;
    }
    if (name && kind) {
      const startPos = sourceFile.getLineAndCharacterOfPosition(node.getStart());
      const endPos = sourceFile.getLineAndCharacterOfPosition(node.getEnd());
      definitions.push({ name, kind, startLine: startPos.line + 1, endLine: endPos.line + 1, startIndex: node.getStart(), endIndex: node.getEnd() });
    }
    ts.forEachChild(node, visit);
  }
  ts.forEachChild(sourceFile, visit);
  return definitions;
}

// #endregion Definition Parser

// #region Rule Engine

const RULES: RegisteredRule[] = [];

function registerRule(meta: RuleMeta, run: RuleFn): void {
  RULES.push({ meta, run });
}

function createRuleContext(scope: Scope, projects: NxProject[]): RuleContext {
  const fileCache = new Map<string, string>();
  const regionCache = new Map<string, RegionInfo[]>();
  const definitionCache = new Map<string, DefinitionInfo[]>();
  return {
    scope,
    rootDir: ROOT_DIR,
    projects: () => projects,
    project: (name: string) => projects.find((p) => p.name === name),
    files: (pattern?: string) => {
      if (pattern) return simpleGlob(pattern, { cwd: ROOT_DIR, ignore: ["**/node_modules/**", "**/.venv/**"] });
      return scopeToFiles(scope, projects);
    },
    readText: (filePath: string) => {
      const absPath = join(ROOT_DIR, filePath);
      if (!fileCache.has(absPath)) {
        fileCache.set(absPath, existsSync(absPath) ? readTextFile(absPath) : "");
      }
      return fileCache.get(absPath)!;
    },
    regions: (filePath: string) => {
      if (!regionCache.has(filePath)) {
        const content = existsSync(join(ROOT_DIR, filePath)) ? readTextFile(join(ROOT_DIR, filePath)) : "";
        const lang = getLanguageFromPath(filePath);
        regionCache.set(filePath, lang ? parseRegions(content, lang) : []);
      }
      return regionCache.get(filePath)!;
    },
    definitions: (filePath: string) => {
      if (!definitionCache.has(filePath)) {
        const content = existsSync(join(ROOT_DIR, filePath)) ? readTextFile(join(ROOT_DIR, filePath)) : "";
        definitionCache.set(filePath, parseDefinitions(content, filePath));
      }
      return definitionCache.get(filePath)!;
    },
    createIssue: (partial) => ({ ...partial, id: `${partial.kind}-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`, priority: RULES.find((r) => r.meta.id === partial.kind)?.meta.priority ?? "medium", autofixable: RULES.find((r) => r.meta.id === partial.kind)?.meta.autofixable ?? false }),
    createFix: (description, edits) => ({ description, edits }),
  };
}

async function runRules(scope: Scope, ruleIds?: string[]): Promise<Issue[]> {
  const projects = getNxProjects();
  const issues: Issue[] = [];
  const rulesToRun = ruleIds ? RULES.filter((r) => ruleIds.includes(r.meta.id)) : RULES.filter((r) => matchesScope(r.meta.scopes, scope));
  for (const rule of rulesToRun) {
    const ctx = createRuleContext(scope, projects);
    const ruleIssues = await rule.run(ctx);
    issues.push(...ruleIssues);
  }
  return issues;
}

// #endregion Rule Engine

// #region Built-in Rules

//#region Header Region Rule
registerRule(
  { id: "header-region", name: "Header Region", description: "Ensures source files have a proper header region with SPDX license", scopes: ["**/*.{ts,tsx,py,cs}"], priority: "high", autofixable: true },
  async (ctx) => {
    const issues: Issue[] = [];
    const files = ctx.files();
    for (const file of files) {
      const content = ctx.readText(file);
      if (!content) continue;
      const lang = getLanguageFromPath(file);
      if (!lang) continue;
      const regions = ctx.regions(file);
      const headerRegion = regions.find((r) => r.name.toLowerCase() === "header");
      if (!headerRegion) {
        issues.push(ctx.createIssue({ summary: `Missing header region in ${file}`, kind: "header-region", severity: "error", solution: "Add a header region with SPDX license information", reason: "Every source file must include an SPDX license header", scope: file }));
      }
    }
    return issues;
  }
);
//#endregion Header Region Rule

//#region Empty Region Rule
registerRule(
  { id: "empty-region", name: "Empty Region", description: "Detects empty region blocks", scopes: ["**/*.{ts,tsx,py,cs}"], priority: "low", autofixable: true },
  async (ctx) => {
    const issues: Issue[] = [];
    const files = ctx.files();
    function checkRegion(file: string, region: RegionInfo, content: string): void {
      const regionContent = content.slice(region.startIndex, region.endIndex);
      const lines = regionContent.split("\n").slice(1, -1);
      const nonEmptyLines = lines.filter((l) => l.trim() && !l.trim().startsWith("//") && !l.trim().startsWith("#"));
      if (nonEmptyLines.length === 0 && region.children.length === 0) {
        issues.push(ctx.createIssue({ summary: `Empty region "${region.name}" in ${file}`, kind: "empty-region", severity: "warning", solution: "Remove the empty region or add content to it", reason: "Empty regions add noise without providing value", scope: `${file}#${region.name}`, line: region.startLine }));
      }
      for (const child of region.children) {
        checkRegion(file, child, content);
      }
    }
    for (const file of files) {
      const content = ctx.readText(file);
      const regions = ctx.regions(file);
      for (const region of regions) {
        checkRegion(file, region, content);
      }
    }
    return issues;
  }
);
//#endregion Empty Region Rule

//#region Comment Rule
registerRule(
  { id: "inline-comment", name: "Inline Comment", description: "Detects inline comments (documentation should be in README.md and AGENTS.md)", scopes: ["**/*.{ts,tsx}"], priority: "medium", autofixable: false },
  async (ctx) => {
    const issues: Issue[] = [];
    const files = ctx.files();
    for (const file of files) {
      const content = ctx.readText(file);
      if (!content) continue;
      const lines = content.split("\n");
      for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        const trimmed = line.trim();
        if (trimmed.startsWith("// #region") || trimmed.startsWith("// #endregion") || trimmed.startsWith("//#region") || trimmed.startsWith("//#endregion")) continue;
        if (trimmed.startsWith("// SPDX") || trimmed.includes("Copyright") || trimmed.includes("License")) continue;
        if (trimmed.startsWith("[DEBUG]") || trimmed.includes("[DEBUG]")) continue;
        const commentMatch = line.match(/\/\/(?!\s*#region|\s*#endregion)/);
        if (commentMatch && !trimmed.startsWith("//") && !line.includes("://")) {
          issues.push(ctx.createIssue({ summary: `Inline comment in ${file}:${i + 1}`, kind: "inline-comment", severity: "warning", solution: "Remove the comment and document in README.md or AGENTS.md", reason: "Code is never documented inline", scope: file, line: i + 1, excerpt: trimmed.slice(0, 80) }));
        }
      }
    }
    return issues;
  }
);
//#endregion Comment Rule

// #endregion Built-in Rules

// #region Ticket Engine

function getTicketPath(year: number, month: number, day: number, slug: string): string {
  return join(TICKETS_DIR, String(year), padNumber(month), padNumber(day), `${slug}.md`);
}

function createTicket(title: string, prompt: string): Ticket {
  const now = new Date();
  const { year, month, day } = formatDate(now);
  const slug = slugify(title);
  const filePath = getTicketPath(year, month, day, slug);
  let gitAuthor = "";
  try {
    const name = execSync("git config --get user.name", { encoding: "utf-8" }).trim();
    const email = execSync("git config --get user.email", { encoding: "utf-8" }).trim();
    gitAuthor = email ? `${name} <${email}>` : name;
  } catch {}
  let gitCommit = "";
  try {
    gitCommit = execSync("git rev-parse HEAD", { encoding: "utf-8" }).trim();
  } catch {}
  const frontmatter: TicketFrontmatter = { slug, prompt, status: "open", author: gitAuthor || undefined, date: { created: isoTimestamp() }, commit: gitCommit || undefined };
  const content = `
# Previously

# Plan

# Changes
`.trim();
  const ticket: Ticket = { year, month, day, slug, frontmatter, content, filePath };
  const fileContent = matter.stringify(content, frontmatter as any);
  ensureDir(dirname(filePath));
  writeTextFile(filePath, fileContent);
  return ticket;
}

function readTicket(year: number, month: number, day: number, slug: string): Ticket | null {
  const filePath = getTicketPath(year, month, day, slug);
  if (!existsSync(filePath)) return null;
  const raw = readTextFile(filePath);
  const parsed = matter(raw);
  return { year, month, day, slug, frontmatter: parsed.data as TicketFrontmatter, content: parsed.content, filePath };
}

function listTickets(year?: number, month?: number, day?: number): Ticket[] {
  const tickets: Ticket[] = [];
  if (!existsSync(TICKETS_DIR)) return tickets;
  const years = year ? [String(year)] : readdirSync(TICKETS_DIR).filter((f) => statSync(join(TICKETS_DIR, f)).isDirectory());
  for (const y of years) {
    const yearPath = join(TICKETS_DIR, y);
    if (!existsSync(yearPath)) continue;
    const months = month ? [padNumber(month)] : readdirSync(yearPath).filter((f) => statSync(join(yearPath, f)).isDirectory());
    for (const m of months) {
      const monthPath = join(yearPath, m);
      if (!existsSync(monthPath)) continue;
      const days = day ? [padNumber(day)] : readdirSync(monthPath).filter((f) => statSync(join(monthPath, f)).isDirectory());
      for (const d of days) {
        const dayPath = join(monthPath, d);
        if (!existsSync(dayPath)) continue;
        const files = readdirSync(dayPath).filter((f) => f.endsWith(".md"));
        for (const f of files) {
          const slug = f.replace(".md", "");
          const ticket = readTicket(parseInt(y), parseInt(m), parseInt(d), slug);
          if (ticket) tickets.push(ticket);
        }
      }
    }
  }
  return tickets;
}

function updateTicketIssues(ticket: Ticket, issues: Issue[]): void {
  let issuesSection = "## Issues\n\n";
  if (issues.length === 0) {
    issuesSection += "(No issues)\n";
  } else {
    for (const issue of issues) {
      const checkbox = issue.autofixable ? "[ ]" : "[x]";
      issuesSection += `- ${checkbox} (${issue.priority}) \`${issue.kind}\` in \`${issue.scope}\`\n`;
      issuesSection += `  - Summary: ${issue.summary}\n`;
      issuesSection += `  - Solution: ${issue.solution}\n`;
    }
  }
  let newContent = ticket.content;
  const issuesRegex = /## Issues[\s\S]*?(?=\n## |$)/;
  if (issuesRegex.test(newContent)) {
    newContent = newContent.replace(issuesRegex, issuesSection);
  } else {
    newContent += "\n\n" + issuesSection;
  }
  const fileContent = matter.stringify(newContent, ticket.frontmatter as any);
  writeTextFile(ticket.filePath, fileContent);
}

function canCloseTicket(ticket: Ticket): { canClose: boolean; reasons: string[] } {
  const reasons: string[] = [];
  const content = ticket.content;
  const issuesMatch = content.match(/## Issues[\s\S]*?(?=\n## |$)/);
  if (issuesMatch) {
    const issuesContent = issuesMatch[0];
    if (!issuesContent.includes("(No issues)") && issuesContent.includes("- [")) {
      reasons.push("Issues section is not empty");
    }
  }
  const planMatch = content.match(/# Plan[\s\S]*?(?=\n# |$)/);
  if (!planMatch || planMatch[0].trim() === "# Plan") {
    reasons.push("Plan section is empty");
  }
  const changesMatch = content.match(/# Changes[\s\S]*?(?=\n# |$)/);
  if (!changesMatch || changesMatch[0].trim() === "# Changes") {
    reasons.push("Changes section is empty");
  }
  return { canClose: reasons.length === 0, reasons };
}

function closeTicket(ticket: Ticket): boolean {
  const { canClose, reasons } = canCloseTicket(ticket);
  if (!canClose) {
    console.error("Cannot close ticket:", reasons.join(", "));
    return false;
  }
  ticket.frontmatter.status = "closed";
  ticket.frontmatter.date.finished = isoTimestamp();
  const fileContent = matter.stringify(ticket.content, ticket.frontmatter as any);
  writeTextFile(ticket.filePath, fileContent);
  return true;
}

// #endregion Ticket Engine

// #region Command Parser

function parseCommand(argv: string[]): ParsedCommand {
  const args = argv.slice(2);
  if (args.length === 0 || args[0] === "--help" || args[0] === "-h") {
    return { name: "help", args: [], options: {} };
  }
  const name = args[0] as CommandName;
  const options: Record<string, string | boolean> = {};
  const positionalArgs: string[] = [];
  let subcommand: string | undefined;
  for (let i = 1; i < args.length; i++) {
    const arg = args[i];
    if (arg.startsWith("--")) {
      const [key, value] = arg.slice(2).split("=");
      options[key] = value ?? true;
    } else if (arg.startsWith("-")) {
      options[arg.slice(1)] = true;
    } else if (!subcommand && ["list", "tree", "new", "read", "iterate", "close", "run"].includes(arg)) {
      subcommand = arg;
    } else {
      positionalArgs.push(arg);
    }
  }
  return { name, subcommand, args: positionalArgs, options };
}

// #endregion Command Parser

// #region Command Handlers

async function handleAnalyze(cmd: ParsedCommand): Promise<void> {
  const scopeRaw = (cmd.options.scope as string) || "@semio";
  const scope = parseScope(scopeRaw);
  const issues = await runRules(scope);
  const report: AnalyzeReport = {
    timestamp: isoTimestamp(),
    status: issues.some((i) => i.severity === "error") ? "error" : issues.length > 0 ? "warning" : "success",
    scope: scopeRaw,
    summary: {
      total: issues.length,
      byPriority: { high: issues.filter((i) => i.priority === "high").length, medium: issues.filter((i) => i.priority === "medium").length, low: issues.filter((i) => i.priority === "low").length },
      bySeverity: { error: issues.filter((i) => i.severity === "error").length, warning: issues.filter((i) => i.severity === "warning").length },
      byKind: issues.reduce(
        (acc, i) => {
          acc[i.kind] = (acc[i.kind] || 0) + 1;
          return acc;
        },
        {} as Record<string, number>
      ),
    },
    issues,
  };
  ensureDir(REPORTS_DIR);
  writeJsonFile(join(REPORTS_DIR, "analyze.json"), report);
  if (cmd.options.json) {
    console.log(JSON.stringify(report, null, 2));
  } else {
    console.log(`\n📊 Analysis complete: ${issues.length} issues found`);
    console.log(`   Errors: ${report.summary.bySeverity.error}, Warnings: ${report.summary.bySeverity.warning}`);
    console.log(`   Report: ${join(REPORTS_DIR, "analyze.json")}`);
  }
  process.exit(report.status === "error" ? 1 : 0);
}

async function handleFix(cmd: ParsedCommand): Promise<void> {
  const scopeRaw = (cmd.options.scope as string) || "@semio";
  const scope = parseScope(scopeRaw);
  const dryRun = !!cmd.options["dry-run"];
  const issues = await runRules(scope);
  const fixable = issues.filter((i) => i.autofixable && i.autofix);
  if (dryRun) {
    console.log(`\n🔧 Dry run: ${fixable.length} fixable issues found`);
    for (const issue of fixable) {
      console.log(`   - ${issue.kind}: ${issue.summary}`);
    }
  } else {
    let fixed = 0;
    for (const issue of fixable) {
      if (issue.autofix) {
        for (const [filePath, edits] of issue.autofix.edits) {
          const absPath = join(ROOT_DIR, filePath);
          let content = readTextFile(absPath);
          const sortedEdits = [...edits].sort((a, b) => b.start - a.start);
          for (const edit of sortedEdits) {
            content = content.slice(0, edit.start) + edit.newText + content.slice(edit.end);
          }
          writeTextFile(absPath, content);
        }
        fixed++;
      }
    }
    console.log(`\n✅ Fixed ${fixed} issues`);
  }
  process.exit(0);
}

async function handleRule(cmd: ParsedCommand): Promise<void> {
  if (cmd.subcommand === "list") {
    console.log("\n📜 Registered rules:\n");
    for (const rule of RULES) {
      console.log(`   ${rule.meta.id}`);
      console.log(`      ${rule.meta.name}: ${rule.meta.description}`);
      console.log(`      Priority: ${rule.meta.priority}, Autofixable: ${rule.meta.autofixable}`);
      console.log("");
    }
  } else if (cmd.subcommand === "run") {
    const ruleId = cmd.args[0];
    if (!ruleId) {
      console.error("Error: Rule ID required");
      process.exit(1);
    }
    const scopeRaw = (cmd.options.scope as string) || "@semio";
    const scope = parseScope(scopeRaw);
    const issues = await runRules(scope, [ruleId]);
    console.log(`\n📊 Rule "${ruleId}" found ${issues.length} issues`);
    for (const issue of issues.slice(0, 10)) {
      console.log(`   - ${issue.summary}`);
    }
    if (issues.length > 10) {
      console.log(`   ... and ${issues.length - 10} more`);
    }
  } else {
    console.log("Usage: repo rule <list|run> [id]");
  }
  process.exit(0);
}

async function handleTicket(cmd: ParsedCommand): Promise<void> {
  if (cmd.subcommand === "new") {
    const title = cmd.args.join(" ") || "New Ticket";
    const prompt = (cmd.options.prompt as string) || title;
    const ticket = createTicket(title, prompt);
    console.log(`\n🎫 Created ticket: ${ticket.slug}`);
    console.log(`   Path: ${ticket.filePath}`);
  } else if (cmd.subcommand === "list") {
    const pathParts = cmd.args[0]?.split("/") || [];
    const year = pathParts[0] ? parseInt(pathParts[0]) : undefined;
    const month = pathParts[1] ? parseInt(pathParts[1]) : undefined;
    const day = pathParts[2] ? parseInt(pathParts[2]) : undefined;
    const tickets = listTickets(year, month, day);
    console.log(`\n🎫 Found ${tickets.length} tickets:\n`);
    for (const ticket of tickets) {
      const status = ticket.frontmatter.status === "open" ? "🟢" : "✅";
      console.log(`   ${status} ${ticket.year}/${padNumber(ticket.month)}/${padNumber(ticket.day)}/${ticket.slug}`);
      if (ticket.frontmatter.summary) {
        console.log(`      ${ticket.frontmatter.summary}`);
      }
    }
  } else if (cmd.subcommand === "read") {
    const pathParts = cmd.args[0]?.split("/") || [];
    if (pathParts.length < 4) {
      console.error("Error: Path format should be YYYY/MM/DD/SLUG");
      process.exit(1);
    }
    const ticket = readTicket(parseInt(pathParts[0]), parseInt(pathParts[1]), parseInt(pathParts[2]), pathParts[3]);
    if (!ticket) {
      console.error("Error: Ticket not found");
      process.exit(1);
    }
    console.log(`\n🎫 Ticket: ${ticket.slug}`);
    console.log(`   Status: ${ticket.frontmatter.status}`);
    console.log(`   Created: ${ticket.frontmatter.date.created}`);
    console.log(`   Prompt: ${ticket.frontmatter.prompt}`);
    console.log(`\n${ticket.content}`);
  } else if (cmd.subcommand === "iterate") {
    const pathParts = cmd.args[0]?.split("/") || [];
    if (pathParts.length < 4) {
      console.error("Error: Path format should be YYYY/MM/DD/SLUG");
      process.exit(1);
    }
    const ticket = readTicket(parseInt(pathParts[0]), parseInt(pathParts[1]), parseInt(pathParts[2]), pathParts[3]);
    if (!ticket) {
      console.error("Error: Ticket not found");
      process.exit(1);
    }
    const scopeRaw = (cmd.options.scope as string) || "@semio";
    const scope = parseScope(scopeRaw);
    const issues = await runRules(scope);
    updateTicketIssues(ticket, issues);
    console.log(`\n🔄 Updated ticket issues: ${issues.length} issues found`);
  } else if (cmd.subcommand === "close") {
    const pathParts = cmd.args[0]?.split("/") || [];
    if (pathParts.length < 4) {
      console.error("Error: Path format should be YYYY/MM/DD/SLUG");
      process.exit(1);
    }
    const ticket = readTicket(parseInt(pathParts[0]), parseInt(pathParts[1]), parseInt(pathParts[2]), pathParts[3]);
    if (!ticket) {
      console.error("Error: Ticket not found");
      process.exit(1);
    }
    if (closeTicket(ticket)) {
      console.log(`\n✅ Ticket closed: ${ticket.slug}`);
    } else {
      process.exit(1);
    }
  } else {
    console.log("Usage: repo ticket <new|list|read|iterate|close> [args]");
  }
  process.exit(0);
}

async function handleProject(cmd: ParsedCommand): Promise<void> {
  const projects = getNxProjects();
  if (cmd.subcommand === "list") {
    console.log(`\n📦 Found ${projects.length} projects:\n`);
    for (const project of projects) {
      console.log(`   ${project.name}`);
      console.log(`      Root: ${project.root}`);
      if (project.tags?.length) {
        console.log(`      Tags: ${project.tags.join(", ")}`);
      }
    }
  } else if (cmd.subcommand === "tree") {
    console.log("\n📦 Project tree:\n");
    for (const project of projects) {
      console.log(`   └── ${project.name} (${project.root})`);
    }
  } else {
    console.log("Usage: repo project <list|tree>");
  }
  process.exit(0);
}

async function handleFolder(cmd: ParsedCommand): Promise<void> {
  const folderPath = cmd.args[0] || ".";
  const absPath = join(ROOT_DIR, folderPath);
  if (!existsSync(absPath)) {
    console.error(`Error: Folder not found: ${folderPath}`);
    process.exit(1);
  }
  if (cmd.subcommand === "tree") {
    console.log(`\n📁 Folder tree: ${folderPath}\n`);
    function printTree(dir: string, prefix: string = ""): void {
      const allItems = readdirSync(dir).filter((f) => !f.startsWith("."));
      const relativePaths = allItems.map((item) => getRelativePath(join(dir, item)));
      const ignoredSet = getGitIgnoredSet(relativePaths);
      const items = allItems.filter((item) => {
        const relPath = normalizePathSeparators(getRelativePath(join(dir, item)));
        return !ignoredSet.has(relPath) && !ignoredSet.has(relPath + "/");
      });
      items.forEach((item, index) => {
        const isLast = index === items.length - 1;
        const fullPath = join(dir, item);
        const isDir = statSync(fullPath).isDirectory();
        console.log(`${prefix}${isLast ? "└── " : "├── "}${item}${isDir ? "/" : ""}`);
        if (isDir) {
          printTree(fullPath, prefix + (isLast ? "    " : "│   "));
        }
      });
    }
    printTree(absPath);
  } else {
    console.log("Usage: repo folder tree [path]");
  }
  process.exit(0);
}

async function handleFile(cmd: ParsedCommand): Promise<void> {
  const scopeRaw = cmd.args[0] || "@semio";
  const scope = parseScope(scopeRaw);
  const projects = getNxProjects();
  const files = scopeToFiles(scope, projects);
  if (cmd.subcommand === "list" || !cmd.subcommand) {
    console.log(`\n📄 Found ${files.length} files in scope "${scopeRaw}":\n`);
    for (const file of files.slice(0, 50)) {
      console.log(`   ${file}`);
    }
    if (files.length > 50) {
      console.log(`   ... and ${files.length - 50} more`);
    }
  }
  process.exit(0);
}

async function handleRegion(cmd: ParsedCommand): Promise<void> {
  const filePath = cmd.args[0];
  if (!filePath) {
    console.error("Error: File path required");
    process.exit(1);
  }
  const absPath = join(ROOT_DIR, filePath);
  if (!existsSync(absPath)) {
    console.error(`Error: File not found: ${filePath}`);
    process.exit(1);
  }
  const content = readTextFile(absPath);
  const lang = getLanguageFromPath(filePath);
  if (!lang) {
    console.error("Error: Unsupported file type");
    process.exit(1);
  }
  const regions = parseRegions(content, lang);
  if (cmd.subcommand === "tree" || !cmd.subcommand) {
    console.log(`\n🏷️ Regions in ${filePath}:\n`);
    function printRegion(region: RegionInfo, prefix: string = ""): void {
      console.log(`${prefix}└── ${region.name} (lines ${region.startLine}-${region.endLine})`);
      for (const child of region.children) {
        printRegion(child, prefix + "    ");
      }
    }
    for (const region of regions) {
      printRegion(region);
    }
    if (regions.length === 0) {
      console.log("   (no regions found)");
    }
  }
  process.exit(0);
}

async function handleDefinition(cmd: ParsedCommand): Promise<void> {
  const filePath = cmd.args[0];
  if (!filePath) {
    console.error("Error: File path required");
    process.exit(1);
  }
  const absPath = join(ROOT_DIR, filePath);
  if (!existsSync(absPath)) {
    console.error(`Error: File not found: ${filePath}`);
    process.exit(1);
  }
  const content = readTextFile(absPath);
  const definitions = parseDefinitions(content, filePath);
  if (cmd.subcommand === "list" || !cmd.subcommand) {
    console.log(`\n📋 Definitions in ${filePath}:\n`);
    for (const def of definitions) {
      console.log(`   ${def.kind}: ${def.name} (lines ${def.startLine}-${def.endLine})`);
    }
    if (definitions.length === 0) {
      console.log("   (no definitions found)");
    }
  }
  process.exit(0);
}

async function handleTool(cmd: ParsedCommand): Promise<void> {
  const target = cmd.args[0];
  if (!target) {
    console.error("Error: Tool/target name required");
    process.exit(1);
  }
  const projectScope = cmd.options.scope as string | undefined;
  const projects = projectScope ? [projectScope] : undefined;
  const extraArgs = cmd.args.slice(1);
  console.log(`\n🔧 Running Nx target: ${target}`);
  const result = runNxTarget(target, projects, extraArgs);
  process.exit(result.success ? 0 : 1);
}

// #endregion Command Handlers

// #region Ink App

interface AppProps {
  command: ParsedCommand;
}

function App({ command }: AppProps) {
  const { exit } = useApp();
  const [status, setStatus] = React.useState<"running" | "done">("running");
  const [message, setMessage] = React.useState<string>("Initializing...");

  React.useEffect(() => {
    (async () => {
      try {
        switch (command.name) {
          case "help":
            console.log(HELP_TEXT);
            break;
          case "analyze":
            await handleAnalyze(command);
            break;
          case "fix":
            await handleFix(command);
            break;
          case "rule":
            await handleRule(command);
            break;
          case "ticket":
            await handleTicket(command);
            break;
          case "project":
            await handleProject(command);
            break;
          case "folder":
            await handleFolder(command);
            break;
          case "file":
            await handleFile(command);
            break;
          case "region":
            await handleRegion(command);
            break;
          case "definition":
            await handleDefinition(command);
            break;
          case "tool":
            await handleTool(command);
            break;
          default:
            console.log(HELP_TEXT);
        }
        setStatus("done");
        exit();
      } catch (error) {
        setMessage(`Error: ${error instanceof Error ? error.message : String(error)}`);
        setStatus("done");
        exit();
      }
    })();
  }, [command, exit]);

  if (status === "done") return null;

  return (
    <Box flexDirection="column">
      <Text>🔄 {message}</Text>
    </Box>
  );
}

// #endregion Ink App

// #region Main

const command = parseCommand(process.argv);
if (command.name === "help") {
  console.log(HELP_TEXT);
  process.exit(0);
}
render(<App command={command} />);

// #endregion Main
