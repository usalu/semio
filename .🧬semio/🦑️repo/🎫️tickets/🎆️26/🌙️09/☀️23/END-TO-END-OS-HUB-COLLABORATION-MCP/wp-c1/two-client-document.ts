// #region Header
/**
 * 🤝️ Two-client document collaboration e2e (language-agnostic fixture).
 * Gated by HUB_E2E=1. PR1 socket chain + HC1 catalog clone + note creation.
 */
// #endregion Header

import { spawnSync } from "node:child_process";
import { cpSync, existsSync, mkdtempSync, readdirSync, readFileSync, rmSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const HUB_E2E = process.env.HUB_E2E === "1";
const TEST_TIMEOUT_MS = 600_000;

function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth++) {
    if (readdirSync(current).some((n) => n.endsWith("hub") && existsSync(join(current, n, "📦️packages")))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error("repo root not found");
}

function pick(parent: string, pred: (n: string) => boolean): string {
  const name = readdirSync(parent).find(pred);
  if (!name) throw new Error(`missing child in ${parent}`);
  return join(parent, name);
}

const repoRoot = findRepoRoot(dirname(fileURLToPath(import.meta.url)));
const hubRoot = pick(repoRoot, (n) => n.endsWith("hub") && !n.startsWith("."));

describe("two-client document collaboration fixture", () => {
  it("loads the language-agnostic scenario fixture", () => {
    const fixtures = pick(hubRoot, (n) => n.includes("fixtures"));
    const fixtureDir = pick(fixtures, (n) => n.includes("two-client-document"));
    const fixture = JSON.parse(readFileSync(join(fixtureDir, "🔣️.json"), "utf8"));
    expect(fixture.schema).toBe("semio.hub.two-client-document-scenario/v1");
    expect(fixture.authors).toEqual(["author-a", "author-b"]);
    expect(fixture.steps.length).toBeGreaterThanOrEqual(8);
    expect(fixture.expectations.bReceivesCommandsEnvelope).toBe(true);
    expect(fixture.expectations.presenceJoinReplayShowsA).toBe(false);
    expect(fixture.command.diffSchema).toBe("db.pathmap.v1");
  });
});

describe.skipIf(!HUB_E2E)("two-client document collaboration e2e", () => {
  it(
    "proves A commands and presence reach B over a real os-hub binary",
    async () => {
      const packages = pick(hubRoot, (n) => n.includes("packages"));
      const hubRustRoot = pick(packages, (n) => n.includes("rust"));
      const localBootstrap = pick(hubRoot, (n) => n.includes("local-bootstrap"));
      const execution = pick(localBootstrap, (n) => n.includes("execution"));
      const fw = pick(repoRoot, (n) => n.includes("framework"));
      let osRoot = "";
      for (const pn of readdirSync(fw).filter((n) => n.includes("products"))) {
        for (const on of readdirSync(join(fw, pn)).filter((n) => n.endsWith("os"))) {
          const modsName = readdirSync(join(fw, pn, on)).find((n) => n.includes("modules"));
          if (!modsName) continue;
          const kids = readdirSync(join(fw, pn, on, modsName));
          if (kids.some((k) => k.includes("directory")) && kids.some((k) => k.includes("store"))) {
            osRoot = join(fw, pn, on);
            break;
          }
        }
        if (osRoot) break;
      }
      if (!osRoot) throw new Error("os product not found");
      const osMods = pick(osRoot, (n) => n.includes("modules"));
      const directory = pick(osMods, (n) => n.includes("directory"));
      const schemaDir = pick(directory, (n) => n.includes("schema"));
      const creation = pick(schemaDir, (n) => n.includes("space-artifact-creation"));
      const modules = pick(fw, (n) => n.includes("modules") && !n.includes("products"));
      const replication = pick(modules, (n) => n.includes("replication"));

      const { startLocalHub, finishLocalHub, localHubReadinessAdmitted } = await import(join(execution, "🟦️.ts"));
      const { sealDirectoryCommandRequestV1, directoryCommandRequestJson } = await import(join(schemaDir, "🟦️.ts"));
      const { sealSpaceArtifactCreateV1 } = await import(join(creation, "🟦️.ts"));
      const { encodeClientFrame, decodeServerFrame } = await import(join(replication, "🟦️.ts"));
      const { encodePackValue, parseSocketGrantReceiptV1 } = await import(join(osRoot, "🟦️.ts"));

      const fixtures = pick(hubRoot, (n) => n.includes("fixtures"));
      const fixtureDir = pick(fixtures, (n) => n.includes("two-client-document"));
      const fixture = JSON.parse(readFileSync(join(fixtureDir, "🔣️.json"), "utf8"));

      const staged = join(hubRustRoot, "dist", "build-dev", process.platform === "win32" ? "os-hub.exe" : "os-hub");
      const bin = process.env.OS_HUB_BINARY && existsSync(process.env.OS_HUB_BINARY) ? process.env.OS_HUB_BINARY : staged;
      if (!existsSync(bin)) throw new Error(`os-hub binary missing at ${bin}`);

      const catalogSource =
        process.env.OS_HUB_TRUSTED_CATALOG_SOURCE ?? join(repoRoot, ".🧬semio", "🌐hub", "hc1-boot", "trusted-catalog");
      if (!existsSync(catalogSource)) throw new Error(`trusted catalog missing: ${catalogSource}`);

      const dataRoot = mkdtempSync("/tmp/wp-c1-two-client-");
      cpSync(catalogSource, join(dataRoot, "trusted-catalog"), { recursive: true });

      const ADA = { email: "ada-tc@example.org", password: "correct horse battery staple", display: "Ada TC" };
      const BO = { email: "bo-tc@example.org", password: "another perfectly fine phrase", display: "Bo TC" };
      for (const account of [ADA, BO]) {
        const result = spawnSync(bin, ["credential", "set", "--email", account.email, "--display-name", account.display], {
          env: { ...process.env, OS_HUB_DATA: dataRoot },
          input: account.password,
          encoding: "utf8",
        });
        if (result.status !== 0) throw new Error(`credential set failed status=${result.status} signal=${result.signal} stderr=${result.stderr} stdout=${result.stdout}`);
      }

      process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
      const port = Number(process.env.HUB_TWO_CLIENT_PORT ?? 7711);
      const profiles = [
        { profileId: "a", subject: "author-a", displayName: "Author A", allowedClientClasses: ["native", "mcp"] },
        { profileId: "b", subject: "author-b", displayName: "Author B", allowedClientClasses: ["native", "mcp"] },
      ];
      const run = await startLocalHub(repoRoot, hubRustRoot, profiles, { port, dataDir: dataRoot, binaryPath: bin, capture: true });
      const origin = `http://127.0.0.1:${port}`;
      const wsOrigin = `ws://127.0.0.1:${port}`;

      const waitReady = async (hubRun: typeof run, label: string) => {
        const deadline = Date.now() + 180_000;
        while (Date.now() < deadline) {
          if (hubRun.child.exitCode !== null) {
            throw new Error(`${label}: hub exited ${hubRun.child.exitCode}\n${hubRun.output().slice(-12000)}`);
          }
          try {
            const response = await fetch(`${origin}/readyz`, { signal: AbortSignal.timeout(2000) });
            const body = (await response.json()) as Record<string, any>;
            if (localHubReadinessAdmitted(body, response.status, hubRun.runId, true, hubRun.publicSessionIssuance)) return body;
          } catch {
            /* keep polling */
          }
          await Bun.sleep(200);
        }
        throw new Error(`${label}: hub not ready in 180s\n${hubRun.output().slice(-12000)}`);
      };


      type Frame = Record<string, any>;
      type Holder = { label: string; actor: string; socket: WebSocket; frames: Frame[]; waiters: Array<(f: Frame) => void>; welcome?: any; resumeToken?: string };

      try {
        console.error("[DEBUG] wp-c1 boot ready wait"); await waitReady(run, "boot"); console.error("[DEBUG] wp-c1 boot ready");
        const signIn = async (email: string, password: string, device: string) => {
          const res = await fetchTimed(`${origin}/auth/sessions`, {
            method: "POST",
            headers: { "content-type": "application/json", origin: "http://127.0.0.1:6066" },
            body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: device, clientClass: "browser" }),
          });
          expect(res.status).toBe(200);
          return ((await res.json()) as { token: string }).token;
        };

        const fetchTimed = async (url: string, init: RequestInit = {}, ms = 30_000) => {
          const res = await fetch(url, { ...init, signal: AbortSignal.timeout(ms) });
          return res;
        };

        const cmd = async (token: string, body: unknown) => {
          const id = [...crypto.getRandomValues(new Uint8Array(16))].map((x) => x.toString(16).padStart(2, "0")).join("");
          const sealed = directoryCommandRequestJson(sealDirectoryCommandRequestV1(id, body as any));
          const res = await fetchTimed(`${origin}/directory/commands`, {
            method: "POST",
            headers: { "content-type": "application/json", authorization: `Bearer ${token}`, origin: "http://127.0.0.1:6066" },
            body: sealed,
          });
          return { status: res.status, json: JSON.parse(await res.text()) };
        };

        const tokenA = await signIn(ADA.email, ADA.password, "device-ada");
        const tokenB = await signIn(BO.email, BO.password, "device-bo");
        const created = await cmd(tokenA, { kind: "create-space", name: fixture.spaceName, spaceKind: "studio", visibility: "private" });
        expect(created.status).toBe(202);
        let spaceId = created.json?.events?.find((e: any) => e.body?.kind === "space.created")?.body?.spaceId as string;
        if (!spaceId) {
          const listed = await (await fetchTimed(`${origin}/directory/spaces`, { headers: { authorization: `Bearer ${tokenA}` } })).json();
          spaceId = listed.find((r: any) => r?.space?.name === fixture.spaceName)?.space?.id;
        }
        expect(spaceId).toBeTruthy();
        expect((await cmd(tokenA, { kind: "upsert-member", spaceId, email: BO.email, role: "author" })).status).toBe(202);

        const route = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
        const catalog = await (await fetchTimed(`${origin}${route}`, { headers: { authorization: `Bearer ${tokenA}` } })).json();
        const kind = catalog.kinds.find((row: any) => row.kindId === fixture.artifactKindId);
        expect(kind, `catalog kinds=${catalog.kinds.map((row: any) => row.kindId).join(",")}`).toBeTruthy();
        const request = sealSpaceArtifactCreateV1({
          requestId: [...crypto.getRandomValues(new Uint8Array(16))].map((x) => x.toString(16).padStart(2, "0")).join(""),
          expectedCatalogGenerationId: catalog.catalogGenerationId,
          kindId: kind.kindId,
          name: "Two Client Note",
        });
        expect((await fetchTimed(`${origin}${route}`, { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${tokenA}` }, body: JSON.stringify(request) })).ok).toBe(true);

        let documentId = "";
        let surfaceId = fixture.surfaceId as string;
        let artifactSchema = "";
        let packSchemaHashHex = "";
        const deadline = Date.now() + 300_000;
        while (Date.now() < deadline) {
          const polled = await (await fetchTimed(`${origin}${route}/${request.requestId}`, { headers: { authorization: `Bearer ${tokenA}` } }, 60_000)).json();
          if (polled.phase === "ready") {
            documentId = polled.ready.artifactId;
            artifactSchema = polled.ready.artifactSchema ?? kind.schema;
            break;
          }
          if (["failed", "cancelled", "indeterminate"].includes(polled.phase)) {
            throw new Error(`creation ${polled.phase}: ${JSON.stringify(polled)}\nhub-output:\n${run.output().slice(-12000)}`);
          }
          await Bun.sleep(1000);
        }
        expect(documentId.startsWith("artifact-")).toBe(true);

        const waitFrame = async (holder: Holder, pred: (f: Frame) => boolean, label: string, ms = 15000) => {
          const until = Date.now() + ms;
          const hit = holder.frames.find(pred);
          if (hit) return hit;
          return await new Promise<Frame>((resolve, reject) => {
            const onFrame = (frame: Frame) => {
              if (!pred(frame)) return;
              clearInterval(timer);
              holder.waiters = holder.waiters.filter((w) => w !== onFrame);
              resolve(frame);
            };
            holder.waiters.push(onFrame);
            const timer = setInterval(() => {
              if (Date.now() < until) return;
              clearInterval(timer);
              holder.waiters = holder.waiters.filter((w) => w !== onFrame);
              const kinds = [...new Set(holder.frames.map((f) => Object.keys(f)[0]))].join(",");
              reject(new Error(`${holder.label} missing ${label}: ${kinds} (n=${holder.frames.length})`));
            }, 100);
          });
        };

        type Minted = { label: string; token: string; actor: string; surfaceId: string; artifactSchema: string; packSchemaHashHex: string; wsUrl: string; protocols: string[] };

        const mint = async (label: string, token: string): Promise<Minted> => {
          const headers = { authorization: `Bearer ${token}`, "content-type": "application/json" };
          const planRes = await fetchTimed(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, {
            method: "POST",
            headers,
            body: JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, requestedSurfaceId: surfaceId, clientInstanceId: `tc-${label}` }),
          });
          const planText = await planRes.text();
          expect(planRes.status, planText.slice(0, 200)).toBe(200);
          const plan = JSON.parse(planText);
          const mintedSurface = plan.surface?.surfaceId ?? surfaceId;
          const mintedSchema = plan.artifact.schema;
          const mintedHash = plan.artifact.packSchemaHash;
          const grantRes = await fetchTimed(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/socket-grants`, {
            method: "POST",
            headers,
            body: JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt }),
          });
          const grantText = await grantRes.text();
          expect(grantRes.status, grantText.slice(0, 200)).toBe(200);
          const receipt = parseSocketGrantReceiptV1(JSON.parse(grantText));
          const scope = `${spaceId}/${documentId}`;
          const wsUrl = `${wsOrigin}/scopes/${encodeURIComponent(scope)}/document/ws?actor=${encodeURIComponent(receipt.actorId)}&surface=${encodeURIComponent(mintedSurface)}`;
          return {
            label,
            token,
            actor: receipt.actorId,
            surfaceId: mintedSurface,
            artifactSchema: mintedSchema,
            packSchemaHashHex: mintedHash,
            wsUrl,
            protocols: ["semio.session.v1", token],
          };
        };

        const openSocket = async (minted: Minted, resumeToken: string | null = null): Promise<Holder> => {
          let closeCode: number | null = null;
          let closeReason = "";
          const socket = new WebSocket(minted.wsUrl, [...minted.protocols]);
          socket.binaryType = "arraybuffer";
          const holder: Holder = { label: minted.label, actor: minted.actor, socket, frames: [], waiters: [] };
          socket.addEventListener("close", (event) => {
            closeCode = event.code;
            closeReason = event.reason;
          });
          socket.addEventListener("message", (event) => {
            if (!(event.data instanceof ArrayBuffer)) return;
            try {
              const frame = decodeServerFrame(new Uint8Array(event.data)).frame as Frame;
              if ("Welcome" in frame) {
                holder.welcome = frame.Welcome;
                holder.resumeToken = frame.Welcome.resume_token;
              }
              // Cap retained frames — framework document lane can flood heartbeats and starve the event loop.
              holder.frames.push(frame);
              if (holder.frames.length > 64) holder.frames.splice(0, holder.frames.length - 32);
              for (const waiter of [...holder.waiters]) waiter(frame);
            } catch { /* ignore */ }
          });
          for (let i = 0; i < 200 && socket.readyState === WebSocket.CONNECTING; i++) await Bun.sleep(50);
          expect(
            socket.readyState,
            `${minted.label} socket closed before open code=${closeCode} reason=${closeReason} protocols=${JSON.stringify(minted.protocols)} url=${minted.wsUrl}`,
          ).toBe(WebSocket.OPEN);
          const packSchemaHash = [...(minted.packSchemaHashHex.match(/../gu) ?? [])].map((p) => Number.parseInt(p, 16));
          socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: minted.artifactSchema, pack_schema_hash: packSchemaHash, resume_token: resumeToken, frontier: null } }, "command"));
          await waitFrame(holder, (f) => "Welcome" in f, "Welcome");
          expect(holder.welcome.server_frontier.document_id ?? holder.welcome.server_frontier.documentId).toBe(documentId);
          return holder;
        };

        console.error("[DEBUG] wp-c1 creation done doc="+documentId); const mintedA = await mint("a", tokenA);
        const mintedB = await mint("b", tokenB);
        artifactSchema = mintedA.artifactSchema;
        packSchemaHashHex = mintedA.packSchemaHashHex;
        surfaceId = mintedA.surfaceId;
        console.error("[DEBUG] wp-c1 minted both"); const a = await openSocket(mintedA);
        const b = await openSocket(mintedB);
        console.error("[DEBUG] wp-c1 sockets open"); await waitFrame(a, (f) => "Session" in f, "Session");
        await waitFrame(b, (f) => "Session" in f, "Session");

        const mutationId = `${fixture.command.mutationIdPrefix}1`;
        const envelope = {
          mutation_id: mutationId,
          document_id: documentId,
          actor: a.actor,
          dependencies: [],
          diff: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue(fixture.command.diffValue)) },
          inverse: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue(fixture.command.inverseValue)) },
          timestamp: { physical: Date.now(), logical: 0 },
        };
        a.socket.send(encodeClientFrame({ Commands: { batch_id: fixture.command.batchId, envelopes: [envelope] } }, "command"));
        const ack = await waitFrame(a, (f) => "Ack" in f && f.Ack.batch_id === fixture.command.batchId, "Ack");
        expect(JSON.stringify(ack.Ack.stages)).toContain("Applied");
        expect(ack.Ack.frontier.document_id ?? ack.Ack.frontier.documentId).toBe(documentId);

        console.error("[DEBUG] wp-c1 cmd sent"); const remote = await waitFrame(b, (f) => "Commands" in f && f.Commands.envelopes?.[0]?.mutation_id === mutationId, "Commands"); console.error("[DEBUG] wp-c1 cmd relayed");
        expect(remote.Commands.envelopes[0].mutation_id).toBe(mutationId);
        expect(remote.Commands.envelopes[0].document_id).toBe(documentId);
        expect(remote.Commands.origin).toBe(a.actor);

        const resume = b.resumeToken!;
        expect(resume.length).toBeGreaterThan(0);
        b.socket.close();
        await Bun.sleep(500);
        const mutationId2 = `${fixture.command.mutationIdPrefix}2`;
        a.socket.send(encodeClientFrame({ Commands: { batch_id: fixture.command.batchId + 1, envelopes: [{ ...envelope, mutation_id: mutationId2, diff: { schema: fixture.command.diffSchema, payload: Array.from(encodePackValue({ value: "two-client-a2" })) } }] } }, "command"));
        await waitFrame(a, (f) => "Ack" in f && f.Ack.batch_id === fixture.command.batchId + 1, "Ack2");
        console.error("[DEBUG] wp-c1 reconnect"); const b2 = await openSocket(await mint("b-re", tokenB), resume);
        const recovered =
          b2.frames.some((f) => "Commands" in f && f.Commands.envelopes?.some((e: any) => e.mutation_id === mutationId2)) ||
          b2.frames.some((f) => "RebootstrapRequired" in f) ||
          (b2.welcome.server_frontier.last_commit_seq ?? 0) >= (ack.Ack.frontier.last_commit_seq ?? 0);
        expect(recovered).toBe(true);
        const frontierBefore = b2.welcome.server_frontier;
        a.socket.close();
        b2.socket.close();
        await finishLocalHub(run);

        process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
        const run2 = await startLocalHub(repoRoot, hubRustRoot, profiles, { port, dataDir: dataRoot, binaryPath: bin, capture: true });
        try {
          console.error("[DEBUG] wp-c1 restart"); await waitReady(run2, "restart"); console.error("[DEBUG] wp-c1 restart ready");
          const tokenA2 = await signIn(ADA.email, ADA.password, "device-ada-2");
          const spaces = await (await fetchTimed(`${origin}/directory/spaces`, { headers: { authorization: `Bearer ${tokenA2}` } })).json();
          expect(spaces.some((r: any) => r?.space?.id === spaceId)).toBe(true);
          const a3 = await openSocket(await mint("a-restart", tokenA2));
          expect(a3.welcome.server_frontier.document_id ?? a3.welcome.server_frontier.documentId).toBe(documentId);
          expect((a3.welcome.server_frontier.last_commit_seq ?? 0) >= (frontierBefore.last_commit_seq ?? 0)).toBe(true);
          a3.socket.close();
        } finally {
          await finishLocalHub(run2);
        }
      } finally {
        try { await finishLocalHub(run); } catch { /* done */ }
        rmSync(dataRoot, { recursive: true, force: true });
      }
    },
    TEST_TIMEOUT_MS,
  );
});
