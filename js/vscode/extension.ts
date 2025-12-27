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

import { applyKitDiff, deserializeKit, Fix, Issue, Kit, SemioDomainLocation, serializeKit, validateSemioKit } from "@semio/js/semio";
import * as fs from "fs";
import * as jsonc from "jsonc-parser";
import * as path from "path";
import * as vscode from "vscode";

// #endregion Imports

// #region Constants

const SEMIO_KIT_LANGUAGE = "json";
const DIAGNOSTIC_SOURCE_KIT = "semio-kit";
const DIAGNOSTIC_SOURCE_REPO = "semio-repo";
const ANALYZE_REPORT_PATH = "reports/rules.json";

// #endregion Constants

// #region Types

interface RepoIssue {
  id: string;
  summary: string;
  kind: string;
  priority: "high" | "medium" | "low";
  severity: "error" | "warning";
  autofixable: boolean;
  solution: string;
  reason: string;
  scope: string;
  line?: number;
  column?: number;
  excerpt?: string;
}

interface AnalyzeReport {
  timestamp: string;
  status: string;
  scope: string;
  summary: {
    total: number;
    byPriority: Record<string, number>;
    bySeverity: Record<string, number>;
    byKind: Record<string, number>;
  };
  issues: RepoIssue[];
}

// #endregion Types

// #region Utilities

function getWorkspaceRoot(): string | undefined {
  return vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
}

function getRepoTsxPath(): string | undefined {
  const root = getWorkspaceRoot();
  if (!root) return undefined;
  const repoPath = path.join(root, "repo.tsx");
  return fs.existsSync(repoPath) ? repoPath : undefined;
}

// #endregion Utilities

// #region Repo Diagnostics

let repoDiagnosticCollection: vscode.DiagnosticCollection;

function loadAnalyzeReport(): AnalyzeReport | undefined {
  const root = getWorkspaceRoot();
  if (!root) return undefined;
  const reportPath = path.join(root, ANALYZE_REPORT_PATH);
  if (!fs.existsSync(reportPath)) return undefined;
  try {
    const content = fs.readFileSync(reportPath, "utf-8");
    return JSON.parse(content) as AnalyzeReport;
  } catch {
    return undefined;
  }
}

function extractFilePathFromScope(scope: string): string | undefined {
  if (scope.endsWith(".ts") || scope.endsWith(".tsx") || scope.endsWith(".js") || scope.endsWith(".json") || scope.endsWith(".py") || scope.endsWith(".cs")) {
    return scope.split("#")[0].split("§")[0];
  }
  return undefined;
}

function updateRepoDiagnostics(): void {
  repoDiagnosticCollection.clear();
  const report = loadAnalyzeReport();
  if (!report || report.issues.length === 0) return;
  const root = getWorkspaceRoot();
  if (!root) return;
  const diagnosticsByFile = new Map<string, vscode.Diagnostic[]>();
  for (const issue of report.issues) {
    const filePath = extractFilePathFromScope(issue.scope);
    if (!filePath) continue;
    const absPath = path.join(root, filePath);
    const uri = vscode.Uri.file(absPath);
    const line = Math.max(0, (issue.line ?? 1) - 1);
    const column = Math.max(0, (issue.column ?? 1) - 1);
    const range = new vscode.Range(line, column, line, column + 1);
    const severity = issue.severity === "error" ? vscode.DiagnosticSeverity.Error : vscode.DiagnosticSeverity.Warning;
    const diagnostic = new vscode.Diagnostic(range, `${issue.summary}\n\nReason: ${issue.reason}\nSolution: ${issue.solution}`, severity);
    diagnostic.source = DIAGNOSTIC_SOURCE_REPO;
    diagnostic.code = issue.id;
    if (!diagnosticsByFile.has(uri.toString())) {
      diagnosticsByFile.set(uri.toString(), []);
    }
    diagnosticsByFile.get(uri.toString())!.push(diagnostic);
  }
  for (const [uriString, diagnostics] of diagnosticsByFile) {
    repoDiagnosticCollection.set(vscode.Uri.parse(uriString), diagnostics);
  }
}

function watchAnalyzeReport(context: vscode.ExtensionContext): void {
  const root = getWorkspaceRoot();
  if (!root) return;
  const watcher = vscode.workspace.createFileSystemWatcher(new vscode.RelativePattern(root, ANALYZE_REPORT_PATH));
  watcher.onDidChange(() => updateRepoDiagnostics());
  watcher.onDidCreate(() => updateRepoDiagnostics());
  watcher.onDidDelete(() => repoDiagnosticCollection.clear());
  context.subscriptions.push(watcher);
}

