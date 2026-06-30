// #region 🧲Header
/// <reference types="vitest/importMeta" />
/** @emoji ✍️ `@semio-tech/writer-core` — writer documents, LSP client, and grammar registry. */
// #endregion 🧲Header

// #region 📐WriterDocument
export interface WriterCamera {
	readonly x: number;
	readonly y: number;
	readonly zoom: number;
}

export interface WriterDocumentV1 {
	readonly schema: "writer.document/v1";
	readonly id: string;
	readonly languageId: string;
	readonly uri: string;
	readonly text: string;
	readonly camera: WriterCamera;
}

export const WRITER_DOCUMENT_SCHEMA = "writer.document/v1" as const;

export const WRITER_DEFAULT_CAMERA: WriterCamera = { x: 0, y: 0, zoom: 1 };

export function createWriterDocument(input: {
	readonly id: string;
	readonly languageId: string;
	readonly uri?: string;
	readonly text?: string;
	readonly camera?: WriterCamera;
}): WriterDocumentV1 {
	return {
		schema: WRITER_DOCUMENT_SCHEMA,
		id: input.id,
		languageId: input.languageId,
		uri: input.uri ?? `writer://${input.id}`,
		text: input.text ?? "",
		camera: input.camera ?? WRITER_DEFAULT_CAMERA,
	};
}

export function parseWriterDocumentJson(json: string): WriterDocumentV1 {
	const value = JSON.parse(json) as Partial<WriterDocumentV1>;
	if (value.schema !== WRITER_DOCUMENT_SCHEMA) {
		throw new Error(`expected schema ${WRITER_DOCUMENT_SCHEMA}`);
	}
	if (!value.id || !value.languageId) {
		throw new Error("writer document requires id and languageId");
	}
	return createWriterDocument({
		id: value.id,
		languageId: value.languageId,
		uri: value.uri,
		text: value.text,
		camera: value.camera,
	});
}

export function writerDocumentToJson(doc: WriterDocumentV1): string {
	return JSON.stringify(doc);
}
// #endregion 📐WriterDocument

// #region 🔖Lsp
export type LspId = number | string;

export interface JsonRpcRequest {
	readonly jsonrpc: "2.0";
	readonly id?: LspId;
	readonly method: string;
	readonly params?: unknown;
}

export interface JsonRpcResponse {
	readonly jsonrpc: "2.0";
	readonly id: LspId | null;
	readonly result?: unknown;
	readonly error?: { readonly code: number; readonly message: string; readonly data?: unknown };
}

export interface JsonRpcNotification {
	readonly jsonrpc: "2.0";
	readonly method: string;
	readonly params?: unknown;
}

export type LspMessage = JsonRpcRequest | JsonRpcResponse | JsonRpcNotification;

export interface LspPosition {
	readonly line: number;
	readonly character: number;
}

export interface LspRange {
	readonly start: LspPosition;
	readonly end: LspPosition;
}

export interface LspTextDocumentItem {
	readonly uri: string;
	readonly languageId: string;
	readonly version: number;
	readonly text: string;
}

export interface LspCompletionItem {
	readonly label: string;
	readonly kind?: number;
	readonly detail?: string;
	readonly insertText?: string;
}

export interface LspHover {
	readonly contents: string | { readonly value: string };
	readonly range?: LspRange;
}

export interface LspDiagnostic {
	readonly range: LspRange;
	readonly severity?: number;
	readonly code?: string;
	readonly message: string;
}

export interface LspTextEdit {
	readonly range: LspRange;
	readonly newText: string;
}

export interface LspSemanticTokenSpan {
	readonly start: number;
	readonly end: number;
	readonly class: string;
}

export interface LspTransport {
	send(message: LspMessage): void;
	onMessage(handler: (message: LspMessage) => void): void;
	dispose(): void;
}

export interface LanguageServer {
	handle(message: LspMessage): readonly LspMessage[];
}

export function isJsonRpcRequest(message: LspMessage): message is JsonRpcRequest {
	return "method" in message && !("result" in message) && !("error" in message);
}

export function isJsonRpcResponse(message: LspMessage): message is JsonRpcResponse {
	return "jsonrpc" in message && ("result" in message || "error" in message);
}

export function isJsonRpcNotification(message: LspMessage): message is JsonRpcNotification {
	return "method" in message && !("id" in message);
}

export function offsetToPosition(text: string, offset: number): LspPosition {
	const clamped = Math.max(0, Math.min(offset, text.length));
	let line = 0;
	let last = 0;
	for (let i = 0; i < clamped; i += 1) {
		if (text.charCodeAt(i) === 10) {
			line += 1;
			last = i + 1;
		}
	}
	return { line, character: clamped - last };
}

