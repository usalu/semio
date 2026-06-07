/** @emoji 🧪 DAG play end-to-end runtime probe — canvas mount + debug logs. */
import { chromium } from "@playwright/test";
import { createConnection } from "node:net";

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

async function findDagDevUrl(preferredPort) {
  const explicit = process.env.DAG_PLAY_URL;
  if (explicit) return explicit;
  for (let port = preferredPort; port < preferredPort + 20; port++) {
    if (!(await isPortListening(port))) continue;
    try {
      const res = await fetch(`http://127.0.0.1:${port}/index.ts`);
      const body = await res.text();
      if (body.includes('PUZZLE_PLAY_ENTRY": "dag"') && body.includes("DagPlayController")) {
        return `http://127.0.0.1:${port}/`;
      }
    } catch {
      /* try next */
    }
  }
  return null;
}

const preferredPort = Number(process.env.DAG_PLAY_PORT ?? "6017");
const baseUrl = await findDagDevUrl(preferredPort);
if (!baseUrl) {
  console.error(`[validate-dag] No dag dev server found near port ${preferredPort}. Run bun run dev:dag first.`);
  process.exit(1);
}

const debugLogs = [];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("console", (msg) => {
  const text = msg.text();
  if (text.includes("[DEBUG]")) debugLogs.push(text);
});

await page.goto(baseUrl, { waitUntil: "networkidle", timeout: 120_000 });
await page.waitForSelector("canvas", { timeout: 60_000 });
await page.waitForTimeout(1500);

const canvasCount = await page.locator("canvas").count();
const unsupported = await page.getByText("Unsupported UiNode").count();
await browser.close();

console.log("[validate-dag] url:", baseUrl);
console.log("[validate-dag] debug logs:", debugLogs);
console.log("[validate-dag] canvas count:", canvasCount);
console.log("[validate-dag] unsupported nodes:", unsupported);

if (unsupported > 0) {
  console.error("[validate-dag] Unsupported UiNode rendered");
  process.exit(1);
}
if (canvasCount < 1) {
  console.error("[validate-dag] expected at least one canvas");
  process.exit(1);
}
if (!debugLogs.some((l) => l.includes("dag canvas loaded fixture") || l.includes("dag play surface mount"))) {
  console.error("[validate-dag] missing dag debug log");
  process.exit(1);
}
console.log("[validate-dag] ok");
