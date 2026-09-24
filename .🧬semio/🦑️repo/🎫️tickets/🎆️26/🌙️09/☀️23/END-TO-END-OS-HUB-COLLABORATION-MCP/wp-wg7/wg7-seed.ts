/** 🌱️ WG7 (copied from C10) directory commands through the product's own sealed request (the same bytes the shell posts).
 * Usage (from wp-wg7): bun wg7-seed.ts <hubOrigin> spaces
 *                      bun wg7-seed.ts <hubOrigin> create <name>
 *                      bun wg7-seed.ts <hubOrigin> member <spaceId> <email> <author|spectator>
 *                      bun wg7-seed.ts <hubOrigin> remove <spaceId> <userId>
 *                      bun wg7-seed.ts <hubOrigin> artifact <spaceId> <kindId> <name>   (prints `DOCUMENT <id>`)
 * Acts as user1 (the space author); prints `SPACE <id>` for `create`. */
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { parseSpaceArtifactCreationCatalogJsonV1, parseSpaceArtifactCreationStatusJsonV1, sealSpaceArtifactCreateV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";

const [origin, verb, ...rest] = process.argv.slice(2);
const USERS: Record<string, string> = { "user1@semio.dev": "gm1-local-dev-pass-1", "user2@semio.dev": "gm1-local-dev-pass-2" };

async function mint(email: string): Promise<string> {
  const response = await fetch(`${origin}/auth/sessions`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password: USERS[email], deviceInstanceId: `wg7seed0${email.replace(/\W/g, "")}0000000000000000000000`.slice(0, 32), clientClass: "browser" }),
  });
  const body = (await response.json()) as { token?: string };
  if (!body.token) throw new Error(`mint ${email}: HTTP ${response.status}`);
  return body.token;
}

async function spaces(token: string): Promise<Array<{ id: string; name: string }>> {
  const text = await fetch(`${origin}/directory/spaces`, { headers: { authorization: `Bearer ${token}` } }).then((r) => r.text());
  const parsed = JSON.parse(text) as unknown;
  const rows = Array.isArray(parsed) ? parsed : ((parsed as { spaces?: unknown[] }).spaces ?? []);
  return rows.map((row) => ((row as { space?: { id: string; name: string } }).space ?? (row as { id: string; name: string })));
}

async function command(token: string, body: Parameters<typeof sealDirectoryCommandRequestV1>[1]): Promise<void> {
  const requestId = [...crypto.getRandomValues(new Uint8Array(16))].map((byte) => byte.toString(16).padStart(2, "0")).join("");
  const response = await fetch(`${origin}/directory/commands`, { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${token}` }, body: directoryCommandRequestJson(sealDirectoryCommandRequestV1(requestId, body)) });
  console.log(`COMMAND ${body.kind} HTTP ${response.status} ${(await response.text()).slice(0, 400)}`);
  if (!response.ok) process.exitCode = 1;
}

const author = await mint("user1@semio.dev");
if (verb === "spaces") {
  for (const email of Object.keys(USERS)) console.log(`${email} ${JSON.stringify(await spaces(await mint(email)))}`);
} else if (verb === "create") {
  const name = rest[0]!;
  await command(author, { kind: "create-space", name, spaceKind: "studio", visibility: "public" });
  for (let attempt = 0; attempt < 40; attempt += 1) {
    const row = (await spaces(author)).find((space) => space.name === name);
    if (row) {
      console.log(`SPACE ${row.id}`);
      break;
    }
    await Bun.sleep(250);
  }
} else if (verb === "member") {
  await command(author, { kind: "upsert-member", spaceId: rest[0]!, email: rest[1]!, role: rest[2] as "author" | "spectator" });
} else if (verb === "remove") {
  await command(author, { kind: "remove-member", spaceId: rest[0]!, userId: rest[1]! });
} else if (verb === "artifact") {
  const [spaceId, kindId, name] = rest as [string, string, string];
  const path = `${origin}/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const catalog = parseSpaceArtifactCreationCatalogJsonV1(await fetch(path, { headers: { authorization: `Bearer ${author}` } }).then((r) => r.text()));
  const requestId = [...crypto.getRandomValues(new Uint8Array(16))].map((byte) => byte.toString(16).padStart(2, "0")).join("");
  const body = JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: catalog.catalogGenerationId, kindId, name }));
  const posted = await fetch(path, { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${author}` }, body });
  console.log(`CREATE HTTP ${posted.status} ${(await posted.text()).slice(0, 300)}`);
  for (let attempt = 0; attempt < 600; attempt += 1) {
    const response = await fetch(`${path}/${requestId}`, { headers: { authorization: `Bearer ${author}` } });
    const text = await response.text();
    const status = response.ok ? parseSpaceArtifactCreationStatusJsonV1(text) : null;
    if (attempt % 20 === 0 || status?.phase !== "accepted") console.log(`STATUS ${attempt} HTTP ${response.status} ${text.slice(0, 400)}`);
    if (status?.phase === "ready") {
      console.log(`DOCUMENT ${status.ready!.artifactId}`);
      break;
    }
    if (status?.phase === "failed" || status?.phase === "cancelled" || status?.phase === "indeterminate") {
      process.exitCode = 1;
      break;
    }
    await Bun.sleep(500);
  }
} else throw new Error(`unknown verb ${verb}`);
