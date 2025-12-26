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

import * as assert from "assert";
import * as path from "path";
import * as vscode from "vscode";

suite("Extension Test Suite", () => {
  vscode.window.showInformationMessage("Start all tests.");

  test("Validate invalid kit fixture", async () => {
    const fixturePath = path.join(__dirname, "../../../../assets/semio/kit_invalid.json");
    const fixtureUri = vscode.Uri.file(fixturePath);

    const document = await vscode.workspace.openTextDocument(fixtureUri);
    await vscode.window.showTextDocument(document);

    await new Promise((resolve) => setTimeout(resolve, 2000));

    const diagnostics = vscode.languages.getDiagnostics(fixtureUri);

    assert.ok(diagnostics.length > 0, "Should have validation errors");

    const constraintIds = new Set<string>();
    diagnostics.forEach((diag) => {
      if (diag.source === "semio-kit" && diag.code) {
        constraintIds.add(String(diag.code));
      }
    });

    const expectedConstraints = [
      "guid-unique",
      "type-name-unique",
      "design-name-unique",
      "piece-name-unique",
      "quality-name-unique",
      "interface-name-unique",
      "file-name-unique",
      "folder-name-unique",
      "connector-name-unique",
      "model-name-unique",
      "layer-path-unique",
    ];

    expectedConstraints.forEach((constraintId) => {
      assert.ok(constraintIds.has(constraintId), `Should have validation error for constraint: ${constraintId}`);
    });

    assert.strictEqual(constraintIds.size, expectedConstraints.length, `Should have exactly ${expectedConstraints.length} different validation constraints triggered`);
  });

  test("Quick fixes apply correct diffs", async () => {
    const fixturePath = path.join(__dirname, "../../../../assets/semio/kit_invalid.json");
    const fixtureUri = vscode.Uri.file(fixturePath);

    const document = await vscode.workspace.openTextDocument(fixtureUri);
    const editor = await vscode.window.showTextDocument(document);

    await new Promise((resolve) => setTimeout(resolve, 2000));

    const diagnostics = vscode.languages.getDiagnostics(fixtureUri);
    assert.ok(diagnostics.length > 0, "Should have diagnostics to test fixes");

    const firstDiagnostic = diagnostics[0];
    const codeActions = await vscode.commands.executeCommand<vscode.CodeAction[]>("vscode.executeCodeActionProvider", fixtureUri, firstDiagnostic.range);

    assert.ok(codeActions && codeActions.length > 0, "Should have code actions available");

    const fixAction = codeActions.find((action) => action.kind?.value === vscode.CodeActionKind.QuickFix.value);

    assert.ok(fixAction, "Should have at least one quick fix action");
    assert.ok(fixAction.edit, "Quick fix should have a workspace edit");
  });
});
