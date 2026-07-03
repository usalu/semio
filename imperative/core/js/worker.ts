/** @emoji 👷 Dedicated imperative run worker — executes paths off the UI thread. */

import initImperativeWasm, { ImperativeSession, initSync } from "../rs/pkg/imperative_core.js";
import imperativeCoreWasmUrl from "../rs/pkg/imperative_core_bg.wasm?url";

type ImperativeWorkerRequest =
	| { readonly op: "init" }
	| { readonly op: "run"; readonly reqId: number; readonly documentJson: string };

type ImperativeWorkerResponse =
	| { readonly op: "ready" }
	| { readonly op: "result"; readonly reqId: number; readonly json: string }
	| { readonly op: "error"; readonly reqId: number; readonly message: string };

let session: ImperativeSession | null = null;

function post(message: ImperativeWorkerResponse): void {
	self.postMessage(message);
}

async function ensureSession(): Promise<ImperativeSession> {
	if (session) return session;
	if (typeof initSync === "function") {
		const response = await fetch(imperativeCoreWasmUrl);
		const bytes = await response.arrayBuffer();
		initSync({ module: bytes });
	}
	await initImperativeWasm({ module_or_path: imperativeCoreWasmUrl });
	session = new ImperativeSession();
	return session;
}

self.onmessage = async (event: MessageEvent<ImperativeWorkerRequest>) => {
	const msg = event.data;
	try {
		if (msg.op === "init") {
			await ensureSession();
			post({ op: "ready" });
			return;
		}
		const active = await ensureSession();
		if (msg.op === "run") {
			active.loadPathJson(msg.documentJson);
			const result = active.run();
			post({ op: "result", reqId: msg.reqId, json: result });
		}
	} catch (err) {
		const reqId = "reqId" in msg ? msg.reqId : 0;
		post({ op: "error", reqId, message: err instanceof Error ? err.message : String(err) });
	}
};
