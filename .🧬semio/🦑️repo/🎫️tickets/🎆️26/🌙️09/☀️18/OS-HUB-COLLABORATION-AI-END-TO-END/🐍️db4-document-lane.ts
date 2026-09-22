/** 📄️ DB4 document lane — outcome 2 with a REAL DOCUMENT, on a hub whose whole durable state is a
 * live PostgreSQL (or Postgres documents + a Neo4j directory).
 *
 * What it proves, in order, every step from a measured value:
 *
 *   1  the hub boots on the database and `/readyz` answers **200** with every gate open — which
 *      needs a trusted catalog published from today's tree in the data root
 *   2  two humans sign in with credentials, share one space, both as Authors
 *   3  Author A creates a real `s.gis.gismap` document through the server-owned creation
 *      transaction (`POST /spaces/{s}/artifact-creations`), which mints the artifact id, the genesis
 *      checkpoint and the descriptor — all of it in the database
 *   4  both humans open their own document socket through their own open-plan + socket grant
 *   5  Author A commits one mutation; the hub persists it and **fans the same edit out to B's
 *      socket**
 *   6  the hub is stopped with SIGTERM and started again on the same database
 *   7  both re-attach and the Welcome frontier each one receives names A's edit — the edit survived
 *      the restart in the database, not in a process
 *
 * Usage: `bun 🐍️db4-document-lane.ts [--binary PATH] [--port N] [--data DIR] [--directory-backend postgres|neo4j]`
 * Every step prints one PASS/FAIL line; a failure exits non-zero naming the observed value. */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, "🌎️hub", "📦️packages", "🦀️rust", "Cargo.toml"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error("probe could not locate the repository root above " + start);
}

const ticketDir = fileURLToPath(new URL("./", import.meta.url));
const repoRoot = findRepoRoot(ticketDir);
const argv = process.argv.slice(2);
const flag = (name: string, fallback: string): string => {
  const index = argv.indexOf(`--${name}`);
  return index >= 0 ? String(argv[index + 1]) : fallback;
};
const binaryPath = flag("binary", join(repoRoot, ".🧬semio", "🦑️repo", "⚡️cache", "cargo", "target-db4", "debug", "os-hub"));
const port = Number(flag("port", "7691"));
const dataRoot = flag("data", join(repoRoot, ".🧬semio", "🌐hub", "db4-pg"));
const origin = `http://127.0.0.1:${port}`;
if (!existsSync(binaryPath)) throw new Error(`build the hub first (missing ${binaryPath})`);
mkdirSync(dataRoot, { recursive: true, mode: 0o700 });

const fixtureRoot = join(ticketDir, "🗑️generated", "db4-fixture", "checkpoint-publication-process-fixture");
const fixture = JSON.parse(readFileSync(join(fixtureRoot, "fixture.json"), "utf8")) as any;
const diffBytes = readFileSync(join(fixtureRoot, "payload", "diff.bin"));
const inverseBytes = readFileSync(join(fixtureRoot, "payload", "inverse.bin"));

const ADA = { email: "ada@db4.example.org", password: "correct horse battery staple", display: "Ada Lovelace" };
const BO = { email: "bo@db4.example.org", password: "another perfectly fine phrase", display: "Bo Peep" };

let failures = 0;
function check(label: string, actual: unknown, expected: unknown): void {
  const ok = JSON.stringify(actual) === JSON.stringify(expected);
  if (!ok) failures += 1;
  console.log(`${ok ? "PASS" : "FAIL"} ${label}: ${JSON.stringify(actual)}${ok ? "" : ` (expected ${JSON.stringify(expected)})`}`);
}

function provision(account: { email: string; password: string; display: string }): string {
  const result = spawnSync(binaryPath, ["credential", "set", "--email", account.email, "--display-name", account.display], {
    env: { ...process.env, OS_HUB_DATA: dataRoot },
    input: account.password,
    encoding: "utf8",
  });
  if (result.status !== 0) throw new Error(`credential set failed for ${account.email}: ${result.stderr}`);
  return result.stdout.trim();
}

