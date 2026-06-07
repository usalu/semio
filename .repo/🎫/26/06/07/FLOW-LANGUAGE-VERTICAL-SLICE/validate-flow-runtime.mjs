/** @emoji 🧪 Flow play end-to-end runtime probe — workbench catalogue drag-drop, evaluate, persistence. */
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
  console.error(`[validate-flow] No flow dev server found near port ${preferredPort}. Run bun run dev:flow first.`);
  process.exit(1);
}

const debugLogs = [];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("console", (msg) => {
  const text = msg.text();
  if (text.includes("[DEBUG]")) debugLogs.push(text);
});

async function waitForDebugLog(logs, pattern, timeoutMs = 60_000) {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    if (logs.some((line) => line.includes(pattern))) return;
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
  throw new Error(`timeout waiting for debug log: ${pattern}`);
}

function latestPreviewFromLogs(logs) {
  for (let i = logs.length - 1; i >= 0; i -= 1) {
    const match = logs[i].match(/flow evaluate preview:\s*(\S+)/);
    if (match) return match[1];
  }
  return "";
}

await page.goto(`${baseUrl}?probe=${Date.now()}`, { waitUntil: "networkidle", timeout: 120_000 });
await page.evaluate(() => localStorage.removeItem("flow.fixture/v1"));
await page.reload({ waitUntil: "networkidle" });
const canvas = page.locator("canvas").first();
await canvas.waitFor({ timeout: 60_000 });
await waitForDebugLog(debugLogs, "flow evaluate preview: 3");

async function pointerOnCanvas(canvasLocator, type, x, y) {
  const box = await canvasLocator.boundingBox();
  if (!box) throw new Error("canvas missing bounding box");
  await canvasLocator.dispatchEvent(type, {
    bubbles: true,
    cancelable: true,
    clientX: box.x + x,
    clientY: box.y + y,
    pointerId: 1,
    pointerType: "mouse",
    isPrimary: true,
    button: 0,
    buttons: type === "pointerup" ? 0 : 1,
  });
}

const workbenchToggle = page.getByRole("radio", { name: "Workbench" });
if (await workbenchToggle.count()) {
  const pressed = await workbenchToggle.getAttribute("aria-checked");
  if (pressed !== "true") {
    await workbenchToggle.click();
  }
}

await page.waitForFunction(() => {
  const labels = [...document.querySelectorAll("[data-tree-item-label], .tree-item-label, button, span")].map((el) => el.textContent?.trim() ?? "");
  return labels.some((t) => /math/i.test(t)) && labels.some((t) => /inputs/i.test(t)) && labels.some((t) => /outputs/i.test(t));
}, { timeout: 60_000 });

const sectionLabels = await page.locator("body").innerText();
const hasMath = /math/i.test(sectionLabels);
const hasText = /text/i.test(sectionLabels);
const hasLogic = /logic/i.test(sectionLabels);
const hasInputs = /inputs/i.test(sectionLabels);
const hasOutputs = /outputs/i.test(sectionLabels);

const previewBefore = latestPreviewFromLogs(debugLogs) || "3";

async function adjustCanvasSlider(canvasLocator, deltaX, worldOffset = { x: 0, y: 0 }) {
  const box = await canvasLocator.boundingBox();
  if (!box) throw new Error("canvas missing bounding box");
  const cx = box.width / 2 + worldOffset.x;
  const cy = box.height / 2 + worldOffset.y;
  await pointerOnCanvas(canvasLocator, "pointerdown", cx - 30, cy);
  await pointerOnCanvas(canvasLocator, "pointermove", cx - 30 + deltaX, cy);
  await pointerOnCanvas(canvasLocator, "pointerup", cx - 30 + deltaX, cy);
  await new Promise((resolve) => setTimeout(resolve, 400));
}

