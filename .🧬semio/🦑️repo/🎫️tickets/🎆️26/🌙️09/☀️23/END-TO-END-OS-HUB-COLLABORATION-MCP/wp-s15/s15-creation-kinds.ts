#!/usr/bin/env bun
/** 🏷️ S15: prints a hub's space artifact creation catalog (kind id, schema, dialect, labels) in its own order, for picking a kind by index.
 * usage: bun s15-creation-kinds.ts <hubOrigin> <email> <password> <spaceId> */
import { randomBytes } from "node:crypto";
const [origin = "http://127.0.0.1:8040", email = "user1@semio.dev", password = "", spaceId = ""] = process.argv.slice(2);
const json = async (method: string, path: string, token: string | null, body?: unknown) => (await fetch(`${origin}${path}`, { method, headers: { ...(token ? { authorization: `Bearer ${token}` } : {}), "content-type": "application/json" }, body: body === undefined ? undefined : JSON.stringify(body), signal: AbortSignal.timeout(10_000) })).json() as Promise<any>;
const { token } = await json("POST", "/auth/sessions", null, { schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: randomBytes(16).toString("hex"), clientClass: "browser" });
const catalog = await json("GET", `/spaces/${spaceId}/artifact-creations`, token);
console.log(`generation ${catalog.catalogGenerationId} space ${spaceId}`);
catalog.kinds.forEach((kind: any, index: number) => console.log(`${index} ${kind.kindId} ${kind.schema} ${kind.dialect.artifactKind}@${kind.dialect.standard}/${kind.dialect.subset} ${kind.label.en}/${kind.label.de}`));
