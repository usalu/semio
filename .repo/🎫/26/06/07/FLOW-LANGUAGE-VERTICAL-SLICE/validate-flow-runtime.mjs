/** @emoji 🧪 Flow play end-to-end runtime probe — catalogue, drag-drop, evaluate, persistence. */
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

await page.goto(baseUrl, { waitUntil: "networkidle", timeout: 120_000 });
await page.waitForSelector('[data-testid="flow-catalogue"]', { timeout: 60_000 });
await page.waitForSelector('input[type="range"]', { timeout: 60_000 });

const sectionTitles = await page.locator('[data-testid="flow-catalogue"] summary').allTextContents();
const hasMath = sectionTitles.some((t) => /math/i.test(t));
const hasText = sectionTitles.some((t) => /text/i.test(t));
const hasLogic = sectionTitles.some((t) => /logic/i.test(t));
const hasInputs = sectionTitles.some((t) => /inputs/i.test(t));
const hasOutputs = sectionTitles.some((t) => /outputs/i.test(t));

await page.waitForFunction(() => {
  const strong = document.querySelector("strong.tabular-nums");
  return strong != null && strong.textContent !== "—" && strong.textContent.trim().length > 0;
}, { timeout: 60_000 });

const previewBefore = (await page.locator("strong.tabular-nums").first().textContent())?.trim() ?? "";
await page.locator('input[type="range"]').evaluate((el) => {
  const input = el;
  const setter = Object.getOwnPropertyDescriptor(window.HTMLInputElement.prototype, "value")?.set;
  setter?.call(input, "5");
  input.dispatchEvent(new Event("input", { bubbles: true }));
  input.dispatchEvent(new Event("change", { bubbles: true }));
});
await page.waitForFunction(
  (before) => {
    const text = document.querySelector("strong.tabular-nums")?.textContent?.trim() ?? "";
    return text !== "—" && text !== before;
  },
  previewBefore,
  { timeout: 15_000 },
);

const previewAfter = (await page.locator("strong.tabular-nums").first().textContent())?.trim() ?? "";

const addItem = page.locator('[data-testid="flow-catalogue-item-math.add"]');
await addItem.dragTo(page.locator("canvas"), { targetPosition: { x: 200, y: 200 } });
await page.waitForTimeout(500);

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
await page.waitForFunction(() => {
  const strong = document.querySelector("strong.tabular-nums");
  return strong?.textContent?.trim() === "7";
}, { timeout: 60_000 });
const previewPersisted = (await page.locator("strong.tabular-nums").first().textContent())?.trim() ?? "";

const unsupported = await page.getByText("Unsupported UiNode").count();
await browser.close();

console.log("[validate-flow] url:", baseUrl);
console.log("[validate-flow] sections:", sectionTitles);
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
