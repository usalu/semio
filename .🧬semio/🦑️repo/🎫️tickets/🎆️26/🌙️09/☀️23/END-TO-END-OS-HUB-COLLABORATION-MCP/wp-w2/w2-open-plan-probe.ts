#!/usr/bin/env bun
/** 🎯️ W2: proves a hub serves the catalog's creation catalog and issues a document-open plan for each named kind.
 * Signs user1 in, creates one private probe space, lists `/spaces/{s}/artifact-creations`, then per kind: server-owned
 * creation → Ready → `POST …/open-plan` (editor surface) → plan package/artifact/surface printed. One row per kind.
 * usage: bun w2-open-plan-probe.ts <origin> <kindId|all> [kindId…] */
import { randomBytes } from "node:crypto";

const origin = process.argv[2]!;
const wanted = process.argv.slice(3);
const { sealSpaceArtifactCreateV1 } = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts");
const call = async (method: string, path: string, token: string | undefined, body?: unknown) => {
  const response = await fetch(`${origin}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body: JSON.stringify(body) }), signal: AbortSignal.timeout(120_000) });
  const text = await response.text();
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {}
  return { status: response.status, text, json };
};

const signIn = await call("POST", "/auth/sessions", undefined, { schema: "semio.hub.auth.credential-sign-in/v1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1", deviceInstanceId: `w2openplan${randomBytes(11).toString("hex")}`, clientClass: "browser" });
const token = String(signIn.json?.token ?? "");
if (signIn.status !== 200 || !token) throw new Error(`sign-in failed ${signIn.status}`);
const created = await call("POST", "/directory/commands", token, { schema: "semio.directory.command-request.v1", requestId: randomBytes(16).toString("hex"), command: { kind: "create-space", name: `W2 open-plan probe ${new Date().toISOString()}`, spaceKind: "studio", visibility: "private" } });
const spaceId = created.json?.events?.find((event: any) => event?.body?.kind === "space.created")?.body?.spaceId;
if (created.status !== 202 || typeof spaceId !== "string") throw new Error(`create-space failed ${created.status} ${created.text.slice(0, 300)}`);
const catalog = await call("GET", `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`, token);
const kinds = (catalog.json?.kinds ?? []) as { kindId: string; schema: string; dialect: { artifactKind: string } }[];
console.log(`space=${spaceId} generation=${catalog.json?.catalogGenerationId} creatableKinds=${kinds.length} ${kinds.map((kind) => kind.kindId).join(",")}`);
const selected = wanted[0] === "all" ? kinds : kinds.filter((kind) => wanted.includes(kind.kindId));
let failed = wanted[0] === "all" ? 0 : wanted.filter((kindId) => !kinds.some((kind) => kind.kindId === kindId)).length;
for (const kind of selected) {
  const started = Date.now();
  const requestId = randomBytes(16).toString("hex");
  const route = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const accepted = await call("POST", route, token, sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: catalog.json.catalogGenerationId, kindId: kind.kindId, name: `W2 ${kind.kindId}` }));
  let status = accepted.json ?? {};
  while (accepted.status < 300 && ["accepted", "preparing", "indeterminate"].includes(status.phase) && Date.now() - started < 600_000) {
    await Bun.sleep(500);
    status = (await call("GET", `${route}/${requestId}`, token)).json ?? {};
  }
  const documentId = status.ready?.artifactId;
  if (status.phase !== "ready" || typeof documentId !== "string") {
    failed += 1;
    console.log(`FAIL ${kind.kindId} creation phase=${status.phase} http=${accepted.status} ${JSON.stringify(status).slice(0, 300)} ${Date.now() - started}ms`);
    continue;
  }
  const plan = await call("POST", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, token, { schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, clientInstanceId: "w2-open-plan-probe" });
  const ok = plan.status === 200 && plan.json?.artifact?.kind === kind.kindId && plan.json?.surface?.role === "editor";
  if (!ok) failed += 1;
  console.log(`${ok ? "PASS" : "FAIL"} ${kind.kindId} document=${documentId} plan=${plan.status} package=${plan.json?.package?.pluginId}@${String(plan.json?.package?.componentSha256 ?? "").slice(0, 12)} surface=${plan.json?.surface?.surfaceId} actor=${plan.json?.browserActor?.kind ?? "-"} ${Date.now() - started}ms${ok ? "" : ` ${plan.text.slice(0, 200)}`}`);
}
console.log(`DONE kinds=${selected.length} failed=${failed}`);
process.exit(failed === 0 ? 0 : 1);
