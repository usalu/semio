import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { timingSafeEqual, createHash, randomBytes } from "node:crypto";
import { createReadStream, readFileSync } from "node:fs";
import { mkdtemp, readFile } from "node:fs/promises";
import { createServer as serve } from "node:http";
import { once } from "node:events";
import { join, resolve } from "node:path";
import { createServer as vite } from "vite";

const workspace = process.cwd();
const ticket = import.meta.dirname;
const generated = await mkdtemp(join(ticket, "🗑️generated", "on-"));
const courseName = "🎓️teaching/🏛️architecture/📚️course";
const courseRoot = resolve(courseName);
const courseFile = join(courseRoot, "🔣️.json");
const published = resolve(".cache/b/p/m/print/prototype/documents");
const proctorRoot = resolve("🎓️teaching/🛂️proctor/📦️packages/🦀️rust/dist/build/prototype");
const proctorReceipt = JSON.parse(await readFile(join(proctorRoot, "receipt.json"), "utf8"));
assert.equal(proctorReceipt.mode, "prototype");
assert.equal(proctorReceipt.platform, process.platform);
assert.equal(proctorReceipt.architecture, process.arch);
const executable = join(proctorRoot, proctorReceipt.executable);
let size = 0;
const digest = createHash("sha256");
for await (const bytes of createReadStream(executable)) {
  size += bytes.length;
  digest.update(bytes);
}
assert.equal(size, proctorReceipt.bytes);
assert.equal(digest.digest("hex"), proctorReceipt.sha256);

