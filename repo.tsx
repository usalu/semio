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

//#region Violation
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
//#endregion Violation

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

//#region AST
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
//#endregion AST

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
  priority: ViolationPriority;
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
  parseAST: (filePath: string) => ASTFile | null;
  getASTNode: (filePath: string, startIndex: number, endIndex: number) => ASTNode | null;
  queryAST: (filePath: string, query: string) => Array<{ node: ASTNode; captures: Record<string, ASTNode[]> }>;
  createViolation: (partial: Omit<Violation, "id" | "priority" | "autofixable">) => Violation;
  createFix: (description: string, edits: Map<string, TextEdit[]>) => Fix;
}

type RuleFn = (ctx: RuleContext) => Promise<Violation[]>;

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
  summary: { total: number; byPriority: Record<ViolationPriority, number>; byKind: Record<string, number> };
  violations: Violation[];
}
//#endregion Report

//#region Command
type CommandName = "help" | "analyze" | "fix" | "rule" | "ticket" | "project" | "folder" | "file" | "region" | "definition" | "tool";

interface ParsedCommand {
  name: CommandName;
  subcommand?: string;
  subsubcommand?: string;
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
  help                                          Show this help message
  analyze [--scope=<scope>]                     Analyze codebase for violations (multiple scopes supported)
  fix [--scope=<scope>]                         Apply autofixes for violations (multiple scopes supported)
  rule list [--id=<id-pattern>] [--scope=<scope>]  List all registered rules
  rule run [--scope=<scope>] [--id=<id>]        Run specific rules
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
  region create <file-path> <region-path>       Create a region in a file
  region move <file-path> <region-path> <new-region-path>  Move a region in a file
  region delete <file-path> <region-path>       Delete a region in a file
  region list [--scope=<scope>]                 List regions in a file
  region tree [--scope=<scope>]                 Show region structure of a file
  definition list [--scope=<scope>]             List definitions in a file
  definition tree [--scope=<scope>]             Show definition structure
  tool <name> [args...]                         Run a tool (e.g., i18n, update-metabolism)

Options:
  --scope=<scope>          Limit operation to scope
  --id=<id>                Filter by rule ID or pattern
  --json                   Output as JSON
  --dry-run                Preview without making changes
  --help, -h               Show help for command

Scope syntax:
  @semio                   Repo scope (all files)
  @semio/js                Project scope (Nx project)
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
    return simpleGlob("**/*.{ts,tsx,py,cs}", { cwd: ROOT_DIR, ignore: ["node_modules/**", "**/node_modules/**", "**/.venv/**", "assets/repo/**"] });
  }
  if (scope.kind === "project") {
    const project = projects.find((p) => p.name === scope.projectName);
    if (!project) return [];
    return simpleGlob(`${project.root}/**/*.{ts,tsx,py,cs}`, { cwd: ROOT_DIR, ignore: ["**/node_modules/**", "**/.venv/**", "assets/repo/**"] });
  }
  if (scope.kind === "folder" && scope.filePath) {
    return simpleGlob(`${scope.filePath}**/*.{ts,tsx,py,cs}`, { cwd: ROOT_DIR, ignore: ["**/node_modules/**", "**/.venv/**", "assets/repo/**"] });
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
    if (targetScope.kind === "repo" && pattern.startsWith("**/*.")) return true;
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

// #region Rule Engine

const RULES: RegisteredRule[] = [];

function registerRule(meta: RuleMeta, run: RuleFn): void {
  RULES.push({ meta, run });
}

function createRuleContext(scope: Scope, projects: NxProject[]): RuleContext {
  const fileCache = new Map<string, string>();
  const regionCache = new Map<string, RegionInfo[]>();
  const definitionCache = new Map<string, DefinitionInfo[]>();
  const astCache = new Map<string, ASTFile | null>();
  return {
    scope,
    rootDir: ROOT_DIR,
    projects: () => projects,
    project: (name: string) => projects.find((p) => p.name === name),
    files: (pattern?: string) => {
      if (pattern) return simpleGlob(pattern, { cwd: ROOT_DIR, ignore: ["**/node_modules/**", "**/.venv/**", "assets/repo/**"] });
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
      priority: RULES.find((r) => r.meta.id === partial.kind)?.meta.priority ?? "medium",
      autofixable: RULES.find((r) => r.meta.id === partial.kind)?.meta.autofixable ?? false,
    }),
    createFix: (description, edits) => ({ description, edits }),
  };
}

