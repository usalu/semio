import { chromium } from "playwright";
import { ENTWERFEN_MIT_BESTAND_BRAND } from "../../../../../../framework/product/os/dev/brand/index.ts";
import { isEphemeralShellBrand, shouldPersistIntroductionSeen } from "../../../../../../framework/renderer/react/index.tsx";

const url = "http://127.0.0.1:6023/";
const polluteKeys = {
  "ui.chrome.appearance": "dark",
  "ui.chrome.layout": "tablet",
  "ui.chrome.compact": "true",
  "ui.chrome.expertise": "expert",
  "ui.chrome.locale": "en",
  "ui.chrome.terminology": "native",
  "ui.chrome.theme": "other",
  "ui.introduction.seen.entwerfen-mit-bestand:puzzle3d-play": "true",
  "semio.os.dock": JSON.stringify({ version: 3, anchors: { "top-left": [{ id: "polluted" }], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] } }),
  "semio.os.dock.puzzle3d-play": JSON.stringify({ version: 3, anchors: { "top-left": [{ id: "polluted-app" }], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] } }),
  "semio.os.dockUi": "{}",
  "semio.os.paneUi": "{}",
  "compose.display.layouts.puzzle3d-play": "[]",
};

console.log(`[DEBUG] brand.ephemeral=${ENTWERFEN_MIT_BESTAND_BRAND.ephemeral}`);
console.log(`[DEBUG] isEphemeral=${isEphemeralShellBrand(ENTWERFEN_MIT_BESTAND_BRAND)}`);
console.log(`[DEBUG] shouldPersistIntroduction=${shouldPersistIntroductionSeen(ENTWERFEN_MIT_BESTAND_BRAND)}`);

if (!ENTWERFEN_MIT_BESTAND_BRAND.ephemeral || !isEphemeralShellBrand(ENTWERFEN_MIT_BESTAND_BRAND) || shouldPersistIntroductionSeen(ENTWERFEN_MIT_BESTAND_BRAND)) {
  console.error("[DEBUG] ephemeral policy verification failed");
  process.exit(1);
}

const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage();
const debugLogs: string[] = [];
page.on("console", (msg) => {
  const text = msg.text();
  if (text.includes("[DEBUG]")) debugLogs.push(text);
});

await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120_000 });
await page.evaluate((keys) => {
  for (const [key, value] of Object.entries(keys)) localStorage.setItem(key, value);
}, polluteKeys);
await page.reload({ waitUntil: "networkidle", timeout: 120_000 });
await page.waitForTimeout(4000);

const snapshot = await page.evaluate(() => {
  const shellKey = (key: string) => key.startsWith("ui.chrome.") || key.startsWith("ui.introduction.seen.") || key.startsWith("ui.themes.") || key.startsWith("ui.compute.") || key.startsWith("semio.os.") || key.startsWith("compose.display.layouts.");
  const keys = Object.keys(localStorage).sort();
  return {
    remainingShellKeys: keys.filter(shellKey),
    title: document.title,
    hasWelcome: !!document.body.innerText.includes("Willkommen"),
  };
});

await page.reload({ waitUntil: "networkidle", timeout: 120_000 });
await page.waitForTimeout(4000);
const afterSecondRefresh = await page.evaluate(() => {
  const shellKey = (key: string) => key.startsWith("ui.chrome.") || key.startsWith("ui.introduction.seen.") || key.startsWith("ui.themes.") || key.startsWith("ui.compute.") || key.startsWith("semio.os.") || key.startsWith("compose.display.layouts.");
  return {
    hasWelcome: !!document.body.innerText.includes("Willkommen"),
    title: document.title,
    remainingShellKeys: Object.keys(localStorage).filter(shellKey).sort(),
  };
});

console.log(`[DEBUG] snapshot=${JSON.stringify(snapshot)}`);
console.log(`[DEBUG] afterSecondRefresh=${JSON.stringify(afterSecondRefresh)}`);
console.log(`[DEBUG] capturedLogs=${JSON.stringify(debugLogs.filter((line) => line.includes("ephemeral")))}`);

await browser.close();

const ephemeralLogged = debugLogs.some((line) => line.includes("ephemeral brand entwerfen-mit-bestand"));
const ok =
  ephemeralLogged &&
  afterSecondRefresh.hasWelcome &&
  afterSecondRefresh.title.includes("Aggregator") &&
  snapshot.hasWelcome &&
  snapshot.remainingShellKeys.length === 0 &&
  afterSecondRefresh.remainingShellKeys.length === 0;

if (!ok) {
  console.error("[DEBUG] runtime ephemeral verification failed");
  process.exit(1);
}
console.log("[DEBUG] runtime ephemeral verification passed — Aggregator clears and never keeps durable shell state across refresh");
