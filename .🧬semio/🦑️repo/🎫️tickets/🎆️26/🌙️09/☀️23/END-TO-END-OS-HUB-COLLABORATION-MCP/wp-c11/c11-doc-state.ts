/** 🔎️ C10: prints the hub's own view of one document (GET /spaces/{space}/documents/{id}) as user1 — the frontier witness.
 * Usage: bun c10-doc-state.ts <hubOrigin> <spaceId> <documentId> */
const [origin, space, doc] = process.argv.slice(2);
const mint = await fetch(`${origin}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1", deviceInstanceId: "c10docstate00000000000000000000x", clientClass: "browser" }) }).then((r) => r.json() as Promise<{ token: string }>);
const response = await fetch(`${origin}/spaces/${space}/documents/${doc}`, { headers: { authorization: `Bearer ${mint.token}` } });
console.log(`HTTP ${response.status} ${(await response.text()).slice(0, 1500)}`);