async function runRules(scope: Scope, ruleIds?: string[]): Promise<Violation[]> {
  const projects = getNxProjects();
  const violations: Violation[] = [];
  const rulesToRun = ruleIds ? RULES.filter((r) => ruleIds.includes(r.meta.id)) : RULES.filter((r) => matchesScope(r.meta.scopes, scope));
  for (const rule of rulesToRun) {
    const ctx = createRuleContext(scope, projects);
    const ruleViolations = await rule.run(ctx);
    violations.push(...ruleViolations);
  }
  return violations;
}

// #endregion Rule Engine

// #region Built-in Rules

//#region Header Rule
registerRule({ id: "header", name: "Header", description: "Validates source file header region with filename, contributors, and AGPL-3.0 license", scopes: ["**/*.{ts,tsx,py,cs}"], priority: "high", autofixable: true }, async (ctx) => {
  const violations: Violation[] = [];
  const files = ctx.files();
  const agplMarkers = ["GNU Affero General Public License", "AGPL", "https://www.gnu.org/licenses/"];
  for (const file of files) {
    const content = ctx.readText(file);
    if (!content) continue;
    const lang = getLanguageFromPath(file);
    if (!lang) continue;
    const regions = ctx.regions(file);
    const headerRegion = regions.find((r) => r.name.toLowerCase() === "header");
    if (!headerRegion) {
      violations.push(
        ctx.createViolation({
          summary: `Missing header region in ${file}`,
          kind: "header:missing-region",
          solution: "Add a #region Header with filename, contributors, and AGPL-3.0 license",
          reason: "Every source file must include a header region",
          scope: file,
        }),
      );
      continue;
    }
    const headerContent = content.slice(headerRegion.startIndex, headerRegion.endIndex);
    const headerLines = headerContent.split("\n");
    const filename = file.split("/").pop() ?? file;
    const hasFilename = headerLines.some((l) => l.includes(filename));
    if (!hasFilename) {
      violations.push(
        ctx.createViolation({
          summary: `Missing filename in header of ${file}`,
          kind: "header:missing-filename",
          solution: `Add the filename "${filename}" to the header region`,
          reason: "Header must include the source file name",
          scope: `${file}#Header`,
          line: headerRegion.startLine,
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
          line: headerRegion.startLine,
        }),
      );
    }
    const hasLicense = agplMarkers.some((marker) => headerContent.includes(marker));
    if (!hasLicense) {
      violations.push(
        ctx.createViolation({
          summary: `Missing license in header of ${file}`,
          kind: "header:missing-license",
          solution: "Add AGPL-3.0 license text to the header region",
          reason: "Header must include AGPL-3.0 license",
          scope: `${file}#Header`,
          line: headerRegion.startLine,
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
            line: headerRegion.startLine,
          }),
        );
      }
    }
  }
  return violations;
});
//#endregion Header Rule

