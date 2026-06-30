/** @emoji 📡 Jack LSP worker — JSON-RPC pump over WASM `JackLspSession`. */
import {
	isJsonRpcNotification,
	isJsonRpcRequest,
	isJsonRpcResponse,
	type LanguageServer,
	type LspMessage,
} from "@semio-tech/writer-core";
import init, { JackLspSession } from "./pkg/trinity_jack_lsp.js";

let session: JackLspSession | null = null;
let fixtureJson = "";
let initPromise: Promise<void> | null = null;

function ensureSession(): Promise<void> {
	if (!initPromise) {
		initPromise = (async () => {
			await init();
			session = new JackLspSession();
			if (fixtureJson) {
				session.loadFixtureJson(fixtureJson);
			}
		})();
	}
	return initPromise;
}

const server: LanguageServer = {
	handle(message) {
		if (!session) return [];
		if (isJsonRpcRequest(message) && message.method === "jack/loadFixture" && message.params) {
			const params = message.params as { json?: string };
			if (params.json) {
				fixtureJson = params.json;
				session.loadFixtureJson(fixtureJson);
			}
			return message.id == null ? [] : [{ jsonrpc: "2.0", id: message.id, result: null }];
		}
		return JSON.parse(session.handleMessageJson(JSON.stringify(message))) as LspMessage[];
	},
};

self.addEventListener("message", (event: MessageEvent<LspMessage | { op?: string; fixtureJson?: string }>) => {
	const data = event.data;
	if (!data || typeof data !== "object") return;
	if ("op" in data) {
		if (data.op !== "init") return;
		if (data.fixtureJson) fixtureJson = data.fixtureJson;
		void ensureSession().then(() => {
			self.postMessage({ op: "ready" });
		});
		return;
	}
	if (!isJsonRpcRequest(data) && !isJsonRpcNotification(data) && !isJsonRpcResponse(data)) return;
	void ensureSession().then(() => {
		for (const reply of server.handle(data)) {
			self.postMessage(reply);
		}
	});
});
