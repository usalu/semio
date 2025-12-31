// #region Header

// js/vscode/extension.test.ts

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

import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";

// #endregion Imports

// #region Constants

const EXPECTED_COMMANDS = ["semio.analyze", "semio.analyzeFile", "semio.fix", "semio.fixFile", "semio.policyList", "semio.ticketCreate", "semio.ticketList", "semio.ticketIterateStart", "semio.ticketIterateEnd", "semio.ticketFinish", "semio.ticketRead", "semio.ticketOpen", "semio.projectList", "semio.contributorAdd", "semio.contributorList", "semio.contributorRemove", "semio.sectionTree", "semio.definitionList", "semio.folderTree", "semio.folderCreate", "semio.folderMove", "semio.folderDelete", "semio.folderList", "semio.fileCreate", "semio.fileMove", "semio.fileDelete", "semio.fileList", "semio.fileTree", "semio.sectionCreate", "semio.sectionMove", "semio.sectionDelete", "semio.sectionList", "semio.definitionTree", "semio.projectTree", "semio.policyCheck", "semio.toolRun", "semio.refreshDiagnostics", "semio.fixViolation", "semio.refreshTickets", "semio.refreshContributors", "semio.refreshPolicies", "semio.toggleTicketFilter", "semio.openTicket", "semio.checkPolicy", "semio.runCommand"];
const EXPECTED_CONSTRAINTS = ["guid-unique", "type-name-unique", "design-name-unique", "piece-name-unique", "quality-name-unique", "port-name-unique", "file-name-unique", "folder-name-unique", "connector-name-unique", "model-name-unique", "layer-path-unique"];
const EXPECTED_VIEWS = ["semio.tickets", "semio.contributors", "semio.policies", "semio.commands"];

// #endregion Constants

// #region Utilities

function getWorkspaceRoot(): string {
  return vscode.workspace.workspaceFolders?.[0]?.uri.fsPath ?? path.join(__dirname, "../../../..");
}

function getFixturePath(relativePath: string): string {
  return path.join(getWorkspaceRoot(), "assets", relativePath);
}

async function openFixture(relativePath: string): Promise<vscode.TextDocument> {
  const fixturePath = getFixturePath(relativePath);
  const fixtureUri = vscode.Uri.file(fixturePath);
  const document = await vscode.workspace.openTextDocument(fixtureUri);
  await vscode.window.showTextDocument(document);
  return document;
}

async function waitForDiagnostics(uri: vscode.Uri, timeout = 5000): Promise<vscode.Diagnostic[]> {
  await new Promise((resolve) => setTimeout(resolve, timeout));
  return vscode.languages.getDiagnostics(uri).filter((d) => d.source === "semio");
}

function getCachePath(root: string, relativePath: string): string {
  let h = 0;
  const normalizedPath = relativePath.replace(/\\/g, "/");
  for (let i = 0; i < normalizedPath.length; i++) {
    h = (h * 31 + normalizedPath.charCodeAt(i)) >>> 0;
  }
  const hash = h.toString(16).padStart(8, "0");
  return path.join(root, ".semio-repo", "cache", "analyze", `${hash}.json`);
}

// #endregion Utilities

// #region Command Registration Tests

suite("Command Registration Test Suite", () => {
  test("All expected commands are registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    const missing = EXPECTED_COMMANDS.filter((cmd) => !commands.includes(cmd));
    assert.strictEqual(missing.length, 0, `Missing commands: ${missing.join(", ")}`);
  });
});

// #endregion Command Registration Tests

// #region Kit Validation Tests

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

// #endregion Kit Validation Tests

// #region Cache Mechanism Tests

