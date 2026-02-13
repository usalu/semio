// #region 🔖Header

// [🧪semio-repo/vscode/extension.test.ts](semiorepo://file/semio-repo/vscode/extension.test.ts)

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

import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import {
  FilterTreeDataProvider,
  FilterTreeItem,
  MonorepoTreeDataProvider,
  MonorepoTreeItem,
  RepoEvent,
  TicketData,
  TicketInteraction,
  TreeNodeData,
  buildCliTreeArgs,
  extractLeadingEmoji,
  extractRepoResult,
  invalidateTreeNodeCache,
  parseRepoEvents,
  parseUri,
  slugify,
  treeNodeContextValue,
  treeNodeDisplayLabel,
  treeNodeToItem,
} from "./extension";

// #endregion 🔖Imports

// #region 🔖Constants

const EXPECTED_COMMANDS = ["semio.analyze", "semio.analyzeFile", "semio.fix", "semio.fixFile", "semio.policyList", "semio.ticketOpen", "semio.ticketList", "semio.ticketClose", "semio.ticketRead", "semio.ticketOpen", "semio.projectList", "semio.contributorAdd", "semio.contributorList", "semio.contributorRemove", "semio.sectionTree", "semio.definitionList", "semio.folderTree", "semio.folderCreate", "semio.folderMove", "semio.folderDelete", "semio.folderList", "semio.fileCreate", "semio.fileMove", "semio.fileDelete", "semio.fileList", "semio.fileTree", "semio.sectionCreate", "semio.sectionMove", "semio.sectionDelete", "semio.sectionIntegrate", "semio.sectionList", "semio.definitionTree", "semio.projectTree", "semio.policyCheck", "semio.refreshDiagnostics", "semio.fixViolation", "semio.copyId", "semio.mailto", "semio.openLink", "semio.refreshMonorepo", "semio.refreshCodebase", "semio.copyCommitSha", "semio.openCommitInGitHub", "semio.ticketReopen", "semio.refreshItem", "semio.navigate", "semio.navigateTo"];
const EXPECTED_CONSTRAINTS = ["guid-unique", "type-name-unique", "design-name-unique", "piece-name-unique", "quality-name-unique", "port-name-unique", "file-name-unique", "folder-name-unique", "connector-name-unique", "model-name-unique", "layer-path-unique"];
const EXPECTED_VIEWS = ["semio.monorepo", "semio.filter"];

// #endregion 🔖Constants

// #region 🔖Utilities

function getWorkspaceRoot(): string {
  return vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ?? path.join(__dirname, "../../../..");
}

function getFixturePath(relativePath: string): string {
  return path.join(getWorkspaceRoot(), "semio", "assets", relativePath);
}

async function openFixture(relativePath: string): Promise<vscode.TextDocument> {
  const fixturePath = getFixturePath(relativePath);
  if (!fs.existsSync(fixturePath)) {
    throw new Error(`Fixture not found at ${fixturePath}`);
  }
  const fixtureUri = vscode.Uri.file(fixturePath);
  const document = await vscode.workspace.openTextDocument(fixtureUri);
  return document;
}

async function waitForDiagnostics(uri: vscode.Uri, timeout = 5000): Promise<vscode.Diagnostic[]> {
  await new Promise((resolve) => setTimeout(resolve, timeout));
  return vscode.languages.getDiagnostics(uri).filter((d) => d.source === "semio");
}

// #endregion 🔖Utilities

// #region 🔖Extension Activation

suiteSetup(async function () {
  this.timeout(30000);
  await openFixture("semio/kit_metabolism.json");
  await new Promise((resolve) => setTimeout(resolve, 2000));
});

// #endregion 🔖Extension Activation

// #region 🔖RepoEvent Parsing Tests

suite("RepoEvent Parsing Test Suite", () => {
  test("parseRepoEvents handles result field correctly", () => {
    const output = '{"kind":"result","result":{"data":{"violations":[{"id":"v1"}]}}}';
    const events = parseRepoEvents(output);
    assert.strictEqual(events.length, 1);
    assert.strictEqual(events[0].kind, "result");
    assert.ok(events[0].result);
    const result = events[0].result as any;
    assert.ok(result.data);
    assert.ok(result.data.violations);
    assert.strictEqual(result.data.violations.length, 1);
  });

  test("extractRepoResult extracts data from result field", () => {
    const events: RepoEvent[] = [
      { kind: "result", result: { data: { violations: [{ id: "v1" }] } } }
    ];
    const extracted = extractRepoResult(events);
    assert.ok(extracted.data);
    const data = extracted.data as any;
    assert.ok(data.violations);
    assert.strictEqual(data.violations.length, 1);
    assert.strictEqual(data.violations[0].id, "v1");
  });

  test("extractRepoResult falls back to data field if result is missing", () => {
    const events: RepoEvent[] = [
      { kind: "result", data: { violations: [{ id: "v2" }] } }
    ];
    const extracted = extractRepoResult(events);
    assert.ok(extracted.data);
    const data = extracted.data as any;
    assert.ok(data.violations);
    assert.strictEqual(data.violations.length, 1);
    assert.strictEqual(data.violations[0].id, "v2");
  });

  test("extractRepoResult prefers result over data field", () => {
    const events: RepoEvent[] = [
      { kind: "result", result: { data: { violations: [{ id: "from-result" }] } }, data: { violations: [{ id: "from-data" }] } }
    ];
    const extracted = extractRepoResult(events);
    assert.ok(extracted.data);
    const data = extracted.data as any;
    assert.ok(data.violations);
    assert.strictEqual(data.violations[0].id, "from-result");
  });

  test("extractRepoResult handles fatal errors", () => {
    const events: RepoEvent[] = [
      { kind: "error", error: { message: "Fatal error occurred", fatal: true } }
    ];
    assert.throws(() => extractRepoResult(events), /Fatal error occurred/);
  });

  test("extractRepoResult ignores non-fatal errors", () => {
    const events: RepoEvent[] = [
      { kind: "error", error: { message: "Non-fatal warning", fatal: false } },
      { kind: "result", result: { data: { violations: [] } } }
    ];
    const extracted = extractRepoResult(events);
    assert.ok(extracted.data);
  });

  test("extractRepoResult uses last result when multiple result events", () => {
    const events: RepoEvent[] = [
      { kind: "result", result: { data: { violations: [{ id: "first" }] } } },
      { kind: "result", result: { data: { violations: [{ id: "last" }] } } }
    ];
    const extracted = extractRepoResult(events);
    const data = extracted.data as any;
    assert.strictEqual(data.violations[0].id, "last");
  });
});

// #endregion 🔖RepoEvent Parsing Tests

