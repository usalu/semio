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

import { exportKit, importKit, Kit } from "@semio/js/semio";
import { execSync, spawnSync } from "child_process";
import fg from "fast-glob";
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "fs";
import Ignore from "ignore";
import matter from "gray-matter";
import { Box, render, Text } from "ink";
import { dirname, join, relative } from "path";
import React from "react";
import Parser from "tree-sitter";
import CSharp from "tree-sitter-c-sharp";
import JavaScript from "tree-sitter-javascript";
import Python from "tree-sitter-python";
import TypeScript from "tree-sitter-typescript";
import * as ts from "typescript";
import { fileURLToPath } from "url";

// #endregion Imports

// #region Types

// #region Scope
type ScopeKind = "repo" | "project" | "folder" | "file" | "section" | "definition";

interface Scope {
  raw: string;
  kind: ScopeKind;
  projectName?: string;
  filePath?: string;
  sectionPath?: string[];
  definitionName?: string;
}
// #endregion Scope

// #region Violation
type ViolationPriority = "high" | "medium" | "low";

interface Violation {
  id: string;
  summary: string;
  kind: string;
  priority: ViolationPriority;
  autofixable: boolean;
  solution: string;
  reason: string;
  scope: string;
  line?: number;
  column?: number;
  excerpt?: string;
  autofix?: Fix;
}
// #endregion Violation

// #region Fix
interface TextEdit {
  start: number;
  end: number;
  newText: string;
}

interface Fix {
  description: string;
  edits: Map<string, TextEdit[]>;
}
// #endregion Fix

// #region Project
interface NxProject {
  name: string;
  root: string;
  sourceRoot?: string;
  projectType?: "library" | "application";
  tags?: string[];
}
// #endregion Project

// #region Section
interface SectionInfo {
  name: string;
  startLine: number;
  endLine: number;
  startIndex: number;
  endIndex: number;
  children: SectionInfo[];
}
// #endregion Section

// #region Definition
interface DefinitionInfo {
  name: string;
  kind: "function" | "class" | "variable" | "interface" | "type" | "enum" | "method" | "property";
  startLine: number;
  endLine: number;
  startIndex: number;
  endIndex: number;
}
// #endregion Definition

// #region AST
interface ASTNode {
  type: string;
  text: string;
  startPosition: { row: number; column: number };
  endPosition: { row: number; column: number };
  startIndex: number;
  endIndex: number;
  children: ASTNode[];
  parent: ASTNode | null;
}

interface ASTFile {
  filePath: string;
  root: ASTNode | null;
  language: string | null;
}
// #endregion AST

// #region Ticket
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
// #endregion Ticket

// #region Policy
interface PolicyMeta {
  id: string;
  name: string;
  description: string;
  scopes: string[];
  priority: ViolationPriority;
}

interface PolicyContext {
  scope: Scope;
  rootDir: string;
  projects: () => NxProject[];
  project: (name: string) => NxProject | undefined;
  files: (pattern?: string) => string[];
  readText: (filePath: string) => string;
  sections: (filePath: string) => SectionInfo[];
  definitions: (filePath: string) => DefinitionInfo[];
  parseAST: (filePath: string) => ASTFile | null;
  getASTNode: (filePath: string, startIndex: number, endIndex: number) => ASTNode | null;
  queryAST: (filePath: string, query: string) => Array<{ node: ASTNode; captures: Record<string, ASTNode[]> }>;
  createViolation: (partial: Omit<Violation, "id" | "priority" | "autofixable">) => Violation;
  createFix: (description: string, edits: Map<string, TextEdit[]>) => Fix;
}

type PolicyFn = (ctx: PolicyContext) => Promise<Violation[]>;

interface RegisteredPolicy {
  meta: PolicyMeta;
  run: PolicyFn;
}
// #endregion Policy

// #region Report
interface AnalyzeReport {
  timestamp: string;
  status: "success" | "error" | "warning";
  scope: string;
  summary: { total: number; byPriority: Record<ViolationPriority, number>; byKind: Record<string, number> };
  violations: Violation[];
}
// #endregion Report

// #region Command
type CommandName = "help" | "analyze" | "fix" | "policy" | "ticket" | "project" | "folder" | "file" | "section" | "definition" | "tool";

interface ParsedCommand {
  name: CommandName;
  subcommand?: string;
  subsubcommand?: string;
  args: string[];
  options: Record<string, string | boolean>;
}
// #endregion Command

// #region Output
type OutputType = "info" | "success" | "error" | "warn" | "plain";

interface OutputLine {
  type: OutputType;
  text: string;
}

interface CommandOutput {
  lines: OutputLine[];
  exitCode: number;
}

function createOutput(): CommandOutput {
  return { lines: [], exitCode: 0 };
}

function addLine(output: CommandOutput, type: OutputType, text: string): void {
  output.lines.push({ type, text });
}

function info(output: CommandOutput, text: string): void {
  addLine(output, "info", text);
}

function success(output: CommandOutput, text: string): void {
  addLine(output, "success", text);
}

function error(output: CommandOutput, text: string): void {
  addLine(output, "error", text);
  output.exitCode = 1;
}

function warn(output: CommandOutput, text: string): void {
  addLine(output, "warn", text);
}

function plain(output: CommandOutput, text: string): void {
  addLine(output, "plain", text);
}
// #endregion Output

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
  help                                          Show this help message
  analyze [--scope=<scope>]                     Analyze codebase for violations (multiple scopes supported)
  fix [--scope=<scope>]                         Apply autofixes for violations (multiple scopes supported)
  policy list [--id=<id-pattern>] [--scope=<scope>]  List all registered policies
  policy run [--scope=<scope>] [--id=<id>]        Run specific policies
  ticket create <slug> [--prompt=<prompt>] [--model=<model>]  Create a new ticket
  ticket iterate start <year> <month> <day> <slug>  Start a ticket iteration
  ticket iterate end <year> <month> <day> <slug>    End a ticket iteration
  ticket finish <year> <month> <day> <slug>     Finish a ticket
  ticket list [--year=<year>] [--month=<month>] [--day=<day>]  List tickets
  ticket read <year> <month> <day> <slug>       Read a ticket
  project list [--scope=<scope>]                List Nx projects
  project tree [--scope=<scope>]                Show project dependency tree
  folder create <folder-path>                   Create a folder
  folder move <folder-path> <new-folder-path>   Move a folder
  folder delete <folder-path>                   Delete a folder
  folder list [--scope=<scope>]                 List folders in scope
  folder tree [--scope=<scope>]                 Show folder structure
  file create <file-path>                       Create a file
  file move <file-path> <new-file-path>         Move a file
  file delete <file-path>                       Delete a file
  file list [--scope=<scope>]                   List files in scope
  file tree [--scope=<scope>]                   Show file structure
  section create <file-path> <section-path>     Create a section in a file
  section move <file-path> <section-path> <new-section-path>  Move a section in a file
  section delete <file-path> <section-path>     Delete a section in a file
  section list [--scope=<scope>]                List sections in a file
  section tree [--scope=<scope>]                Show section structure of a file
  definition list [--scope=<scope>]             List definitions in a file
  definition tree [--scope=<scope>]             Show definition structure
  tool <name> [args...]                         Run a tool (e.g., i18n, update-metabolism)

Options:
  --scope=<scope>          Limit operation to scope
  --id=<id>                Filter by policy ID or pattern
  --json                   Output as JSON
  --dry-run                Preview without making changes
  --help, -h               Show help for command

Scope syntax:
  @semio                   Repo scope (all files)
  @semio/js                Project scope (Nx project)
  js/js/sketchpad/         Folder scope
  js/js/sketchpad/App.tsx  File scope
  file.tsx#Section         Section scope (regions in code, headers in markdown)
  file.tsx§Function        Definition scope
