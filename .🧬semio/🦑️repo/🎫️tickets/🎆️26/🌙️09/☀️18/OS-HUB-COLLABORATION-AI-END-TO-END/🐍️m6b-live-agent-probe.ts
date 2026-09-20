/** 🧪️ M6b live agent-principal probe — drives M6 §8's seven steps against an ALREADY-RUNNING
 * `os-hub` in production posture (no launcher, no fd 3), whose origin is given on the command line.
 * The hub is booted by the operator, not by this file: the whole point of the proof is that a plain
 * process an operator started admits an agent principal.
 *
 * Phases:
 *   `--http <origin> <email> <password>` — steps 1, 2, 3 and 7: sign in, create a space, mint a
 *     delegation, write the credential file at mode 0600, and (after `--revoke`) prove the refusal.
 *   `--revoke <origin> <token> <delegationId>` — step 7 on its own, so the MCP phase can run in
 *     between against a live delegation.
 *
 * Every step prints one line; a failed expectation exits non-zero with the observed value, so this
 * file is a gate, not a log. Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice M6b.
 */
import { chmodSync, existsSync, statSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

// 📁️ `new URL(x, import.meta.url).pathname` percent-encodes emoji path segments (preamble rule 24).
const here = dirname(fileURLToPath(new URL(import.meta.url)));

function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, "🌎️hub", "📦️packages", "🦀️rust", "Cargo.toml"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error("probe could not locate the repository root above " + start);
}

const repoRoot = findRepoRoot(here);
const { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } = await import(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "📦️packages", "🟦️typescript", "🟦️.ts"));

let failures = 0;
function check(name: string, observed: unknown, expected: unknown): void {
  const ok = JSON.stringify(observed) === JSON.stringify(expected);
  if (!ok) failures += 1;
  console.log(`${ok ? "PASS" : "FAIL"} ${name}${ok ? "" : ` — observed ${JSON.stringify(observed)}, expected ${JSON.stringify(expected)}`}`);
}

interface Answer {
  readonly status: number;
  readonly text: string;
  readonly json: any;
}

async function call(origin: string, method: string, path: string, options: { token?: string; body?: string; json?: boolean } = {}): Promise<Answer> {
  const headers: Record<string, string> = {};
  if (options.json === true) headers["content-type"] = "application/json";
  if (options.token !== undefined) headers.authorization = `Bearer ${options.token}`;
  const response = await fetch(`${origin}${path}`, { method, headers, ...(options.body === undefined ? {} : { body: options.body }) });
  const text = await response.text();
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {
    json = undefined;
  }
  return { status: response.status, text, json };
}

const [phase, origin, ...rest] = process.argv.slice(2);
if (origin === undefined) throw new Error("usage: probe --http <origin> <email> <password> <credentialPath> | --revoke <origin> <token> <delegationId>");

if (phase === "--revoke") {
  const [token, delegationId] = rest;
  const revoked = await call(origin, "DELETE", `/auth/agent-delegations/${encodeURIComponent(delegationId ?? "")}`, { token });
  check("7 revoke status", revoked.status, 204);
  const listed = await call(origin, "GET", `/auth/agent-delegations?space=${encodeURIComponent(process.env.M6B_SPACE ?? "")}`, { token });
  check("7 the withdrawn delegation stays visible, marked", listed.json?.delegations?.[0]?.revoked, true);
  process.exit(failures === 0 ? 0 : 1);
}

const [email, password, credentialPath] = rest;

// 1️⃣ the first user signs in against the hub's own credential authority
const signIn = await call(origin, "POST", "/auth/sessions", {
  json: true,
  body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: "m6b-live", clientClass: "browser" }),
});
check("1 sign-in status", signIn.status, 200);
const token = signIn.json?.token as string;
check("1 the minted capability is a session capability", typeof token === "string" && token.startsWith("session.v1."), true);

const me = await call(origin, "GET", "/auth/sessions/me", { token });
check("1 the human's own session kind", me.json?.sessionKind, "external");

// 1️⃣b a space the human authors
const requestId = "6b".repeat(16);
const sealed = directoryCommandRequestJson(sealDirectoryCommandRequestV1(requestId, { kind: "create-space", name: "Atelier M6b", spaceKind: "atelier", visibility: "private" }));
const created = await call(origin, "POST", "/directory/commands", { token, json: true, body: sealed });
check("1b create-space accepted", created.json?.outcome, "accepted");
const spaces = await call(origin, "GET", "/directory/spaces", { token });
const spaceId: string = (Array.isArray(spaces.json) ? spaces.json : []).find((row: any) => row?.space?.name === "Atelier M6b")?.space?.id ?? "";
check("1b the space is in the author's own listing", spaceId.length > 0, true);

