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

const FLOW_EVAL_TIMEOUT_MS = 30_000;

export class FlowOrchestratorClient {
  private worker: Worker;
  private readonly createWorker: () => Worker;
  private nextReqId = 1;
  private ready: Promise<void>;
  private readonly pending = new Map<number, { resolve: (json: string) => void; reject: (err: Error) => void }>();

  constructor(createWorker: () => Worker = createFlowOrchestratorWorker) {
    this.createWorker = createWorker;
    this.worker = createWorker();
    this.ready = this.bootWorker();
  }

  private bootWorker(): Promise<void> {
    this.worker.addEventListener("message", this.onWorkerMessage);
    return new Promise((resolve, reject) => {
      const onReady = (event: MessageEvent<FlowWorkerResponse>) => {
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
      this.worker.postMessage({ op: "init" } satisfies FlowWorkerRequest);
    });
  }

  private readonly onWorkerMessage = (event: MessageEvent<FlowWorkerResponse>) => {
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

  private restartWorker(reason: string): void {
    this.rejectPending(reason);
    this.worker.removeEventListener("message", this.onWorkerMessage);
    this.worker.terminate();
    this.worker = this.createWorker();
    this.ready = this.bootWorker();
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

  async evaluate(timeoutMs = FLOW_EVAL_TIMEOUT_MS): Promise<FlowEvalWorkerResult> {
    let timeoutId: ReturnType<typeof setTimeout> | undefined;
    try {
      const json = await Promise.race([
        this.request("evaluate", {}),
        new Promise<string>((_, reject) => {
          timeoutId = setTimeout(() => reject(new Error("flow evaluate timed out")), timeoutMs);
        }),
      ]);
      return JSON.parse(json) as FlowEvalWorkerResult;
    } catch (err) {
      if (err instanceof Error && err.message === "flow evaluate timed out") {
        this.restartWorker("flow worker restarted after evaluate timeout");
      }
      throw err;
    } finally {
      if (timeoutId !== undefined) clearTimeout(timeoutId);
    }
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