`;

// #endregion Constants

// #region Utilities

// #region Path Utilities
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
// #endregion Path Utilities

// #region File Operations
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

function loadGitignore(cwd: string): ReturnType<typeof Ignore> {
  const ig = Ignore();
  const gitignorePath = join(cwd, ".gitignore");
  if (existsSync(gitignorePath)) {
    const content = readFileSync(gitignorePath, "utf-8");
    ig.add(content);
  }
  return ig;
}

function simpleGlob(pattern: string, options: { cwd?: string; ignore?: string[]; respectGitignore?: boolean } = {}): string[] {
  const cwd = options.cwd ?? ROOT_DIR;
  const ignorePatterns = options.ignore ?? [];
  const respectGitignore = options.respectGitignore ?? true;
  const gitignoreFilter = respectGitignore ? loadGitignore(cwd) : null;
  const files = fg.sync(pattern, {
    cwd,
    ignore: ignorePatterns,
    dot: false,
    onlyFiles: true,
    absolute: false,
  });
  if (gitignoreFilter) {
    return files.filter((f) => !gitignoreFilter.ignores(f));
  }
  return files;
}
// #endregion File Operations

// #region Date Utilities
function formatDate(date: Date): { year: number; month: number; day: number } {
  return { year: date.getFullYear(), month: date.getMonth() + 1, day: date.getDate() };
}

function padNumber(n: number, width: number = 2): string {
  return String(n).padStart(width, "0");
}

function isoTimestamp(): string {
  return new Date().toISOString();
}
// #endregion Date Utilities

// #region String Utilities
function slugify(text: string): string {
  return text
    .toUpperCase()
    .replace(/[^A-Z0-9]+/g, "-")
    .replace(/^-|-$/g, "");
}
// #endregion String Utilities

// #region Process Management
function execCommand(command: string, args: string[] = [], options?: { cwd?: string }): { stdout: string; stderr: string; exitCode: number } {
  const result = spawnSync(command, args, { cwd: options?.cwd ?? ROOT_DIR, encoding: "utf-8", shell: true });
  return { stdout: result.stdout ?? "", stderr: result.stderr ?? "", exitCode: result.status ?? 1 };
}
// #endregion Process Management

// #region Git Utilities
function getGitIgnoredSet(paths: string[]): Set<string> {
  if (paths.length === 0) return new Set();
  const result = spawnSync("git", ["check-ignore", ...paths.map(normalizePathSeparators)], { cwd: ROOT_DIR, encoding: "utf-8", shell: true });
  if (result.status === null || result.stdout === null) return new Set();
  return new Set(result.stdout.split("\n").filter(Boolean).map(normalizePathSeparators));
}
// #endregion Git Utilities

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
    const [filePart, ...sectionParts] = raw.split("#");
    return { raw, kind: "section", filePath: filePart, sectionPath: sectionParts };
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
  if ((scope.kind === "file" || scope.kind === "section" || scope.kind === "definition") && scope.filePath) {
    return [scope.filePath];
  }
  return [];
}

function matchesScope(policyScopes: string[], targetScope: Scope): boolean {
  for (const pattern of policyScopes) {
    if (pattern === "*" || pattern === "**/*") return true;
    if (pattern.startsWith("@semio")) {
      if (targetScope.kind === "repo" || (targetScope.kind === "project" && targetScope.projectName?.startsWith(pattern))) return true;
    }
    if (targetScope.kind === "repo" && pattern.startsWith("**/*.")) return true;
    if (targetScope.filePath) {
      const normalizedTarget = normalizePathSeparators(targetScope.filePath);
      const normalizedPattern = normalizePathSeparators(pattern);
      if (normalizedTarget.includes(normalizedPattern)) return true;
      const regexPattern = normalizedPattern
        .replace(/\{([^}]+)\}/g, (_, group) => `(${group.replace(/,/g, "|")})`)
        .replace(/\./g, "\\.")
        .replace(/\*\*/g, "\0")
        .replace(/\*/g, "[^/]*")
        .replace(/\0/g, ".*");
      if (normalizedTarget.match(new RegExp(`^${regexPattern}$`))) return true;
    }
  }
  return false;
}

// #endregion Scope Parser

// #region Nx Adapter

let cachedProjectNames: string[] | null = null;
const cachedProjectDetails: Map<string, NxProject> = new Map();

function getNxProjectNames(): string[] {
  if (cachedProjectNames) return cachedProjectNames;
  try {
    const result = execCommand("npx", ["nx", "show", "projects", "--json"]);
    if (result.exitCode !== 0) {
      cachedProjectNames = [];
      return cachedProjectNames;
    }
    cachedProjectNames = JSON.parse(result.stdout);
    return cachedProjectNames;
  } catch {
    cachedProjectNames = [];
    return cachedProjectNames;
  }
}

function getNxProjectDetails(name: string): NxProject {
  if (cachedProjectDetails.has(name)) return cachedProjectDetails.get(name)!;
  const configResult = execCommand("npx", ["nx", "show", "project", name, "--json"]);
  if (configResult.exitCode === 0) {
    try {
      const config = JSON.parse(configResult.stdout);
      const project = { name, root: config.root ?? "", sourceRoot: config.sourceRoot, projectType: config.projectType, tags: config.tags };
      cachedProjectDetails.set(name, project);
      return project;
    } catch {
      const project = { name, root: "" };
      cachedProjectDetails.set(name, project);
      return project;
    }
  }
  const project = { name, root: "" };
  cachedProjectDetails.set(name, project);
  return project;
}

function getNxProjects(): NxProject[] {
  return getNxProjectNames().map((name) => getNxProjectDetails(name));
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

// #region Section Parser

function parseCodeSections(content: string, language: "typescript" | "python" | "csharp"): SectionInfo[] {
  const lines = content.split("\n");
  const stack: SectionInfo[] = [];
  const roots: SectionInfo[] = [];
  const sectionStartPatterns: Record<string, RegExp> = { typescript: /^\s*\/\/\s*#region\s+(.+?)\r?$/i, python: /^\s*#\s*region\s+(.+?)\r?$/i, csharp: /^\s*#region\s+(.+?)\r?$/i };
  const sectionEndPatterns: Record<string, RegExp> = { typescript: /^\s*\/\/\s*#endregion/i, python: /^\s*#\s*endregion/i, csharp: /^\s*#endregion/i };
  const startPattern = sectionStartPatterns[language];
  const endPattern = sectionEndPatterns[language];
  let charIndex = 0;
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const lineStart = charIndex;
    const startMatch = line.match(startPattern);
    if (startMatch) {
      const section: SectionInfo = { name: startMatch[1].trim(), startLine: i + 1, endLine: -1, startIndex: lineStart, endIndex: -1, children: [] };
      if (stack.length > 0) {
        stack[stack.length - 1].children.push(section);
      } else {
        roots.push(section);
      }
      stack.push(section);
    } else if (endPattern.test(line)) {
      const section = stack.pop();
      if (section) {
        section.endLine = i + 1;
        section.endIndex = charIndex + line.length;
      }
    }
    charIndex += line.length + 1;
  }
  return roots;
}

function parseMarkdownSections(content: string): SectionInfo[] {
  const parsed = matter(content);
  const bodyContent = parsed.content;
  const frontmatterLines = content.slice(0, content.indexOf(bodyContent)).split("\n").length - 1;
  const lines = bodyContent.split("\n");
  const sections: SectionInfo[] = [];
  const stack: { level: number; section: SectionInfo }[] = [];
  let charIndex = content.indexOf(bodyContent);
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const lineStart = charIndex;
    const headerMatch = line.match(/^(#{1,6})\s+(.+?)\r?$/);
    if (headerMatch) {
      const level = headerMatch[1].length;
      const name = headerMatch[2].trim();
      while (stack.length > 0 && stack[stack.length - 1].level >= level) {
        const popped = stack.pop()!;
        popped.section.endLine = frontmatterLines + i;
        popped.section.endIndex = lineStart - 1;
      }
      const section: SectionInfo = { name, startLine: frontmatterLines + i + 1, endLine: -1, startIndex: lineStart, endIndex: -1, children: [] };
      if (stack.length > 0) {
        stack[stack.length - 1].section.children.push(section);
      } else {
        sections.push(section);
      }
      stack.push({ level, section });
    }
    charIndex += line.length + 1;
  }
  while (stack.length > 0) {
    const popped = stack.pop()!;
    popped.section.endLine = frontmatterLines + lines.length;
    popped.section.endIndex = content.length;
  }
  return sections;
}

function parseSections(content: string, filePath: string): SectionInfo[] {
  const ext = filePath.split(".").pop()?.toLowerCase();
  if (ext === "md" || ext === "mdx") {
    return parseMarkdownSections(content);
  }
  const lang = getLanguageFromPath(filePath);
  if (lang) {
    return parseCodeSections(content, lang);
  }
  return [];
}

function getLanguageFromPath(filePath: string): "typescript" | "python" | "csharp" | null {
  const ext = filePath.split(".").pop()?.toLowerCase();
  if (ext === "ts" || ext === "tsx" || ext === "js" || ext === "jsx") return "typescript";
  if (ext === "py") return "python";
  if (ext === "cs") return "csharp";
  return null;
}

// #endregion Section Parser

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

// #region AST Parser

let parserCache: Map<string, Parser> | null = null;

function getParserForLanguage(language: string): Parser | null {
  if (!parserCache) {
    parserCache = new Map();
  }
  if (parserCache.has(language)) {
    return parserCache.get(language)!;
  }
  const parser = new Parser();
  let grammar: any = null;
  if (language === "typescript" || language === "tsx") {
    grammar = TypeScript;
    parser.setLanguage(language === "tsx" ? TypeScript.tsx : TypeScript.typescript);
  } else if (language === "javascript" || language === "jsx") {
    grammar = JavaScript;
    parser.setLanguage(language === "jsx" ? JavaScript.jsx : JavaScript.javascript);
  } else if (language === "python") {
    grammar = Python;
    parser.setLanguage(Python);
  } else if (language === "csharp") {
    grammar = CSharp;
    parser.setLanguage(CSharp);
  } else {
    return null;
  }
  parserCache.set(language, parser);
  return parser;
}

function getLanguageFromPathForAST(filePath: string): string | null {
  const ext = filePath.split(".").pop()?.toLowerCase();
  if (ext === "ts") return "typescript";
  if (ext === "tsx") return "tsx";
  if (ext === "js") return "javascript";
  if (ext === "jsx") return "jsx";
  if (ext === "py") return "python";
  if (ext === "cs") return "csharp";
  return null;
}

function convertTreeSitterNode(node: Parser.SyntaxNode, content: string, parent: ASTNode | null = null): ASTNode {
  const children: ASTNode[] = [];
  const astNode: ASTNode = {
    type: node.type,
    text: content.slice(node.startIndex, node.endIndex),
    startPosition: { row: node.startPosition.row, column: node.startPosition.column },
    endPosition: { row: node.endPosition.row, column: node.endPosition.column },
    startIndex: node.startIndex,
    endIndex: node.endIndex,
    children,
    parent,
  };
  for (let i = 0; i < node.childCount; i++) {
    const child = node.child(i);
    if (child) {
      children.push(convertTreeSitterNode(child, content, astNode));
    }
  }
  return astNode;
}

function parseAST(content: string, filePath: string): ASTFile | null {
  const language = getLanguageFromPathForAST(filePath);
  if (!language) return null;
  const parser = getParserForLanguage(language);
  if (!parser) return null;
  try {
    const tree = parser.parse(content);
    const root = tree.rootNode;
    if (!root) return null;
    return {
      filePath,
      root: convertTreeSitterNode(root, content),
      language,
    };
  } catch {
    return null;
  }
}

function getASTNodeAtPosition(astFile: ASTFile, startIndex: number, endIndex: number): ASTNode | null {
  if (!astFile.root) return null;
  function findNode(node: ASTNode): ASTNode | null {
    if (node.startIndex <= startIndex && node.endIndex >= endIndex) {
      for (const child of node.children) {
        const found = findNode(child);
        if (found && found.startIndex <= startIndex && found.endIndex >= endIndex) {
          return found;
        }
      }
      return node;
    }
    return null;
  }
  return findNode(astFile.root);
}

function queryAST(astFile: ASTFile, queryString: string): Array<{ node: ASTNode; captures: Record<string, ASTNode[]> }> {
  if (!astFile.root || !astFile.language) return [];
  const parser = getParserForLanguage(astFile.language);
  if (!parser) return [];
  try {
    const language = parser.getLanguage();
    const query = new Parser.Query(language, queryString);
    const content = astFile.root.text;
    const tree = parser.parse(content);
    const matches = query.matches(tree.rootNode);
    return matches.map((match) => {
      const captures: Record<string, ASTNode[]> = {};
      for (const capture of match.captures) {
        const name = capture.name;
        if (!captures[name]) {
          captures[name] = [];
        }
        captures[name].push(convertTreeSitterNode(capture.node, content));
      }
      const firstCapture = match.captures[0];
      return {
        node: firstCapture ? convertTreeSitterNode(firstCapture.node, content) : astFile.root!,
        captures,
      };
    });
  } catch {
    return [];
  }
}

// #endregion AST Parser

// #region Policy Engine

const RULES: RegisteredPolicy[] = [];

function registerPolicy(meta: PolicyMeta, run: PolicyFn): void {
  RULES.push({ meta, run });
}

function createPolicyContext(scope: Scope, projects: NxProject[]): PolicyContext {
  const fileCache = new Map<string, string>();
  const sectionCache = new Map<string, SectionInfo[]>();
  const definitionCache = new Map<string, DefinitionInfo[]>();
  const astCache = new Map<string, ASTFile | null>();
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
    sections: (filePath: string) => {
      if (!sectionCache.has(filePath)) {
        const content = existsSync(join(ROOT_DIR, filePath)) ? readTextFile(join(ROOT_DIR, filePath)) : "";
        sectionCache.set(filePath, parseSections(content, filePath));
      }
      return sectionCache.get(filePath)!;
    },
    definitions: (filePath: string) => {
      if (!definitionCache.has(filePath)) {
        const content = existsSync(join(ROOT_DIR, filePath)) ? readTextFile(join(ROOT_DIR, filePath)) : "";
        definitionCache.set(filePath, parseDefinitions(content, filePath));
      }
      return definitionCache.get(filePath)!;
    },
    parseAST: (filePath: string) => {
      if (!astCache.has(filePath)) {
        const content = existsSync(join(ROOT_DIR, filePath)) ? readTextFile(join(ROOT_DIR, filePath)) : "";
        astCache.set(filePath, parseAST(content, filePath));
      }
      return astCache.get(filePath)!;
    },
    getASTNode: (filePath: string, startIndex: number, endIndex: number) => {
      const astFile = (() => {
        if (!astCache.has(filePath)) {
          const content = existsSync(join(ROOT_DIR, filePath)) ? readTextFile(join(ROOT_DIR, filePath)) : "";
          astCache.set(filePath, parseAST(content, filePath));
        }
        return astCache.get(filePath)!;
      })();
      if (!astFile) return null;
      return getASTNodeAtPosition(astFile, startIndex, endIndex);
    },
    queryAST: (filePath: string, query: string) => {
      const astFile = (() => {
        if (!astCache.has(filePath)) {
          const content = existsSync(join(ROOT_DIR, filePath)) ? readTextFile(join(ROOT_DIR, filePath)) : "";
          astCache.set(filePath, parseAST(content, filePath));
        }
        return astCache.get(filePath)!;
      })();
      if (!astFile) return [];
      return queryAST(astFile, query);
    },
    createViolation: (partial) => ({
      ...partial,
      id: `${partial.kind}-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
      priority: RULES.find((r) => partial.kind === r.meta.id || partial.kind.startsWith(r.meta.id + ":"))?.meta.priority ?? "medium",
      autofixable: !!partial.autofix,
    }),
    createFix: (description, edits) => ({ description, edits }),
  };
}

