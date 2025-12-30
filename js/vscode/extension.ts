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

import { applyKitDiff, deserializeKit, Fix, Kit, Problem, SemioDomainLocation, serializeKit, validateSemioKit } from "@semio/js/semio";
import { exec } from "child_process";
import * as fs from "fs";
import * as jsonc from "jsonc-parser";
import * as path from "path";
import { promisify } from "util";
import * as vscode from "vscode";

const execAsync = promisify(exec);

// #endregion Imports

// #region Constants

const SEMIO_KIT_LANGUAGE = "json";
const DIAGNOSTIC_SOURCE = "semio";

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

interface Violation {
  id: string;
  summary: string;
  kind: string;
  priority: "high" | "medium" | "low";
  autofixable: boolean;
  solution: string;
  reason: string;
  scope: string;
  line?: number;
  column?: number;
  excerpt?: string;
  autofix?: AutoFix;
}

interface AnalyzeReport {
  timestamp: string;
  status: string;
  scope: string;
  summary: {
    total: number;
    byPriority: Record<string, number>;
    byKind: Record<string, number>;
  };
  violations: Violation[];
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
  if (fs.existsSync(binaryPath)) return binaryPath;
  const builtPath = path.join(root, "go", "bin", binaryName);
  if (fs.existsSync(builtPath)) return builtPath;
  return undefined;
}

function getRepoCommand(): { command: string; useGo: boolean } {
  const binaryPath = getRepoBinaryPath();
  if (binaryPath) {
    return { command: binaryPath, useGo: true };
  }
  const root = getWorkspaceRoot();
  if (root && fs.existsSync(path.join(root, "repo.tsx"))) {
    return { command: "npx tsx repo.tsx", useGo: false };
  }
  return { command: "", useGo: false };
}

function getRepoTsxPath(): string | undefined {
  const root = getWorkspaceRoot();
  if (!root) return undefined;
  const repoPath = path.join(root, "repo.tsx");
  return fs.existsSync(repoPath) ? repoPath : undefined;
}

function hasRepoAccess(): boolean {
  const { command } = getRepoCommand();
  return command !== "";
}

function runRepoCommand(args: string): void {
  const { command } = getRepoCommand();
  if (!command) return;
  const terminal = vscode.window.createTerminal("semio");
  terminal.show();
  terminal.sendText(`${command} ${args}`);
}

async function runRepoCommandJson<T>(args: string): Promise<T | null> {
  const root = getWorkspaceRoot();
  if (!root) return null;
  const { command } = getRepoCommand();
  if (!command) return null;
  try {
    const { stdout } = await execAsync(`${command} ${args}`, { cwd: root, timeout: 30000 });
    return JSON.parse(stdout) as T;
  } catch {
    return null;
  }
}

interface ToolResult<T = unknown> {
  output: { lines: { type: string; text: string }[]; exitCode: number };
  data?: T;
  error?: string;
}

interface TicketData {
  year: number;
  month: number;
  day: number;
  slug: string;
  frontmatter: { status: string; prompt: string; summary?: string };
  filePath: string;
}

interface PolicyData {
  id: string;
  name: string;
  description: string;
}

