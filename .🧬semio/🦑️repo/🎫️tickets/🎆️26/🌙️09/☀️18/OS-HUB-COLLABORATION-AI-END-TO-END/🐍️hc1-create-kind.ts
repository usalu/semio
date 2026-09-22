/** 🌱️ HC1 — create ONE document of a NAMED kind on a hub and follow it to its terminal phase.
 *
 * `🐍️c8-create-diagnose.ts` hard-picks `s.gis.gismap` and gives up after 120 s, which was enough
 * while every creation died on a 30 s wall clock. A three-package catalog offers more than one kind,
 * and a guest genesis on the biggest staged component legitimately takes minutes, so this takes the
 * kind as an argument and polls until the hub reaches a terminal phase.
 *
 * Usage: `bun 🐍️hc1-create-kind.ts <origin> <email> <password> <kindId> [maxSeconds] [spaceId]` */
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
const [origin, email, password, kindId] = process.argv.slice(2) as string[];
const maxSeconds = Number(process.argv[6] ?? 1200);
const reuseSpace = process.argv[7];
const schemaRoot = join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema");
const { sealSpaceArtifactCreateV1 } = await import(join(schemaRoot, "🌱️space-artifact-creation-v1", "🟦️.ts"));

const minted = await fetch(`${origin}/auth/sessions`, {
  method: "POST",
  headers: { "content-type": "application/json" },
  body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: "hc1-create-kind", clientClass: "browser" }),
});
const token: string = ((await minted.json()) as { token?: string })?.token ?? "";
console.log(`SIGN-IN ${minted.status}`);
const headers = { authorization: `Bearer ${token}`, "content-type": "application/json" };

let spaceId = reuseSpace ?? "";
if (!spaceId) {
  const createdSpace = await fetch(`${origin}/directory/commands`, {
    method: "POST",
    headers,
    body: JSON.stringify({ schema: "semio.directory.command-request.v1", requestId: randomBytes(16).toString("hex"), command: { kind: "create-space", name: `HC1 ${kindId} ${randomBytes(4).toString("hex")}`, spaceKind: "studio", visibility: "private" } }),
  });
  const body = JSON.parse(await createdSpace.text()) as { events: { body?: { kind?: string; spaceId?: string } }[] };
  spaceId = body.events.find((candidate) => candidate?.body?.kind === "space.created")!.body!.spaceId!;
  console.log(`SPACE ${createdSpace.status} ${spaceId}`);
}

const route = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
const catalogResponse = await fetch(`${origin}${route}`, { headers: { authorization: headers.authorization } });
const catalog = JSON.parse(await catalogResponse.text()) as { catalogGenerationId: string; kinds: { kindId: string }[] };
console.log(`CATALOG ${catalogResponse.status} generation=${catalog.catalogGenerationId} kinds=${catalog.kinds.map((row) => row.kindId).join(",")}`);
const kind = catalog.kinds.find((row) => row.kindId === kindId);
if (!kind) throw new Error(`the catalog does not offer ${kindId}`);

const request = sealSpaceArtifactCreateV1({ requestId: randomBytes(16).toString("hex"), expectedCatalogGenerationId: catalog.catalogGenerationId, kindId: kind.kindId, name: `HC1 ${kindId}` });
const started = Date.now();
const accepted = await fetch(`${origin}${route}`, { method: "POST", headers, body: JSON.stringify(request) });
console.log(`CREATE ${accepted.status} request=${request.requestId}`);

let seen = "";
for (;;) {
  const polled = await fetch(`${origin}${route}/${encodeURIComponent(request.requestId)}`, { headers: { authorization: headers.authorization } });
  const body = (await polled.json()) as { phase?: string; ready?: { artifactId?: string } };
  const elapsed = Math.round((Date.now() - started) / 1000);
  if (body.phase !== seen) {
    seen = body.phase ?? "";
    console.log(`${elapsed}s ${polled.status} phase=${seen}`);
  }
  if (["ready", "failed", "cancelled", "indeterminate"].includes(seen)) {
    console.log(`TERMINAL ${seen} after ${elapsed}s space=${spaceId} ${JSON.stringify(body)}`);
    break;
  }
  if (elapsed > maxSeconds) {
    console.log(`GAVE UP in phase ${seen} after ${elapsed}s space=${spaceId}`);
    break;
  }
  await new Promise((resolve) => setTimeout(resolve, 2000));
}