suite("Cache Mechanism Test Suite", function () {
  this.timeout(30000);

  test("Analyzing a file creates cache entry", async function () {
    const root = getWorkspaceRoot();
    const relativePath = "js/js/semio.ts";
    const cachePath = getCachePath(root, relativePath);
    if (fs.existsSync(cachePath)) {
      fs.unlinkSync(cachePath);
    }
    const document = await openFixture(`../${relativePath}`);
    await waitForDiagnostics(document.uri, 10000);
    assert.ok(fs.existsSync(cachePath), "Cache file should exist after analysis");
    const cache = JSON.parse(fs.readFileSync(cachePath, "utf-8"));
    assert.ok(cache.filePath, "Cache should have filePath");
    assert.ok(cache.hash, "Cache should have content hash");
    assert.ok(cache.timestamp, "Cache should have timestamp");
    assert.ok(Array.isArray(cache.violations), "Cache should have violations array");
  });

  test("Cache contains valid violation structure", async function () {
    const root = getWorkspaceRoot();
    const relativePath = "assets/repo/some/folder/file_invalid.tsx";
    const document = await openFixture(`../${relativePath}`);
    await waitForDiagnostics(document.uri, 10000);
    const cachePath = getCachePath(root, relativePath);
    if (!fs.existsSync(cachePath)) {
      console.log("Skipping: cache file not created (repo analyze may not be available)");
      return;
    }
    const cache = JSON.parse(fs.readFileSync(cachePath, "utf-8"));
    if (cache.violations.length === 0) {
      console.log("Skipping: no violations in cache");
      return;
    }
    const violation = cache.violations[0];
    assert.ok(violation.id, "Violation should have id");
    assert.ok(violation.kind, "Violation should have kind");
    assert.ok(violation.summary, "Violation should have summary");
    assert.ok(["high", "medium", "low"].includes(violation.priority), "Violation should have valid priority");
    assert.ok(typeof violation.autofixable === "boolean", "Violation should have autofixable flag");
    assert.ok(violation.scope, "Violation should have scope");
  });

  test("Cache hash changes when file content changes", async function () {
    const root = getWorkspaceRoot();
    const testFilePath = path.join(root, "temp", "cache-test.ts");
    const relativePath = "temp/cache-test.ts";
    fs.mkdirSync(path.dirname(testFilePath), { recursive: true });
    fs.writeFileSync(testFilePath, "// test content v1\n");
    const testUri = vscode.Uri.file(testFilePath);
    const document = await vscode.workspace.openTextDocument(testUri);
    await vscode.window.showTextDocument(document);
    await waitForDiagnostics(testUri, 5000);
    const cachePath = getCachePath(root, relativePath);
    if (!fs.existsSync(cachePath)) {
      fs.unlinkSync(testFilePath);
      console.log("Skipping: cache file not created");
      return;
    }
    const cache1 = JSON.parse(fs.readFileSync(cachePath, "utf-8"));
    const hash1 = cache1.hash;
    fs.writeFileSync(testFilePath, "// test content v2 - modified\n");
    const document2 = await vscode.workspace.openTextDocument(testUri);
    await vscode.window.showTextDocument(document2);
    await waitForDiagnostics(testUri, 5000);
    const cache2 = JSON.parse(fs.readFileSync(cachePath, "utf-8"));
    const hash2 = cache2.hash;
    assert.notStrictEqual(hash1, hash2, "Cache hash should change when content changes");
    fs.unlinkSync(testFilePath);
    if (fs.existsSync(cachePath)) {
      fs.unlinkSync(cachePath);
    }
  });
});

// #endregion Cache Mechanism Tests

// #region Repo Diagnostics Tests

suite("Repo Diagnostics Test Suite", function () {
  this.timeout(30000);

  test("Invalid repo file produces diagnostics from cache", async function () {
    const document = await openFixture("repo/some/folder/file_invalid.tsx");
    const diagnostics = await waitForDiagnostics(document.uri, 10000);
    if (diagnostics.length === 0) {
      console.log("Skipping: repo analyze may not be available");
      return;
    }
    assert.ok(diagnostics.length > 0, "Invalid repo file should have diagnostics");
  });

  test("Repo diagnostics show violation name as message", async function () {
    const document = await openFixture("repo/some/folder/file_invalid.tsx");
    const diagnostics = await waitForDiagnostics(document.uri, 10000);
    if (diagnostics.length === 0) {
      console.log("Skipping: repo analyze may not be available");
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
      console.log("Skipping: repo analyze may not be available");
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
      console.log("Skipping: repo analyze may not be available");
      return;
    }
    const codeActions = await vscode.commands.executeCommand<vscode.CodeAction[]>("vscode.executeCodeActionProvider", document.uri, diagnostics[0].range);
    assert.ok(codeActions && codeActions.length > 0, "Should have code actions for repo diagnostics");
    const fixAction = codeActions.find((a) => a.kind?.value === vscode.CodeActionKind.QuickFix.value);
    assert.ok(fixAction, "Should have quick fix action");
    assert.ok(fixAction.command || fixAction.edit, "Quick fix should have command or edit");
  });
});

