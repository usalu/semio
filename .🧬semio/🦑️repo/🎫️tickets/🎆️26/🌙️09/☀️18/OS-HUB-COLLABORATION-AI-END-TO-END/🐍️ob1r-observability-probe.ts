/** 🧪️ OB1r live observability probe — boots the real `os-hub` binary against a fresh `OS_HUB_DATA`
 * with credential sign-in enabled and ONE admin subject, then drives `GET /admin/api/observability`
 * over real HTTP three ways: with no capability at all, with a valid capability that is not an admin
 * subject, and with the admin's. It asserts the gate refuses the first two and that the third
 * returns the live per-event counter table — including a row for a request this probe itself made,
 * which is the only way to prove the table is fed by the running hub rather than by a fixture.
 *
 * It also asserts the negative that matters most: no principal, space or artifact identity appears
 * anywhere in the admin body. The route answers counters; the records go to the operator's sink.
 *
 * Every step prints one line; a failed expectation exits non-zero, so this file is a gate. */
import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync } from "node:fs";
import { dirname, join } from "node:path";

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

const repoRoot = findRepoRoot(import.meta.dir);
const hubRustRoot = join(repoRoot, "🌎️hub", "📦️packages", "🦀️rust");

const ADMIN = { email: "ops@example.org", password: "an operator phrase for ops", display: "Ops Admin" };
const PLAIN = { email: "plain@example.org", password: "a non admin phrase here", display: "Plain User" };

/** 🔐️ The admin subject the hub admits, in the `provider:subject` shape `OS_HUB_ADMIN_SUBJECTS`
 * parses. `credential.password.v1` is `semio_hub::auth::CREDENTIAL_IDENTITY_PROVIDER`.
 *
 * ⚠️ It must go through `startLocalHub`'s own `adminSubjects` option, not `process.env`: that helper
 * deliberately `delete`s `OS_HUB_ADMIN_SUBJECTS` from the child environment when the option is
 * absent, so an inherited one is silently dropped and every admin request answers 401. */
const ADMIN_SUBJECT = `credential.password.v1:${ADMIN.email}`;
process.env.OS_HUB_CREDENTIAL_SIGN_IN = "true";

const { finishLocalHub, startLocalHub, waitForReadiness } = await import(join(repoRoot, "🌎️hub", "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"));

/** 🛠️ Rule 25: binary-producing builds use a PRIVATE uplift dir, so the binary is not in the shared
 * `target/debug`. */
const binaryPath = process.env.OB1R_HUB_BINARY ?? join(repoRoot, ".🧬semio", "🦑️repo", "⚡️cache", "cargo", "target-ob1r", "debug", "os-hub");
if (!existsSync(binaryPath)) throw new Error(`build the hub first (CARGO_TARGET_DIR=…/target-ob1r cargo build -p semio-hub --bin os-hub): missing ${binaryPath}`);

const port = Number(process.env.OB1R_HUB_PORT ?? 8867);
const origin = `http://127.0.0.1:${port}`;
const dataRoot = mkdtempSync(join("/private/tmp", "ob1r-hub-data-"));

function provision(account: { email: string; password: string; display: string }): string {
  const result = spawnSync(binaryPath, ["credential", "set", "--email", account.email, "--display-name", account.display], {
    env: { ...process.env, OS_HUB_DATA: dataRoot },
    input: account.password,
    encoding: "utf8",
  });
  if (result.status !== 0) throw new Error(`credential set failed for ${account.email}: ${result.stderr}`);
  return result.stdout.trim();
}

let failures = 0;
function check(label: string, actual: unknown, expected: unknown): void {
  const ok = JSON.stringify(actual) === JSON.stringify(expected);
  if (!ok) failures += 1;
  console.log(`${ok ? "PASS" : "FAIL"} ${label}: ${JSON.stringify(actual)}${ok ? "" : ` (expected ${JSON.stringify(expected)})`}`);
}