//#region Region Rule
registerRule({ id: "region", name: "Region", description: "Validates region blocks for proper naming and content", scopes: ["**/*.{ts,tsx,py,cs}"], priority: "low", autofixable: true }, async (ctx) => {
  const violations: Violation[] = [];
  const files = ctx.files();
  const regionPatterns: Record<string, { start: RegExp; end: RegExp }> = {
    typescript: { start: /^\s*\/\/\s*#region(?:\s+(\S.*?))?\s*$/i, end: /^\s*\/\/\s*#endregion(?:\s+(\S.*?))?\s*$/i },
    python: { start: /^\s*#\s*region(?:\s+(\S.*?))?\s*$/i, end: /^\s*#\s*endregion(?:\s+(\S.*?))?\s*$/i },
    csharp: { start: /^\s*#region(?:\s+(\S.*?))?\s*$/i, end: /^\s*#endregion(?:\s+(\S.*?))?\s*$/i },
  };
  function checkRegion(file: string, region: RegionInfo, content: string): void {
    const regionContent = content.slice(region.startIndex, region.endIndex);
    const lines = regionContent.split("\n").slice(1, -1);
    const nonEmptyLines = lines.filter((l) => l.trim() && !l.trim().startsWith("//") && !l.trim().startsWith("#"));
    if (nonEmptyLines.length === 0 && region.children.length === 0) {
      violations.push(
        ctx.createViolation({
          summary: `Empty region "${region.name}" in ${file}`,
          kind: "region:empty",
          solution: "Remove the empty region or add content to it",
          reason: "Empty regions add noise without providing value",
          scope: `${file}#${region.name}`,
          line: region.startLine,
        }),
      );
    }
    for (const child of region.children) {
      checkRegion(file, child, content);
    }
  }
  for (const file of files) {
    const content = ctx.readText(file);
    if (!content) continue;
    const lang = getLanguageFromPath(file);
    if (!lang) continue;
    const patterns = regionPatterns[lang];
    const lines = content.split("\n");
    const regionStack: { name: string; line: number }[] = [];
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i].replace(/\r$/, "");
      const lineNum = i + 1;
      const startMatch = line.match(patterns.start);
      if (startMatch) {
        const name = startMatch[1]?.trim() ?? "";
        if (!name) {
          violations.push(
            ctx.createViolation({
              summary: `Missing region name at ${file}:${lineNum}`,
              kind: "region:missing-start-name",
              solution: "Add a name after #region",
              reason: "Region blocks should have descriptive names",
              scope: file,
              line: lineNum,
              excerpt: line.trim(),
            }),
          );
        }
        regionStack.push({ name, line: lineNum });
        continue;
      }
      const endMatch = line.match(patterns.end);
      if (endMatch) {
        const endName = endMatch[1]?.trim() ?? "";
        const openRegion = regionStack.pop();
        if (openRegion && openRegion.name) {
          if (!endName) {
            violations.push(
              ctx.createViolation({
                summary: `Missing end region name at ${file}:${lineNum}`,
                kind: "region:missing-end-name",
                solution: `Add the region name "${openRegion.name}" after #endregion`,
                reason: "End region should match start region name for clarity",
                scope: file,
                line: lineNum,
                excerpt: line.trim(),
              }),
            );
          } else if (endName !== openRegion.name) {
            violations.push(
              ctx.createViolation({
                summary: `Region name mismatch at ${file}:${lineNum}`,
                kind: "region:name-mismatch",
                solution: `Change end name from "${endName}" to "${openRegion.name}"`,
                reason: "Start and end region names must match",
                scope: file,
                line: lineNum,
                excerpt: `Start: "${openRegion.name}" at line ${openRegion.line}, End: "${endName}"`,
              }),
            );
          }
        }
        continue;
      }
    }
    const regions = ctx.regions(file);
    for (const region of regions) {
      checkRegion(file, region, content);
    }
  }
  return violations;
});
//#endregion Region Rule

//#region Comment Rule
registerRule({ id: "comment", name: "Comment", description: "Detects forbidden comments (inline, block, JSDoc) - documentation belongs in README.md and AGENTS.md", scopes: ["**/*.{ts,tsx}"], priority: "low", autofixable: true }, async (ctx) => {
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
      if (trimmed.startsWith("// #region") || trimmed.startsWith("// #endregion") || trimmed.startsWith("//#region") || trimmed.startsWith("//#endregion")) continue;
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
//#endregion Comment Rule

// #endregion Built-in Rules

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

function endIteration(ticket: Ticket): void {
  if (!ticket.frontmatter.iterations || ticket.frontmatter.iterations.length === 0) {
    console.error("Error: No active iteration to end");
    return;
  }
  const lastIteration = ticket.frontmatter.iterations[ticket.frontmatter.iterations.length - 1];
  if (lastIteration.date.ended) {
    console.error("Error: Last iteration already ended");
    return;
  }
  lastIteration.date.ended = isoTimestamp();
  try {
    lastIteration.commit = execSync("git rev-parse HEAD", { encoding: "utf-8" }).trim();
  } catch {}
  saveTicket(ticket);
}

function finishTicket(ticket: Ticket): boolean {
  if (ticket.frontmatter.iterations && ticket.frontmatter.iterations.length > 0) {
    const lastIteration = ticket.frontmatter.iterations[ticket.frontmatter.iterations.length - 1];
    if (!lastIteration.date.ended) {
      console.error("Error: Cannot finish ticket with unfinished iteration");
      return false;
    }
  }
  ticket.frontmatter.status = "closed";
  ticket.frontmatter.date.finished = isoTimestamp();
  saveTicket(ticket);
  return true;
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

function closeTicket(ticket: Ticket): boolean {
  const { canClose, reasons } = canCloseTicket(ticket);
  if (!canClose) {
    console.error("Cannot close ticket:", reasons.join(", "));
    return false;
  }
  ticket.frontmatter.status = "closed";
  ticket.frontmatter.date.finished = isoTimestamp();
  saveTicket(ticket);
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

async function handleAnalyze(cmd: ParsedCommand): Promise<void> {
  const scopeRaws = cmd.args.length > 0 ? cmd.args : ["@semio"];
  const violations: Violation[] = [];
  for (const scopeRaw of scopeRaws) {
    const scope = parseScope(scopeRaw);
    const scopeViolations = await runRules(scope);
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
  writeJsonFile(join(REPORTS_DIR, "rules.json"), report);
  if (cmd.options.json) {
    console.log(JSON.stringify(report, null, 2));
  } else {
    console.log(`\n📊 Analysis complete: ${violations.length} violations found`);
    console.log(`   Report: ${join(REPORTS_DIR, "rules.json")}`);
  }
  process.exit(report.status === "error" ? 1 : 0);
}

async function handleFix(cmd: ParsedCommand): Promise<void> {
  const scopeRaw = cmd.args[0] || (cmd.options.scope as string) || "@semio";
  const scope = parseScope(scopeRaw);
  const dryRun = !!cmd.options["dry-run"];
  const violations = await runRules(scope);
  const fixable = violations.filter((i) => i.autofixable && i.autofix);
  if (dryRun) {
    console.log(`\n🔧 Dry run: ${fixable.length} fixable violations found`);
    for (const violation of fixable) {
      console.log(`   - ${violation.kind}: ${violation.summary}`);
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
    console.log(`\n✅ Fixed ${fixed} violations`);
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
    const violations = await runRules(scope, [ruleId]);
    console.log(`\n📊 Rule "${ruleId}" found ${violations.length} violations`);
    for (const violation of violations.slice(0, 10)) {
      console.log(`   - ${violation.summary}`);
    }
    if (violations.length > 10) {
      console.log(`   ... and ${violations.length - 10} more`);
    }
  } else {
    console.log("Usage: repo rule <list|run> [id]");
  }
  process.exit(0);
}

async function handleTicket(cmd: ParsedCommand): Promise<void> {
  if (cmd.subcommand === "create") {
    const slug = cmd.args[0];
    if (!slug) {
      console.error("Error: Ticket slug required");
      process.exit(1);
    }
    const prompt = (cmd.options.prompt as string) || slug;
    const model = cmd.options.model as string | undefined;
    const ticket = createTicket(slug, prompt, model);
    console.log(`\n🎫 Created ticket: ${ticket.slug}`);
    console.log(`   Path: ${ticket.filePath}`);
  } else if (cmd.subcommand === "iterate") {
    if (cmd.subsubcommand === "start") {
      const [year, month, day, slug] = cmd.args;
      if (!year || !month || !day || !slug) {
        console.error("Error: Format: ticket iterate start <year> <month> <day> <slug>");
        process.exit(1);
      }
      const ticket = readTicket(parseInt(year), parseInt(month), parseInt(day), slug);
      if (!ticket) {
        console.error("Error: Ticket not found");
        process.exit(1);
      }
      const prompt = (cmd.options.prompt as string) || "";
      const model = cmd.options.model as string | undefined;
      startIteration(ticket, prompt, model);
      console.log(`\n🔄 Started iteration on ticket: ${ticket.slug}`);
    } else if (cmd.subsubcommand === "end") {
      const [year, month, day, slug] = cmd.args;
      if (!year || !month || !day || !slug) {
        console.error("Error: Format: ticket iterate end <year> <month> <day> <slug>");
        process.exit(1);
      }
      const ticket = readTicket(parseInt(year), parseInt(month), parseInt(day), slug);
      if (!ticket) {
        console.error("Error: Ticket not found");
        process.exit(1);
      }
      endIteration(ticket);
      console.log(`\n✅ Ended iteration on ticket: ${ticket.slug}`);
    } else {
      console.log("Usage: repo ticket iterate <start|end> <year> <month> <day> <slug>");
    }
  } else if (cmd.subcommand === "finish") {
    const [year, month, day, slug] = cmd.args;
    if (!year || !month || !day || !slug) {
      console.error("Error: Format: ticket finish <year> <month> <day> <slug>");
      process.exit(1);
    }
    const ticket = readTicket(parseInt(year), parseInt(month), parseInt(day), slug);
    if (!ticket) {
      console.error("Error: Ticket not found");
      process.exit(1);
    }
    if (finishTicket(ticket)) {
      console.log(`\n✅ Ticket finished: ${ticket.slug}`);
    } else {
      process.exit(1);
    }
  } else if (cmd.subcommand === "list") {
    const year = cmd.options.year ? parseInt(cmd.options.year as string) : undefined;
    const month = cmd.options.month ? parseInt(cmd.options.month as string) : undefined;
    const day = cmd.options.day ? parseInt(cmd.options.day as string) : undefined;
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
    const [year, month, day, slug] = cmd.args;
    if (!year || !month || !day || !slug) {
      console.error("Error: Format: ticket read <year> <month> <day> <slug>");
      process.exit(1);
    }
    const ticket = readTicket(parseInt(year), parseInt(month), parseInt(day), slug);
    if (!ticket) {
      console.error("Error: Ticket not found");
      process.exit(1);
    }
    console.log(`\n🎫 Ticket: ${ticket.slug}`);
    console.log(`   Status: ${ticket.frontmatter.status}`);
    console.log(`   Created: ${ticket.frontmatter.date.created}`);
    console.log(`   Prompt: ${ticket.frontmatter.prompt}`);
    if (ticket.frontmatter.model) {
      console.log(`   Model: ${ticket.frontmatter.model}`);
    }
    console.log(`\n${ticket.content}`);
  } else {
    console.log("Usage: repo ticket <create|iterate|finish|list|read> [args]");
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
  if (cmd.subcommand === "create") {
    const folderPath = cmd.args[0];
    if (!folderPath) {
      console.error("Error: Folder path required");
      process.exit(1);
    }
    const absPath = join(ROOT_DIR, folderPath);
    if (existsSync(absPath)) {
      console.error(`Error: Folder already exists: ${folderPath}`);
      process.exit(1);
    }
    ensureDir(absPath);
    console.log(`\n📁 Created folder: ${folderPath}`);
  } else if (cmd.subcommand === "move") {
    const sourcePath = cmd.args[0];
    const targetPath = cmd.args[1];
    if (!sourcePath || !targetPath) {
      console.error("Error: Source and target paths required");
      process.exit(1);
    }
    const absSource = join(ROOT_DIR, sourcePath);
    const absTarget = join(ROOT_DIR, targetPath);
    if (!existsSync(absSource)) {
      console.error(`Error: Source folder not found: ${sourcePath}`);
      process.exit(1);
    }
    if (existsSync(absTarget)) {
      console.error(`Error: Target folder already exists: ${targetPath}`);
      process.exit(1);
    }
    const { renameSync } = await import("fs");
    ensureDir(dirname(absTarget));
    renameSync(absSource, absTarget);
    console.log(`\n📁 Moved folder: ${sourcePath} → ${targetPath}`);
  } else if (cmd.subcommand === "delete") {
    const folderPath = cmd.args[0];
    if (!folderPath) {
      console.error("Error: Folder path required");
      process.exit(1);
    }
    const absPath = join(ROOT_DIR, folderPath);
    if (!existsSync(absPath)) {
      console.error(`Error: Folder not found: ${folderPath}`);
      process.exit(1);
    }
    const { rmSync } = await import("fs");
    rmSync(absPath, { recursive: true });
    console.log(`\n🗑️ Deleted folder: ${folderPath}`);
  } else if (cmd.subcommand === "list") {
    const scopeRaw = cmd.args[0] || (cmd.options.scope as string) || ".";
    const absPath = join(ROOT_DIR, scopeRaw.replace(/\/$/, ""));
    if (!existsSync(absPath)) {
      console.error(`Error: Folder not found: ${scopeRaw}`);
      process.exit(1);
    }
    const allItems = readdirSync(absPath, { withFileTypes: true }).filter((f) => !f.name.startsWith(".") && f.isDirectory());
    const relativePaths = allItems.map((item) => getRelativePath(join(absPath, item.name)));
    const ignoredSet = getGitIgnoredSet(relativePaths);
    const folders = allItems.filter((item) => {
      const relPath = normalizePathSeparators(getRelativePath(join(absPath, item.name)));
      return !ignoredSet.has(relPath) && !ignoredSet.has(relPath + "/");
    });
    console.log(`\n📁 Found ${folders.length} folders in ${scopeRaw}:\n`);
    for (const folder of folders) {
      console.log(`   ${folder.name}/`);
    }
  } else if (cmd.subcommand === "tree") {
    const scopeRaw = cmd.args[0] || (cmd.options.scope as string) || ".";
    const absPath = join(ROOT_DIR, scopeRaw.replace(/\/$/, ""));
    if (!existsSync(absPath)) {
      console.error(`Error: Folder not found: ${scopeRaw}`);
      process.exit(1);
    }
    console.log(`\n📁 Folder tree: ${scopeRaw}\n`);
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
    console.log("Usage: repo folder <create|move|delete|list|tree> [args]");
  }
  process.exit(0);
}

async function handleFile(cmd: ParsedCommand): Promise<void> {
  if (cmd.subcommand === "create") {
    const filePath = cmd.args[0];
    if (!filePath) {
      console.error("Error: File path required");
      process.exit(1);
    }
    const absPath = join(ROOT_DIR, filePath);
    if (existsSync(absPath)) {
      console.error(`Error: File already exists: ${filePath}`);
      process.exit(1);
    }
    const lang = getLanguageFromPath(filePath);
    let content = "";
    if (lang) {
      const filename = filePath.split("/").pop() ?? filePath;
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
    console.log(`\n📄 Created file: ${filePath}`);
  } else if (cmd.subcommand === "move") {
    const sourcePath = cmd.args[0];
    const targetPath = cmd.args[1];
    if (!sourcePath || !targetPath) {
      console.error("Error: Source and target paths required");
      process.exit(1);
    }
    const absSource = join(ROOT_DIR, sourcePath);
    const absTarget = join(ROOT_DIR, targetPath);
    if (!existsSync(absSource)) {
      console.error(`Error: Source file not found: ${sourcePath}`);
      process.exit(1);
    }
    if (existsSync(absTarget)) {
      console.error(`Error: Target file already exists: ${targetPath}`);
      process.exit(1);
    }
    const { renameSync } = await import("fs");
    ensureDir(dirname(absTarget));
    renameSync(absSource, absTarget);
    console.log(`\n📄 Moved file: ${sourcePath} → ${targetPath}`);
  } else if (cmd.subcommand === "delete") {
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
    const { unlinkSync } = await import("fs");
    unlinkSync(absPath);
    console.log(`\n🗑️ Deleted file: ${filePath}`);
  } else if (cmd.subcommand === "list" || !cmd.subcommand) {
    const scopeRaw = cmd.args[0] || (cmd.options.scope as string) || "@semio";
    const scope = parseScope(scopeRaw);
    const projects = getNxProjects();
    const files = scopeToFiles(scope, projects);
    console.log(`\n📄 Found ${files.length} files in scope "${scopeRaw}":\n`);
    for (const file of files.slice(0, 50)) {
      console.log(`   ${file}`);
    }
    if (files.length > 50) {
      console.log(`   ... and ${files.length - 50} more`);
    }
  } else if (cmd.subcommand === "tree") {
    const scopeRaw = cmd.args[0] || (cmd.options.scope as string) || ".";
    const absPath = join(ROOT_DIR, scopeRaw.replace(/\/$/, ""));
    if (!existsSync(absPath)) {
      console.error(`Error: Path not found: ${scopeRaw}`);
      process.exit(1);
    }
    console.log(`\n📄 File tree: ${scopeRaw}\n`);
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
        console.log(`${prefix}${isLast ? "└── " : "├── "}${item}${isDir ? "/" : ""}`);
        if (isDir) {
          printTree(fullPath, prefix + (isLast ? "    " : "│   "));
        }
      });
    }
    printTree(absPath);
  } else {
    console.log("Usage: repo file <create|move|delete|list|tree> [args]");
  }
  process.exit(0);
}

async function handleRegion(cmd: ParsedCommand): Promise<void> {
  if (cmd.subcommand === "create") {
    const filePath = cmd.args[0];
    const regionPath = cmd.args[1];
    if (!filePath || !regionPath) {
      console.error("Error: File path and region path required");
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
    const regionName = regionPath.split("#").pop() ?? regionPath;
    let newRegion = "";
    if (lang === "typescript") {
      newRegion = `\n// #region ${regionName}\n\n// #endregion ${regionName}\n`;
    } else if (lang === "python") {
      newRegion = `\n# region ${regionName}\n\n# endregion ${regionName}\n`;
    } else if (lang === "csharp") {
      newRegion = `\n#region ${regionName}\n\n#endregion ${regionName}\n`;
    }
    writeTextFile(absPath, content + newRegion);
    console.log(`\n🏷️ Created region "${regionName}" in ${filePath}`);
  } else if (cmd.subcommand === "move") {
    const filePath = cmd.args[0];
    const oldRegionPath = cmd.args[1];
    const newRegionPath = cmd.args[2];
    if (!filePath || !oldRegionPath || !newRegionPath) {
      console.error("Error: File path, old region path, and new region path required");
      process.exit(1);
    }
    const absPath = join(ROOT_DIR, filePath);
    if (!existsSync(absPath)) {
      console.error(`Error: File not found: ${filePath}`);
      process.exit(1);
    }
    let content = readTextFile(absPath);
    const lang = getLanguageFromPath(filePath);
    if (!lang) {
      console.error("Error: Unsupported file type");
      process.exit(1);
    }
    const oldName = oldRegionPath.split("#").pop() ?? oldRegionPath;
    const newName = newRegionPath.split("#").pop() ?? newRegionPath;
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
    writeTextFile(absPath, content);
    console.log(`\n🏷️ Renamed region "${oldName}" to "${newName}" in ${filePath}`);
  } else if (cmd.subcommand === "delete") {
    const filePath = cmd.args[0];
    const regionPath = cmd.args[1];
    if (!filePath || !regionPath) {
      console.error("Error: File path and region path required");
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
    const regionName = regionPath.split("#").pop() ?? regionPath;
    function findRegion(regionList: RegionInfo[], name: string): RegionInfo | null {
      for (const r of regionList) {
        if (r.name === name) return r;
        const found = findRegion(r.children, name);
        if (found) return found;
      }
      return null;
    }
    const region = findRegion(regions, regionName);
    if (!region) {
      console.error(`Error: Region not found: ${regionName}`);
      process.exit(1);
    }
    const lines = content.split("\n");
    const newLines = lines.filter((_, i) => i + 1 < region.startLine || i + 1 > region.endLine);
    writeTextFile(absPath, newLines.join("\n"));
    console.log(`\n🗑️ Deleted region "${regionName}" from ${filePath}`);
  } else if (cmd.subcommand === "list") {
    const scopeRaw = cmd.args[0] || (cmd.options.scope as string);
    if (!scopeRaw) {
      console.error("Error: Scope or file path required");
      process.exit(1);
    }
    const scope = parseScope(scopeRaw);
    if (scope.kind !== "file" && scope.kind !== "region") {
      console.error("Error: Scope must be a file or region");
      process.exit(1);
    }
    const filePath = scope.filePath!;
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
    console.log(`\n🏷️ Regions in ${filePath}:\n`);
    function printRegion(region: RegionInfo, indent: string = ""): void {
      console.log(`${indent}${region.name} (lines ${region.startLine}-${region.endLine})`);
      for (const child of region.children) {
        printRegion(child, indent + "  ");
      }
    }
    for (const region of regions) {
      printRegion(region);
    }
    if (regions.length === 0) {
      console.log("   (no regions found)");
    }
  } else if (cmd.subcommand === "tree" || !cmd.subcommand) {
    const scopeRaw = cmd.args[0] || (cmd.options.scope as string);
    if (!scopeRaw) {
      console.error("Error: File path required");
      process.exit(1);
    }
    const filePath = scopeRaw.split("#")[0];
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
  } else {
    console.log("Usage: repo region <create|move|delete|list|tree> [args]");
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

// #endregion
