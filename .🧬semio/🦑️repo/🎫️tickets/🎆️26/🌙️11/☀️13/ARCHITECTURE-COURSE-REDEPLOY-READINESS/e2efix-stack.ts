/** 🧱️ Graph-free native run of the architecture-course Playwright e2e gate: boots the proctor (port 8791, temporary data), the control endpoint the proctor-away and proctor-faults specs use, and the site (dev topology: vite dev server on 6061 proxying to the proctor; rehearsal topology: release build served statically on 6061 calling the proctor cross-origin), runs the boot spec as warm-up, then `playwright test`, and stops everything.
 *
 * Usage: `node native-stack.ts <dev|rehearsal> <shared|exclusive|all|PROJECT,...> [playwright args…]`
 * Environment: `PDF_DIRECTORY` (assignment PDFs), `WORKERS` (shared shard, default 3), `RUN_DIRECTORY`, `KEEP=1` (boot the stack and keep it for manual debugging until SIGTERM) (default `🗑️generated/e2efix/run`), `PROCTOR_BINARY`, `SKIP_WARMUP=1`.
 * @see ../../../../../../../🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🟦️.ts — the gate this mirrors */
import { spawn, type ChildProcess } from "node:child_process";
import { randomBytes } from "node:crypto";
import { existsSync, mkdirSync, openSync, readFileSync, rmSync, statSync } from "node:fs";
import { createServer } from "node:http";
import { homedir } from "node:os";
import { extname, join, resolve } from "node:path";
import { freshProctor } from "./fresh-proctor.mjs";

const here = import.meta.dirname;
const repo = resolve(here, "../../../../../../..");
const site = join(repo, "🎓️teaching/🏛️architecture/📚️course");
const [topology = "dev", selection = "all", ...forwarded] = process.argv.slice(2);
const run = resolve(process.env.RUN_DIRECTORY ?? join(here, "🗑️generated/e2efix/run"));
const node = process.execPath;
const playwright = join(repo, ".cache/b/d/m/j/b87aa6c238609ac86876/node_modules/@playwright/test/cli.js");
const proctorBinary = process.env.PROCTOR_BINARY ?? freshProctor(process.env.PROCTOR_CONFIGURATION ?? "prototype");
const pdfs = process.env.PDF_DIRECTORY ?? "/private/tmp/claude-501/-Users-ueli-Documents-semio/9d0eceba-b1b8-4814-b9cd-caae5bbd19aa/scratchpad/pdfs/out";
const SITE_PORT = 6070;
const PROCTOR_PORT = 8801;
const siteOrigin = `http://127.0.0.1:${SITE_PORT}`;
const proctorOrigin = `http://127.0.0.1:${PROCTOR_PORT}`;
const projects: Readonly<Record<string, string[]>> = { shared: ["desktop", "phone", "layout", "pets"], exclusive: ["presence", "shortage", "away", "faults"], all: ["desktop", "phone", "layout", "pets", "presence", "shortage", "away", "faults"] };
const chosen = projects[selection] ?? selection.split(",");
const owned: ChildProcess[] = [];
const say = (line: string): void => console.log(`[native-stack] ${line}`);
const pause = (ms: number): Promise<void> => new Promise((accept) => setTimeout(accept, ms));

async function answers(url: string): Promise<boolean> {
  try {
    const answer = await fetch(url, { signal: AbortSignal.timeout(15_000) });
    await answer.arrayBuffer();
    return answer.ok;
  } catch {
    return false;
  }
}

async function awaitUp(label: string, url: string, child?: ChildProcess): Promise<void> {
  for (let waited = 0; waited < 240_000; waited += 500) {
    if (child && child.exitCode !== null) throw Error(`${label} exited with ${child.exitCode}`);
    if (await answers(url)) return say(`${label} ready after ${waited} ms`);
    await pause(500);
  }
  throw Error(`${label} not ready`);
}

function launch(command: string, args: string[], env: NodeJS.ProcessEnv, log: string, cwd = repo): ChildProcess {
  const sink = openSync(log, "a");
  const child = spawn(command, args, { cwd, env, stdio: ["ignore", sink, sink] });
  owned.push(child);
  return child;
}

