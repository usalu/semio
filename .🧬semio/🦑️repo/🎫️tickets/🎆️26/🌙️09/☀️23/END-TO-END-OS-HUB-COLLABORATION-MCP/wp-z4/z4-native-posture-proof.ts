/** 🧪️ Z4 one-off: the docker-image check's production-posture drill against a NATIVE os-hub (no image build allowed yet):
 * fresh data root + catalog copy + `credential set` (stdin), hub on loopback in production mode with origin allowlist and
 * OS_HUB_TRUSTED_FORWARDING=proxy, reached through the harness's forwarding proxy; /readyz, /healthz through and around
 * the proxy, the two-client relay smoke, SIGTERM drain. usage: bun z4-native-posture-proof.ts <os-hub> <catalog root> <data root> <port> [kind] */
import { spawn, spawnSync } from "node:child_process";
import { randomBytes } from "node:crypto";
import { mkdirSync, rmSync } from "node:fs";
import { hubForwardingProxy, hubProbeCall, hubSeedTrustedCatalog } from "../../../../../../../../🌎️hub/🤝️integration-harness/🟦️.ts";
import { runHubTwoClientSmoke } from "../../../../../../../../🌎️hub/🧪️tests/🐳️docker-image/🟦️.ts";

const [binary, catalogRoot, dataRoot, portText, kind = "note"] = process.argv.slice(2);
const port = Number(portText);
const email = "z4-posture@semio.dev";
const password = randomBytes(18).toString("hex");
rmSync(dataRoot!, { recursive: true, force: true });
mkdirSync(dataRoot!, { recursive: true, mode: 0o700 });
const generation = hubSeedTrustedCatalog(catalogRoot!, dataRoot!);
console.log(`[z4] seeded ${generation.slice(0, 12)} into ${dataRoot}`);
const seeded = spawnSync(binary!, ["credential", "set", "--email", email, "--display-name", "Z4 Posture"], { env: { ...process.env, OS_HUB_DATA: dataRoot }, input: `${password}\n`, encoding: "utf8" });
console.log(`[z4] credential set exit ${seeded.status} ${seeded.stderr.trim().slice(-200)}`);
if (seeded.status !== 0) process.exit(1);
const env = { ...process.env, OS_HUB_DATA: dataRoot, OS_HUB_BIND: "127.0.0.1", OS_HUB_PORT: String(port), OS_HUB_MODE: "production", OS_HUB_CREDENTIAL_SIGN_IN: "true", OS_HUB_STORAGE_BACKEND: "fs", OS_HUB_DIRECTORY_BACKEND: "sqlite", OS_HUB_ADMIN_SUBJECTS: `credential.password.v1:${email}`, OS_HUB_ALLOWED_ORIGINS: "https://s.example.com", OS_HUB_TRUSTED_FORWARDING: "proxy" };
const started = Date.now();
const hub = spawn(binary!, [], { env, stdio: ["ignore", "ignore", "pipe"] });
let stderr = "";
hub.stderr!.on("data", (chunk: Buffer) => { stderr += chunk.toString("utf8"); if (stderr.length > 200_000) stderr = stderr.slice(-100_000); });
let exitCode: number | null = null;
const exited = new Promise<void>((resolve) => hub.once("exit", (code) => { exitCode = code; resolve(); }));
const proxy = hubForwardingProxy(`http://127.0.0.1:${port}`);
console.log(`[z4] hub pid ${hub.pid} on 127.0.0.1:${port}; proxy ${proxy.origin}`);
const controller = new AbortController();
try {
  let last = "";
  let readyMs = -1;
  while (Date.now() - started < 600_000 && exitCode === null) {
    const ready = await hubProbeCall(proxy.origin, "GET", "/readyz").catch((error: unknown) => ({ status: 0, text: String(error) }) as any);
    const line = `${ready.status} ${String(ready.text).slice(0, 160)}`;
    if (line !== last) console.log(`[z4] /readyz ${Date.now() - started} ms: ${line}`);
    last = line;
    if (ready.status === 200) { readyMs = Date.now() - started; break; }
    await new Promise((resolve) => setTimeout(resolve, 1_000));
  }
  if (readyMs < 0) throw new Error(`not ready; exit ${exitCode}; ${stderr.slice(-600)}`);
  const healthz = await hubProbeCall(proxy.origin, "GET", "/healthz");
  const direct = await fetch(`http://127.0.0.1:${port}/healthz`);
  console.log(`[z4] ready ${readyMs} ms; /healthz via proxy ${healthz.status}; direct ${direct.status} ${direct.headers.get("x-semio-refusal")}`);
  const smoke = await runHubTwoClientSmoke({ origin: proxy.origin, email, password, kind, signal: controller.signal, onProgress: (line) => console.log(`[z4] smoke: ${line}`) });
  console.log(`[z4] SMOKE ${JSON.stringify(smoke)}`);
  console.log(`[z4] proxy served ${proxy.requests()} requests`);
} catch (error) {
  console.log(`[z4] FAILED ${error instanceof Error ? error.message : String(error)}`);
  process.exitCode = 1;
} finally {
  proxy.stop();
  const stopping = Date.now();
  if (exitCode === null) hub.kill("SIGTERM");
  await Promise.race([exited, new Promise((resolve) => setTimeout(resolve, 30_000))]);
  if (exitCode === null) hub.kill("SIGKILL");
  console.log(`[z4] SIGTERM → exit ${exitCode} in ${Date.now() - stopping} ms`);
  console.log(`[z4] hub stderr tail: ${stderr.split("\n").filter((line) => /shutdown|refus|error|panic/iu.test(line)).slice(-6).join(" | ").slice(0, 1200)}`);
}
