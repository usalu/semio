/** 🎼️ The release rehearsal of the end-to-end gate without the build graph: the release site baked for a throw-away proctor, served as static files on its own origin, a production-mode proctor that admits only that origin, and Playwright driving the named projects against it. The production build in `dist/release` is restored afterwards.
 * Usage (from the repository root): node <this file> <log directory> [Playwright arguments…] */
import { spawn, spawnSync, type ChildProcess } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync } from "node:fs";
import { createServer } from "node:http";
import { createRequire } from "node:module";
import { dirname, extname, join, resolve, sep } from "node:path";

const [logs, ...forwarded] = process.argv.slice(2);
if (!logs) throw Error("Usage: site-rehearsal.ts <log directory> [Playwright arguments…]");
const workspace = resolve(import.meta.dirname, "../../../../../../..");
const site = join(workspace, "🎓️teaching/🏛️architecture/📚️course");
const bundle = join(site, "📦️packages/🟦️typescript");
const ports = { site: 6063, proctor: 8793 };
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
const types: Readonly<Record<string, string>> = { ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".css": "text/css", ".json": "application/json", ".webmanifest": "application/manifest+json", ".svg": "image/svg+xml", ".png": "image/png", ".ico": "image/x-icon", ".pdf": "application/pdf", ".txt": "text/plain; charset=utf-8", ".wasm": "application/wasm" };
const server = createServer((request, response) => {
  const path = decodeURIComponent(new URL(request.url ?? "/", "http://site").pathname);
  let file = resolve(artifact, "." + path);
  if (!file.startsWith(artifact + sep) && file !== artifact) file = join(artifact, "404.html");
  if (existsSync(file) && statSync(file).isDirectory()) file = join(file, "index.html");
  const found = existsSync(file);
  if (!found) file = join(artifact, "404.html");
  response.statusCode = found ? 200 : 404;
  response.setHeader("content-type", types[extname(file)] ?? "application/octet-stream");
  response.end(readFileSync(file));
});
await new Promise<void>(accept => server.listen(ports.site, "127.0.0.1", accept));
const data = join(resolve(logs), "proctor-data");
rmSync(data, { recursive: true, force: true });
mkdirSync(data, { recursive: true });
const executable = join(workspace, ".cache", "b", "r", "m", "u", "l", "acd", "t", "debug", process.platform === "win32" ? "proctor.exe" : "proctor");
const inherited = Object.fromEntries(Object.entries(process.env).filter(([name]) => !name.startsWith("PROCTOR_")));
const proctor: ChildProcess = spawn(executable, ["serve"], { cwd: workspace, stdio: "inherit", env: { ...inherited, PROCTOR_PORT: String(ports.proctor), PROCTOR_DATA: data, PROCTOR_COURSE: join(site, "🔣️.json"), PROCTOR_MODE: "production", PROCTOR_ALLOWED_ORIGINS: origin.site } });
const stop = (status: number): never => { proctor.kill(); server.close(); server.closeAllConnections(); process.exit(status); };
for (let waited = 0; !(await fetch(`${origin.proctor}/instance`, { signal: AbortSignal.timeout(2_000) }).then(answer => answer.ok, () => false)); waited += 1) { if (waited > 120) stop(1); await new Promise(accept => setTimeout(accept, 1_000)); }
say(`site at ${origin.site}, proctor at ${origin.proctor} (production)`);
const cli = join(dirname(createRequire(join(site, "🧪️tests", "🚀️site-boot", "🟦️.ts")).resolve("@playwright/test/package.json")), "..", "..", "playwright", "cli.js");
const specs = spawn(process.execPath, [cli, "test", "--config", join(site, "🎭️e2e/🎚️config/🟦️.ts"), "--output", join(resolve(logs), "playwright"), ...forwarded], { cwd: workspace, stdio: "inherit", env: { ...process.env, PLAYWRIGHT_BASE_URL: origin.site, TEACHING_ARCHITECTURE_COURSE_E2E_TOPOLOGY: "rehearsal", TEACHING_ARCHITECTURE_COURSE_E2E_PROCTOR: origin.proctor } });
specs.on("exit", status => { say(`Playwright ended with status ${status}`); stop(status ?? 1); });
