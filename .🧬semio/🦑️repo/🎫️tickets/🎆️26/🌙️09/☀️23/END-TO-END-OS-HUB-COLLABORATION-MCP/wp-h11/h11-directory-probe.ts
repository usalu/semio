#!/usr/bin/env bun
/** 🏘️ H11 item 2: directory latency on a live hub for a member of many spaces. Signs user1 in, creates spaces until
 * user1 owns ≥ <target> (named `H11 dir <n>`), then measures `GET /directory/spaces` (best/median of 5) and every
 * `GET /directory/event-page/v1` page from 0 (ms each), and checks the list against the event pages (same ids).
 * usage: bun h11-directory-probe.ts <origin> <targetSpaces> */
import { randomBytes } from "node:crypto";

const origin = process.argv[2]!;
const target = Number(process.argv[3] ?? "85");
const call = async (method: string, path: string, token?: string, body?: unknown) => {
  const started = performance.now();
  const response = await fetch(`${origin}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body: JSON.stringify(body) }), signal: AbortSignal.timeout(180_000) });
  const text = await response.text();
  const ms = performance.now() - started;
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {}
  return { status: response.status, text, json, ms, bytes: text.length };
};
const signIn = await call("POST", "/auth/sessions", undefined, { schema: "semio.hub.auth.credential-sign-in/v1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1", deviceInstanceId: `h11dir${randomBytes(12).toString("hex")}`, clientClass: "browser" });
if (signIn.status !== 200) throw new Error(`sign-in ${signIn.status} ${signIn.text.slice(0, 200)}`);
const token = String(signIn.json.token);
const listed = async () => {
  const answer = await call("GET", "/directory/spaces", token);
  if (answer.status !== 200) throw new Error(`list ${answer.status} ${answer.text.slice(0, 200)}`);
  return answer;
};
let current = (await listed()).json as any[];
let owned = current.filter((entry) => entry.access === "author").length;
console.log(`start: listed=${current.length} authored=${owned}`);
const createMs: number[] = [];
while (owned < target) {
  const created = await call("POST", "/directory/commands", token, { schema: "semio.directory.command-request.v1", requestId: randomBytes(16).toString("hex"), command: { kind: "create-space", name: `H11 dir ${owned}`, spaceKind: owned % 2 ? "atelier" : "studio", visibility: owned % 4 ? "private" : "public" } });
  if (created.status !== 202) throw new Error(`create ${created.status} ${created.text.slice(0, 200)}`);
  createMs.push(created.ms);
  owned += 1;
}
if (createMs.length) console.log(`created ${createMs.length} spaces: p50 ${createMs.sort((a, b) => a - b)[Math.floor(createMs.length / 2)].toFixed(1)} ms, max ${Math.max(...createMs).toFixed(1)} ms`);
const lists: number[] = [];
for (let round = 0; round < 5; round += 1) {
  const answer = await listed();
  lists.push(answer.ms);
  current = answer.json;
}
const sorted = [...lists].sort((a, b) => a - b);
console.log(`GET /directory/spaces: ${current.length} entries, ${(await listed()).bytes} B; best ${sorted[0].toFixed(1)} ms, median ${sorted[2].toFixed(1)} ms, worst ${sorted[4].toFixed(1)} ms`);
let after = 0;
const pageMs: number[] = [];
const spaceIds = new Set<string>();
let events = 0;
for (;;) {
  const page = await call("GET", `/directory/event-page/v1?after=${after}`, token);
  if (page.status !== 200) throw new Error(`event page ${page.status} ${page.text.slice(0, 200)}`);
  pageMs.push(page.ms);
  for (const event of page.json.events ?? []) {
    events += 1;
    const spaceId = event?.body?.spaceId ?? event?.spaceId;
    if (typeof spaceId === "string") spaceIds.add(spaceId);
  }
  const through = Number(page.json.throughSeqInclusive);
  if (!page.json.hasMore || through === after) break;
  after = through;
}
console.log(`GET /directory/event-page/v1: ${pageMs.length} pages, ${events} events; per page ms ${pageMs.map((ms) => ms.toFixed(1)).join(", ")}; max ${Math.max(...pageMs).toFixed(1)} ms`);
const missing = current.map((entry) => entry.space.id).filter((id: string) => !spaceIds.has(id) && current.find((e) => e.space.id === id)?.access !== "public");
console.log(`list vs event pages: ${current.length} listed, ${spaceIds.size} spaces in events, member spaces missing from events: ${missing.length}`);
console.log(`VERDICT list-best<200ms=${sorted[0] < 200} pages-max<400ms=${Math.max(...pageMs) < 400}`);