async function runPolicies(scope: Scope, policyIds?: string[]): Promise<Violation[]> {
  const needsProjects = scope.kind === "repo" || scope.kind === "project";
  const projects = needsProjects ? getNxProjects() : [];
  const violations: Violation[] = [];
  const policiesToRun = policyIds ? RULES.filter((r) => policyIds.includes(r.meta.id)) : RULES.filter((r) => matchesScope(r.meta.scopes, scope));
  for (const policy of policiesToRun) {
    const ctx = createPolicyContext(scope, projects);
    const policyViolations = await policy.run(ctx);
    violations.push(...policyViolations);
  }
  return violations;
}

// #endregion Policy Engine

// #region Policies

// #region Header
registerPolicy({ id: "header", name: "Header", description: "Validates source file header section with filename, contributors, and license", scopes: ["**/*.{ts,tsx,py,cs}"], priority: "low" }, async (ctx) => {
  const violations: Violation[] = [];
  const files = ctx.files();
  const agplMarkers = ["GNU Affero General Public License", "AGPL", "https://www.gnu.org/licenses/"];
  for (const file of files) {
    const content = ctx.readText(file);
    if (!content) continue;
    const lang = getLanguageFromPath(file);
    if (!lang) continue;
    const sections = ctx.sections(file);
    const headerSection = sections.find((r) => r.name.toLowerCase() === "header");
    if (!headerSection) {
      const filename = file.split("/").pop() ?? file;
      const year = new Date().getFullYear();
      let gitAuthor = "";
      try {
        const name = execSync("git config --get user.name", { encoding: "utf-8" }).trim();
        const email = execSync("git config --get user.email", { encoding: "utf-8" }).trim();
        gitAuthor = email ? `${name} <${email}>` : name;
      } catch {}
      const projects = ctx.projects();
      const project = projects.find((p) => file.startsWith(p.root + "/") || file.startsWith(p.root));
      let licenseType: "agpl" | "lgpl" | "mit" | "cc0" | "cc-by-nd" = "agpl";
      if (file.includes("/examples/") || file.startsWith("examples/")) {
        licenseType = "mit";
      } else if (file.includes("/templates/") || file.startsWith("templates/")) {
        licenseType = "cc0";
      } else if (file.includes("/assets/") || file.startsWith("assets/")) {
        licenseType = "cc-by-nd";
      } else if (project?.projectType === "library") {
        licenseType = "lgpl";
      } else if (project?.projectType === "application") {
        licenseType = "agpl";
      }
      const licenseTexts: Record<typeof licenseType, { text: string; url: string }> = {
        agpl: {
          text: `This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.`,
          url: "https://www.gnu.org/licenses/agpl-3.0.en.html",
        },
        lgpl: {
          text: `This library is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This library is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public License
along with this library.  If not, see <https://www.gnu.org/licenses/>.`,
          url: "https://www.gnu.org/licenses/lgpl-3.0.en.html",
        },
        mit: {
          text: `Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.`,
          url: "https://mit-license.org",
        },
        cc0: {
          text: `To the extent possible under law, the author(s) have dedicated all copyright
and related and neighboring rights to this software to the public domain
worldwide. This software is distributed without any warranty.

See <https://creativecommons.org/publicdomain/zero/1.0/>.`,
          url: "https://creativecommons.org/publicdomain/zero/1.0/",
        },
        "cc-by-nd": {
          text: `This work is licensed under the Creative Commons Attribution-NoDerivatives
4.0 International License. To view a copy of this license, visit
<https://creativecommons.org/licenses/by-nd/4.0/>.`,
          url: "https://creativecommons.org/licenses/by-nd/4.0/",
        },
      };
      const license = licenseTexts[licenseType];
      const formatLicenseForLang = (text: string, commentPrefix: string): string => {
        return text
          .split("\n")
          .map((line) => (line ? `${commentPrefix} ${line}` : commentPrefix))
          .join("\n");
      };
      let headerContent = "";
      if (lang === "typescript") {
        headerContent = `// #region Header

// ${filename}

// ${year} ${gitAuthor}

${formatLicenseForLang(license.text, "//")}

// #endregion Header

`;
      } else if (lang === "python") {
        headerContent = `# region Header

# ${filename}

# ${year} ${gitAuthor}

${formatLicenseForLang(license.text, "#")}

# endregion Header

`;
      } else if (lang === "csharp") {
        headerContent = `#region Header

// ${filename}

// ${year} ${gitAuthor}

${formatLicenseForLang(license.text, "//")}

#endregion Header

`;
      }
      const edits = new Map<string, TextEdit[]>();
      edits.set(file, [{ start: 0, end: 0, newText: headerContent }]);
      violations.push(
        ctx.createViolation({
          summary: `Missing header section in ${file}`,
          kind: "header:missing-section",
          solution: "Add a #region Header with filename, contributors, and appropriate license",
          reason: "Every source file must include a header section",
          scope: file,
          autofix: ctx.createFix("Add header section", edits),
        }),
      );
      continue;
    }
    const headerContent = content.slice(headerSection.startIndex, headerSection.endIndex);
    const headerLines = headerContent.split("\n");
    const filename = file.split("/").pop() ?? file;
    const hasFilename = headerLines.some((l) => l.includes(filename));
    if (!hasFilename) {
      violations.push(
        ctx.createViolation({
          summary: `Missing filename in header of ${file}`,
          kind: "header:missing-filename",
          solution: `Add the filename "${filename}" to the header section`,
          reason: "Header must include the source file name",
          scope: `${file}#Header`,
          line: headerSection.startLine,
        }),
      );
    }
    const contributorPattern = /\d{4}\s+[\w\s]+<[\w.@-]+>/;
    const hasContributors = headerLines.some((l) => contributorPattern.test(l));
    if (!hasContributors) {
      violations.push(
        ctx.createViolation({
          summary: `Missing contributors in header of ${file}`,
          kind: "header:missing-contributors",
          solution: "Add contributor line in format: YEAR Name <email>",
          reason: "Header must include at least one contributor",
          scope: `${file}#Header`,
          line: headerSection.startLine,
        }),
      );
    }
    const hasLicense = agplMarkers.some((marker) => headerContent.includes(marker));
    if (!hasLicense) {
      violations.push(
        ctx.createViolation({
          summary: `Missing license in header of ${file}`,
          kind: "header:missing-license",
          solution: "Add AGPL-3.0 license text to the header section",
          reason: "Header must include AGPL-3.0 license",
          scope: `${file}#Header`,
          line: headerSection.startLine,
        }),
      );
    } else {
      const hasWrongLicense = headerContent.includes("MIT") || headerContent.includes("Apache") || headerContent.includes("BSD") || (headerContent.includes("GPL") && !headerContent.includes("AGPL"));
      if (hasWrongLicense) {
        violations.push(
          ctx.createViolation({
            summary: `Wrong license in header of ${file}`,
            kind: "header:wrong-license",
            solution: "Replace with AGPL-3.0 license text",
            reason: "Project uses AGPL-3.0, not other licenses",
            scope: `${file}#Header`,
            line: headerSection.startLine,
          }),
        );
      }
    }
  }
  return violations;
});
// #endregion Header