const course = JSON.parse(await readFile(courseFile, "utf8"));
const assignments = course.assignments.flatMap((assignment) => Object.values(assignment.documents));
assert.equal(assignments.length, 6);
const pdfs = new Map();
for (const document of assignments) {
  const directory = join(published, document.namespace, document.id);
  const receipt = JSON.parse(readFileSync(join(directory, ".semio-artifact.json"), "utf8"));
  assert.equal(receipt.owner, document.producer);
  assert.ok(receipt.files.includes(document.file));
  const bytes = await readFile(join(directory, document.file));
  assert.equal(bytes.subarray(0, 5).toString(), "%PDF-");
  pdfs.set(document.file, bytes);
}
const secret = randomBytes(32).toString("hex");
const bearer = Buffer.from("Bearer " + secret);
const pdfsChannel = serve((request, response) => {
  const authorization = Buffer.from(request.headers.authorization ?? "");
  if (authorization.length !== bearer.length || !timingSafeEqual(authorization, bearer)) {
    response.writeHead(403).end();
    return;
  }
  const bytes = request.url?.startsWith("/pdfs/") ? pdfs.get(request.url.slice(6)) : undefined;
  if (request.method !== "GET" || !bytes) {
    response.writeHead(404).end();
    return;
  }
  response.writeHead(200, { "content-type": "application/pdf", "content-length": bytes.length }).end(bytes);
});
const loopback = "127.0.0.1";
const proctorPort = process.env.COURSE_LIVE_PROCTOR_PORT ?? "6281";
const proctorOrigin = `http://${loopback}:${proctorPort}`;
const inherited = Object.fromEntries(Object.entries(process.env).filter(([name]) => !name.startsWith("PROCTOR_")));
let proctor;
let client;
let playwright;
const interrupt = () => {
  playwright?.kill();
  proctor?.kill();
  pdfsChannel.closeAllConnections();
  void client?.close();
};
process.once("SIGINT", interrupt);
process.once("SIGTERM", interrupt);
try {
  pdfsChannel.listen(0, loopback);
  await once(pdfsChannel, "listening");
  proctor = spawn(executable, ["serve"], {
    cwd: workspace,
    env: { ...inherited, PROCTOR_MODE: "development", PROCTOR_PORT: proctorPort, PROCTOR_DATA: join(generated, "p"), PROCTOR_COURSE: courseFile },
    stdio: "inherit",
  });
  const expires = Date.now() + 30_000;
  for (;;) {
    const result = await fetch(proctorOrigin + "/instance", { signal: AbortSignal.timeout(1000) }).catch(() => undefined);
    if (result?.ok && (await result.text()).includes("teaching-proctor")) break;
    assert.equal(proctor.exitCode, null, "Native Proctor exited before readiness");
    assert.ok(Date.now() < expires, "Native Proctor did not become ready");
    await new Promise((accept) => setTimeout(accept, 150));
  }
  console.info("[DEBUG] Native Proctor ready");
  process.env.PROCTOR_PORT = proctorPort;
  process.env.TEACHING_ARCHITECTURE_COURSE_WATCH = "off";
  process.env.SEMIO_BUILD_MODE = "prototype";
  process.env.SEMIO_VITE_CACHE_DIRECTORY = resolve(".cache/b/p/m/phone-course-live");
  process.env.SEMIO_VITE_RUNTIME_CHANNELS_V1 = JSON.stringify([{ name: "course-assignment-pdfs", endpoint: `http://${loopback}:${pdfsChannel.address().port}/pdfs`, secret }]);
  const { default: load } = await import(new URL(`../../../../../../../${courseName}/🏗️builder/🌐️vite/🟦️.ts`, import.meta.url));
  const config = await load({ command: "serve", mode: "prototype" });
  client = await vite({
    ...config,
    cacheDir: join(generated, "v"),
    configFile: false,
    envFile: false,
    mode: "prototype",
    server: { ...config.server, host: loopback, port: Number(process.env.COURSE_LIVE_SITE_PORT ?? "6282"), strictPort: true },
  });
  await client.listen();
  const origin = `http://${loopback}:${client.httpServer.address().port}`;
  const instance = await fetch(origin + "/instance");
  assert.equal(instance.status, 200);
  assert.match(await instance.text(), /teaching-proctor/u);
  for (const [file, bytes] of pdfs) {
    const response = await fetch(origin + "/assignments/" + file);
    assert.equal(response.status, 200);
    assert.equal(response.headers.get("content-type"), "application/pdf");
    assert.deepEqual(Buffer.from(await response.arrayBuffer()), bytes);
  }
  console.info("[DEBUG] Vite proxy and byte-for-byte HTTP delivery of six real assignment PDFs passed", origin);
  if (process.env.COURSE_LIVE_READY_ONLY !== "1") {
    const output = join(generated, "b");
    const report = join(generated, "r.json");
    playwright = spawn("repo", ["tools", "exec", "node", "--light", "--", join(workspace, "🎓️teaching/node_modules/playwright/cli.js"), "test", "--config", join(courseRoot, "🎭️e2e/🎚️config/🟦️.ts"), "--project", "boot", "--project", "phone", "--workers", "1", "--output", output, "--reporter", "list,json"], {
      cwd: workspace,
      env: {
        ...inherited,
        PLAYWRIGHT_BROWSERS_PATH: resolve(".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright"),
        PLAYWRIGHT_BASE_URL: origin,
        PLAYWRIGHT_LAST_RUN_OUTPUT_FILE: join(output, ".last-run.json"),
        PLAYWRIGHT_JSON_OUTPUT_FILE: report,
        TEACHING_ARCHITECTURE_COURSE_E2E_TOPOLOGY: "dev",
        TEACHING_ARCHITECTURE_COURSE_E2E_PROCTOR: proctorOrigin,
      },
      stdio: "inherit",
    });
    const [status] = await once(playwright, "exit");
    assert.equal(status, 0, "Canonical boot and phone Playwright projects");
  }
} finally {
  await client?.close();
  if (proctor && proctor.exitCode === null && proctor.signalCode === null) {
    proctor.kill();
    await once(proctor, "close");
  }
  pdfsChannel.closeAllConnections();
  if (pdfsChannel.listening) {
    pdfsChannel.close();
    await once(pdfsChannel, "close");
  }
}
