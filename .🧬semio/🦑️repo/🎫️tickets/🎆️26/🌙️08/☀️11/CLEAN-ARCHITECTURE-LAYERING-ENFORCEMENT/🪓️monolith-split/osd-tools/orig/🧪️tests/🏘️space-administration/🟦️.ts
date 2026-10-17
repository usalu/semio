/** 🏘️ Space administration through the worker: verified transport, page reads, sealed command receipts, invites and denial handling. */
import type { BackboneWorkerTestDependencies } from "../../../🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1 } from "../../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";

type BackboneWorkerResponse = import("../../../../🟦️.ts").BackboneWorkerResponse;
type DirectoryCommand = import("@semio-tech/framework/directory/schema").DirectoryCommand;
type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerSpaceAdministrationTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it, vi } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  const { testSeams, DirectoryClient, handleTsRequest, closeDirectory, revokeDirectoryAdministrationForScope } = dependencies;
  //#region 🔖️DirectoryLaneTests
  describe("backbone-worker space administration", () => {
    beforeEach(resetWorkerLaw);
    const SPACE = "space-admin-01";

    /** ⏳️ Waits for actual operation completion, including asynchronous page and receipt verification. */
    const settleAdministrationTurns = async (_harness?: { readonly posted: readonly unknown[] }): Promise<void> => {
      await new Promise((resolve) => setTimeout(resolve, 0));
      await vi.waitFor(() => {
        const operation = testSeams.directoryAdministration;
        if (operation === null) return;
        expect(operation.phase).toBe("ready");
        expect(operation.requestId).toBeNull();
        expect(operation.pageRead).toBeNull();
      }, { timeout: 2000, interval: 5 });
      await new Promise((resolve) => setTimeout(resolve, 0));
    };

    async function sealAdministrationPage(members: readonly { userId: string; email: string; role: "author" | "spectator"; owner: boolean }[], invites: readonly { inviteId: string; createdAtMs: number }[], properties: { name?: string; visibility?: "public" | "private"; capabilities?: Readonly<Record<string, boolean>> } = {}): Promise<string> {
      const unsigned = {
        access: "author" as const,
        schema: "semio.directory.space-administration-page.v1" as const,
        sessionBindingSha256: "a".repeat(64),
        authorizationGeneration: 5,
        spaceId: SPACE,
        space: { id: SPACE, name: properties.name ?? "Administered", kind: "studio", visibility: properties.visibility ?? "private", ownerUserId: "user-a", role: "author", memberCount: members.length, documentCount: 0, activeConnections: 0, createdAtMs: 1, updatedAtMs: 2 },
        members: { rows: members.map((row) => ({ userId: row.userId, email: row.email, displayName: row.userId, role: row.role, owner: row.owner })) },
        documents: { rows: [] as unknown[] },
        invites: { rows: invites.map((row) => ({ inviteId: row.inviteId, role: "spectator" as const, createdAtMs: row.createdAtMs, expiresAtMs: 900000, revoked: false, accepted: false })) },
        capabilities: { renameSpace: true, setVisibility: true, deleteSpace: true, upsertMember: true, removeMember: true, createInvite: true, revokeInvite: true, ...properties.capabilities },
      };
      const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(JSON.stringify(unsigned))));
      return JSON.stringify({ ...unsigned, receiptSha256: Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("") });
    }

    async function sealMemberAdministrationPage(): Promise<string> {
      const unsigned = {
        access: "member" as const,
        schema: "semio.directory.space-administration-page.v1" as const,
        sessionBindingSha256: "a".repeat(64),
        authorizationGeneration: 6,
        spaceId: SPACE,
        space: { id: SPACE, name: "Administered", kind: "studio", visibility: "private", ownerUserId: "user-a", role: "spectator", memberCount: 2, documentCount: 0, activeConnections: 0, createdAtMs: 1, updatedAtMs: 3 },
        members: { rows: [{ userId: "user-b", email: "b@example.invalid", displayName: "user-b", role: "spectator" as const, owner: false }] },
        documents: { rows: [] as unknown[] },
      };
      const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(JSON.stringify(unsigned))));
      return JSON.stringify({ ...unsigned, receiptSha256: Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("") });
    }

    async function sealCommandReceipt(requestId: string, command: DirectoryCommand, inviteToken?: string, outcome: "accepted" | "previously-accepted" | "secret-undeliverable" = "accepted"): Promise<string> {
      const commandSha256 = await directorySha256(JSON.stringify(command));
      const unsigned = {
        schema: "semio.directory.command-receipt.v1" as const,
        requestId,
        commandSha256,
        outcome,
        events: [] as unknown[],
        result: inviteToken === undefined ? { kind: "none" as const } : { kind: "invite" as const, inviteToken },
      };
      const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(JSON.stringify(unsigned))));
      return JSON.stringify({ ...unsigned, receiptSha256: Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("") });
    }

    async function directorySha256(text: string): Promise<string> {
      const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(text)));
      return Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("");
    }

    function administrationHarness(pageStatuses: readonly number[], receiptStatus = 200) {
      const posted: BackboneWorkerResponse[] = [];
      const original = testSeams.workerPostTestSink;
      testSeams.workerPostTestSink = (message) => posted.push(message);
      const requests: string[] = [];
      let pageIndex = 0;
      const bodies: string[] = [];
      const fetches = vi.fn(async (input: string, init?: { method?: string }) => {
        requests.push(`${init?.method ?? "GET"} ${input}`);
        if ((init?.method ?? "GET") === "POST") {
          return { ok: receiptStatus < 400, status: receiptStatus, statusText: "", headers: { get: () => "application/json" }, json: async () => ({}), text: async () => bodies.shift() ?? "" };
        }
        const status = pageStatuses[Math.min(pageIndex, pageStatuses.length - 1)] ?? 200;
        pageIndex += 1;
        return { ok: status < 400, status, statusText: "", headers: { get: () => "application/json" }, json: async () => ({}), text: async () => bodies.shift() ?? "" };
      });
      return {
        posted,
        requests,
        bodies,
        fetches,
        release: () => {
          testSeams.workerPostTestSink = original;
        },
      };
    }

    it("admits administration transport only for the verified page's exact space and command capability", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const base = "./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/";
      const fixture = JSON.parse(readFileSync(new URL(base + "🏛️administration/🛂️command-admission/🔣️.json", source.url), "utf8"));
      const members = [{ userId: "user-a", email: "a@example.invalid", role: "author" as const, owner: true }];
      let epoch = 100;
      for (const row of fixture.allowed as Array<{ capability: string; command: DirectoryCommand }>) {
        for (const refusal of ["foreign-command", "withdrawn-capability", "member-page", "missing-page"]) {
          const harness = administrationHarness([200]);
          try {
            testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
            harness.bodies.push(refusal === "member-page" ? await sealMemberAdministrationPage() : await sealAdministrationPage(members, [], { capabilities: refusal === "withdrawn-capability" ? { [row.capability]: false } : {} }));
            handleTsRequest({ kind: "directory-administration-open", operationEpoch: ++epoch, spaceId: SPACE });
            if (refusal !== "missing-page") await settleAdministrationTurns(harness);
            const command = refusal === "foreign-command" ? { ...row.command, spaceId: "another-space" } : row.command;
            handleTsRequest({ kind: "directory-administration-submit", operationEpoch: epoch, requestId: "1".repeat(32), command });
            await settleAdministrationTurns(harness);
            expect(equal(harness.requests.filter((entry) => entry.startsWith("POST")), []), row.command.kind + ":" + refusal).toBe(true);
            expect(harness.posted.some((message) => message.kind === "directory-administration-state" && message.phase === "submitting")).toBe(false);
            expect(testSeams.directoryAdministration?.requestId).toBeNull();
          } finally {
            harness.release();
            closeDirectory();
          }
        }
      }
      const harness = administrationHarness([200]);
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(await sealAdministrationPage(members, []));
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: ++epoch, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        for (const command of [...fixture.unrelated, ...fixture.malformed]) {
          handleTsRequest({ kind: "directory-administration-submit", operationEpoch: epoch, requestId: "1".repeat(32), command });
          await settleAdministrationTurns(harness);
          expect(equal(harness.requests.filter((entry) => entry.startsWith("POST")), [])).toBe(true);
          expect(testSeams.directoryAdministration?.phase).toBe("ready");
        }
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("retires older administration page reads and refuses refresh during a sealed command", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const base = "./🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🏛️administration/📄️page-retirement";
      const fixture = JSON.parse(readFileSync(new URL(base + "/🔣️.json", source.url), "utf8"));
      
      
      const author = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], []);
      const member = await sealMemberAdministrationPage();
      for (const row of fixture.cases) {
        const harness = administrationHarness([200]);
        const pending: Array<(value: Response) => void> = [];
        let reads = 0;
        try {
          testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: async () => ++reads === 1 ? new Response(author) : await new Promise<Response>((resolve) => pending.push(resolve)) });
          handleTsRequest({ kind: "directory-administration-open", operationEpoch: 145, spaceId: SPACE });
          await settleAdministrationTurns(harness);
          handleTsRequest({ kind: "directory-administration-refresh", operationEpoch: 145 });
          handleTsRequest({ kind: "directory-administration-refresh", operationEpoch: 145 });
          expect(pending).toHaveLength(2);
          pending[1]!(new Response(member));
          await settleAdministrationTurns(harness);
          expect(testSeams.directoryAdministration?.canonicalJson).toBe(member);
          const messages = harness.posted.length;
          pending[0]!(row.older === "failure" ? new Response("", { status: 403 }) : new Response(author));
          await settleAdministrationTurns(harness);
          expect(equal(testSeams.directoryAdministration?.page?.access, row.expected), row.id).toBe(true);
          expect(testSeams.directoryAdministration?.canonicalJson).toBe(member);
          expect(harness.posted).toHaveLength(messages);
        } finally {
          harness.release();
          closeDirectory();
        }
      }
      const harness = administrationHarness([200]);
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(author);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 146, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        const command: DirectoryCommand = { kind: "rename-space", spaceId: SPACE, name: "Research" };
        harness.bodies.push(await sealCommandReceipt("3".repeat(32), command), author);
        handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 146, requestId: "3".repeat(32), command });
        const requests = harness.requests.length;
        handleTsRequest({ kind: "directory-administration-refresh", operationEpoch: 146 });
        expect(harness.requests.length - requests).toBe(fixture.refreshDuringSubmit.additionalRequests);
        expect(harness.posted.at(-1)).toMatchObject({ phase: "submitting", code: fixture.refreshDuringSubmit.code });
        await settleAdministrationTurns(harness);
        await settleAdministrationTurns(harness);
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("changes space properties only after exact worker receipts and canonical refreshes", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: equal } = await import("fast-deep-equal");
      const base = "./🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛂️SpaceAdministration/🧪️tests/🧫️fixtures/⚙️properties";
      const fixture = JSON.parse(readFileSync(new URL(base + "/🔣️.json", source.url), "utf8"));
      
      const directorySchema = JSON.parse(readFileSync(new URL("./🔨️modules/📇️directory/🧬️schema/🔣️.json", source.url), "utf8"));
      const members = [{ userId: "user-a", email: "a@example.invalid", role: "author" as const, owner: true }];
      const properties: { name: string; visibility: "public" | "private" } = { name: "Administered", visibility: "private" };
      const harness = administrationHarness([200]);
      let canonical = await sealAdministrationPage(members, [], properties);
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(canonical);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 140, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        let index = 0;
        for (const row of fixture.cases as Array<{ command: Extract<DirectoryCommand, { kind: "rename-space" | "set-visibility" }> }>) {
          const requestId = (++index).toString(16).padStart(32, "0");
          const receipt = await sealCommandReceipt(requestId, row.command);
          if (row.command.kind === "rename-space") properties.name = row.command.name;
          else properties.visibility = row.command.visibility;
          const next = await sealAdministrationPage(members, [], properties);
          harness.bodies.push(receipt, next);
          const start = harness.posted.length;
          handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 140, requestId, command: row.command });
          expect(testSeams.directoryAdministration?.canonicalJson).toBe(canonical);
          await settleAdministrationTurns(harness);
          await settleAdministrationTurns(harness);
          const states = harness.posted.slice(start).filter((message) => message.kind === "directory-administration-state");
          expect(equal(states.map((message) => message.phase), ["submitting", "receipt", "refreshing", "ready"])).toBe(true);
          expect(states.slice(0, 3).every((message) => message.canonicalJson === canonical)).toBe(true);
          expect(states[1]?.receiptSha256).toBe(JSON.parse(receipt).receiptSha256);
          expect(states[3]?.canonicalJson).toBe(next);
          expect(equal(JSON.parse(next).space.name, properties.name)).toBe(true);
          expect(equal(JSON.parse(next).space.visibility, properties.visibility)).toBe(true);
          canonical = next;
        }
        expect(harness.requests.filter((entry) => entry.startsWith("POST"))).toHaveLength(fixture.cases.length);
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("drives loading → ready → submitting → receipt → refreshing without changing state before the receipt", async () => {
      const harness = administrationHarness([200, 200]);
      const first = await sealAdministrationPage(
        [
          { userId: "user-a", email: "a@example.invalid", role: "author", owner: true },
          { userId: "user-b", email: "b@example.invalid", role: "spectator", owner: false },
        ],
        [{ inviteId: "invite-1", createdAtMs: 20 }],
      );
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(first);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 1, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        const loading = harness.posted.filter((message) => message.kind === "directory-administration-state");
        expect(loading.map((message) => (message as { phase: string }).phase)).toEqual(["loading", "ready"]);
        expect((loading.at(-1) as { canonicalJson?: string }).canonicalJson).toBe(first);

        const command: DirectoryCommand = { kind: "remove-member", spaceId: SPACE, userId: "user-b" };
        const receipt = await sealCommandReceipt("0".repeat(31) + "1", command, undefined);
        const second = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], [{ inviteId: "invite-1", createdAtMs: 20 }]);
        harness.bodies.push(receipt, second);
        handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 1, requestId: "0".repeat(31) + "1", command });
        await settleAdministrationTurns(harness);
        await settleAdministrationTurns(harness);
        const phases = harness.posted.filter((message) => message.kind === "directory-administration-state").map((message) => (message as { phase: string }).phase);
        expect(phases).toEqual(["loading", "ready", "submitting", "receipt", "refreshing", "ready"]);
        const submitting = harness.posted.filter((message) => message.kind === "directory-administration-state").find((message) => (message as { phase: string }).phase === "submitting") as { canonicalJson?: string; receiptSha256?: string };
        expect(submitting.canonicalJson).toBe(first);
        expect(submitting.receiptSha256).toBeUndefined();
        const receiptState = harness.posted.filter((message) => message.kind === "directory-administration-state").find((message) => (message as { phase: string }).phase === "receipt") as { canonicalJson?: string; receiptSha256?: string };
        expect(receiptState.canonicalJson).toBe(first);
        expect(receiptState.receiptSha256).toMatch(/^[0-9a-f]{64}$/u);
        expect(harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).toMatchObject({ phase: "ready", canonicalJson: second });
        expect(harness.requests.filter((entry) => entry.startsWith("POST"))).toHaveLength(1);
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("settles delete only from an exact accepted receipt and never interprets a page 404 as deletion", async () => {
      const { readFileSync } = await import("node:fs");
      const base = "./🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛂️SpaceAdministration/🧪️tests/🧫️fixtures/🗑️delete-space";
      const fixture = JSON.parse(readFileSync(new URL(base + "/🔣️.json", source.url), "utf8")) as { acceptedOutcomes: Array<"accepted" | "previously-accepted">; missingPageStatus: number; confirmation: string };
      
      
      expect(fixture.confirmation).toBe("get-after-accepted-receipt");
      const page = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], []);
      let epoch = 160;
      for (const outcome of fixture.acceptedOutcomes) {
        const harness = administrationHarness([200, fixture.missingPageStatus]);
        const command: DirectoryCommand = { kind: "delete-space", spaceId: SPACE };
        const requestId = (++epoch).toString(16).padStart(32, "0");
        try {
          testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
          harness.bodies.push(page, await sealCommandReceipt(requestId, command, undefined, outcome));
          handleTsRequest({ kind: "directory-administration-open", operationEpoch: epoch, spaceId: SPACE });
          await settleAdministrationTurns(harness);
          const requestCount = harness.requests.length;
          handleTsRequest({ kind: "directory-administration-submit", operationEpoch: epoch, requestId, command });
          await vi.waitFor(() => expect(harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).toMatchObject({ phase: "deleted", receiptSha256: expect.stringMatching(/^[0-9a-f]{64}$/u) }));
          const terminal = harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as Extract<BackboneWorkerResponse, { kind: "directory-administration-state" }>;
          expect(terminal.canonicalJson).toBeUndefined();
          expect(terminal.inviteCapabilityPending).toBeUndefined();
          expect(harness.requests.slice(requestCount).filter((request) => request.startsWith("POST"))).toHaveLength(1);
          expect(harness.requests.slice(requestCount).filter((request) => request.startsWith("GET"))).toHaveLength(1);
          expect(testSeams.directoryAdministration).toBeNull();
        } finally {
          harness.release();
          closeDirectory();
        }
      }
      const refused = administrationHarness([200]);
      try {
        const command: DirectoryCommand = { kind: "delete-space", spaceId: SPACE };
        const requestId = "f".repeat(32);
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: refused.fetches as never });
        refused.bodies.push(page, await sealCommandReceipt(requestId, command, undefined, "secret-undeliverable"));
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 190, spaceId: SPACE });
        await settleAdministrationTurns(refused);
        handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 190, requestId, command });
        await vi.waitFor(() => expect(refused.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).toMatchObject({ phase: "failed", code: "invalid" }));
        expect(refused.posted.some((message) => message.kind === "directory-administration-state" && message.phase === "deleted")).toBe(false);
      } finally {
        refused.release();
        closeDirectory();
      }
      const missing = administrationHarness([404]);
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: missing.fetches as never });
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 191, spaceId: SPACE });
        await vi.waitFor(() => expect(missing.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).toMatchObject({ phase: "denied", code: "forbidden" }));
        expect(missing.posted.some((message) => message.kind === "directory-administration-state" && message.phase === "deleted")).toBe(false);
      } finally {
        missing.release();
        closeDirectory();
      }
    });

    it("retains the invite capability until exact clipboard success and rejects duplicate results without redisclosure", async () => {
      const harness = administrationHarness([200, 200]);
      const page = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], []);
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(page);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 2, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        const command: DirectoryCommand = { kind: "create-invite", spaceId: SPACE, role: "spectator", ttlSecs: 3600 };
        harness.bodies.push(await sealCommandReceipt("1".repeat(32), command, "invite.v1.secret"), page);
        handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 2, requestId: "1".repeat(32), command });
        await settleAdministrationTurns(harness);
        await settleAdministrationTurns(harness);
        expect(harness.posted.some((message) => message.kind === "directory-administration-state" && (message as { inviteCapabilityPending?: boolean }).inviteCapabilityPending === true)).toBe(true);
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 2 });
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 2 });
        let capabilities = harness.posted.filter((message) => message.kind === "directory-administration-capability");
        expect(capabilities).toHaveLength(1);
        expect(capabilities[0]).toMatchObject({ operationEpoch: 2, transferEpoch: 1, inviteToken: "invite.v1.secret" });
        expect(harness.posted).toContainEqual({ kind: "directory-administration-capability-rejected", operationEpoch: 2, transferEpoch: 1, code: "capacity" });

        handleTsRequest({ kind: "directory-administration-capability-result", operationEpoch: 2, transferEpoch: 1, copied: false });
        expect(harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).toMatchObject({
          inviteCapabilityPending: true,
          inviteCapabilityStatus: "failed",
        });
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 2 });
        capabilities = harness.posted.filter((message) => message.kind === "directory-administration-capability");
        expect(capabilities).toHaveLength(2);
        expect(capabilities[1]).toMatchObject({ operationEpoch: 2, transferEpoch: 2, inviteToken: "invite.v1.secret" });

        handleTsRequest({ kind: "directory-administration-capability-result", operationEpoch: 2, transferEpoch: 1, copied: true });
        expect(harness.posted).toContainEqual({ kind: "directory-administration-capability-rejected", operationEpoch: 2, transferEpoch: 1, code: "mismatch" });
        handleTsRequest({ kind: "directory-administration-capability-result", operationEpoch: 2, transferEpoch: 2, copied: true });
        handleTsRequest({ kind: "directory-administration-capability-result", operationEpoch: 2, transferEpoch: 2, copied: true });
        expect(harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).not.toMatchObject({ inviteCapabilityPending: true });
        expect(harness.posted).toContainEqual({ kind: "directory-administration-capability-rejected", operationEpoch: 2, transferEpoch: 2, code: "already-settled" });
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 2 });
        expect(harness.posted.filter((message) => message.kind === "directory-administration-capability")).toHaveLength(2);
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("erases an invite capability when the canonical refresh no longer grants author access", async () => {
      const harness = administrationHarness([200, 200]);
      const author = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], []);
      const member = await sealMemberAdministrationPage();
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(author);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 6, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        const command: DirectoryCommand = { kind: "create-invite", spaceId: SPACE, role: "spectator", ttlSecs: 3600 };
        harness.bodies.push(await sealCommandReceipt("3".repeat(32), command, "invite.v1.spectator-secret"), member);
        handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 6, requestId: "3".repeat(32), command });
        await settleAdministrationTurns(harness);
        await settleAdministrationTurns(harness);
        expect(harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).toMatchObject({ phase: "ready", canonicalJson: member });
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 6 });
        expect(harness.posted.some((message) => message.kind === "directory-administration-capability")).toBe(false);
        expect(harness.posted.at(-1)).toEqual({ kind: "directory-administration-capability-rejected", operationEpoch: 6, code: "already-settled" });
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("clears the pane on 401 and on 403 and never retains a page after the denial", async () => {
      for (const status of [401, 403]) {
        const harness = administrationHarness([status]);
        try {
          testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
          handleTsRequest({ kind: "directory-administration-open", operationEpoch: 3, spaceId: SPACE });
          await settleAdministrationTurns(harness);
          const terminal = harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as { phase: string; canonicalJson?: string; code?: string };
          expect(terminal.phase).toBe("denied");
          expect(terminal.canonicalJson).toBeUndefined();
          expect(terminal.code).toBe(status === 401 ? "unauthorized" : "forbidden");
        } finally {
          harness.release();
          closeDirectory();
        }
      }
    });

    it("retires the pane on a scoped 4401 for its own space and ignores another space's revocation", async () => {
      const harness = administrationHarness([200, 200]);
      const page = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], []);
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(page);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 5, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        expect((harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as { phase: string }).phase).toBe("ready");

        revokeDirectoryAdministrationForScope("some-other-space");
        expect((harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as { phase: string }).phase).toBe("ready");

        revokeDirectoryAdministrationForScope(SPACE);
        const terminal = harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as { phase: string; canonicalJson?: string; code?: string };
        expect(terminal.phase).toBe("denied");
        expect(terminal.code).toBe("forbidden");
        expect(terminal.canonicalJson).toBeUndefined();
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("cancels on close and drops every later turn for the retired epoch", async () => {
      const harness = administrationHarness([200]);
      const page = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], []);
      try {
        testSeams.directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(page);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 4, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        if (testSeams.directoryAdministration === null) throw new Error("administration operation missing");
        testSeams.directoryAdministration.inviteToken = "invite.v1.close-secret";
        testSeams.directoryAdministration.inviteCapabilityStatus = "available";
        handleTsRequest({ kind: "directory-administration-close", operationEpoch: 4 });
        const terminal = harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as { phase: string; canonicalJson?: string };
        expect(terminal.phase).toBe("cancelled");
        expect(terminal.canonicalJson).toBeUndefined();
        const before = harness.requests.length;
        handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 4, requestId: "2".repeat(32), command: { kind: "remove-member", spaceId: SPACE, userId: "user-b" } });
        handleTsRequest({ kind: "directory-administration-refresh", operationEpoch: 4 });
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 4 });
        await settleAdministrationTurns(harness);
        expect(harness.requests).toHaveLength(before);
        expect(harness.posted.some((message) => message.kind === "directory-administration-capability")).toBe(false);
        expect(terminal).not.toMatchObject({ inviteCapabilityPending: true });
      } finally {
        harness.release();
        closeDirectory();
      }
    });
  });
  //#endregion 🔖️DirectoryLaneTests
}