function end(child: ChildProcess): Promise<void> {
  return new Promise((accept) => {
    if (child.exitCode !== null || child.signalCode !== null) return accept();
    child.once("close", () => accept());
    child.kill("SIGTERM");
    setTimeout(() => child.kill("SIGKILL"), 5_000).unref();
  });
}

mkdirSync(run, { recursive: true });
const data = join(run, "proctor-data");
rmSync(data, { recursive: true, force: true });
mkdirSync(data, { recursive: true });
const proctorEnv: NodeJS.ProcessEnv = { ...Object.fromEntries(Object.entries(process.env).filter(([name]) => !name.startsWith("PROCTOR_"))), PROCTOR_PORT: String(PROCTOR_PORT), PROCTOR_DATA: data, PROCTOR_COURSE: join(site, "🔣️.json"), ...(topology === "dev" ? { PROCTOR_MODE: "development" } : { PROCTOR_MODE: "production", PROCTOR_ALLOWED_ORIGINS: siteOrigin }) };
let proctor: ChildProcess | undefined;
const startProctor = async (): Promise<void> => {
  if (proctor && proctor.exitCode === null) return;
  proctor = launch(proctorBinary, ["serve"], proctorEnv, join(run, "proctor.log"));
  await awaitUp("proctor", `${proctorOrigin}/instance`, proctor);
};
const signalProctor = async (signal: NodeJS.Signals): Promise<void> => void proctor?.kill(signal);
const stopProctor = async (): Promise<void> => {
  const ending = proctor;
  proctor = undefined;
  if (ending) await end(ending);
};

const control = createServer((request, answer) => {
  const act = request.method !== "POST" ? undefined : request.url === "/proctor/stop" ? stopProctor : request.url === "/proctor/start" ? startProctor : request.url === "/proctor/pause" ? () => signalProctor("SIGSTOP") : request.url === "/proctor/resume" ? () => signalProctor("SIGCONT") : undefined;
  if (!act) return void answer.writeHead(404).end("POST /proctor/stop, /proctor/start, /proctor/pause or /proctor/resume");
  act().then(() => answer.writeHead(204).end(), (error: unknown) => answer.writeHead(500).end(String(error)));
});