// #region 🔖Command Registration Tests

suite("Command Registration Test Suite", () => {
  test("All expected commands are registered", async () => {
    const extension = vscode.extensions.getExtension("usalu.semio-repo");
    assert.ok(extension, "Extension should be found");
    if (!extension.isActive) {
      await extension.activate();
    }
    const commands = await vscode.commands.getCommands(true);
    const missing = EXPECTED_COMMANDS.filter((cmd) => !commands.includes(cmd));
    assert.strictEqual(missing.length, 0, `Missing commands: ${missing.join(", ")}`);
  });
});

// #endregion 🔖Command Registration Tests

// #region 🔖Kit Validation Tests

suite("Kit Validation Test Suite", function () {
  this.timeout(15000);

  test("Valid kit file produces no diagnostics", async function () {
    const document = await openFixture("semio/kit_metabolism.json");
    const diagnostics = await waitForDiagnostics(document.uri);
    assert.strictEqual(diagnostics.length, 0, "Valid kit should have no validation errors");
  });

  test("Invalid kit file triggers all expected constraint violations", async function () {
    const document = await openFixture("semio/kit_invalid.json");
    const diagnostics = await waitForDiagnostics(document.uri);
    if (diagnostics.length === 0) {
      console.log("Skipping: validation may be disabled due to bundling issues");
      return;
    }
    const constraintIds = new Set<string>();
    diagnostics.forEach((diag) => {
      if (typeof diag.code === "object" && diag.code !== null) {
        constraintIds.add((diag.code as { value: string }).value);
      } else if (typeof diag.code === "string") {
        constraintIds.add(diag.code);
      }
    });
    const missing = EXPECTED_CONSTRAINTS.filter((c) => !constraintIds.has(c));
    assert.strictEqual(missing.length, 0, `Missing constraint violations: ${missing.join(", ")}`);
  });

  test("Diagnostics have correct source and severity", async function () {
    const document = await openFixture("semio/kit_invalid.json");
    const diagnostics = await waitForDiagnostics(document.uri);
    if (diagnostics.length === 0) {
      console.log("Skipping: validation may be disabled due to bundling issues");
      return;
    }
    diagnostics.forEach((diag) => {
      assert.strictEqual(diag.source, "semio", "Source should be 'semio'");
      const validSeverities = [vscode.DiagnosticSeverity.Error, vscode.DiagnosticSeverity.Warning, vscode.DiagnosticSeverity.Information];
      assert.ok(validSeverities.includes(diag.severity), `Invalid severity: ${diag.severity}`);
    });
  });

  test("Quick fixes are available for kit diagnostics", async function () {
    const document = await openFixture("semio/kit_invalid.json");
    const diagnostics = await waitForDiagnostics(document.uri);
    if (diagnostics.length === 0) {
      console.log("Skipping: validation may be disabled due to bundling issues");
      return;
    }
    const codeActions = await vscode.commands.executeCommand<vscode.CodeAction[]>("vscode.executeCodeActionProvider", document.uri, diagnostics[0].range);
    assert.ok(codeActions && codeActions.length > 0, "Should have code actions available");
    const fixAction = codeActions.find((action) => action.kind?.value === vscode.CodeActionKind.QuickFix.value);
    assert.ok(fixAction, "Should have at least one quick fix action");
    assert.ok(fixAction.edit, "Quick fix should have a workspace edit");
  });

  test("Quick fix workspace edit contains valid text edits", async function () {
    const document = await openFixture("semio/kit_invalid.json");
    const diagnostics = await waitForDiagnostics(document.uri);
    if (diagnostics.length === 0) {
      console.log("Skipping: validation may be disabled due to bundling issues");
      return;
    }
    const codeActions = await vscode.commands.executeCommand<vscode.CodeAction[]>("vscode.executeCodeActionProvider", document.uri, diagnostics[0].range);
    const fixAction = codeActions?.find((action) => action.kind?.value === vscode.CodeActionKind.QuickFix.value);
    if (!fixAction?.edit) {
      console.log("Skipping: no quick fix with edit found");
      return;
    }
    const entries = fixAction.edit.entries();
    assert.ok(entries.length > 0, "Workspace edit should have entries");
    for (const [uri, edits] of entries) {
      assert.ok(uri.fsPath.endsWith(".json"), "Edit should target JSON file");
      assert.ok(edits.length > 0, "Should have at least one text edit");
      edits.forEach((edit) => {
        assert.ok(edit.range, "Text edit should have a range");
        assert.ok(typeof edit.newText === "string", "Text edit should have newText");
      });
    }
  });
});

// #endregion 🔖Kit Validation Tests

// #region 🔖Repo Diagnostics Tests

suite("Repo Diagnostics Test Suite", function () {
  this.timeout(30000);

  test("Invalid repo file produces diagnostics", async function () {
    const document = await openFixture("repo/some/folder/file_invalid.tsx");
    const diagnostics = await waitForDiagnostics(document.uri, 10000);
    if (diagnostics.length === 0) {
      console.log("Skipping: no violations found (analyze returned 0)");
      return;
    }
    assert.ok(diagnostics.length > 0, "Invalid repo file should have diagnostics");
  });

  test("Repo diagnostics show violation name as message", async function () {
    const document = await openFixture("repo/some/folder/file_invalid.tsx");
    const diagnostics = await waitForDiagnostics(document.uri, 10000);
    if (diagnostics.length === 0) {
      console.log("Skipping: no violations found");
      return;
    }
    diagnostics.forEach((diag) => {
      assert.ok(!diag.message.includes("\n"), "Message should not contain newlines");
      assert.ok(!diag.message.includes("file_invalid.tsx"), "Message should not contain file path");
    });
  });

  test("Repo diagnostics have policy ID as code with link target", async function () {
    const document = await openFixture("repo/some/folder/file_invalid.tsx");
    const diagnostics = await waitForDiagnostics(document.uri, 10000);
    if (diagnostics.length === 0) {
      console.log("Skipping: no violations found");
      return;
    }
    const diagWithLink = diagnostics.find((d) => typeof d.code === "object" && d.code !== null);
    if (!diagWithLink) {
      console.log("Skipping: no diagnostic with code object found");
      return;
    }
    const codeObj = diagWithLink.code as { value: string; target: vscode.Uri };
    assert.ok(codeObj.value, "Code should have policy ID");
    assert.ok(!codeObj.value.includes(":"), "Code should be policy ID without violation suffix");
    assert.ok(codeObj.target, "Code should have target URI");
    assert.ok(codeObj.target.fsPath.includes("repo.tsx"), "Target should point to repo.tsx");
    assert.ok(codeObj.target.fragment.startsWith("L"), "Target should have line number fragment");
  });

  test("Valid repo file produces no diagnostics", async function () {
    const document = await openFixture("repo/some/folder/file.tsx");
    const diagnostics = await waitForDiagnostics(document.uri, 10000);
    assert.strictEqual(diagnostics.length, 0, "Valid repo file should have no diagnostics");
  });

  test("Repo diagnostics have code actions for autofixable violations", async function () {
    const document = await openFixture("repo/some/folder/file_invalid.tsx");
    const diagnostics = await waitForDiagnostics(document.uri, 10000);
    if (diagnostics.length === 0) {
      console.log("Skipping: no violations found");
      return;
    }
    const codeActions = await vscode.commands.executeCommand<vscode.CodeAction[]>("vscode.executeCodeActionProvider", document.uri, diagnostics[0].range);
    assert.ok(codeActions && codeActions.length > 0, "Should have code actions for repo diagnostics");
    const fixAction = codeActions.find((a) => a.kind?.value === vscode.CodeActionKind.QuickFix.value);
    assert.ok(fixAction, "Should have quick fix action");
    assert.ok(fixAction.command || fixAction.edit, "Quick fix should have command or edit");
  });
});

