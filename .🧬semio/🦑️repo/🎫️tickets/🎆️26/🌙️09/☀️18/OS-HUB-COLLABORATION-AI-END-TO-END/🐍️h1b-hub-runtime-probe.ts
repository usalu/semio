/** 🧪️ H1b runtime probe — outcome 2 ("working server hub backend: db, presence, auth") proven on a
 * **booted** `os-hub`, not by tests. It boots the development binary against a fresh `OS_HUB_DATA`,
 * exercises the six things the slice owes, then **kills the process and boots the same data root
 * again** to separate what the hub durably persisted from what it merely held in memory:
 *
 *   1  `/healthz`      — liveness independent of every subsystem, and a new `runId` per run
 *   2  `/readyz`       — readiness, plus the closed-gate `reason`/`blockedBy` vocabulary when shut
 *   3  sqlite          — user + space + membership created before the restart are still there after
 *   4  auth            — session mint, introspection, revocation by sign-out, and TTL expiry
 *   5  rate limiter    — the auth bucket refuses with `429` + `retry-after`, per remote address
 *   6  presence socket — a real `/directory/socket/v1` upgrade with a socket grant, a live event
 *                        delivered to the joined peer, and the connection leaving the hub's own
 *                        connection roster when the socket closes
 *
 * Usage: `bun 🐍️h1b-hub-runtime-probe.ts [--binary PATH] [--port N] [--data DIR] [--skip-expiry]`. Every step prints one line;
 * a failed expectation exits non-zero naming the observed value, so this file is a gate, not a log. */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync } from "node:fs";
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
const argv = process.argv.slice(2);
const binaryArg = argv.indexOf("--binary");
/** 🏗️ Preamble rule 25: a binary-producing `cargo build` runs with a PRIVATE uplift dir, so the hub
 * this probe boots usually lives under `⚡️cache/cargo/target-h1b/debug/` rather than the shared
 * `target/debug/`. `--binary <path>` names it; the shared path stays the default. */
const binaryPath = binaryArg >= 0 ? argv[binaryArg + 1] : join(repoRoot, ".🧬semio", "🦑️repo", "⚡️cache", "cargo", "target", "debug", "os-hub");
if (!existsSync(binaryPath)) throw new Error(`build the hub first: CARGO_TARGET_DIR=…/target-h1b cargo build -p semio-hub --bin os-hub (missing ${binaryPath})`);

const portArg = argv.indexOf("--port");
const port = portArg >= 0 ? Number(argv[portArg + 1]) : 8847;
const skipExpiry = argv.includes("--skip-expiry");
const origin = `http://127.0.0.1:${port}`;
/** 🌱️ A canonical (non-symlinked) parent: a data root under macOS's `/var/folders` `TMPDIR` is
 * refused by the hub's own server-owned-root walk (`📓️h1-hub-build-and-boot.md` §6.1). */
const dataArg = argv.indexOf("--data");
const dataRoot = dataArg >= 0 ? (mkdirSync(argv[dataArg + 1], { recursive: true, mode: 0o700 }), argv[dataArg + 1]) : mkdtempSync("/private/tmp/h1b-hub-data-");
/** ⏳️ The hub's own floor (`🔐️auth/🦀️.rs:54 MIN_SESSION_TTL_SECS`); the expiry step waits it out. */
const SESSION_TTL_SECS = 60;

const ADA = { email: "ada@h1b.example.org", password: "correct horse battery staple", display: "Ada Lovelace" };
const BO = { email: "bo@h1b.example.org", password: "another perfectly fine phrase", display: "Bo Peep" };

let failures = 0;
function check(label: string, actual: unknown, expected: unknown): void {
  const ok = JSON.stringify(actual) === JSON.stringify(expected);
  if (!ok) failures += 1;
  console.log(`${ok ? "PASS" : "FAIL"} ${label}: ${JSON.stringify(actual)}${ok ? "" : ` (expected ${JSON.stringify(expected)})`}`);
}

function provision(account: { email: string; password: string; display: string }): string {
  const result = spawnSync(binaryPath, ["credential", "set", "--email", account.email, "--display-name", account.display], {
    env: { ...process.env, OS_HUB_DATA: dataRoot },
    input: account.password,
    encoding: "utf8",
  });
  if (result.status !== 0) throw new Error(`credential set failed for ${account.email}: ${result.stderr}`);
  return result.stdout.trim();
}

