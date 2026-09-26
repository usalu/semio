#!/usr/bin/env bun
/** ⏱️ S15: polls one space artifact creation's status on a hub every 5 s until it leaves `accepted`/`preparing` or the bound
 * passes, printing each phase change with its elapsed time.
 * usage: bun s15-creation-status.ts <hubOrigin> <spaceId> <requestId> [boundSeconds] */
import { randomBytes } from "node:crypto";
const [origin = "http://127.0.0.1:7800", spaceId = "", requestId = "", bound = "300"] = process.argv.slice(2);
const signIn = await fetch(`${origin}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1", deviceInstanceId: randomBytes(16).toString("hex"), clientClass: "browser" }) });
const token = ((await signIn.json()) as { token: string }).token;
const started = Date.now();
let last = "";
while (Date.now() - started < Number(bound) * 1000) {
  const response = await fetch(`${origin}/spaces/${spaceId}/artifact-creations/${requestId}`, { headers: { authorization: `Bearer ${token}` } });
  const text = await response.text();
  const phase = /"phase":"([a-z-]+)"/u.exec(text)?.[1] ?? `http-${response.status}`;
  if (phase !== last) console.log(`${Math.round((Date.now() - started) / 1000)} s ${response.status} ${phase} ${text.slice(0, 300)}`);
  last = phase;
  if (!/^(accepted|preparing)$/u.test(phase)) break;
  await Bun.sleep(5_000);
}
console.log(`final ${last} after ${Math.round((Date.now() - started) / 1000)} s`);
