#!/usr/bin/env bun
/** 🏘️ G10: lists a user's spaces on a hub, and archives the ones a G10 probe created (names this slice's scripts mint).
 * usage: bun g10-space-census.ts <hubOrigin> <email> <password> [--archive-g10] */
import { randomBytes } from "node:crypto";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { archiveSpaceCommandV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts";
const [HUB, EMAIL, PASSWORD, FLAG] = process.argv.slice(2);
const token = ((await (await fetch(`${HUB}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: EMAIL, password: PASSWORD, deviceInstanceId: `g10census${randomBytes(11).toString("hex")}`, clientClass: "browser" }) })).json()) as any).token;
const list = async () => ((await (await fetch(`${HUB}/directory/spaces`, { headers: { authorization: `Bearer ${token}` } })).json()) as any[]);
const spaces = await list();
for (const entry of spaces) console.log(`${entry.space?.id} ${entry.access} ${entry.space?.archived ?? ""} ${entry.space?.name}`);
if (FLAG === "--archive-g10") {
  for (const entry of spaces.filter((row) => /^(G10 |Hub agent participant |Hub edit durability )/u.test(String(row.space?.name ?? "")))) {
    const response = await fetch(`${HUB}/directory/commands`, { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${token}` }, body: directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), archiveSpaceCommandV1(entry.space.id))) });
    console.log(`archive ${entry.space.id} ${entry.space.name} → ${response.status}`);
  }
  console.log(`after: ${(await list()).length} listed`);
}
