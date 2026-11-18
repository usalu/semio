/** 🌪️ Native chaos stack: PDF channel + proctor (killable through a loopback control endpoint) + vite dev site. Usage: node chaos-stack.mjs; prints the env lines for the spec and runs until SIGTERM. */
import { spawn } from "node:child_process";
import { randomBytes } from "node:crypto";
import { existsSync, mkdirSync, openSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { createServer } from "node:http";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const repo = resolve(here, "../../../../../../..");
const site = join(repo, "🎓️teaching/🏛️architecture/📚️course");
const work = join(here, process.env.CHAOS_WORK ?? "🗑️generated/chaos");
const data = join(work, "data");
const PROCTOR_PORT = process.env.PROCTOR_PORT ?? "8793";
const SITE_PORT = process.env.SITE_PORT ?? "6062";
const executable = process.env.PROCTOR_EXECUTABLE ?? join(repo, ".cache/b/r/m/u/l/p001ec5/prototype/build/teaching-proctor/762af25000b726d1/out/proctor");
const pdfs = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9d0eceba-b1b8-4814-b9cd-caae5bbd19aa/scratchpad/pdfs/out";
rmSync(data, { recursive: true, force: true });
mkdirSync(data, { recursive: true });

const secret = randomBytes(32).toString("hex");
const channel = createServer((request, response) => {
  const name = new URL(request.url ?? "/", "http://localhost").pathname.replace(/^\/pdfs\//, "");
  if (request.headers.authorization !== "Bearer " + secret || name.includes("/")) { response.statusCode = 403; return void response.end(); }
  const bytes = readFileSync(join(pdfs, name));
  response.setHeader("content-type", "application/pdf");
  response.setHeader("content-length", bytes.length);
  response.end(bytes);
});
await new Promise((accept) => channel.listen(0, "127.0.0.1", accept));

let proctor;
const startProctor = async () => {
  if (proctor) return;
  const log = openSync(join(work, "proctor.log"), "a");
  proctor = spawn(executable, ["serve"], { env: { ...Object.fromEntries(Object.entries(process.env).filter(([k]) => !k.startsWith("PROCTOR_"))), PROCTOR_PORT, PROCTOR_DATA: data, PROCTOR_COURSE: join(site, "🔣️.json"), ...(process.env.STATIC === "1" ? { PROCTOR_MODE: "production", PROCTOR_ALLOWED_ORIGINS: `http://127.0.0.1:${SITE_PORT}` } : { PROCTOR_MODE: "development" }), PROCTOR_LIMIT_SIGNUPS_PER_HOUR: "100000" }, stdio: ["ignore", log, log] });
  proctor.once("exit", () => { proctor = undefined; });
  for (let i = 0; i < 200; i++) {
    try { if ((await fetch(`http://127.0.0.1:${PROCTOR_PORT}/instance`)).ok) return; } catch {}
    await new Promise((r) => setTimeout(r, 100));
  }
  throw new Error("proctor not ready");
};
const stopProctor = async () => {
  const child = proctor;
  if (!child) return;
  proctor = undefined;
  const gone = new Promise((r) => child.once("exit", r));
  child.kill("SIGKILL");
  await gone;
};
await startProctor();

const control = createServer((request, response) => {
  const act = request.method === "POST" && request.url === "/proctor/stop" ? stopProctor : request.method === "POST" && request.url === "/proctor/start" ? startProctor : undefined;
  if (!act) { response.statusCode = 404; return void response.end(); }
  act().then(() => { response.statusCode = 204; response.end(); }, (e) => { response.statusCode = 500; response.end(String(e)); });
});
await new Promise((accept) => control.listen(0, "127.0.0.1", accept));

const types = { ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".mjs": "text/javascript", ".css": "text/css", ".json": "application/json", ".svg": "image/svg+xml", ".woff2": "font/woff2", ".woff": "font/woff", ".png": "image/png", ".wasm": "application/wasm", ".pdf": "application/pdf", ".ico": "image/x-icon", ".webp": "image/webp", ".txt": "text/plain", ".map": "application/json" };
const root = join(work, "site");
let vite;
let statics;
if (process.env.STATIC === "1") {
  statics = createServer((request, response) => {
    const path = decodeURIComponent(new URL(request.url ?? "/", "http://x").pathname);
    let file = join(root, path);
    if (!file.startsWith(root) || !existsSync(file) || statSync(file).isDirectory()) file = join(root, "index.html");
    const bytes = readFileSync(file);
    const extension = file.slice(file.lastIndexOf("."));
    response.setHeader("content-type", types[extension] ?? "application/octet-stream");
    response.setHeader("cache-control", "public, max-age=3600");
    response.end(bytes);
  });
  await new Promise((accept) => statics.listen(Number(SITE_PORT), "127.0.0.1", accept));
} else {
  vite = spawn(process.execPath, [join(repo, "node_modules/vite/bin/vite.js"), "--config", join(site, "🏗️builder/🌐️vite/🟦️.ts"), "--mode", "prototype", "--host", "127.0.0.1", "--port", SITE_PORT, "--strictPort"], {
    cwd: site,
    env: { ...process.env, PROCTOR_PORT, TEACHING_ARCHITECTURE_COURSE_WATCH: "off", SEMIO_BUILD_MODE: "prototype", SEMIO_VITE_CACHE_DIRECTORY: join(repo, ".cache/b/p/m/vite-course-native-chaos"), SEMIO_VITE_RUNTIME_CHANNELS_V1: JSON.stringify([{ name: "course-assignment-pdfs", endpoint: `http://127.0.0.1:${channel.address().port}/pdfs`, secret }]) },
    stdio: ["ignore", openSync(join(work, "site.log"), "a"), openSync(join(work, "site.log"), "a")],
  });
}
for (let i = 0; i < 600; i++) {
  try { if ((await fetch(`http://127.0.0.1:${SITE_PORT}/`)).ok) break; } catch {}
  await new Promise((r) => setTimeout(r, 200));
}
writeFileSync(join(work, "stack.env"), `export PLAYWRIGHT_BASE_URL=http://127.0.0.1:${SITE_PORT}\nexport TEACHING_ARCHITECTURE_COURSE_E2E_PROCTOR=http://127.0.0.1:${PROCTOR_PORT}\nexport TEACHING_ARCHITECTURE_COURSE_E2E_CONTROL=http://127.0.0.1:${control.address().port}\n`);
console.log("stack ready");
const end = async () => { vite?.kill("SIGTERM"); statics?.close(); await stopProctor(); channel.close(); control.close(); process.exit(0); };
process.on("SIGTERM", end);
process.on("SIGINT", end);