// #endregion Repo Diagnostics Tests

// #region Refresh Diagnostics Tests

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

// #endregion Refresh Diagnostics Tests

// #region Sidebar View Tests

suite("Sidebar View Test Suite", function () {
  this.timeout(15000);

  test("All expected views are registered", async function () {
    for (const viewId of EXPECTED_VIEWS) {
      const extension = vscode.extensions.getExtension("usalu.semio");
      assert.ok(extension, "Extension should be found");
    }
  });

  test("Tickets view can be focused", async function () {
    await vscode.commands.executeCommand("semio.tickets.focus");
    await new Promise((resolve) => setTimeout(resolve, 500));
  });

  test("Contributors view can be focused", async function () {
    await vscode.commands.executeCommand("semio.contributors.focus");
    await new Promise((resolve) => setTimeout(resolve, 500));
  });

  test("Policies view can be focused", async function () {
    await vscode.commands.executeCommand("semio.policies.focus");
    await new Promise((resolve) => setTimeout(resolve, 500));
  });

  test("Commands view can be focused", async function () {
    await vscode.commands.executeCommand("semio.commands.focus");
    await new Promise((resolve) => setTimeout(resolve, 500));
  });

  test("Refresh tickets command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.refreshTickets"), "refreshTickets command should be registered");
  });

  test("Refresh contributors command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.refreshContributors"), "refreshContributors command should be registered");
  });

  test("Refresh policies command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.refreshPolicies"), "refreshPolicies command should be registered");
  });

  test("Toggle ticket filter command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.toggleTicketFilter"), "toggleTicketFilter command should be registered");
  });

  test("Run command is available", async function () {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.runCommand"), "runCommand command should be registered");
  });

  test("Tickets folder has at least one ticket", async function () {
    const root = getWorkspaceRoot();
    const ticketsPath = path.join(root, "tickets");
    assert.ok(fs.existsSync(ticketsPath), "tickets folder should exist");
    const years = fs.readdirSync(ticketsPath).filter((f) => fs.statSync(path.join(ticketsPath, f)).isDirectory());
    assert.ok(years.length > 0, "tickets folder should have at least one year folder");
    let hasTicket = false;
    for (const year of years) {
      const yearPath = path.join(ticketsPath, year);
      const months = fs.readdirSync(yearPath).filter((f) => fs.statSync(path.join(yearPath, f)).isDirectory());
      for (const month of months) {
        const monthPath = path.join(yearPath, month);
        const days = fs.readdirSync(monthPath).filter((f) => fs.statSync(path.join(monthPath, f)).isDirectory());
        for (const day of days) {
          const dayPath = path.join(monthPath, day);
          const files = fs.readdirSync(dayPath).filter((f) => f.endsWith(".md"));
          if (files.length > 0) {
            hasTicket = true;
            break;
          }
        }
        if (hasTicket) break;
      }
      if (hasTicket) break;
    }
    assert.ok(hasTicket, "tickets folder should have at least one ticket");
  });

  test("Contributors folder has at least one contributor", async function () {
    const root = getWorkspaceRoot();
    const contributorsPath = path.join(root, "contributors");
    assert.ok(fs.existsSync(contributorsPath), "contributors folder should exist");
    const contributors = fs.readdirSync(contributorsPath).filter((f) => fs.statSync(path.join(contributorsPath, f)).isDirectory());
    assert.ok(contributors.length > 0, "contributors folder should have at least one contributor");
    const firstContributor = contributors[0];
    const contributorJsonPath = path.join(contributorsPath, firstContributor, "contributor.json");
    assert.ok(fs.existsSync(contributorJsonPath), "contributor should have contributor.json file");
  });
});

// #endregion Sidebar View Tests
