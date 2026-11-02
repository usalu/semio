import { chromium } from "@playwright/test";

const base = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:6010/";
const response = await fetch(new URL("index.json", base));
if (!response.ok) throw new Error(`Index: ${response.status}`);
const index = await response.json() as { entries: Record<string, { type: string; title: string }> };
const entry = Object.entries(index.entries).find(([, value]) => value.type === "story" && value.title === "🖱️ui⚛️react/Button");
if (!entry) throw new Error("Button story missing");
const browser = await chromium.launch({ executablePath: process.env.STORYBOOK_CHROMIUM_PATH || undefined, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"] });
const errors: string[] = [];
const sockets: { url: string; received: number; largestFrame: number; errors: string[] }[] = [];
try {
  const page = await browser.newPage();
  page.on("pageerror", error => errors.push(error.message));
  page.on("response", response => { if (response.status() >= 400) errors.push(`${response.status()} ${response.url()}`); });
  page.on("websocket", socket => {
    const address = new URL(socket.url());
    const evidence = { url: address.origin + address.pathname, received: 0, largestFrame: 0, errors: [] as string[] };
    sockets.push(evidence);
    socket.on("framereceived", frame => {
      evidence.received++;
      evidence.largestFrame = Math.max(evidence.largestFrame, Buffer.byteLength(frame.payload));
    });
    socket.on("socketerror", error => evidence.errors.push(error));
  });
  await page.goto(`${base}?path=/story/${encodeURIComponent(entry[0])}`, { waitUntil: "domcontentloaded", timeout: 120000 });
  const iframe = await page.locator("#storybook-preview-iframe").elementHandle({ timeout: 120000 });
  const frame = await iframe?.contentFrame();
  if (!frame) throw new Error("Manager preview frame missing");
  await frame.waitForFunction(id => {
    const render = (globalThis as typeof globalThis & { __STORYBOOK_PREVIEW__?: { currentRender?: { id: string; phase: string } } }).__STORYBOOK_PREVIEW__?.currentRender;
    return render?.id === id && render.phase === "finished";
  }, entry[0], { timeout: 120000 });
  console.log(JSON.stringify({ manager: page.url(), storyFinished: entry[0], errors, sockets }, null, 2));
  if (errors.length || sockets.some(socket => socket.errors.length)) throw new Error("Manager browser errors");
} finally {
  await browser.close();
}