type Answer = { readonly status: number; readonly body: string; readonly json: any };
async function call(method: string, path: string, options: { token?: string; body?: unknown; raw?: string } = {}): Promise<Answer> {
  const response = await fetch(`${origin}${path}`, {
    method,
    headers: {
      ...(options.body === undefined && options.raw === undefined ? {} : { "content-type": "application/json" }),
      ...(options.token === undefined ? {} : { authorization: `Bearer ${options.token}` }),
      origin: "http://127.0.0.1:6066",
    },
    ...(options.raw !== undefined ? { body: options.raw } : options.body === undefined ? {} : { body: JSON.stringify(options.body) }),
  });
  const body = await response.text();
  let json: any;
  try {
    json = JSON.parse(body);
  } catch {
    json = undefined;
  }
  return { status: response.status, body, json };
}

const signInBody = (email: string, password: string, device: string) => ({
  schema: "semio.hub.auth.credential-sign-in/v1",
  email,
  password,
  deviceInstanceId: device,
  clientClass: "browser" as const,
});

const { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } = await import(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema", "🟦️.ts"));
const { sealSpaceArtifactCreateV1, parseSpaceArtifactCreationStatusJsonV1 } = await import(
  join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema", "🌱️space-artifact-creation-v1", "🟦️.ts")
);
const { parseDocumentOpenPlanV1 } = await import(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema", "🟦️.ts"));
const { parseSocketGrantReceiptV1, socketGrantProtocolsV1 } = await import(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🟦️.ts"));
const { encodeClientFrame, decodeServerFrame } = await import(join(repoRoot, "🧰️framework", "🔨️modules", "📡️replication", "🟦️.ts"));
const { finishLocalHub, startLocalHub, waitForReadiness } = await import(join(repoRoot, "🌎️hub", "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"));

const hubRustRoot = join(repoRoot, "🌎️hub", "📦️packages", "🦀️rust");
const PROFILES = [{ profileId: "developer", subject: "local-developer-01", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] }];

async function command(token: string, requestId: string, body: any): Promise<Answer> {
  return await call("POST", "/directory/commands", { token, raw: directoryCommandRequestJson(sealDirectoryCommandRequestV1(requestId, body)) });
}

let run: any;
async function boot(phase: string): Promise<Record<string, any>> {
  const started = Date.now();
  process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
  run = await startLocalHub(repoRoot, hubRustRoot, PROFILES, { port, dataDir: dataRoot, binaryPath, capture: true });
  const readiness = await waitForReadiness(run, true);
  console.log(`boot ${phase}: pid=${run.child.pid} after ${Math.round((Date.now() - started) / 1000)}s`);
  return readiness;
}
async function halt(): Promise<number | null> {
  if (run === undefined) return null;
  const pid = run.child.pid;
  await finishLocalHub(run);
  console.log(`halt: pid=${pid} exit=${run.child.exitCode}`);
  run = undefined;
  return pid ?? null;
}

/** 🔌️ One human's own view of the document: their own open plan, their own socket grant, their own
 * socket, their own decoded frames. Nothing is shared between the two peers but the hub. */
type Peer = {
  readonly label: string;
  readonly token: string;
  readonly socket: WebSocket;
  readonly actorId: string;
  readonly frames: Record<string, any>[];
  error?: unknown;
};

async function attach(label: string, token: string, spaceId: string, documentId: string): Promise<Peer> {
  const documentRoot = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`;
  const planAnswer = await call("POST", `${documentRoot}/open-plan`, {
    token,
    body: { schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, requestedSurfaceId: fixture.surfaceId, clientInstanceId: `db4-${label}` },
  });
  if (planAnswer.status !== 200) throw new Error(`${label} open-plan refused: ${planAnswer.status} ${planAnswer.body.slice(0, 300)}`);
  const plan = parseDocumentOpenPlanV1(planAnswer.json, Date.now());
  const grantAnswer = await call("POST", `${documentRoot}/socket-grants`, { token, body: { schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt } });
  if (grantAnswer.status !== 200) throw new Error(`${label} socket grant refused: ${grantAnswer.status} ${grantAnswer.body.slice(0, 300)}`);
  const grant = parseSocketGrantReceiptV1(grantAnswer.json);
  const frames: Record<string, any>[] = [];
  const socket = new WebSocket(`ws://127.0.0.1:${port}${documentRoot}/socket/v1?surface=${encodeURIComponent(fixture.surfaceId)}`, [...socketGrantProtocolsV1(grant)]);
  socket.binaryType = "arraybuffer";
  const peer: Peer = { label, token, socket, actorId: grant.actorId, frames };
  socket.onmessage = (event) => {
    try {
      frames.push(decodeServerFrame(new Uint8Array(event.data as ArrayBuffer)).frame as unknown as Record<string, any>);
    } catch (error) {
      peer.error = error;
    }
  };
  socket.onerror = (event) => {
    peer.error = event;
  };
  await new Promise<void>((resolveOpen, rejectOpen) => {
    const timer = setTimeout(() => rejectOpen(new Error(`${label} socket open deadline exceeded`)), 15_000);
    socket.onopen = () => {
      clearTimeout(timer);
      resolveOpen();
    };
  });
  socket.send(
    encodeClientFrame(
      {
        SocketHelloV1: {
          wire_version: 1,
          protocol_version: 1,
          schema: fixture.artifact.schema,
          pack_schema_hash: Array.from(Buffer.from(fixture.artifact.packSchemaHash, "hex")),
          resume_token: null,
          frontier: null,
        },
      },
      "command",
    ),
  );
  return peer;
}

async function waitFrame<T>(peer: Peer, select: (frame: Record<string, any>) => T | undefined, label: string, budgetMs = 15_000): Promise<T> {
  const deadline = Date.now() + budgetMs;
  for (;;) {
    for (const frame of peer.frames) {
      const selected = select(frame);
      if (selected !== undefined) return selected;
    }
    if (peer.error) throw peer.error;
    if (peer.socket.readyState >= WebSocket.CLOSING) throw new Error(`${peer.label} socket closed before ${label}`);
    if (Date.now() >= deadline) throw new Error(`${peer.label} never saw ${label} within ${budgetMs}ms (frames: ${peer.frames.map((frame) => Object.keys(frame)[0]).join(",")})`);
    await Bun.sleep(25);
  }
}

const hex = (bytes: readonly number[] | undefined): string => (bytes ?? []).map((byte) => byte.toString(16).padStart(2, "0")).join("");

let documentId = "";
let spaceId = "";
try {
  console.log(`OS_HUB_DATA=${dataRoot} port=${port} binary=${binaryPath}`);
  console.log(`storage=${process.env.OS_HUB_STORAGE_BACKEND} directory=${process.env.OS_HUB_DIRECTORY_BACKEND}`);
  const adaId = provision(ADA);
  const boId = provision(BO);
  console.log(`operator bootstrap: ada=${adaId} bo=${boId}`);

  // 1️⃣ the hub answers /readyz with a loaded catalog
  const readiness = await boot("first");
  const firstRunId = (await call("GET", "/healthz")).json?.runId ?? "";
  const ready = await call("GET", "/readyz");
  check("1 readyz status", ready.status, 200);
  check("1 readyz says ready", ready.json?.status, "ready");
  check("1 every gate is open", [ready.json?.directory?.ready, ready.json?.storage?.ready, ready.json?.artifactAuthority?.ready, ready.json?.artifactCasBarrier?.ready, ready.json?.artifactPublication?.ready, ready.json?.adminAssets?.ready], [true, true, true, true, true, true]);
  void readiness;

  // 2️⃣ two humans, one space, both Authors
  const adaMint = await call("POST", "/auth/sessions", { body: signInBody(ADA.email, ADA.password, "device-ada") });
  const boMint = await call("POST", "/auth/sessions", { body: signInBody(BO.email, BO.password, "device-bo") });
  const adaToken: string = adaMint.json?.token ?? "";
  const boToken: string = boMint.json?.token ?? "";
  check("2 both humans signed in", [adaMint.status, boMint.status], [200, 200]);
  const created = await command(adaToken, "d".repeat(32), { kind: "create-space", name: "DB4 Document Studio", spaceKind: "studio", visibility: "private" });
  check("2 create-space status", created.status, 202);
  spaceId = (created.json?.events ?? []).find((event: any) => event?.body?.kind === "space.created")?.body?.spaceId ?? "";
  check("2 the new space has an id", spaceId.length > 0, true);
  const promoted = await command(adaToken, "e".repeat(32), { kind: "upsert-member", spaceId, email: BO.email, role: "author" });
  check("2 the second human is an Author", promoted.status, 202);
  const creationCatalog = await call("GET", `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`, { token: adaToken });
  check("2 the hub serves the creation catalog", creationCatalog.status, 200);
  check("2 the loaded catalog is the published generation", creationCatalog.json?.catalogGenerationId, fixture.generationId);
  check("2 the catalog offers the kind this lane creates", (creationCatalog.json?.kinds ?? []).map((kind: any) => kind.kindId), [fixture.artifact.kind]);

  // 3️⃣ a REAL document through the server-owned creation transaction
  const request = sealSpaceArtifactCreateV1({ requestId: "1".repeat(32), expectedCatalogGenerationId: fixture.generationId, kindId: fixture.artifact.kind, name: "DB4 Verified GIS Map" });
  const creationRoot = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const accepted = await call("POST", creationRoot, { token: adaToken, raw: JSON.stringify(request) });
  check("3 artifact-creation accepted", [200, 202].includes(accepted.status), true);
  let status = parseSpaceArtifactCreationStatusJsonV1(accepted.body);
  let statusBody = accepted.body;
  const creationDeadline = Date.now() + 120_000;
  while (status.phase !== "ready") {
    if (Date.now() >= creationDeadline) throw new Error(`creation stalled in ${status.phase}: ${statusBody}`);
    if (!["accepted", "preparing", "indeterminate"].includes(status.phase)) {
      console.log(`creation status body: ${statusBody}`);
      console.log(`hub tail: ${String(run?.output?.() ?? "").slice(-4000)}`);
      throw new Error(`creation reached ${status.phase}`);
    }
    await Bun.sleep(25);
    const polled = await call("GET", `${creationRoot}/${encodeURIComponent(request.requestId)}`, { token: adaToken });
    statusBody = polled.body;
    status = parseSpaceArtifactCreationStatusJsonV1(polled.body);
  }
  documentId = status.ready?.artifactId ?? "";
  check("3 the creation reached Ready", status.phase, "ready");
  check("3 the hub minted an artifact id", /^artifact-[0-9a-f]{32}$/u.test(documentId), true);
  check("3 the created kind is the selected one", [status.ready?.kindId, status.ready?.artifactSchema], [fixture.artifact.kind, fixture.artifact.schema]);
  const descriptor = await call("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, { token: adaToken });
  check("3 the document has a status route", descriptor.status, 200);

  // 4️⃣ two sessions attach to the same document
  const ada = await attach("ada", adaToken, spaceId, documentId);
  const bo = await attach("bo", boToken, spaceId, documentId);
  const adaWelcome = await waitFrame(ada, (frame) => ("Welcome" in frame ? frame.Welcome : undefined), "Welcome");
  const boWelcome = await waitFrame(bo, (frame) => ("Welcome" in frame ? frame.Welcome : undefined), "Welcome");
  check("4 both sessions received Welcome", [typeof adaWelcome === "object", typeof boWelcome === "object"], [true, true]);
  check("4 the welcome frontier names the document itself", [adaWelcome.server_frontier?.document_id, boWelcome.server_frontier?.document_id], [documentId, documentId]);
  check("4 both open at the same genesis frontier", adaWelcome.server_frontier?.head_edit_ordinal, boWelcome.server_frontier?.head_edit_ordinal);
  check("4 the two sessions are different actors", ada.actorId !== bo.actorId, true);

  // 5️⃣ one edit, committed by A, crossing to B
  const mutationId = "db4documentlaneedit0000000000001";
  const boFramesBefore = bo.frames.length;
  ada.socket.send(
    encodeClientFrame(
      {
        Commands: {
          batch_id: 1,
          envelopes: [
            {
              mutation_id: mutationId,
              document_id: documentId,
              actor: ada.actorId,
              dependencies: [],
              diff: { schema: fixture.payload.diff.schema, payload: Array.from(diffBytes) },
              inverse: { schema: fixture.payload.inverse.schema, payload: Array.from(inverseBytes) },
              timestamp: { actor: 1, physical_ms: Date.now(), logical: 1 },
            },
          ],
        },
      },
      "command",
    ),
  );
  const ack = await waitFrame(ada, (frame) => ("Ack" in frame && frame.Ack.batch_id === 1 ? frame.Ack : undefined), "Ack");
  check("5 the hub persisted the edit", ack.stages?.some((stage: any) => stage === "Persisted"), true);
  check("5 the hub applied the edit", ack.stages?.some((stage: any) => stage?.Applied?.outcome === "Accepted"), true);
  check("5 the frontier advanced to the edit", [ack.frontier?.head_edit_id, ack.frontier?.head_edit_ordinal], [mutationId, 1]);
  const crossed = await waitFrame(
    bo,
    (frame) => ("Commands" in frame && frame.Commands.envelopes?.some((envelope: any) => envelope.mutation_id === mutationId) ? frame.Commands : undefined),
    "A's edit",
  );
  check("5 the edit crossed to the second session", crossed.envelopes?.[0]?.mutation_id, mutationId);
  check("5 the edit crossed as a NEW frame, not a replay of the welcome", bo.frames.length > boFramesBefore, true);
  check("5 the crossed frontier names the document itself", crossed.frontier?.document_id, documentId);
  const chainAfterEdit = hex(ack.frontier?.chain_hash);

  // 6️⃣ SIGTERM + restart on the same database
  ada.socket.close(1000, "db4 restart");
  bo.socket.close(1000, "db4 restart");
  const firstPid = await halt();
  check("6 the hub stopped answering", await call("GET", "/healthz").then((answer) => answer.status).catch(() => 0), 0);
  await boot("restart");
  const restartHealth = await call("GET", "/healthz");
  check("6 healthz after restart", restartHealth.status, 200);
  check("6 the restart is a NEW run", restartHealth.json?.runId !== firstRunId && (firstPid === null || run.child.pid !== firstPid), true);
  check("6 readyz after restart", (await call("GET", "/readyz")).status, 200);

  // 7️⃣ both re-attach and see the edit
  const adaAgain = await call("POST", "/auth/sessions", { body: signInBody(ADA.email, ADA.password, "device-ada-2") });
  const boAgain = await call("POST", "/auth/sessions", { body: signInBody(BO.email, BO.password, "device-bo-2") });
  check("7 both humans sign in again", [adaAgain.status, boAgain.status], [200, 200]);
  const listed = await call("GET", "/directory/spaces", { token: adaAgain.json?.token });
  check("7 the space survived the restart", (Array.isArray(listed.json) ? listed.json : []).some((row: any) => row?.space?.id === spaceId), true);
  const descriptorAgain = await call("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, { token: adaAgain.json?.token });
  check("7 the document survived the restart", descriptorAgain.status, 200);
  const adaBack = await attach("ada-again", adaAgain.json?.token, spaceId, documentId);
  const boBack = await attach("bo-again", boAgain.json?.token, spaceId, documentId);
  const adaBackWelcome = await waitFrame(adaBack, (frame) => ("Welcome" in frame ? frame.Welcome : undefined), "Welcome after restart");
  const boBackWelcome = await waitFrame(boBack, (frame) => ("Welcome" in frame ? frame.Welcome : undefined), "Welcome after restart");
  check("7 the first session re-attaches at A's edit", [adaBackWelcome.server_frontier?.head_edit_id, adaBackWelcome.server_frontier?.head_edit_ordinal], [mutationId, 1]);
  check("7 the second session re-attaches at the same edit", [boBackWelcome.server_frontier?.head_edit_id, boBackWelcome.server_frontier?.head_edit_ordinal], [mutationId, 1]);
  check("7 the chain hash is byte-identical across the restart", [hex(adaBackWelcome.server_frontier?.chain_hash), hex(boBackWelcome.server_frontier?.chain_hash)], [chainAfterEdit, chainAfterEdit]);
  check("7 the re-attached frontier still names the document itself", adaBackWelcome.server_frontier?.document_id, documentId);
  adaBack.socket.close(1000, "db4 done");
  boBack.socket.close(1000, "db4 done");
  console.log(`document=${documentId} space=${spaceId} mutation=${mutationId} chain=${chainAfterEdit}`);
} finally {
  await halt().catch(() => undefined);
}

console.log(failures === 0 ? "ALL CHECKS PASSED" : `${failures} CHECK(S) FAILED`);
process.exit(failures === 0 ? 0 : 1);
