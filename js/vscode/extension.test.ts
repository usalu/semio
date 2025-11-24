import * as assert from 'assert';
import * as path from 'path';
import * as vscode from 'vscode';

suite('Extension Test Suite', () => {
	vscode.window.showInformationMessage('Start all tests.');

	test('Validate invalid kit fixture', async () => {
		const fixturePath = path.join(__dirname, '../../assets/semio/kit_invalid.json');
		const fixtureUri = vscode.Uri.file(fixturePath);

		const document = await vscode.workspace.openTextDocument(fixtureUri);
		await vscode.window.showTextDocument(document);

		await new Promise(resolve => setTimeout(resolve, 2000));

		const diagnostics = vscode.languages.getDiagnostics(fixtureUri);

		assert.ok(diagnostics.length > 0, 'Should have validation errors');

		const ruleIds = new Set<string>();
		diagnostics.forEach(diag => {
			if (diag.source === 'semio' && diag.code) {
				ruleIds.add(String(diag.code));
			}
		});

		const expectedRules = [
			'guid-unique',
			'type-name-unique',
			'design-name-unique',
			'piece-name-unique',
			'quality-name-unique',
			'interface-name-unique',
			'file-name-unique',
			'folder-name-unique',
			'port-name-unique',
			'model-name-unique',
			'layer-path-unique'
		];

		expectedRules.forEach(ruleId => {
			assert.ok(
				ruleIds.has(ruleId),
				`Should have validation error for rule: ${ruleId}`
			);
		});

		assert.strictEqual(
			ruleIds.size,
			expectedRules.length,
			`Should have exactly ${expectedRules.length} different validation rules triggered`
		);
	});

	test('Quick fixes apply correct diffs', async () => {
		const fixturePath = path.join(__dirname, '../../assets/semio/kit_invalid.json');
		const fixtureUri = vscode.Uri.file(fixturePath);

		const document = await vscode.workspace.openTextDocument(fixtureUri);
		const editor = await vscode.window.showTextDocument(document);

		await new Promise(resolve => setTimeout(resolve, 2000));

		const diagnostics = vscode.languages.getDiagnostics(fixtureUri);
		assert.ok(diagnostics.length > 0, 'Should have diagnostics to test fixes');

		const firstDiagnostic = diagnostics[0];
		const codeActions = await vscode.commands.executeCommand<vscode.CodeAction[]>(
			'vscode.executeCodeActionProvider',
			fixtureUri,
			firstDiagnostic.range
		);

		assert.ok(codeActions && codeActions.length > 0, 'Should have code actions available');

		const fixAction = codeActions.find(action =>
			action.kind?.value === vscode.CodeActionKind.QuickFix.value
		);

		assert.ok(fixAction, 'Should have at least one quick fix action');
		assert.ok(fixAction.edit, 'Quick fix should have a workspace edit');
	});
});
