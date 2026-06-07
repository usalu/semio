/** @emoji 🧪 Flow node LOD probe — wheel zoom crosses dag draw bands. */
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

async function findFlowDevUrl(port) {
  const explicit = process.env.FLOW_PLAY_URL;
  if (explicit) return explicit;
  if (!(await isPortListening(port))) return null;
  try {
    const res = await fetch(`http://127.0.0.1:${port}/index.ts`);
    const body = await res.text();
    if (body.includes('PUZZLE_PLAY_ENTRY": "flow"') && body.includes("FlowPlayController")) {
      return `http://127.0.0.1:${port}/`;
    }
  } catch {
    return null;
  }
  return null;
}

const preferredPort = Number(process.env.FLOW_PLAY_PORT ?? "6016");
const baseUrl = await findFlowDevUrl(preferredPort);
if (!baseUrl) {
  console.error(`[validate-flow-lods] No flow dev server found near port ${preferredPort}. Run dev:flow first.`);
  process.exit(1);
}

const debugLogs = [];
const browser = await chromium.launch({
  headless: true,
  args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan"],
});
const page = await browser.newPage({ viewport: { width: 1280, height: 800 } });
page.on("console", (msg) => {
  const text = msg.text();
  if (text.includes("[DEBUG]")) debugLogs.push(text);
});

await page.goto(`${baseUrl}?probe=${Date.now()}`, { waitUntil: "domcontentloaded", timeout: 120_000 });
await page.evaluate(() => localStorage.removeItem("flow.fixture/v1"));
await page.reload({ waitUntil: "domcontentloaded", timeout: 120_000 });
const canvas = page.locator("canvas").first();
await canvas.waitFor({ timeout: 60_000 });
await page.waitForTimeout(2000);

const box = await canvas.boundingBox();
if (!box) {
  console.error("[validate-flow-lods] canvas bounding box missing");
  process.exit(1);
}
const cx = box.x + box.width * 0.5;
const cy = box.y + box.height * 0.5;
await page.mouse.move(cx, cy);
await canvas.dispatchEvent("wheel", { deltaY: 800 });
for (let i = 0; i < 30; i += 1) {
  await page.mouse.move(cx, cy);
  await page.mouse.wheel(0, 400);
  await page.waitForTimeout(60);
}
for (let i = 0; i < 60; i += 1) {
  await page.mouse.move(cx, cy);
  await page.mouse.wheel(0, -400);
  await page.waitForTimeout(60);
}

await page.waitForTimeout(500);
await browser.close();

const lodLogs = debugLogs.filter((line) => line.includes("dag draw lod="));
const bands = new Set(lodLogs.map((line) => {
  const match = line.match(/dag draw lod=(\w+)/);
  return match?.[1] ?? "";
}).filter(Boolean));

console.log("[validate-flow-lods] url:", baseUrl);
console.log("[validate-flow-lods] lod transitions:", lodLogs);
console.log("[validate-flow-lods] bands:", [...bands]);

if (lodLogs.length < 2) {
  console.error("[validate-flow-lods] expected multiple dag draw lod transitions");
  process.exit(1);
}
if (!bands.has("overview") && !bands.has("compact") && !bands.has("minimap")) {
  console.error("[validate-flow-lods] expected at least one far-zoom band (minimap/overview/compact)");
  process.exit(1);
}
if (!bands.has("normal") && !bands.has("detail") && !bands.has("micro")) {
  console.error("[validate-flow-lods] expected at least one near-zoom band (normal/detail/micro)");
  process.exit(1);
}
console.log("[validate-flow-lods] ok");
