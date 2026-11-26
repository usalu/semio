/** 🧾️ Pins the published component and actual served cores around both registered Draw journeys. */
import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";
import { rename, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";

const [run, pinned, origin = "http://127.0.0.1:6064/", mode = "run", expectedEmitter] = Bun.argv.slice(2);
if (!run || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(run) || !/^[0-9a-f]{64}$/.test(pinned ?? "") || !["run", "viewer", "check-only"].includes(mode)) throw Error("Expected run-id, exact lowercase component SHA-256, optional origin and run|viewer|check-only");
const base = new URL(origin); if (!/^https?:$/.test(base.protocol) || base.pathname !== "/" || base.search || base.hash) throw Error("Expected a root HTTP(S) origin");
const root = process.cwd(), ticket = resolve(import.meta.dir), generated = join(ticket, "🗑️generated"), modules = join(root, ".cache/b/p/m/browser/prototype/modules/🖍️draw"), models = join(root, ".cache/b/p/m/browser/prototype/components");
if (mode === "viewer" && join(generated, `viewer-${run}-en/report.json`).length > 256) throw Error("Viewer output path exceeds 256 characters");
const output = join(generated, `attestation-${run}.json`), proof: { version: number; pinned: string; origin: string; status: string; checkpoints: unknown[]; journeys: unknown[]; error?: string } = { version: 1, pinned, origin, status: "running", checkpoints: [], journeys: [] };
const save = async () => { const temporary = `${output}.${process.pid}.tmp`; await writeFile(temporary, JSON.stringify(proof, null, 2) + "\n"); await rename(temporary, output); };
const fileHash = async (path: string) => { const hash = createHash("sha256"); for await (const chunk of createReadStream(path)) hash.update(chunk); return hash.digest("hex"); };
async function runRepo(args: string[], environment: Record<string, string> = {}) {
  const child = Bun.spawn(["repo", ...args], { cwd: root, env: { ...process.env, DRAW_NATIVE_BUILD_MODE: "prototype", ...environment }, stdout: "pipe", stderr: "pipe" });
  const stop = () => child.kill("SIGTERM"); process.once("SIGTERM", stop); process.once("SIGINT", stop);
  const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  process.off("SIGTERM", stop); process.off("SIGINT", stop);
  if (stdout) process.stdout.write(stdout); if (stderr) process.stderr.write(stderr);
  if (code !== 0) throw Error(`Registered command failed (${code}): ${args.join(" ")}`);
  return stdout;
}
const tool = ["tools", "exec", "--light", "bun", "--"], launcher = join(ticket, "draw-native-dev.ts");
const dev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts", plugin = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📜️script.ts", draw = "✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts";
async function findModel() {
  const receipt = await Bun.file(join(modules, "🧾️receipt.json")).json();
  if (receipt.wasmSha256 !== pinned) throw Error(`Published browser receipt does not match pinned component ${pinned}`);
  const matches: string[] = [];
  for await (const entry of new Bun.Glob("*/🧾️receipt.json").scan({ cwd: models, type: "file" })) {
    const path = join(models, entry), candidate = await Bun.file(path).json();
    if (candidate.wasmSha256 === pinned && candidate.semanticDescriptorSha256 === receipt.semanticDescriptorSha256) matches.push(resolve(path, ".."));
  }
  if (matches.length !== 1) throw Error(`Expected exactly one model matching pinned receipt; found ${matches.length}`);
  return matches[0]!;
}
async function verifyModel(model: string) {
  const stdout = await runRepo([...tool, join(ticket, "verify-published-draw-receipt.ts"), model, modules, ...(expectedEmitter ? [expectedEmitter] : [])]);
  const row = JSON.parse(stdout.trim().split("\n").at(-1)!);
  if (row.sha !== pinned || await fileHash(join(model, "📦️component.wasm")) !== pinned) throw Error("Component bytes changed from pinned SHA-256");
  return row;
}
async function checkpoint(name: string, model: string, baseline?: string) {
  const verified = await verifyModel(model);
  const url = new URL(["🔌️plugin-modules", "🖍️draw", "🧾️receipt.json"].map(encodeURIComponent).join("/"), base);
  const response = await fetch(url, { cache: "no-store", signal: AbortSignal.timeout(15000) });
  if (!response.ok) throw Error(`Served receipt HTTP ${response.status}`);
  const served = await response.json();
  if (served.wasmSha256 !== pinned || served.semanticDescriptorSha256 !== verified.semantic) throw Error("Served receipt does not identify pinned component and descriptor");
  const lines = await runRepo(["tools", "exec", "--light", "python3", "--", join(ticket, "verify-served-draw-cores-25.py"), origin, modules]);
  const cores = lines.trim().split("\n").filter(Boolean).map(line => { const match = /^(\S+\.wasm) (\d+) ([0-9a-f]{64})$/.exec(line); if (!match) throw Error(`Unexpected core verifier output: ${line}`); return { name: match[1], bytes: Number(match[2]), sha256: match[3], url: new URL(["🔌️plugin-modules", "🖍️draw", match[1]!].map(encodeURIComponent).join("/"), base).toString() }; });
  if (cores.length !== 3 || new Set(cores.map(row => row.name)).size !== 3) throw Error(`Expected exactly three independently verified served cores, got ${cores.length}`);
  const identity = JSON.stringify(cores); if (baseline !== undefined && identity !== baseline) throw Error("Served core identities changed between checkpoints");
  proof.checkpoints.push({ name, at: new Date().toISOString(), componentSha256: pinned, semanticSha256: verified.semantic, receiptUrl: url.toString(), cores }); await save(); return identity;
}
try {
  await save();
  const model = await findModel();
  const initial = await verifyModel(model);
  proof.checkpoints.push({ name: "before-activation", at: new Date().toISOString(), model, componentSha256: pinned, semanticSha256: initial.semantic, ...(initial.emitter ? { emitter: initial.emitter } : {}) }); await save();
  if (mode === "check-only") { proof.status = "verified-publication"; await save(); process.exit(0); }
  if (mode === "run") for (const [stage, target, args] of [["support", plugin, ["support"]], ["prepare", dev, ["prepare", "draw", "react", "prototype"]], ["activate", dev, ["activate", "draw", "react", "prototype"]]] as const) await runRepo([...tool, launcher, generated, `${stage}-${run}`, target, ...args]);
  let baseline: string | undefined;
  for (const locale of ["en", "de"]) {
    baseline = await checkpoint(`${locale}-before`, model, baseline);
    const started = new Date().toISOString();
    const journey = join(generated, `${mode === "viewer" ? "viewer" : "journey"}-${run}-${locale}`);
    if (mode === "viewer") {
      const { repoCacheDirectory } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts"));
      const browsers = process.env.PLAYWRIGHT_BROWSERS_PATH ?? repoCacheDirectory(root, "tools", "ms-playwright");
      await runRepo(["tools", "exec", "--light", "node", "--", join(ticket, "viewer-journey-26.mjs"), origin, locale, journey, pinned, baseline], { PLAYWRIGHT_BROWSERS_PATH: browsers });
      const observed = await Bun.file(join(journey, "report.json")).json();
      if (observed.result !== "passed" || observed.componentSha256 !== pinned || observed.locale !== locale) throw Error("Viewer runtime proof disagrees with the attestation");
    } else {
      await runRepo([...tool, launcher, generated, `browser-${locale}-${run}`, draw, "browser-acceptance", "--serve", origin, "--locale", locale, "--output", journey]);
    }
    proof.journeys.push({ locale, started, completed: new Date().toISOString(), result: "passed", output: journey }); await save();
    await checkpoint(`${locale}-after`, model, baseline);
  }
  proof.status = "passed"; await save();
} catch (error) { proof.status = "failed"; proof.error = String(error); await save(); throw error; }
