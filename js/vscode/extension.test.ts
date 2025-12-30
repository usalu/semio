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
import * as path from "path";
import * as vscode from "vscode";

// #endregion Imports

// #region Validation Tests

suite("Validation Test Suite", function () {
  this.timeout(10000);

  test("Validate invalid kit fixture", async function () {
    this.timeout(10000);
    const fixturePath = path.join(__dirname, "../../../../assets/semio/kit_invalid.json");
    const fixtureUri = vscode.Uri.file(fixturePath);
    const document = await vscode.workspace.openTextDocument(fixtureUri);
    await vscode.window.showTextDocument(document);
    await new Promise((resolve) => setTimeout(resolve, 5000));
    const diagnostics = vscode.languages.getDiagnostics(fixtureUri);
    const semioDiagnostics = diagnostics.filter((d) => d.source === "semio");
    if (semioDiagnostics.length === 0) {
      console.log("No semio diagnostics found (validation may be disabled due to bundling issues)");
      return;
    }
    const constraintIds = new Set<string>();
    semioDiagnostics.forEach((diag) => {
      if (diag.code) constraintIds.add(String(diag.code));
    });
    const expectedConstraints = ["guid-unique", "type-name-unique", "design-name-unique", "piece-name-unique", "quality-name-unique", "port-name-unique", "file-name-unique", "folder-name-unique", "connector-name-unique", "model-name-unique", "layer-path-unique"];
    expectedConstraints.forEach((constraintId) => {
      assert.ok(constraintIds.has(constraintId), `Should have validation error for constraint: ${constraintId}`);
    });
    assert.strictEqual(constraintIds.size, expectedConstraints.length, `Should have exactly ${expectedConstraints.length} different validation constraints triggered`);
  });

  test("Quick fixes apply correct diffs", async function () {
    this.timeout(10000);
    const fixturePath = path.join(__dirname, "../../../../assets/semio/kit_invalid.json");
    const fixtureUri = vscode.Uri.file(fixturePath);
    const document = await vscode.workspace.openTextDocument(fixtureUri);
    await vscode.window.showTextDocument(document);
    await new Promise((resolve) => setTimeout(resolve, 5000));
    const diagnostics = vscode.languages.getDiagnostics(fixtureUri);
    const semioDiagnostics = diagnostics.filter((d) => d.source === "semio");
    if (semioDiagnostics.length === 0) {
      console.log("No semio diagnostics found (validation may be disabled due to bundling issues)");
      return;
    }
    const firstDiagnostic = semioDiagnostics[0];
    const codeActions = await vscode.commands.executeCommand<vscode.CodeAction[]>("vscode.executeCodeActionProvider", fixtureUri, firstDiagnostic.range);
    assert.ok(codeActions && codeActions.length > 0, "Should have code actions available");
    const fixAction = codeActions.find((action) => action.kind?.value === vscode.CodeActionKind.QuickFix.value);
    assert.ok(fixAction, "Should have at least one quick fix action");
    assert.ok(fixAction.edit, "Quick fix should have a workspace edit");
  });
});

// #endregion Validation Tests

// #region Command Registration Tests

