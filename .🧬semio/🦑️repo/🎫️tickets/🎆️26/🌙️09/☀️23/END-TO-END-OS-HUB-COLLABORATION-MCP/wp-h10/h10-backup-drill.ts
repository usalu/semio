#!/usr/bin/env bun
/** 🗄️ H10 backup/restore drill client (ticket 26/09/23 session 12), against a running hub signed in as user1.
 *   bun h10-backup-drill.ts seed <origin> <state.json> <kindId> <edits>   create a space + one document of <kindId>, apply
 *                                                                        <edits> chained edits over the document socket,
 *                                                                        record frontier, active checkpoint pair digest,
 *                                                                        descriptor and directory listing in <state.json>
 *   bun h10-backup-drill.ts verify <origin> <state.json>                  reopen the recorded document on a (restored) hub:
 *                                                                        same frontier, checkpoint, descriptor, listing,
 *                                                                        and the next chained edit is accepted */
import { createHash, randomBytes } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";

const [mode, origin, statePath, kindArg, editsArg] = process.argv.slice(2);
const repo = "/Users/ueli/Documents/semio";
const { encodeClientFrame, decodeServerFrame } = await import(`${repo}/🧰️framework/🔨️modules/📡️replication/🟦️.ts`);
const { parseDocumentSocketGrantReceiptV1 } = await import(`${repo}/🧰️framework/🛍️products/💻️os/🟦️.ts`);
const { sealSpaceArtifactCreateV1 } = await import(`${repo}/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts`);
const wsOrigin = origin!.replace(/^http/u, "ws");
const hex = () => randomBytes(16).toString("hex");
const call = async (method: string, path: string, token: string | undefined, body?: unknown, accept?: string) => {
  const response = await fetch(`${origin}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}), ...(accept ? { accept } : {}) }, ...(body === undefined ? {} : { body: JSON.stringify(body) }), signal: AbortSignal.timeout(120_000) });
  const bytes = new Uint8Array(await response.arrayBuffer());
  const text = new TextDecoder().decode(bytes);
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {}
  return { status: response.status, text, json, bytes };
};
const signIn = async () => {
  const response = await call("POST", "/auth/sessions", undefined, { schema: "semio.hub.auth.credential-sign-in/v1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1", deviceInstanceId: `h10drill${randomBytes(12).toString("hex")}`, clientClass: "browser" });
  if (response.status !== 200) throw new Error(`sign-in ${response.status} ${response.text.slice(0, 200)}`);
  return String(response.json.token);
};
type Frame = Record<string, any>;
const open = async (token: string, spaceId: string, documentId: string) => {
  const scope = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`;
  const plan = await call("POST", `${scope}/open-plan`, token, { schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, clientInstanceId: "h10-backup-drill" });
  if (plan.status !== 200) throw new Error(`open-plan ${plan.status} ${plan.text.slice(0, 300)}`);
  const grant = await call("POST", `${scope}/socket-grants`, token, { schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.json.receipt });
  if (grant.status !== 200) throw new Error(`socket-grant ${grant.status} ${grant.text.slice(0, 300)}`);
  const granted = parseDocumentSocketGrantReceiptV1(grant.json);
  const socket = new WebSocket(`${wsOrigin}/scopes/${encodeURIComponent(`${spaceId}/${documentId}`)}/document/ws?surface=${encodeURIComponent(plan.json.surface.surfaceId)}`, ["semio.session.v1", token]);
  socket.binaryType = "arraybuffer";
  const frames: Frame[] = [];
  const waiters: Array<(frame: Frame) => void> = [];
  socket.addEventListener("message", (event) => {
    if (!(event.data instanceof ArrayBuffer)) return;
    const frame = decodeServerFrame(new Uint8Array(event.data)).frame as Frame;
    if ("Commands" in frame || "Presence" in frame) return;
    frames.push(frame);
    for (const waiter of [...waiters]) waiter(frame);
  });
  const waitFrame = (pred: (frame: Frame) => boolean, label: string, ms = 60_000) =>
    new Promise<Frame>((resolve, reject) => {
      const hit = frames.find(pred);
      if (hit) return resolve(hit);
      const onFrame = (frame: Frame) => {
        if (!pred(frame)) return;
        clearTimeout(timer);
        waiters.splice(waiters.indexOf(onFrame), 1);
        resolve(frame);
      };
      waiters.push(onFrame);
      const timer = setTimeout(() => reject(new Error(`missing ${label}; frames=${JSON.stringify(frames.slice(-3)).slice(0, 400)}`)), ms);
    });
  for (let i = 0; i < 400 && socket.readyState === WebSocket.CONNECTING; i++) await Bun.sleep(50);
  if (socket.readyState !== WebSocket.OPEN) throw new Error("document socket did not open");
  const packSchemaHash = [...(plan.json.artifact.packSchemaHash.match(/../gu) ?? [])].map((pair: string) => Number.parseInt(pair, 16));
  socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: plan.json.artifact.schema, pack_schema_hash: packSchemaHash, resume_token: null, frontier: null } }, "command"));
  const welcome = await waitFrame((frame) => "Welcome" in frame || "Error" in frame, "Welcome");
  if ("Error" in welcome) throw new Error(`refused: ${JSON.stringify(welcome.Error)}`);
  const edit = async (index: number, last: string) => {
    const mutationId = `h10-drill-${documentId}-${index}`;
    socket.send(encodeClientFrame({ Commands: { batch_id: index + 1, envelopes: [{ mutation_id: mutationId, document_id: documentId, actor: granted.actorId, dependencies: last ? [last] : [], diff: { schema: plan.json.artifact.schema, payload: Array.from(new TextEncoder().encode(`drill:${index}:${"b".repeat(512)}`)) }, inverse: { schema: plan.json.artifact.schema, payload: [] }, timestamp: { actor: 1, physical_ms: Date.now(), logical: 0 } }] } }, "command"));
    const acked = await waitFrame((frame) => "Ack" in frame && frame.Ack.batch_id === index + 1, `Ack ${index}`);
    if (!JSON.stringify(acked.Ack.stages).includes("Accepted")) throw new Error(`edit ${index} not accepted: ${JSON.stringify(acked.Ack)}`);
    return mutationId;
  };
  return { welcome: welcome.Welcome, edit, close: () => socket.close(1000, "drill") };
};
const snapshot = async (token: string, spaceId: string, documentId: string) => {
  const scope = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`;
  const descriptor = await call("GET", scope, token);
  const pair = await call("GET", `${scope}/active-checkpoint/pair`, token, undefined, "application/vnd.semio.canonical-checkpoint-pair.v1");
  const spaces = await call("GET", "/directory/spaces", token);
  return {
    descriptorStatus: descriptor.status,
    descriptorSha256: createHash("sha256").update(descriptor.bytes).digest("hex"),
    pairStatus: pair.status,
    pairBytes: pair.bytes.length,
    pairSha256: createHash("sha256").update(pair.bytes).digest("hex"),
    spaceListed: JSON.stringify(spaces.json ?? []).includes(spaceId),
  };
};
const frontierOf = (welcome: any) => ({ lastCommitSeq: welcome.server_frontier.last_commit_seq, headEditId: welcome.server_frontier.head_edit_id, headEditOrdinal: welcome.server_frontier.head_edit_ordinal, chainHash: Buffer.from(welcome.server_frontier.chain_hash).toString("hex") });

if (mode === "seed") {
  const token = await signIn();
  const created = await call("POST", "/directory/commands", token, { schema: "semio.directory.command-request.v1", requestId: hex(), command: { kind: "create-space", name: `H10 backup drill ${new Date().toISOString()}`, spaceKind: "studio", visibility: "private" } });
  const spaceId = created.json?.events?.find((event: any) => event?.body?.kind === "space.created")?.body?.spaceId;
  if (created.status !== 202 || typeof spaceId !== "string") throw new Error(`create-space ${created.status} ${created.text.slice(0, 300)}`);
  const catalog = await call("GET", `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`, token);
  const requestId = hex();
  const route = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const started = Date.now();
  const accepted = await call("POST", route, token, sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: catalog.json.catalogGenerationId, kindId: kindArg, name: `H10 drill ${kindArg}` }));
  if (accepted.status !== 202) throw new Error(`creation ${accepted.status} ${accepted.text.slice(0, 300)}`);
  let status = accepted.json;
  while (["accepted", "preparing", "indeterminate"].includes(status.phase)) {
    await Bun.sleep(500);
    status = (await call("GET", `${route}/${requestId}`, token)).json;
  }
  if (status.phase !== "ready") throw new Error(`creation ended ${JSON.stringify(status)}`);
  const documentId = status.ready.artifactId as string;
  console.log(`created ${kindArg} ${documentId} in ${Date.now() - started} ms`);
  const session = await open(token, spaceId, documentId);
  let last = "";
  const edits = Number(editsArg ?? 20);
  for (let index = 0; index < edits; index += 1) last = await session.edit(index, last);
  session.close();
  const reopened = await open(token, spaceId, documentId);
  const state = { spaceId, documentId, kindId: kindArg, edits, last, frontier: frontierOf(reopened.welcome), ...(await snapshot(token, spaceId, documentId)) };
  reopened.close();
  writeFileSync(statePath!, `${JSON.stringify(state, null, 2)}\n`);
  console.log(`SEED ${JSON.stringify(state)}`);
  process.exit(0);
}

if (mode === "verify") {
  const state = JSON.parse(readFileSync(statePath!, "utf8"));
  const token = await signIn();
  const session = await open(token, state.spaceId, state.documentId);
  const frontier = frontierOf(session.welcome);
  const now = await snapshot(token, state.spaceId, state.documentId);
  const checks = {
    frontier: JSON.stringify(frontier) === JSON.stringify(state.frontier),
    descriptor: now.descriptorStatus === 200 && now.descriptorSha256 === state.descriptorSha256,
    checkpointPair: now.pairStatus === 200 && now.pairSha256 === state.pairSha256,
    spaceListed: now.spaceListed,
    nextEditAccepted: false,
  };
  await session.edit(state.edits, state.last);
  checks.nextEditAccepted = true;
  session.close();
  console.log(`VERIFY ${JSON.stringify({ checks, frontier, expected: state.frontier, now })}`);
  process.exit(Object.values(checks).every(Boolean) ? 0 : 1);
}
throw new Error(`unknown mode ${mode}`);
