#!/usr/bin/env bun
/** 🕰️ H14 14c one-off: where a hub's boot time goes. Seeds `<root>` (APFS clone) with a published catalog, provisions a probe
 * credential (random, never printed), then boots `<binary>` `boots` times on it (first cold, rest warm): every stdout/stderr
 * line is stamped with ms since spawn, `/readyz` is read every 100 ms until 200, then admin observability `catalog` every
 * 250 ms until every package settled; every per-package phase change is stamped. Output: `<out>-<boot>.log` (stamped lines)
 * and `<out>-<boot>.json` (transitions + totals).
 *   bun h14-boot-timeline.ts <catalog-root> <binary> <root> <port> <boots> <out-prefix> [residency-bytes] */
import { randomBytes } from "node:crypto";
import { spawn, spawnSync } from "node:child_process";
import { existsSync, readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const [catalogRoot, binary, root, portText, bootsText, out, residency] = process.argv.slice(2);
const port = Number(portText);
const email = "boot-timeline@semio.dev";
const password = randomBytes(18).toString("hex");
const origin = `http://127.0.0.1:${port}`;
if (!existsSync(root!)) {
  const generation = String(JSON.parse(readFileSync(join(catalogRoot!, "trusted-catalog", "current.json"), "utf8")).generationId);
  mkdirSync(join(root!, "trusted-catalog", "generations"), { recursive: true, mode: 0o700 });
  const cloned = spawnSync("cp", ["-Rc", join(catalogRoot!, "trusted-catalog", "generations", generation), join(root!, "trusted-catalog", "generations")], { encoding: "utf8" });
  if (cloned.status !== 0) throw new Error(`clone ${cloned.stderr}`);
  spawnSync("cp", ["-p", join(catalogRoot!, "trusted-catalog", "current.json"), join(root!, "trusted-catalog", "current.json")]);
}
const provisioned = spawnSync(binary!, ["credential", "set", "--email", email, "--display-name", "Boot Timeline"], { env: { ...process.env, OS_HUB_DATA: root }, input: password, encoding: "utf8" });
if (provisioned.status !== 0) throw new Error(`credential ${provisioned.stderr}`);

const pause = (ms: number) => new Promise((resolveDelay) => setTimeout(resolveDelay, ms));
async function call(path: string, token?: string, body?: string): Promise<{ status: number; json: any }> {
  try {
    const response = await fetch(`${origin}${path}`, { method: body === undefined ? "GET" : "POST", headers: { ...(token ? { authorization: `Bearer ${token}` } : {}), ...(body ? { "content-type": "application/json" } : {}) }, ...(body === undefined ? {} : { body }), signal: AbortSignal.timeout(30_000) });
    const text = await response.text();
    let json: any = null;
    try { json = JSON.parse(text); } catch { json = null; }
    return { status: response.status, json };
  } catch {
    return { status: 0, json: null };
  }
}

for (let boot = 0; boot < Number(bootsText); boot += 1) {
  const lines: string[] = [];
  const transitions: { atMs: number; pluginId: string; phase: string; rowsPinned: number; rowsVerified: number }[] = [];
  const stages: { atMs: number; stage: string }[] = [];
  const phases = new Map<string, string>();
  const env = { ...process.env, OS_HUB_DATA: root, OS_HUB_PORT: String(port), OS_HUB_MODE: "production", OS_HUB_BIND: "127.0.0.1", OS_HUB_CREDENTIAL_SIGN_IN: "true", OS_HUB_ADMIN_SUBJECTS: `credential.password.v1:${email}`, SEMIO_TRACE_LEVEL: "info", ...(residency ? { OS_HUB_GUEST_RESIDENCY_BYTES: residency } : {}) };
  const started = performance.now();
  const at = () => Math.round(performance.now() - started);
  const child = spawn(binary!, [], { env, stdio: ["ignore", "pipe", "pipe"] });
  for (const stream of [child.stdout, child.stderr]) {
    let tail = "";
    stream.on("data", (chunk: Buffer) => {
      const parts = (tail + chunk.toString("utf8")).split("\n");
      tail = parts.pop() ?? "";
      for (const part of parts) lines.push(`${String(at()).padStart(7)} ${part}`);
    });
  }
  const note = (catalog: any) => {
    for (const entry of catalog?.packages ?? []) {
      if (phases.get(entry.pluginId) !== entry.phase) {
        phases.set(entry.pluginId, entry.phase);
        transitions.push({ atMs: at(), pluginId: entry.pluginId, phase: entry.phase, rowsPinned: entry.rowsPinned, rowsVerified: entry.rowsVerified });
      }
    }
  };
  let readyMs = -1;
  let firstAnswerMs = -1;
  let lastStage = "";
  while (at() < 1_800_000) {
    const answer = await call("/readyz");
    if (answer.status !== 0 && firstAnswerMs < 0) firstAnswerMs = at();
    const stage = answer.json?.startup ? `${answer.json.startup.stage} ${answer.json.startup.completedUnits}/${answer.json.startup.totalUnits}` : String(answer.status);
    if (stage.split(" ")[0] !== lastStage) stages.push({ atMs: at(), stage });
    lastStage = stage.split(" ")[0]!;
    note(answer.json?.startup?.catalog);
    if (answer.status === 200) { readyMs = at(); break; }
    await pause(100);
  }
  const signIn = await call("/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: `h14tl${randomBytes(12).toString("hex")}`, clientClass: "browser" }));
  const token = String(signIn.json?.token ?? "");
  let verifiedMs = -1;
  let catalog: any = null;
  while (at() < 1_800_000) {
    const answer = await call("/admin/api/observability", token);
    catalog = answer.json?.catalog;
    note(catalog);
    if (catalog && catalog.packagesReady + catalog.packagesRefused === catalog.packagesTotal) { verifiedMs = at(); break; }
    await pause(250);
  }
  const stopAt = performance.now();
  child.kill("SIGTERM");
  await new Promise((resolveExit) => child.once("exit", resolveExit));
  const sigtermToExitMs = Math.round(performance.now() - stopAt);
  await pause(300);
  writeFileSync(`${out}-${boot}.log`, `${lines.join("\n")}\n`);
  const summary = { boot, kind: boot === 0 ? "cold" : "warm", firstAnswerMs, readyMs, verifiedMs, sigtermToExitMs, packagesTotal: catalog?.packagesTotal, packagesRefused: catalog?.packagesRefused, rowsTotal: catalog?.rowsTotal, rowsPinned: catalog?.rowsPinned, rowsVerified: catalog?.rowsVerified, componentBytesTotal: catalog?.componentBytesTotal, stages, transitions, packages: catalog?.packages };
  writeFileSync(`${out}-${boot}.json`, JSON.stringify(summary, null, 1));
  console.log(`[boot-timeline] boot ${boot} ${summary.kind}: first answer ${firstAnswerMs} ms, ready ${readyMs} ms, settled ${verifiedMs} ms, exit ${sigtermToExitMs} ms, refused ${catalog?.packagesRefused}`);
}
