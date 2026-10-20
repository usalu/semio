/** 🔶️ The before phase of registerSessionBrokerPortTests. */
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { stubFetch } from "../../../../../🧪️tests/🌐️fetch-stub/🟦️.ts";
import type { BackboneWorkerTestDependencies } from "../../../../🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1, WORKER_LAW_CAPABILITY } from "../../../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../../../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";
import type { Vitest } from "../🟢️shared/🟦️.ts";

/** 🔷 Builds beforeEach, resetWorkerLaw, testSeams, clearHubSessionCapability of registerSessionBrokerPortTests. */
export function useBeforeState({ vitest, dependencies, source }: { readonly vitest: Vitest; readonly dependencies: BackboneWorkerTestDependencies; readonly source: TestSource }) {
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
  return { resetWorkerLaw, clearHubSessionCapability, testSeams };
}