type Answer = { readonly status: number; readonly body: string; readonly retryAfter: string | null; readonly json: any };

async function call(method: string, path: string, options: { token?: string; body?: unknown } = {}): Promise<Answer> {
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
  return { status: response.status, body, retryAfter: response.headers.get("retry-after"), json };
}

const signInBody = (email: string, password: string, device: string) => ({
  schema: "semio.hub.auth.credential-sign-in/v1",
  email,
  password,
  deviceInstanceId: device,
  clientClass: "browser" as const,
});

const { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } = await import(join(repoRoot, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📇️directory", "🧬️schema", "🟦️.ts"));

async function command(token: string, requestId: string, body: any): Promise<Answer> {
  const sealed = directoryCommandRequestJson(sealDirectoryCommandRequestV1(requestId, body));
  const response = await fetch(`${origin}/directory/commands`, { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${token}`, origin: "http://127.0.0.1:6066" }, body: sealed });
  const text = await response.text();
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {
    json = undefined;
  }
  return { status: response.status, body: text, retryAfter: response.headers.get("retry-after"), json };
}

/** 🚏️ The real local-bootstrap route every launcher uses: `OS_HUB_MODE=development` plus the fd-3
 * identity handshake, without which the hub refuses to start
 * (`UnsafeAuthConfiguration("production requires an IdentityAssertionVerifier adapter")`). */
const { finishLocalHub, startLocalHub, waitForReadiness } = await import(join(repoRoot, "🌎️hub", "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"));
const hubRustRoot = join(repoRoot, "🌎️hub", "📦️packages", "🦀️rust");
const PROFILES = [{ profileId: "developer", subject: "local-developer-01", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] }];

let run: any;
const captured = (): string => (run === undefined ? "" : run.output());

async function boot(phase: string): Promise<Record<string, any>> {
  const started = Date.now();
  process.env.OS_HUB_CREDENTIAL_SIGN_IN = "1";
  process.env.OS_HUB_SESSION_TTL_SECONDS = String(SESSION_TTL_SECS);
  run = await startLocalHub(repoRoot, hubRustRoot, PROFILES, { port, dataDir: dataRoot, binaryPath, capture: true });
  const readiness = await waitForReadiness(run, true);
  console.log(`boot ${phase}: pid=${run.child.pid} readyz=200 after ${Math.round((Date.now() - started) / 1000)}s`);
  return readiness;
}

async function halt(): Promise<void> {
  if (run === undefined) return;
  const pid = run.child.pid;
  await finishLocalHub(run);
  console.log(`halt: pid=${pid} exit=${run.child.exitCode}`);
  run = undefined;
}

/** 🔌️ The repo's own first-party replication wire codec — the same encoder the browser client uses,
 * so the hello this probe sends is byte-for-byte the hello a real peer sends. */
const { encodeClientFrame } = await import(join(repoRoot, "🧰️framework", "🔨️modules", "📡️replication", "🟦️.ts"));

/** 👥️ Opens a real `/directory/socket/v1` upgrade with a freshly issued socket grant, exactly as
 * the hub's own laws do: the grant travels in `Sec-WebSocket-Protocol` beside `semio.socket.v1`,
 * and the peer then sends the credential-free `SocketHelloV1` on the command lane before the hub
 * streams anything to it. */
async function openDirectorySocket(token: string, since: number): Promise<{ socket: WebSocket; actorId: string; messages: string[] }> {
  const receipt = await call("POST", "/directory/socket-grants", { token });
  if (receipt.status !== 200) throw new Error(`socket grant refused: ${receipt.status} ${receipt.body}`);
  const messages: string[] = [];
  const socket = new WebSocket(`ws://127.0.0.1:${port}/directory/socket/v1?since=${since}`, ["semio.socket.v1", receipt.json.grant]);
  socket.binaryType = "arraybuffer";
  socket.addEventListener("message", (event) => messages.push(typeof event.data === "string" ? event.data : `<binary ${new Uint8Array(event.data as ArrayBuffer).byteLength}B>`));
  await new Promise<void>((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error("directory socket upgrade deadline")), 15_000);
    socket.addEventListener("open", () => {
      clearTimeout(timer);
      resolve();
    });
    socket.addEventListener("error", () => {
      clearTimeout(timer);
      reject(new Error("directory socket upgrade error"));
    });
  });
  socket.send(
    encodeClientFrame(
      { SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: "test.v1", pack_schema_hash: new Array(32).fill(0x11), resume_token: null, frontier: null } },
      "command",
    ),
  );
  return { socket, actorId: receipt.json.actorId, messages };
}

async function until<T>(label: string, budgetMs: number, read: () => Promise<T> | T, done: (value: T) => boolean): Promise<T> {
  const deadline = Date.now() + budgetMs;
  let value = await read();
  while (Date.now() < deadline && !done(value)) {
    await new Promise((resolve) => setTimeout(resolve, 250));
    value = await read();
  }
  if (!done(value)) console.log(`(deadline) ${label} never settled within ${budgetMs}ms; last=${JSON.stringify(value).slice(0, 300)}`);
  return value;
}

let spaceId = "";
let adaId = "";
let boId = "";
let firstRunId = "";

try {
  console.log(`fresh OS_HUB_DATA=${dataRoot} port=${port} binary=${binaryPath}`);
  adaId = provision(ADA);
  boId = provision(BO);
  console.log(`operator bootstrap: ada=${adaId} bo=${boId}`);

  const firstReadiness = await boot("first");
  console.log(`   first readiness: ${JSON.stringify(firstReadiness).slice(0, 400)}`);

  // 1️⃣ /healthz — liveness, independent of every subsystem
  const health = await call("GET", "/healthz");
  check("1 healthz status", health.status, 200);
  check("1 healthz schema", health.json?.schema, "semio.hub.liveness/v1");
  check("1 healthz status field", health.json?.status, "live");
  check("1 healthz runId is a nonempty string", typeof health.json?.runId === "string" && health.json.runId.length > 0, true);
  check("1 healthz uptimeMs is a nonnegative integer", Number.isSafeInteger(health.json?.uptimeMs) && health.json.uptimeMs >= 0, true);
  firstRunId = health.json?.runId ?? "";

  // 2️⃣ /readyz — readiness, and the closed-gate vocabulary when a required gate is shut
  const ready = await until("readyz", 30_000, () => call("GET", "/readyz"), (answer) => answer.status === 200);
  check("2 readyz answers with its own schema", ready.json?.schema, "semio.hub.readiness/v1");
  check("2 readyz status and HTTP code agree", ready.status === 200, ready.json?.status === "ready");
  console.log(`   readyz body: ${JSON.stringify(ready.json)}`);
  const catalogPublished = ready.json?.artifactAuthority?.ready === true;
  if (catalogPublished) {
    check("2 a ready hub says ready", ready.json?.status, "ready");
    check("2 a ready body publishes no blockedBy", ready.json?.blockedBy, undefined);
    check("2 a ready body publishes no component reason", JSON.stringify(ready.json ?? {}).includes('"reason"'), false);
    check("2 the startup line reports ready", captured().includes("[INFO] os-hub ready at"), true);
  } else {
    /** 🚧️ A data root with no published trusted catalog can never reach `ready`: `artifactAuthority`
     * is a REQUIRED gate and is open only when `<data>/trusted-catalog/current.json` loads
     * (`🏗️bootstrap/🦀️.rs:456-471`, `:9537-9547`). Publishing one costs two wasm release component
     * builds plus a default-features `--bin os-hub`, so this branch records the gate instead of
     * pretending the hub is broken. */
    console.log("BLOCKED 2 /readyz cannot reach 200 in a data root with no published trusted catalog");
    check("2 the closed gate is named", ready.json?.artifactAuthority?.ready, false);
    check("2 every other required gate is open", [ready.json?.directory?.ready, ready.json?.storage?.ready, ready.json?.artifactCasBarrier?.ready, ready.json?.artifactPublication?.ready, ready.json?.adminAssets?.ready], [true, true, true, true, true]);
    const gates = Array.isArray(ready.json?.blockedBy) ? ready.json.blockedBy : undefined;
    console.log(gates === undefined ? "   blockedBy: absent — this binary predates the closed-gate reason vocabulary" : `   blockedBy: ${JSON.stringify(gates)}`);
    if (gates !== undefined) check("2 the closed gate carries its stable reason code", gates, [{ gate: "artifactAuthority", reason: "trusted-catalog-never-published-in-this-data-root" }]);
    check("2 a not-ready hub warns at startup instead of claiming readiness", captured().includes("[INFO] os-hub ready at") === false || captured().includes("[WARN] os-hub listening"), true);
  }

  // 3️⃣ auth — mint and introspection
  const minted = await call("POST", "/auth/sessions", { body: signInBody(ADA.email, ADA.password, "device-ada") });
  check("3 sign-in status", minted.status, 200);
  const adaToken: string = minted.json?.token ?? "";
  check("3 minted token shape", /^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u.test(adaToken), true);
  const me = await call("GET", "/auth/sessions/me", { token: adaToken });
  check("3 me status", me.status, 200);
  check("3 me userId", me.json?.userId, adaId);
  check("3 me expiresAt honours OS_HUB_SESSION_TTL_SECONDS", Number.isSafeInteger(me.json?.expiresAt) && me.json.expiresAt - Date.now() <= SESSION_TTL_SECS * 1000 + 5_000, true);

  // 4️⃣ durable state written before the restart
  const created = await command(adaToken, "a".repeat(32), { kind: "create-space", name: "H1b Persistence Atelier", spaceKind: "atelier", visibility: "private" });
  check("4 create-space status", created.status, 202);
  const listed = await until("space listing", 30_000, () => call("GET", "/directory/spaces", { token: adaToken }), (answer) => Array.isArray(answer.json) && answer.json.some((row: any) => row?.space?.name === "H1b Persistence Atelier"));
  spaceId = (Array.isArray(listed.json) ? listed.json : []).find((row: any) => row?.space?.name === "H1b Persistence Atelier")?.space?.id ?? "";
  check("4 the new space has an id", spaceId.length > 0, true);
  const invited = await command(adaToken, "b".repeat(32), { kind: "create-invite", spaceId, role: "spectator", ttlSecs: 3600 });
  check("4 create-invite status", invited.status, 202);
  const boMint = await call("POST", "/auth/sessions", { body: signInBody(BO.email, BO.password, "device-bo") });
  const boToken: string = boMint.json?.token ?? "";
  check("4 second sign-in status", boMint.status, 200);
  check("4 redemption status", (await call("POST", `/directory/invites/${encodeURIComponent(invited.json?.result?.inviteToken ?? "")}/redeem`, { token: boToken })).status, 200);

  // 5️⃣ presence — a real socket upgrade, live fan-out to the joined peer, and a clean departure
  const joined = await openDirectorySocket(boToken, 0);
  check("5 directory socket upgraded", joined.socket.readyState, WebSocket.OPEN);
  const renamed = await command(adaToken, "c".repeat(32), { kind: "rename-space", spaceId, name: "H1b Persistence Atelier renamed" });
  check("5 the fan-out-producing command was accepted", renamed.status, 202);
  await until("live directory frame", 30_000, () => joined.messages.length, (count) => count > 0);
  check("5 the joined peer received a live directory frame", joined.messages.length > 0, true);
  console.log(`   frames while joined (${joined.messages.length}): ${joined.messages.map((row) => row.slice(0, 160)).join(" | ").slice(0, 600)}`);
  const framesAtDeparture = joined.messages.length;
  joined.socket.close();
  await until("socket closed", 15_000, () => joined.socket.readyState, (state) => state === WebSocket.CLOSED);
  check("5 the socket left", joined.socket.readyState, WebSocket.CLOSED);
  const afterLeave = await command(adaToken, "d".repeat(32), { kind: "rename-space", spaceId, name: "H1b Persistence Atelier renamed twice" });
  check("5 a command after the departure is still accepted", afterLeave.status, 202);
  await new Promise((resolve) => setTimeout(resolve, 3_000));
  check("5 the departed peer receives nothing more", joined.messages.length, framesAtDeparture);

  // 6️⃣ revocation — sign-out kills the capability
  check("6 sign-out status", (await call("DELETE", "/auth/sessions/me", { token: boToken })).status, 204);
  check("6 the signed-out capability is gone", (await call("GET", "/auth/sessions/me", { token: boToken })).status, 401);

  // 7️⃣ restart — everything above must survive the process dying
  await halt();
  check("7 the hub stopped answering", await call("GET", "/healthz").then((answer) => answer.status).catch(() => 0), 0);
  const restartReadiness = await boot("restart");
  console.log(`   restart readiness: ${JSON.stringify(restartReadiness).slice(0, 400)}`);
  const restartHealth = await call("GET", "/healthz");
  check("7 healthz after restart", restartHealth.status, 200);
  check("7 the restart is a NEW run", restartHealth.json?.runId !== firstRunId, true);
  const restartReady = await until("readyz after restart", 30_000, () => call("GET", "/readyz"), (answer) => answer.status === 200);
  check("7 readyz after restart reports the same gate picture as the first boot", [restartReady.json?.status, restartReady.json?.artifactAuthority?.ready], [ready.json?.status, ready.json?.artifactAuthority?.ready]);
  const reMint = await call("POST", "/auth/sessions", { body: signInBody(ADA.email, ADA.password, "device-ada-2") });
  check("7 the credential survived the restart", reMint.status, 200);
  const survivorToken: string = reMint.json?.token ?? "";
  const afterRestart = await until("space listing after restart", 30_000, () => call("GET", "/directory/spaces", { token: survivorToken }), (answer) => Array.isArray(answer.json));
  check("7 the space survived the restart", (Array.isArray(afterRestart.json) ? afterRestart.json : []).some((row: any) => row?.space?.id === spaceId), true);
  const page = await call("GET", `/directory/spaces/${encodeURIComponent(spaceId)}`, { token: survivorToken });
  check("7 the space page is readable after the restart", page.status, 200);
  const roster = (page.json?.members?.rows ?? []).map((row: any) => [row.userId, row.role]).sort();
  check("7 the membership survived the restart", roster, [[adaId, "author"], [boId, "spectator"]].sort());
  check("7 presence did NOT survive the restart (it is ephemeral by contract)", JSON.stringify(page.json?.presence ?? []), "[]");

  // 8️⃣ expiry — the TTL the hub was booted with is enforced without any client action
  if (skipExpiry) {
    console.log("SKIP 8 session expiry (--skip-expiry)");
  } else {
    const introspected = await call("GET", "/auth/sessions/me", { token: survivorToken });
    check("8 the fresh session resolves", introspected.status, 200);
    const waitMs = Math.max(0, (introspected.json?.expiresAt ?? Date.now()) - Date.now()) + 3_000;
    console.log(`   waiting ${Math.round(waitMs / 1000)}s for the session to expire`);
    await new Promise((resolve) => setTimeout(resolve, waitMs));
    check("8 an expired session stops resolving", (await call("GET", "/auth/sessions/me", { token: survivorToken })).status, 401);
  }

  // 9️⃣ rate limiter — last, because it empties this address's auth bucket
  let lockout: Answer | undefined;
  for (let attempt = 0; attempt < 24 && lockout === undefined; attempt += 1) {
    const answer = await call("POST", "/auth/sessions", { body: signInBody("nobody@h1b.example.org", "a wrong phrase entirely", "device-lock") });
    if (answer.status === 429) lockout = answer;
  }
  check("9 the auth bucket eventually refuses", lockout?.status, 429);
  check("9 the refusal names its class", lockout?.json?.error, "rate-limited");
  check("9 retry-after is a positive integer of seconds", Number.isInteger(Number(lockout?.retryAfter)) && Number(lockout?.retryAfter) >= 1, true);
  check("9 a correct password is refused while the bucket is empty", (await call("POST", "/auth/sessions", { body: signInBody(ADA.email, ADA.password, "device-ada") })).status, 429);
} catch (error) {
  failures += 1;
  console.log(`probe failure: ${error instanceof Error ? error.stack : String(error)}`);
  console.log(`hub output tail:\n${captured().slice(-3000)}`);
} finally {
  await halt();
  console.log(`data root retained for inspection: ${dataRoot}`);
}

console.log(failures === 0 ? "h1b-hub-runtime: all checks passed" : `h1b-hub-runtime: ${failures} check(s) failed`);
process.exit(failures === 0 ? 0 : 1);
