#!/usr/bin/env bun
/** 📊️ H12 one-off (ticket 26/09/23 session 13): signs user1 in, reads `GET /admin/api/observability`, writes the body and
 * validates it against `HubObservabilityV1` with Ajv (third-party oracle).
 *   bun obs-read.ts <origin> <out.json> */
import Ajv from "ajv";
import { randomBytes } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";

const [origin, out] = process.argv.slice(2);
const read = (path: string) => JSON.parse(readFileSync(path, "utf8"));
const signIn = await fetch(`${origin}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1", deviceInstanceId: `h12obs${randomBytes(12).toString("hex")}`, clientClass: "browser" }) });
const token = String(((await signIn.json()) as { token?: string }).token ?? "");
const response = await fetch(`${origin}/admin/api/observability`, { headers: { authorization: `Bearer ${token}` } });
const text = await response.text();
writeFileSync(out!, text);
const ajv = new Ajv({ allErrors: true, strict: false });
ajv.addSchema(read("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json"));
ajv.addSchema(read("/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️.json"));
const validate = ajv.compile(read("/Users/ueli/Documents/semio/🌎️hub/📊️observability/🧬️schema/🔣️.json"));
const body = JSON.parse(text);
console.log(`observability status=${response.status} valid=${validate(body)} errors=${JSON.stringify(validate.errors ?? []).slice(0, 400)}`);
console.log(`residency ${JSON.stringify(body.residency)}`);
console.log(`catalog ${JSON.stringify(body.catalog)?.slice(0, 1_500)}`);
for (const route of [...(body.routes ?? [])].sort((a, b) => b.p95Us - a.p95Us).slice(0, 12)) console.log(`route ${route.method} ${route.route} requests=${route.requests} ok=${route.successes} p50Us=${route.p50Us} p95Us=${route.p95Us} maxUs=${route.maxUs}`);
console.log(`dbIo ${JSON.stringify({ ...body.dbIo, kinds: undefined })}`);
