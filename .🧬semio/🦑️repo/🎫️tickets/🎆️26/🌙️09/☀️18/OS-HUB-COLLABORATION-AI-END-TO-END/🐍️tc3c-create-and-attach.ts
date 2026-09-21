/** 🌱️ TC3c — create ONE document of a named kind on a hub and attach a live document socket to it,
 * browser-free, printing EVERY http code the product's own routes answer.
 *
 * The chain is the product's, in the order the hub requires it (GM1 §4b/§5, C3 §2, HS1's probe):
 *   sign-in → create-space → `GET  /spaces/{s}/artifact-creations` (the creation catalog)
 *           → `POST /spaces/{s}/artifact-creations` polled to `ready`
 *           → `POST …/open-plan`
 *           → `POST …/execution-target/{manifest,component,descriptor}`
 *           → `POST …/socket-grants`
 *           → `GET  …/socket/v1` upgraded with `Sec-WebSocket-Protocol: semio.socket.v1, <grant>`
 *             and a `SocketHelloV1` command frame, waiting for the hub's own `Session` frame.
 *
 * The execution-target reads are a CLIENT-side lease the hub never requires for the socket, but the
 * brief asks for their codes, so they are read and reported rather than skipped.
 *
 * Usage: bun 🐍️tc3c-create-and-attach.ts <origin> <kindId> [email] [password] [spaceName] */
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { randomBytes } from "node:crypto";

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

const repoRoot = findRepoRoot(import.meta.dir);
const { encodeClientFrame, decodeServerFrame } = await import(join(repoRoot, "🧰️framework", "🔨️modules", "📡️replication", "🟦️.ts"));
const { sealSpaceArtifactCreateV1, parseSpaceArtifactCreationStatusJsonV1 } = await import(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema", "🌱️space-artifact-creation-v1", "🟦️.ts"));

const origin = process.argv[2] ?? "http://127.0.0.1:7651";
const kindId = process.argv[3] ?? "s.note.note";
const email = process.argv[4] ?? "user1@semio.dev";
const password = process.argv[5] ?? "gm1-local-dev-pass-1";
const spaceName = process.argv[6] ?? `TC3c ${kindId} ${randomBytes(4).toString("hex")}`;
const say = (line: string) => console.log(`${new Date().toISOString().slice(11, 23)} ${line}`);

const minted = await fetch(`${origin}/auth/sessions`, {
  method: "POST",
  headers: { "content-type": "application/json" },
  body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: "tc3c-create-attach", clientClass: "browser" }),
});
const token: string = ((await minted.json()) as { token?: string })?.token ?? "";
say(`HTTP ${minted.status} POST /auth/sessions  (${email}) tokenShape=${/^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u.test(token)}`);
if (minted.status !== 200) process.exit(2);
const headers = { authorization: `Bearer ${token}`, "content-type": "application/json" };
const post = async (route: string, body: unknown): Promise<{ status: number; text: string }> => {
  const response = await fetch(`${origin}${route}`, { method: "POST", headers, body: typeof body === "string" ? body : JSON.stringify(body) });
  return { status: response.status, text: await response.text() };
};

const ready = await fetch(`${origin}/readyz`).then(async (r) => ({ status: r.status, body: (await r.json()) as any }));
say(`HTTP ${ready.status} GET  /readyz  artifactAuthority.ready=${ready.body?.artifactAuthority?.ready} features.openPlan=${ready.body?.features?.openPlan}`);

const created = await post("/directory/commands", { schema: "semio.directory.command-request.v1", requestId: randomBytes(16).toString("hex"), command: { kind: "create-space", name: spaceName, spaceKind: "studio", visibility: "private" } });
const spaceId: string = (JSON.parse(created.text) as any)?.events?.find((candidate: any) => candidate?.body?.kind === "space.created")?.body?.spaceId ?? "";
say(`HTTP ${created.status} POST /directory/commands  (create-space) space=${spaceId}`);
if (!spaceId) {
  say(`CREATE-SPACE body=${created.text.slice(0, 400)}`);
  process.exit(3);
}

const catalogResponse = await fetch(`${origin}/spaces/${encodeURIComponent(spaceId)}/artifact-creations`, { headers: { authorization: headers.authorization } });
const catalog = (await catalogResponse.json()) as any;
const generationId: string = catalog?.catalogGenerationId ?? "";
say(`HTTP ${catalogResponse.status} GET  /spaces/{s}/artifact-creations  generation=${generationId.slice(0, 16)}… kinds=${(catalog.kinds ?? []).map((row: any) => row.kindId).join(",")}`);
const kind = (catalog.kinds ?? []).find((row: any) => row.kindId === kindId);
if (!kind) {
  say(`CREATION-CATALOG does not offer ${kindId}`);
  process.exit(4);
}

