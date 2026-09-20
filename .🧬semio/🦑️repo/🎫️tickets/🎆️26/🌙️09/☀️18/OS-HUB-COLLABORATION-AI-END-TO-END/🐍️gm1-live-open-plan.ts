/** 🌍️ GM1 — outcome 3's open half, against the LIVE hub whose trusted catalog is PUBLISHED.
 *
 * Signs a provisioned human in over `/auth/sessions`, creates a space, creates a gis Map document
 * through the Hub's OWN server-owned artifact-creation transaction (a bare `announce-document`
 * registers a descriptor with no active checkpoint and the open-plan issuer refuses `not-found` —
 * see 📓️gm1-gis-cold-load-law-and-publish.md §4b), and prints the issued document-open plan.
 *
 * Usage: `bun 🐍️gm1-live-open-plan.ts <origin> <email> <password>` */
import { randomBytes } from "node:crypto";
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";

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
const origin = process.argv[2]!;
const email = process.argv[3]!;
const password = process.argv[4]!;
const schemaRoot = join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema");
const { parseDocumentOpenPlanV1 } = await import(join(schemaRoot, "🟦️.ts"));
const { sealSpaceArtifactCreateV1, parseSpaceArtifactCreationStatusJsonV1 } = await import(join(schemaRoot, "🌱️space-artifact-creation-v1", "🟦️.ts"));

const minted = await fetch(`${origin}/auth/sessions`, {
  method: "POST",
  headers: { "content-type": "application/json" },
  body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: "gm1-live-open-plan", clientClass: "browser" }),
});
const token: string = ((await minted.json()) as any)?.token ?? "";
console.log(`sign-in status=${minted.status} tokenShape=${/^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u.test(token)}`);
if (minted.status !== 200) throw new Error("sign-in failed");
const headers = { authorization: `Bearer ${token}`, "content-type": "application/json" };
const post = async (path: string, body: unknown) => {
  const response = await fetch(`${origin}${path}`, { method: "POST", headers, body: JSON.stringify(body), signal: AbortSignal.timeout(15_000) });
  return { status: response.status, text: await response.text() };
};

const readiness = await (await fetch(`${origin}/readyz`)).json();
console.log(`readyz status=${(readiness as any).status} artifactAuthority.ready=${(readiness as any).artifactAuthority?.ready} features.openPlan=${(readiness as any).features?.openPlan}`);

const created = await post("/directory/commands", { schema: "semio.directory.command-request.v1", requestId: randomBytes(16).toString("hex"), command: { kind: "create-space", name: `GM1 Shared Map ${randomBytes(4).toString("hex")}`, spaceKind: "studio", visibility: "private" } });
const spaceId = JSON.parse(created.text).events.find((candidate: any) => candidate?.body?.kind === "space.created").body.spaceId;
console.log(`create-space status=${created.status} spaceId=${spaceId}`);

const catalogResponse = await fetch(`${origin}/spaces/${encodeURIComponent(spaceId)}/artifact-creations`, { headers: { authorization: headers.authorization } });
const catalog = JSON.parse(await catalogResponse.text());
const generationId: string = catalog.catalogGenerationId;
const kind = catalog.kinds.find((row: any) => row.kindId === "s.gis.gismap") ?? catalog.kinds[0];
console.log(`creation-catalog status=${catalogResponse.status} generation=${generationId} kinds=${catalog.kinds.map((row: any) => row.kindId).join(",")}`);

const route = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
const request = sealSpaceArtifactCreateV1({ requestId: randomBytes(16).toString("hex"), expectedCatalogGenerationId: generationId, kindId: kind.kindId, name: "GM1 Shared GIS Map" });
const accepted = await post(route, request);
let creation = parseSpaceArtifactCreationStatusJsonV1(accepted.text);
const deadline = Date.now() + 120_000;
while (creation.phase !== "ready") {
  if (!["accepted", "preparing", "indeterminate"].includes(creation.phase) || Date.now() >= deadline) throw new Error(`artifact creation reached ${creation.phase}`);
  await Bun.sleep(50);
  const polled = await fetch(`${origin}${route}/${encodeURIComponent(request.requestId)}`, { headers: { authorization: headers.authorization }, signal: AbortSignal.timeout(15_000) });
  creation = parseSpaceArtifactCreationStatusJsonV1(await polled.text());
}
const documentId = creation.ready.artifactId;
console.log(`artifact-creation ready documentId=${documentId} kind=${creation.ready.kindId} schema=${creation.ready.artifactSchema} surface=${creation.ready.surfaceId ?? "<derived>"}`);

const planResponse = await post(`/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, {
  schema: "semio.hub.document-open-intent/v1",
  version: 1,
  scope: { spaceId, documentId },
  clientInstanceId: "gm1-live-open-plan",
});
console.log(`open-plan status=${planResponse.status}`);
if (planResponse.status !== 200) throw new Error(`open-plan refused: ${planResponse.text}`);
const plan = parseDocumentOpenPlanV1(JSON.parse(planResponse.text), Date.now());
console.log(`open-plan surface=${JSON.stringify(plan.surface)}`);
console.log(`open-plan grant=${JSON.stringify(plan.grant)} catalog=${plan.catalog.generationId}`);
console.log(`open-plan package=${JSON.stringify(plan.package)}`);
console.log(`open-plan browserActor.importInterfaces=${(plan as any).browserActor?.importInterfaces?.length ?? 0}`);
console.log(`open-plan checkpoint=${JSON.stringify(plan.checkpoint)}`);
console.log(`GM1-LIVE-OPEN-PLAN ok space=${spaceId} document=${documentId}`);
