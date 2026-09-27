/** 🔎️ G12: prints the hub's artifact-creation catalog shape (first space of the signed-in user) — credentials from env. */
import { randomBytes } from "node:crypto";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { createSpaceCommandV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts";
const HUB = process.argv[2]!;
const call = async (method: string, path: string, token?: string, body?: string) => (await fetch(`${HUB}${path}`, { method, headers: { ...(body ? { "content-type": "application/json" } : {}), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body ? { body } : {}) })).json().catch(() => null);
const token = String((await call("POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: process.env.OS_MCP_HUB_EMAIL, password: process.env.OS_MCP_HUB_PASSWORD, deviceInstanceId: `g12peek${randomBytes(10).toString("hex")}`, clientClass: "browser" })))?.token ?? "");
const name = `G12 peek ${randomBytes(3).toString("hex")}`;
await call("POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(name, "atelier", "private"))));
const spaceId = String(((await call("GET", "/directory/spaces", token)) ?? []).find((entry: any) => entry?.space?.name === name)?.space?.id ?? "");
const catalog = await call("GET", `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`, token);
console.log(JSON.stringify({ catalogGenerationId: catalog?.catalogGenerationId, count: catalog?.kinds?.length, first: catalog?.kinds?.[0], kinds: (catalog?.kinds ?? []).map((kind: any) => kind.kindId) }, null, 1).slice(0, 3000));