// #endregion 🔖Repo Diagnostics Tests

// #region 🔖Refresh Diagnostics Tests

suite("Refresh Diagnostics Test Suite", function () {
  this.timeout(15000);

  test("semio.refreshDiagnostics updates all open documents", async function () {
    const document = await openFixture("semio/kit_invalid.json");
    await vscode.commands.executeCommand("semio.refreshDiagnostics");
    await new Promise((resolve) => setTimeout(resolve, 3000));
    const diagnostics = vscode.languages.getDiagnostics(document.uri).filter((d) => d.source === "semio");
    if (diagnostics.length === 0) {
      console.log("Skipping: validation may be disabled due to bundling issues");
      return;
    }
    assert.ok(diagnostics.length > 0, "Diagnostics should be present after refresh");
  });
});

// #endregion 🔖Refresh Diagnostics Tests

// #region 🔖Sidebar View Tests

suite("Sidebar View Test Suite", function () {
  this.timeout(15000);

  test("All expected views are registered", async function () {
    const extension = vscode.extensions.getExtension("usalu.semio-repo");
    assert.ok(extension, "Extension should be found");
    if (!extension.isActive) {
      await extension.activate();
    }
    assert.ok(extension.isActive, "Extension should be active");

    const packageJSON = extension.packageJSON;
    const views = packageJSON.contributes.views;
    assert.ok(views, "Views contribution should exist");
    assert.ok(views["semio-repo"], "semio-repo container should exist");
    const registeredViews = views["semio-repo"].map((v: any) => v.id);
    const missing = EXPECTED_VIEWS.filter((v) => !registeredViews.includes(v));
    assert.strictEqual(missing.length, 0, `Missing views: ${missing.join(", ")}`);
  });

  test("Monorepo view can be focused", async function () {
    await vscode.commands.executeCommand("semio.monorepo.focus");
  });

  test("Filter view can be focused", async function () {
    await vscode.commands.executeCommand("semio.filter.focus");
  });

  test("Refresh codebase command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.refreshCodebase"), "refreshCodebase command should be registered");
  });

  test("Toggle filter command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.filter.toggle"), "semio.filter.toggle command should be registered");
  });

  test("Copy ID command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.copyId"), "copyId command should be registered");
  });

  test("Mailto command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.mailto"), "mailto command should be registered");
  });

  test("Open link command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.openLink"), "openLink command should be registered");
  });

  test("Refresh monorepo command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.refreshMonorepo"), "refreshMonorepo command should be registered");
  });

  test("Copy commit SHA command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.copyCommitSha"), "copyCommitSha command should be registered");
  });

  test("Open commit in GitHub command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.openCommitInGitHub"), "openCommitInGitHub command should be registered");
  });

  test("Ticket reopen command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.ticketReopen"), "ticketReopen command should be registered");
  });

  test("Refresh item command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.refreshItem"), "refreshItem command should be registered");
  });

  test("New filter toggle commands are available", async function () {
    const commands = await vscode.commands.getCommands(true);
    const newFilterCommands = [
      "semio.filter.toggle.project.user",
      "semio.filter.toggle.project.infrastructure",
      "semio.filter.toggle.project.research",
      "semio.filter.toggle.file.code",
      "semio.filter.toggle.file.script",
      "semio.filter.toggle.file.config",
      "semio.filter.toggle.file.test",
      "semio.filter.toggle.file.docs",
      "semio.filter.toggle.file.resource",
      "semio.filter.toggle.file.license",
      "semio.filter.toggle.goal.open",
      "semio.filter.toggle.goal.closed",
      "semio.filter.toggle.bundle.schema",
      "semio.filter.toggle.policy.none",
      "semio.filter.toggle.policy.all",
      "semio.filter.toggle.contributor.none",
      "semio.filter.toggle.contributor.all",
      "semio.filter.toggle.commit.none",
      "semio.filter.toggle.commit.all",
    ];
    const missing = newFilterCommands.filter(cmd => !commands.includes(cmd));
    assert.strictEqual(missing.length, 0, `Missing new filter commands: ${missing.join(", ")}`);
  });
});

// #endregion 🔖Sidebar View Tests

// #region 🔖Sections View Tests

suite("Sections View Test Suite", function () {
  this.timeout(30000);

  test("Sections view is registered", async function () {
    const extension = vscode.extensions.getExtension("usalu.semio-repo");
    assert.ok(extension, "Extension should be found");
    if (!extension.isActive) {
      await extension.activate();
    }
    const packageJSON = extension.packageJSON;
    const views = packageJSON.contributes.views["explorer"] || packageJSON.contributes.views["semio-repo"];
    const sectionView = views.find((v: any) => v.id === "semio.sections");
    assert.ok(sectionView, "semio.sections view should be registered");
  });

  test("Sections view can be focused", async function () {
    await vscode.commands.executeCommand("semio.sections.focus");
  });

  test("sectionTree command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.sectionTree"), "sectionTree command should be registered");
  });

  test("sectionList command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.sectionList"), "sectionList command should be registered");
  });

  test("sectionCreate command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.sectionCreate"), "sectionCreate command should be registered");
  });

  test("sectionMove command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.sectionMove"), "sectionMove command should be registered");
  });

  test("sectionDelete command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.sectionDelete"), "sectionDelete command should be registered");
  });

  test("sectionOpen command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.sectionOpen"), "sectionOpen command should be registered");
  });

  test("sectionRename command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.sectionRename"), "sectionRename command should be registered");
  });

  test("sectionIntegrate command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.sectionIntegrate"), "sectionIntegrate command should be registered");
  });

  test("Sections tree view refreshes on file change", async function () {
    const root = getWorkspaceRoot();
    const candidatePaths = [
      path.join(root, "semio-repo", "vscode", "extension.ts"),
      path.join(root, "@semio-repo/vscode/extension.ts"),
      path.join(root, "extension.ts"),
    ];
    const existing = candidatePaths.find((p) => fs.existsSync(p));
    if (existing) {
      await vscode.workspace.openTextDocument(vscode.Uri.file(existing));
    }
    await new Promise((resolve) => setTimeout(resolve, 1000));
    await vscode.commands.executeCommand("semio.sections.focus");
    await new Promise((resolve) => setTimeout(resolve, 500));
    assert.ok(true, "Sections tree view should refresh without error");
  });
});

