#!/usr/bin/env bun
/** 📊️ H14 one-off: signs the probe user in (credentials only from `OS_HUB_PROBE_EMAIL` / `OS_HUB_PROBE_PASSWORD`), reads
 * `GET /admin/api/observability`, writes the body. bun h14-obs-read.ts <origin> <out.json> */
import { randomBytes } from "node:crypto";
import { writeFileSync } from "node:fs";

const [origin, out] = process.argv.slice(2);
const signIn = await fetch(`${origin}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: process.env.OS_HUB_PROBE_EMAIL, password: process.env.OS_HUB_PROBE_PASSWORD, deviceInstanceId: `h14obs${randomBytes(12).toString("hex")}`, clientClass: "browser" }) });
const token = String(((await signIn.json()) as { token?: string }).token ?? "");
const response = await fetch(`${origin}/admin/api/observability`, { headers: { authorization: `Bearer ${token}` } });
writeFileSync(out!, await response.text());
console.log(`sign-in ${signIn.status} observability ${response.status}`);
