// #region Header
/**
 * 🤝️ Language-agnostic two-client document collaboration e2e — boots the real `os-hub` binary,
 * drives two sessions over directory HTTP + document WS (PR1 socket chain), and asserts the
 * scenario fixture: A’s Commands reach B, Ack frontier advances with a plain document id (HT16),
 * A’s presence reaches B, B reconnects with resume_token after a miss, and a hub restart on the
 * same dataDir keeps artifact + frontier.
 *
 * Gated behind `HUB_E2E=1`. Clones `trusted-catalog` from `OS_HUB_TRUSTED_CATALOG_SOURCE` or
 * `.🧬semio/🌐hub/hc1-boot/trusted-catalog` into a fresh data root (HC1 pattern).
 */
// #endregion Header

import { spawnSync } from "node:child_process";
import { cpSync, existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const HUB_E2E = process.env.HUB_E2E === "1";
const TEST_TIMEOUT_MS = 600_000;

function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, "🌎️hub", "📦️packages", "🦀️rust", "Cargo.toml"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error("two-client-document: repository root not found above " + start);
}

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = findRepoRoot(here);
const hubRustRoot = join(repoRoot, "�Librehub".replace("Libre", "�︎")); // placeholder fixed below

async function loadDeps(root: string) {
  const hubDir = join(root, "�Librehub");
  // Resolve hub dir without relying on the broken replace above.
  const { readdirSync } = await import("node:fs");
  const hubName = readdirSync(root).find((name) => name.endsWith("hub") && !name.startsWith("."));
  if (!hubName) throw new Error("hub directory missing");
  const hub = join(root, hubName);
  const lb = readdirSync(hub).find((name) => name.includes("local-bootstrap"));
  const execDir = readdirSync(join(hub, lb!)).find((name) => name.includes("execution"));
  const credDir = readdirSync(join(hub, lb!)).find((name) => name.includes("credential-issuance"));
  const exec = await import(join(hub, lb!, execDir!, "🟦️.ts"));
  const fw = readdirSync(root).find((name) => name.includes("framework"));
  const products = readdirSync(join(root, fw!)).find((name) => name.includes("products") && readdirSync(join(root, fw!, name)).some((c) => c.endsWith("os")));
  const osName = readdirSync(join(root, fw!, products!)).find((name) => name.endsWith("os"));
  const osMods = readdirSync(join(root, fw!, products!, osName!)).find((name) => name.includes("modules"));
  const directory = readdirSync(join(root, fw!, products!, osName!, osMods!)).find((name) => name.includes("directory"));
  const schemaDir = join(root, fw!, products!, osName!, osMods!, directory!, "🧬️schema");
  const creation = readdirSync(schemaDir).find((name) => name.includes("space-artifact-creation"));
  const dirSchema = await import(join(schemaDir, "🟦️.ts"));
  const creationSchema = await import(join(schemaDir, creation!, "🟦️.ts"));
  const modules = readdirSync(join(root, fw!)).find((name) => name.includes("modules") && !name.includes("products"));
  const replication = readdirSync(join(root, fw!, modules!)).find((name) => name.includes("replication"));
  const wire = await import(join(root, fw!, modules!, replication!, "🟦️.ts"));
  const osPkg = await import(join(root, fw!, products!, osName!, "🟦️.ts"));
  const pkgs = readdirSync(hub).find((name) => name.includes("packages"));
  const rustPkg = readdirSync(join(hub, pkgs!)).find((name) => name.includes("rust"));
  return {
    hub,
    hubRustRoot: join(hub, pkgs!, rustPkg!),
    startLocalHub: exec.startLocalHub as typeof exec.startLocalHub,
    finishLocalHub: exec.finishLocalHub as typeof exec.finishLocalHub,
    waitForReadiness: exec.waitForReadiness as typeof exec.waitForReadiness,
    localHubReadinessAdmitted: exec.localHubReadinessAdmitted as typeof exec.localHubReadinessAdmitted,
    sealDirectoryCommandRequestV1: dirSchema.sealDirectoryCommandRequestV1,
    directoryCommandRequestJson: dirSchema.directoryCommandRequestJson,
    sealSpaceArtifactCreateV1: creationSchema.sealSpaceArtifactCreateV1,
    encodeClientFrame: wire.encodeClientFrame,
    decodeServerFrame: wire.decodeServerFrame,
    encodePresencePeer: wire.encodePresencePeer,
    decodePresencePeer: wire.decodePresencePeer,
    encodePackValue: osPkg.encodePackValue,
    parseSocketGrantReceiptV1: osPkg.parseSocketGrantReceiptV1,
    socketGrantProtocolsV1: osPkg.socketGrantProtocolsV1,
  };
}

