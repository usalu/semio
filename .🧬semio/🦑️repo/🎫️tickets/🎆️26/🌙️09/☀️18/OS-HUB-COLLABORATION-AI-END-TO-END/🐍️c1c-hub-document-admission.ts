/** 🚪️ Slice C1c — what the running hub actually admits for a SHARED DOCUMENT, asked over plain HTTP
 * with two real signed-in humans, so the browser probe is not written against guesses. Signs both
 * humans in, creates one space as user1, shares it with user2, and then asks the hub for the calls a
 * two-browser shared-document run needs: the space listing from both sides, the space's own document
 * index, and a document open plan. Every answer is printed with its status and body — a refusal here
 * is the honest reason the browser run cannot reach a step, and names which hub feature gates it.
 *
 * Usage: bun 🐍️c1c-hub-document-admission.ts <hubOrigin>
 */
const hubOrigin = process.argv[2] ?? "http://127.0.0.1:7501";
const USERS = [
  { email: "user1@semio.dev", password: "collab e2e first human phrase" },
  { email: "user2@semio.dev", password: "collab e2e second human phrase" },
];

async function show(label: string, path: string, init: RequestInit = {}): Promise<{ status: number; body: string }> {
  const response = await fetch(`${hubOrigin}${path}`, init);
  const body = await response.text();
  console.log(`${response.status} ${init.method ?? "GET"} ${path} — ${label}\n    ${body.slice(0, 400)}`);
  return { status: response.status, body };
}

const bearers: string[] = [];
for (const user of USERS) {
  const minted = await show(`sign in ${user.email}`, "/auth/sessions", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ email: user.email, password: user.password }),
  });
  bearers.push(JSON.parse(minted.body).token);
}
const auth = (index: number): Record<string, string> => ({ authorization: `Bearer ${bearers[index]}`, "content-type": "application/json" });

console.log("\n--- readiness the run depends on ---");
const readiness = await show("readyz", "/readyz");
console.log(`    features: ${JSON.stringify(JSON.parse(readiness.body).features)}`);

console.log("\n--- directory: can two humans share one space? ---");
const spaceName = `c1c shared ${Date.now()}`;
const created = await show("user1 creates a space", "/directory/spaces", {
  method: "POST",
  headers: auth(0),
  body: JSON.stringify({ name: spaceName, kind: "studio", visibility: "public" }),
});
let spaceId: string | undefined;
try {
  const value = JSON.parse(created.body);
  spaceId = value.spaceId ?? value.id ?? value.space?.id;
} catch {
  spaceId = undefined;
}
console.log(`    spaceId=${spaceId ?? "(none)"}`);
await show("user1 lists spaces", "/directory/spaces", { headers: auth(0) });
await show("user2 lists spaces", "/directory/spaces", { headers: auth(1) });

if (spaceId !== undefined) {
  console.log("\n--- the document calls a shared editor makes ---");
  await show("space documents", `/spaces/${spaceId}/documents`, { headers: auth(0) });
  await show("open plan for a fresh document", `/spaces/${spaceId}/documents/c1c-shared-doc/open-plan`, { method: "POST", headers: auth(0), body: JSON.stringify({}) });
  await show("document head", `/spaces/${spaceId}/documents/c1c-shared-doc`, { headers: auth(0) });
  await show("admin connections", "/admin/api/connections", { headers: { authorization: "Bearer c1c-admin" } });
}