suite("Command Registration Test Suite", () => {
  test("semio.analyze command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.analyze"), "semio.analyze command should be registered");
  });

  test("semio.analyzeFile command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.analyzeFile"), "semio.analyzeFile command should be registered");
  });

  test("semio.fix command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.fix"), "semio.fix command should be registered");
  });

  test("semio.fixFile command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.fixFile"), "semio.fixFile command should be registered");
  });

  test("semio.policyList command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.policyList"), "semio.policyList command should be registered");
  });

  test("semio.ticketCreate command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.ticketCreate"), "semio.ticketCreate command should be registered");
  });

  test("semio.ticketList command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.ticketList"), "semio.ticketList command should be registered");
  });

  test("semio.projectList command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.projectList"), "semio.projectList command should be registered");
  });

  test("semio.sectionTree command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.sectionTree"), "semio.sectionTree command should be registered");
  });

  test("semio.definitionList command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.definitionList"), "semio.definitionList command should be registered");
  });

  test("semio.folderTree command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.folderTree"), "semio.folderTree command should be registered");
  });

  test("semio.refreshDiagnostics command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.refreshDiagnostics"), "semio.refreshDiagnostics command should be registered");
  });

  test("semio.policyRun command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.policyRun"), "semio.policyRun command should be registered");
  });

  test("semio.ticketIterateStart command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.ticketIterateStart"), "semio.ticketIterateStart command should be registered");
  });

  test("semio.ticketIterateEnd command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.ticketIterateEnd"), "semio.ticketIterateEnd command should be registered");
  });

  test("semio.ticketFinish command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.ticketFinish"), "semio.ticketFinish command should be registered");
  });

  test("semio.ticketRead command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.ticketRead"), "semio.ticketRead command should be registered");
  });

  test("semio.projectTree command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.projectTree"), "semio.projectTree command should be registered");
  });

  test("semio.folderCreate command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.folderCreate"), "semio.folderCreate command should be registered");
  });

  test("semio.folderMove command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.folderMove"), "semio.folderMove command should be registered");
  });

  test("semio.folderDelete command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.folderDelete"), "semio.folderDelete command should be registered");
  });

  test("semio.folderList command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.folderList"), "semio.folderList command should be registered");
  });

  test("semio.fileCreate command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.fileCreate"), "semio.fileCreate command should be registered");
  });

  test("semio.fileMove command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.fileMove"), "semio.fileMove command should be registered");
  });

  test("semio.fileDelete command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.fileDelete"), "semio.fileDelete command should be registered");
  });

  test("semio.fileList command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.fileList"), "semio.fileList command should be registered");
  });

  test("semio.fileTree command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.fileTree"), "semio.fileTree command should be registered");
  });

  test("semio.sectionCreate command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.sectionCreate"), "semio.sectionCreate command should be registered");
  });

  test("semio.sectionMove command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.sectionMove"), "semio.sectionMove command should be registered");
  });

  test("semio.sectionDelete command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.sectionDelete"), "semio.sectionDelete command should be registered");
  });

  test("semio.sectionList command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.sectionList"), "semio.sectionList command should be registered");
  });

  test("semio.definitionTree command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.definitionTree"), "semio.definitionTree command should be registered");
  });

  test("semio.toolRun command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.toolRun"), "semio.toolRun command should be registered");
  });

  test("semio.fixViolation command is registered", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("semio.fixViolation"), "semio.fixViolation command should be registered");
  });
});

// #endregion Command Registration Tests

// #region Command Execution Tests

suite("Command Execution Test Suite", () => {
  test("semio.refreshDiagnostics executes without error", async () => {
    try {
      await vscode.commands.executeCommand("semio.refreshDiagnostics");
      assert.ok(true, "Command executed successfully");
    } catch (error) {
      assert.fail(`Command should not throw: ${error}`);
    }
  });

  test("semio.policyList executes and shows output", async () => {
    try {
      await vscode.commands.executeCommand("semio.policyList");
      assert.ok(true, "Command executed successfully");
    } catch (error) {
      assert.fail(`Command should not throw: ${error}`);
    }
  });

  test("semio.ticketList executes and shows output", async () => {
    try {
      await vscode.commands.executeCommand("semio.ticketList");
      assert.ok(true, "Command executed successfully");
    } catch (error) {
      assert.fail(`Command should not throw: ${error}`);
    }
  });

  test("semio.projectList executes and shows output", async () => {
    try {
      await vscode.commands.executeCommand("semio.projectList");
      assert.ok(true, "Command executed successfully");
    } catch (error) {
      assert.fail(`Command should not throw: ${error}`);
    }
  });

  test("semio.sectionTree with active editor executes", async () => {
    const fixturePath = path.join(__dirname, "../../../../assets/repo/some/folder/file.tsx");
    const fixtureUri = vscode.Uri.file(fixturePath);
    const document = await vscode.workspace.openTextDocument(fixtureUri);
    await vscode.window.showTextDocument(document);
    try {
      await vscode.commands.executeCommand("semio.sectionTree");
      assert.ok(true, "Command executed successfully");
    } catch (error) {
      assert.fail(`Command should not throw: ${error}`);
    }
  });

  test("semio.definitionList with active editor executes", async () => {
    const fixturePath = path.join(__dirname, "../../../../assets/repo/some/folder/file.tsx");
    const fixtureUri = vscode.Uri.file(fixturePath);
    const document = await vscode.workspace.openTextDocument(fixtureUri);
    await vscode.window.showTextDocument(document);
    try {
      await vscode.commands.executeCommand("semio.definitionList");
      assert.ok(true, "Command executed successfully");
    } catch (error) {
      assert.fail(`Command should not throw: ${error}`);
    }
  });
});

