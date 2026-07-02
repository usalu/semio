export * from "./internal.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { writerPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for writer. */
export function buildWriterProgramDefinition(): PlatformDefinition {
	const app = writerPlayAppDefinition;
	return {
		id: "writer",
		name: "Writer",
		apiVersion: "1",
		apps: [{ id: "writer", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("writer document", () => {
		it("round-trips json", () => {
			const doc = createWriterDocument({ id: "jack", languageId: "jack", text: "MATCH (a:Piece) RETURN a.name" });
			const parsed = parseWriterDocumentJson(writerDocumentToJson(doc));
			expect(parsed.text).toBe(doc.text);
			expect(parsed.languageId).toBe("jack");
		});
	});

	describe("createWriterAppVcsHandler", () => {
		it("materializes inline writer documents", () => {
			const doc = createWriterDocument({ id: "t", languageId: "jack", text: "RETURN 1" });
			const projection = createWriterAppVcsHandler().materializeProjection({ inline: writerDocumentToJson(doc) });
			expect(projection.text).toBe("RETURN 1");
		});
	});

	describe("lsp offsets", () => {
		it("maps offset to position and back", () => {
			const text = "line one\nline two";
			const pos = offsetToPosition(text, 9);
			expect(pos.line).toBe(1);
			expect(positionToOffset(text, pos)).toBe(9);
		});
	});

	describe("applyTextEdits", () => {
		it("replaces range", () => {
			const out = applyTextEdits("abc def", [{ range: rangeFromOffsets("abc def", 4, 7), newText: "xyz" }]);
			expect(out).toBe("abc xyz");
		});
	});

	describe("grammar", () => {
		it("highlights jack keywords", () => {
			const tokens = tokenizeWithGrammar("MATCH (a:Piece)", grammarForLanguage("jack")!);
			expect(tokens.some((t) => t.class === "keyword")).toBe(true);
		});

		it("does not emit overlapping keyword and ident spans", () => {
			const tokens = tokenizeWithGrammar("MATCH (a:Piece)", grammarForLanguage("jack")!);
			const matchTokens = tokens.filter((t) => t.start === 0 && t.end === 5);
			expect(matchTokens).toHaveLength(1);
			expect(matchTokens[0]?.class).toBe("keyword");
		});

		it("builds jack selectable composite spans", () => {
			const text = "MATCH (a1:Piece) RETURN a1.name";
			const grammar = grammarForLanguage("jack")!;
			const tokens = tokenizeWithGrammar(text, grammar);
			const spans = selectableSpansForJack(text, tokens);
			expect(spans.some((s) => s.kind === "varLabel" && s.start === 7 && s.end === 15)).toBe(true);
			expect(spans.some((s) => s.kind === "propertyAccess" && s.start === 24 && s.end === 31)).toBe(true);
			expect(spans.some((s) => s.kind === "atomic" && s.start === 7 && s.end === 9)).toBe(true);
		});
	});

	describe("jack ast", () => {
		it("parses match return query with edge pattern", () => {
			const text = "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = 'core'\nRETURN a.name, b.name";
			const root = parseJackAst(text);
			expect(root.kind).toBe("query");
			expect(root.children.some((c) => c.kind === "match")).toBe(true);
			expect(root.children.some((c) => c.kind === "where")).toBe(true);
			expect(root.children.some((c) => c.kind === "return")).toBe(true);
			const match = root.children.find((c) => c.kind === "match");
			const pattern = match?.children[0];
			expect(pattern?.children.some((c) => c.kind === "edge")).toBe(true);
		});

		it("maps offset and selection to ast nodes", () => {
			const text = "MATCH (a:Piece) RETURN a.name";
			const root = parseJackAst(text);
			const atLabel = findDeepestJackAstNodeAt(root, 10);
			expect(atLabel?.kind).toBe("label");
			const selected = jackAstNodeForSelection(root, 7, 14);
			expect(selected?.kind).toBe("patternNode");
		});
	});

	describe("jack newline insertion", () => {
		const query = "MATCH (a:Piece) RETURN a.name";

		it("allows newline after keywords", () => {
			expect(jackNewlineAllowedAt(query, "MATCH".length)).toBe(true);
			expect(jackNewlineAllowedAt(query, query.indexOf("RETURN") + "RETURN".length)).toBe(true);
		});

		it("allows newline after closing pattern paren", () => {
			expect(jackNewlineAllowedAt(query, query.indexOf(")") + 1)).toBe(true);
		});

		it("disallows newline inside tokens", () => {
			expect(jackNewlineAllowedAt(query, 2)).toBe(false);
			expect(jackNewlineAllowedAt(query, query.indexOf("Piece") + 2)).toBe(false);
		});

		it("disallows newline before property access", () => {
			const dot = query.indexOf(".");
			expect(jackNewlineAllowedAt(query, dot)).toBe(false);
		});

		it("disallows newline between colon and label", () => {
			const colon = query.indexOf(":");
			expect(jackNewlineAllowedAt(query, colon + 1)).toBe(false);
		});

		it("allows newline for non-jack languages via writerNewlineAllowedAt", () => {
			expect(writerNewlineAllowedAt("hello world", "plaintext", 5)).toBe(true);
		});
	});

	describe("jack editor placeholders", () => {
		it("shows expr after AND near caret", () => {
			const text = "WHERE a.name = 'x' AND ";
			const placeholders = jackEditorPlaceholders(text, text.length);
			expect(placeholders.some((p) => p.label === "expr")).toBe(true);
		});

		it("shows label after colon", () => {
			const text = "MATCH (a:";
			const placeholders = jackEditorPlaceholders(text, text.length);
			expect(placeholders.some((p) => p.label === "Label")).toBe(true);
		});
	});

	describe("jack symbols", () => {
		const query = "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = 'core'\nRETURN a.name, b.name";

		it("finds all variable occurrences for bound name", () => {
			const occ = jackVariableOccurrences(query, "a");
			expect(occ.map((o) => query.slice(o.start, o.end))).toEqual(["a", "a", "a"]);
		});

		it("resolves variable symbol at reference offset", () => {
			const symbol = jackSymbolAtOffset(query, query.indexOf("a.name"));
			expect(symbol?.kind).toBe("variable");
			expect(symbol?.occurrences.length).toBe(3);
		});

		it("does not treat node kinds as variables", () => {
			const symbol = jackSymbolAtOffset(query, query.indexOf("Piece"));
			expect(symbol?.kind).toBe("nodeKind");
			expect(symbol?.occurrences).toHaveLength(1);
		});

		it("renames all variable occurrences", () => {
			const occ = jackVariableOccurrences(query, "a");
			const renamed = applyJackRename(query, occ, "nodeA");
			expect(renamed.text).toContain("nodeA.name");
			expect(renamed.text.match(/nodeA/g)?.length).toBe(3);
			expect(renamed.occurrences).toHaveLength(3);
		});
	});

	describe("lsp client round-trip", () => {
		it("correlates initialize and diagnostics notification", async () => {
			const inbound: LspMessage[] = [];
			const handlers = new Set<(message: LspMessage) => void>();
			const transport: LspTransport = {
				send(message) {
					inbound.push(message);
					if (isJsonRpcRequest(message) && message.method === "initialize" && typeof message.id === "number") {
						queueMicrotask(() => {
							for (const handler of handlers) {
								handler({ jsonrpc: "2.0", id: message.id, result: { capabilities: {} } });
							}
						});
					}
					if (isJsonRpcRequest(message) && message.method === "textDocument/didOpen") {
						queueMicrotask(() => {
							for (const handler of handlers) {
								handler({
									jsonrpc: "2.0",
									method: "textDocument/publishDiagnostics",
									params: {
										diagnostics: [
											{
												range: { start: { line: 0, character: 0 }, end: { line: 0, character: 5 } },
												severity: 1,
												message: "syntax",
											},
										],
									},
								});
							}
						});
					}
				},
				onMessage(handler) {
					handlers.add(handler);
				},
				dispose() {
					handlers.clear();
				},
			};
			const client = new LspClient(transport, { formatting: true });
			const seen: string[] = [];
			client.subscribeDiagnostics((items) => {
				seen.push(...items.map((item) => item.message));
			});
			await client.initialize("jack", "writer://");
			await client.openDocument({ uri: "writer://jack", languageId: "jack", version: 1, text: "RETURN a" });
			await new Promise((resolve) => setTimeout(resolve, 0));
			expect(inbound.some((message) => isJsonRpcRequest(message) && message.method === "textDocument/didOpen")).toBe(true);
			expect(seen).toContain("syntax");
			client.dispose();
		});
	});
}
// #endregion 🧪Tests
