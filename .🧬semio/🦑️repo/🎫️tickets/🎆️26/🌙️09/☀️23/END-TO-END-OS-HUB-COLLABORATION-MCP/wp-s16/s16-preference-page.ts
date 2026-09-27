#!/usr/bin/env bun
/** 🎚️ S16 item 3: reads one hub user's preference lane over HTTP (sign in → `GET /directory/preference-page/v1?after=0`) and
 * prints the status and the recorded preference events — is a device's customization on the hub at all?
 * usage: bun s16-preference-page.ts <hubOrigin> [email] [password] */
import { randomBytes } from "node:crypto";
const [origin = "http://127.0.0.1:8042", email = "user1@semio.dev", password = "gm1-local-dev-pass-1"] = process.argv.slice(2);
const signIn = await fetch(`${origin}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: randomBytes(16).toString("hex"), clientClass: "browser" }) });
const token = ((await signIn.json()) as { token: string }).token;
for (const path of ["/directory/preference-page/v1?after=0", "/directory/event-page/v1?after=0"]) {
  const response = await fetch(`${origin}${path}`, { headers: { authorization: `Bearer ${token}` } });
  const text = await response.text();
  console.log(`${path} → ${response.status} ${text.length} B`);
  console.log(text.slice(0, 1500));
}