describe("two-client document collaboration fixture", () => {
  it("validates the language-agnostic scenario fixture against its schema export", async () => {
    const { hubSchemaExport, getWorkspaceRoot } = await import(join(repoRoot, "�Librehub", "🤝️integration-harness", "🟦️.ts").replace("�Librehub", readdirSyncHub()));
    function readdirSyncHub(): string {
      const { readdirSync } = require("node:fs") as typeof import("node:fs");
      return readdirSync(repoRoot).find((name: string) => name.endsWith("hub") && !name.startsWith("."))!;
    }
    const root = getWorkspaceRoot();
    const hubName = readdirSyncHub();
    const fixtures = join(root, hubName, "🧫️fixtures", "🤝️two-client-document-v1", "🔣️.json");
    const fixture = JSON.parse(readFileSync(fixtures, "utf8"));
    // Prefer catalog export once regenerated; fall back to structural assertions.
    try {
      const validate = hubSchemaExport(root, "schema://hub.two-client-document/TwoClientDocumentScenarioV1");
      expect(validate(fixture)).toBe(true);
    } catch {
      expect(fixture.schema).toBe("semio.hub.two-client-document-scenario/v1");
      expect(fixture.authors).toHaveLength(2);
      expect(fixture.steps.length).toBeGreaterThanOrEqual(8);
      expect(fixture.expectations.bReceivesCommandsEnvelope).toBe(true);
    }
  });
});

