#!/usr/bin/env bun
/** ⏱️ S16 item 2: an HTTP-only reproducer of the slow hub creation lane (no browser, no shell) — signs in as a dev user,
 * creates a fresh space, posts one sealed artifact creation per kind at once, and polls every creation's status every 3 s
 * until each leaves `accepted`/`preparing` or the bound passes; prints each phase change with its elapsed seconds and a
 * final table. usage: bun s16-creation-timing.ts <hubOrigin> <boundSeconds> <kindId>… */
import { randomBytes } from "node:crypto";
import { sealSpaceArtifactCreateV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";

const [origin = "http://127.0.0.1:8041", bound = "900", ...kinds] = process.argv.slice(2);
const call = async (method: string, path: string, token: string | null, body?: unknown) => {
  const response = await fetch(`${origin}${path}`, { method, headers: { ...(token ? { authorization: `Bearer ${token}` } : {}), "content-type": "application/json" }, body: body === undefined ? undefined : JSON.stringify(body), signal: AbortSignal.timeout(60_000) });
  return { status: response.status, text: await response.text() };
};
const t0 = Date.now();
const at = () => ((Date.now() - t0) / 1000).toFixed(1);
const signIn = await call("POST", "/auth/sessions", null, { schema: "semio.hub.auth.credential-sign-in/v1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1", deviceInstanceId: randomBytes(16).toString("hex"), clientClass: "browser" });
const token = JSON.parse(signIn.text).token as string;
const space = await call("POST", "/directory/commands", token, { schema: "semio.directory.command-request.v1", requestId: randomBytes(16).toString("hex"), command: { kind: "create-space", name: `S16 Creation Timing ${Date.now() % 100000}`, spaceKind: "studio", visibility: "private" } });
const spaceId = JSON.parse(space.text).events.find((row: { body?: { kind?: string } }) => row?.body?.kind === "space.created").body.spaceId as string;
const catalog = await call("GET", `/spaces/${spaceId}/artifact-creations`, token);
const generation = JSON.parse(catalog.text).catalogGenerationId as string;
console.log(`[${at()} s] space ${spaceId} catalog ${generation}`);
const rows = [];
for (const kindId of kinds) {
  const request = sealSpaceArtifactCreateV1({ requestId: randomBytes(16).toString("hex"), expectedCatalogGenerationId: generation, kindId, name: `S16 ${kindId}` });
  const posted = await call("POST", `/spaces/${spaceId}/artifact-creations`, token, request);
  console.log(`[${at()} s] POST ${kindId} → ${posted.status} ${posted.text.slice(0, 160)}`);
  rows.push({ kindId, requestId: request.requestId, phase: posted.status === 202 ? "accepted" : `post-${posted.status}`, since: Date.now(), phases: [] as string[] });
}
while (rows.some((row) => /^(accepted|preparing)$/u.test(row.phase)) && Date.now() - t0 < Number(bound) * 1000) {
  for (const row of rows.filter((entry) => /^(accepted|preparing)$/u.test(entry.phase))) {
    const status = await call("GET", `/spaces/${spaceId}/artifact-creations/${row.requestId}`, token).catch((error) => ({ status: 0, text: String(error) }));
    const phase = /"phase":"([a-z-]+)"/u.exec(status.text)?.[1] ?? `http-${status.status}`;
    if (phase !== row.phase || row.phases.length === 0) {
      row.phases.push(`${phase}@${at()}s`);
      console.log(`[${at()} s] ${row.kindId} ${row.phase} → ${phase} ${status.text.slice(0, 200)}`);
      row.phase = phase;
    }
  }
  await Bun.sleep(3_000);
}
console.log("\nkind | final phase | phases");
for (const row of rows) console.log(`${row.kindId} | ${row.phase} | ${row.phases.join(" → ")}`);
