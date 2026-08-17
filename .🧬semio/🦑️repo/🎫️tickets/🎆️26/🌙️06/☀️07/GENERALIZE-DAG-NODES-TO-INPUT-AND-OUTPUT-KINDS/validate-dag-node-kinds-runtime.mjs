/** @emoji 🧪️ DAG node kinds runtime probe — slider drag + screen overlay. */
import { spawn } from "node:child_process";
import { chromium } from "@playwright/test";
import { createConnection } from "node:net";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(ticketDir, "../../../../../..");
const validateScript = join(repoRoot, ".repo/🎫️/26/06/07/EXTRACT-GENERIC-GRAPH-CANVAS-FROM-PUZZLE-2D-AND-ADD-DAG/validate-dag-runtime.mjs");

async function isPortListening(port) {
  return new Promise((resolve) => {
    const socket = createConnection({ host: "127.0.0.1", port });
    socket.setTimeout(300);
    socket.once("connect", () => {
      socket.destroy();
      resolve(true);
    });
    socket.once("timeout", () => {
      socket.destroy();
      resolve(false);
    });
    socket.once("error", () => resolve(false));
  });
}

const port = Number(process.env.DAG_PLAY_PORT ?? "6017");
let dev = null;
if (!(await isPortListening(port))) {
  dev = spawn("bun", ["run", "dev:dag"], { cwd: repoRoot, stdio: "pipe", env: { ...process.env, DAG_PLAY_PORT: String(port) } });
  for (let i = 0; i < 60; i += 1) {
    if (await isPortListening(port)) break;
    await new Promise((r) => setTimeout(r, 1000));
  }
}

const proc = spawn("bun", [validateScript], {
  cwd: repoRoot,
  stdio: "inherit",
  env: { ...process.env, DAG_PLAY_PORT: String(port) },
});
const code = await new Promise((resolve) => proc.on("close", resolve));
if (dev) dev.kill("SIGTERM");
process.exit(code ?? 1);