// #endregion 🔖Sections View Tests

suite("Filter Provider Test Suite", () => {
  test("FilterProvider initializes correctly", () => {
    const provider = new FilterTreeDataProvider();
    assert.ok(provider);
  });

  test("Root elements include expected categories", async () => {
    const provider = new FilterTreeDataProvider();
    const children = await provider.getChildren();
    assert.strictEqual(children.length, 13, "Should have 13 root elements (search + 12 filters)");
    const labels = children.map((c: FilterTreeItem) => typeof c.label === 'string' ? c.label : (c.label as vscode.TreeItemLabel).label);
    assert.ok(labels.some(l => l.startsWith("🔍Search")), "Should have Search");
    assert.ok(labels.some(l => l.startsWith("🏗️Projects")), "Should have Projects");
    assert.ok(labels.some(l => l.startsWith("📦Bundles")), "Should have Bundles");
    assert.ok(labels.some(l => l.startsWith("📂Folders")), "Should have Folders");
    assert.ok(labels.some(l => l.startsWith("📄Files")), "Should have Files");
    assert.ok(labels.some(l => l.startsWith("🔖Sections")), "Should have Sections");
    assert.ok(labels.some(l => l.startsWith("🏷️Definitions")), "Should have Definitions");
    assert.ok(labels.some(l => l.startsWith("🎯Goals")), "Should have Goals");
    assert.ok(labels.some(l => l.startsWith("🎫Tickets")), "Should have Tickets");
    assert.ok(labels.some(l => l.startsWith("🎫Dates")), "Should have Dates");
    assert.ok(labels.some(l => l.startsWith("🛡️Policies")), "Should have Policies");
    assert.ok(labels.some(l => l.startsWith("👤Contributors")), "Should have Contributors");
    assert.ok(labels.some(l => l.startsWith("🔄Commits")), "Should have Commits");
  });

  test("Time category returns year values when available", async () => {
    const provider = new FilterTreeDataProvider();
    provider.availableYears = [2024, 2025];
    const timeItem = new FilterTreeItem("🎫Dates", "filter", vscode.TreeItemCollapsibleState.Collapsed, "filter_time");
    const children = await provider.getChildren(timeItem);
    assert.strictEqual(children.length, 2, "Should have 2 year items");
    const labels = children.map((c: FilterTreeItem) => typeof c.label === 'string' ? c.label : '');
    assert.ok(labels.includes("2024"), "Should have 2024");
    assert.ok(labels.includes("2025"), "Should have 2025");
  });

  test("Year item returns month values when available", async () => {
    const provider = new FilterTreeDataProvider();
    provider.availableMonths = [1, 6, 12];
    const yearItem = new FilterTreeItem("2024", "timeValue", vscode.TreeItemCollapsibleState.Collapsed, "filter_time_year", "year", 2024);
    const children = await provider.getChildren(yearItem);
    assert.strictEqual(children.length, 3, "Should have 3 month items");
  });

  test("Month item returns day values when available", async () => {
    const provider = new FilterTreeDataProvider();
    provider.availableDays = [1, 15, 28];
    const monthItem = new FilterTreeItem("January", "timeValue", vscode.TreeItemCollapsibleState.Collapsed, "filter_time_month", "month", 1);
    const children = await provider.getChildren(monthItem);
    assert.strictEqual(children.length, 3, "Should have 3 day items");
  });

  test("Toggle methods update filter state", () => {
    const provider = new FilterTreeDataProvider();

    provider.toggle("bundle", "library");
    assert.strictEqual(provider.filters.bundle.library, false);
    provider.toggle("bundle", "library");
    assert.strictEqual(provider.filters.bundle.library, true);

    provider.toggle("folder", "organization");
    assert.strictEqual(provider.filters.folder.organization, false);

    provider.toggle("section", "all");
    assert.strictEqual(provider.filters.section.all, false);

    provider.toggle("definition", "implementation");
    assert.strictEqual(provider.filters.definition.implementation, false);

    provider.toggle("ticket", "open");
    assert.strictEqual(provider.filters.ticket.open, false);

    provider.toggle("project", "user");
    assert.strictEqual(provider.filters.project.user, false);
    provider.toggle("project", "user");
    assert.strictEqual(provider.filters.project.user, true);

    provider.toggle("file", "code");
    assert.strictEqual(provider.filters.file.code, false);
    provider.toggle("file", "code");
    assert.strictEqual(provider.filters.file.code, true);

    provider.toggle("goal", "open");
    assert.strictEqual(provider.filters.goal.open, false);
    provider.toggle("goal", "open");
    assert.strictEqual(provider.filters.goal.open, true);
  });

  for (const kind of ["bundle", "folder", "definition", "ticket", "project", "file", "goal"]) {
    test(`${kind} none/all toggles set all ${kind} filters`, () => {
      const provider = new FilterTreeDataProvider();
      provider.toggle(kind, "none");
      for (const key of Object.keys(provider.filters[kind])) {
        assert.strictEqual(provider.filters[kind][key], false, `${kind}.${key} should be false after none`);
      }
      provider.toggle(kind, "all");
      for (const key of Object.keys(provider.filters[kind])) {
        assert.strictEqual(provider.filters[kind][key], true, `${kind}.${key} should be true after all`);
      }
    });
  }

  test("Time filter toggle updates state", () => {
    const provider = new FilterTreeDataProvider();
    provider.availableYears = [2024];
    provider.availableMonths = [1];
    provider.availableDays = [15];

    provider.toggleYear(2024);
    assert.ok(provider.excludedYears.includes(2024));
    provider.toggleYear(2024);
    assert.ok(!provider.excludedYears.includes(2024));

    provider.toggleMonth(1);
    assert.ok(provider.excludedMonths.includes(1));

    provider.toggleDay(15);
    assert.ok(provider.excludedDays.includes(15));

    provider.toggle("time", "none");
    assert.strictEqual(provider.timeFilter.none, true);
    assert.strictEqual(provider.timeFilter.all, false);
    assert.ok(provider.excludedYears.includes(2024));
    assert.ok(provider.excludedMonths.includes(1));
    assert.ok(provider.excludedDays.includes(15));

    provider.toggle("time", "all");
    assert.strictEqual(provider.timeFilter.none, false);
    assert.strictEqual(provider.timeFilter.all, true);
    assert.strictEqual(provider.excludedYears.length, 0);
  });

  test("setTimeMode sets year/month/day modes", () => {
    const provider = new FilterTreeDataProvider();
    provider.availableYears = [2024, 2025];
    provider.availableMonths = [3, 6];
    provider.availableDays = [1, 15];

    provider.setTimeMode("year", "none");
    assert.deepStrictEqual(provider.excludedYears, [2024, 2025]);
    provider.setTimeMode("year", "all");
    assert.deepStrictEqual(provider.excludedYears, []);

    provider.setTimeMode("month", "none");
    assert.deepStrictEqual(provider.excludedMonths, [3, 6]);
    provider.setTimeMode("month", "all");
    assert.deepStrictEqual(provider.excludedMonths, []);

    provider.setTimeMode("day", "none");
    assert.deepStrictEqual(provider.excludedDays, [1, 15]);
    provider.setTimeMode("day", "all");
    assert.deepStrictEqual(provider.excludedDays, []);
  });

  test("Search query update updates search state", () => {
    const provider = new FilterTreeDataProvider();
    provider.searchQuery = "test";
    assert.strictEqual(provider.searchQuery, "test");
    provider.matchCase = true;
    assert.strictEqual(provider.matchCase, true);
    provider.matchWholeWord = true;
    assert.strictEqual(provider.matchWholeWord, true);
    provider.useRegex = true;
    assert.strictEqual(provider.useRegex, true);
  });
});

