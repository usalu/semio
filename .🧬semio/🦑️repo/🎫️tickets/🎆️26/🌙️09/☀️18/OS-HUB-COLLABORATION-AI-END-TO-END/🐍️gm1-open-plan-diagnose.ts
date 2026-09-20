/** 🩺️ GM1 — why `proveTrustedStdioGisCandidatePlan` throws `document-open.invalid-fields`.
 *
 * That prover calls `parseDocumentOpenPlanV1(await response.json(), …)` BEFORE it looks at
 * `response.ok`, so EVERY hub refusal of `POST …/open-plan` — whose body is a two-field
 * `semio.hub.document-open-plan-error/v1 { schema, code }` — surfaces as the parser's generic
 * `document-open.invalid-fields` and the actual `code` is never printed.
 *
 * This boots a hub against the candidate data root the failed bootstrap left on disk (its trusted
 * catalog is already staged, so `artifactAuthority.ready` is the run's own), replays the prover's
 * exact three calls (create-space → announce-document → open-plan) and prints the raw status and
 * body of each.
 *
 * Usage: `bun 🐍️gm1-open-plan-diagnose.ts <absoluteCandidateDataRoot> <absoluteBinaryPath> [port]` */
import { randomBytes } from "node:crypto";
import { existsSync, readFileSync, readdirSync } from "node:fs";
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
const hubRustRoot = join(repoRoot, "🌎️hub", "📦️packages", "🦀️rust");
const candidateData = process.argv[2]!;
const binaryPath = process.argv[3]!;
const port = Number(process.argv[4] ?? 7623);
if (!existsSync(candidateData) || !existsSync(binaryPath)) throw new Error("candidate data root and hub binary must both exist");

const { startLocalHub, waitForReadiness, finishLocalHub } = await import(join(repoRoot, "🌎️hub", "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"));
const { issueLocalCredential } = await import(join(repoRoot, "🌎️hub", "🚀️local-bootstrap", "🔐️credential-issuance", "🟦️.ts"));

const generationsRoot = join(candidateData, "trusted-catalog", "generations");
const generationId = readdirSync(generationsRoot)[0]!;
const bundle = JSON.parse(readFileSync(join(generationsRoot, generationId, "trusted-catalog.json"), "utf8"));
const profileRow = bundle.profiles[0];
const target = profileRow.openTarget.target;
const selected = bundle.packages.find((candidate: any) => candidate.pluginId === "gis");
console.log(`generation=${generationId} profile=${profileRow.id} surface=${target.surfaceId}`);

const profile = { profileId: "trusted-bootstrap-probe", subject: "trusted-bootstrap-subject", displayName: "Trusted Bootstrap Probe", allowedClientClasses: ["native" as const] };
const run = await startLocalHub(repoRoot, hubRustRoot, [profile], { capture: true, dataDir: candidateData, binaryPath, port });
try {
  const readiness = await waitForReadiness(run);
  console.log(`readyz artifactAuthority.ready=${readiness.artifactAuthority?.ready} openPlan=${readiness.features?.openPlan} openPlanExchange=${readiness.features?.openPlanExchange}`);
  console.log(`readyz body=${JSON.stringify(readiness).slice(0, 1200)}`);
  const envelope = await issueLocalCredential(run, profile.profileId, "native");
  const headers = { authorization: `Bearer ${envelope.capability}`, "content-type": "application/json" };
  const post = async (path: string, body: unknown) => {
    const response = await fetch(`http://127.0.0.1:${run.port}${path}`, { method: "POST", headers, body: JSON.stringify(body), signal: AbortSignal.timeout(10_000) });
    return { status: response.status, text: await response.text() };
  };
  const created = await post("/directory/commands", {
    schema: "semio.directory.command-request.v1",
    requestId: randomBytes(16).toString("hex"),
    command: { kind: "create-space", name: "Trusted GIS Bootstrap Probe", spaceKind: "studio", visibility: "private" },
  });
  console.log(`create-space status=${created.status}`);
  const spaceId = JSON.parse(created.text).events.find((candidate: any) => candidate?.body?.kind === "space.created").body.spaceId;
  const documentId = `trusted-gis-map-${randomBytes(8).toString("hex")}`;
  const descriptor = {
    spaceId,
    documentId,
    artifactKind: target.artifactKind,
    artifactSchema: target.artifactSchema,
    owner: { pluginId: selected.pluginId, packageId: selected.packageId, version: selected.version, packageHash: selected.component.sha256 },
    packSchemaHash: target.packSchemaHash,
    bootstrapVersion: 1,
    bootstrapFrontier: { headSeq: 0, commitSeq: 0, epoch: 0 },
    bootstrapSnapshotHash: "11".repeat(32),
  };
  const announced = await post("/directory/commands", { schema: "semio.directory.command-request.v1", requestId: randomBytes(16).toString("hex"), command: { kind: "announce-document", descriptor } });
  console.log(`announce-document status=${announced.status} text=${announced.text.slice(0, 600)}`);
  const plan = await post(`/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, {
    schema: "semio.hub.document-open-intent/v1",
    version: 1,
    scope: { spaceId, documentId },
    requestedSurfaceId: target.surfaceId,
    clientInstanceId: "trusted-bootstrap-candidate",
  });
  console.log(`open-plan status=${plan.status}`);
  console.log(`open-plan body=${plan.text.slice(0, 4000)}`);

  // 🌱️ Hypothesis: `announce-document` registers a descriptor but no ACTIVE ARTIFACT CHECKPOINT, and
  // the open-plan issuer refuses `not-found` without one (`🏗️bootstrap/🦀️.rs:3099-3104`). The hub's
  // own server-owned creation transaction is what establishes it.
  const { sealSpaceArtifactCreateV1, parseSpaceArtifactCreationStatusJsonV1 } = await import(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema", "🌱️space-artifact-creation-v1", "🟦️.ts"));
  const request = sealSpaceArtifactCreateV1({ requestId: randomBytes(16).toString("hex"), expectedCatalogGenerationId: generationId, kindId: target.artifactKind, name: "Trusted GIS Bootstrap Probe Map" });
  const route = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const accepted = await post(route, request);
  console.log(`artifact-creation status=${accepted.status} text=${accepted.text.slice(0, 600)}`);
  let creation = parseSpaceArtifactCreationStatusJsonV1(accepted.text);
  const deadline = Date.now() + 120_000;
  while (creation.phase !== "ready") {
    if (!["accepted", "preparing", "indeterminate"].includes(creation.phase) || Date.now() >= deadline) throw new Error(`artifact creation reached ${creation.phase}`);
    await Bun.sleep(50);
    const poll = await fetch(`http://127.0.0.1:${run.port}${route}/${encodeURIComponent(request.requestId)}`, { headers, signal: AbortSignal.timeout(10_000) });
    creation = parseSpaceArtifactCreationStatusJsonV1(await poll.text());
  }
  console.log(`artifact-creation ready artifactId=${creation.ready.artifactId} kind=${creation.ready.kindId} schema=${creation.ready.artifactSchema}`);
  const createdPlan = await post(`/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(creation.ready.artifactId)}/open-plan`, {
    schema: "semio.hub.document-open-intent/v1",
    version: 1,
    scope: { spaceId, documentId: creation.ready.artifactId },
    requestedSurfaceId: target.surfaceId,
    clientInstanceId: "trusted-bootstrap-candidate",
  });
  console.log(`created open-plan status=${createdPlan.status}`);
  console.log(`created open-plan body=${createdPlan.text.slice(0, 4000)}`);
  envelope.capability = "";
} finally {
  await finishLocalHub(run);
}
