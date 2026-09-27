/** 🔬️ C13: what the hub plans for a Spectator's open, with and without the editor surface the React shell requests.
 * usage: bun probe-c13-viewer-plan.ts <hubOrigin> <spaceId> <documentId> <surfaceIdPrefix>  (credentials: SEMIO_TWO_HUMAN_USER2_EMAIL/PASSWORD) */
import { hubProbeCall, hubProbeSignIn } from "/Users/ueli/Documents/semio/🌎️hub/🤝️integration-harness/🟦️.ts";
const [origin, spaceId, documentId, prefix] = process.argv.slice(2) as [string, string, string, string];
const token = await hubProbeSignIn(origin, process.env.SEMIO_TWO_HUMAN_USER2_EMAIL!, process.env.SEMIO_TWO_HUMAN_USER2_PASSWORD!, "c13plan");
for (const requestedSurfaceId of [undefined, `${prefix}#editor`, `${prefix}#viewer`]) {
  const intent = { schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, clientInstanceId: crypto.randomUUID(), ...(requestedSurfaceId ? { requestedSurfaceId } : {}) };
  const answer = await hubProbeCall(origin, "POST", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, token, JSON.stringify(intent));
  console.log(`requested=${requestedSurfaceId ?? "<none>"} HTTP ${answer.status} surface=${answer.json?.surface?.surfaceId ?? "-"} role=${answer.json?.surface?.role ?? "-"} write=${answer.json?.grant?.write ?? "-"} code=${answer.json?.code ?? "-"}`);
}