const request = sealSpaceArtifactCreateV1({ requestId: randomBytes(16).toString("hex"), expectedCatalogGenerationId: generationId, kindId: kind.kindId, name: `TC3c ${kindId}` });
const creationRoute = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
const creationResponse = await post(creationRoute, JSON.stringify(request));
say(`HTTP ${creationResponse.status} POST /spaces/{s}/artifact-creations  kind=${kind.kindId}`);
if (creationResponse.status !== 201 && creationResponse.status !== 202 && creationResponse.status !== 200) {
  say(`ARTIFACT-CREATION body=${creationResponse.text.slice(0, 600)}`);
  process.exit(5);
}
let creation = parseSpaceArtifactCreationStatusJsonV1(creationResponse.text) as any;
const creationDeadline = Date.now() + 120_000;
let polls = 0;
while (creation.phase !== "ready") {
  if (!["accepted", "preparing", "indeterminate"].includes(creation.phase) || Date.now() >= creationDeadline) {
    say(`ARTIFACT-CREATION reached ${creation.phase}: ${JSON.stringify(creation).slice(0, 600)}`);
    process.exit(6);
  }
  await Bun.sleep(100);
  const poll = await fetch(`${origin}${creationRoute}/${encodeURIComponent(request.requestId)}`, { headers: { authorization: headers.authorization } });
  if (polls === 0) say(`HTTP ${poll.status} GET  /spaces/{s}/artifact-creations/{requestId}  (poll)`);
  polls += 1;
  creation = parseSpaceArtifactCreationStatusJsonV1(await poll.text()) as any;
}
const documentId: string = creation.ready.artifactId;
say(`CREATED document=${documentId} kind=${creation.ready.kindId} schema=${creation.ready.artifactSchema}`);

const surfaceId: string = creation.ready.surfaceId ?? `${kindId}@1/*#editor`;
const intent = { schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, requestedSurfaceId: surfaceId, clientInstanceId: "tc3c-create-attach" };
const documentRoute = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`;
const planResponse = await post(`${documentRoute}/open-plan`, intent);
say(`HTTP ${planResponse.status} POST …/open-plan`);
if (planResponse.status !== 200) {
  say(`OPEN-PLAN body=${planResponse.text.slice(0, 600)}`);
  process.exit(7);
}
const plan = JSON.parse(planResponse.text) as any;
say(`OPEN-PLAN surface=${JSON.stringify(plan.surface)} schema=${plan.artifact.schema} packSchemaHash=${String(plan.artifact.packSchemaHash).slice(0, 16)}…`);
say(`OPEN-PLAN package=${JSON.stringify(plan.package)}`);

for (const leaf of ["manifest", "component", "descriptor"] as const) {
  const lease = await post(`${documentRoute}/execution-target/${leaf}`, intent);
  say(`HTTP ${lease.status} POST …/execution-target/${leaf}  bytes=${lease.text.length}${lease.status === 200 ? "" : ` body=${lease.text.slice(0, 200)}`}`);
}

const grantResponse = await post(`${documentRoute}/socket-grants`, { schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt });
say(`HTTP ${grantResponse.status} POST …/socket-grants`);
if (grantResponse.status !== 200) {
  say(`SOCKET-GRANT body=${grantResponse.text.slice(0, 400)}`);
  process.exit(8);
}
const receipt = JSON.parse(grantResponse.text) as { grant: string; protocol: string };

const packSchemaHash = [...(String(plan.artifact.packSchemaHash).match(/../gu) ?? [])].map((pair) => Number.parseInt(pair, 16));
const wsUrl = `${origin.replace(/^http/u, "ws")}${documentRoute}/socket/v1?surface=${encodeURIComponent(surfaceId)}`;
const socket = new WebSocket(wsUrl, [receipt.protocol, receipt.grant]);
socket.binaryType = "arraybuffer";
const seen: string[] = [];
let opened = false;
socket.addEventListener("open", () => {
  opened = true;
  say(`WS-OPEN protocol=${socket.protocol}`);
  const hello = encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: plan.artifact.schema, pack_schema_hash: packSchemaHash, resume_token: null, frontier: null } }, "command");
  socket.send(hello);
  say(`WS-HELLO sent bytes=${hello.byteLength}`);
});
socket.addEventListener("message", (event) => {
  if (!(event.data instanceof ArrayBuffer)) {
    seen.push(`text:${String(event.data).slice(0, 120)}`);
    say(`WS-FRAME text ${String(event.data).slice(0, 200)}`);
    return;
  }
  let named = `binary(${event.data.byteLength})`;
  try {
    named = Object.keys(decodeServerFrame(new Uint8Array(event.data)).frame)[0]!;
  } catch (error) {
    named = `undecodable(${(error as Error).message.slice(0, 60)})`;
  }
  seen.push(named);
  say(`WS-FRAME ${named} bytes=${event.data.byteLength}`);
});
socket.addEventListener("error", () => say("WS-ERROR"));
socket.addEventListener("close", (event) => say(`WS-CLOSE code=${event.code} reason=${event.reason}`));

const deadline = Date.now() + 20_000;
while (Date.now() < deadline && !seen.some((name) => name.startsWith("Session"))) await Bun.sleep(250);
const session = seen.some((name) => name.startsWith("Session"));
say(`RESULT kind=${kindId} space=${spaceId} document=${documentId} opened=${opened} frames=${seen.join("|")} session=${session}`);
socket.close();
process.exit(opened && session ? 0 : 1);