const secret = randomBytes(32).toString("hex");
const placeholder = new TextEncoder().encode("%PDF-1.4\n%%EOF\n");
const pdfServer = createServer((request, response) => {
  const name = new URL(request.url ?? "/", "http://localhost").pathname.replace(/^\/pdfs\//u, "");
  if (request.headers.authorization !== `Bearer ${secret}` || name.includes("/")) return void response.writeHead(403).end();
  const path = join(pdfs, name);
  const bytes = existsSync(path) ? readFileSync(path) : placeholder;
  response.writeHead(200, { "content-type": "application/pdf", "content-length": bytes.length }).end(bytes);
});

const TYPES: Readonly<Record<string, string>> = { ".html": "text/html; charset=utf-8", ".js": "text/javascript; charset=utf-8", ".mjs": "text/javascript; charset=utf-8", ".css": "text/css; charset=utf-8", ".json": "application/json", ".svg": "image/svg+xml", ".pdf": "application/pdf", ".wasm": "application/wasm", ".webmanifest": "application/manifest+json", ".txt": "text/plain; charset=utf-8", ".png": "image/png", ".woff2": "font/woff2" };
function staticSite(directory: string) {
  return createServer((request, response) => {
    const pathname = decodeURIComponent(new URL(request.url ?? "/", "http://localhost").pathname);
    let file = resolve(directory, "." + pathname);
    if (!file.startsWith(directory)) file = join(directory, "404.html");
    if (existsSync(file) && statSync(file).isDirectory()) file = join(file, "index.html");
    const found = existsSync(file) && statSync(file).isFile();
    const target = found ? file : join(directory, "404.html");
    const bytes = existsSync(target) ? readFileSync(target) : Buffer.from("not found");
    response.writeHead(found ? 200 : 404, { "content-type": TYPES[extname(target)] ?? "application/octet-stream", "content-length": bytes.length }).end(bytes);
  });
}

const listen = (server: ReturnType<typeof createServer>, port: number): Promise<number> => new Promise((accept) => server.listen(port, "127.0.0.1", () => accept((server.address() as { port: number }).port)));

function playwrightRun(extra: string[], output: string, env: NodeJS.ProcessEnv): Promise<number> {
  const child = spawn(node, [playwright, "test", "--config", join(site, "🎭️e2e/🎚️config/🟦️.ts"), "--output", output, ...extra], { cwd: repo, env, stdio: "inherit" });
  return new Promise((accept) => child.once("close", (code) => accept(code ?? 1)));
}

let code = 1;
try {
  await startProctor();
  const controlPort = await listen(control, 0);
  const pdfPort = await listen(pdfServer, 0);
  const channels = JSON.stringify([{ name: "course-assignment-pdfs", endpoint: `http://127.0.0.1:${pdfPort}/pdfs`, secret }]);
  const viteEnv: NodeJS.ProcessEnv = { ...process.env, SEMIO_VITE_RUNTIME_CHANNELS_V1: channels, TEACHING_ARCHITECTURE_COURSE_WATCH: "off" };
  const vite = join(repo, "node_modules/vite/bin/vite.js");
  const config = join(site, "🏗️builder/🌐️vite/🟦️.ts");
  if (topology === "dev") {
    const server = launch(node, [vite, "--config", config, "--mode", "prototype", "--host", "127.0.0.1", "--port", String(SITE_PORT), "--strictPort"], { ...viteEnv, SEMIO_BUILD_MODE: "prototype", SEMIO_VITE_CACHE_DIRECTORY: join(repo, ".cache/b/p/m/vite-course-native-dev-e2efix"), PROCTOR_PORT: String(PROCTOR_PORT) }, join(run, "site.log"), site);
    await awaitUp("dev site", siteOrigin, server);
  } else {
    const out = join(run, "release-site");
    say("building the release site");
    const building = spawn(node, [vite, "build", "--config", config, "--mode", "release", "--outDir", out, "--emptyOutDir"], { cwd: site, env: { ...viteEnv, SEMIO_BUILD_MODE: "release", SEMIO_VITE_CACHE_DIRECTORY: join(repo, ".cache/b/p/m/vite-course-native"), PROCTOR_URL: proctorOrigin }, stdio: "inherit" });
    const status = await new Promise<number>((accept) => building.once("close", (code) => accept(code ?? 1)));
    if (status !== 0) throw Error(`release build failed (${status})`);
    await listen(staticSite(out), SITE_PORT);
    await awaitUp("static site", siteOrigin);
  }
  const env: NodeJS.ProcessEnv = { ...process.env, PLAYWRIGHT_BROWSERS_PATH: join(homedir(), "Library/Caches/ms-playwright"), PLAYWRIGHT_BASE_URL: siteOrigin, TEACHING_ARCHITECTURE_COURSE_E2E_TOPOLOGY: topology, TEACHING_ARCHITECTURE_COURSE_E2E_PROCTOR: proctorOrigin, TEACHING_ARCHITECTURE_COURSE_E2E_CONTROL: `http://127.0.0.1:${controlPort}`, PLAYWRIGHT_LAST_RUN_OUTPUT_FILE: join(run, ".last-run.json") };
  const output = join(run, "pw-output");
  if (process.env.KEEP === "1") {
    say(`kept up: control http://127.0.0.1:${controlPort}`);
    await new Promise<void>((accept) => process.once("SIGTERM", () => accept()));
    throw Error("stopped");
  }
  if (process.env.SKIP_WARMUP !== "1") {
    const warm = await playwrightRun(["--project", "boot", "--workers", "1"], output, env);
    if (warm !== 0) throw Error(`warm-up boot spec failed (${warm})`);
  }
  const workers = chosen.every((name) => projects.exclusive!.includes(name)) ? "1" : process.env.WORKERS ?? "3";
  code = await playwrightRun([...chosen.flatMap((name) => ["--project", name]), "--workers", workers, ...forwarded], output, env);
  await startProctor().catch(() => undefined);
} catch (error) {
  console.error(`[native-stack] ${error instanceof Error ? error.message : String(error)}`);
} finally {
  control.close();
  pdfServer.close();
  await Promise.all(owned.map(end));
  process.exit(code);
}