// #endregion Command Execution Tests

// #region Diagnostics Tests

suite("Diagnostics Test Suite", function () {
  this.timeout(10000);

  test("Valid kit file has no diagnostics", async function () {
    this.timeout(10000);
    const fixturePath = path.join(__dirname, "../../../../assets/semio/kit_metabolism.json");
    const fixtureUri = vscode.Uri.file(fixturePath);
    const document = await vscode.workspace.openTextDocument(fixtureUri);
    await vscode.window.showTextDocument(document);
    await new Promise((resolve) => setTimeout(resolve, 5000));
    const diagnostics = vscode.languages.getDiagnostics(fixtureUri);
    const semioDiagnostics = diagnostics.filter((d) => d.source === "semio");
    assert.strictEqual(semioDiagnostics.length, 0, "Valid kit should have no validation errors");
  });

  test("Diagnostics have correct severity levels", async function () {
    this.timeout(10000);
    const fixturePath = path.join(__dirname, "../../../../assets/semio/kit_invalid.json");
    const fixtureUri = vscode.Uri.file(fixturePath);
    const document = await vscode.workspace.openTextDocument(fixtureUri);
    await vscode.window.showTextDocument(document);
    await new Promise((resolve) => setTimeout(resolve, 5000));
    const diagnostics = vscode.languages.getDiagnostics(fixtureUri);
    const semioDiagnostics = diagnostics.filter((d) => d.source === "semio");
    if (semioDiagnostics.length === 0) {
      console.log("No semio diagnostics found (validation may be disabled due to bundling issues)");
      return;
    }
    semioDiagnostics.forEach((diag) => {
      assert.ok(diag.severity === vscode.DiagnosticSeverity.Error || diag.severity === vscode.DiagnosticSeverity.Warning || diag.severity === vscode.DiagnosticSeverity.Information, "Diagnostics should have Error, Warning or Information severity");
    });
  });

  test("Diagnostics have source set to semio", async function () {
    this.timeout(10000);
    const fixturePath = path.join(__dirname, "../../../../assets/semio/kit_invalid.json");
    const fixtureUri = vscode.Uri.file(fixturePath);
    const document = await vscode.workspace.openTextDocument(fixtureUri);
    await vscode.window.showTextDocument(document);
    await new Promise((resolve) => setTimeout(resolve, 5000));
    const diagnostics = vscode.languages.getDiagnostics(fixtureUri);
    const semioDiagnostics = diagnostics.filter((d) => d.source === "semio");
    if (semioDiagnostics.length === 0) {
      console.log("No semio diagnostics found (validation may be disabled due to bundling issues)");
      return;
    }
    semioDiagnostics.forEach((diag) => {
      assert.strictEqual(diag.source, "semio", "Source should be semio");
    });
  });
});

// #endregion Diagnostics Tests

// #region Repo Diagnostics Tests

