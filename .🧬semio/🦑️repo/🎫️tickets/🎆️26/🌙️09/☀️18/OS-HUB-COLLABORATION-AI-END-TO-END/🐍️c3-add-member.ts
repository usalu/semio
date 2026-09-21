/** 👥️ C3 — add the SECOND human to the shared space, through the product's own directory command.
 *
 * `authorize_directory_command` (`🌎️hub/🏗️bootstrap/🦀️.rs:5901`) admits `UpsertMember` from any AUTHOR
 * of the named space, so the space's author (user1) invites user2 — no admin provider needed. The
 * request body is sealed with the product's own `sealDirectoryCommandRequestV1` +
 * `directoryCommandRequestJson`, which is the exact canonical JSON the hub's
 * `DirectoryCommandRequestV1::parse_canonical_json` accepts.
 *
 * Usage: bun 🐍️c3-add-member.ts [hubOrigin] [spaceId] [inviteeEmail] [role]
 */
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";

const ORIGIN = process.argv[2] ?? "http://127.0.0.1:7611";
const SPACE = process.argv[3] ?? "01a0c00f-4f3c-7834-a7e6-2ccf9de925db";
const EMAIL = process.argv[4] ?? "user2@semio.dev";
const ROLE = (process.argv[5] ?? "author") as "author" | "reader";
const AUTHOR = { email: "user1@semio.dev", password: "gm1-local-dev-pass-1" };

const mint = async (email: string, password: string): Promise<string> => {
  const response = await fetch(`${ORIGIN}/auth/sessions`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: `c3addmember${email.length}0000000000000000000`.slice(0, 32), clientClass: "browser" }),
  });
  const body = (await response.json()) as { token?: string; error?: string };
  if (!body.token) throw new Error(`mint ${email}: ${response.status} ${JSON.stringify(body)}`);
  return body.token;
};

const token = await mint(AUTHOR.email, AUTHOR.password);
const requestId = [...crypto.getRandomValues(new Uint8Array(16))].map((byte) => byte.toString(16).padStart(2, "0")).join("");
const sealed = sealDirectoryCommandRequestV1(requestId, { kind: "upsert-member", spaceId: SPACE, email: EMAIL, role: ROLE });
const canonical = directoryCommandRequestJson(sealed);
console.log(`REQUEST ${canonical}`);

const response = await fetch(`${ORIGIN}/directory/commands`, {
  method: "POST",
  headers: { "content-type": "application/json", authorization: `Bearer ${token}` },
  body: canonical,
});
const text = await response.text();
console.log(`COMMAND HTTP ${response.status} ${text.slice(0, 600)}`);

for (const who of [
  { label: "user1", email: AUTHOR.email, password: AUTHOR.password },
  { label: "user2", email: "user2@semio.dev", password: "gm1-local-dev-pass-2" },
]) {
  const bearer = await mint(who.email, who.password);
  const spaces = await fetch(`${ORIGIN}/directory/spaces`, { headers: { authorization: `Bearer ${bearer}` } }).then((r) => r.text());
  console.log(`${who.label} spaces ${spaces.slice(0, 340)}`);
}
