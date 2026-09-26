#!/usr/bin/env bun
/** 🏘️ S15: signs in to a hub over HTTP as a provisioned dev user and lists the spaces it sees (name, id, member count,
 * indexed documents); with `create <name>` it first creates a private atelier space through the directory command lane.
 * usage: bun s15-hub-spaces.ts <origin> [create <name>] */
import { randomBytes } from "node:crypto";

const [origin = "http://127.0.0.1:8040", verb = "", name = ""] = process.argv.slice(2);
const EMAIL = process.env.S6_SIGN_IN_EMAIL ?? "user1@semio.dev";
const PASSWORD = process.env.S6_SIGN_IN_PASSWORD ?? "gm1-local-dev-pass-1";
const hub = async (method: string, path: string, token?: string, body?: unknown) => {
  const response = await fetch(`${origin}${path}`, { method, headers: { ...(token ? { authorization: `Bearer ${token}` } : {}), ...(body === undefined ? {} : { "content-type": "application/json" }) }, body: body === undefined ? undefined : JSON.stringify(body) });
  const text = await response.text();
  let json: any = null;
  try {
    json = JSON.parse(text);
  } catch {}
  return { status: response.status, json, text };
};
const signIn = await hub("POST", "/auth/sessions", undefined, { schema: "semio.hub.auth.credential-sign-in/v1", email: EMAIL, password: PASSWORD, deviceInstanceId: `s15spaces${randomBytes(12).toString("hex")}`, clientClass: "browser" });
const token = String(signIn.json?.token ?? "");
console.log(`sign-in ${signIn.status}`);
if (verb === "create") {
  const { directoryCommandRequestJson, sealDirectoryCommandRequestV1, parseDirectoryCommandReceiptV1 } = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts");
  const { createSpaceCommandV1 } = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts");
  const request = sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(name, "atelier", "private"));
  const created = await hub("POST", "/directory/commands", token, JSON.parse(directoryCommandRequestJson(request)));
  console.log(`create ${created.status} ${created.text.slice(0, 200)}`);
  console.log(`shell receipt parse: ${await Promise.resolve().then(() => parseDirectoryCommandReceiptV1(created.text, request)).then(() => "ok", (error) => `REFUSED ${String(error).slice(0, 300)}`)}`);
}
const spaces = await hub("GET", "/directory/spaces", token);
console.log(`spaces ${spaces.status}`);
for (const entry of spaces.json ?? []) console.log(JSON.stringify({ id: entry?.space?.id, name: entry?.space?.name, members: entry?.members?.length, documents: entry?.documents?.length ?? entry?.indexed?.length }));
if (!Array.isArray(spaces.json)) console.log(spaces.text);