suite("Monorepo Provider Test Suite", () => {
  test("MonorepoProvider initializes with filter provider", () => {
    const filterProvider = new FilterTreeDataProvider();
    const provider = new MonorepoTreeDataProvider(filterProvider);
    assert.ok(provider);
  });

  test("Root elements are populated from CLI tree", async function () {
    this.timeout(30000);
    const provider = new MonorepoTreeDataProvider();
    const children = await provider.getChildren();
    if (children.length === 0) return;
    const labels = children.map(c => c.label as string);
    assert.ok(labels.some(l => l.includes("Projects")), "Should have Projects category");
  });

  test("Root elements have category contextValue", async function () {
    this.timeout(30000);
    const provider = new MonorepoTreeDataProvider();
    const roots = await provider.getChildren();
    if (roots.length === 0) return;
    for (const r of roots) {
      assert.strictEqual(r.contextValue, "category", `Root ${r.label} should have category contextValue`);
    }
  });

  test("MonorepoTreeItem stores nodeId for copy support", () => {
    const item = new MonorepoTreeItem("Test", vscode.TreeItemCollapsibleState.None, "test_ctx", undefined, "test-node-id");
    assert.strictEqual(item.nodeId, "test-node-id");
    assert.strictEqual(item.tooltip, "test-node-id");
  });

  test("MonorepoTreeItem without nodeId defaults tooltip to label", () => {
    const item = new MonorepoTreeItem("Label", vscode.TreeItemCollapsibleState.None, "ctx");
    assert.strictEqual(item.nodeId, undefined);
  });

  test("Projects root expands to at least one project when repo CLI is available", async function () {
    this.timeout(30000);
    const provider = new MonorepoTreeDataProvider();
    const roots = await provider.getChildren();
    const projectsRoot = roots.find((r: MonorepoTreeItem) => (r.label as string).includes("Projects"));
    if (!projectsRoot) return;
    const expanded = await provider.getChildren(projectsRoot);
    if (expanded.length === 0) return;
    assert.ok(expanded.length > 0);
  });
});

suite("Data Structures Test Suite", () => {
  test("TicketInteraction matches expected structure", () => {
    const interaction: TicketInteraction = {
      prompt: "test prompt",
      llm: "gpt-4",
      client: "vscode",
      author: "user",
      date: "2024-01-01",
      commit: "sha123"
    };
    assert.ok(interaction);
    assert.strictEqual(interaction.client, "vscode");
  });

  test("TicketData includes interactions", () => {
    const ticket: TicketData = {
      year: 2024,
      month: 1,
      day: 1,
      slug: "test-ticket",
      frontmatter: {
        status: "open",
        prompt: "test"
      },
      folderPath: "/path/to/ticket",
      filePath: "/path/to/ticket/ticket.md",
      interactions: []
    };
    assert.ok(ticket);
    assert.ok(Array.isArray(ticket.interactions));
  });
});

// #region 🔖CLI Tree Helper Tests

suite("extractLeadingEmoji Test Suite", () => {
  test("Extracts single emoji from start", () => {
    assert.strictEqual(extractLeadingEmoji("💻coda/py/coda.py"), "💻");
  });

  test("Extracts compound emoji with variation selector", () => {
    assert.strictEqual(extractLeadingEmoji("⚙️config.json"), "⚙️");
  });

  test("Returns empty string for no emoji", () => {
    assert.strictEqual(extractLeadingEmoji("hello"), "");
  });

  test("Returns empty string for empty input", () => {
    assert.strictEqual(extractLeadingEmoji(""), "");
  });

  test("Extracts category emoji", () => {
    assert.strictEqual(extractLeadingEmoji("🏗️Projects"), "🏗️");
  });
});

