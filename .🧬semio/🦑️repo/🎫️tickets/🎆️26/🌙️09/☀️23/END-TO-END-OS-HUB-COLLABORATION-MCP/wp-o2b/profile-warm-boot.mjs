#!/usr/bin/env bun
import { spawn } from "node:child_process";
import { createWriteStream, mkdirSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { setTimeout as sleep } from "node:timers/promises";

const REPO = "/Users/ueli/Documents/semio";
const T = join(REPO, ".tmp-ticket/wp-o2b");
const G = join(T, "generated");
mkdirSync(G, { recursive: true });
const PORT = Number(process.env.O2B_PORT || 6220);
const OSDEV = process.env.O2B_OSDEV;
if (!OSDEV) throw new Error("O2B_OSDEV required");
const mode = process.argv[2] || "direct";
const t0 = Date.now();
const marks = [];
const mark = (label) => {
  const ms = Date.now() - t0;
  marks.push({ label, ms });
  console.log(`[phase] +${ms}ms ${label}`);
};

const serveLog = join(G, `warm-${mode}-p${PORT}-serve.txt`);
const out = createWriteStream(serveLog, { flags: "w" });
mark(`start mode=${mode} port=${PORT}`);

const env = {
  ...process.env,
  S_LOCAL_ONLY: "1",
  S_OS_PORT: String(PORT),
  SEMIO_PLUGIN: "s",
  SEMIO_RENDERER: "react",
  SEMIO_VITE_HMR: "0",
};

const scriptName =
  process.env.O2B_SCRIPT || readdirSync(OSDEV).find((n) => n.endsWith("script.ts"));
if (!scriptName && mode === "direct") throw new Error("no *script.ts in OSDEV");

const child =
  mode === "nx"
    ? spawn(
        "bun",
        [
          "nx",
          "run",
          "@semio-tech/framework-os-dev:serve-s-react-dev",
          "--",
          "--port",
          String(PORT),
          "--strictPort",
        ],
        { cwd: REPO, env, stdio: ["ignore", "pipe", "pipe"] },
      )
    : spawn(
        "bun",
        [
          `./${scriptName}`,
          "serve",
          "s",
          "react",
          "dev",
          "--port",
          String(PORT),
          "--strictPort",
        ],
        { cwd: OSDEV, env, stdio: ["ignore", "pipe", "pipe"] },
      );

writeFileSync(join(G, `warm-${mode}-p${PORT}-pid.txt`), String(child.pid));
const onChunk = (chunk) => {
  const s = String(chunk);
  out.write(s);
  if (/\[fresh\]/.test(s) && !marks.some((m) => m.label === "freshness")) mark("freshness/[fresh]");
  if (/\[stale\]/.test(s) && !marks.some((m) => m.label.startsWith("stale"))) mark("stale-line");
  if (/Re-optimizing dependencies/.test(s) && !marks.some((m) => m.label.includes("reopt"))) {
    mark(`vite-reopt: ${s.match(/Re-optimizing[^\n]+/)?.[0] ?? "?"}`);
  }
  if (/ready in (\d+) ms/.test(s) && !marks.some((m) => m.label.startsWith("vite-ready"))) {
    const m = s.match(/ready in (\d+) ms/);
    mark(`vite-ready ${m[1]}ms (vite internal)`);
  }
  if (/Local:\s+http/.test(s) && !marks.some((m) => m.label === "local-url")) mark("local-url");
  if (/lazy-activate/.test(s) && marks.filter((m) => m.label.startsWith("lazy:")).length < 8) {
    mark(`lazy: ${s.match(/\[lazy-activate\][^\n]+/)?.[0] ?? "hit"}`);
  }
};

child.stdout.on("data", onChunk);
child.stderr.on("data", onChunk);
child.on("exit", (code) => {
  if (!marks.some((m) => m.label.startsWith("child-exit"))) mark(`child-exit code=${code}`);
});

let http200 = null;
const deadline = Date.now() + 180_000;
while (Date.now() < deadline) {
  try {
    const res = await fetch(`http://127.0.0.1:${PORT}/`, { signal: AbortSignal.timeout(30_000) });
    if (res.status === 200) {
      const bytes = (await res.arrayBuffer()).byteLength;
      http200 = { status: 200, bytes, secs: (Date.now() - t0) / 1000 };
      mark(`HTTP 200 bytes=${bytes}`);
      break;
    }
  } catch {
    /* retry */
  }
  await sleep(50);
}

const meta = { marks, http200, pid: child.pid, port: PORT, mode, serveLog };
writeFileSync(join(G, `warm-${mode}-p${PORT}-meta.json`), JSON.stringify(meta, null, 2));
console.log(JSON.stringify(meta, null, 2));
await sleep(Number(process.env.O2B_HOLD_MS || 5000));
try {
  process.kill(child.pid, "SIGTERM");
} catch {
  /* gone */
}
await sleep(500);
process.exit(http200 ? 0 : 2);
