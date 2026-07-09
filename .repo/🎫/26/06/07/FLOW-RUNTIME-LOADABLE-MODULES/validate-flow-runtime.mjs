/** @emoji 🧪 Flow play runtime probe — loadable modules activate/deactivate and catalogue updates. */
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
const browser = await chromium.launch({
  headless: true,
  args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan"],
});
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

async function openWorkbench(page) {
  const workbenchToggle = page.getByRole("radio", { name: "Workbench" });
  if (await workbenchToggle.count()) {
    const pressed = await workbenchToggle.getAttribute("aria-checked");
    if (pressed !== "true") {
      await workbenchToggle.click();
    }
  }
}

await page.goto(`${baseUrl}?probe=${Date.now()}`, { waitUntil: "domcontentloaded", timeout: 120_000 });
await page.evaluate(() => localStorage.removeItem("flow.fixture/v1"));
await page.reload({ waitUntil: "domcontentloaded", timeout: 120_000 });
const canvas = page.locator("canvas").first();
await canvas.waitFor({ timeout: 60_000 });
await waitForDebugLog(debugLogs, "flow evaluate preview: 3");
await waitForDebugLog(debugLogs, "flow extension activated: math");

await openWorkbench(page);
await page.waitForFunction(
  () => {
    const host = window.__flowExtensionHost;
    if (!host) return false;
    const sections = host.catalogueSections().map((section) => section.id);
    const operators = JSON.parse(host.kindInfosJson());
    const add = operators.find((operator) => operator.id === "math.add");
    const constructVector = operators.find((operator) => operator.id === "math.constructVector");
    return ["core", "dictionary", "list", "logic", "math", "text"].every((id) => sections.includes(id)) && add?.inputs?.map((input) => input.id).join(",") === "a,b" && constructVector?.inputs?.map((input) => input.id).join(",") === "x,y,z";
  },
  { timeout: 60_000 },
);

const hasMathBefore = await page.evaluate(() => {
  const host = window.__flowExtensionHost;
  if (!host) throw new Error("missing window.__flowExtensionHost");
  return host.catalogueSections().some((section) => section.id === "math");
});

const deactivated = await page.evaluate(async () => {
  const host = window.__flowExtensionHost;
  if (!host) throw new Error("missing window.__flowExtensionHost");
  await host.deactivate("math");
  return host.catalogueSections().some((section) => section.id === "math");
});
await page.waitForTimeout(700);

const hasMathAfterDeactivate = await page.evaluate(() => {
  const host = window.__flowExtensionHost;
  if (!host) throw new Error("missing window.__flowExtensionHost");
  return host.catalogueSections().some((section) => section.id === "math");
});

const reactivated = await page.evaluate(async () => {
  const host = window.__flowExtensionHost;
  if (!host) throw new Error("missing window.__flowExtensionHost");
  await host.activate("math");
  return host.catalogueSections().some((section) => section.id === "math");
});
await page.waitForTimeout(700);
await waitForDebugLog(debugLogs, "flow evaluate preview: 3", 15_000);

const previewAfterReactivate = latestPreviewFromLogs(debugLogs);
const unsupported = await page.getByText("Unsupported UiNode").count();
await browser.close();

console.log("[validate-flow] url:", baseUrl);
console.log("[validate-flow] hasMathBefore:", hasMathBefore);
console.log("[validate-flow] deactivated math in catalogue:", deactivated);
console.log("[validate-flow] hasMathAfterDeactivate:", hasMathAfterDeactivate);
console.log("[validate-flow] reactivated math in catalogue:", reactivated);
console.log("[validate-flow] previewAfterReactivate:", previewAfterReactivate);
console.log("[validate-flow] debug logs:", debugLogs);
console.log("[validate-flow] unsupported nodes:", unsupported);

if (unsupported > 0) {
  console.error("[validate-flow] Unsupported UiNode rendered");
  process.exit(1);
}
if (!hasMathBefore) {
  console.error("[validate-flow] expected math section in catalogue before deactivate");
  process.exit(1);
}
if (deactivated !== false) {
  console.error("[validate-flow] expected math to be removed from host catalogue after deactivate");
  process.exit(1);
}
if (hasMathAfterDeactivate) {
  console.error("[validate-flow] expected workbench catalogue to drop math kinds after deactivate");
  process.exit(1);
}
if (reactivated !== true) {
  console.error("[validate-flow] expected math to return to host catalogue after reactivate");
  process.exit(1);
}
if (previewAfterReactivate !== "3") {
  console.error(`[validate-flow] expected preview 3 after reactivate, got ${previewAfterReactivate}`);
  process.exit(1);
}
console.log("[validate-flow] ok");