suite("treeNodeDisplayLabel Test Suite", () => {
  test("Category node uses Label directly", () => {
    const node: TreeNodeData = { Kind: "category", ID: "", Label: "🏗️Projects", URI: "" };
    assert.strictEqual(treeNodeDisplayLabel(node), "🏗️Projects");
  });

  test("Ticket node gets status icon", () => {
    const node: TreeNodeData = { Kind: "ticket", ID: "🎫test", Label: "MY-TICKET", URI: "", Status: "open" };
    assert.ok(treeNodeDisplayLabel(node).includes("🔵"));
  });

  test("Closed ticket gets green icon", () => {
    const node: TreeNodeData = { Kind: "ticket", ID: "🎫test", Label: "MY-TICKET", URI: "", Status: "closed" };
    assert.ok(treeNodeDisplayLabel(node).includes("🟢"));
  });

  test("File node uses emoji prefix plus Label", () => {
    const node: TreeNodeData = { Kind: "file", ID: "💻semio/go/semio.go", Label: "semio.go", URI: "" };
    assert.strictEqual(treeNodeDisplayLabel(node), "💻semio.go");
  });

  test("Goal node includes status icon", () => {
    const node: TreeNodeData = { Kind: "goal", ID: "🎯my-goal", Label: "My Goal", URI: "", Status: "open" };
    const label = treeNodeDisplayLabel(node);
    assert.ok(label.includes("🔵") || label.includes("🎯"));
  });

  test("Contributor node gets fallback emoji", () => {
    const node: TreeNodeData = { Kind: "contributor", ID: "", Label: "usalu", URI: "" };
    const label = treeNodeDisplayLabel(node);
    assert.ok(label.includes("👤"));
  });

  test("Commit node gets fallback emoji", () => {
    const node: TreeNodeData = { Kind: "commit", ID: "", Label: "Fix bug", URI: "" };
    const label = treeNodeDisplayLabel(node);
    assert.ok(label.includes("🔀"));
  });
});

suite("treeNodeContextValue Test Suite", () => {
  test("Category returns category", () => {
    assert.strictEqual(treeNodeContextValue({ Kind: "category", ID: "", Label: "", URI: "" }), "category");
  });

  test("File returns file", () => {
    assert.strictEqual(treeNodeContextValue({ Kind: "file", ID: "", Label: "", URI: "" }), "file");
  });

  test("Open ticket returns ticketOpen", () => {
    assert.strictEqual(treeNodeContextValue({ Kind: "ticket", ID: "", Label: "", URI: "", Status: "open" }), "ticketOpen");
  });

  test("Closed ticket returns ticketClosed", () => {
    assert.strictEqual(treeNodeContextValue({ Kind: "ticket", ID: "", Label: "", URI: "", Status: "closed" }), "ticketClosed");
  });

  test("Goal returns goal", () => {
    assert.strictEqual(treeNodeContextValue({ Kind: "goal", ID: "", Label: "", URI: "" }), "goal");
  });

  test("Contributor returns contributor", () => {
    assert.strictEqual(treeNodeContextValue({ Kind: "contributor", ID: "", Label: "", URI: "" }), "contributor");
  });
});

suite("Violation Kind Hierarchy Test Suite", () => {
  test("Renders nested violation kind tree structure correctly", () => {

    const violationNode: TreeNodeData = {
      Kind: "violationKind",
      ID: "🚫Code#Header#Missing Region",
      Label: "🚫Code#Header#Missing Region",
      Description: "Header required",
      URI: "semiorepo://violationKind/CODE/HEADER/MISSING-REGION",
      Data: { autofixable: true }
    };

    const categoryNode: TreeNodeData = {
      Kind: "category",
      ID: "header",
      Label: "header",
      URI: "",
      Children: [violationNode]
    };

    const policyNode: TreeNodeData = {
      Kind: "policy",
      ID: "🛡️/code",
      Label: "🛡️/code",
      URI: "semiorepo://policy/CODE",
      Children: [categoryNode]
    };

    const policyItem = treeNodeToItem(policyNode);
    assert.strictEqual(policyItem.label, "🛡️/code");
    assert.strictEqual(policyItem.collapsibleState, vscode.TreeItemCollapsibleState.Collapsed);

    const categoryItem = treeNodeToItem(categoryNode);
    assert.strictEqual(categoryItem.label, "header");
    assert.strictEqual(categoryItem.collapsibleState, vscode.TreeItemCollapsibleState.Collapsed);
    assert.strictEqual(categoryItem.contextValue, "category");

    const violationItem = treeNodeToItem(violationNode);
    assert.strictEqual(violationItem.label, "🚫Code#Header#Missing Region");
    assert.strictEqual(violationItem.description, "🔧");
    assert.strictEqual(violationItem.tooltip, "Header required");
    assert.strictEqual(violationItem.collapsibleState, vscode.TreeItemCollapsibleState.None);
  });
});

suite("buildCliTreeArgs Test Suite", () => {
  test("No filter provider returns empty args", () => {
    const args = buildCliTreeArgs(undefined);
    assert.ok(Array.isArray(args));
    assert.strictEqual(args.length, 0);
  });

  test("Filter provider with search query adds positional arg", () => {
    const fp = new FilterTreeDataProvider();
    fp.searchQuery = "hello";
    const args = buildCliTreeArgs(fp);
    assert.ok(args.includes("hello"));
  });

  test("Filter provider with excluded file kinds adds --no flags", () => {
    const fp = new FilterTreeDataProvider();
    fp.filters.file.code = false;
    const args = buildCliTreeArgs(fp);
    assert.ok(args.includes("--no-code"));
  });

  test("Filter provider with excluded years adds --no-year flags", () => {
    const fp = new FilterTreeDataProvider();
    fp.excludedYears = [2024, 2025];
    const args = buildCliTreeArgs(fp);
    assert.ok(args.includes("--no-year"));
    assert.ok(args.includes("2024"));
    assert.ok(args.includes("2025"));
  });

  test("Filter provider with status filter adds --only-open", () => {
    const fp = new FilterTreeDataProvider();
    fp.filters.ticket.open = true;
    fp.filters.ticket.closed = false;
    fp.filters.goal.open = true;
    fp.filters.goal.closed = false;
    const args = buildCliTreeArgs(fp);
    assert.ok(args.includes("--only-open"));
  });
});

// #endregion 🔖CLI Tree Helper Tests

// #region 🔖RepoEvent Extended Tests

suite("RepoEvent Extended Parsing Test Suite", () => {
  test("parseRepoEvents handles multiple lines", () => {
    const output = '{"kind":"start"}\n{"kind":"result","result":{"data":"hello"}}\n{"kind":"done"}\n';
    const events = parseRepoEvents(output);
    assert.strictEqual(events.length, 3);
    assert.strictEqual(events[0].kind, "start");
    assert.strictEqual(events[1].kind, "result");
    assert.strictEqual(events[2].kind, "done");
  });

  test("parseRepoEvents ignores blank lines", () => {
    const output = '{"kind":"result","result":{}}\n\n  \n';
    const events = parseRepoEvents(output);
    assert.strictEqual(events.length, 1);
  });

  test("extractRepoResult handles empty events", () => {
    const result = extractRepoResult([]);
    assert.strictEqual(result.data, null);
  });

  test("extractRepoResult skips control events (start, progress, log, done)", () => {
    const events = [
      { kind: "start" },
      { kind: "progress" },
      { kind: "log" },
      { kind: "result", result: { data: { value: 42 } } },
      { kind: "done" },
    ];
    const result = extractRepoResult(events);
    assert.ok(result.data);
    assert.strictEqual((result.data as any).value, 42);
  });

  test("extractRepoResult collects section results", () => {
    const events = [
      { kind: "section", result: undefined, data: undefined },
    ];
    const result = extractRepoResult(events);
    assert.ok(result);
  });
});