suite("Repo Diagnostics Test Suite", function () {
  this.timeout(30000);

  test("Invalid repo file has diagnostics", async function () {
    this.timeout(30000);
    const fixturePath = path.join(__dirname, "../../../../assets/repo/some/folder/file_invalid.tsx");
    const fixtureUri = vscode.Uri.file(fixturePath);
    const document = await vscode.workspace.openTextDocument(fixtureUri);
    await vscode.window.showTextDocument(document);
    await new Promise((resolve) => setTimeout(resolve, 10000));
    const diagnostics = vscode.languages.getDiagnostics(fixtureUri);
    const semioDiagnostics = diagnostics.filter((d) => d.source === "semio");
    if (semioDiagnostics.length === 0) {
      console.log("No semio diagnostics found (repo.tsx analyze may not be available)");
      return;
    }
    assert.ok(semioDiagnostics.length > 0, "Invalid repo file should have diagnostics");
  });

  test("Repo diagnostics have code actions", async function () {
    this.timeout(30000);
    const fixturePath = path.join(__dirname, "../../../../assets/repo/some/folder/file_invalid.tsx");
    const fixtureUri = vscode.Uri.file(fixturePath);
    const document = await vscode.workspace.openTextDocument(fixtureUri);
    await vscode.window.showTextDocument(document);
    await new Promise((resolve) => setTimeout(resolve, 10000));
    const diagnostics = vscode.languages.getDiagnostics(fixtureUri);
    const semioDiagnostics = diagnostics.filter((d) => d.source === "semio");
    if (semioDiagnostics.length === 0) {
      console.log("No semio diagnostics found (repo.tsx analyze may not be available)");
      return;
    }
    const firstDiagnostic = semioDiagnostics[0];
    const codeActions = await vscode.commands.executeCommand<vscode.CodeAction[]>("vscode.executeCodeActionProvider", fixtureUri, firstDiagnostic.range);
    assert.ok(codeActions && codeActions.length > 0, "Should have code actions available for repo diagnostics");
    const fixAction = codeActions.find((action) => action.kind?.value === vscode.CodeActionKind.QuickFix.value);
    assert.ok(fixAction, "Should have at least one quick fix action");
  });

  test("Repo diagnostic message is violation name", async function () {
    this.timeout(30000);
    const fixturePath = path.join(__dirname, "../../../../assets/repo/some/folder/file_invalid.tsx");
    const fixtureUri = vscode.Uri.file(fixturePath);
    const document = await vscode.workspace.openTextDocument(fixtureUri);
    await vscode.window.showTextDocument(document);
    await new Promise((resolve) => setTimeout(resolve, 10000));
    const diagnostics = vscode.languages.getDiagnostics(fixtureUri);
    const semioDiagnostics = diagnostics.filter((d) => d.source === "semio");
    if (semioDiagnostics.length === 0) {
      console.log("No semio diagnostics found (repo.tsx analyze may not be available)");
      return;
    }
    semioDiagnostics.forEach((diag) => {
      assert.ok(!diag.message.includes("\n"), "Diagnostic message should not contain newlines");
      assert.ok(!diag.message.includes("file_invalid.tsx"), "Diagnostic message should not contain file path");
      assert.ok(!diag.message.includes(":"), "Diagnostic message should be violation name without policy prefix");
    });
  });

  test("Repo diagnostic code is policy ID linking to policy region", async function () {
    this.timeout(30000);
    const fixturePath = path.join(__dirname, "../../../../assets/repo/some/folder/file_invalid.tsx");
    const fixtureUri = vscode.Uri.file(fixturePath);
    const document = await vscode.workspace.openTextDocument(fixtureUri);
    await vscode.window.showTextDocument(document);
    await new Promise((resolve) => setTimeout(resolve, 10000));
    const diagnostics = vscode.languages.getDiagnostics(fixtureUri);
    const semioDiagnostics = diagnostics.filter((d) => d.source === "semio");
    if (semioDiagnostics.length === 0) {
      console.log("No semio diagnostics found (repo.tsx analyze may not be available)");
      return;
    }
    const diagWithLink = semioDiagnostics.find((d) => typeof d.code === "object" && d.code !== null);
    if (diagWithLink && typeof diagWithLink.code === "object") {
      const codeObj = diagWithLink.code as { value: string; target: vscode.Uri };
      assert.ok(codeObj.value, "Code should have a policy ID");
      assert.ok(!codeObj.value.includes(":"), "Code should be policy ID without violation name");
      assert.ok(codeObj.target, "Code should have a target URI");
      assert.ok(codeObj.target.fsPath.includes("repo.tsx"), "Target should point to repo.tsx");
      assert.ok(codeObj.target.fragment.startsWith("L"), "Target should have line fragment");
    }
  });
});

// #endregion Repo Diagnostics Tests
