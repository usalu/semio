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

function parseReorganizedFixtureFromLog(log) {
  const marker = "[DEBUG] dag canvas reorganized:";
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

function latestReorganizedFixtureFromLogs(logs) {
  for (let i = logs.length - 1; i >= 0; i -= 1) {
    const fixture = parseReorganizedFixtureFromLog(logs[i]);
    if (fixture) return fixture;
  }
  return null;
}

function nodeById(fixture, id) {
  return fixture?.nodes?.find((node) => node.id === id) ?? null;
}

async function triggerReorganizeEngagement(page) {
  const input = page.locator('[data-slot="engagement-command-input"] input').first();
  if (await input.count()) {
    await input.click({ force: true });
    await input.fill("reorganize");
    await input.press("Enter");
    await page.waitForTimeout(700);
    return;
  }
  const reorganizeRow = page.getByText("Reorganize", { exact: true }).first();
  if (await reorganizeRow.count()) {
    await reorganizeRow.click({ force: true });
    await page.waitForTimeout(700);
  }
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

function nodePorts(node, input) {
  if (node.kind === "computation") return input ? (node.inputs ?? []) : (node.outputs ?? []);
  if (input && node.kind === "screen") return [node.input];
  if (!input && (node.kind === "slider" || node.kind === "select")) return [node.output];
  return [];
}

function portScreenPx(fixture, nodeId, portId, input, viewW, viewH) {
  const node = fixture.nodes.find((n) => n.id === nodeId);
  if (!node) return null;
  const ports = nodePorts(node, input);
  const idx = ports.findIndex((p) => p.id === portId);
  if (idx < 0) return null;
  const t = (idx + 0.5) / Math.max(ports.length, 1);
  const hw = node.width * 0.5;
  const hh = node.height * 0.5;
  const worldX = input ? node.x - hw : node.x + hw;
  const worldY = node.y - hh + t * node.height;
  return { x: worldX + viewW * 0.5, y: worldY + viewH * 0.5 };
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

await pointerOnCanvas(canvas, "pointerdown", midX, midY);
await pointerOnCanvas(canvas, "pointerup", midX, midY);
await page.waitForTimeout(200);
const initialFixture = latestFixtureFromLogs(debugLogs);
const combineNode = initialFixture?.nodes?.find((n) => n.id === "combine");
if (combineNode) {
  const combineCenter = { x: (combineNode.x ?? 0) + box.width * 0.5, y: (combineNode.y ?? 0) + box.height * 0.5 };
  await dragOnCanvas(canvas, combineCenter, { x: combineCenter.x + 50, y: combineCenter.y + 30 });
}
const fixtureAfterNodeDrag = latestFixtureFromLogs(debugLogs);
const sliderNode = fixtureAfterNodeDrag?.nodes?.find((n) => n.id === "slider");
if (sliderNode?.kind === "slider") {
  const hw = (sliderNode.width ?? 180) * 0.5;
  const hh = (sliderNode.height ?? 80) * 0.5;
  const trackY = (sliderNode.y ?? 0) + hh * 0.2;
  const trackLeft = (sliderNode.x ?? 0) - hw + 14;
  const trackRight = (sliderNode.x ?? 0) + hw - 32;
  const from = { x: trackLeft + box.width * 0.5, y: trackY + box.height * 0.5 };
  const to = { x: trackRight + box.width * 0.5, y: trackY + box.height * 0.5 };
  await dragOnCanvas(canvas, from, to);
}
const combineB = fixtureAfterNodeDrag ? portScreenPx(fixtureAfterNodeDrag, "combine", "b", true, box.width, box.height) : null;
const scaleOut = fixtureAfterNodeDrag ? portScreenPx(fixtureAfterNodeDrag, "scale", "out", false, box.width, box.height) : null;
if (combineB && scaleOut) {
  await dragOnCanvas(canvas, combineB, scaleOut);
}

await page.locator("[data-dag-media-overlays] img").first().waitFor({ timeout: 10_000 });
const screenOverlayCount = await page.locator("[data-dag-media-overlays] img").count();

await triggerReorganizeEngagement(page);
await page.waitForTimeout(1500);
const reorganizedLog = debugLogs.some((l) => l.includes("dag canvas reorganized"));
const reorganizedFixture = latestReorganizedFixtureFromLogs(debugLogs);
const sliderNodeAfterLayout = nodeById(reorganizedFixture, "slider");
const screenNodeAfterLayout = nodeById(reorganizedFixture, "screen");

const canvasCount = await page.locator("canvas").count();
const unsupported = await page.getByText("Unsupported UiNode").count();
const fixture = latestFixtureFromLogs(debugLogs);
const nodeMoved = debugLogs.some((l) => l.includes("dag node moved"));
const edgeWired = debugLogs.some((l) => l.includes("dag edge connected") || l.includes("dag edge removed"));
const sliderChanged = debugLogs.some((l) => l.includes("dag slider value"));

await browser.close();

console.log("[validate-dag] url:", baseUrl);
console.log("[validate-dag] debug logs:", debugLogs);
console.log("[validate-dag] canvas count:", canvasCount);
console.log("[validate-dag] unsupported nodes:", unsupported);
console.log("[validate-dag] node moved log:", nodeMoved);
console.log("[validate-dag] edge wired log:", edgeWired);
console.log("[validate-dag] slider changed log:", sliderChanged);
console.log("[validate-dag] screen overlay count:", screenOverlayCount);
console.log("[validate-dag] reorganized log:", reorganizedLog);
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
if (!fixture || fixture.nodes?.length !== 5 || fixture.edges?.length !== 4) {
  console.error("[validate-dag] expected fixture snapshot with 5 nodes and 4 edges after interaction");
  process.exit(1);
}
if (!sliderChanged) {
  console.error("[validate-dag] expected slider drag to emit dag slider value debug log");
  process.exit(1);
}
if (screenOverlayCount < 1) {
  console.error("[validate-dag] expected at least one screen media overlay element");
  process.exit(1);
}
if (!nodeMoved && !reorganizedLog) {
  console.error("[validate-dag] expected node drag log or reorganize layout update");
  process.exit(1);
}
if (!edgeWired) {
  console.error("[validate-dag] expected edge reconnect to emit dag edge connected/removed debug log");
  process.exit(1);
}
if (!reorganizedLog) {
  console.error("[validate-dag] expected reorganize engagement to emit dag canvas reorganized debug log");
  process.exit(1);
}
if (!sliderNodeAfterLayout || !screenNodeAfterLayout || !(screenNodeAfterLayout.x > sliderNodeAfterLayout.x + 1)) {
  console.error("[validate-dag] expected left-to-right reorganize to place screen right of slider", {
    sliderNodeAfterLayout,
    screenNodeAfterLayout,
  });
  process.exit(1);
}
console.log("[validate-dag] ok");
