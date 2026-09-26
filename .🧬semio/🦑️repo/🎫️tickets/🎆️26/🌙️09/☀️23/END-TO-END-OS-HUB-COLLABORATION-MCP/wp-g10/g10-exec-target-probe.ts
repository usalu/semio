#!/usr/bin/env bun
/** 🔎️ G10: asks a hub for every document's execution-target manifest and descriptor in one space, as the MCP gateway's
 * catalog refresh does, and prints each HTTP status with the head of its body — the status the gateway's
 * `map_catalog_client_error` folds into "hub directory is temporarily unavailable".
 * usage: bun g10-exec-target-probe.ts <hubOrigin> <spaceId> (credentials: OS_MCP_HUB_EMAIL / OS_MCP_HUB_PASSWORD, local test users) */
import { randomBytes } from "node:crypto";

const [ORIGIN, SPACE] = process.argv.slice(2);
const EMAIL = process.env.OS_MCP_HUB_EMAIL ?? "user1@semio.dev";
const PASSWORD = process.env.OS_MCP_HUB_PASSWORD ?? "gm1-local-dev-pass-1";
const call = async (method: string, path: string, token?: string, body?: unknown) => {
  const started = Date.now();
  const response = await fetch(`${ORIGIN}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body: JSON.stringify(body) }) });
  const text = await response.text();
  return { status: response.status, ms: Date.now() - started, text };
};
const signIn = await call("POST", "/auth/sessions", undefined, { schema: "semio.hub.auth.credential-sign-in/v1", email: EMAIL, password: PASSWORD, deviceInstanceId: `g10probe${randomBytes(12).toString("hex")}`, clientClass: "browser" });
const token = String(JSON.parse(signIn.text).token ?? "");
const space = await call("GET", `/directory/spaces/${encodeURIComponent(SPACE)}`, token);
const documents: string[] = [...space.text.matchAll(/"(?:document_id|documentId)":"(artifact-[0-9a-f]{32})"/g)].map((match) => match[1]!).filter((id, index, all) => all.indexOf(id) === index);
console.log(`space ${SPACE}: HTTP ${space.status}, documents ${JSON.stringify(documents)}`);
for (const [index, documentId] of documents.entries()) {
  const intent = { schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId: SPACE, documentId }, requestedSurfaceId: null, clientInstanceId: `g10-probe-${index}` };
  for (const part of ["manifest", "descriptor"]) {
    const answer = await call("POST", `/spaces/${encodeURIComponent(SPACE)}/documents/${encodeURIComponent(documentId)}/execution-target/${part}`, token, intent);
    console.log(`${documentId} ${part}: HTTP ${answer.status} in ${answer.ms} ms — ${answer.text.slice(0, 300).replace(/\s+/g, " ")}`);
  }
}