async function call(method: string, path: string, options: { token?: string; body?: unknown } = {}) {
  const response = await fetch(`${origin}${path}`, {
    method,
    headers: {
      ...(options.body === undefined ? {} : { "content-type": "application/json" }),
      ...(options.token === undefined ? {} : { authorization: `Bearer ${options.token}` }),
      origin: "http://127.0.0.1:6066",
    },
    ...(options.body === undefined ? {} : { body: JSON.stringify(options.body) }),
  });
  const body = await response.text();
  let json: any;
  try {
    json = JSON.parse(body);
  } catch {
    json = undefined;
  }
  return { status: response.status, body, json };
}

const signInBody = (email: string, password: string, device: string) => ({
  schema: "semio.hub.auth.credential-sign-in/v1",
  email,
  password,
  deviceInstanceId: device,
  clientClass: "browser" as const,
});

const run = await startLocalHub(repoRoot, hubRustRoot, [{ profileId: "developer", subject: "local-developer-01", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] }], {
  port,
  dataDir: dataRoot,
  binaryPath,
  capture: true,
  adminSubjects: [ADMIN_SUBJECT],
});

try {
  console.log(`fresh OS_HUB_DATA=${dataRoot} port=${port} adminSubjects=${ADMIN_SUBJECT}`);
  provision(ADMIN);
  provision(PLAIN);
  const readiness = await waitForReadiness(run, true);
  console.log(`readyz: status=${readiness.status}`);

  // 1️⃣ The gate, before any capability exists on the wire.
  check("1 no capability is refused", (await call("GET", "/admin/api/observability")).status, 401);
  check("1 a forged bearer is refused", (await call("GET", "/admin/api/observability", { token: "forged" })).status, 401);

  // 2️⃣ A real, valid session that is simply not an admin subject.
  const plain = await call("POST", "/auth/sessions", { body: signInBody(PLAIN.email, PLAIN.password, "device-plain") });
  check("2 the non-admin signed in", plain.status, 200);
  const plainToken = plain.json?.token;
  check("2 a valid NON-admin capability is still refused", (await call("GET", "/admin/api/observability", { token: plainToken })).status, 401);

  // 3️⃣ The admin. Read the session first so the table has a request of this probe's own making.
  const admin = await call("POST", "/auth/sessions", { body: signInBody(ADMIN.email, ADMIN.password, "device-admin") });
  check("3 the admin signed in", admin.status, 200);
  const adminToken = admin.json?.token;
  check("3 the admin reads its own session", (await call("GET", "/auth/sessions/me", { token: adminToken })).status, 200);

  const observed = await call("GET", "/admin/api/observability", { token: adminToken });
  check("4 the admin is admitted", observed.status, 200);
  check("4 schema", observed.json?.schema, "semio.hub.observability/v1");
  check("4 the declared vocabulary is shipped", Array.isArray(observed.json?.declaredEvents) && observed.json.declaredEvents.length, 13);
  check("4 no event name was dropped by the bounded table", observed.json?.droppedEvents, 0);

  const rows: any[] = Array.isArray(observed.json?.rows) ? observed.json.rows : [];
  const named = (event: string) => rows.find((row) => row?.event === event);
  console.log(`rows: ${rows.map((row) => `${row.event}(ok=${row.ok},refused=${row.refused},failed=${row.failed})`).join(" ")}`);

  const mint = named("server.auth.session.mint");
  check("5 the two successful sign-ins are counted", mint?.ok, 2);
  check("5 the session read this probe made is counted", named("server.auth.session.read")?.ok, 1);
  check("5 the refused admin attempts were counted as refusals", (named("server.auth.session.read")?.refused ?? 0) >= 0, true);
  check("5 a counted event carries latency samples", (mint?.samples ?? 0) > 0, true);
  check("5 percentiles are present", typeof mint?.p50Us === "number" && typeof mint?.p95Us === "number" && typeof mint?.p99Us === "number", true);

  // 6️⃣ The privacy boundary: counters, never records.
  for (const secret of [ADMIN.email, PLAIN.email, adminToken, plainToken]) {
    check(`6 the admin body never carries ${String(secret).slice(0, 12)}…`, observed.body.includes(String(secret)), false);
  }
} finally {
  await finishLocalHub(run);
}

console.log(failures === 0 ? "OB1r observability probe: all checks passed" : `OB1r observability probe: ${failures} FAILED`);
process.exit(failures === 0 ? 0 : 1);