await waitForDebugLog(debugLogs, "flow fixture layout:");
const layoutLine = debugLogs.find((entry) => entry.includes("flow fixture layout:"));
const layout = JSON.parse(layoutLine.replace(/^.*flow fixture layout:\s*/, ""));
const sliderWorld = layout.slider ?? { x: 40, y: 0 };
await adjustCanvasSlider(canvas, 90, sliderWorld);
await waitForDebugLog(debugLogs, "flow evaluate preview: 5", 15_000);
const previewAfter = latestPreviewFromLogs(debugLogs);

async function paletteDragToCanvas(page, sourceLocator, canvasLocator, target) {
  const sourceBox = await sourceLocator.boundingBox();
  const canvasBox = await canvasLocator.boundingBox();
  if (!sourceBox || !canvasBox) {
    throw new Error("palette drag: missing bounding box");
  }
  const fromX = sourceBox.x + sourceBox.width / 2;
  const fromY = sourceBox.y + sourceBox.height / 2;
  const toX = canvasBox.x + target.x;
  const toY = canvasBox.y + target.y;
  await page.mouse.move(fromX, fromY);
  await page.mouse.down();
  await page.mouse.move(fromX + 12, fromY + 12);
  await page.mouse.move(toX, toY);
  await page.mouse.up();
  await page.waitForTimeout(400);
}

const addRow = page.locator(".cursor-grab", { hasText: "Add" }).first();
await addRow.waitFor({ timeout: 60_000 });
await paletteDragToCanvas(page, addRow, canvas, { x: 200, y: 200 });

await page.evaluate(() => {
  localStorage.setItem("flow.fixture/v1", JSON.stringify({
    schema: "flow.fixture/v1",
    camera: { x: 0, y: 0, zoom: 1 },
    widgets: [
      { kind: "inputSlider", id: "slider", value: 7 },
      { kind: "neuron", id: "add", neuronKind: "math.add", params: {} },
      { kind: "outputPreview", id: "preview", preview: {} },
    ],
    synapses: [
      { id: "s1", from: "slider", to: "add" },
      { id: "s2", from: "add", to: "preview" },
    ],
    layout: { slider: { x: -200, y: 0 }, add: { x: 0, y: 0 }, preview: { x: 200, y: 0 } },
  }));
});
await page.reload({ waitUntil: "networkidle" });
await canvas.waitFor({ timeout: 60_000 });
await waitForDebugLog(debugLogs, "flow evaluate preview: 7");
const previewPersisted = latestPreviewFromLogs(debugLogs);

const unsupported = await page.getByText("Unsupported UiNode").count();
await browser.close();

console.log("[validate-flow] url:", baseUrl);
console.log("[validate-flow] sections:", { hasMath, hasText, hasLogic, hasInputs, hasOutputs });
console.log("[validate-flow] debug logs:", debugLogs);
console.log("[validate-flow] preview before:", previewBefore);
console.log("[validate-flow] preview after slider:", previewAfter);
console.log("[validate-flow] preview persisted:", previewPersisted);
console.log("[validate-flow] unsupported nodes:", unsupported);

if (unsupported > 0) {
  console.error("[validate-flow] Unsupported UiNode rendered");
  process.exit(1);
}
if (!hasMath || !hasText || !hasLogic || !hasInputs || !hasOutputs) {
  console.error("[validate-flow] catalogue sections incomplete", { hasMath, hasText, hasLogic, hasInputs, hasOutputs });
  process.exit(1);
}
if (!debugLogs.some((l) => l.includes("flow evaluate preview"))) {
  console.error("[validate-flow] missing evaluate debug log");
  process.exit(1);
}
if (previewBefore !== "3") {
  console.error(`[validate-flow] expected initial preview 3, got ${previewBefore}`);
  process.exit(1);
}
if (previewAfter !== "5") {
  console.error(`[validate-flow] expected preview 5 after slider, got ${previewAfter}`);
  process.exit(1);
}
if (previewPersisted !== "7") {
  console.error(`[validate-flow] expected persisted preview 7, got ${previewPersisted}`);
  process.exit(1);
}
if (!debugLogs.some((l) => l.includes("flow add widget"))) {
  console.error("[validate-flow] missing drag-drop add widget debug log");
  process.exit(1);
}
console.log("[validate-flow] ok");
