/** 🔑️ The Shell's private session-broker port: acknowledged reads, the closed proof, and serial refresh of session authority through the actual worker broker. */
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { stubFetch } from "../../../../🧪️tests/🌐️fetch-stub/🟦️.ts";
import type { BackboneWorkerTestDependencies } from "../../../🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1, WORKER_LAW_CAPABILITY, WORKER_LAW_CAPABILITY_TWO } from "../../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";

type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerSessionBrokerPortTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, clearHubSessionCapability } = dependencies;
  vitest.it("binds acknowledged Shell session reads through the actual worker broker", async () => {
    resetWorkerLaw();
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const { HubSessionPortClientV1 } = await import("@semio-tech/framework/directory/session-refresh/session-port");
    const directory = "./🔨️modules/📇️directory/";
    const corpus = JSON.parse(readFileSync(new URL(directory + "🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL(directory + "🧬️schema/🪪️session-authority-v1/🧬️.schema.json", source.url), "utf8"));
    const validate = semioSchemaAjvV1({ strict: true }).compile(schema);
    const authorities = corpus.rows.filter((row: { accepted: boolean }) => row.accepted).map((row: { value: unknown }) => row.value);
    const originalFetch = globalThis.fetch;
    const channel = new MessageChannel();
    const calls: Array<{ path: string; bearer: string }> = [];
    clearHubSessionCapability();
    testSeams.attachHubSessionPort(channel.port2);
    const client = new HubSessionPortClientV1(channel.port1, WORKER_LAW_CAPABILITY);
    try {
      for (const authority of authorities) {
        vitest.expect(validate(authority)).toBe(true);
        const body = JSON.stringify(authority);
        globalThis.fetch = stubFetch(vitest.vi.fn(async (input: RequestInfo | URL, init?: RequestInit) => {
          const headers = new Headers(init?.headers);
          calls.push({ path: String(input), bearer: headers.get("authorization") ?? "" });
          return new Response(body, { status: 200, headers: { "content-length": String(new TextEncoder().encode(body).byteLength) } });
        }));
        const response = await client.me();
        vitest.expect(equal(response, { status: 200, body })).toBe(true);
        vitest.expect(equal(testSeams.browserSessionAuthority, authority)).toBe(true);
      }
      vitest.expect(equal(calls, authorities.map(() => ({ path: "/_semio/hub/auth/sessions/me", bearer: `Bearer ${WORKER_LAW_CAPABILITY}` })))).toBe(true);
    } finally {
      client.close();
      testSeams.detachHubSessionPort();
      channel.port1.close();
      channel.port2.close();
      clearHubSessionCapability();
      globalThis.fetch = originalFetch;
    }
  });

  vitest.it("requires an acknowledged private broker port before Shell session requests", async () => {
    resetWorkerLaw();
    const { readFileSync } = await import("node:fs");
    const { setImmediate: immediate } = await import("node:timers/promises");
    const { default: equal } = await import("fast-deep-equal");
    const directory = "./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-port/";
    const fixture = JSON.parse(readFileSync(new URL(directory + "🔣️.json", source.url), "utf8"));
    const { HubSessionPortClientV1, HUB_SESSION_PORT_CLIENT_TIMEOUT_MS, HUB_SESSION_PORT_CLIENT_INITIALIZATION_TIMEOUT_MS, HUB_SESSION_PORT_CLIENT_MAX_PENDING } = await import("@semio-tech/framework/directory/session-refresh/session-port");
    vitest.expect(equal([HUB_SESSION_PORT_CLIENT_TIMEOUT_MS, HUB_SESSION_PORT_CLIENT_MAX_PENDING], [fixture.timeoutMs, fixture.maximumPending])).toBe(true);
    vitest.expect(HUB_SESSION_PORT_CLIENT_INITIALIZATION_TIMEOUT_MS).toBe(fixture.initializationTimeoutMs);
    vitest.expect(fixture.initializationTimeoutMs + fixture.timeoutMs * 2).toBeLessThan(fixture.bootstrapTimeoutMs);
    vitest.vi.useFakeTimers();
    try {
      for (const row of fixture.cases) {
        const channel = new MessageChannel();
        const requests: Array<Record<string, any>> = [];
        let initialized!: () => void;
        const initialization = new Promise<void>((resolve) => { initialized = resolve; });
        let requested!: () => void;
        const request = new Promise<void>((resolve) => { requested = resolve; });
        channel.port2.onmessage = (event: MessageEvent<Record<string, any>>) => {
          requests.push(event.data);
          if (event.data.kind === "initialize") initialized();
          if (event.data.kind === "request") {
            requested();
            if (row !== "close-before-response") channel.port2.postMessage({ kind: "response", requestId: event.data.requestId, ...fixture.accepted });
          }
        };
        channel.port2.start();
        const client = new HubSessionPortClientV1(channel.port1, WORKER_LAW_CAPABILITY);
        const abort = new AbortController();
        const response = client.me(abort.signal).then((value) => ({ kind: "accepted", value }), () => ({ kind: "refused" }));
        try {
          await initialization;
          vitest.expect(requests.filter((value) => value.kind === "request")).toHaveLength(0);
          if (row === "slow-initialization") await vitest.vi.advanceTimersByTimeAsync(fixture.timeoutMs + 1);
          if (row === "cancel-before-ack") abort.abort();
          if (row === "response-before-ack") channel.port2.postMessage({ kind: "response", requestId: "1".repeat(36), ...fixture.accepted });
          else if (row !== "timeout") channel.port2.postMessage(row === "malformed-ack" ? { kind: "initialized", ok: true, authority: "overpost" } : { kind: "initialized", ok: row !== "refused" });
          await immediate();
          if (row === "timeout" || row === "malformed-ack") await vitest.vi.advanceTimersByTimeAsync(fixture.initializationTimeoutMs);
          if (row === "close-before-response") {
            await request;
            client.close();
            const pending = requests.find((value) => value.kind === "request")!;
            channel.port2.postMessage({ kind: "response", requestId: pending.requestId, ...fixture.accepted });
          }
          const result = await response;
          vitest.expect(equal(result, row === "acknowledged" || row === "slow-initialization" ? { kind: "accepted", value: fixture.accepted } : { kind: "refused" })).toBe(true);
          vitest.expect(requests.filter((value) => value.kind === "request")).toHaveLength(Number(row === "acknowledged" || row === "slow-initialization" || row === "close-before-response"));
        } finally {
          client.close();
          channel.port1.close();
          channel.port2.close();
        }
      }
    } finally {
      vitest.vi.useRealTimers();
    }
  });

  vitest.it("closes the private broker proof before or during a session-authority read", async () => {
    resetWorkerLaw();
    const { readFileSync } = await import("node:fs");
    const { setImmediate: immediate } = await import("node:timers/promises");
    const { default: equal } = await import("fast-deep-equal");
    const directory = "./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-port/";
    const fixture = JSON.parse(readFileSync(new URL(directory + "🔣️.json", source.url), "utf8"));
    const { HubSessionPortClientV1 } = await import("@semio-tech/framework/directory/session-refresh/session-port");
    const originalFetch = globalThis.fetch;
    try {
      for (const row of fixture.close.cases) {
        clearHubSessionCapability();
        const channel = new MessageChannel();
        testSeams.attachHubSessionPort(channel.port2);
        let readStarted = false;
        let requestSignal: AbortSignal | undefined;
        let resolveStarted!: () => void;
        const started = new Promise<void>((resolve) => { resolveStarted = resolve; });
        globalThis.fetch = stubFetch(vitest.vi.fn(async (_input: RequestInfo | URL, init?: RequestInit) => {
          readStarted = true;
          requestSignal = init?.signal ?? undefined;
          resolveStarted();
          return new Promise<Response>((_resolve, reject) => requestSignal?.addEventListener("abort", () => reject(new Error("cancelled")), { once: true }));
        }));
        const client = new HubSessionPortClientV1(channel.port1, row.id === "before-read" ? WORKER_LAW_CAPABILITY : WORKER_LAW_CAPABILITY_TWO);
        try {
          if (row.id === "before-read") {
            client.close();
            await vitest.expect(client.me()).rejects.toThrow("hub session unavailable");
          } else {
            await immediate();
            const pending = client.me();
            await started;
            client.close();
            await vitest.expect(pending).rejects.toThrow("hub session closed");
          }
          await immediate();
          await immediate();
          vitest.expect(equal({ id: row.id, readStarted }, row)).toBe(true);
          vitest.expect(testSeams.browserSessionAuthority).toBe(null);
          if (row.readStarted) vitest.expect(requestSignal?.aborted).toBe(true);
        } finally {
          client.close();
          testSeams.detachHubSessionPort();
          channel.port1.close();
          channel.port2.close();
        }
      }
    } finally {
      clearHubSessionCapability();
      globalThis.fetch = originalFetch;
    }
  });

  vitest.it("refreshes Shell session authority serially and cancels stale callbacks", async () => {
    resetWorkerLaw();
    const { readFileSync } = await import("node:fs");
    const { default: equal } = await import("fast-deep-equal");
    const fixture = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-refresh/🔣️.json", source.url), "utf8"));
    const { directorySessionAuthorityIsCurrentV1, directorySessionAuthorityTextV1, directorySessionRefreshRetryDelayMsV1, startDirectorySessionRefreshV1, DIRECTORY_SESSION_REFRESH_INTERVAL_MS, DIRECTORY_SESSION_AUTHORITY_TEXT_V1 } = await import("@semio-tech/framework/directory/session-refresh");
    const corpus = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json", source.url), "utf8"));
    const first = corpus.rows[0].value;
    const second = corpus.rows[1].value;
    for (const row of fixture.actionAdmission) {
      const current = row.id === "missing" ? null : row.id === "different-binding" ? second : row.id === "different-generation" ? { ...first, authorizationGeneration: first.authorizationGeneration + 1 } : row.id === "expired" ? { ...first, expiresAt: Date.now() } : { ...first };
      vitest.expect(directorySessionAuthorityIsCurrentV1(first, current)).toBe(row.accepted);
      vitest.expect(directorySessionAuthorityIsCurrentV1(null, current)).toBe(false);
    }
    vitest.expect(DIRECTORY_SESSION_REFRESH_INTERVAL_MS).toBe(fixture.intervalMs);
    vitest.expect(equal(DIRECTORY_SESSION_AUTHORITY_TEXT_V1, fixture.presentation.text)).toBe(true);
    for (const locale of fixture.presentation.locales) vitest.expect(equal(directorySessionAuthorityTextV1(locale), fixture.presentation.text[locale])).toBe(true);
    vitest.expect(() => directorySessionAuthorityTextV1("fr")).toThrow("directory.session-authority.locale-unsupported");
    vitest.expect(fixture.intervalMs * 2).toBeLessThan(fixture.proofTtlMs);
    vitest.vi.useFakeTimers();
    const owners: Array<{ close(): void }> = [];
    try {
      const seen: unknown[] = [];
      const responses = [first, first, second];
      let calls = 0;
      let active = 0;
      let maximum = 0;
      let release: (() => void) | undefined;
      const firstGate = new Promise<void>((resolve) => { release = resolve; });
      const owner = startDirectorySessionRefreshV1({
        read: async () => {
          active += 1;
          maximum = Math.max(maximum, active);
          const value = responses[calls++];
          if (calls === 1) await firstGate;
          active -= 1;
          return { status: 200, body: JSON.stringify(value) };
        },
        onAuthority: (value) => { seen.push(value); },
        onUnavailable: () => { throw new Error("unexpected authority retirement"); },
      });
      owners.push(owner);
      const joined = owner.refresh();
      const alsoJoined = owner.refresh();
      await vitest.vi.advanceTimersByTimeAsync(fixture.intervalMs);
      vitest.expect(calls).toBe(1);
      release!();
      await Promise.all([joined, alsoJoined]);
      await vitest.vi.advanceTimersByTimeAsync(fixture.intervalMs);
      await vitest.vi.advanceTimersByTimeAsync(fixture.intervalMs);
      vitest.expect(equal(seen, responses)).toBe(true);
      vitest.expect(maximum).toBe(fixture.maximumConcurrentRequests);
      owner.close();
      await vitest.vi.advanceTimersByTimeAsync(fixture.proofTtlMs);
      vitest.expect(calls).toBe(3);
      for (const refusal of fixture.refusals) {
        const accepted: unknown[] = [];
        let refused = 0;
        let reads = 0;
        const failed = startDirectorySessionRefreshV1({
          read: async () => {
            reads += 1;
            const status = refusal === "http-401" ? 401 : refusal === "http-403" ? 403 : 200;
            return { status, body: JSON.stringify(refusal === "expired" ? { ...first, expiresAt: Date.now() } : first) };
          },
          onAuthority: (value) => { accepted.push(value); },
          onUnavailable: () => { refused += 1; },
        });
        owners.push(failed);
        await failed.refresh();
        await vitest.vi.advanceTimersByTimeAsync(fixture.proofTtlMs);
        await failed.refresh();
        vitest.expect(equal({ refusal, accepted, refused, reads }, { refusal, accepted: [], refused: 1, reads: 1 + fixture.unavailableRetries })).toBe(true);
      }
      // 🔁️ A failure the hub did not pronounce on keeps the session: the loop retries on the fixture's
      // own backoff, reports the degraded window's two edges exactly once each, and the authority that
      // was already observed is never retired. This is the law the live two-user run needed — the first
      // shell used to be signed out for ever by one queued `me` while a second context booted.
      for (const transient of fixture.transientFailures) {
        const accepted: unknown[] = [];
        const degradedEdges: boolean[] = [];
        let refused = 0;
        let reads = 0;
        let failing = false;
        const riding = startDirectorySessionRefreshV1({
          read: async () => {
            reads += 1;
            if (!failing) return { status: 200, body: JSON.stringify({ ...first, expiresAt: Date.now() + fixture.proofTtlMs * 100 }) };
            if (transient === "transport") throw new Error("offline");
            const status = transient === "http-428" ? 428 : transient === "http-503" ? 503 : transient === "non-200" ? 201 : 200;
            return { status, body: transient === "invalid-body" ? "<html>proxy error</html>" : JSON.stringify({ ...first, expiresAt: Date.now() + fixture.proofTtlMs * 100 }) };
          },
          onAuthority: (value) => { accepted.push(value); },
          onUnavailable: () => { refused += 1; },
          onDegraded: (value) => { degradedEdges.push(value); },
        });
        owners.push(riding);
        await riding.refresh();
        vitest.expect(accepted.length).toBe(1);
        failing = true;
        const readsBeforeLoss = reads;
        for (const [index, delay] of fixture.retryDelaysMs.entries()) {
          await vitest.vi.advanceTimersByTimeAsync(index === 0 ? fixture.intervalMs : 0);
          await vitest.vi.advanceTimersByTimeAsync(delay);
          vitest.expect(reads, `${transient} retry ${index}`).toBe(readsBeforeLoss + index + 2);
        }
        vitest.expect(equal({ transient, refused, accepted: accepted.length, degradedEdges }, { transient, refused: 0, accepted: 1, degradedEdges: [true] })).toBe(true);
        failing = false;
        await vitest.vi.advanceTimersByTimeAsync(fixture.retryDelaysMs[fixture.retryDelaysMs.length - 1]);
        vitest.expect(equal({ transient, refused, accepted: accepted.length, degradedEdges }, { transient, refused: 0, accepted: 2, degradedEdges: [true, false] })).toBe(true);
        riding.close();
      }
      // ⌛️ Riding out a loss is bounded by the session's OWN deadline: once the last observed authority
      // has expired there is nothing left to ride out, and the next transient failure retires it.
      {
        let refused = 0;
        let expired = false;
        const expiring = startDirectorySessionRefreshV1({
          read: async () => {
            if (expired) throw new Error("offline");
            return { status: 200, body: JSON.stringify({ ...first, expiresAt: Date.now() + fixture.intervalMs }) };
          },
          onAuthority: () => undefined,
          onUnavailable: () => { refused += 1; },
        });
        owners.push(expiring);
        await expiring.refresh();
        expired = true;
        await vitest.vi.advanceTimersByTimeAsync(fixture.intervalMs * 2);
        vitest.expect(refused).toBe(1);
      }
      for (const [attempt, delay] of fixture.retryDelaysMs.entries()) vitest.expect(directorySessionRefreshRetryDelayMsV1(attempt + 1)).toBe(delay);
      vitest.expect(() => directorySessionRefreshRetryDelayMsV1(0)).toThrow("directory.session-authority.retry-attempt-invalid");
      for (const cancellation of fixture.cancellation) {
        const parent = new AbortController();
        let releaseRead: (() => void) | undefined;
        let readStarted!: () => void;
        const started = new Promise<void>((resolve) => { readStarted = resolve; });
        let requestSignal: AbortSignal | undefined;
        let accepted = 0;
        let refused = 0;
        let calls = 0;
        const gate = new Promise<void>((resolve) => { releaseRead = resolve; });
        const cancelled = startDirectorySessionRefreshV1({
          signal: parent.signal,
          read: async (signal) => {
            calls += 1;
            requestSignal = signal;
            readStarted();
            if (cancellation === "during-request") await gate;
            return { status: 200, body: JSON.stringify(first) };
          },
          onAuthority: () => { accepted += 1; },
          onUnavailable: () => { refused += 1; },
        });
        owners.push(cancelled);
        const pending = cancelled.refresh();
        if (cancellation === "during-request") await started;
        if (cancellation === "during-delay") await pending;
        parent.abort();
        releaseRead!();
        await pending;
        await vitest.vi.advanceTimersByTimeAsync(fixture.proofTtlMs);
        vitest.expect(requestSignal?.aborted).toBe(cancellation === "before-request" ? undefined : true);
        vitest.expect(equal({ calls, accepted, refused }, { calls: Number(cancellation !== "before-request"), accepted: Number(cancellation === "during-delay"), refused: 0 })).toBe(true);
      }
    } finally {
      for (const owner of owners) owner.close();
      vitest.vi.useRealTimers();
    }
  });
}
