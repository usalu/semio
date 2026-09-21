/** 🤖️ M8 — mints a delegated agent credential for an EXISTING hub space that already holds a real
 * document, and writes it as the `semio.hub.agent-credential/v1` file `semio-os-mcp
 * --credential-file` reads. M6b's own probe created a fresh empty space; this slice needs the agent
 * inside the space a human is already editing, because the thing being proven is a THIRD
 * participant in an occupied room, not a private one.
 *
 * Usage: bun 🐍️m8-agent-delegate.ts <origin> <spaceId> <email> <password> <credentialPath> [audience]
 * Prints `DELEGATION <json>` and `CREDENTIAL <path>`; exits non-zero on the first refusal.
 */
import { chmodSync, statSync, writeFileSync } from "node:fs";

const [origin, spaceId, email, password, credentialPath, audience = "edit"] = process.argv.slice(2);
if (!origin || !spaceId || !email || !password || !credentialPath) throw new Error("usage: bun 🐍️m8-agent-delegate.ts <origin> <spaceId> <email> <password> <credentialPath> [audience]");

async function call(method: string, path: string, options: { token?: string; body?: string } = {}): Promise<{ status: number; text: string; json: any }> {
  const headers: Record<string, string> = {};
  if (options.body !== undefined) headers["content-type"] = "application/json";
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

const signIn = await call("POST", "/auth/sessions", { body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: "m8delegate0000000000000000000000".slice(0, 32), clientClass: "browser" }) });
if (signIn.status !== 200 || typeof signIn.json?.token !== "string") throw new Error(`sign-in ${signIn.status}: ${signIn.text.slice(0, 300)}`);
const token: string = signIn.json.token;
console.log(`SIGNIN 200 user=${signIn.json?.user_id ?? signIn.json?.userId} sessionKind=${signIn.json?.sessionKind ?? "<unset>"}`);

const delegation = await call("POST", "/auth/agent-delegations", {
  token,
  body: JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: "M8 participant agent", audience, ttlSecs: 7200 }),
});
if (delegation.status !== 201) throw new Error(`delegation ${delegation.status}: ${delegation.text.slice(0, 400)}`);
const delegationId: string = delegation.json.delegationId;
const delegationToken: string = delegation.json.token;
console.log(`DELEGATION ${JSON.stringify({ delegationId, agentPrincipalId: delegation.json.agentPrincipalId, audience, spaceId })}`);

writeFileSync(credentialPath, `${JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: origin, spaceId, audience, token: delegationToken })}\n`, { mode: 0o600 });
chmodSync(credentialPath, 0o600);
console.log(`CREDENTIAL ${credentialPath} mode=${(statSync(credentialPath).mode & 0o777).toString(8)}`);
console.log(`HUMAN_TOKEN ${token}`);
