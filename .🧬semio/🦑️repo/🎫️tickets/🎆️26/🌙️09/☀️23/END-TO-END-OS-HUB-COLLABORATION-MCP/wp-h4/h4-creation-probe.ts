#!/usr/bin/env bun
/** 🌱️ H4 probe: does a server-owned artifact creation reach Ready on a hub? Usage: bun h4-creation-probe.ts <origin> <kindSubstring> */
import { randomBytes } from "node:crypto";
const origin = process.argv[2] ?? "http://127.0.0.1:7850";
const want = process.argv[3] ?? "note";
const post = (path: string, token: string | undefined, body: unknown) => fetch(`${origin}${path}`, { method: "POST", headers: { "content-type": "application/json", ...(token ? { authorization: `Bearer ${token}` } : {}) }, body: JSON.stringify(body) });
const token = (await (await post("/auth/sessions", undefined, { schema: "semio.hub.auth.credential-sign-in/v1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1", deviceInstanceId: `h4creation${randomBytes(8).toString("hex")}`, clientClass: "browser" })).json()).token;
const spaces = await (await fetch(`${origin}/directory/spaces`, { headers: { authorization: `Bearer ${token}` } })).json();
const spaceId = spaces.map((entry: any) => entry.space.id).at(-1);
const catalog = await (await fetch(`${origin}/spaces/${spaceId}/artifact-creations`, { headers: { authorization: `Bearer ${token}` } })).json();
console.log("catalog", JSON.stringify(catalog).slice(0, 600));
const kinds = (catalog.kinds ?? catalog.entries ?? catalog.rows ?? []) as any[];
const kind = kinds.find((row) => JSON.stringify(row).includes(want));
const requestId = randomBytes(16).toString("hex");
const started = Date.now();
const { sealSpaceArtifactCreateV1 } = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts");
const accepted = await post(`/spaces/${spaceId}/artifact-creations`, token, sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: catalog.catalogGenerationId, kindId: kind.kindId, name: "H4 creation probe" }));
console.log("accepted", accepted.status, (await accepted.text()).slice(0, 400));
for (let poll = 0; poll < 120; poll++) {
  const status = (await (await fetch(`${origin}/spaces/${spaceId}/artifact-creations/${requestId}`, { headers: { authorization: `Bearer ${token}` } })).json().catch(() => null)) ?? {};
  if (status.phase !== "accepted" && status.phase !== "preparing") { console.log(`phase=${status.phase} after ${Date.now() - started} ms`, JSON.stringify(status).slice(0, 400)); process.exit(0); }
  await Bun.sleep(1000);
}
console.log(`still not ready after ${Date.now() - started} ms`);
