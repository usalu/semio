// #region 🧲Header
/// <reference types="vitest/importMeta" />
/** @emoji ✍️ `@semio-tech/writer-core` — writer documents, LSP client, and grammar registry. */
// #endregion 🧲Header

import {
	applyJsonReplaceOp,
	createDocumentVcsEnvelope,
	type DocumentVcsEnvelope,
	materializeDocumentProjection,
	type JsonReplaceOp,
} from "@semio-tech/framework-core";
import { WRITERLANGUAGES_LANGUAGE_IDS, type WriterLanguagesLanguageKindId } from "@semio-tech/graph-manifest";

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

export type WriterLanguageKindId = WriterLanguagesLanguageKindId;
export { WRITERLANGUAGES_LANGUAGE_IDS as WRITER_LANGUAGE_KIND_IDS };

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
	if (!(WRITERLANGUAGES_LANGUAGE_IDS as readonly string[]).includes(value.languageId)) {
		throw new Error(`unknown writer languageId: ${value.languageId}`);
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
	const occupied = new Array<boolean>(text.length).fill(false);
	const tokens: GrammarToken[] = [];
	for (const rule of grammar.rules) {
		const pattern = new RegExp(rule.pattern.source, rule.pattern.flags.includes("g") ? rule.pattern.flags : `${rule.pattern.flags}g`);
		for (const match of text.matchAll(pattern)) {
			if (match.index == null) continue;
			const start = match.index;
			const end = start + match[0].length;
			if (occupied.slice(start, end).some(Boolean)) continue;
			for (let i = start; i < end; i++) occupied[i] = true;
			tokens.push({ class: rule.class, start, end });
		}
	}
	return tokens.sort((a, b) => a.start - b.start || b.end - a.end);
}

export type SelectableSpanKind = "atomic" | "varLabel" | "propertyAccess";

export interface SelectableSpan {
	readonly start: number;
	readonly end: number;
	readonly kind: SelectableSpanKind;
	readonly headEnd?: number;
	readonly tailStart?: number;
}

function tokenSlice(text: string, token: GrammarToken): string {
	return text.slice(token.start, token.end);
}

/** 🎯 Builds atomic and composite jack spans for token-wise selection. */
export function selectableSpansForJack(text: string, tokens: readonly GrammarToken[]): readonly SelectableSpan[] {
	const spans: SelectableSpan[] = tokens.map((token) => ({
		start: token.start,
		end: token.end,
		kind: "atomic",
	}));
	for (let i = 0; i + 2 < tokens.length; i++) {
		const head = tokens[i];
		const colon = tokens[i + 1];
		const tail = tokens[i + 2];
		if (head?.class === "ident" && tokenSlice(text, colon!) === ":" && tail?.class === "ident") {
			spans.push({ start: head.start, end: tail.end, kind: "varLabel", headEnd: head.end });
		}
	}
	for (let i = 0; i < tokens.length; i++) {
		const head = tokens[i];
		if (head?.class !== "ident") continue;
		let end = head.end;
		let tailStart = head.start;
		let j = i;
		while (j + 2 < tokens.length && tokenSlice(text, tokens[j + 1]!) === "." && tokens[j + 2]?.class === "ident") {
			const tail = tokens[j + 2]!;
			end = tail.end;
			tailStart = tail.start;
			spans.push({ start: head.start, end, kind: "propertyAccess", headEnd: head.end, tailStart });
			j += 2;
		}
	}
	return spans.sort((a, b) => a.start - b.start || b.end - a.end);
}

