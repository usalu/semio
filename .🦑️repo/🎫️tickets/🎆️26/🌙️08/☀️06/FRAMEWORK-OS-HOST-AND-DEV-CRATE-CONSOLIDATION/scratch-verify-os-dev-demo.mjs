import { spawn } from "child_process";
import { writeFileSync } from "fs";
import { join } from "path";

const tick = process.argv[2];
const devPkg = process.argv[3];
const logPath = join(tick, "🧪verify-os-dev-server.log");
const outPath = join(tick, "🧪os-dev-browser-errors.json");

const child = spawn("bun", ["./📜️script.ts", "dev"], {
  cwd: devPkg,
  env: {
    ...process.env,
    PATH: "/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:" + (process.env.PATH || ""),
    DEVELOPER_DIR: "/Library/Developer/CommandLineTools",
    SDKROOT: "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk",
    SKIP_ENGINE_BUILD: "1",
    SKIP_PLUGIN_BUILD: "1",
    SEMIO_PLUGIN: "demonstrator",,
    S_OS_PORT: "6066",
  },
  stdio: ["ignore", "pipe", "pipe"],
  detached: true,
});

let log = "";
const append = (buf) => {
  log += buf.toString();
  writeFileSync(logPath, log);
};
child.stdout.on("data", append);
child.stderr.on("data", append);

let port = null;
const deadline = Date.now() + 120000;
while (Date.now() < deadline) {
  const m = log.match(/http:\/\/127\.0\.0\.1:(\d+)\//);
  if (m) port = m[1];
  if (port) {
    try {
      const res = await fetch("http://127.0.0.1:" + port + "/");
      if (res.ok || res.status === 200) break;
    } catch {}
  }
  if (child.exitCode != null && Date.now() > deadline - 1000) break;
  await new Promise((r) => setTimeout(r, 500));
}

if (!port) {
  const out = { error: "vite never became ready", logTail: log.slice(-2000), exitCode: child.exitCode };
  writeFileSync(outPath, JSON.stringify(out, null, 2));
  console.log(JSON.stringify(out, null, 2));
  try { process.kill(-child.pid, "SIGKILL"); } catch {}
  process.exit(1);
}

// confirm fetch works
let fetchOk = false;
for (let i = 0; i < 20; i++) {
  try {
    const res = await fetch("http://127.0.0.1:" + port + "/");
    fetchOk = res.status > 0;
    if (fetchOk) break;
  } catch {}
  await new Promise((r) => setTimeout(r, 500));
}

const { chromium } = await import("playwright");
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const errors = [];
page.on("pageerror", (e) => errors.push("pageerror:" + String(e)));
page.on("console", (msg) => { if (msg.type() === "error") errors.push("console:" + msg.text()); });
const url = `http://127.0.0.1:${port}/`;
try {
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
  await page.waitForTimeout(15000);
} catch (e) {
  errors.push("goto:" + e);
}
const title = await page.title().catch(() => "");
const bodyText = await page.locator("body").innerText().catch(() => "");
const rootChildCount = await page.locator("#root > *").count().catch(() => -1);
await browser.close();

const out = { url, title, bodyPreview: bodyText.slice(0, 1500), rootChildCount, errors, fetchOk, logTail: log.slice(-2000) };
writeFileSync(outPath, JSON.stringify(out, null, 2));
console.log(JSON.stringify(out, null, 2));
try { process.kill(-child.pid, "SIGKILL"); } catch { try { child.kill("SIGKILL"); } catch {} }
process.exit(errors.some((e) => e.startsWith("pageerror:")) ? 2 : 0);