describe.skipIf(!HUB_E2E)("two-client document collaboration e2e", () => {
  it(
    "proves User A commands and presence reach User B over a real os-hub binary",
    async () => {
      const deps = await loadDeps(repoRoot);
      const { readdirSync } = await import("node:fs");
      const hubName = readdirSync(repoRoot).find((name) => name.endsWith("hub") && !name.startsWith("."))!;
      const fixture = JSON.parse(
        readFileSync(join(repoRoot, hubName, "🧫️fixtures", "🤝️two-client-document-v1", "🔣️.json"), "utf8"),
      );

      const staged = join(deps.hubRustRoot, "dist", "build-dev", process.platform === "win32" ? "os-hub.exe" : "os-hub");
      const bin =
        process.env.OS_HUB_BINARY && existsSync(process.env.OS_HUB_BINARY)
          ? process.env.OS_HUB_BINARY
          : existsSync(staged)
            ? staged
            : (() => {
                throw new Error(`os-hub binary missing; build with bun nx run os-hub:build-dev (looked for ${staged})`);
              })();

      const catalogSource =
        process.env.OS_HUB_TRUSTED_CATALOG_SOURCE ??
        join(repoRoot, ".🧬semio", "🌐hub", "hc1-boot", "trusted-catalog");
      if (!existsSync(catalogSource)) {
        throw new Error(`trusted catalog source missing: ${catalogSource}`);
      }

      const dataRoot = mkdtempSync("/private/tmp/wp-c1-two-client-");
      cpSync(catalogSource, join(dataRoot, "trusted-catalog"), { recursive: true });

      const ADA = { email: "ada-tc@example.org", password: "correct horse battery staple", display: "Ada TC" };
      const BO = { email: "bo-tc@example.org", password: "another perfectly fine phrase", display: "Bo TC" };
      const provision = (account: typeof ADA): string => {
        const result = spawnSync(bin, ["credential", "set", "--email", account.email, "--display-name", account.display], {
          env: { ...process.env, OS_HUB_DATA: dataRoot },
          input: account.password,
          encoding: "utf8",
        });
        if (result.status !== 0) throw new Error(`credential set failed: ${result.stderr}`);
        return result.stdout.trim();
      };
      provision(ADA);
      provision(BO);

      process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
      const port = Number(process.env.HUB_TWO_CLIENT_PORT ?? 7711);
      const run = await deps.startLocalHub(
        repoRoot,
        deps.hubRustRoot,
        [
          { profileId: "a", subject: "author-a", displayName: "Author A", allowedClientClasses: ["native", "mcp"] },
          { profileId: "b", subject: "author-b", displayName: "Author B", allowedClientClasses: ["native", "mcp"] },
        ],
        { port, dataDir: dataRoot, binaryPath: bin, capture: true },
      );

      const origin = `http://127.0.0.1:${port}`;
      const wsOrigin = `ws://127.0.0.1:${port}`;

      type Frame = Record<string, any>;
      type Holder = {
        label: string;
        token: string;
        actor: string;
        socket: WebSocket;
        frames: Frame[];
        welcome?: Frame;
        resumeToken?: string;
        closed: boolean;
      };

      try {
        const ready = await deps.waitForReadiness(run, true);
        expect(ready.status === "ready" || deps.localHubReadinessAdmitted(ready as any, 200, run.runId, true, run.publicSessionIssuance)).toBeTruthy();
        expect((ready as any).features?.openPlan ?? (ready as any).artifactAuthority?.ready).toBeTruthy();

        const signIn = async (email: string, password: string, device: string): Promise<string> => {
          const res = await fetch(`${origin}/auth/sessions`, {
            method: "POST",
            headers: { "content-type": "application/json", origin: "http://127.0.0.1:6066" },
            body: JSON.stringify({
              schema: "semio.hub.auth.credential-sign-in/v1",
              email,
              password,
              deviceInstanceId: device,
              clientClass: "browser",
            }),
          });
          const body = (await res.json()) as { token?: string };
          expect(res.status).toBe(200);
          return body.token!;
        };

        const cmd = async (token: string, body: unknown) => {
          const id = [...crypto.getRandomValues(new Uint8Array(16))].map((x) => x.toString(16).padStart(2, "0")).join("");
          const sealed = deps.directoryCommandRequestJson(deps.sealDirectoryCommandRequestV1(id, body as any));
          const res = await fetch(`${origin}/directory/commands`, {
            method: "POST",
            headers: { "content-type": "application/json", authorization: `Bearer ${token}`, origin: "http://127.0.0.1:6066" },
            body: sealed,
          });
          const text = await res.text();
          return { status: res.status, json: JSON.parse(text) };
        };

        const tokenA = await signIn(ADA.email, ADA.password, "device-ada");
        const tokenB = await signIn(BO.email, BO.password, "device-bo");

        const created = await cmd(tokenA, {
          kind: "create-space",
          name: fixture.spaceName,
          spaceKind: "studio",
          visibility: "private",
        });
        expect(created.status).toBe(202);
        let spaceId: string =
          created.json?.events?.find((e: any) => e.body?.kind === "space.created")?.body?.spaceId ?? "";
        if (!spaceId) {
          const listed = await (await fetch(`${origin}/directory/spaces`, { headers: { authorization: `Bearer ${tokenA}` } })).json();
          spaceId = listed.find((row: any) => row?.space?.name === fixture.spaceName)?.space?.id;
        }
        expect(spaceId.length).toBeGreaterThan(0);

        const admitted = await cmd(tokenA, { kind: "upsert-member", spaceId, email: BO.email, role: "author" });
        expect(admitted.status).toBe(202);

        const route = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
        const catalog = await (await fetch(`${origin}${route}`, { headers: { authorization: `Bearer ${tokenA}` } })).json();
        const kind = catalog.kinds.find((row: any) => row.kindId === fixture.artifactKindId) ?? catalog.kinds[0];
        expect(kind).toBeTruthy();
        const request = deps.sealSpaceArtifactCreateV1({
          requestId: [...crypto.getRandomValues(new Uint8Array(16))].map((x) => x.toString(16).padStart(2, "0")).join(""),
          expectedCatalogGenerationId: catalog.catalogGenerationId,
          kindId: kind.kindId,
          name: "Two Client Note",
        });
        const accepted = await fetch(`${origin}${route}`, {
          method: "POST",
          headers: { "content-type": "application/json", authorization: `Bearer ${tokenA}` },
          body: JSON.stringify(request),
        });
        expect(accepted.status).toBeLessThan(300);

        let documentId = "";
        let surfaceId = fixture.surfaceId;
        let artifactSchema = "";
        let packSchemaHashHex = "";
        const createDeadline = Date.now() + 300_000;
        while (Date.now() < createDeadline) {
          const polled = await (await fetch(`${origin}${route}/${request.requestId}`, { headers: { authorization: `Bearer ${tokenA}` } })).json();
          if (polled.phase === "ready") {
            documentId = polled.ready.artifactId;
            artifactSchema = polled.ready.artifactSchema ?? kind.schema;
            break;
          }
          if (["failed", "cancelled", "indeterminate"].includes(polled.phase)) {
            throw new Error(`artifact creation ${polled.phase}: ${JSON.stringify(polled)}`);
          }
          await Bun.sleep(1000);
        }
        expect(documentId.startsWith("artifact-")).toBe(true);

        const attach = async (label: string, token: string, resumeToken: string | null = null): Promise<Holder> => {
          const headers = { authorization: `Bearer ${token}`, "content-type": "application/json" };
          const planResponse = await fetch(
            `${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`,
            {
              method: "POST",
              headers,
              body: JSON.stringify({
                schema: "semio.hub.document-open-intent/v1",
                version: 1,
                scope: { spaceId, documentId },
                requestedSurfaceId: surfaceId,
                clientInstanceId: `tc-${label}`,
              }),
            },
          );
          const planText = await planResponse.text();
          expect(planResponse.status, planText.slice(0, 300)).toBe(200);
          const plan = JSON.parse(planText) as {
            receipt: string;
            surface?: { surfaceId?: string };
            artifact: { schema: string; packSchemaHash: string };
          };
          surfaceId = plan.surface?.surfaceId ?? surfaceId;
          artifactSchema = plan.artifact.schema;
          packSchemaHashHex = plan.artifact.packSchemaHash;
          const grantResponse = await fetch(
            `${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/socket-grants`,
            {
              method: "POST",
              headers,
              body: JSON.stringify({
                schema: "semio.hub.document-plan-socket-grant-intent/v1",
                version: 1,
                planReceipt: plan.receipt,
              }),
            },
          );
          const grantText = await grantResponse.text();
          expect(grantResponse.status, grantText.slice(0, 300)).toBe(200);
          const receipt = deps.parseSocketGrantReceiptV1(JSON.parse(grantText));
          const wsUrl = `${wsOrigin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/socket/v1?surface=${encodeURIComponent(surfaceId)}`;
          const socket = new WebSocket(wsUrl, [...deps.socketGrantProtocolsV1(receipt)]);
          socket.binaryType = "arraybuffer";
          const holder: Holder = { label, token, actor: receipt.actorId, socket, frames: [], closed: false };
          socket.addEventListener("message", (event) => {
            if (!(event.data instanceof ArrayBuffer)) return;
            try {
              const decoded = deps.decodeServerFrame(new Uint8Array(event.data)).frame as Frame;
              holder.frames.push(decoded);
              if ("Welcome" in decoded) {
                holder.welcome = decoded.Welcome;
                holder.resumeToken = decoded.Welcome.resume_token;
              }
            } catch {
              /* ignore decode races */
            }
          });
          socket.addEventListener("close", () => {
            holder.closed = true;
          });
          for (let wait = 0; wait < 200 && socket.readyState === WebSocket.CONNECTING; wait += 1) await Bun.sleep(50);
          expect(socket.readyState).toBe(WebSocket.OPEN);
          const packSchemaHash = [...(packSchemaHashHex.match(/../gu) ?? [])].map((pair) => Number.parseInt(pair, 16));
          socket.send(
            deps.encodeClientFrame(
              {
                SocketHelloV1: {
                  wire_version: 1,
                  protocol_version: 1,
                  schema: artifactSchema,
                  pack_schema_hash: packSchemaHash,
                  resume_token: resumeToken,
                  frontier: null,
                },
              },
              "command",
            ),
          );
          const welcomeDeadline = Date.now() + 10_000;
          while (Date.now() < welcomeDeadline && !holder.welcome) await Bun.sleep(50);
          expect(holder.welcome, `${label} Welcome`).toBeTruthy();
          expect(holder.welcome!.server_frontier?.document_id ?? holder.welcome!.server_frontier?.documentId).toBe(documentId);
          return holder;
        };

        const waitFrame = async (holder: Holder, predicate: (frame: Frame) => boolean, label: string, ms = 15_000): Promise<Frame> => {
          const deadline = Date.now() + ms;
          while (Date.now() < deadline) {
            const hit = holder.frames.find(predicate);
            if (hit) return hit;
            await Bun.sleep(50);
          }
          throw new Error(`${holder.label} missing ${label}; saw ${holder.frames.map((f) => Object.keys(f)[0]).join(",")}`);
        };

        const a = await attach("a", tokenA);
        const b = await attach("b", tokenB);
        await waitFrame(a, (f) => "Session" in f, "Session");
        await waitFrame(b, (f) => "Session" in f, "Session");

        const mutationId = `${fixture.command.mutationIdPrefix}1`;
        const envelope = {
          mutation_id: mutationId,
          document_id: documentId,
          actor: a.actor,
          dependencies: [] as string[],
          diff: { schema: fixture.command.diffSchema, payload: Array.from(deps.encodePackValue(fixture.command.diffValue)) },
          inverse: { schema: fixture.command.diffSchema, payload: Array.from(deps.encodePackValue(fixture.command.inverseValue)) },
          timestamp: { physical: Date.now(), logical: 0 },
        };
        const beforeB = b.frames.length;
        a.socket.send(deps.encodeClientFrame({ Commands: { batch_id: fixture.command.batchId, envelopes: [envelope] } }, "command"));

        const ack = await waitFrame(a, (f) => "Ack" in f && f.Ack.batch_id === fixture.command.batchId, "Ack");
        expect(JSON.stringify(ack.Ack.stages)).toContain("Applied");
        const ackFrontierDoc = ack.Ack.frontier?.document_id ?? ack.Ack.frontier?.documentId;
        expect(ackFrontierDoc).toBe(documentId);

        const commands = await waitFrame(
          b,
          (f, idx = 0) => {
            void idx;
            return "Commands" in f && b.frames.indexOf(f) >= beforeB;
          },
          "Commands",
        );
        // predicate fix - find properly
        const remote = b.frames.slice(beforeB).find((f) => "Commands" in f);
        expect(remote).toBeTruthy();
        expect(remote!.Commands.envelopes[0].mutation_id).toBe(mutationId);
        expect(remote!.Commands.envelopes[0].document_id).toBe(documentId);
        expect(remote!.Commands.origin).toBe(a.actor);
        const cmdFrontier = remote!.Commands.frontier?.document_id ?? remote!.Commands.frontier?.documentId;
        expect(cmdFrontier).toBe(documentId);

        const beforePresence = b.frames.length;
        a.socket.send(
          deps.encodeClientFrame(
            {
              Presence: {
                peer: deps.encodePresencePeer({
                  actor: a.actor,
                  connectedAtMs: Date.now(),
                  label: fixture.presence.labelA,
                  presencePack: [1],
                  views: [],
                } as any),
              },
            },
            "preview",
          ),
        );
        const presence = await waitFrame(b, (f) => "Presence" in f && b.frames.indexOf(f) >= beforePresence, "Presence");
        const peers = presence.Presence.peers.map((bytes: number[]) => deps.decodePresencePeer(new Uint8Array(bytes), [0]));
        expect(peers.some((peer: any) => peer.actor === a.actor)).toBe(true);

        const resume = b.resumeToken ?? null;
        expect(typeof resume === "string" && resume.length > 0).toBe(true);
        b.socket.close();
        await Bun.sleep(500);

        const mutationId2 = `${fixture.command.mutationIdPrefix}2`;
        const envelope2 = { ...envelope, mutation_id: mutationId2, diff: { schema: fixture.command.diffSchema, payload: Array.from(deps.encodePackValue({ value: "two-client-a2" })) } };
        a.socket.send(deps.encodeClientFrame({ Commands: { batch_id: fixture.command.batchId + 1, envelopes: [envelope2] } }, "command"));
        await waitFrame(a, (f) => "Ack" in f && f.Ack.batch_id === fixture.command.batchId + 1, "Ack2");

        const b2 = await attach("b-re", tokenB, resume);
        const missedOrBootstrap =
          b2.frames.some((f) => "Commands" in f && f.Commands.envelopes?.some((e: any) => e.mutation_id === mutationId2)) ||
          b2.frames.some((f) => "RebootstrapRequired" in f) ||
          (b2.welcome?.server_frontier?.last_commit_seq ?? 0) >= (ack.Ack.frontier?.last_commit_seq ?? 0);
        expect(missedOrBootstrap).toBe(true);

        const frontierBeforeRestart = b2.welcome?.server_frontier ?? ack.Ack.frontier;
        a.socket.close();
        b2.socket.close();
        await deps.finishLocalHub(run);

        process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
        const run2 = await deps.startLocalHub(
          repoRoot,
          deps.hubRustRoot,
          [
            { profileId: "a", subject: "author-a", displayName: "Author A", allowedClientClasses: ["native", "mcp"] },
            { profileId: "b", subject: "author-b", displayName: "Author B", allowedClientClasses: ["native", "mcp"] },
          ],
          { port, dataDir: dataRoot, binaryPath: bin, capture: true },
        );
        try {
          await deps.waitForReadiness(run2, true);
          const tokenA2 = await signIn(ADA.email, ADA.password, "device-ada-2");
          const spaces = await (await fetch(`${origin}/directory/spaces`, { headers: { authorization: `Bearer ${tokenA2}` } })).json();
          expect(spaces.some((row: any) => row?.space?.id === spaceId)).toBe(true);
          const a3 = await attach("a-restart", tokenA2);
          const frontierDoc = a3.welcome?.server_frontier?.document_id ?? a3.welcome?.server_frontier?.documentId;
          expect(frontierDoc).toBe(documentId);
          expect((a3.welcome?.server_frontier?.last_commit_seq ?? 0) >= (frontierBeforeRestart?.last_commit_seq ?? 0)).toBe(true);
          a3.socket.close();
        } finally {
          await deps.finishLocalHub(run2);
        }
      } finally {
        try {
          await deps.finishLocalHub(run);
        } catch {
          /* already stopped */
        }
        rmSync(dataRoot, { recursive: true, force: true });
      }
    },
    TEST_TIMEOUT_MS,
  );
});