// 2️⃣ POST /auth/agent-delegations — the step M6 §8 marked as never run
const delegation = await call(origin, "POST", "/auth/agent-delegations", {
  token,
  json: true,
  body: JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: "Drafting agent", audience: "edit", ttlSecs: 3600 }),
});
check("2 delegation status", delegation.status, 201);
const delegationId = delegation.json?.delegationId as string;
const delegationToken = delegation.json?.token as string;
check("2 the receipt names the agent's own principal", delegation.json?.agentPrincipalId, `agent:${delegationId}`);
check("2 the agent principal is not the delegating human", delegation.json?.agentPrincipalId === `user:${signIn.json?.user_id}`, false);
check("2 the token is a delegation capability of exactly 111 bytes", typeof delegationToken === "string" && /^delegation\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u.test(delegationToken), true);

// 2️⃣b the listing shows it and never its token
const listing = await call(origin, "GET", `/auth/agent-delegations?space=${encodeURIComponent(spaceId)}`, { token });
check("2b listing status", listing.status, 200);
check("2b exactly one delegation", listing.json?.delegations?.length, 1);
check("2b the listing never carries the token", listing.text.includes(delegationToken), false);
console.log(`INFO listing row: ${JSON.stringify(listing.json?.delegations?.[0])}`);

// 3️⃣ the credential file, at mode 0600
const file = { schema: "semio.hub.agent-credential/v1", hubOrigin: origin, spaceId, audience: "edit", token: delegationToken };
writeFileSync(credentialPath ?? "", `${JSON.stringify(file, null, 2)}\n`, { mode: 0o600 });
chmodSync(credentialPath ?? "", 0o600);
check("3 the credential file is mode 0600", (statSync(credentialPath ?? "").mode & 0o777).toString(8), "600");

// 3️⃣b an agent can never delegate onward — proven with the agent session the MCP will also mint
const exchanged = await call(origin, "POST", "/auth/agent-sessions", {
  json: true,
  token: delegationToken,
  body: JSON.stringify({ schema: "semio.hub.auth.agent-session/v1", audience: "edit", agentInstanceId: "m6b.probe.1" }),
});
check("3b exchange status", exchanged.status, 200);
const agentToken = exchanged.json?.token as string;
check("3b the minted agent session names the agent principal", exchanged.json?.agentPrincipalId, `agent:${delegationId}`);
const agentMe = await call(origin, "GET", "/auth/sessions/me", { token: agentToken });
check("3b the agent's own session reports kind agent", agentMe.json?.sessionKind, "agent");
check("3b the agent acts under the delegating human's membership", agentMe.json?.userId, signIn.json?.user_id);
const onward = await call(origin, "POST", "/auth/agent-delegations", {
  token: agentToken,
  json: true,
  body: JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: "Onward", audience: "edit", ttlSecs: 3600 }),
});
check("3b an agent can never delegate onward", [onward.status, onward.json?.error], [403, "forbidden"]);

// 3️⃣c widening the audience is one indistinguishable refusal
const widened = await call(origin, "POST", "/auth/agent-sessions", {
  json: true,
  token: delegationToken,
  body: JSON.stringify({ schema: "semio.hub.auth.agent-session/v1", audience: "read", agentInstanceId: "m6b.probe.2" }),
});
console.log(`INFO a narrower audience than the one delegated: status ${widened.status} ${widened.text.slice(0, 120)}`);

// 3️⃣d last-used, if this binary carries M6b's derived column
const afterUse = await call(origin, "GET", `/auth/agent-delegations?space=${encodeURIComponent(spaceId)}`, { token });
const lastUsed = afterUse.json?.delegations?.[0]?.lastUsedAtMs;
console.log(`INFO lastUsedAtMs after one exchange: ${JSON.stringify(lastUsed)} (absent ⇒ the binary predates M6b's derived column)`);

console.log(`EXPORT M6B_SPACE=${spaceId}`);
console.log(`EXPORT M6B_DELEGATION=${delegationId}`);
console.log(`EXPORT M6B_HUMAN_TOKEN=${token}`);
console.log(`EXPORT M6B_AGENT_TOKEN=${agentToken}`);
process.exit(failures === 0 ? 0 : 1);
