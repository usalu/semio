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

import {
	applyKitDiff,
	deserializeKit,
	Kit,
	SemioDomainLocation,
	SemioKitFix,
	SemioValidationIssue,
	serializeKit,
	validateSemioKit,
} from "@semio/js/semio";
import * as jsonc from "jsonc-parser";
import * as vscode from "vscode";

const SEMIO_KIT_LANGUAGE = "json";
const DIAGNOSTIC_SOURCE = "semio";

export function activate(context: vscode.ExtensionContext) {
	console.log('Semio validation extension activated');

	const diagnosticCollection = vscode.languages.createDiagnosticCollection(DIAGNOSTIC_SOURCE);
	context.subscriptions.push(diagnosticCollection);

	const validateDocument = (document: vscode.TextDocument) => {
		if (!isSemioKitDocument(document)) {
			return;
		}

		try {
			const text = document.getText();
			const kit = deserializeKit(text);
			const result = validateSemioKit(kit);
			const diagnostics = result.issues.map((issue) => issueToDiagnostic(document, issue));
			diagnosticCollection.set(document.uri, diagnostics);
		} catch (error) {
			console.error("Failed to validate Semio kit:", error);
			diagnosticCollection.delete(document.uri);
		}
	};

	context.subscriptions.push(
		vscode.workspace.onDidOpenTextDocument(validateDocument),
		vscode.workspace.onDidChangeTextDocument((e) => validateDocument(e.document)),
		vscode.workspace.onDidCloseTextDocument((doc) => diagnosticCollection.delete(doc.uri))
	);

	vscode.workspace.textDocuments.forEach(validateDocument);

	context.subscriptions.push(
		vscode.languages.registerCodeActionsProvider(
			{ language: SEMIO_KIT_LANGUAGE },
			new SemioCodeActionProvider(),
			{ providedCodeActionKinds: [vscode.CodeActionKind.QuickFix] }
		)
	);
}

export function deactivate() { }

function isSemioKitDocument(document: vscode.TextDocument): boolean {
	if (document.languageId !== SEMIO_KIT_LANGUAGE) {
		return false;
	}

	const basename = document.uri.path.split("/").pop()?.toLowerCase() || "";
	return basename.startsWith("kit_") || basename.includes("_kit") || basename === "kit.json";
}

function issueToDiagnostic(document: vscode.TextDocument, issue: SemioValidationIssue): vscode.Diagnostic {
	const range = locationToRange(document, issue.location);
	const severity = issue.severity === "error" ? vscode.DiagnosticSeverity.Error : vscode.DiagnosticSeverity.Warning;

	const diagnostic = new vscode.Diagnostic(range, issue.message, severity);
	diagnostic.source = DIAGNOSTIC_SOURCE;
	diagnostic.code = issue.ruleId;

	if (issue.relatedGuids && issue.relatedGuids.length > 1) {
		diagnostic.relatedInformation = issue.relatedGuids.slice(1).map((guid) => {
			const relatedRange = findGuidRange(document, guid);
			return new vscode.DiagnosticRelatedInformation(
				new vscode.Location(document.uri, relatedRange),
				`Related entity: ${guid}`
			);
		});
	}

	return diagnostic;
}

function locationToRange(document: vscode.TextDocument, location: SemioDomainLocation): vscode.Range {
	if (!location.entityGuid) {
		return new vscode.Range(0, 0, 0, 0);
	}

	const text = document.getText();
	const tree = jsonc.parseTree(text);

	if (!tree) {
		return new vscode.Range(0, 0, 0, 0);
	}

	const entityNode = findEntityNode(tree, location);
	if (!entityNode) {
		return new vscode.Range(0, 0, 0, 0);
	}

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
	if (!arrayName) {
		return undefined;
	}

	const arrayNode = jsonc.findNodeAtLocation(tree, [arrayName]);
	if (!arrayNode || arrayNode.type !== "array") {
		return undefined;
	}

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

	if (!tree) {
		return new vscode.Range(0, 0, 0, 0);
	}

	const node = findNodeByGuid(tree, guid);
	if (!node) {
		return new vscode.Range(0, 0, 0, 0);
	}

	const startPos = document.positionAt(node.offset);
	const endPos = document.positionAt(node.offset + node.length);
	return new vscode.Range(startPos, endPos);
}

function findNodeByGuid(node: jsonc.Node, guid: string): jsonc.Node | undefined {
	if (node.type === "object") {
		const guidNode = jsonc.findNodeAtLocation(node, ["guid"]);
		if (guidNode?.type === "string" && guidNode.value === guid) {
			return node;
		}
	}

	if (node.type === "array" || node.type === "object") {
		for (const child of node.children || []) {
			const result = findNodeByGuid(child, guid);
			if (result) {
				return result;
			}
		}
	}

	return undefined;
}

class SemioCodeActionProvider implements vscode.CodeActionProvider {
	provideCodeActions(
		document: vscode.TextDocument,
		range: vscode.Range | vscode.Selection,
		context: vscode.CodeActionContext,
		token: vscode.CancellationToken
	): vscode.CodeAction[] | undefined {
		const semiosDiagnostics = context.diagnostics.filter((d) => d.source === DIAGNOSTIC_SOURCE);
		if (semiosDiagnostics.length === 0) {
			return undefined;
		}

		const actions: vscode.CodeAction[] = [];

		for (const diagnostic of semiosDiagnostics) {
			try {
				const text = document.getText();
				const kit = deserializeKit(text);
				const result = validateSemioKit(kit);

				const issue = result.issues.find((i) => i.message === diagnostic.message && i.ruleId === diagnostic.code);
				if (!issue) {
					continue;
				}

				for (const fix of issue.fixes) {
					const action = createCodeAction(document, diagnostic, fix, kit);
					if (action) {
						actions.push(action);
					}
				}
			} catch (error) {
				console.error("Failed to generate code actions:", error);
			}
		}

		return actions;
	}
}

function createCodeAction(
	document: vscode.TextDocument,
	diagnostic: vscode.Diagnostic,
	fix: SemioKitFix,
	kit: Kit
): vscode.CodeAction | undefined {
	try {
		const fixedKit = applyKitDiff(kit, fix.diff);
		const fixedJson = serializeKit(fixedKit);

		const action = new vscode.CodeAction(fix.title, vscode.CodeActionKind.QuickFix);
		action.diagnostics = [diagnostic];
		action.isPreferred = true;

		const edit = new vscode.WorkspaceEdit();
		const fullRange = new vscode.Range(
			document.positionAt(0),
			document.positionAt(document.getText().length)
		);
		edit.replace(document.uri, fullRange, fixedJson);
		action.edit = edit;

		return action;
	} catch (error) {
		console.error("Failed to create code action:", error);
		return undefined;
	}
}
