/** @emoji 📡 Main-thread client for the imperative run worker. */

import type { RunResult } from "./index.ts";

type ImperativeWorkerRequest =
	| { readonly op: "init" }
	| { readonly op: "run"; readonly reqId: number; readonly documentJson: string };

type ImperativeWorkerResponse =
	| { readonly op: "ready" }
	| { readonly op: "result"; readonly reqId: number; readonly json: string }
	| { readonly op: "error"; readonly reqId: number; readonly message: string };

export function createImperativeRunWorker(): Worker {
	return new Worker(new URL("./worker.ts", import.meta.url), { type: "module" });
}

export class ImperativeRunClient {
	private worker: Worker;
	private readonly createWorker: () => Worker;
	private nextReqId = 1;
	private ready: Promise<void>;
	private readonly pending = new Map<number, { resolve: (json: string) => void; reject: (err: Error) => void }>();

	constructor(createWorker: () => Worker = createImperativeRunWorker) {
		this.createWorker = createWorker;
		this.worker = createWorker();
		this.ready = this.bootWorker();
	}

	private bootWorker(): Promise<void> {
		this.worker.addEventListener("message", this.onWorkerMessage);
		return new Promise((resolve, reject) => {
			const onReady = (event: MessageEvent<ImperativeWorkerResponse>) => {
				const msg = event.data;
				if (msg.op === "ready") {
					this.worker.removeEventListener("message", onReady);
					resolve();
					return;
				}
				if (msg.op === "error" && msg.reqId === 0) {
					this.worker.removeEventListener("message", onReady);
					reject(new Error(msg.message));
				}
			};
			this.worker.addEventListener("message", onReady);
			this.worker.postMessage({ op: "init" } satisfies ImperativeWorkerRequest);
		});
	}

	private readonly onWorkerMessage = (event: MessageEvent<ImperativeWorkerResponse>) => {
		const msg = event.data;
		if (msg.op !== "result" && msg.op !== "error") return;
		const entry = this.pending.get(msg.reqId);
		if (!entry) return;
		this.pending.delete(msg.reqId);
		if (msg.op === "error") entry.reject(new Error(msg.message));
		else entry.resolve(msg.json);
	};

	private rejectPending(message: string): void {
		for (const [, entry] of this.pending) entry.reject(new Error(message));
		this.pending.clear();
	}

	restart(reason: string): void {
		this.rejectPending(reason);
		this.worker.removeEventListener("message", this.onWorkerMessage);
		this.worker.terminate();
		this.worker = this.createWorker();
		this.ready = this.bootWorker();
	}

	private async request(documentJson: string): Promise<string> {
		await this.ready;
		const reqId = this.nextReqId++;
		return new Promise((resolve, reject) => {
			this.pending.set(reqId, { resolve, reject });
			this.worker.postMessage({ op: "run", reqId, documentJson } satisfies ImperativeWorkerRequest);
		});
	}

	async runDocument(documentJson: string): Promise<RunResult> {
		const json = await this.request(documentJson);
		return JSON.parse(json) as RunResult;
	}

	stop(): void {
		this.restart("imperative worker stopped by user");
	}

	terminate(): void {
		this.worker.terminate();
	}
}
