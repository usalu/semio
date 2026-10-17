/** 🛤️ The worker's Directory command lane: sealed retries, bounded transport, ordered drains and the scoped stream. */
import type { BackboneWorkerTestDependencies } from "../../../🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1, FakeDirectoryWebSocket } from "../../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";

type DirectoryCommand = import("@semio-tech/framework/directory/schema").DirectoryCommand;
type DirectoryCommandOutcomeV1 = import("@semio-tech/framework/directory/schema").DirectoryCommandOutcomeV1;
type DirectoryCommandRequestV1 = import("@semio-tech/framework/directory/schema").DirectoryCommandRequestV1;
type DirectoryCommandResultV1 = import("@semio-tech/framework/directory/schema").DirectoryCommandResultV1;
type DirectoryCommandTransportOperationV1 = import("../../../🏪️store/👷️worker/🟦️.ts").DirectoryCommandTransportOperationV1;
type FetchTimeoutResponse = import("@semio-tech/framework/io/fetch-timeout").FetchTimeoutResponse;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerDirectoryLaneTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it, vi } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, sealDirectoryCommandReceiptV1, directoryCommandSha256, DirectoryClient, handleTsRequest, directoryCommandQueue, flushDirectoryQueue, directoryCommandOperations, closeDirectory, DIRECTORY_COMMAND_TRANSPORT_CAPACITY, sealDirectoryCommandRequestV1, directoryWorkerEpoch, settleDirectoryCommand } = dependencies;
  //#region 🔖️DirectoryLaneTests
  describe("backbone-worker directory lane", () => {

    beforeEach(resetWorkerLaw);

    async function flushMicrotasks(): Promise<void> {
      await new Promise((resolve) => setTimeout(resolve, 0));
      await new Promise((resolve) => setTimeout(resolve, 0));
    }

    const commandRequestId = (index: number): string => index.toString(16).padStart(32, "b");

    const sampleCommand: DirectoryCommand = { kind: "create-space", name: "Atelier", spaceKind: "atelier", visibility: "private" };

    const inviteCommand = (spaceId: string): DirectoryCommand => ({ kind: "create-invite", spaceId, role: "spectator", ttlSecs: 3600 });

    async function commandReceiptBody(request: DirectoryCommandRequestV1, outcome: DirectoryCommandOutcomeV1 = "accepted", result: DirectoryCommandResultV1 = { kind: "none" }): Promise<string> {
      return JSON.stringify(await sealDirectoryCommandReceiptV1(request.requestId, await directoryCommandSha256(request.command), outcome, [], result));
    }

    /** ⏳️ Drains enough macrotask turns for every in-flight command turn (fetch, response text, and
     * two SHA-256 digests) to settle, so a transport assertion never races the receipt parser. */
    async function settleDirectoryTransport(): Promise<void> {
      for (let turn = 0; turn < 8; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
    }

    /** 🧪️ Installs a directory client whose transport is a recorded stub, isolating the command
     * transport owner from the browser broker's own proof ratchet (proved separately above). */
    function installFakeDirectoryClient(respond: (request: DirectoryCommandRequestV1) => Promise<{ status: number; body: string }> | { status: number; body: string }): string[] {
      const bodies: string[] = [];
      testSeams.directoryClient = new DirectoryClient("http://hub.test", {
        request: async (_input: string, init: RequestInit = {}) => {
          const body = String(init.body ?? "");
          bodies.push(body);
          const response = await respond(JSON.parse(body) as DirectoryCommandRequestV1);
          return { ok: response.status >= 200 && response.status < 300, status: response.status, text: async () => response.body } as unknown as FetchTimeoutResponse;
        },
      });
      testSeams.directorySessionEpoch += 1;
      return bodies;
    }

    it("retries only a transient fault with the byte-identical sealed request and never retries a terminal rejection", async () => {
      let status = 503;
      const bodies = installFakeDirectoryClient(async (request) => (status === 202 ? { status, body: await commandReceiptBody(request) } : { status, body: "hub text the client must never echo" }));
      try {
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(1), command: sampleCommand });
        await settleDirectoryTransport();
        expect(bodies).toHaveLength(1);
        expect(directoryCommandQueue).toHaveLength(1);
        status = 202;
        await flushDirectoryQueue();
        expect(bodies).toHaveLength(2);
        expect(bodies[0]).toBe(bodies[1]);
        expect(bodies[0]).toBe(JSON.stringify({ schema: "semio.directory.command-request.v1", requestId: commandRequestId(1), command: sampleCommand }));
        expect(directoryCommandQueue).toHaveLength(0);
        expect(directoryCommandOperations.size).toBe(0);

        for (const [index, terminal] of [401, 403, 409, 413].entries()) {
          status = terminal;
          bodies.length = 0;
          handleTsRequest({ kind: "directory-command", requestId: commandRequestId(10 + index), command: sampleCommand });
          await settleDirectoryTransport();
          expect(bodies).toHaveLength(1);
          expect(directoryCommandQueue).toHaveLength(0);
          expect(directoryCommandOperations.size).toBe(0);
        }
      } finally {
        closeDirectory();
      }
    });

    it("bounds the transport at a fixed capacity, keeps the oldest intent, and terminates a malformed correlation", async () => {
      installFakeDirectoryClient(() => {
        throw new Error("network unreachable");
      });
      try {
        for (let index = 0; index < DIRECTORY_COMMAND_TRANSPORT_CAPACITY; index += 1) {
          handleTsRequest({ kind: "directory-command", requestId: commandRequestId(100 + index), command: sampleCommand });
        }
        await settleDirectoryTransport();
        expect(directoryCommandOperations.size).toBe(DIRECTORY_COMMAND_TRANSPORT_CAPACITY);
        expect(directoryCommandQueue).toHaveLength(DIRECTORY_COMMAND_TRANSPORT_CAPACITY);
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(9999), command: sampleCommand });
        await settleDirectoryTransport();
        expect(directoryCommandOperations.size).toBe(DIRECTORY_COMMAND_TRANSPORT_CAPACITY);
        expect(directoryCommandOperations.has(commandRequestId(100))).toBe(true);
        expect(directoryCommandOperations.has(commandRequestId(9999))).toBe(false);
        expect(directoryCommandQueue[0]!.request.requestId).toBe(commandRequestId(100));

        handleTsRequest({ kind: "directory-command", requestId: "r1", command: sampleCommand });
        await settleDirectoryTransport();
        expect(directoryCommandOperations.has("r1")).toBe(false);

        handleTsRequest({ kind: "directory-command-cancel", requestId: commandRequestId(100) });
        expect(directoryCommandOperations.size).toBe(DIRECTORY_COMMAND_TRANSPORT_CAPACITY - 1);
        expect(directoryCommandOperations.has(commandRequestId(100))).toBe(false);
        closeDirectory();
        expect(directoryCommandOperations.size).toBe(0);
        expect(directoryCommandQueue).toHaveLength(0);
      } finally {
        closeDirectory();
      }
    });

    it("stops the queue at a transient head, then drains it in order without a capability reaching any log", async () => {
      const token = "inv.01920000000070008000000000000001.dGhpcy1jYXBhYmlsaXR5LW5ldmVyLXJlYWNoZXMtYS1sb2c";
      const errorSpy = vi.spyOn(console, "error").mockImplementation(() => {});
      let head: "transient" | "accepted" = "transient";
      const bodies = installFakeDirectoryClient(async (request) => {
        if (request.requestId === commandRequestId(200) && head === "transient") return { status: 503, body: "" };
        return { status: 202, body: await commandReceiptBody(request, "accepted", request.command.kind === "create-invite" ? { kind: "invite", inviteToken: token } : { kind: "none" }) };
      });
      try {
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(200), command: inviteCommand("space-a") });
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(201), command: sampleCommand });
        await settleDirectoryTransport();
        expect(bodies).toHaveLength(1);
        expect(directoryCommandQueue.map((operation) => operation.request.requestId)).toEqual([commandRequestId(200), commandRequestId(201)]);

        head = "accepted";
        await flushDirectoryQueue();
        expect(bodies).toHaveLength(3);
        expect(directoryCommandQueue).toHaveLength(0);
        expect(directoryCommandOperations.size).toBe(0);
        expect(JSON.stringify([...directoryCommandOperations.values(), ...directoryCommandQueue])).not.toContain(token);
        expect(
          errorSpy.mock.calls
            .flat()
            .map((entry) => String(entry))
            .join("|"),
        ).not.toContain(token);
      } finally {
        errorSpy.mockRestore();
        closeDirectory();
      }
    });

    it("suppresses delivery for an operation whose session epoch was replaced and settles each operation exactly once", async () => {
      const bodies = installFakeDirectoryClient(async (request) => ({ status: 202, body: await commandReceiptBody(request) }));
      try {
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(300), command: sampleCommand });
        await settleDirectoryTransport();
        expect(bodies).toHaveLength(1);
        expect(directoryCommandOperations.size).toBe(0);

        const stale: DirectoryCommandTransportOperationV1 = {
          request: sealDirectoryCommandRequestV1(commandRequestId(301), sampleCommand),
          abort: new AbortController(),
          sessionEpoch: testSeams.directorySessionEpoch - 1,
          workerEpoch: directoryWorkerEpoch,
          settled: false,
        };
        directoryCommandOperations.set(commandRequestId(301), stale);
        directoryCommandQueue.push(stale);
        await flushDirectoryQueue();
        expect(bodies).toHaveLength(1);
        expect(directoryCommandOperations.has(commandRequestId(301))).toBe(false);
        expect(directoryCommandQueue).toHaveLength(0);
        expect(stale.settled).toBe(true);

        settleDirectoryCommand(stale, { kind: "directory-command-failed", requestId: commandRequestId(301), code: "closed" });
        expect(directoryCommandOperations.size).toBe(0);
        expect(directoryCommandQueue).toHaveLength(0);
      } finally {
        closeDirectory();
      }
    });

    it("queues a directory command while the hub is unreachable, then flushes it in order on the next live signal", async () => {
      FakeDirectoryWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      let reachable = false;
      try {
        handleTsRequest({ kind: "directory-open", baseUrl: "http://hub.test", since: 0 });
        const bodies = installFakeDirectoryClient(async (request) => {
          if (!reachable) throw new Error("network unreachable");
          return { status: 202, body: await commandReceiptBody(request) };
        });
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(1), command: sampleCommand });
        await settleDirectoryTransport();
        expect(bodies).toHaveLength(1);
        expect(directoryCommandQueue).toHaveLength(1);
        expect(directoryCommandQueue[0]!.request.requestId).toBe(commandRequestId(1));

        // 🟢️ Hub becomes reachable — any live signal on the stream (a heartbeat here) triggers a flush.
        reachable = true;
        FakeDirectoryWebSocket.instances.at(-1)!.triggerMessage({ kind: "heartbeat", headSeq: 0 });
        await settleDirectoryTransport();
        expect(bodies).toHaveLength(2);
        expect(directoryCommandQueue).toHaveLength(0);
        expect(directoryCommandOperations.size).toBe(0);
      } finally {
        // 🧹️ Restores the real global — an un-restored `FakeDirectoryWebSocket` (no `OPEN` static)
        // previously leaked into every later test's `WebSocket.OPEN` comparisons, silently making
        // `relayMutationsToHub`/`sendWireFrame`'s "is the socket actually open" checks pass when
        // `state.socket` was `null`.
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        closeDirectory();
      }
    });

  });
  //#endregion 🔖️DirectoryLaneTests
}
