#!/usr/bin/env bun
/** 🔎️ S15: reproduces the trusted-catalog candidate's GIS Map creation over HTTP against a running hub and prints every
 * status and body (the bootstrap's own proof parses the body before it reads the status, so a refusal reads as a JSON error).
 * usage: bun s15-creation-diag.ts <hubOrigin> <email> <password> [kindId] */
import { randomBytes } from "node:crypto";
import { sealSpaceArtifactCreateV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";
const [origin = "http://127.0.0.1:8040", email = "user1@semio.dev", password = "", kindId = "s.gis.gismap"] = process.argv.slice(2);
const call = async (method: string, path: string, token: string | null, body?: unknown) => {
  const response = await fetch(`${origin}${path}`, { method, headers: { ...(token ? { authorization: `Bearer ${token}` } : {}), "content-type": "application/json" }, body: body === undefined ? undefined : JSON.stringify(body), signal: AbortSignal.timeout(10_000) });
  const text = await response.text();
  console.log(`${method} ${path} → ${response.status} ${text.slice(0, 600)}`);
  return { status: response.status, text };
};
const signIn = await call("POST", "/auth/sessions", null, { schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: randomBytes(16).toString("hex"), clientClass: "browser" });
const token = JSON.parse(signIn.text).token as string;
const space = await call("POST", "/directory/commands", token, { schema: "semio.directory.command-request.v1", requestId: randomBytes(16).toString("hex"), command: { kind: "create-space", name: `S15 Diag ${Date.now() % 100000}`, spaceKind: "studio", visibility: "private" } });
const spaceId = JSON.parse(space.text).events.find((row: any) => row?.body?.kind === "space.created").body.spaceId as string;
const catalog = await call("GET", `/spaces/${spaceId}/artifact-creations`, token);
const generation = JSON.parse(catalog.text).catalogGenerationId as string;
const request = sealSpaceArtifactCreateV1({ requestId: randomBytes(16).toString("hex"), expectedCatalogGenerationId: generation, kindId, name: "S15 Diag Map" });
await call("POST", `/spaces/${spaceId}/artifact-creations`, token, request);
for (let poll = 0; poll < 40; poll += 1) {
  const status = await call("GET", `/spaces/${spaceId}/artifact-creations/${request.requestId}`, token);
  if (!/"phase":"(accepted|preparing|indeterminate)"/u.test(status.text)) break;
  await Bun.sleep(500);
}