export function positionToOffset(text: string, position: LspPosition): number {
	const lines = text.split("\n");
	const line = Math.max(0, Math.min(position.line, lines.length - 1));
	let offset = 0;
	for (let i = 0; i < line; i += 1) offset += lines[i]!.length + 1;
	return offset + Math.max(0, Math.min(position.character, lines[line]?.length ?? 0));
}

export function rangeFromOffsets(text: string, start: number, end: number): LspRange {
	return { start: offsetToPosition(text, start), end: offsetToPosition(text, end) };
}

export function runLanguageServerLoop(server: LanguageServer, transport: LspTransport): () => void {
	const onMessage = (message: LspMessage) => {
		for (const reply of server.handle(message)) {
			transport.send(reply);
		}
	};
	transport.onMessage(onMessage);
	return () => transport.dispose();
}

type LspClientListener<T> = (value: T) => void;

export class LspClient {
	private nextId = 1;
	private readonly pending = new Map<number, { resolve: (value: unknown) => void; reject: (err: Error) => void }>();
	private readonly diagnosticsListeners = new Set<LspClientListener<readonly LspDiagnostic[]>>();
	private readonly semanticListeners = new Set<LspClientListener<readonly LspSemanticTokenSpan[]>>();
	private document: LspTextDocumentItem | null = null;
	private diagnostics: readonly LspDiagnostic[] = [];
	private semanticTokens: readonly LspSemanticTokenSpan[] = [];
	private initialized = false;
	private initPromise: Promise<void> | null = null;

	constructor(
		private readonly transport: LspTransport,
		private readonly serverCapabilities?: { readonly formatting?: boolean },
	) {
		transport.onMessage((message) => {
			if (isJsonRpcResponse(message) && message.id != null && typeof message.id === "number") {
				const entry = this.pending.get(message.id);
				if (entry) {
					this.pending.delete(message.id);
					if (message.error) entry.reject(new Error(message.error.message));
					else entry.resolve(message.result);
				}
			}
			if (isJsonRpcNotification(message)) {
				if (message.method === "textDocument/publishDiagnostics") {
					const params = message.params as { diagnostics?: LspDiagnostic[] };
					this.diagnostics = params.diagnostics ?? [];
					for (const listener of this.diagnosticsListeners) listener(this.diagnostics);
				}
				if (message.method === "writer/semanticTokens") {
					const params = message.params as { tokens?: LspSemanticTokenSpan[] };
					this.semanticTokens = params.tokens ?? [];
					for (const listener of this.semanticListeners) listener(this.semanticTokens);
				}
			}
		});
	}

	async initialize(languageId: string, rootUri: string): Promise<void> {
		if (this.initialized) return;
		if (!this.initPromise) {
			this.initPromise = this.request("initialize", {
				processId: null,
				rootUri,
				capabilities: {},
				clientInfo: { name: "writer", version: "1" },
			}).then(() => {
				this.transport.send({ jsonrpc: "2.0", method: "initialized", params: {} });
				this.initialized = true;
			});
		}
		await this.initPromise;
		if (!this.document) {
			await this.openDocument({ uri: `${rootUri}/untitled`, languageId, version: 1, text: "" });
		}
	}

	async openDocument(doc: LspTextDocumentItem): Promise<void> {
		this.document = doc;
		await this.notify("textDocument/didOpen", { textDocument: doc });
	}

	async changeDocument(text: string, version: number): Promise<void> {
		if (!this.document) return;
		this.document = { ...this.document, text, version };
		await this.notify("textDocument/didChange", {
			textDocument: { uri: this.document.uri, version },
			contentChanges: [{ text }],
		});
	}

	async completion(position: LspPosition): Promise<readonly LspCompletionItem[]> {
		if (!this.document) return [];
		const result = (await this.request("textDocument/completion", {
			textDocument: { uri: this.document.uri },
			position,
		})) as { items?: LspCompletionItem[] } | LspCompletionItem[] | null;
		if (Array.isArray(result)) return result;
		return result?.items ?? [];
	}

	async hover(position: LspPosition): Promise<LspHover | null> {
		if (!this.document) return null;
		return (await this.request("textDocument/hover", {
			textDocument: { uri: this.document.uri },
			position,
		})) as LspHover | null;
	}

	async formatDocument(): Promise<readonly LspTextEdit[]> {
		if (!this.document || !this.serverCapabilities?.formatting) return [];
		const result = (await this.request("textDocument/formatting", {
			textDocument: { uri: this.document.uri },
			options: { tabSize: 2, insertSpaces: true },
		})) as LspTextEdit[] | null;
		return result ?? [];
	}

	getDiagnostics(): readonly LspDiagnostic[] {
		return this.diagnostics;
	}

