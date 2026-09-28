#!/usr/bin/env bun
/** ⏱️ F3 — hub creation phases per kind, from W3's open-plan probe: per kind (`rounds` times, the first round is the
 * package's first creation since the probe started): POST accept → polled status every 100 ms (each phase's first-seen ms)
 * → Ready → `POST …/open-plan` ms. Writes one JSON line per creation. usage: bun f3-hub-creation-phases.ts <origin> <rounds> <kindId|all> [kindId…] */
import { randomBytes } from "node:crypto";

const origin = process.argv[2]!;
const rounds = Number(process.argv[3] ?? 2);
const wanted = process.argv.slice(4);
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

const signIn = await call("POST", "/auth/sessions", undefined, { schema: "semio.hub.auth.credential-sign-in/v1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1", deviceInstanceId: `w3openplan${randomBytes(11).toString("hex")}`, clientClass: "browser" });
const token = String(signIn.json?.token ?? "");
if (signIn.status !== 200 || !token) throw new Error(`sign-in failed ${signIn.status}`);
const created = await call("POST", "/directory/commands", token, { schema: "semio.directory.command-request.v1", requestId: randomBytes(16).toString("hex"), command: { kind: "create-space", name: `W3 open-plan probe ${new Date().toISOString()}`, spaceKind: "studio", visibility: "private" } });
const spaceId = created.json?.events?.find((event: any) => event?.body?.kind === "space.created")?.body?.spaceId;
if (created.status !== 202 || typeof spaceId !== "string") throw new Error(`create-space failed ${created.status} ${created.text.slice(0, 300)}`);
const catalog = await call("GET", `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`, token);
const kinds = (catalog.json?.kinds ?? []) as { kindId: string; schema: string; dialect: { artifactKind: string } }[];
console.log(`space=${spaceId} generation=${catalog.json?.catalogGenerationId} creatableKinds=${kinds.length} ${kinds.map((kind) => kind.kindId).join(",")}`);
const selected = wanted[0] === "all" ? kinds : kinds.filter((kind) => wanted.includes(kind.kindId));
for (let round = 1; round <= rounds; round += 1) {
  for (const kind of selected) {
    const started = performance.now();
    const requestId = randomBytes(16).toString("hex");
    const route = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
    const accepted = await call("POST", route, token, sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: catalog.json.catalogGenerationId, kindId: kind.kindId, name: `F3 ${kind.kindId} ${round}` }));
    const phases: Record<string, number> = { [`accept:${accepted.json?.phase ?? accepted.status}`]: Math.round(performance.now() - started) };
    let status = accepted.json ?? {};
    while (accepted.status < 300 && ["accepted", "preparing", "indeterminate"].includes(status.phase) && performance.now() - started < 600_000) {
      await Bun.sleep(100);
      status = (await call("GET", `${route}/${requestId}`, token)).json ?? {};
      phases[`${status.phase}${status.progress ? `:${JSON.stringify(status.progress).slice(0, 80)}` : ""}`] ??= Math.round(performance.now() - started);
    }
    const documentId = status.ready?.artifactId;
    const readyMs = Math.round(performance.now() - started);
    let planMs = -1;
    let planStatus = 0;
    if (status.phase === "ready" && typeof documentId === "string") {
      const planStarted = performance.now();
      const plan = await call("POST", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, token, { schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, clientInstanceId: "f3-creation-phases" });
      planMs = Math.round(performance.now() - planStarted);
      planStatus = plan.status;
    }
    console.log(JSON.stringify({ round, kindId: kind.kindId, phase: status.phase, readyMs, planStatus, planMs, phases, at: new Date().toISOString() }));
  }
}
