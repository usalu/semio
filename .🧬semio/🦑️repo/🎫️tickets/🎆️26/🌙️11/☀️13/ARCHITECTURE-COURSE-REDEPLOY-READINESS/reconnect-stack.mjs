/** 🔁️ Static reconnect-storm stack: the release site (reconnect-build.mjs) with SPA fallback on 8797's sibling port, and one production-mode proctor on 8797 that exactly one control queue starts and kills by PID. Usage: PROCTOR_EXECUTABLE=… node reconnect-stack.mjs; writes 🗑️generated/reconnect/stack.env and runs until SIGTERM. */
import { spawn } from "node:child_process";
import { existsSync, mkdirSync, openSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { createServer } from "node:http";
import { connect } from "node:net";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const repo = resolve(here, "../../../../../../..");
const site = join(repo, "🎓️teaching/🏛️architecture/📚️course");
const work = join(here, "🗑️generated/reconnect");
const data = join(work, "data");
const PORT = "8797";
const SITE_PORT = "6066";
const executable = process.env.PROCTOR_EXECUTABLE;
if (!executable || !existsSync(executable)) throw new Error("PROCTOR_EXECUTABLE must name the built proctor");
rmSync(data, { recursive: true, force: true });
mkdirSync(data, { recursive: true });

const listening = (port) => new Promise((done) => {
  const socket = connect(Number(port), "127.0.0.1");
  socket.once("connect", () => (socket.destroy(), done(true)));
  socket.once("error", () => done(false));
});

let child;
let exited;
let queue = Promise.resolve();
let spawned = 0;
const serial = (task) => (queue = queue.then(task, task));

const up = async () => {
  if (child) return;
  if (await listening(PORT)) throw new Error("port 8797 is taken by a process this stack does not own");
  const log = openSync(join(work, "proctor.log"), "a");
  const process_ = spawn(executable, ["serve"], { env: { ...Object.fromEntries(Object.entries(process.env).filter(([key]) => !key.startsWith("PROCTOR_"))), PROCTOR_PORT: PORT, PROCTOR_DATA: data, PROCTOR_COURSE: join(site, "🔣️.json"), PROCTOR_MODE: "production", PROCTOR_ALLOWED_ORIGINS: `http://127.0.0.1:${SITE_PORT}`, PROCTOR_LIMIT_SIGNUPS_PER_HOUR: "100000" }, stdio: ["ignore", log, log] });
  child = process_;
  spawned += 1;
  exited = new Promise((done) => process_.once("exit", () => (child === process_ && (child = undefined), done())));
  for (let tries = 0; tries < 300; tries++) {
    if (!child) throw new Error("the proctor exited while starting");
    try { if ((await fetch(`http://127.0.0.1:${PORT}/instance`)).ok) return; } catch {}
    await new Promise((r) => setTimeout(r, 50));
  }
  throw new Error("proctor not ready");
};
const down = async () => {
  const process_ = child;
  if (!process_) return;
  process_.kill("SIGKILL");
  await exited;
  for (let tries = 0; tries < 100 && (await listening(PORT)); tries++) await new Promise((r) => setTimeout(r, 20));
  if (await listening(PORT)) throw new Error("port 8797 still listens after the kill");
};
await up();

const signal = (name) => async () => {
  if (!child) throw new Error("the proctor is not running");
  child.kill(name);
};

const control = createServer((request, response) => {
  const act = request.method === "POST" && request.url === "/proctor/stop" ? down : request.method === "POST" && request.url === "/proctor/start" ? up : request.method === "POST" && request.url === "/proctor/pause" ? signal("SIGSTOP") : request.method === "POST" && request.url === "/proctor/resume" ? signal("SIGCONT") : request.method === "GET" && request.url === "/proctor/spawned" ? async () => { response.setHeader("x-spawned", String(spawned)); } : undefined;
  if (!act) { response.statusCode = 404; return void response.end(); }
  serial(act).then(() => { response.statusCode = 204; response.end(); }, (error) => { response.statusCode = 500; response.end(String(error)); });
});
await new Promise((accept) => control.listen(0, "127.0.0.1", accept));

const types = { ".html": "text/html; charset=utf-8", ".js": "text/javascript", ".mjs": "text/javascript", ".css": "text/css", ".json": "application/json", ".svg": "image/svg+xml", ".woff2": "font/woff2", ".woff": "font/woff", ".png": "image/png", ".wasm": "application/wasm", ".pdf": "application/pdf", ".ico": "image/x-icon", ".webp": "image/webp", ".txt": "text/plain", ".map": "application/json" };
const root = join(work, "site");
const statics = createServer((request, response) => {
  const path = decodeURIComponent(new URL(request.url ?? "/", "http://x").pathname);
  let file = join(root, path);
  if (!file.startsWith(root) || !existsSync(file) || statSync(file).isDirectory()) file = join(root, "index.html");
  const bytes = readFileSync(file);
  response.setHeader("content-type", types[file.slice(file.lastIndexOf("."))] ?? "application/octet-stream");
  response.setHeader("cache-control", "public, max-age=3600");
  response.end(bytes);
});
await new Promise((accept) => statics.listen(Number(SITE_PORT), "127.0.0.1", accept));
writeFileSync(join(work, "stack.env"), `export PLAYWRIGHT_BASE_URL=http://127.0.0.1:${SITE_PORT}\nexport TEACHING_ARCHITECTURE_COURSE_E2E_TOPOLOGY=rehearsal\nexport TEACHING_ARCHITECTURE_COURSE_E2E_PROCTOR=http://127.0.0.1:${PORT}\nexport TEACHING_ARCHITECTURE_COURSE_E2E_CONTROL=http://127.0.0.1:${control.address().port}\n`);
console.log("stack ready");
const end = async () => { statics.close(); control.close(); await serial(down); process.exit(0); };
process.on("SIGTERM", end);
process.on("SIGINT", end);