async function pickTicket(): Promise<TicketData | undefined> {
  const result = await runRepoCommandJson<ToolResult<TicketData[]>>("ticket list");
  if (!result?.data || result.data.length === 0) {
    vscode.window.showWarningMessage("No tickets found");
    return undefined;
  }
  const items = result.data.map((t) => ({
    label: `${t.year}/${String(t.month).padStart(2, "0")}/${String(t.day).padStart(2, "0")}/${t.slug}`,
    description: t.frontmatter.status === "closed" ? "✅" : "🟢",
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

// #endregion Utilities

// #region File Analysis

let repoDiagnosticCollection: vscode.DiagnosticCollection;
const fileViolationsMap = new Map<string, Violation[]>();

function extractFilePathFromScope(scope: string): string | undefined {
  if (scope.endsWith(".ts") || scope.endsWith(".tsx") || scope.endsWith(".js") || scope.endsWith(".json") || scope.endsWith(".py") || scope.endsWith(".cs")) {
    return scope.split("#")[0].split("§")[0];
  }
  return undefined;
}

function shouldAnalyzeFile(document: vscode.TextDocument): boolean {
  const supportedLanguages = ["typescript", "javascript", "typescriptreact", "javascriptreact", "json", "python", "csharp"];
  return supportedLanguages.includes(document.languageId);
}

async function analyzeFile(document: vscode.TextDocument): Promise<void> {
  if (!shouldAnalyzeFile(document)) return;
  const root = getWorkspaceRoot();
  if (!root) return;
  if (!hasRepoAccess()) return;
  const { command, useGo } = getRepoCommand();
  const relativePath = vscode.workspace.asRelativePath(document.uri);
  try {
    const cmd = useGo ? `${command} analyze --scope=${relativePath} --json` : `${command} analyze --scope=${relativePath} --json`;
    const { stdout } = await execAsync(cmd, { cwd: root, timeout: 30000 });
    const parsed = JSON.parse(stdout);
    const report: AnalyzeReport = useGo ? parsed.output ? { ...parsed, violations: parsed.data?.violations || [] } : parsed : parsed;
    fileViolationsMap.set(document.uri.toString(), report.violations || []);
    updateFileDiagnostics(document, report.violations || []);
  } catch (error) {
    if (error instanceof Error && "stdout" in error) {
      try {
        const stdout = (error as { stdout: string }).stdout;
        const parsed = JSON.parse(stdout);
        const report: AnalyzeReport = parsed.data || parsed;
        fileViolationsMap.set(document.uri.toString(), report.violations || []);
        updateFileDiagnostics(document, report.violations || []);
      } catch {
        console.error("Failed to parse analyze output:", error);
      }
    } else {
      console.error("Failed to run analyze:", error);
    }
  }
}

function getPolicyLineNumber(root: string, kind: string): number | undefined {
  const repoPath = path.join(root, "repo.tsx");
  if (!fs.existsSync(repoPath)) return undefined;
  const policyPrefix = kind.split(":")[0];
  const policyName = policyPrefix.charAt(0).toUpperCase() + policyPrefix.slice(1);
  const content = fs.readFileSync(repoPath, "utf-8");
  const lines = content.split("\n");
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].includes(`#region ${policyName} Policy`)) return i + 1;
  }
  return undefined;
}

function updateFileDiagnostics(document: vscode.TextDocument, violations: Violation[]): void {
  const root = getWorkspaceRoot();
  if (!root) return;
  const diagnostics: vscode.Diagnostic[] = [];
  for (const violation of violations) {
    const filePath = extractFilePathFromScope(violation.scope);
    if (!filePath) continue;
    const absPath = path.join(root, filePath);
    if (absPath !== document.uri.fsPath) continue;
    const line = Math.max(0, (violation.line ?? 1) - 1);
    const column = Math.max(0, (violation.column ?? 1) - 1);
    const endColumn = violation.excerpt ? column + violation.excerpt.length : column + 1;
    const range = new vscode.Range(line, column, line, endColumn);
    const severity = violation.priority === "high" ? vscode.DiagnosticSeverity.Error : violation.priority === "medium" ? vscode.DiagnosticSeverity.Warning : vscode.DiagnosticSeverity.Information;
    const [policyId, violationName] = violation.kind.split(":");
    const diagnostic = new vscode.Diagnostic(range, violationName || violation.kind, severity);
    diagnostic.source = DIAGNOSTIC_SOURCE;
    const policyLine = getPolicyLineNumber(root, violation.kind);
    if (policyLine) {
      const repoUri = vscode.Uri.file(path.join(root, "repo.tsx")).with({ fragment: `L${policyLine}` });
      diagnostic.code = { value: policyId, target: repoUri };
    } else {
      diagnostic.code = policyId;
    }
    diagnostics.push(diagnostic);
  }
  repoDiagnosticCollection.set(document.uri, diagnostics);
}

class SemioRepoCodeActionProvider implements vscode.CodeActionProvider {
  provideCodeActions(document: vscode.TextDocument, range: vscode.Range | vscode.Selection, context: vscode.CodeActionContext): vscode.CodeAction[] | undefined {
    const repoDiagnostics = context.diagnostics.filter((d) => d.source === DIAGNOSTIC_SOURCE);
    if (repoDiagnostics.length === 0) return undefined;
    const violations = fileViolationsMap.get(document.uri.toString()) || [];
    const actions: vscode.CodeAction[] = [];
    for (const diagnostic of repoDiagnostics) {
      const diagnosticLine = diagnostic.range.start.line + 1;
      const policyId = typeof diagnostic.code === "object" && diagnostic.code !== null ? (diagnostic.code as { value: string }).value : diagnostic.code;
      const violation = violations.find((v) => v.kind.startsWith(`${policyId}:`) && (v.line ?? 1) === diagnosticLine);
      if (!violation) continue;
      const action = createRepoCodeAction(document, diagnostic, violation);
      if (action) actions.push(action);
    }
    return actions;
  }
}

function createRepoCodeAction(document: vscode.TextDocument, diagnostic: vscode.Diagnostic, violation: Violation): vscode.CodeAction | undefined {
  const relativePath = vscode.workspace.asRelativePath(document.uri);
  const action = new vscode.CodeAction(`Fix: ${violation.solution}`, vscode.CodeActionKind.QuickFix);
  action.diagnostics = [diagnostic];
  action.isPreferred = true;
  action.command = {
    command: "semio.fixViolation",
    title: "Fix violation",
    arguments: [relativePath],
  };
  return action;
}

async function fixViolation(relativePath: string): Promise<void> {
  const root = getWorkspaceRoot();
  if (!root) return;
  if (!hasRepoAccess()) {
    vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
    return;
  }
  const { command } = getRepoCommand();
  try {
    await vscode.window.withProgress({ location: vscode.ProgressLocation.Notification, title: "Fixing violation..." }, async () => {
      const { stdout, stderr } = await execAsync(`${command} fix ${relativePath}`, { cwd: root, timeout: 30000 });
      if (stderr) console.log("Fix stderr:", stderr);
      if (stdout) console.log("Fix stdout:", stdout);
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
    console.error("Failed to run fix:", error);
    vscode.window.showErrorMessage(`Failed to fix violation: ${error}`);
  }
}

// #endregion File Analysis

// #region Kit Validation

let kitDiagnosticCollection: vscode.DiagnosticCollection;

function isSemioKitDocument(document: vscode.TextDocument): boolean {
  if (document.languageId !== SEMIO_KIT_LANGUAGE) return false;
  const basename = document.uri.path.split("/").pop()?.toLowerCase() || "";
  return basename.startsWith("kit_") || basename.includes("_kit") || basename === "kit.json";
}

function problemToDiagnostic(document: vscode.TextDocument, problem: Problem): vscode.Diagnostic {
  const range = locationToRange(document, problem.location);
  const diagnostic = new vscode.Diagnostic(range, problem.message, vscode.DiagnosticSeverity.Error);
  diagnostic.source = DIAGNOSTIC_SOURCE;
  diagnostic.code = problem.constraintId;
  if (problem.relatedGuids && problem.relatedGuids.length > 1) {
    diagnostic.relatedInformation = problem.relatedGuids.slice(1).map((guid) => {
      const relatedRange = findGuidRange(document, guid);
      return new vscode.DiagnosticRelatedInformation(new vscode.Location(document.uri, relatedRange), `Related entity: ${guid}`);
    });
  }
  return diagnostic;
}

function locationToRange(document: vscode.TextDocument, location: SemioDomainLocation): vscode.Range {
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

function findEntityNode(tree: jsonc.Node, location: SemioDomainLocation): jsonc.Node | undefined {
  const entityKindToArrayName: Record<string, string> = {
    Type: "types",
    Design: "designs",
    Quality: "qualities",
    Interface: "ports",
    File: "files",
    Folder: "folders",
    Piece: "pieces",
    Connection: "connections",
    Stat: "stats",
    Model: "models",
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
  if (location.entityKind === "Model" || location.entityKind === "Connector") {
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
  if (!isSemioKitDocument(document)) return;
  try {
    const text = document.getText();
    const kit = deserializeKit(text);
    const result = validateSemioKit(kit);
    const diagnostics = result.problems.map((problem) => problemToDiagnostic(document, problem));
    kitDiagnosticCollection.set(document.uri, diagnostics);
  } catch (error) {
    console.error("Failed to validate semio kit:", error);
    kitDiagnosticCollection.delete(document.uri);
  }
}

class SemioKitCodeActionProvider implements vscode.CodeActionProvider {
  provideCodeActions(document: vscode.TextDocument, range: vscode.Range | vscode.Selection, context: vscode.CodeActionContext): vscode.CodeAction[] | undefined {
    const kitDiagnostics = context.diagnostics.filter((d) => d.source === DIAGNOSTIC_SOURCE);
    if (kitDiagnostics.length === 0) return undefined;
    const actions: vscode.CodeAction[] = [];
    for (const diagnostic of kitDiagnostics) {
      try {
        const text = document.getText();
        const kit = deserializeKit(text);
        const result = validateSemioKit(kit);
        const problem = result.problems.find((i) => i.message === diagnostic.message && i.constraintId === diagnostic.code);
        if (!problem) continue;
        for (const fix of problem.fixes) {
          const action = createKitCodeAction(document, diagnostic, fix, kit);
          if (action) actions.push(action);
        }
      } catch (error) {
        console.error("Failed to generate code actions:", error);
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
    console.error("Failed to create code action:", error);
    return undefined;
  }
}

// #endregion Kit Validation

// #region Commands

function registerCommands(context: vscode.ExtensionContext): void {
  context.subscriptions.push(
    vscode.commands.registerCommand("semio.fixViolation", fixViolation),
    vscode.commands.registerCommand("semio.analyze", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
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
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      runRepoCommand(`analyze ${relativePath}`);
    }),
    vscode.commands.registerCommand("semio.fix", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
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
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      runRepoCommand(`fix ${relativePath}`);
    }),
    vscode.commands.registerCommand("semio.policyList", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      runRepoCommand("policy list");
    }),
    vscode.commands.registerCommand("semio.ticketCreate", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const slug = await vscode.window.showInputBox({
        prompt: "Enter ticket slug (e.g., MY-FEATURE)",
        placeHolder: "TICKET-SLUG",
      });
      if (!slug) return;
      const prompt = await vscode.window.showInputBox({
        prompt: "Enter ticket description",
        placeHolder: "What needs to be done?",
      });
      if (!prompt) return;
      runRepoCommand(`ticket create ${slug} --prompt="${prompt.replace(/"/g, '\\"')}"`);
    }),
    vscode.commands.registerCommand("semio.ticketList", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      runRepoCommand("ticket list");
    }),
    vscode.commands.registerCommand("semio.ticketIterateStart", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const ticket = await pickTicket();
      if (!ticket) return;
      const prompt = await vscode.window.showInputBox({ prompt: "Enter iteration prompt (optional)", placeHolder: "What will be done in this iteration?" });
      const promptArg = prompt ? ` --prompt="${prompt.replace(/"/g, '\\"')}"` : "";
      runRepoCommand(`ticket iterate start ${ticket.year} ${ticket.month} ${ticket.day} ${ticket.slug}${promptArg}`);
    }),
    vscode.commands.registerCommand("semio.ticketIterateEnd", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const ticket = await pickTicket();
      if (!ticket) return;
      runRepoCommand(`ticket iterate end ${ticket.year} ${ticket.month} ${ticket.day} ${ticket.slug}`);
    }),
    vscode.commands.registerCommand("semio.ticketFinish", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const ticket = await pickTicket();
      if (!ticket) return;
      runRepoCommand(`ticket finish ${ticket.year} ${ticket.month} ${ticket.day} ${ticket.slug}`);
    }),
    vscode.commands.registerCommand("semio.ticketRead", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const ticket = await pickTicket();
      if (!ticket) return;
      runRepoCommand(`ticket read ${ticket.year} ${ticket.month} ${ticket.day} ${ticket.slug}`);
    }),
    vscode.commands.registerCommand("semio.ticketOpen", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const ticket = await pickTicket();
      if (!ticket) return;
      const root = getWorkspaceRoot();
      if (!root) return;
      const ticketUri = vscode.Uri.file(path.join(root, ticket.filePath));
      const doc = await vscode.workspace.openTextDocument(ticketUri);
      await vscode.window.showTextDocument(doc);
    }),
    vscode.commands.registerCommand("semio.projectList", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      runRepoCommand("project list");
    }),
    vscode.commands.registerCommand("semio.sectionTree", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      runRepoCommand(`section tree ${relativePath}`);
    }),
    vscode.commands.registerCommand("semio.definitionList", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      runRepoCommand(`definition list ${relativePath}`);
    }),
    vscode.commands.registerCommand("semio.folderTree", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const folderPath = await vscode.window.showInputBox({
        prompt: "Enter folder path (relative to workspace root)",
        placeHolder: "js/js/sketchpad",
        value: ".",
      });
      if (!folderPath) return;
      runRepoCommand(`folder tree ${folderPath}`);
    }),
    vscode.commands.registerCommand("semio.folderCreate", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
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
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const sourcePath = await vscode.window.showInputBox({
        prompt: "Enter source folder path",
        placeHolder: "js/js/old-folder",
      });
      if (!sourcePath) return;
      const targetPath = await vscode.window.showInputBox({
        prompt: "Enter target folder path",
        placeHolder: "js/js/new-folder",
      });
      if (!targetPath) return;
      runRepoCommand(`folder move ${sourcePath} ${targetPath}`);
    }),
    vscode.commands.registerCommand("semio.folderDelete", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const folderPath = await vscode.window.showInputBox({
        prompt: "Enter folder path to delete",
        placeHolder: "js/js/folder-to-delete",
      });
      if (!folderPath) return;
      runRepoCommand(`folder delete ${folderPath}`);
    }),
    vscode.commands.registerCommand("semio.folderList", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const folderPath = await vscode.window.showInputBox({
        prompt: "Enter folder path (relative to workspace root)",
        placeHolder: "js/js/sketchpad",
        value: ".",
      });
      if (!folderPath) return;
      runRepoCommand(`folder list ${folderPath}`);
    }),
    vscode.commands.registerCommand("semio.fileCreate", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
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
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const sourcePath = await vscode.window.showInputBox({
        prompt: "Enter source file path",
        placeHolder: "js/js/old-file.ts",
      });
      if (!sourcePath) return;
      const targetPath = await vscode.window.showInputBox({
        prompt: "Enter target file path",
        placeHolder: "js/js/new-file.ts",
      });
      if (!targetPath) return;
      runRepoCommand(`file move ${sourcePath} ${targetPath}`);
    }),
    vscode.commands.registerCommand("semio.fileDelete", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const filePath = await vscode.window.showInputBox({
        prompt: "Enter file path to delete",
        placeHolder: "js/js/file-to-delete.ts",
      });
      if (!filePath) return;
      runRepoCommand(`file delete ${filePath}`);
    }),
    vscode.commands.registerCommand("semio.fileList", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const folderPath = await vscode.window.showInputBox({
        prompt: "Enter folder path (relative to workspace root)",
        placeHolder: "js/js/sketchpad",
        value: ".",
      });
      if (!folderPath) return;
      runRepoCommand(`file list ${folderPath}`);
    }),
    vscode.commands.registerCommand("semio.fileTree", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const folderPath = await vscode.window.showInputBox({
        prompt: "Enter folder path (relative to workspace root)",
        placeHolder: "js/js/sketchpad",
        value: ".",
      });
      if (!folderPath) return;
      runRepoCommand(`file tree ${folderPath}`);
    }),
    vscode.commands.registerCommand("semio.sectionCreate", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
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
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
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
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
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
    vscode.commands.registerCommand("semio.sectionList", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      runRepoCommand(`section list ${relativePath}`);
    }),
    vscode.commands.registerCommand("semio.definitionTree", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      runRepoCommand(`definition tree ${relativePath}`);
    }),
    vscode.commands.registerCommand("semio.projectTree", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      runRepoCommand("project tree");
    }),
    vscode.commands.registerCommand("semio.policyRun", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const policy = await pickPolicy();
      if (!policy) return;
      runRepoCommand(`policy run ${policy.id}`);
    }),
    vscode.commands.registerCommand("semio.toolRun", async () => {
      if (!hasRepoAccess()) {
        vscode.window.showErrorMessage("repo binary or repo.tsx not found in workspace");
        return;
      }
      const toolName = await vscode.window.showInputBox({
        prompt: "Enter tool name",
        placeHolder: "i18n",
      });
      if (!toolName) return;
      runRepoCommand(`tool ${toolName}`);
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

export function activate(context: vscode.ExtensionContext) {
  console.log("semio extension activated");
  kitDiagnosticCollection = vscode.languages.createDiagnosticCollection(DIAGNOSTIC_SOURCE);
  repoDiagnosticCollection = vscode.languages.createDiagnosticCollection(DIAGNOSTIC_SOURCE);
  context.subscriptions.push(kitDiagnosticCollection, repoDiagnosticCollection);
  context.subscriptions.push(
    vscode.workspace.onDidOpenTextDocument(validateKitDocument),
    vscode.workspace.onDidChangeTextDocument((e) => validateKitDocument(e.document)),
    vscode.workspace.onDidCloseTextDocument((doc) => kitDiagnosticCollection.delete(doc.uri)),
  );
  vscode.workspace.textDocuments.forEach(validateKitDocument);
  context.subscriptions.push(vscode.languages.registerCodeActionsProvider({ language: SEMIO_KIT_LANGUAGE }, new SemioKitCodeActionProvider(), { providedCodeActionKinds: [vscode.CodeActionKind.QuickFix] }));
  context.subscriptions.push(
    vscode.workspace.onDidOpenTextDocument(analyzeFile),
    vscode.workspace.onDidSaveTextDocument(analyzeFile),
    vscode.workspace.onDidCloseTextDocument((doc) => {
      fileViolationsMap.delete(doc.uri.toString());
      repoDiagnosticCollection.delete(doc.uri);
    }),
  );
  vscode.workspace.textDocuments.forEach(analyzeFile);
  context.subscriptions.push(vscode.languages.registerCodeActionsProvider("*", new SemioRepoCodeActionProvider(), { providedCodeActionKinds: [vscode.CodeActionKind.QuickFix] }));
  registerCommands(context);
}

export function deactivate() { }

// #endregion Activation
