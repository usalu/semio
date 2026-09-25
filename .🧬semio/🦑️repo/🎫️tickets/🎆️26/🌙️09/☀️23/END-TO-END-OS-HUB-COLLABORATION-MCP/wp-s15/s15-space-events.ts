#!/usr/bin/env bun
/** 📇️ S15: prints every directory event of one space from the hub's sealed event pages (kind, seq, document), the history a space index folds.
 * usage: bun s15-space-events.ts <hubOrigin> <email> <password> <spaceId> */
import { randomBytes } from "node:crypto";
const [origin = "http://127.0.0.1:8040", email = "user1@semio.dev", password = "", spaceId = ""] = process.argv.slice(2);
const signIn = await (await fetch(`${origin}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: randomBytes(16).toString("hex"), clientClass: "browser" }) })).json() as { token: string };
let after = 0;
for (let more = true; more; ) {
  const page = await (await fetch(`${origin}/directory/event-page/v1?after=${after}`, { headers: { authorization: `Bearer ${signIn.token}` } })).json() as { events: { seq: number; spaceId?: string; body: Record<string, unknown> }[]; throughSeqInclusive: number; hasMore: boolean };
  for (const event of page.events) if (event.spaceId === spaceId) console.log(event.seq, event.body.kind, JSON.stringify(event.body).slice(0, 260));
  after = page.throughSeqInclusive;
  more = page.hasMore;
}
