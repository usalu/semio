/** @emoji 📡 Jack LSP worker — JSON-RPC pump over WASM `JackLspSession`. */
import { isJsonRpcNotification, isJsonRpcRequest, isJsonRpcResponse, type LanguageServer, type LspMessage } from "./protocol.ts";
import init, { JackLspSession } from "../rs/pkg/trinity_jack_lsp.js";

let session: JackLspSession | null = null;
let fixtureJson = "";
let graphDomain = "trinity";
let initPromise: Promise<void> | null = null;

function applyFixtureToSession(): void {
  if (!session || !fixtureJson) return;
  if (typeof session.loadFixtureForDomain === "function") {
    session.loadFixtureForDomain(fixtureJson, graphDomain);
    return;
  }
  session.loadFixtureJson(fixtureJson);
}

function ensureSession(): Promise<void> {
  if (!initPromise) {
    initPromise = (async () => {
      await init();
      session = new JackLspSession();
      applyFixtureToSession();
    })();
  }
  return initPromise;
}

const server: LanguageServer = {
  handle(message) {
    if (!session) return [];
    if (isJsonRpcRequest(message) && message.method === "jack/loadFixture" && message.params) {
      const params = message.params as { json?: string; graphDomain?: string };
      if (params.json) {
        fixtureJson = params.json;
      }
      if (params.graphDomain) {
        graphDomain = params.graphDomain;
      }
      applyFixtureToSession();
      return message.id == null ? [] : [{ jsonrpc: "2.0", id: message.id, result: null }];
    }
    return JSON.parse(session.handleMessageJson(JSON.stringify(message))) as LspMessage[];
  },
};

self.addEventListener("message", (event: MessageEvent<LspMessage | { operation?: string; fixtureJson?: string }>) => {
  const data = event.data;
  if (!data || typeof data !== "object") return;
  if ("operation" in data) {
    if (data.operation !== "init") return;
    if (data.fixtureJson) fixtureJson = data.fixtureJson;
    void ensureSession().then(() => {
      self.postMessage({ operator: "ready" });
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

//#region 🔖protocol
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
  return typeof message === "object" && message !== null && typeof (message as LspMessage).method === "string" && "id" in message;
}

export function isJsonRpcNotification(message: unknown): message is LspMessage & { readonly method: string } {
  return typeof message === "object" && message !== null && typeof (message as LspMessage).method === "string" && !("id" in message);
}

export function isJsonRpcResponse(message: unknown): message is LspMessage & { readonly id: string | number } {
  return typeof message === "object" && message !== null && ("result" in message || "error" in message) && (typeof (message as LspMessage).id === "string" || typeof (message as LspMessage).id === "number");
}
//#endregion 🔖protocol
