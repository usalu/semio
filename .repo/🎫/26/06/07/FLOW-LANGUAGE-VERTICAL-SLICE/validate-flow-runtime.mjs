/** @emoji 🧪 Flow play end-to-end runtime probe — slider, evaluate, preview. */
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
await page.waitForSelector('input[type="range"]', { timeout: 60_000 });
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
const unsupported = await page.getByText("Unsupported UiNode").count();
await browser.close();

console.log("[validate-flow] url:", baseUrl);
console.log("[validate-flow] debug logs:", debugLogs);
console.log("[validate-flow] preview before:", previewBefore);
console.log("[validate-flow] preview after slider:", previewAfter);
console.log("[validate-flow] unsupported nodes:", unsupported);

if (unsupported > 0) {
  console.error("[validate-flow] Unsupported UiNode rendered");
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
console.log("[validate-flow] ok");