	getSemanticTokens(): readonly LspSemanticTokenSpan[] {
		return this.semanticTokens;
	}

	getDocument(): LspTextDocumentItem | null {
		return this.document;
	}

	subscribeDiagnostics(listener: LspClientListener<readonly LspDiagnostic[]>): () => void {
		this.diagnosticsListeners.add(listener);
		listener(this.diagnostics);
		return () => this.diagnosticsListeners.delete(listener);
	}

	subscribeSemanticTokens(listener: LspClientListener<readonly LspSemanticTokenSpan[]>): () => void {
		this.semanticListeners.add(listener);
		listener(this.semanticTokens);
		return () => this.semanticListeners.delete(listener);
	}

	dispose(): void {
		for (const [, entry] of this.pending) entry.reject(new Error("lsp client disposed"));
		this.pending.clear();
		this.transport.dispose();
	}

	private request(method: string, params: unknown): Promise<unknown> {
		const id = this.nextId++;
		return new Promise((resolve, reject) => {
			this.pending.set(id, { resolve, reject });
			this.transport.send({ jsonrpc: "2.0", id, method, params });
		});
	}

	private async notify(method: string, params: unknown): Promise<void> {
		this.transport.send({ jsonrpc: "2.0", method, params });
	}
}

export function applyTextEdits(text: string, edits: readonly LspTextEdit[]): string {
	const sorted = [...edits].sort((a, b) => positionToOffset(text, b.range.start) - positionToOffset(text, a.range.start));
	let out = text;
	for (const edit of sorted) {
		const start = positionToOffset(out, edit.range.start);
		const end = positionToOffset(out, edit.range.end);
		out = `${out.slice(0, start)}${edit.newText}${out.slice(end)}`;
	}
	return out;
}

export function createWorkerLspTransport(worker: Worker): LspTransport {
	const handlers = new Set<(message: LspMessage) => void>();
	let ready = false;
	const outboundQueue: LspMessage[] = [];
	const flushOutbound = () => {
		if (!ready) return;
		for (const message of outboundQueue.splice(0)) worker.postMessage(message);
	};
	const onMessage = (event: MessageEvent<LspMessage | { op?: string }>) => {
		const data = event.data;
		if (data && typeof data === "object" && "op" in data) return;
		for (const handler of handlers) handler(data as LspMessage);
	};
	worker.addEventListener("message", onMessage);
	worker.addEventListener("message", (event: MessageEvent<{ op?: string }>) => {
		if (event.data?.op === "ready") {
			ready = true;
			flushOutbound();
		}
	});
	return {
		send(message) {
			if (!ready) {
				outboundQueue.push(message);
				return;
			}
			worker.postMessage(message);
		},
		onMessage(handler) {
			handlers.add(handler);
		},
		dispose() {
			worker.removeEventListener("message", onMessage);
			worker.terminate();
		},
	};
}
// #endregion 🔖Lsp

// #region 🔖Grammar
export interface GrammarToken {
	readonly class: string;
	readonly start: number;
	readonly end: number;
}

export interface GrammarRule {
	readonly pattern: RegExp;
	readonly class: string;
}

export interface Grammar {
	readonly languageId: string;
	readonly rules: readonly GrammarRule[];
}

const grammarByLanguage = new Map<string, Grammar>();

export function registerGrammar(grammar: Grammar): void {
	grammarByLanguage.set(grammar.languageId, grammar);
}

export function grammarForLanguage(languageId: string): Grammar | undefined {
	return grammarByLanguage.get(languageId);
}

export function tokenizeWithGrammar(text: string, grammar: Grammar): readonly GrammarToken[] {
	const tokens: GrammarToken[] = [];
	for (const rule of grammar.rules) {
		const pattern = new RegExp(rule.pattern.source, rule.pattern.flags.includes("g") ? rule.pattern.flags : `${rule.pattern.flags}g`);
		for (const match of text.matchAll(pattern)) {
			if (match.index == null) continue;
			tokens.push({ class: rule.class, start: match.index, end: match.index + match[0].length });
		}
	}
	return tokens.sort((a, b) => a.start - b.start || b.end - a.end);
}

registerGrammar({
	languageId: "jack",
	rules: [
		{ pattern: /\b(MATCH|WHERE|RETURN|CREATE|DELETE|SET|MERGE|AND|OR)\b/gi, class: "keyword" },
		{ pattern: /'[^']*'|"[^"]*"/g, class: "string" },
		{ pattern: /\b\d+(?:\.\d+)?\b/g, class: "number" },
		{ pattern: /->|!=|[:=.,\[\]()-]/g, class: "operator" },
		{ pattern: /\b[A-Za-z_][A-Za-z0-9_]*\b/g, class: "ident" },
	],
});
// #endregion 🔖Grammar

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
