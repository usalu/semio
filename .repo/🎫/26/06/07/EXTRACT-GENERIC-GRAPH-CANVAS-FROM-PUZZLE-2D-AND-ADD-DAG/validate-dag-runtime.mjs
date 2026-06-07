/** @emoji 🧪 DAG play end-to-end runtime probe — canvas mount, drag, reconnect. */
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

async function findDagDevUrl(port) {
  const explicit = process.env.DAG_PLAY_URL;
  if (explicit) return explicit;
  if (!(await isPortListening(port))) return null;
  try {
    const res = await fetch(`http://127.0.0.1:${port}/index.ts`);
    const body = await res.text();
    if (body.includes('PUZZLE_PLAY_ENTRY": "dag"') && body.includes("DagPlayController")) {
      return `http://127.0.0.1:${port}/`;
    }
  } catch {
    return null;
  }
  return null;
}

function parseFixtureFromLog(log) {
  const marker = "[DEBUG] dag fixture after pointer:";
  const idx = log.indexOf(marker);
  if (idx < 0) return null;
  const jsonStart = log.indexOf("{", idx);
  if (jsonStart < 0) return null;
  try {
    return JSON.parse(log.slice(jsonStart));
  } catch {
    return null;
  }
}

function latestFixtureFromLogs(logs) {
  for (let i = logs.length - 1; i >= 0; i -= 1) {
    const fixture = parseFixtureFromLog(logs[i]);
    if (fixture) return fixture;
  }
  return null;
}

async function pointerOnCanvas(canvas, type, x, y) {
  const box = await canvas.boundingBox();
  if (!box) throw new Error("canvas missing bounding box");
  await canvas.dispatchEvent(type, {
    bubbles: true,
    clientX: box.x + x,
    clientY: box.y + y,
    pointerId: 1,
    pointerType: "mouse",
    isPrimary: true,
    button: type === "pointerup" ? 0 : 0,
    buttons: type === "pointerup" ? 0 : 1,
  });
}

async function dragOnCanvas(canvas, from, to) {
  await pointerOnCanvas(canvas, "pointerdown", from.x, from.y);
  await pointerOnCanvas(canvas, "pointermove", to.x, to.y);
  await pointerOnCanvas(canvas, "pointerup", to.x, to.y);
  await canvas.page().waitForTimeout(400);
}

const preferredPort = Number(process.env.DAG_PLAY_PORT ?? "6017");
const baseUrl = await findDagDevUrl(preferredPort);
if (!baseUrl) {
  console.error(`[validate-dag] No dag dev server found near port ${preferredPort}. Run bun run dev:dag first.`);
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

await page.goto(baseUrl, { waitUntil: "networkidle", timeout: 120_000 });
const canvas = page.locator("canvas").first();
await canvas.waitFor({ timeout: 60_000 });
await page.waitForTimeout(1500);

const box = await canvas.boundingBox();
if (!box) {
  console.error("[validate-dag] canvas has no layout box");
  process.exit(1);
}

const midX = box.width * 0.5;
const midY = box.height * 0.5;

await dragOnCanvas(canvas, { x: midX - 120, y: midY }, { x: midX - 60, y: midY - 50 });
await dragOnCanvas(canvas, { x: midX + 40, y: midY + 20 }, { x: midX - 40, y: midY - 30 });

const canvasCount = await page.locator("canvas").count();
const unsupported = await page.getByText("Unsupported UiNode").count();
const fixture = latestFixtureFromLogs(debugLogs);
const nodeMoved = debugLogs.some((l) => l.includes("dag node moved"));
const edgeWired = debugLogs.some((l) => l.includes("dag edge connected") || l.includes("dag edge removed"));

await browser.close();

console.log("[validate-dag] url:", baseUrl);
console.log("[validate-dag] debug logs:", debugLogs);
console.log("[validate-dag] canvas count:", canvasCount);
console.log("[validate-dag] unsupported nodes:", unsupported);
console.log("[validate-dag] node moved log:", nodeMoved);
console.log("[validate-dag] edge wired log:", edgeWired);
if (fixture) {
  console.log("[validate-dag] fixture nodes:", fixture.nodes?.length, "edges:", fixture.edges?.length);
}

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
if (!fixture || fixture.nodes?.length !== 6 || fixture.edges?.length !== 6) {
  console.error("[validate-dag] expected fixture snapshot with 6 nodes and 6 edges after interaction");
  process.exit(1);
}
if (!nodeMoved) {
  console.error("[validate-dag] expected node drag to emit dag node moved debug log");
  process.exit(1);
}
if (!edgeWired) {
  console.error("[validate-dag] expected edge reconnect to emit dag edge connected/removed debug log");
  process.exit(1);
}
console.log("[validate-dag] ok");