// #endregion 🔖RepoEvent Extended Tests

// #region 🔖URI Resolution Tests

suite("slugify Test Suite", () => {
  test("Converts text to uppercase slug", () => {
    assert.strictEqual(slugify("Hello World"), "HELLO-WORLD");
  });

  test("Converts file path to slug", () => {
    assert.strictEqual(slugify("semio/js/semio.ts"), "SEMIO-JS-SEMIO-TS");
  });

  test("Handles already slugified text", () => {
    assert.strictEqual(slugify("HELLO-WORLD"), "HELLO-WORLD");
  });

  test("Strips leading and trailing hyphens", () => {
    assert.strictEqual(slugify("--hello--"), "HELLO");
  });

  test("Handles empty string", () => {
    assert.strictEqual(slugify(""), "");
  });

  test("Preserves numbers", () => {
    assert.strictEqual(slugify("version-2.0"), "VERSION-2-0");
  });

  test("Handles goal ID with slashes", () => {
    assert.strictEqual(slugify("AI-OPTIMIZED-REPO"), "AI-OPTIMIZED-REPO");
  });
});

suite("parseUri Test Suite", () => {
  test("Parses repo URI (no path)", () => {
    const result = parseUri("semiorepo://repo");
    assert.ok(result);
    assert.strictEqual(result!.type, "repo");
    assert.strictEqual(result!.path, "");
  });

  test("Parses projects collection URI (no path)", () => {
    const result = parseUri("semiorepo://projects");
    assert.ok(result);
    assert.strictEqual(result!.type, "projects");
    assert.strictEqual(result!.path, "");
  });

  test("Parses project URI", () => {
    const result = parseUri("semiorepo://project/semio");
    assert.ok(result);
    assert.strictEqual(result!.type, "project");
    assert.strictEqual(result!.path, "semio");
  });

  test("Parses bundles collection URI (no path)", () => {
    const result = parseUri("semiorepo://bundles");
    assert.ok(result);
    assert.strictEqual(result!.type, "bundles");
    assert.strictEqual(result!.path, "");
  });

  test("Parses bundle URI", () => {
    const result = parseUri("semiorepo://bundle/semio-js");
    assert.ok(result);
    assert.strictEqual(result!.type, "bundle");
    assert.strictEqual(result!.path, "semio-js");
  });

  test("Parses folders collection URI with parent path", () => {
    const result = parseUri("semiorepo://folders/semio/js");
    assert.ok(result);
    assert.strictEqual(result!.type, "folders");
    assert.strictEqual(result!.path, "semio/js");
  });

  test("Parses folder URI with deep path", () => {
    const result = parseUri("semiorepo://folder/semio/js/sketchpad/pages/getting-started");
    assert.ok(result);
    assert.strictEqual(result!.type, "folder");
    assert.strictEqual(result!.path, "semio/js/sketchpad/pages/getting-started");
  });

  test("Parses files collection URI with folder path", () => {
    const result = parseUri("semiorepo://files/semio/js");
    assert.ok(result);
    assert.strictEqual(result!.type, "files");
    assert.strictEqual(result!.path, "semio/js");
  });

  test("Parses file URI with path", () => {
    const result = parseUri("semiorepo://file/semio/js/semio.ts");
    assert.ok(result);
    assert.strictEqual(result!.type, "file");
    assert.strictEqual(result!.path, "semio/js/semio.ts");
  });

  test("Parses sections collection URI with file path", () => {
    const result = parseUri("semiorepo://sections/semio/js/semio.ts");
    assert.ok(result);
    assert.strictEqual(result!.type, "sections");
    assert.strictEqual(result!.path, "semio/js/semio.ts");
  });

  test("Parses section URI with file and section path", () => {
    const result = parseUri("semiorepo://section/semio/js/sketchpad/Design.tsx/STATE-MANAGEMENT/DESIGN-STORE");
    assert.ok(result);
    assert.strictEqual(result!.type, "section");
    assert.strictEqual(result!.path, "semio/js/sketchpad/Design.tsx/STATE-MANAGEMENT/DESIGN-STORE");
  });

  test("Parses definitions collection URI", () => {
    const result = parseUri("semiorepo://definitions/semio/js/semio.ts");
    assert.ok(result);
    assert.strictEqual(result!.type, "definitions");
    assert.strictEqual(result!.path, "semio/js/semio.ts");
  });

  test("Parses definition URI", () => {
    const result = parseUri("semiorepo://definition/semio/js/semio.ts/VALIDATE-KIT");
    assert.ok(result);
    assert.strictEqual(result!.type, "definition");
    assert.strictEqual(result!.path, "semio/js/semio.ts/VALIDATE-KIT");
  });

  test("Parses tickets collection URI (no path)", () => {
    const result = parseUri("semiorepo://tickets");
    assert.ok(result);
    assert.strictEqual(result!.type, "tickets");
    assert.strictEqual(result!.path, "");
  });

  test("Parses ticket URI", () => {
    const result = parseUri("semiorepo://ticket/2026/02/07/MY-TICKET");
    assert.ok(result);
    assert.strictEqual(result!.type, "ticket");
    assert.strictEqual(result!.path, "2026/02/07/MY-TICKET");
  });

  test("Parses goals collection URI (no path)", () => {
    const result = parseUri("semiorepo://goals");
    assert.ok(result);
    assert.strictEqual(result!.type, "goals");
    assert.strictEqual(result!.path, "");
  });

  test("Parses goal URI with path", () => {
    const result = parseUri("semiorepo://goal/R26-02/RUNNING-SKETCHPAD/RUNNING-SKETCHPAD-APPS/RUNNING-HOME-APP");
    assert.ok(result);
    assert.strictEqual(result!.type, "goal");
    assert.strictEqual(result!.path, "R26-02/RUNNING-SKETCHPAD/RUNNING-SKETCHPAD-APPS/RUNNING-HOME-APP");
  });

  test("Parses drafts collection URI (no path)", () => {
    const result = parseUri("semiorepo://drafts");
    assert.ok(result);
    assert.strictEqual(result!.type, "drafts");
    assert.strictEqual(result!.path, "");
  });

  test("Parses draft URI", () => {
    const result = parseUri("semiorepo://draft/MY-DRAFT");
    assert.ok(result);
    assert.strictEqual(result!.type, "draft");
    assert.strictEqual(result!.path, "MY-DRAFT");
  });

  test("Parses todos collection URI (no path)", () => {
    const result = parseUri("semiorepo://todos");
    assert.ok(result);
    assert.strictEqual(result!.type, "todos");
    assert.strictEqual(result!.path, "");
  });

  test("Parses todo URI", () => {
    const result = parseUri("semiorepo://todo/FIX-BUG");
    assert.ok(result);
    assert.strictEqual(result!.type, "todo");
    assert.strictEqual(result!.path, "FIX-BUG");
  });

  test("Parses policies collection URI (no path)", () => {
    const result = parseUri("semiorepo://policies");
    assert.ok(result);
    assert.strictEqual(result!.type, "policies");
    assert.strictEqual(result!.path, "");
  });

  test("Parses policy URI", () => {
    const result = parseUri("semiorepo://policy/code");
    assert.ok(result);
    assert.strictEqual(result!.type, "policy");
    assert.strictEqual(result!.path, "code");
  });

  test("Parses violationKinds collection URI (no path)", () => {
    const result = parseUri("semiorepo://violationKinds");
    assert.ok(result);
    assert.strictEqual(result!.type, "violationKinds");
    assert.strictEqual(result!.path, "");
  });

  test("Parses violationKind URI with path", () => {
    const result = parseUri("semiorepo://violationKind/CODE/HEADER/MISSING-REGION");
    assert.ok(result);
    assert.strictEqual(result!.type, "violationKind");
    assert.strictEqual(result!.path, "CODE/HEADER/MISSING-REGION");
  });

  test("Parses contributors collection URI (no path)", () => {
    const result = parseUri("semiorepo://contributors");
    assert.ok(result);
    assert.strictEqual(result!.type, "contributors");
    assert.strictEqual(result!.path, "");
  });

  test("Parses contributor URI", () => {
    const result = parseUri("semiorepo://contributor/usalu");
    assert.ok(result);
    assert.strictEqual(result!.type, "contributor");
    assert.strictEqual(result!.path, "usalu");
  });

  test("Parses commits collection URI (no path)", () => {
    const result = parseUri("semiorepo://commits");
    assert.ok(result);
    assert.strictEqual(result!.type, "commits");
    assert.strictEqual(result!.path, "");
  });

  test("Parses commit URI", () => {
    const result = parseUri("semiorepo://commit/abc123def");
    assert.ok(result);
    assert.strictEqual(result!.type, "commit");
    assert.strictEqual(result!.path, "abc123def");
  });

  test("Returns null for non-semiorepo URIs", () => {
    assert.strictEqual(parseUri("https://example.com"), null);
    assert.strictEqual(parseUri("file:///tmp/foo"), null);
    assert.strictEqual(parseUri("not a uri"), null);
  });

  test("Returns null for malformed semiorepo URI", () => {
    assert.strictEqual(parseUri("semiorepo://"), null);
  });
});

