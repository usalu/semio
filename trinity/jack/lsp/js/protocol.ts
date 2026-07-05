/** @emoji 📡 Minimal JSON-RPC and LSP message guards for Jack language-server workers. */

export type LspMessage = {
	readonly jsonrpc?: string;
	readonly id?: string | number | null;
	readonly method?: string;
	readonly params?: unknown;
	readonly result?: unknown;
	readonly error?: unknown;
};

export interface LanguageServer {
	handle(message: LspMessage): LspMessage[];
}

export function isJsonRpcRequest(message: unknown): message is LspMessage & { readonly method: string } {
	return (
		typeof message === "object" &&
		message !== null &&
		typeof (message as LspMessage).method === "string" &&
		"id" in message
	);
}

export function isJsonRpcNotification(message: unknown): message is LspMessage & { readonly method: string } {
	return (
		typeof message === "object" &&
		message !== null &&
		typeof (message as LspMessage).method === "string" &&
		!("id" in message)
	);
}

export function isJsonRpcResponse(message: unknown): message is LspMessage & { readonly id: string | number } {
	return (
		typeof message === "object" &&
		message !== null &&
		("result" in message || "error" in message) &&
		(typeof (message as LspMessage).id === "string" || typeof (message as LspMessage).id === "number")
	);
}
