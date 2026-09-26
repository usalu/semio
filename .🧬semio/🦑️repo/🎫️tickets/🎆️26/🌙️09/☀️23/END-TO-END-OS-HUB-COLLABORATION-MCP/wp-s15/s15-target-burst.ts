#!/usr/bin/env bun
/** 🌩️ S15: requests the execution-target component and browser actor of hub documents CONCURRENTLY (the document open lane's
 * largest bodies, 5–20 MB each) as a signed-in member and prints every non-200 answer, short body and time, so a hub that reads
 * a whole asset under the route deadline (503 `DeadlineExceeded` once it is busy) shows it. Document ids come from this ticket's
 * journey logs for the given space.
 * usage: bun s15-target-burst.ts <hub origin> <spaceId> [rounds] [copies per document] */
import { randomBytes } from "node:crypto";
import { readdirSync, readFileSync } from "node:fs";

const [origin = "http://127.0.0.1:8040", spaceId = "01a0da67-8eba-7535-b03d-cc1ee778e9dc", rounds = "1", copiesArg = "2"] = process.argv.slice(2);
const generated = new URL("./generated/", import.meta.url);
const documents = [...new Set(readdirSync(generated).filter((name) => name.endsWith(".txt")).flatMap((name) => [...readFileSync(new URL(name, generated), "utf8").matchAll(new RegExp(`/spaces/${spaceId}/documents/(artifact-[0-9a-f]{32})/execution-target/component`, "gu"))].map((match) => match[1])))];
const signIn = await fetch(`${origin}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: process.env.S6_SIGN_IN_EMAIL ?? "user1@semio.dev", password: process.env.S6_SIGN_IN_PASSWORD ?? "gm1-local-dev-pass-1", deviceInstanceId: `s15burst${randomBytes(12).toString("hex")}`, clientClass: "browser" }) });
const token = String(((await signIn.json()) as { token?: string }).token ?? "");
console.log(`sign-in ${signIn.status}, ${documents.length} documents`);
const post = async (documentId: string, asset: string) => {
  const t0 = performance.now();
  const intent = { schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, clientInstanceId: `s15burst${randomBytes(8).toString("hex")}` };
  try {
    const response = await fetch(`${origin}/spaces/${spaceId}/documents/${documentId}/execution-target/${asset}`, { method: "POST", headers: { authorization: `Bearer ${token}`, "content-type": "application/json" }, body: JSON.stringify(intent) });
    const body = new Uint8Array(await response.arrayBuffer());
    const declared = Number(response.headers.get("content-length") ?? -1);
    return { documentId, asset, status: response.status, bytes: body.byteLength, declared, ms: Math.round(performance.now() - t0), text: response.status === 200 ? "" : new TextDecoder().decode(body).slice(0, 120) };
  } catch (error) {
    return { documentId, asset, status: -1, bytes: 0, declared: -1, ms: Math.round(performance.now() - t0), text: String(error).slice(0, 120) };
  }
};
for (let round = 1; round <= Number(rounds); round += 1) {
  const started = performance.now();
  const rows = await Promise.all(documents.flatMap((documentId) => Array.from({ length: Number(copiesArg) }, () => ["component", "browser-actor"].map((asset) => post(documentId, asset))).flat()));
  const bad = rows.filter((row) => row.status !== 200 || row.bytes !== row.declared);
  console.log(`round ${round}: ${rows.length} requests, ${rows.reduce((sum, row) => sum + row.bytes, 0)} bytes, ${bad.length} bad, slowest ${Math.max(...rows.map((row) => row.ms))} ms, ${Math.round(performance.now() - started)} ms`);
  const statuses = rows.reduce<Record<string, number>>((counts, row) => ({ ...counts, [row.status]: (counts[row.status] ?? 0) + 1 }), {});
  console.log(`  statuses ${JSON.stringify(statuses)}`);
  for (const row of bad.slice(0, 12)) console.log(`  BAD ${row.status} ${row.bytes}/${row.declared} ${row.ms}ms ${row.asset} ${row.documentId} ${row.text}`);
}
