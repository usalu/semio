/** 🔎️ WG8: signs in and asks the hub for one document's execution-target manifest, printing status + body. */
const [origin, space, document, surface] = process.argv.slice(2);
const mint = await fetch(`${origin}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1", deviceInstanceId: "wg8-probe-0000000000000001", clientClass: "native" }) });
const minted = await mint.json().catch(() => ({}));
console.log("mint", mint.status, Object.keys(minted));
const token = minted.token ?? minted.sessionToken ?? minted.capability;
const intent = { schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId: space, documentId: document }, requestedSurfaceId: surface, clientInstanceId: "wg8-probe" };
for (const asset of ["manifest", "descriptor", "component"]) {
  const answer = await fetch(`${origin}/spaces/${encodeURIComponent(space)}/documents/${encodeURIComponent(document)}/execution-target/${asset}`, { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${token}` }, body: JSON.stringify(intent) });
  const started = Date.now(); const bytes = new Uint8Array(await answer.arrayBuffer()); const text = asset === "manifest" ? new TextDecoder().decode(bytes) : `${bytes.length} bytes in ${Date.now() - started} ms`;
  console.log(asset, answer.status, text.slice(0, 600));
}
