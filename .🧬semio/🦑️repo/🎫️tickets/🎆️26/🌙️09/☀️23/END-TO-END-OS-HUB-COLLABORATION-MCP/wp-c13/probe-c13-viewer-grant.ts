/** 🔬️ C13: a Spectator's full viewer open over HTTP + socket: plan (viewer requested) → socket-grant exchange → hello → Welcome.
 * usage: bun probe-c13-viewer-grant.ts <hubOrigin> <spaceId> <documentId> <surfaceId>  (credentials: SEMIO_TWO_HUMAN_USER2_EMAIL/PASSWORD) */
import { hubProbeCall, hubProbeSignIn, hubProbeOpenDocument } from "/Users/ueli/Documents/semio/🌎️hub/🤝️integration-harness/🟦️.ts";
const [origin, spaceId, documentId, surfaceId] = process.argv.slice(2) as [string, string, string, string];
const token = await hubProbeSignIn(origin, process.env.SEMIO_TWO_HUMAN_USER2_EMAIL!, process.env.SEMIO_TWO_HUMAN_USER2_PASSWORD!, "c13grant");
const scope = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`;
const intent = { schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, clientInstanceId: crypto.randomUUID(), requestedSurfaceId: surfaceId };
const plan = await hubProbeCall(origin, "POST", `${scope}/open-plan`, token, JSON.stringify(intent));
console.log(`plan HTTP ${plan.status} surface=${plan.json?.surface?.surfaceId} role=${plan.json?.surface?.role} write=${plan.json?.grant?.write} renderer=${plan.json?.surface?.rendererTarget} browserActor=${JSON.stringify(plan.json?.browserActor)?.slice(0, 160)}`);
const grant = await hubProbeCall(origin, "POST", `${scope}/socket-grants`, token, JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.json?.receipt }));
console.log(`socket-grant HTTP ${grant.status} ${grant.text.slice(0, 200)}`);
const opened = await hubProbeOpenDocument(origin, token, spaceId, documentId, `c13grant-${Date.now()}`).then((doc) => { doc.close(); return `welcome ${JSON.stringify(doc.welcome, (_k, v) => (typeof v === "bigint" ? v.toString() : v)).slice(0, 200)}`; }, (error) => `open failed: ${String(error).slice(0, 300)}`);
console.log(opened);
for (const part of ["manifest", "component", "descriptor", "browser-actor"]) {
  const asset = await hubProbeCall(origin, "GET", `${scope}/execution-target/${part}`, token);
  console.log(`asset ${part} HTTP ${asset.status} bytes=${asset.bytes.byteLength}`);
}