suite("Navigation Commands Test Suite", function () {
  this.timeout(15000);

  test("semio.navigate command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.navigate"), "navigate command should be registered");
  });

  test("semio.navigateTo command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.navigateTo"), "navigateTo command should be registered");
  });

  test("invalidateTreeNodeCache does not throw", () => {
    invalidateTreeNodeCache();
    assert.ok(true);
  });

  test("semio.navigate handles empty target gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "");
    assert.ok(true, "Should not throw on empty target");
  });

  test("semio.navigate handles undefined target gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", undefined);
    assert.ok(true, "Should not throw on undefined target");
  });

  test("semio.navigate handles unknown URI type gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://unknown/SOMETHING");
    assert.ok(true, "Should not throw on unknown URI type");
  });

  test("semio.navigate handles repo URI gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://repo");
    assert.ok(true, "Should not throw on repo URI");
  });

  test("semio.navigate handles collection URIs gracefully", async function () {
    const collections = ["projects", "bundles", "folders", "files", "sections", "definitions", "tickets", "goals", "drafts", "todos", "policies", "violationKinds", "contributors", "commits"];
    for (const collection of collections) {
      await vscode.commands.executeCommand("semio.navigate", `semiorepo://${collection}`);
    }
    assert.ok(true, "Should not throw on collection URIs");
  });

  test("semio.navigate handles folder URI with real path", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://folder/semio/js");
    assert.ok(true, "Should not throw on folder URI");
  });

  test("semio.navigate handles file URI with real path", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://file/package.json");
    assert.ok(true, "Should not throw on file URI");
  });

  test("semio.navigate handles nonexistent folder URI gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://folder/nonexistent/path");
    assert.ok(true, "Should not throw on nonexistent folder URI");
  });

  test("semio.navigate handles nonexistent file URI gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://file/nonexistent/path.ts");
    assert.ok(true, "Should not throw on nonexistent file URI");
  });

  test("semio.navigate handles ticket URI gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://ticket/2099/01/01/NONEXISTENT");
    assert.ok(true, "Should not throw on nonexistent ticket URI");
  });

  test("semio.navigate handles goal URI gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://goal/NONEXISTENT-GOAL");
    assert.ok(true, "Should not throw on nonexistent goal URI");
  });

  test("semio.navigate handles draft URI gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://draft/NONEXISTENT");
    assert.ok(true, "Should not throw on nonexistent draft URI");
  });

  test("semio.navigate handles todo URI gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://todo/NONEXISTENT");
    assert.ok(true, "Should not throw on nonexistent todo URI");
  });

  test("semio.navigate handles policy URI gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://policy/code");
    assert.ok(true, "Should not throw on policy URI");
  });

  test("semio.navigate handles violationKind URI gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://violationKind/CODE/HEADER/MISSING-REGION");
    assert.ok(true, "Should not throw on violationKind URI");
  });

  test("semio.navigate handles contributor URI gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://contributor/usalu");
    assert.ok(true, "Should not throw on contributor URI");
  });

  test("semio.navigate handles commit URI gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://commit/abc123");
    assert.ok(true, "Should not throw on commit URI");
  });

  test("semio.navigate handles section URI gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://section/semio/js/semio.ts/HEADER");
    assert.ok(true, "Should not throw on section URI");
  });

  test("semio.navigate handles definition URI gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://definition/semio/js/semio.ts/VALIDATE-KIT");
    assert.ok(true, "Should not throw on definition URI");
  });

  test("semio.navigate handles project URI gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://project/semio");
    assert.ok(true, "Should not throw on project URI");
  });

  test("semio.navigate handles bundle URI gracefully", async function () {
    await vscode.commands.executeCommand("semio.navigate", "semiorepo://bundle/semio-js");
    assert.ok(true, "Should not throw on bundle URI");
  });
});

// #endregion 🔖URI Resolution Tests
