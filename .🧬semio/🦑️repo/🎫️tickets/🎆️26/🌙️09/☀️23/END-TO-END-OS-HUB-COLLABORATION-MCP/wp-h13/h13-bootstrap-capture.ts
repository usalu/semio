/** 📥️ H13: opens one document read-only (plan → socket grant → hello, no edits) and records every server frame until the
 * bootstrap settles: the Welcome's bootstrap kind and frontier, each bootstrap frame's kind and size, and the mutation ids of every
 * envelope the hub delivers (tail / Commands), plus which of those ids also occur inside the served pair bytes — an id that is
 * both in the checkpoint history and in the tail is delivered twice. Raw frames are written to <out>.
 * usage: bun h13-bootstrap-capture.ts <origin> <space> <document> <out.json> */
import { writeFileSync } from "node:fs";
import { hubProbeCall, hubProbeOpenPlan, hubProbeSignIn } from "/Users/ueli/Documents/semio/🌎️hub/🤝️integration-harness/🟦️.ts";
import { decodeServerFrame, encodeClientFrame } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts";
import { parseDocumentSocketGrantReceiptV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts";

const [origin, spaceId, documentId, out] = process.argv.slice(2);
const token = await hubProbeSignIn(origin!, process.env.OS_HUB_PROBE_EMAIL ?? "", process.env.OS_HUB_PROBE_PASSWORD ?? "", "h13-capture-");
const scope = `/spaces/${encodeURIComponent(spaceId!)}/documents/${encodeURIComponent(documentId!)}`;
const plan = await hubProbeOpenPlan(origin!, token, spaceId!, documentId!, "h13-capture");
if (plan.status !== 200) throw new Error(`open-plan ${plan.status} ${plan.text.slice(0, 300)}`);
const grant = await hubProbeCall(origin!, "POST", `${scope}/socket-grants`, token, JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.json.receipt }));
if (grant.status !== 200) throw new Error(`socket-grant ${grant.status} ${grant.text.slice(0, 300)}`);
parseDocumentSocketGrantReceiptV1(grant.json);
const socket = new WebSocket(`${origin!.replace(/^http/u, "ws")}/scopes/${encodeURIComponent(`${spaceId}/${documentId}`)}/document/ws?surface=${encodeURIComponent(plan.json.surface.surfaceId)}`, ["semio.session.v1", token]);
socket.binaryType = "arraybuffer";
const frames: { kind: string; bytes: number; frame: any }[] = [];
let last = Date.now();
socket.addEventListener("message", (event) => {
  if (!(event.data instanceof ArrayBuffer)) return;
  const bytes = new Uint8Array(event.data);
  let frame: any;
  try {
    frame = decodeServerFrame(bytes).frame;
  } catch (error) {
    frames.push({ kind: `undecodable:${String(error).slice(0, 80)}`, bytes: bytes.length, frame: null });
    return;
  }
  last = Date.now();
  frames.push({ kind: Object.keys(frame)[0] ?? "?", bytes: bytes.length, frame });
});
for (let tick = 0; tick < 400 && socket.readyState === WebSocket.CONNECTING; tick += 1) await new Promise((resolveDelay) => setTimeout(resolveDelay, 50));
const packSchemaHash = [...(String(plan.json.artifact.packSchemaHash).match(/../gu) ?? [])].map((pair) => Number.parseInt(pair, 16));
socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: plan.json.artifact.schema, pack_schema_hash: packSchemaHash, resume_token: null, frontier: null } }, "command"));
while (Date.now() - last < 4_000) await new Promise((resolveDelay) => setTimeout(resolveDelay, 250));
socket.close(1000, "capture");
const replacer = (_: string, value: unknown) => (value instanceof Uint8Array ? { bytes: value.length, hex: Buffer.from(value).toString("hex") } : typeof value === "bigint" ? value.toString() : value);
writeFileSync(out!, JSON.stringify(frames, replacer, 1));
const text = JSON.stringify(frames, replacer);
const envelopeIds = [...text.matchAll(/"mutation_id":"([^"]+)"/gu)].map((match) => match[1]!);
const counts = new Map<string, number>();
for (const id of envelopeIds) counts.set(id, (counts.get(id) ?? 0) + 1);
const pairHex = [...text.matchAll(/"hex":"([0-9a-f]+)"/gu)].map((match) => match[1]!).join("");
const pairText = Buffer.from(pairHex, "hex").toString("latin1");
const alsoInPair = [...counts.keys()].filter((id) => pairText.includes(id));
console.log(`[bootstrap-capture] frames ${frames.map((entry) => `${entry.kind}:${entry.bytes}`).join(" ")}`);
console.log(`[bootstrap-capture] welcome ${JSON.stringify(frames.find((entry) => entry.kind === "Welcome")?.frame?.Welcome, replacer).slice(0, 600)}`);
console.log(`[bootstrap-capture] envelope ids ${envelopeIds.length}, distinct ${counts.size}, repeated ${[...counts].filter(([, count]) => count > 1).map(([id, count]) => `${id}×${count}`).slice(0, 12).join(" ")}; also inside the pair bytes: ${alsoInPair.length} ${alsoInPair.slice(0, 12).join(" ")}`);
