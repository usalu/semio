/** 🎼️ The release rehearsal of the end-to-end gate without the build graph: the release site baked for a throw-away proctor, served as static files on its own origin the way a CDN serves them (compressed, hashed assets immutable, everything else revalidated), a production-mode proctor that admits only that origin, the control endpoint that stops, starts, freezes and thaws that proctor for the specs of outages (`POST /proctor/stop|start|pause|resume`), and Playwright driving the named projects against it. The production build in `dist/release` is restored afterwards.
 * Usage (from the repository root): node <this file> <log directory> [Playwright arguments…]; `REHEARSAL_PORT_SHIFT` moves its three ports so that a second rehearsal runs beside a long one; `REHEARSAL_PROCTOR_DATA` names a data directory the proctor keeps instead of a fresh one. */
import { spawn, spawnSync, type ChildProcess } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync } from "node:fs";
import { createHash } from "node:crypto";
import { createServer } from "node:http";
import { createRequire } from "node:module";
import { dirname, extname, join, resolve, sep } from "node:path";
import { brotliCompressSync, gzipSync } from "node:zlib";

const [logs, ...forwarded] = process.argv.slice(2);
if (!logs) throw Error("Usage: site-rehearsal.ts <log directory> [Playwright arguments…]");
const workspace = resolve(import.meta.dirname, "../../../../../../..");
const site = join(workspace, "🎓️teaching/🏛️architecture/📚️course");
const bundle = join(site, "📦️packages/🟦️typescript");
const shift = Number(process.env.REHEARSAL_PORT_SHIFT ?? 0);
const ports = { site: 6063 + shift, proctor: 8793 + shift, control: 8794 + shift };
const origin = { site: `http://127.0.0.1:${ports.site}`, proctor: `http://127.0.0.1:${ports.proctor}` };
const artifact = join(resolve(logs), "site");
const direct = join(import.meta.dirname, "site-direct.ts");
const say = (line: string): void => console.log(`[rehearsal] ${line}`);
const build = (proctor?: string): void => {
  const built = spawnSync(process.execPath, [direct, "build", "release"], { cwd: workspace, stdio: "inherit", env: { ...process.env, ...(proctor === undefined ? {} : { PROCTOR_URL: proctor }) } });
  if (built.status !== 0) throw Error("The release site did not build");
};
mkdirSync(resolve(logs), { recursive: true });
say(`building the release site for ${origin.proctor}`);
build(origin.proctor);
rmSync(artifact, { recursive: true, force: true });
const copy = (from: string, to: string): void => {
  mkdirSync(to, { recursive: true });
  for (const entry of readdirSync(from, { withFileTypes: true })) entry.isDirectory() ? copy(join(from, entry.name), join(to, entry.name)) : copyFileSync(join(from, entry.name), join(to, entry.name));
};
copy(join(bundle, "dist", "release"), artifact);
say("restoring the production build");
const env = Object.fromEntries(Object.entries(process.env).filter(([name]) => name !== "PROCTOR_URL"));
if (spawnSync(process.execPath, [direct, "build", "release"], { cwd: workspace, stdio: "inherit", env }).status !== 0) throw Error("The production site did not build again");
const types: Readonly<Record<string, string>> = { ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".css": "text/css", ".json": "application/json", ".webmanifest": "application/manifest+json", ".svg": "image/svg+xml", ".png": "image/png", ".ico": "image/x-icon", ".pdf": "application/pdf", ".txt": "text/plain; charset=utf-8", ".wasm": "application/wasm", ".woff2": "font/woff2" };
const packed = new Map<string, { readonly tag: string; readonly plain: Buffer; readonly gzip: Buffer | undefined; readonly brotli: Buffer | undefined }>();
const squeezes = new Set([".html", ".js", ".css", ".json", ".webmanifest", ".svg", ".txt", ".wasm"]);
const held = (file: string): { readonly tag: string; readonly plain: Buffer; readonly gzip: Buffer | undefined; readonly brotli: Buffer | undefined } => {
  const known = packed.get(file);
  if (known !== undefined) return known;
  const plain = readFileSync(file);
  const squeezed = squeezes.has(extname(file));
  const entry = { tag: `"${createHash("sha256").update(plain).digest("hex").slice(0, 24)}"`, plain, gzip: squeezed ? gzipSync(plain) : undefined, brotli: squeezed ? brotliCompressSync(plain) : undefined };
  packed.set(file, entry);
  return entry;
};
const server = createServer((request, response) => {
  const path = decodeURIComponent(new URL(request.url ?? "/", "http://site").pathname);
  let file = resolve(artifact, "." + path);
  if (!file.startsWith(artifact + sep) && file !== artifact) file = join(artifact, "404.html");
  if (existsSync(file) && statSync(file).isDirectory()) file = join(file, "index.html");
  const found = existsSync(file);
  if (!found) file = join(artifact, "404.html");
  const entry = held(file);
  const accepted = String(request.headers["accept-encoding"] ?? "");
  const [encoding, body] = entry.brotli !== undefined && /br/u.test(accepted) ? ["br", entry.brotli] : entry.gzip !== undefined && /gzip/u.test(accepted) ? ["gzip", entry.gzip] : [undefined, entry.plain];
  response.setHeader("content-type", types[extname(file)] ?? "application/octet-stream");
  response.setHeader("cache-control", !found || extname(file) === ".html" ? "no-cache" : path.startsWith("/assets/") ? "public, max-age=31536000, immutable" : "public, max-age=600");
  response.setHeader("etag", entry.tag);
  response.setHeader("vary", "accept-encoding");
  if (found && request.headers["if-none-match"] === entry.tag) { response.statusCode = 304; return void response.end(); }
  if (encoding !== undefined) response.setHeader("content-encoding", encoding);
  response.statusCode = found ? 200 : 404;
  response.end(body);
});
await new Promise<void>(accept => server.listen(ports.site, "127.0.0.1", accept));
const kept = process.env.REHEARSAL_PROCTOR_DATA;
const data = kept === undefined ? join(resolve(logs), "proctor-data") : resolve(kept);
if (kept === undefined) rmSync(data, { recursive: true, force: true });
mkdirSync(data, { recursive: true });
const executable = join(workspace, ".cache", "b", "r", "m", "u", "l", "acd", "t", "debug", process.platform === "win32" ? "proctor.exe" : "proctor");
const inherited = Object.fromEntries(Object.entries(process.env).filter(([name]) => !name.startsWith("PROCTOR_")));
const answers = (): Promise<boolean> => fetch(`${origin.proctor}/instance`, { signal: AbortSignal.timeout(2_000) }).then(answer => answer.ok, () => false);
let proctor: ChildProcess | undefined;
const launch = async (): Promise<void> => {
  if (proctor !== undefined) return;
  proctor = spawn(executable, ["serve"], { cwd: workspace, stdio: "inherit", env: { ...inherited, PROCTOR_PORT: String(ports.proctor), PROCTOR_DATA: data, PROCTOR_COURSE: join(site, "🔣️.json"), PROCTOR_MODE: "production", PROCTOR_ALLOWED_ORIGINS: origin.site } });
  for (let waited = 0; !(await answers()); waited += 1) { if (waited > 120) throw Error("The proctor did not answer"); await new Promise(accept => setTimeout(accept, 500)); }
};
const halt = async (): Promise<void> => {
  const ending = proctor;
  proctor = undefined;
  if (ending === undefined || ending.exitCode !== null) return;
  await new Promise<void>(accept => { ending.once("exit", () => accept()); ending.kill(); });
};
const signalled = async (signal: NodeJS.Signals): Promise<void> => {
  if (proctor === undefined) throw Error("the proctor is not running");
  if (process.platform === "win32") throw Error("this platform has no signal that freezes a process");
  proctor.kill(signal);
};
const control = createServer((request, response) => {
  const act = request.method !== "POST" ? undefined : request.url === "/proctor/stop" ? halt : request.url === "/proctor/start" ? launch : request.url === "/proctor/pause" ? () => signalled("SIGSTOP") : request.url === "/proctor/resume" ? () => signalled("SIGCONT") : undefined;
  if (act === undefined) { response.statusCode = 404; return void response.end("POST /proctor/stop, /proctor/start, /proctor/pause or /proctor/resume"); }
  act().then(() => { response.statusCode = 204; response.end(); }, (error: unknown) => { response.statusCode = 500; response.end(error instanceof Error ? error.message : String(error)); });
});
await new Promise<void>(accept => control.listen(ports.control, "127.0.0.1", accept));
const stop = (status: number): never => { proctor?.kill(); control.close(); control.closeAllConnections(); server.close(); server.closeAllConnections(); process.exit(status); };
await launch().catch(() => stop(1));
say(`site at ${origin.site}, proctor at ${origin.proctor} (production)`);
const cli = join(dirname(createRequire(join(site, "🧪️tests", "🚀️site-boot", "🟦️.ts")).resolve("@playwright/test/package.json")), "..", "..", "playwright", "cli.js");
const specs = spawn(process.execPath, [cli, "test", "--config", join(site, "🎭️e2e/🎚️config/🟦️.ts"), "--output", join(resolve(logs), "playwright"), ...forwarded], { cwd: workspace, stdio: "inherit", env: { ...process.env, PLAYWRIGHT_BASE_URL: origin.site, TEACHING_ARCHITECTURE_COURSE_E2E_TOPOLOGY: "rehearsal", TEACHING_ARCHITECTURE_COURSE_E2E_PROCTOR: origin.proctor, TEACHING_ARCHITECTURE_COURSE_E2E_CONTROL: `http://127.0.0.1:${ports.control}` } });
specs.on("exit", status => { say(`Playwright ended with status ${status}`); stop(status ?? 1); });