export function selectableSpansForLanguage(text: string, languageId: string, tokens: readonly GrammarToken[]): readonly SelectableSpan[] {
	if (languageId === "jack") return selectableSpansForJack(text, tokens);
	return tokens.map((token) => ({ start: token.start, end: token.end, kind: "atomic" as const }));
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

// #region 🔖JackAst
/** @emoji 🌳 One jack AST node with source span for hierarchy sync. */
export interface JackAstNode {
	readonly id: string;
	readonly kind: string;
	readonly label: string;
	readonly start: number;
	readonly end: number;
	readonly children: readonly JackAstNode[];
}

type JackLexToken =
	| { readonly kind: "kwMatch" | "kwWhere" | "kwReturn" | "kwCreate" | "kwDelete" | "kwSet" | "kwMerge" | "kwAnd" | "kwOr"; readonly start: number; readonly end: number }
	| { readonly kind: "ident"; readonly start: number; readonly end: number; readonly text: string }
	| { readonly kind: "number"; readonly start: number; readonly end: number; readonly value: number }
	| { readonly kind: "string"; readonly start: number; readonly end: number; readonly text: string }
	| { readonly kind: "lparen" | "rparen" | "lbracket" | "rbracket" | "colon" | "comma" | "dot" | "eq" | "ne" | "dash" | "arrow"; readonly start: number; readonly end: number }
	| { readonly kind: "eof"; readonly start: number; readonly end: number };

function jackAstId(kind: string, start: number, end: number): string {
	return `jack-ast-${kind}-${start}-${end}`;
}

function jackAstNode(
	kind: string,
	start: number,
	end: number,
	source: string,
	children: readonly JackAstNode[],
	label?: string,
): JackAstNode {
	const slice = source.slice(start, end).replace(/\s+/g, " ").trim();
	return {
		id: jackAstId(kind, start, end),
		kind,
		label: label ?? (slice || kind),
		start,
		end,
		children,
	};
}

function tokenizeJackSource(input: string): JackLexToken[] {
	const tokens: JackLexToken[] = [];
	const bytes = input;
	let i = 0;
	while (i < bytes.length) {
		const start = i;
		const c = bytes[i]!;
		if (/\s/.test(c)) {
			i += 1;
			continue;
		}
		if (c === "(") {
			tokens.push({ kind: "lparen", start, end: start + 1 });
			i += 1;
			continue;
		}
		if (c === ")") {
			tokens.push({ kind: "rparen", start, end: start + 1 });
			i += 1;
			continue;
		}
		if (c === "[") {
			tokens.push({ kind: "lbracket", start, end: start + 1 });
			i += 1;
			continue;
		}
		if (c === "]") {
			tokens.push({ kind: "rbracket", start, end: start + 1 });
			i += 1;
			continue;
		}
		if (c === ":") {
			tokens.push({ kind: "colon", start, end: start + 1 });
			i += 1;
			continue;
		}
		if (c === ",") {
			tokens.push({ kind: "comma", start, end: start + 1 });
			i += 1;
			continue;
		}
		if (c === ".") {
			tokens.push({ kind: "dot", start, end: start + 1 });
			i += 1;
			continue;
		}
		if (c === "=") {
			tokens.push({ kind: "eq", start, end: start + 1 });
			i += 1;
			continue;
		}
		if (c === "!" && bytes[i + 1] === "=") {
			tokens.push({ kind: "ne", start, end: start + 2 });
			i += 2;
			continue;
		}
		if (c === "-" && bytes[i + 1] === ">") {
			tokens.push({ kind: "arrow", start, end: start + 2 });
			i += 2;
			continue;
		}
		if (c === "-" && i + 1 < bytes.length && /\d/.test(bytes[i + 1]!)) {
			let j = i + 1;
			while (j < bytes.length && (/\d/.test(bytes[j]!) || bytes[j] === ".")) j += 1;
			tokens.push({ kind: "number", start, end: j, value: Number(bytes.slice(start, j)) });
			i = j;
			continue;
		}
		if (c === "-") {
			tokens.push({ kind: "dash", start, end: start + 1 });
			i += 1;
			continue;
		}
		if (c === "'" || c === '"') {
			const quote = c;
			i += 1;
			const litStart = i;
			while (i < bytes.length && bytes[i] !== quote) i += 1;
			const text = bytes.slice(litStart, i);
			if (i < bytes.length) i += 1;
			tokens.push({ kind: "string", start, end: i, text });
			continue;
		}
		if (/\d/.test(c)) {
			let j = i;
			while (j < bytes.length && (/\d/.test(bytes[j]!) || bytes[j] === ".")) j += 1;
			tokens.push({ kind: "number", start, end: j, value: Number(bytes.slice(start, j)) });
			i = j;
			continue;
		}
		if (/[A-Za-z_]/.test(c)) {
			let j = i;
			while (j < bytes.length && /[A-Za-z0-9_]/.test(bytes[j]!)) j += 1;
			const word = bytes.slice(start, j);
			const upper = word.toUpperCase();
			const kwMap = {
				MATCH: "kwMatch",
				WHERE: "kwWhere",
				RETURN: "kwReturn",
				CREATE: "kwCreate",
				DELETE: "kwDelete",
				SET: "kwSet",
				MERGE: "kwMerge",
				AND: "kwAnd",
				OR: "kwOr",
			} as const;
			const mapped = kwMap[upper as keyof typeof kwMap];
			if (mapped) tokens.push({ kind: mapped, start, end: j });
			else tokens.push({ kind: "ident", start, end: j, text: word });
			i = j;
			continue;
		}
		i += 1;
	}
	tokens.push({ kind: "eof", start: input.length, end: input.length });
	return tokens;
}

class JackAstParser {
	private pos = 0;

	constructor(
		private readonly tokens: readonly JackLexToken[],
		private readonly source: string,
	) {}

	parseQuery(): JackAstNode {
		const start = 0;
		const children: JackAstNode[] = [];
		while (!this.at("eof")) {
			children.push(this.parseClause());
		}
		const end = this.source.length;
		return jackAstNode("query", start, end, this.source, children, "Query");
	}

	private peek(): JackLexToken {
		return this.tokens[this.pos] ?? { kind: "eof", start: this.source.length, end: this.source.length };
	}

	private at(kind: JackLexToken["kind"]): boolean {
		return this.peek().kind === kind;
	}

	private bump(): JackLexToken {
		const token = this.peek();
		if (token.kind !== "eof") this.pos += 1;
		return token;
	}

	private expectIdent(): JackLexToken & { kind: "ident" } {
		const token = this.bump();
		if (token.kind !== "ident") throw new Error(`expected ident at ${token.start}`);
		return token;
	}

	private expect(kind: JackLexToken["kind"]): JackLexToken {
		const token = this.bump();
		if (token.kind !== kind) throw new Error(`expected ${kind} at ${token.start}, got ${token.kind}`);
		return token;
	}

	private parseClause(): JackAstNode {
		const start = this.peek().start;
		if (this.at("kwMatch")) {
			this.bump();
			const patterns = [this.parsePattern()];
			while (this.at("comma")) {
				this.bump();
				patterns.push(this.parsePattern());
			}
			const end = patterns.at(-1)?.end ?? start;
			return jackAstNode("match", start, end, this.source, patterns, "MATCH");
		}
		if (this.at("kwWhere")) {
			this.bump();
			const expr = this.parseExpr();
			return jackAstNode("where", start, expr.end, this.source, [expr], "WHERE");
		}
		if (this.at("kwReturn")) {
			this.bump();
			const items = [this.parseReturnItem()];
			while (this.at("comma")) {
				this.bump();
				items.push(this.parseReturnItem());
			}
			const end = items.at(-1)?.end ?? start;
			return jackAstNode("return", start, end, this.source, items, "RETURN");
		}
		if (this.at("kwCreate")) {
			this.bump();
			const pattern = this.parsePattern();
			return jackAstNode("create", start, pattern.end, this.source, [pattern], "CREATE");
		}
		if (this.at("kwDelete")) {
			this.bump();
			const vars = [this.expectIdent()];
			while (this.at("comma")) {
				this.bump();
				vars.push(this.expectIdent());
			}
			const children = vars.map((v) => jackAstNode("var", v.start, v.end, this.source, [], v.text));
			const end = vars.at(-1)?.end ?? start;
			return jackAstNode("delete", start, end, this.source, children, "DELETE");
		}
		if (this.at("kwSet")) {
			this.bump();
			const items = [this.parseAssignment()];
			while (this.at("comma")) {
				this.bump();
				items.push(this.parseAssignment());
			}
			const end = items.at(-1)?.end ?? start;
			return jackAstNode("set", start, end, this.source, items, "SET");
		}
		if (this.at("kwMerge")) {
			this.bump();
			const pattern = this.parsePattern();
			return jackAstNode("merge", start, pattern.end, this.source, [pattern], "MERGE");
		}
		throw new Error(`unexpected clause at ${start}`);
	}

	private parsePattern(): JackAstNode {
		const start = this.expect("lparen").start;
		const left = this.parsePatternNode();
		this.expect("rparen");
		if (this.at("dash")) {
			const edgeStart = this.bump().start;
			this.expect("lbracket");
			const edgeChildren: JackAstNode[] = [];
			if (this.at("ident")) {
				const edgeVar = this.expectIdent();
				edgeChildren.push(jackAstNode("edgeVar", edgeVar.start, edgeVar.end, this.source, [], edgeVar.text));
			}
			if (this.at("colon")) {
				this.bump();
				const edgeKind = this.expectIdent();
				edgeChildren.push(jackAstNode("edgeKind", edgeKind.start, edgeKind.end, this.source, [], edgeKind.text));
			}
			this.expect("rbracket");
			this.expect("arrow");
			this.expect("lparen");
			const right = this.parsePatternNode();
			this.expect("rparen");
			const edgeEnd = right.end;
			const edge = jackAstNode("edge", edgeStart, edgeEnd, this.source, edgeChildren, "edge");
			return jackAstNode("pattern", start, edgeEnd, this.source, [left, edge, right]);
		}
		return jackAstNode("pattern", start, left.end, this.source, [left]);
	}

	private parsePatternNode(): JackAstNode {
		const start = this.peek().start;
		const varTok = this.expectIdent();
		this.expect("colon");
		const kindTok = this.expectIdent();
		const varNode = jackAstNode("var", varTok.start, varTok.end, this.source, [], varTok.text);
		const kindNode = jackAstNode("label", kindTok.start, kindTok.end, this.source, [], kindTok.text);
		return jackAstNode("patternNode", start, kindTok.end, this.source, [varNode, kindNode], `${varTok.text}:${kindTok.text}`);
	}

	private parseReturnItem(): JackAstNode {
		const start = this.peek().start;
		const varTok = this.expectIdent();
		if (this.at("dot")) {
			this.bump();
			const propTok = this.expectIdent();
			const varNode = jackAstNode("var", varTok.start, varTok.end, this.source, [], varTok.text);
			const propNode = jackAstNode("property", propTok.start, propTok.end, this.source, [], propTok.text);
			return jackAstNode("returnItem", start, propTok.end, this.source, [varNode, propNode], `${varTok.text}.${propTok.text}`);
		}
		return jackAstNode("returnItem", start, varTok.end, this.source, [jackAstNode("var", varTok.start, varTok.end, this.source, [], varTok.text)], varTok.text);
	}

	private parseAssignment(): JackAstNode {
		const start = this.peek().start;
		const varTok = this.expectIdent();
		this.expect("dot");
		const propTok = this.expectIdent();
		this.expect("eq");
		const value = this.parseValue();
		const varNode = jackAstNode("var", varTok.start, varTok.end, this.source, [], varTok.text);
		const propNode = jackAstNode("property", propTok.start, propTok.end, this.source, [], propTok.text);
		return jackAstNode("assignment", start, value.end, this.source, [varNode, propNode, value]);
	}

	private parseExpr(): JackAstNode {
		return this.parseOrExpr();
	}

	private parseOrExpr(): JackAstNode {
		let left = this.parseAndExpr();
		while (this.at("kwOr")) {
			const opStart = this.bump().start;
			const right = this.parseAndExpr();
			left = jackAstNode("or", opStart, right.end, this.source, [left, right], "OR");
		}
		return left;
	}

	private parseAndExpr(): JackAstNode {
		let left = this.parseCmpExpr();
		while (this.at("kwAnd")) {
			const opStart = this.bump().start;
			const right = this.parseCmpExpr();
			left = jackAstNode("and", opStart, right.end, this.source, [left, right], "AND");
		}
		return left;
	}

	private parseCmpExpr(): JackAstNode {
		const start = this.peek().start;
		const varTok = this.expectIdent();
		this.expect("dot");
		const propTok = this.expectIdent();
		const op = this.bump();
		if (op.kind !== "eq" && op.kind !== "ne") throw new Error(`expected comparison at ${op.start}`);
		const value = this.parseValue();
		const varNode = jackAstNode("var", varTok.start, varTok.end, this.source, [], varTok.text);
		const propNode = jackAstNode("property", propTok.start, propTok.end, this.source, [], propTok.text);
		const kind = op.kind === "eq" ? "eq" : "ne";
		return jackAstNode(kind, start, value.end, this.source, [varNode, propNode, value]);
	}

	private parseValue(): JackAstNode {
		const token = this.bump();
		if (token.kind === "number") return jackAstNode("number", token.start, token.end, this.source, [], String(token.value));
		if (token.kind === "string") return jackAstNode("string", token.start, token.end, this.source, [], token.text);
		if (token.kind === "ident") {
			const lower = token.text.toLowerCase();
			if (lower === "true" || lower === "false") return jackAstNode("bool", token.start, token.end, this.source, [], token.text);
			if (lower === "null") return jackAstNode("null", token.start, token.end, this.source, [], "null");
		}
		throw new Error(`expected value at ${token.start}`);
	}
}

/** @emoji 🌳 Parse jack source into a span-tracked AST for hierarchy panels. */
export function parseJackAst(text: string): JackAstNode {
	try {
		const tokens = tokenizeJackSource(text);
		return new JackAstParser(tokens, text).parseQuery();
	} catch (error) {
		return jackAstNode("error", 0, text.length, text, [], error instanceof Error ? error.message : "parse error");
	}
}

/** @emoji 🎯 Deepest AST node containing a byte offset. */
export function findDeepestJackAstNodeAt(root: JackAstNode, offset: number): JackAstNode | null {
	if (offset < root.start || offset >= root.end) return null;
	for (const child of root.children) {
		const found = findDeepestJackAstNodeAt(child, offset);
		if (found) return found;
	}
	return root;
}

/** @emoji 🔎 Find an AST node by stable id. */
export function jackAstNodeById(root: JackAstNode, id: string): JackAstNode | null {
	if (root.id === id) return root;
	for (const child of root.children) {
		const found = jackAstNodeById(child, id);
		if (found) return found;
	}
	return null;
}

/** @emoji 🖱️ Smallest AST node that fully contains a selection range. */
export function jackAstNodeForSelection(root: JackAstNode, start: number, end: number): JackAstNode | null {
	let best: JackAstNode | null = null;
	const visit = (node: JackAstNode): void => {
		if (node.start <= start && node.end >= end) {
			if (!best || node.end - node.start < best.end - best.start) best = node;
		}
		for (const child of node.children) visit(child);
	};
	visit(root);
	return best;
}

/** @emoji 👻 Inline placeholder for a missing jack token at a byte offset. */
export interface JackEditorPlaceholder {
	readonly offset: number;
	readonly label: string;
}

function jackPlaceholderVisible(caret: number, offset: number): boolean {
	return caret >= offset - 32 && caret <= offset + 48;
}

function jackTokenExpectsExpr(kind: JackLexToken["kind"]): boolean {
	return kind === "kwAnd" || kind === "kwOr";
}

function jackTokenExpectsPattern(kind: JackLexToken["kind"]): boolean {
	return kind === "kwMatch" || kind === "kwCreate" || kind === "kwMerge";
}

/** @emoji 👻 Required jack token placeholders near the caret (e.g. AND → second expr). */
export function jackEditorPlaceholders(text: string, caret: number): readonly JackEditorPlaceholder[] {
	const tokens = tokenizeJackSource(text);
	const out: JackEditorPlaceholder[] = [];
	for (let i = 0; i < tokens.length; i++) {
		const tok = tokens[i]!;
		const next = tokens[i + 1];
		const nextKind = next?.kind;
		if (jackTokenExpectsPattern(tok.kind)) {
			if (!next || nextKind === "eof" || (nextKind !== "lparen" && nextKind !== "ident")) {
				const offset = tok.end;
				if (jackPlaceholderVisible(caret, offset)) {
					out.push({ offset, label: "(var:Label)" });
				}
			}
		}
		if (tok.kind === "kwReturn") {
			if (!next || nextKind === "eof" || nextKind === "comma") {
				const offset = tok.end;
				if (jackPlaceholderVisible(caret, offset)) {
					out.push({ offset, label: "item" });
				}
			}
		}
		if (tok.kind === "kwWhere") {
			if (!next || nextKind === "eof" || nextKind === "kwReturn") {
				const offset = tok.end;
				if (jackPlaceholderVisible(caret, offset)) {
					out.push({ offset, label: "condition" });
				}
			}
		}
		if (jackTokenExpectsExpr(tok.kind)) {
			if (!next || nextKind === "eof" || jackTokenExpectsExpr(nextKind) || nextKind === "kwWhere" || nextKind === "kwReturn") {
				const offset = tok.end;
				if (jackPlaceholderVisible(caret, offset)) {
					out.push({ offset, label: "expr" });
				}
			}
		}
		if (tok.kind === "colon") {
			if (!next || nextKind === "eof" || nextKind === "rparen" || nextKind === "comma" || nextKind === "rbracket") {
				const offset = tok.end;
				if (jackPlaceholderVisible(caret, offset)) {
					out.push({ offset, label: "Label" });
				}
			}
		}
		if (tok.kind === "comma" && tokens[i - 1]?.kind === "kwReturn") {
			if (!next || nextKind === "eof" || nextKind === "comma") {
				const offset = tok.end;
				if (jackPlaceholderVisible(caret, offset)) {
					out.push({ offset, label: "item" });
				}
			}
		}
		if (tok.kind === "dash") {
			if (!next || nextKind !== "lbracket") {
				const offset = tok.end;
				if (jackPlaceholderVisible(caret, offset)) {
					out.push({ offset, label: "[var:Kind]" });
				}
			}
		}
		if (tok.kind === "lbracket") {
			const after = tokens[i + 1];
			const close = tokens.find((t, j) => j > i && t.kind === "rbracket");
			if (!close) {
				const offset = tok.end;
				if (jackPlaceholderVisible(caret, offset)) {
					out.push({ offset, label: "var:Kind" });
				}
			} else if (!after || after.kind === "rbracket" || after.kind === "colon") {
				const offset = tok.end;
				if (jackPlaceholderVisible(caret, offset)) {
					out.push({ offset, label: "var:Kind" });
				}
			}
		}
		if (tok.kind === "eq" || tok.kind === "ne") {
			if (!next || nextKind === "eof" || nextKind === "kwAnd" || nextKind === "kwOr") {
				const offset = tok.end;
				if (jackPlaceholderVisible(caret, offset)) {
					out.push({ offset, label: "value" });
				}
			}
		}
	}
	return out;
}

// #region 🔖JackSymbols
/** @emoji 📍 One source span for a jack symbol occurrence. */
export interface JackSymbolOccurrence {
	readonly start: number;
	readonly end: number;
}

export type JackSymbolKind = "variable" | "property" | "nodeKind" | "edgeKind";

/** @emoji 🏷️ Jack symbol resolved at a byte offset for semantic hover, select, and rename. */
export interface JackSymbolAtCursor {
	readonly kind: JackSymbolKind;
	readonly name: string;
	readonly occurrences: readonly JackSymbolOccurrence[];
}

/** @emoji 🔗 Bound jack variable names from pattern bindings. */
export function jackBoundVariableNames(text: string): ReadonlySet<string> {
	const tokens = tokenizeJackSource(text);
	const vars = new Set<string>();
	for (let i = 0; i + 2 < tokens.length; i++) {
		const open = tokens[i]!;
		const name = tokens[i + 1]!;
		const colon = tokens[i + 2]!;
		if ((open.kind === "lparen" || open.kind === "lbracket") && name.kind === "ident" && colon.kind === "colon") {
			vars.add(name.text);
		}
	}
	return vars;
}

function isJackVariableUseToken(tokens: readonly JackLexToken[], index: number, bound: ReadonlySet<string>): boolean {
	const tok = tokens[index];
	if (tok?.kind !== "ident" || !bound.has(tok.text)) return false;
	const prev = tokens[index - 1];
	if (prev?.kind === "colon" || prev?.kind === "dot") return false;
	return true;
}

/** @emoji 🔁 All bound-variable occurrences for a jack variable name. */
export function jackVariableOccurrences(text: string, varName: string): readonly JackSymbolOccurrence[] {
	const tokens = tokenizeJackSource(text);
	const bound = jackBoundVariableNames(text);
	if (!bound.has(varName)) return [];
	const out: JackSymbolOccurrence[] = [];
	for (let i = 0; i < tokens.length; i++) {
		const tok = tokens[i]!;
		if (tok.kind === "ident" && tok.text === varName && isJackVariableUseToken(tokens, i, bound)) {
			out.push({ start: tok.start, end: tok.end });
		}
	}
	return out;
}

/** @emoji 🎯 Resolve the jack symbol at a byte offset for semantic editor actions. */
export function jackSymbolAtOffset(text: string, offset: number): JackSymbolAtCursor | null {
	const tokens = tokenizeJackSource(text);
	const clamped = Math.max(0, Math.min(offset, text.length));
	const index = tokens.findIndex((tok) => tok.kind === "ident" && clamped >= tok.start && clamped < tok.end);
	if (index < 0) return null;
	const tok = tokens[index]!;
	const prev = tokens[index - 1];
	if (prev?.kind === "colon") {
		const kind: JackSymbolKind = tokens[index - 2]?.kind === "lbracket" ? "edgeKind" : "nodeKind";
		return { kind, name: tok.text, occurrences: [{ start: tok.start, end: tok.end }] };
	}
	if (prev?.kind === "dot") {
		return { kind: "property", name: tok.text, occurrences: [{ start: tok.start, end: tok.end }] };
	}
	const bound = jackBoundVariableNames(text);
	if (!isJackVariableUseToken(tokens, index, bound)) return null;
	return { kind: "variable", name: tok.text, occurrences: jackVariableOccurrences(text, tok.text) };
}

/** @emoji ✏️ Apply a semantic jack rename across all occurrence spans. */
export function applyJackRename(
	text: string,
	occurrences: readonly JackSymbolOccurrence[],
	newName: string,
): { readonly text: string; readonly occurrences: readonly JackSymbolOccurrence[] } {
	const sorted = [...occurrences].sort((a, b) => b.start - a.start);
	let out = text;
	const nextOccurrences: JackSymbolOccurrence[] = [];
	for (const occ of sorted) {
		out = `${out.slice(0, occ.start)}${newName}${out.slice(occ.end)}`;
		nextOccurrences.unshift({ start: occ.start, end: occ.start + newName.length });
	}
	return { text: out, occurrences: nextOccurrences };
}
// #endregion 🔖JackSymbols
// #endregion 🔖JackAst

//#region 🔖DocumentVcs
export type WriterDocumentVcsEnvelope = DocumentVcsEnvelope<WriterDocumentV1, JsonReplaceOp<WriterDocumentV1>>;

/** @emoji 📦 Creates a writer document VCS envelope with an empty or seeded projection. */
export function createWriterDocumentVcsEnvelope(
	id: string,
	projection: WriterDocumentV1 = createWriterDocument({ id, languageId: "plaintext", text: "" }),
): WriterDocumentVcsEnvelope {
	return createDocumentVcsEnvelope(WRITER_DOCUMENT_SCHEMA, id, projection);
}

/** @emoji 🔁 Materializes a writer document from its VCS envelope. */
export function materializeWriterDocument(envelope: WriterDocumentVcsEnvelope, appliedChangeIds: readonly string[] = []): WriterDocumentV1 {
	return materializeDocumentProjection(envelope, appliedChangeIds, applyJsonReplaceOp);
}

/** @emoji 🧩 Semios app VCS handler factory for writer documents. */
export function createWriterAppVcsHandler() {
	return {
		format: WRITER_DOCUMENT_SCHEMA,
		createEnvelope: (id: string) => createWriterDocumentVcsEnvelope(id),
		applyOp: applyJsonReplaceOp,
		serializeEnvelope: (envelope: WriterDocumentVcsEnvelope) => JSON.stringify(envelope),
		deserializeEnvelope: (json: string) => JSON.parse(json) as WriterDocumentVcsEnvelope,
		materializeProjection: (source: { readonly vcsJson?: string; readonly inline?: string }) => {
			if (source.vcsJson) {
				const envelope = JSON.parse(source.vcsJson) as WriterDocumentVcsEnvelope;
				return materializeWriterDocument(envelope, envelope.vcs.operations.map((change) => change.id));
			}
			if (source.inline) return parseWriterDocumentJson(source.inline);
			return createWriterDocument({ id: "writer", languageId: "plaintext", text: "" });
		},
	};
}
//#endregion 🔖DocumentVcs

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
