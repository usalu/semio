/** 🧾️ C11 probe: does a human who is ADDED to an existing space receive that space's earlier events (its `space.created`)
 * on the incremental event page (`after=<their frontier>`), or only the membership event? usage: bun probe-c11-membership-page.ts <hub> */
import { execFileSync } from "node:child_process";
const [hub = "http://127.0.0.1:8021"] = process.argv.slice(2);
const sign = async (email: string, password: string) => {
  const r = await fetch(`${hub}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: crypto.randomUUID().replace(/-/g, ""), clientClass: "browser" }) });
  return String(((await r.json()) as { token?: string }).token);
};
const page = async (token: string, after: number) => (await (await fetch(`${hub}/directory/event-page/v1?after=${after}`, { headers: { authorization: `Bearer ${token}` } })).json()) as { throughSeqInclusive: number; hasMore: boolean; authorizationGeneration: number; sessionBindingSha256: string; events: { seq: number; spaceId?: string; body: { kind: string } }[] };
const user2 = await sign("user2@semio.dev", "gm1-local-dev-pass-2");
const seed = (...args: string[]) => execFileSync("bun", ["c11-seed.ts", hub, ...args], { cwd: import.meta.dir, encoding: "utf8" });
const created = seed("create", `C11 membership probe ${Date.now() % 100000}`);
const spaceId = /SPACE (\S+)/u.exec(created)?.[1];
seed("create", `C11 membership filler ${Date.now() % 100000}`);
let frontier = 0;
for (;;) { const p = await page(user2, frontier); frontier = p.throughSeqInclusive; if (!p.hasMore) break; }
console.log("user2 frontier (after the space was created)", frontier, "space", spaceId);
const before = await page(user2, frontier);
console.log("authz generation before share", before.authorizationGeneration, "page before share:", before.events.filter((e) => e.spaceId === spaceId).map((e) => `${e.seq}:${e.body.kind}`));
seed("member", spaceId!, "user2@semio.dev", "author");
const after = await page(user2, frontier);
console.log("authz generation after share", after.authorizationGeneration, "page after share:", after.events.filter((e) => e.spaceId === spaceId).map((e) => `${e.seq}:${e.body.kind}`), "through", after.throughSeqInclusive);
const full = await page(user2, 0);
let events = full.events; let f = full.throughSeqInclusive; let more = full.hasMore;
while (more) { const p = await page(user2, f); events = events.concat(p.events); f = p.throughSeqInclusive; more = p.hasMore; }
console.log("full replay:", events.filter((e) => e.spaceId === spaceId).map((e) => `${e.seq}:${e.body.kind}`));