// #region Section
registerPolicy({ id: "section", name: "Section", description: "Validates section blocks for proper naming and content", scopes: ["**/*.{ts,tsx,py,cs}"], priority: "low" }, async (ctx) => {
  const violations: Violation[] = [];
  const files = ctx.files();
  const sectionPatterns: Record<string, { start: RegExp; end: RegExp }> = {
    typescript: { start: /^\s*\/\/\s*#region(?:\s+(\S.*?))?\s*$/i, end: /^\s*\/\/\s*#endregion(?:\s+(\S.*?))?\s*$/i },
    python: { start: /^\s*#\s*region(?:\s+(\S.*?))?\s*$/i, end: /^\s*#\s*endregion(?:\s+(\S.*?))?\s*$/i },
    csharp: { start: /^\s*#region(?:\s+(\S.*?))?\s*$/i, end: /^\s*#endregion(?:\s+(\S.*?))?\s*$/i },
  };
  function checkSection(file: string, section: SectionInfo, content: string): void {
    const sectionContent = content.slice(section.startIndex, section.endIndex);
    const lines = sectionContent.split("\n").slice(1, -1);
    const nonEmptyLines = lines.filter((l) => l.trim() && !l.trim().startsWith("//") && !l.trim().startsWith("#"));
    if (nonEmptyLines.length === 0 && section.children.length === 0) {
      violations.push(
        ctx.createViolation({
          summary: `Empty section "${section.name}" in ${file}`,
          kind: "section:empty",
          solution: "Remove the empty section or add content to it",
          reason: "Empty sections add noise without providing value",
          scope: `${file}#${section.name}`,
          line: section.startLine,
        }),
      );
    }
    for (const child of section.children) {
      checkSection(file, child, content);
    }
  }
  for (const file of files) {
    const content = ctx.readText(file);
    if (!content) continue;
    const lang = getLanguageFromPath(file);
    if (!lang) continue;
    const patterns = sectionPatterns[lang];
    const lines = content.split("\n");
    const sectionStack: { name: string; line: number }[] = [];
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i].replace(/\r$/, "");
      const lineNum = i + 1;
      const startMatch = line.match(patterns.start);
      if (startMatch) {
        const name = startMatch[1]?.trim() ?? "";
        if (!name) {
          violations.push(
            ctx.createViolation({
              summary: `Missing section name at ${file}:${lineNum}`,
              kind: "section:missing-start-name",
              solution: "Add a name after #region",
              reason: "Section blocks should have descriptive names",
              scope: file,
              line: lineNum,
              excerpt: line.trim(),
            }),
          );
        }
        sectionStack.push({ name, line: lineNum });
        continue;
      }
      const endMatch = line.match(patterns.end);
      if (endMatch) {
        const endName = endMatch[1]?.trim() ?? "";
        const openSection = sectionStack.pop();
        if (openSection && openSection.name) {
          if (!endName) {
            violations.push(
              ctx.createViolation({
                summary: `Missing end section name at ${file}:${lineNum}`,
                kind: "section:missing-end-name",
                solution: `Add the section name "${openSection.name}" after #endregion`,
                reason: "End section should match start section name for clarity",
                scope: file,
                line: lineNum,
                excerpt: line.trim(),
              }),
            );
          } else if (endName !== openSection.name) {
            violations.push(
              ctx.createViolation({
                summary: `Section name mismatch at ${file}:${lineNum}`,
                kind: "section:name-mismatch",
                solution: `Change end name from "${endName}" to "${openSection.name}"`,
                reason: "Start and end section names must match",
                scope: file,
                line: lineNum,
                excerpt: `Start: "${openSection.name}" at line ${openSection.line}, End: "${endName}"`,
              }),
            );
          }
        }
        continue;
      }
    }
    const sections = ctx.sections(file);
    for (const section of sections) {
      checkSection(file, section, content);
    }
  }
  return violations;
});
// #endregion Section

// #region Comment
registerPolicy({ id: "comment", name: "Comment", description: "Detects forbidden comments (inline, block, JSDoc) - documentation belongs in README.md and AGENTS.md", scopes: ["**/*.{ts,tsx}"], priority: "low" }, async (ctx) => {
  const violations: Violation[] = [];
  const files = ctx.files();
  for (const file of files) {
    const content = ctx.readText(file);
    if (!content) continue;
    const lines = content.split("\n");
    let charIndex = 0;
    const lineOffsets: number[] = [];
    for (const line of lines) {
      lineOffsets.push(charIndex);
      charIndex += line.length + 1;
    }
    let inBlockComment = false;
    let blockCommentStartLine = 0;
    let blockCommentStartIndex = 0;
    let inJsDoc = false;
    let jsDocStartLine = 0;
    let jsDocStartIndex = 0;
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      const trimmed = line.trim();
      const lineNum = i + 1;
      const lineStart = lineOffsets[i];
      const lineEnd = lineStart + line.length + 1;
      if (trimmed.startsWith("/**") && !trimmed.endsWith("*/")) {
        inJsDoc = true;
        jsDocStartLine = lineNum;
        jsDocStartIndex = lineStart;
        continue;
      }
      if (inJsDoc) {
        if (trimmed.endsWith("*/")) {
          const edits = new Map<string, TextEdit[]>();
          edits.set(file, [{ start: jsDocStartIndex, end: lineEnd, newText: "" }]);
          violations.push(
            ctx.createViolation({
              summary: `JSDoc comment in ${file}:${jsDocStartLine}`,
              kind: "comment:jsdoc",
              solution: "Remove JSDoc and document in README.md or AGENTS.md",
              reason: "Documentation is centralized, not inline",
              scope: file,
              line: jsDocStartLine,
              autofix: ctx.createFix("Remove JSDoc comment", edits),
            }),
          );
          inJsDoc = false;
        }
        continue;
      }
      if (trimmed.startsWith("/*") && !trimmed.startsWith("/**") && !trimmed.endsWith("*/")) {
        inBlockComment = true;
        blockCommentStartLine = lineNum;
        blockCommentStartIndex = lineStart;
        continue;
      }
      if (inBlockComment) {
        if (trimmed.endsWith("*/")) {
          const edits = new Map<string, TextEdit[]>();
          edits.set(file, [{ start: blockCommentStartIndex, end: lineEnd, newText: "" }]);
          violations.push(
            ctx.createViolation({
              summary: `Block comment in ${file}:${blockCommentStartLine}`,
              kind: "comment:block",
              solution: "Remove block comment and document in README.md or AGENTS.md",
              reason: "Documentation is centralized, not inline",
              scope: file,
              line: blockCommentStartLine,
              autofix: ctx.createFix("Remove block comment", edits),
            }),
          );
          inBlockComment = false;
        }
        continue;
      }
      if (trimmed.startsWith("/*") && trimmed.endsWith("*/")) {
        const edits = new Map<string, TextEdit[]>();
        edits.set(file, [{ start: lineStart, end: lineEnd, newText: "" }]);
        if (trimmed.startsWith("/**")) {
          violations.push(
            ctx.createViolation({
              summary: `JSDoc comment in ${file}:${lineNum}`,
              kind: "comment:jsdoc",
              solution: "Remove JSDoc and document in README.md or AGENTS.md",
              reason: "Documentation is centralized, not inline",
              scope: file,
              line: lineNum,
              excerpt: trimmed.slice(0, 80),
              autofix: ctx.createFix("Remove JSDoc comment", edits),
            }),
          );
        } else {
          violations.push(
            ctx.createViolation({
              summary: `Block comment in ${file}:${lineNum}`,
              kind: "comment:block",
              solution: "Remove block comment and document in README.md or AGENTS.md",
              reason: "Documentation is centralized, not inline",
              scope: file,
              line: lineNum,
              excerpt: trimmed.slice(0, 80),
              autofix: ctx.createFix("Remove block comment", edits),
            }),
          );
        }
        continue;
      }
      if (trimmed.startsWith("// #region") || trimmed.startsWith("// #endregion") || trimmed.startsWith("// #region") || trimmed.startsWith("// #endregion")) continue;
      if (trimmed.includes("[DEBUG]")) continue;
      const isHeaderLine = trimmed.includes("Copyright") || trimmed.includes("License") || trimmed.includes("SPDX") || trimmed.includes("GNU") || trimmed.includes("AGPL") || /^\d{4}\s+[\w\s]+<[\w.@-]+>/.test(trimmed.replace(/^\/\/\s*/, ""));
      if (isHeaderLine) continue;
      if (trimmed.startsWith("//")) continue;
      const inlineMatch = line.match(/^(.+?)\s*(\/\/(?!\s*#region|\s*#endregion).*)$/);
      if (inlineMatch && !line.includes("://") && !line.includes("/*")) {
        const codePart = inlineMatch[1].trim();
        if (codePart.length > 0) {
          const commentStart = lineStart + inlineMatch[1].length;
          const edits = new Map<string, TextEdit[]>();
          edits.set(file, [{ start: commentStart, end: lineStart + line.length, newText: "" }]);
          violations.push(
            ctx.createViolation({
              summary: `Inline comment in ${file}:${lineNum}`,
              kind: "comment:inline",
              solution: "Remove inline comment and document in README.md or AGENTS.md",
              reason: "Code is never documented inline",
              scope: file,
              line: lineNum,
              excerpt: trimmed.slice(0, 80),
              autofix: ctx.createFix("Remove inline comment", edits),
            }),
          );
        }
      }
    }
  }
  return violations;
});
// #endregion Comment

// #endregion Policies

// #region Ticket Engine

function getTicketPath(year: number, month: number, day: number, slug: string): string {
  return join(TICKETS_DIR, String(year), padNumber(month), padNumber(day), `${slug}.md`);
}

function createTicket(slug: string, prompt: string, model?: string): Ticket {
  const now = new Date();
  const { year, month, day } = formatDate(now);
  const normalizedSlug = slugify(slug);
  const filePath = getTicketPath(year, month, day, normalizedSlug);
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
  const frontmatter: TicketFrontmatter = { slug: normalizedSlug, prompt, status: "open", author: gitAuthor || undefined, date: { created: isoTimestamp() }, commit: gitCommit || undefined, model };
  const content = `
# Previously

# Plan

# Changes
`.trim();
  const ticket: Ticket = { year, month, day, slug: normalizedSlug, frontmatter, content, filePath };
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

function saveTicket(ticket: Ticket): void {
  const fileContent = matter.stringify(ticket.content, ticket.frontmatter as any);
  writeTextFile(ticket.filePath, fileContent);
}

function startIteration(ticket: Ticket, prompt: string, model?: string): void {
  let gitAuthor = "";
  try {
    const name = execSync("git config --get user.name", { encoding: "utf-8" }).trim();
    const email = execSync("git config --get user.email", { encoding: "utf-8" }).trim();
    gitAuthor = email ? `${name} <${email}>` : name;
  } catch {}
  const iteration: TicketIteration = { prompt, model, date: { started: isoTimestamp() }, author: gitAuthor || undefined };
  if (!ticket.frontmatter.iterations) {
    ticket.frontmatter.iterations = [];
  }
  ticket.frontmatter.iterations.push(iteration);
  saveTicket(ticket);
}

function endIteration(ticket: Ticket): { success: boolean; error?: string } {
  if (!ticket.frontmatter.iterations || ticket.frontmatter.iterations.length === 0) {
    return { success: false, error: "Error: No active iteration to end" };
  }
  const lastIteration = ticket.frontmatter.iterations[ticket.frontmatter.iterations.length - 1];
  if (lastIteration.date.ended) {
    return { success: false, error: "Error: Last iteration already ended" };
  }
  lastIteration.date.ended = isoTimestamp();
  try {
    lastIteration.commit = execSync("git rev-parse HEAD", { encoding: "utf-8" }).trim();
  } catch {}
  saveTicket(ticket);
  return { success: true };
}

function finishTicket(ticket: Ticket): { success: boolean; error?: string } {
  if (ticket.frontmatter.iterations && ticket.frontmatter.iterations.length > 0) {
    const lastIteration = ticket.frontmatter.iterations[ticket.frontmatter.iterations.length - 1];
    if (!lastIteration.date.ended) {
      return { success: false, error: "Error: Cannot finish ticket with unfinished iteration" };
    }
  }
  ticket.frontmatter.status = "closed";
  ticket.frontmatter.date.finished = isoTimestamp();
  saveTicket(ticket);
  return { success: true };
}

function updateTicketViolations(ticket: Ticket, violations: Violation[]): void {
  let violationsSection = "## Violations\n\n";
  if (violations.length === 0) {
    violationsSection += "(No violations)\n";
  } else {
    for (const violation of violations) {
      const checkbox = violation.autofixable ? "[ ]" : "[x]";
      violationsSection += `- ${checkbox} (${violation.priority}) \`${violation.kind}\` in \`${violation.scope}\`\n`;
      violationsSection += `  - Summary: ${violation.summary}\n`;
      violationsSection += `  - Solution: ${violation.solution}\n`;
    }
  }
  let newContent = ticket.content;
  const violationsRegex = /## Violations[\s\S]*?(?=\n## |$)/;
  if (violationsRegex.test(newContent)) {
    newContent = newContent.replace(violationsRegex, violationsSection);
  } else {
    newContent += "\n\n" + violationsSection;
  }
  ticket.content = newContent;
  saveTicket(ticket);
}

function canCloseTicket(ticket: Ticket): { canClose: boolean; reasons: string[] } {
  const reasons: string[] = [];
  const content = ticket.content;
  const violationsMatch = content.match(/## Violations[\s\S]*?(?=\n## |$)/);
  if (violationsMatch) {
    const violationsContent = violationsMatch[0];
    if (!violationsContent.includes("(No violations)") && violationsContent.includes("- [")) {
      reasons.push("Violations section is not empty");
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

function closeTicket(ticket: Ticket): { success: boolean; error?: string } {
  const { canClose, reasons } = canCloseTicket(ticket);
  if (!canClose) {
    return { success: false, error: `Cannot close ticket: ${reasons.join(", ")}` };
  }
  ticket.frontmatter.status = "closed";
  ticket.frontmatter.date.finished = isoTimestamp();
  saveTicket(ticket);
  return { success: true };
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
  let subsubcommand: string | undefined;
  const subcommands = ["list", "tree", "create", "read", "iterate", "finish", "run", "move", "delete"];
  const subsubcommands = ["start", "end"];
  for (let i = 1; i < args.length; i++) {
    const arg = args[i];
    if (arg.startsWith("--")) {
      const eqIndex = arg.indexOf("=");
      if (eqIndex !== -1) {
        const key = arg.slice(2, eqIndex);
        const value = arg.slice(eqIndex + 1);
        options[key] = value;
      } else {
        options[arg.slice(2)] = true;
      }
    } else if (arg.startsWith("-")) {
      options[arg.slice(1)] = true;
    } else if (!subcommand && subcommands.includes(arg)) {
      subcommand = arg;
    } else if (subcommand && !subsubcommand && subsubcommands.includes(arg)) {
      subsubcommand = arg;
    } else {
      positionalArgs.push(arg);
    }
  }
  return { name, subcommand, subsubcommand, args: positionalArgs, options };
}

// #endregion Command Parser

// #region Command Handlers

async function handleAnalyze(cmd: ParsedCommand): Promise<CommandOutput> {
  const output = createOutput();
  let scopeRaws: string[];
  if (cmd.options.scope && typeof cmd.options.scope === "string") {
    scopeRaws = [cmd.options.scope];
  } else if (cmd.args.length > 0) {
    scopeRaws = cmd.args;
  } else {
    scopeRaws = ["@semio"];
  }
  const violations: Violation[] = [];
  for (const scopeRaw of scopeRaws) {
    const scope = parseScope(scopeRaw);
    const scopeViolations = await runPolicies(scope);
    violations.push(...scopeViolations);
  }
  const report: AnalyzeReport = {
    timestamp: isoTimestamp(),
    status: violations.length > 0 ? "error" : "success",
    scope: scopeRaws.join(" "),
    summary: {
      total: violations.length,
      byPriority: { high: violations.filter((i) => i.priority === "high").length, medium: violations.filter((i) => i.priority === "medium").length, low: violations.filter((i) => i.priority === "low").length },
      byKind: violations.reduce(
        (acc, i) => {
          acc[i.kind] = (acc[i.kind] || 0) + 1;
          return acc;
        },
        {} as Record<string, number>,
      ),
    },
    violations,
  };
  ensureDir(REPORTS_DIR);
  writeJsonFile(join(REPORTS_DIR, "policies.json"), report);
  if (cmd.options.json) {
    plain(output, JSON.stringify(report, null, 2));
  } else {
    success(output, `\n📊 Analysis complete: ${violations.length} violations found`);
    info(output, `   Report: ${join(REPORTS_DIR, "policies.json")}`);
  }
  if (report.status === "error") output.exitCode = 1;
  return output;
}

async function handleFix(cmd: ParsedCommand): Promise<CommandOutput> {
  const output = createOutput();
  const scopeRaw = (typeof cmd.options.scope === "string" ? cmd.options.scope : null) || cmd.args[0] || "@semio";
  const scope = parseScope(scopeRaw);
  const dryRun = !!cmd.options["dry-run"];
  const violations = await runPolicies(scope);
  const fixable = violations.filter((i) => i.autofixable && i.autofix);
  if (dryRun) {
    info(output, `\n🔧 Dry run: ${fixable.length} fixable violations found`);
    for (const violation of fixable) {
      plain(output, `   - ${violation.kind}: ${violation.summary}`);
    }
  } else {
    let fixed = 0;
    for (const violation of fixable) {
      if (violation.autofix) {
        for (const [filePath, edits] of violation.autofix.edits) {
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
    success(output, `\n✅ Fixed ${fixed} violations`);
  }
  return output;
}

async function handlePolicy(cmd: ParsedCommand): Promise<CommandOutput> {
  const output = createOutput();
  if (cmd.subcommand === "list") {
    info(output, "\n📜 Registered policies:\n");
    for (const policy of RULES) {
      plain(output, `   ${policy.meta.id}`);
      plain(output, `      ${policy.meta.name}: ${policy.meta.description}`);
      plain(output, `      Priority: ${policy.meta.priority}`);
      plain(output, "");
    }
  } else if (cmd.subcommand === "run") {
    const policyId = cmd.args[0];
    if (!policyId) {
      error(output, "Error: Policy ID required");
      return output;
    }
    const scopeRaw = (cmd.options.scope as string) || "@semio";
    const scope = parseScope(scopeRaw);
    const violations = await runPolicies(scope, [policyId]);
    info(output, `\n📊 Policy "${policyId}" found ${violations.length} violations`);
    for (const violation of violations.slice(0, 10)) {
      plain(output, `   - ${violation.summary}`);
    }
    if (violations.length > 10) {
      plain(output, `   ... and ${violations.length - 10} more`);
    }
  } else {
    info(output, "Usage: repo policy <list|run> [id]");
  }
  return output;
}

async function handleTicket(cmd: ParsedCommand): Promise<CommandOutput> {
  const output = createOutput();
  if (cmd.subcommand === "create") {
    const slug = cmd.args[0];
    if (!slug) {
      error(output, "Error: Ticket slug required");
      return output;
    }
    const prompt = (cmd.options.prompt as string) || slug;
    const model = cmd.options.model as string | undefined;
    const ticket = createTicket(slug, prompt, model);
    success(output, `\n🎫 Created ticket: ${ticket.slug}`);
    info(output, `   Path: ${ticket.filePath}`);
  } else if (cmd.subcommand === "iterate") {
    if (cmd.subsubcommand === "start") {
      const [year, month, day, slug] = cmd.args;
      if (!year || !month || !day || !slug) {
        error(output, "Error: Format: ticket iterate start <year> <month> <day> <slug>");
        return output;
      }
      const ticket = readTicket(parseInt(year), parseInt(month), parseInt(day), slug);
      if (!ticket) {
        error(output, "Error: Ticket not found");
        return output;
      }
      const prompt = (cmd.options.prompt as string) || "";
      const model = cmd.options.model as string | undefined;
      startIteration(ticket, prompt, model);
      success(output, `\n🔄 Started iteration on ticket: ${ticket.slug}`);
    } else if (cmd.subsubcommand === "end") {
      const [year, month, day, slug] = cmd.args;
      if (!year || !month || !day || !slug) {
        error(output, "Error: Format: ticket iterate end <year> <month> <day> <slug>");
        return output;
      }
      const ticket = readTicket(parseInt(year), parseInt(month), parseInt(day), slug);
      if (!ticket) {
        error(output, "Error: Ticket not found");
        return output;
      }
      const endResult = endIteration(ticket);
      if (!endResult.success) {
        error(output, endResult.error!);
        return output;
      }
      success(output, `\n✅ Ended iteration on ticket: ${ticket.slug}`);
    } else {
      info(output, "Usage: repo ticket iterate <start|end> <year> <month> <day> <slug>");
    }
  } else if (cmd.subcommand === "finish") {
    const [year, month, day, slug] = cmd.args;
    if (!year || !month || !day || !slug) {
      error(output, "Error: Format: ticket finish <year> <month> <day> <slug>");
      return output;
    }
    const ticket = readTicket(parseInt(year), parseInt(month), parseInt(day), slug);
    if (!ticket) {
      error(output, "Error: Ticket not found");
      return output;
    }
    const finishResult = finishTicket(ticket);
    if (finishResult.success) {
      success(output, `\n✅ Ticket finished: ${ticket.slug}`);
    } else {
      error(output, finishResult.error!);
    }
  } else if (cmd.subcommand === "list") {
    const year = cmd.options.year ? parseInt(cmd.options.year as string) : undefined;
    const month = cmd.options.month ? parseInt(cmd.options.month as string) : undefined;
    const day = cmd.options.day ? parseInt(cmd.options.day as string) : undefined;
    const tickets = listTickets(year, month, day);
    info(output, `\n🎫 Found ${tickets.length} tickets:\n`);
    for (const ticket of tickets) {
      const status = ticket.frontmatter.status === "open" ? "🟢" : "✅";
      plain(output, `   ${status} ${ticket.year}/${padNumber(ticket.month)}/${padNumber(ticket.day)}/${ticket.slug}`);
      if (ticket.frontmatter.summary) {
        plain(output, `      ${ticket.frontmatter.summary}`);
      }
    }
  } else if (cmd.subcommand === "read") {
    const [year, month, day, slug] = cmd.args;
    if (!year || !month || !day || !slug) {
      error(output, "Error: Format: ticket read <year> <month> <day> <slug>");
      return output;
    }
    const ticket = readTicket(parseInt(year), parseInt(month), parseInt(day), slug);
    if (!ticket) {
      error(output, "Error: Ticket not found");
      return output;
    }
    info(output, `\n🎫 Ticket: ${ticket.slug}`);
    plain(output, `   Status: ${ticket.frontmatter.status}`);
    plain(output, `   Created: ${ticket.frontmatter.date.created}`);
    plain(output, `   Prompt: ${ticket.frontmatter.prompt}`);
    if (ticket.frontmatter.model) {
      plain(output, `   Model: ${ticket.frontmatter.model}`);
    }
    plain(output, `\n${ticket.content}`);
  } else {
    info(output, "Usage: repo ticket <create|iterate|finish|list|read> [args]");
  }
  return output;
}

async function handleProject(cmd: ParsedCommand): Promise<CommandOutput> {
  const output = createOutput();
  const projects = getNxProjects();
  if (cmd.subcommand === "list") {
    info(output, `\n📦 Found ${projects.length} projects:\n`);
    for (const project of projects) {
      plain(output, `   ${project.name}`);
      plain(output, `      Root: ${project.root}`);
      if (project.tags?.length) {
        plain(output, `      Tags: ${project.tags.join(", ")}`);
      }
    }
  } else if (cmd.subcommand === "tree") {
    info(output, "\n📦 Project tree:\n");
    for (const project of projects) {
      plain(output, `   └── ${project.name} (${project.root})`);
    }
  } else {
    info(output, "Usage: repo project <list|tree>");
  }
  return output;
}

async function handleFolder(cmd: ParsedCommand): Promise<CommandOutput> {
  const output = createOutput();
  if (cmd.subcommand === "create") {
    const folderPath = cmd.args[0];
    if (!folderPath) {
      error(output, "Error: Folder path required");
      return output;
    }
    const absPath = join(ROOT_DIR, folderPath);
    if (existsSync(absPath)) {
      error(output, `Error: Folder already exists: ${folderPath}`);
      return output;
    }
    ensureDir(absPath);
    success(output, `\n📁 Created folder: ${folderPath}`);
  } else if (cmd.subcommand === "move") {
    const sourcePath = cmd.args[0];
    const targetPath = cmd.args[1];
    if (!sourcePath || !targetPath) {
      error(output, "Error: Source and target paths required");
      return output;
    }
    const absSource = join(ROOT_DIR, sourcePath);
    const absTarget = join(ROOT_DIR, targetPath);
    if (!existsSync(absSource)) {
      error(output, `Error: Source folder not found: ${sourcePath}`);
      return output;
    }
    if (existsSync(absTarget)) {
      error(output, `Error: Target folder already exists: ${targetPath}`);
      return output;
    }
    const { renameSync } = await import("fs");
    ensureDir(dirname(absTarget));
    renameSync(absSource, absTarget);
    success(output, `\n📁 Moved folder: ${sourcePath} → ${targetPath}`);
  } else if (cmd.subcommand === "delete") {
    const folderPath = cmd.args[0];
    if (!folderPath) {
      error(output, "Error: Folder path required");
      return output;
    }
    const absPath = join(ROOT_DIR, folderPath);
    if (!existsSync(absPath)) {
      error(output, `Error: Folder not found: ${folderPath}`);
      return output;
    }
    const { rmSync } = await import("fs");
    rmSync(absPath, { recursive: true });
    success(output, `\n🗑️ Deleted folder: ${folderPath}`);
  } else if (cmd.subcommand === "list") {
    const scopeRaw = cmd.args[0] || (cmd.options.scope as string) || ".";
    const absPath = join(ROOT_DIR, scopeRaw.replace(/\/$/, ""));
    if (!existsSync(absPath)) {
      error(output, `Error: Folder not found: ${scopeRaw}`);
      return output;
    }
    const allItems = readdirSync(absPath, { withFileTypes: true }).filter((f) => !f.name.startsWith(".") && f.isDirectory());
    const relativePaths = allItems.map((item) => getRelativePath(join(absPath, item.name)));
    const ignoredSet = getGitIgnoredSet(relativePaths);
    const folders = allItems.filter((item) => {
      const relPath = normalizePathSeparators(getRelativePath(join(absPath, item.name)));
      return !ignoredSet.has(relPath) && !ignoredSet.has(relPath + "/");
    });
    info(output, `\n📁 Found ${folders.length} folders in ${scopeRaw}:\n`);
    for (const folder of folders) {
      plain(output, `   ${folder.name}/`);
    }
  } else if (cmd.subcommand === "tree") {
    const scopeRaw = cmd.args[0] || (cmd.options.scope as string) || ".";
    const absPath = join(ROOT_DIR, scopeRaw.replace(/\/$/, ""));
    if (!existsSync(absPath)) {
      error(output, `Error: Folder not found: ${scopeRaw}`);
      return output;
    }
    info(output, `\n📁 Folder tree: ${scopeRaw}\n`);
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
        plain(output, `${prefix}${isLast ? "└── " : "├── "}${item}${isDir ? "/" : ""}`);
        if (isDir) {
          printTree(fullPath, prefix + (isLast ? "    " : "│   "));
        }
      });
    }
    printTree(absPath);
  } else {
    info(output, "Usage: repo folder <create|move|delete|list|tree> [args]");
  }
  return output;
}

async function handleFile(cmd: ParsedCommand): Promise<CommandOutput> {
  const output = createOutput();
  if (cmd.subcommand === "create") {
    const filePath = cmd.args[0];
    if (!filePath) {
      error(output, "Error: File path required");
      return output;
    }
    const absPath = join(ROOT_DIR, filePath);
    if (existsSync(absPath)) {
      error(output, `Error: File already exists: ${filePath}`);
      return output;
    }
    const lang = getLanguageFromPath(filePath);
    let content = "";
    if (lang) {
      const year = new Date().getFullYear();
      let gitAuthor = "";
      try {
        const name = execSync("git config --get user.name", { encoding: "utf-8" }).trim();
        const email = execSync("git config --get user.email", { encoding: "utf-8" }).trim();
        gitAuthor = email ? `${name} <${email}>` : name;
      } catch {}
      if (lang === "typescript") {
        content = `// #region Header

// ${filePath}

// ${year} ${gitAuthor}

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
`;
      } else if (lang === "python") {
        content = `# region Header

# ${filePath}

# ${year} ${gitAuthor}

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.

# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# endregion Header
`;
      } else if (lang === "csharp") {
        content = `// ${filePath}

// ${year} ${gitAuthor}

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

#region Header
#endregion Header
`;
      }
    }
    writeTextFile(absPath, content);
    success(output, `\n📄 Created file: ${filePath}`);
  } else if (cmd.subcommand === "move") {
    const sourcePath = cmd.args[0];
    const targetPath = cmd.args[1];
    if (!sourcePath || !targetPath) {
      error(output, "Error: Source and target paths required");
      return output;
    }
    const absSource = join(ROOT_DIR, sourcePath);
    const absTarget = join(ROOT_DIR, targetPath);
    if (!existsSync(absSource)) {
      error(output, `Error: Source file not found: ${sourcePath}`);
      return output;
    }
    if (existsSync(absTarget)) {
      error(output, `Error: Target file already exists: ${targetPath}`);
      return output;
    }
    const { renameSync } = await import("fs");
    ensureDir(dirname(absTarget));
    renameSync(absSource, absTarget);
    success(output, `\n📄 Moved file: ${sourcePath} → ${targetPath}`);
  } else if (cmd.subcommand === "delete") {
    const filePath = cmd.args[0];
    if (!filePath) {
      error(output, "Error: File path required");
      return output;
    }
    const absPath = join(ROOT_DIR, filePath);
    if (!existsSync(absPath)) {
      error(output, `Error: File not found: ${filePath}`);
      return output;
    }
    const { unlinkSync } = await import("fs");
    unlinkSync(absPath);
    success(output, `\n🗑️ Deleted file: ${filePath}`);
  } else if (cmd.subcommand === "list" || !cmd.subcommand) {
    const scopeRaw = cmd.args[0] || (cmd.options.scope as string) || "@semio";
    const scope = parseScope(scopeRaw);
    const projects = getNxProjects();
    const files = scopeToFiles(scope, projects);
    info(output, `\n📄 Found ${files.length} files in scope "${scopeRaw}":\n`);
    for (const file of files.slice(0, 50)) {
      plain(output, `   ${file}`);
    }
    if (files.length > 50) {
      plain(output, `   ... and ${files.length - 50} more`);
    }
  } else if (cmd.subcommand === "tree") {
    const scopeRaw = cmd.args[0] || (cmd.options.scope as string) || ".";
    const absPath = join(ROOT_DIR, scopeRaw.replace(/\/$/, ""));
    if (!existsSync(absPath)) {
      error(output, `Error: Path not found: ${scopeRaw}`);
      return output;
    }
    info(output, `\n📄 File tree: ${scopeRaw}\n`);
    function printTree(dir: string, prefix: string = ""): void {
      const allItems = readdirSync(dir).filter((f) => !f.startsWith("."));
      const relativePaths = allItems.map((item) => getRelativePath(join(dir, item)));
      const ignoredSet = getGitIgnoredSet(relativePaths);
      const items = allItems.filter((item) => {
        const relPath = normalizePathSeparators(getRelativePath(join(dir, item)));
        return !ignoredSet.has(relPath) && !ignoredSet.has(relPath + "/");
      });
      const fileItems = items.filter((item) => statSync(join(dir, item)).isFile());
      const dirItems = items.filter((item) => statSync(join(dir, item)).isDirectory());
      const sortedItems = [...dirItems, ...fileItems];
      sortedItems.forEach((item, index) => {
        const isLast = index === sortedItems.length - 1;
        const fullPath = join(dir, item);
        const isDir = statSync(fullPath).isDirectory();
        plain(output, `${prefix}${isLast ? "└── " : "├── "}${item}${isDir ? "/" : ""}`);
        if (isDir) {
          printTree(fullPath, prefix + (isLast ? "    " : "│   "));
        }
      });
    }
    printTree(absPath);
  } else {
    info(output, "Usage: repo file <create|move|delete|list|tree> [args]");
  }
  return output;
}

async function handleSection(cmd: ParsedCommand): Promise<CommandOutput> {
  const output = createOutput();
  const ext = (filePath: string) => filePath.split(".").pop()?.toLowerCase();
  const isMarkdown = (filePath: string) => ext(filePath) === "md" || ext(filePath) === "mdx";
  if (cmd.subcommand === "create") {
    const filePath = cmd.args[0];
    const sectionPath = cmd.args[1];
    if (!filePath || !sectionPath) {
      error(output, "Error: File path and section path required");
      return output;
    }
    const absPath = join(ROOT_DIR, filePath);
    if (!existsSync(absPath)) {
      error(output, `Error: File not found: ${filePath}`);
      return output;
    }
    const content = readTextFile(absPath);
    const sectionName = sectionPath.split("#").pop() ?? sectionPath;
    let newSection = "";
    if (isMarkdown(filePath)) {
      newSection = `\n## ${sectionName}\n\n`;
    } else {
      const lang = getLanguageFromPath(filePath);
      if (!lang) {
        error(output, "Error: Unsupported file type");
        return output;
      }
      if (lang === "typescript") {
        newSection = `\n// #region ${sectionName}\n\n// #endregion ${sectionName}\n`;
      } else if (lang === "python") {
        newSection = `\n# region ${sectionName}\n\n# endregion ${sectionName}\n`;
      } else if (lang === "csharp") {
        newSection = `\n#region ${sectionName}\n\n#endregion ${sectionName}\n`;
      }
    }
    writeTextFile(absPath, content + newSection);
    success(output, `\n🏷️ Created section "${sectionName}" in ${filePath}`);
  } else if (cmd.subcommand === "move") {
    const filePath = cmd.args[0];
    const oldSectionPath = cmd.args[1];
    const newSectionPath = cmd.args[2];
    if (!filePath || !oldSectionPath || !newSectionPath) {
      error(output, "Error: File path, old section path, and new section path required");
      return output;
    }
    const absPath = join(ROOT_DIR, filePath);
    if (!existsSync(absPath)) {
      error(output, `Error: File not found: ${filePath}`);
      return output;
    }
    let content = readTextFile(absPath);
    const oldName = oldSectionPath.split("#").pop() ?? oldSectionPath;
    const newName = newSectionPath.split("#").pop() ?? newSectionPath;
    if (isMarkdown(filePath)) {
      content = content.replace(new RegExp(`^(#{1,6})\\s+${oldName}\\s*$`, "gim"), `$1 ${newName}`);
    } else {
      const lang = getLanguageFromPath(filePath);
      if (!lang) {
        error(output, "Error: Unsupported file type");
        return output;
      }
      if (lang === "typescript") {
        content = content.replace(new RegExp(`(//\\s*#region\\s+)${oldName}(\\s*)`, "gi"), `$1${newName}$2`);
        content = content.replace(new RegExp(`(//\\s*#endregion\\s*)${oldName}(\\s*)`, "gi"), `$1${newName}$2`);
      } else if (lang === "python") {
        content = content.replace(new RegExp(`(#\\s*region\\s+)${oldName}(\\s*)`, "gi"), `$1${newName}$2`);
        content = content.replace(new RegExp(`(#\\s*endregion\\s*)${oldName}(\\s*)`, "gi"), `$1${newName}$2`);
      } else if (lang === "csharp") {
        content = content.replace(new RegExp(`(#region\\s+)${oldName}(\\s*)`, "gi"), `$1${newName}$2`);
        content = content.replace(new RegExp(`(#endregion\\s*)${oldName}(\\s*)`, "gi"), `$1${newName}$2`);
      }
    }
    writeTextFile(absPath, content);
    success(output, `\n🏷️ Renamed section "${oldName}" to "${newName}" in ${filePath}`);
  } else if (cmd.subcommand === "delete") {
    const filePath = cmd.args[0];
    const sectionPath = cmd.args[1];
    if (!filePath || !sectionPath) {
      error(output, "Error: File path and section path required");
      return output;
    }
    const absPath = join(ROOT_DIR, filePath);
    if (!existsSync(absPath)) {
      error(output, `Error: File not found: ${filePath}`);
      return output;
    }
    const content = readTextFile(absPath);
    const sections = parseSections(content, filePath);
    const sectionName = sectionPath.split("#").pop() ?? sectionPath;
    function findSection(sectionList: SectionInfo[], name: string): SectionInfo | null {
      for (const s of sectionList) {
        if (s.name === name) return s;
        const found = findSection(s.children, name);
        if (found) return found;
      }
      return null;
    }
    const section = findSection(sections, sectionName);
    if (!section) {
      error(output, `Error: Section not found: ${sectionName}`);
      return output;
    }
    const lines = content.split("\n");
    const newLines = lines.filter((_, i) => i + 1 < section.startLine || i + 1 > section.endLine);
    writeTextFile(absPath, newLines.join("\n"));
    success(output, `\n🗑️ Deleted section "${sectionName}" from ${filePath}`);
  } else if (cmd.subcommand === "list") {
    const scopeRaw = cmd.args[0] || (cmd.options.scope as string);
    if (!scopeRaw) {
      error(output, "Error: Scope or file path required");
      return output;
    }
    const scope = parseScope(scopeRaw);
    if (scope.kind !== "file" && scope.kind !== "section") {
      error(output, "Error: Scope must be a file or section");
      return output;
    }
    const filePath = scope.filePath!;
    const absPath = join(ROOT_DIR, filePath);
    if (!existsSync(absPath)) {
      error(output, `Error: File not found: ${filePath}`);
      return output;
    }
    const content = readTextFile(absPath);
    const sections = parseSections(content, filePath);
    info(output, `\n🏷️ Sections in ${filePath}:\n`);
    function printSection(section: SectionInfo, indent: string = ""): void {
      plain(output, `${indent}${section.name} (lines ${section.startLine}-${section.endLine})`);
      for (const child of section.children) {
        printSection(child, indent + "  ");
      }
    }
    for (const section of sections) {
      printSection(section);
    }
    if (sections.length === 0) {
      plain(output, "   (no sections found)");
    }
  } else if (cmd.subcommand === "tree" || !cmd.subcommand) {
    const scopeRaw = cmd.args[0] || (cmd.options.scope as string);
    if (!scopeRaw) {
      error(output, "Error: File path required");
      return output;
    }
    const filePath = scopeRaw.split("#")[0];
    const absPath = join(ROOT_DIR, filePath);
    if (!existsSync(absPath)) {
      error(output, `Error: File not found: ${filePath}`);
      return output;
    }
    const content = readTextFile(absPath);
    const sections = parseSections(content, filePath);
    info(output, `\n🏷️ Sections in ${filePath}:\n`);
    function printSection(section: SectionInfo, prefix: string = ""): void {
      plain(output, `${prefix}└── ${section.name} (lines ${section.startLine}-${section.endLine})`);
      for (const child of section.children) {
        printSection(child, prefix + "    ");
      }
    }
    for (const section of sections) {
      printSection(section);
    }
    if (sections.length === 0) {
      plain(output, "   (no sections found)");
    }
  } else {
    info(output, "Usage: repo section <create|move|delete|list|tree> [args]");
  }
  return output;
}

async function handleDefinition(cmd: ParsedCommand): Promise<CommandOutput> {
  const output = createOutput();
  const filePath = cmd.args[0];
  if (!filePath) {
    error(output, "Error: File path required");
    return output;
  }
  const absPath = join(ROOT_DIR, filePath);
  if (!existsSync(absPath)) {
    error(output, `Error: File not found: ${filePath}`);
    return output;
  }
  const content = readTextFile(absPath);
  const definitions = parseDefinitions(content, filePath);
  if (cmd.subcommand === "list" || !cmd.subcommand) {
    info(output, `\n📋 Definitions in ${filePath}:\n`);
    for (const def of definitions) {
      plain(output, `   ${def.kind}: ${def.name} (lines ${def.startLine}-${def.endLine})`);
    }
    if (definitions.length === 0) {
      plain(output, "   (no definitions found)");
    }
  }
  return output;
}

// #region Update Metabolism Tool

const METABOLISM_INCLUDE_FOLDERS = ["representations", "icons", "images"];

function collectMetabolismFiles(dir: string, basePath: string = ""): Map<string, Blob> {
  const files = new Map<string, Blob>();
  const entries = readdirSync(dir, { withFileTypes: true });

  for (const entry of entries) {
    const fullPath = join(dir, entry.name);
    const relativePath = basePath ? `${basePath}/${entry.name}` : entry.name;

    if (entry.isDirectory()) {
      if (entry.name === ".semio" || entry.name === ".git") continue;
      if (!basePath && !METABOLISM_INCLUDE_FOLDERS.includes(entry.name)) continue;

      const subFiles = collectMetabolismFiles(fullPath, relativePath);
      Array.from(subFiles.entries()).forEach(([path, blob]) => {
        files.set(path, blob);
      });
    } else {
      if (!basePath) continue;

      const buffer = readFileSync(fullPath);
      const blob = new Blob([buffer]);
      files.set(relativePath, blob);
    }
  }

  return files;
}

async function updateMetabolism(output: CommandOutput): Promise<void> {
  info(output, "\n🔄 Updating metabolism assets...");

  const kitPath = join(ROOT_DIR, "assets", "semio", "kit_metabolism.json");
  info(output, `   Reading kit from ${relative(ROOT_DIR, kitPath)}`);
  const kitJson = readFileSync(kitPath, "utf-8");
  const kit = JSON.parse(kitJson) as Kit;

  const metabolismDir = join(ROOT_DIR, "examples", "metabolism");
  info(output, `   Collecting files from ${relative(ROOT_DIR, metabolismDir)}`);
  const files = collectMetabolismFiles(metabolismDir);
  const fileCount = files.size;
  info(output, `   Found ${fileCount} files`);

  info(output, "   Exporting kit to zip...");
  const zipBlob = await exportKit(kit, files);
  const buffer = Buffer.from(await zipBlob.arrayBuffer());

  const outputPath = join(ROOT_DIR, "assets", "semio", "metabolism.zip");
  writeFileSync(outputPath, buffer);
  const size = (buffer.length / 1024).toFixed(2);
  success(output, `   ✅ Written ${relative(ROOT_DIR, outputPath)} (${size} KB)`);

  info(output, "   Copying to public folders...");
  const publicPaths = [join(ROOT_DIR, "js", "js", "public", "metabolism.zip"), join(ROOT_DIR, "js", "play", "public", "metabolism.zip")];

  for (const publicPath of publicPaths) {
    const publicDir = dirname(publicPath);
    if (!existsSync(publicDir)) {
      mkdirSync(publicDir, { recursive: true });
    }
    writeFileSync(publicPath, buffer);
    success(output, `   ✅ Copied to ${relative(ROOT_DIR, publicPath)}`);
  }

  info(output, "   Validating import...");
  const { kit: imported } = await importKit(buffer);
  const typeCount = imported.types?.length ?? 0;
  const designCount = imported.designs?.length ?? 0;
  success(output, `   ✅ Validated: ${typeCount} types, ${designCount} designs`);

  success(output, "\n✅ Metabolism assets updated successfully!");
}

// #endregion Update Metabolism Tool

async function handleTool(cmd: ParsedCommand): Promise<CommandOutput> {
  const output = createOutput();
  const target = cmd.args[0];
  if (!target) {
    error(output, "Error: Tool/target name required");
    return output;
  }

  // Handle built-in tools
  if (target === "update-metabolism") {
    try {
      await updateMetabolism(output);
    } catch (err) {
      error(output, `Error: ${err instanceof Error ? err.message : String(err)}`);
      output.exitCode = 1;
    }
    return output;
  }

  // Fall back to Nx targets
  const projectScope = cmd.options.scope as string | undefined;
  const projects = projectScope ? [projectScope] : undefined;
  const extraArgs = cmd.args.slice(1);
  info(output, `\n🔧 Running Nx target: ${target}`);
  const result = runNxTarget(target, projects, extraArgs);
  if (!result.success) {
    output.exitCode = 1;
  }
  return output;
}

// #endregion Command Handlers

// #region Ink App

interface AppProps {
  command: ParsedCommand;
}

function OutputLineComponent({ line }: { line: OutputLine }) {
  switch (line.type) {
    case "success":
      return <Text color="green">{line.text}</Text>;
    case "error":
      return <Text color="red">{line.text}</Text>;
    case "warn":
      return <Text color="yellow">{line.text}</Text>;
    case "info":
      return <Text color="cyan">{line.text}</Text>;
    default:
      return <Text>{line.text}</Text>;
  }
}

function App({ command }: AppProps) {
  const [output, setOutput] = React.useState<CommandOutput | null>(null);

  React.useEffect(() => {
    (async () => {
      try {
        const result = await runCommand(command);
        setOutput(result);
        setTimeout(() => {
          process.exit(result.exitCode);
        }, 0);
      } catch (err) {
        const result = createOutput();
        error(result, `Error: ${err instanceof Error ? err.message : String(err)}`);
        setOutput(result);
        setTimeout(() => {
          process.exit(1);
        }, 0);
      }
    })();
  }, [command]);

  if (!output) return null;

  return (
    <Box flexDirection="column">
      {output.lines.map((line, i) => (
        <OutputLineComponent key={i} line={line} />
      ))}
    </Box>
  );
}

async function runCommand(command: ParsedCommand): Promise<CommandOutput> {
  switch (command.name) {
    case "help": {
      const result = createOutput();
      plain(result, HELP_TEXT);
      return result;
    }
    case "analyze":
      return await handleAnalyze(command);
    case "fix":
      return await handleFix(command);
    case "policy":
      return await handlePolicy(command);
    case "ticket":
      return await handleTicket(command);
    case "project":
      return await handleProject(command);
    case "folder":
      return await handleFolder(command);
    case "file":
      return await handleFile(command);
    case "section":
      return await handleSection(command);
    case "definition":
      return await handleDefinition(command);
    case "tool":
      return await handleTool(command);
    default: {
      const result = createOutput();
      plain(result, HELP_TEXT);
      return result;
    }
  }
}

// #endregion Ink App

// #region Main

const command = parseCommand(process.argv);
const isTTY = process.stdout.isTTY;

if (!isTTY) {
  (async () => {
    try {
      const result = await runCommand(command);
      for (const line of result.lines) {
        process.stdout.write(line.text + "\n");
      }
      process.exit(result.exitCode);
    } catch (err) {
      process.stderr.write(`Error: ${err instanceof Error ? err.message : String(err)}\n`);
      process.exit(1);
    }
  })();
} else {
  if (command.name === "help") {
    process.stdout.write(HELP_TEXT + "\n");
    process.exit(0);
  }
  render(<App command={command} />);
}

// #endregion
