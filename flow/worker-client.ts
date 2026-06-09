/** @emoji 📡 Main-thread client for the flow orchestrator worker. */

export type FlowEvalWorkerResult = {
  readonly outputsJson: string;
};

type FlowWorkerRequest =
  | { readonly op: "init" }
  | { readonly op: "loadFixture"; readonly reqId: number; readonly json: string }
  | { readonly op: "evaluate"; readonly reqId: number }
  | { readonly op: "tessellatePreviews"; readonly reqId: number; readonly outputsJson: string; readonly tolerance?: number }
  | { readonly op: "previewText"; readonly reqId: number }
  | { readonly op: "fixtureJson"; readonly reqId: number };

type FlowWorkerResponse =
  | { readonly op: "ready" }
  | { readonly op: "result"; readonly reqId: number; readonly json: string }
  | { readonly op: "error"; readonly reqId: number; readonly message: string };

export function createFlowOrchestratorWorker(): Worker {
  return new Worker(new URL("./worker.ts", import.meta.url), { type: "module" });
}

export class FlowOrchestratorClient {
  private readonly worker: Worker;
  private nextReqId = 1;
  private ready: Promise<void>;
  private readonly pending = new Map<number, { resolve: (json: string) => void; reject: (err: Error) => void }>();

  constructor(worker = createFlowOrchestratorWorker()) {
    this.worker = worker;
    this.ready = new Promise((resolve, reject) => {
      const onMessage = (event: MessageEvent<FlowWorkerResponse>) => {
        const msg = event.data;
        if (msg.op === "ready") {
          this.worker.removeEventListener("message", onMessage);
          resolve();
          return;
        }
        if (msg.op === "error" && msg.reqId === 0) {
          this.worker.removeEventListener("message", onMessage);
          reject(new Error(msg.message));
        }
      };
      this.worker.addEventListener("message", onMessage);
      this.worker.postMessage({ op: "init" } satisfies FlowWorkerRequest);
    });
    this.worker.addEventListener("message", (event: MessageEvent<FlowWorkerResponse>) => {
      const msg = event.data;
      if (msg.op !== "result" && msg.op !== "error") return;
      const entry = this.pending.get(msg.reqId);
      if (!entry) return;
      this.pending.delete(msg.reqId);
      if (msg.op === "error") entry.reject(new Error(msg.message));
      else entry.resolve(msg.json);
    });
  }

  private async request(op: Exclude<FlowWorkerRequest["op"], "init">, payload: Omit<FlowWorkerRequest, "op" | "reqId">): Promise<string> {
    await this.ready;
    const reqId = this.nextReqId++;
    return new Promise((resolve, reject) => {
      this.pending.set(reqId, { resolve, reject });
      this.worker.postMessage({ op, reqId, ...payload } as FlowWorkerRequest);
    });
  }

  async loadFixtureJson(json: string): Promise<void> {
    await this.request("loadFixture", { json });
  }

  async evaluate(): Promise<FlowEvalWorkerResult> {
    const json = await this.request("evaluate", {});
    return JSON.parse(json) as FlowEvalWorkerResult;
  }

  async tessellatePreviews(outputsJson: string, tolerance = 0.02): Promise<Readonly<Record<string, unknown>>> {
    const json = await this.request("tessellatePreviews", { outputsJson, tolerance });
    return (JSON.parse(json) as { previewMeshes: Readonly<Record<string, unknown>> }).previewMeshes ?? {};
  }

  async previewText(): Promise<string> {
    const json = await this.request("previewText", {});
    return (JSON.parse(json) as { text: string }).text;
  }

  terminate(): void {
    this.worker.terminate();
  }
}
