/** 🔬️ C13: parses the open plan the hub issues a Spectator (viewer requested) and an Author (editor requested) with the worker's
 * own parser (`parseDocumentOpenPlanV1`) — the worker drops the whole hub session when this parse throws.
 * usage: bun probe-c13-viewer-parse.ts <hubOrigin> <spaceId> <documentId> <viewerSurfaceId> <editorSurfaceId>
 * (credentials: SEMIO_TWO_HUMAN_USER{1,2}_EMAIL/PASSWORD) */
import { hubProbeCall, hubProbeSignIn } from "/Users/ueli/Documents/semio/🌎️hub/🤝️integration-harness/🟦️.ts";
import { parseDocumentOpenPlanV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
const [origin, spaceId, documentId, viewerSurface, editorSurface] = process.argv.slice(2) as [string, string, string, string, string];
const scope = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`;
for (const [user, surface] of [[2, viewerSurface], [1, editorSurface]] as const) {
  const token = await hubProbeSignIn(origin, process.env[`SEMIO_TWO_HUMAN_USER${user}_EMAIL`]!, process.env[`SEMIO_TWO_HUMAN_USER${user}_PASSWORD`]!, "c13parse");
  const intent = { schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, clientInstanceId: crypto.randomUUID(), requestedSurfaceId: surface };
  const plan = await hubProbeCall(origin, "POST", `${scope}/open-plan`, token, JSON.stringify(intent));
  let verdict: string;
  try {
    const parsed = parseDocumentOpenPlanV1(plan.json, Date.now());
    verdict = `parsed surface=${parsed.surface.surfaceId} role=${parsed.surface.role} write=${parsed.grant.write}`;
  } catch (error) {
    verdict = `PARSE FAILED: ${String(error instanceof Error ? error.message : error).slice(0, 600)}`;
  }
  console.log(`user${user} requested ${surface}: HTTP ${plan.status} ${verdict}`);
  if (process.argv.includes("--dump")) console.log(JSON.stringify(plan.json, null, 1).slice(0, 6000));
}
