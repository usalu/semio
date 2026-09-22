/** 🩺️ C8 — the artifact-creation transaction, with every raw body printed.
 *
 * `🐍️gm1-live-open-plan.ts` throws `artifact creation reached failed` and discards the status body,
 * so a creation that fails on a freshly published catalog says nothing about why. This repeats the
 * same three product calls (sign-in → create-space → `POST …/artifact-creations`, then polls the
 * per-request status route) and prints each response verbatim.
 *
 * Usage: `bun 🐍️c8-create-diagnose.ts <origin> <email> <password>` */
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
const { sealSpaceArtifactCreateV1 } = await import(join(schemaRoot, "🌱️space-artifact-creation-v1", "🟦️.ts"));

const minted = await fetch(`${origin}/auth/sessions`, {
  method: "POST",
  headers: { "content-type": "application/json" },
  body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: "c8-create-diagnose", clientClass: "browser" }),
});
const token: string = ((await minted.json()) as any)?.token ?? "";
console.log(`SIGN-IN ${minted.status}`);
const headers = { authorization: `Bearer ${token}`, "content-type": "application/json" };

const createdSpace = await fetch(`${origin}/directory/commands`, {
  method: "POST",
  headers,
  body: JSON.stringify({ schema: "semio.directory.command-request.v1", requestId: randomBytes(16).toString("hex"), command: { kind: "create-space", name: `C8 Shared Map ${randomBytes(4).toString("hex")}`, spaceKind: "studio", visibility: "private" } }),
});
const spaceText = await createdSpace.text();
const spaceId = JSON.parse(spaceText).events.find((candidate: any) => candidate?.body?.kind === "space.created").body.spaceId;
console.log(`SPACE ${createdSpace.status} ${spaceId}`);

const route = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
const catalogResponse = await fetch(`${origin}${route}`, { headers: { authorization: headers.authorization } });
const catalog = JSON.parse(await catalogResponse.text());
console.log(`CATALOG ${catalogResponse.status} ${JSON.stringify(catalog).slice(0, 1200)}`);

const kind = catalog.kinds.find((row: any) => row.kindId === "s.gis.gismap") ?? catalog.kinds[0];
const request = sealSpaceArtifactCreateV1({ requestId: randomBytes(16).toString("hex"), expectedCatalogGenerationId: catalog.catalogGenerationId, kindId: kind.kindId, name: "C8 Shared GIS Map" });
const accepted = await fetch(`${origin}${route}`, { method: "POST", headers, body: JSON.stringify(request) });
console.log(`CREATE ${accepted.status} ${(await accepted.text()).slice(0, 2000)}`);

const deadline = Date.now() + 120_000;
let last = "";
while (Date.now() < deadline) {
  const polled = await fetch(`${origin}${route}/${encodeURIComponent(request.requestId)}`, { headers: { authorization: headers.authorization } });
  const text = await polled.text();
  if (text !== last) {
    console.log(`POLL ${polled.status} ${text.slice(0, 2000)}`);
    last = text;
  }
  const phase = (() => {
    try {
      return JSON.parse(text).phase;
    } catch {
      return "unparsed";
    }
  })();
  if (phase === "ready" || phase === "failed" || phase === "unparsed") break;
  await Bun.sleep(100);
}
console.log(`SPACE-ID ${spaceId}`);