// #endregion Repo Diagnostics

// #region Kit Validation

let kitDiagnosticCollection: vscode.DiagnosticCollection;

function isSemioKitDocument(document: vscode.TextDocument): boolean {
  if (document.languageId !== SEMIO_KIT_LANGUAGE) return false;
  const basename = document.uri.path.split("/").pop()?.toLowerCase() || "";
  return basename.startsWith("kit_") || basename.includes("_kit") || basename === "kit.json";
}

function issueToDiagnostic(document: vscode.TextDocument, issue: Issue): vscode.Diagnostic {
  const range = locationToRange(document, issue.location);
  const severity = issue.severity === "error" ? vscode.DiagnosticSeverity.Error : vscode.DiagnosticSeverity.Warning;
  const diagnostic = new vscode.Diagnostic(range, issue.message, severity);
  diagnostic.source = DIAGNOSTIC_SOURCE_KIT;
  diagnostic.code = issue.constraintId;
  if (issue.relatedGuids && issue.relatedGuids.length > 1) {
    diagnostic.relatedInformation = issue.relatedGuids.slice(1).map((guid) => {
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
    Quality: "quality",
    Interface: "interfaces",
    File: "files",
    Folder: "folders",
    Piece: "pieces",
    Connection: "connections",
    Stat: "stats",
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
  if (location.entityKind === "Piece" || location.entityKind === "Connection" || location.entityKind === "Stat") {
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
    const diagnostics = result.issues.map((issue) => issueToDiagnostic(document, issue));
    kitDiagnosticCollection.set(document.uri, diagnostics);
  } catch (error) {
    console.error("Failed to validate semio kit:", error);
    kitDiagnosticCollection.delete(document.uri);
  }
}

class SemioKitCodeActionProvider implements vscode.CodeActionProvider {
  provideCodeActions(document: vscode.TextDocument, range: vscode.Range | vscode.Selection, context: vscode.CodeActionContext): vscode.CodeAction[] | undefined {
    const kitDiagnostics = context.diagnostics.filter((d) => d.source === DIAGNOSTIC_SOURCE_KIT);
    if (kitDiagnostics.length === 0) return undefined;
    const actions: vscode.CodeAction[] = [];
    for (const diagnostic of kitDiagnostics) {
      try {
        const text = document.getText();
        const kit = deserializeKit(text);
        const result = validateSemioKit(kit);
        const issue = result.issues.find((i) => i.message === diagnostic.message && i.constraintId === diagnostic.code);
        if (!issue) continue;
        for (const fix of issue.fixes) {
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
    vscode.commands.registerCommand("semio.analyze", async () => {
      if (!getRepoTsxPath()) {
        vscode.window.showErrorMessage("repo.tsx not found in workspace");
        return;
      }
      const terminal = vscode.window.createTerminal("semio Analyze");
      terminal.show();
      terminal.sendText("npx tsx repo.tsx analyze --json");
      vscode.window.showInformationMessage("Running semio analyze...");
    }),
    vscode.commands.registerCommand("semio.analyzeFile", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!getRepoTsxPath()) {
        vscode.window.showErrorMessage("repo.tsx not found in workspace");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      const terminal = vscode.window.createTerminal("semio Analyze");
      terminal.show();
      terminal.sendText(`npx tsx repo.tsx analyze ${relativePath} --json`);
    }),
    vscode.commands.registerCommand("semio.fix", async () => {
      if (!getRepoTsxPath()) {
        vscode.window.showErrorMessage("repo.tsx not found in workspace");
        return;
      }
      const terminal = vscode.window.createTerminal("semio Fix");
      terminal.show();
      terminal.sendText("npx tsx repo.tsx fix");
      vscode.window.showInformationMessage("Running semio fix...");
    }),
    vscode.commands.registerCommand("semio.fixFile", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!getRepoTsxPath()) {
        vscode.window.showErrorMessage("repo.tsx not found in workspace");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      const terminal = vscode.window.createTerminal("semio Fix");
      terminal.show();
      terminal.sendText(`npx tsx repo.tsx fix ${relativePath}`);
    }),
    vscode.commands.registerCommand("semio.ruleList", async () => {
      if (!getRepoTsxPath()) {
        vscode.window.showErrorMessage("repo.tsx not found in workspace");
        return;
      }
      const terminal = vscode.window.createTerminal("semio Rules");
      terminal.show();
      terminal.sendText("npx tsx repo.tsx rule list");
    }),
    vscode.commands.registerCommand("semio.ticketNew", async () => {
      if (!getRepoTsxPath()) {
        vscode.window.showErrorMessage("repo.tsx not found in workspace");
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
      const terminal = vscode.window.createTerminal("semio Ticket");
      terminal.show();
      terminal.sendText(`npx tsx repo.tsx ticket new ${slug} --prompt="${prompt.replace(/"/g, '\\"')}"`);
    }),
    vscode.commands.registerCommand("semio.ticketList", async () => {
      if (!getRepoTsxPath()) {
        vscode.window.showErrorMessage("repo.tsx not found in workspace");
        return;
      }
      const terminal = vscode.window.createTerminal("semio Tickets");
      terminal.show();
      terminal.sendText("npx tsx repo.tsx ticket list");
    }),
    vscode.commands.registerCommand("semio.projectList", async () => {
      if (!getRepoTsxPath()) {
        vscode.window.showErrorMessage("repo.tsx not found in workspace");
        return;
      }
      const terminal = vscode.window.createTerminal("semio Projects");
      terminal.show();
      terminal.sendText("npx tsx repo.tsx project list");
    }),
    vscode.commands.registerCommand("semio.regionTree", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!getRepoTsxPath()) {
        vscode.window.showErrorMessage("repo.tsx not found in workspace");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      const terminal = vscode.window.createTerminal("semio Regions");
      terminal.show();
      terminal.sendText(`npx tsx repo.tsx region tree ${relativePath}`);
    }),
    vscode.commands.registerCommand("semio.definitionList", async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage("No active file");
        return;
      }
      if (!getRepoTsxPath()) {
        vscode.window.showErrorMessage("repo.tsx not found in workspace");
        return;
      }
      const relativePath = vscode.workspace.asRelativePath(editor.document.uri);
      const terminal = vscode.window.createTerminal("semio Definitions");
      terminal.show();
      terminal.sendText(`npx tsx repo.tsx definition list ${relativePath}`);
    }),
    vscode.commands.registerCommand("semio.folderTree", async () => {
      if (!getRepoTsxPath()) {
        vscode.window.showErrorMessage("repo.tsx not found in workspace");
        return;
      }
      const folderPath = await vscode.window.showInputBox({
        prompt: "Enter folder path (relative to workspace root)",
        placeHolder: "js/js/sketchpad",
        value: ".",
      });
      if (!folderPath) return;
      const depth = await vscode.window.showInputBox({
        prompt: "Enter depth (leave empty for unlimited)",
        placeHolder: "2",
      });
      const terminal = vscode.window.createTerminal("semio Folder");
      terminal.show();
      const depthArg = depth ? ` --depth=${depth}` : "";
      terminal.sendText(`npx tsx repo.tsx folder tree ${folderPath}${depthArg}`);
    }),
    vscode.commands.registerCommand("semio.refreshDiagnostics", () => {
      updateRepoDiagnostics();
      vscode.workspace.textDocuments.forEach(validateKitDocument);
      vscode.window.showInformationMessage("semio diagnostics refreshed");
    }),
  );
}

// #endregion Commands

// #region Activation

export function activate(context: vscode.ExtensionContext) {
  console.log("semio extension activated");
  kitDiagnosticCollection = vscode.languages.createDiagnosticCollection(DIAGNOSTIC_SOURCE_KIT);
  repoDiagnosticCollection = vscode.languages.createDiagnosticCollection(DIAGNOSTIC_SOURCE_REPO);
  context.subscriptions.push(kitDiagnosticCollection, repoDiagnosticCollection);
  context.subscriptions.push(
    vscode.workspace.onDidOpenTextDocument(validateKitDocument),
    vscode.workspace.onDidChangeTextDocument((e) => validateKitDocument(e.document)),
    vscode.workspace.onDidCloseTextDocument((doc) => kitDiagnosticCollection.delete(doc.uri)),
  );
  vscode.workspace.textDocuments.forEach(validateKitDocument);
  context.subscriptions.push(vscode.languages.registerCodeActionsProvider({ language: SEMIO_KIT_LANGUAGE }, new SemioKitCodeActionProvider(), { providedCodeActionKinds: [vscode.CodeActionKind.QuickFix] }));
  updateRepoDiagnostics();
  watchAnalyzeReport(context);
  registerCommands(context);
}

export function deactivate() { }

// #endregion Activation
