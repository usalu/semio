/** 📄️ The Directory event-page bootstrap: page delivery, Home acknowledgement, the canonical event-page route and the declared session routes. */
import type { BackboneWorkerTestDependencies } from "../../../🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1 } from "../../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";

type BackboneWorkerRequest = import("../../../../🟦️.ts").BackboneWorkerRequest;
type BackboneWorkerResponse = import("../../../../🟦️.ts").BackboneWorkerResponse;
type CanonicalDirectoryEventPageV1 = import("../../../../🟦️.ts").CanonicalDirectoryEventPageV1;
type DirectoryEventPageAckV1 = import("../../../../🟦️.ts").DirectoryEventPageAckV1;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerDirectoryEventPageBootstrapTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  const { DirectoryEventPageBootstrapV1, decodeBackboneWorkerRequest, encodeBackboneWorkerRequest, decodeBackboneWorkerResponse, encodeBackboneWorkerResponse, browserDirectoryRequest, dispatchBackboneWorkerRequest } = dependencies;
  describe("DirectoryEventPageBootstrapV1", () => {
    beforeEach(resetWorkerLaw);
    const page = (afterSeqExclusive: number, throughSeqInclusive: number, hasMore: boolean, receiptSha256: string): CanonicalDirectoryEventPageV1 => ({
      canonicalJson: "{}",
      sessionBindingSha256: "a".repeat(64),
      authorizationGeneration: 9,
      afterSeqExclusive,
      throughSeqInclusive,
      hasMore,
      receiptSha256,
    });
    const ack = (value: CanonicalDirectoryEventPageV1, bootstrapEpoch = 7): DirectoryEventPageAckV1 => ({
      bootstrapEpoch,
      sessionBindingSha256: value.sessionBindingSha256,
      authorizationGeneration: value.authorizationGeneration,
      throughSeqInclusive: value.throughSeqInclusive,
      receiptSha256: value.receiptSha256,
    });

    it("serializes page delivery, exact Home acknowledgement, and live wakeup cursor ownership", () => {
      const machine = new DirectoryEventPageBootstrapV1(7, 3);
      const first = page(3, 5, true, "b".repeat(64));
      const second = page(5, 8, false, "c".repeat(64));
      machine.present(first);
      expect(() => machine.present(second)).toThrow("page ordering mismatch");
      expect(() => machine.acknowledge({ ...ack(first), receiptSha256: "d".repeat(64) })).toThrow("acknowledgement mismatch");
      expect(machine.after()).toBe(3);
      expect(machine.acknowledge(ack(first))).toEqual({ kind: "fetch", after: 5 });
      machine.present(second);
      expect(() => machine.acknowledge(ack(second, 8))).toThrow("acknowledgement mismatch");
      expect(machine.acknowledge(ack(second))).toEqual({ kind: "live", since: 8 });
      expect(machine.wake(false)).toBe(8);
      expect(machine.wake(false)).toBeNull();
    });

    it("round trips exact worker ACK and page envelopes without a raw identity secret", () => {
      const first = page(3, 5, true, "b".repeat(64));
      const request: BackboneWorkerRequest = { kind: "directory-bootstrap-ack", ...ack(first) };
      const response: BackboneWorkerResponse = { kind: "directory-event-page", ...ack(first), canonicalJson: first.canonicalJson, afterSeqExclusive: first.afterSeqExclusive, hasMore: first.hasMore };
      expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request))).toEqual(request);
      expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response))).toEqual(response);
      expect(JSON.stringify(response)).not.toContain("session.v1.");
    });

    it("allowlists only one canonical safe-decimal event-page route and keeps bootstrap on the TypeScript owner", async () => {
      const originalFetch = globalThis.fetch;
      const urls: string[] = [];
      (globalThis as unknown as { fetch: unknown }).fetch = async (input: string) => {
        urls.push(input);
        return new Response("{}", { status: 200 });
      };
      try {
        await browserDirectoryRequest("/directory/event-page/v1?after=3", {}, { timeoutMs: 1_000 });
        await expect(browserDirectoryRequest("/directory/event-page/v1?after=03", {}, { timeoutMs: 1_000 })).rejects.toThrow("directory operation denied");
        await expect(browserDirectoryRequest("/directory/event-page/v1?after=9007199254740992", {}, { timeoutMs: 1_000 })).rejects.toThrow("directory operation denied");
        expect(urls).toEqual(["/_semio/hub/directory/event-page/v1?after=3"]);
        const typescriptRequests: BackboneWorkerRequest[] = [];
        const rustRequests: BackboneWorkerRequest[] = [];
        const request: BackboneWorkerRequest = { kind: "directory-bootstrap-open", baseUrl: "http://hub.test", after: 3, bootstrapEpoch: 7 };
        dispatchBackboneWorkerRequest(request, { handleRequestBytes: (wire) => rustRequests.push(decodeBackboneWorkerRequest(wire)), postReady() {} }, (value) => typescriptRequests.push(value));
        expect(typescriptRequests).toEqual([request]);
        expect(rustRequests).toEqual([]);
      } finally {
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });

    it("admits exactly the declared session routes — the preference page included — equal to a path-to-regexp + WHATWG URL oracle", async () => {
      const { readFileSync } = await import("node:fs");
      const { match } = await import("path-to-regexp");
      const corpus = JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/📇️directory/🚦️browser-directory-routes.json", source.url), "utf8")) as {
        readonly routes: readonly { readonly method: string; readonly pattern: string; readonly query: { readonly name: string; readonly grammar: "digits" | "safe-decimal" } | null }[];
        readonly requests: readonly { readonly method: string; readonly path: string; readonly allowed: boolean }[];
      };
      const oracle = (method: string, path: string): boolean => {
        const url = new URL(path, "http://oracle.invalid");
        const parameters = [...url.searchParams];
        return corpus.routes.some((route) => {
          if (route.method !== method || match(route.pattern)(url.pathname) === false) return false;
          if (route.query === null) return url.search === "";
          if (parameters.length !== 1 || parameters[0]![0] !== route.query.name) return false;
          const value = parameters[0]![1];
          return route.query.grammar === "digits" ? /^\d+$/u.test(value) : /^(?:0|[1-9]\d*)$/u.test(value) && Number.isSafeInteger(Number(value));
        });
      };
      const originalFetch = globalThis.fetch;
      const urls: string[] = [];
      (globalThis as unknown as { fetch: unknown }).fetch = async (input: string) => {
        urls.push(input);
        return new Response("{}", { status: 200 });
      };
      try {
        const admitted: string[] = [];
        for (const row of corpus.requests) {
          const outcome = await browserDirectoryRequest(row.path, { method: row.method }, { timeoutMs: 1_000 }).then(
            () => true,
            (error: unknown) => {
              expect((error as Error).message, row.path).toBe("browser directory operation denied");
              return false;
            },
          );
          expect(outcome, `${row.method} ${row.path}`).toBe(row.allowed);
          expect(oracle(row.method, row.path), `${row.method} ${row.path}: oracle`).toBe(row.allowed);
          if (row.allowed) admitted.push(`/_semio/hub${row.path}`);
        }
        expect(urls).toEqual(admitted);
        expect(corpus.requests.some((row) => row.allowed && row.path.startsWith("/directory/preference-page/v1"))).toBe(true);
      } finally {
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });
  });
}
