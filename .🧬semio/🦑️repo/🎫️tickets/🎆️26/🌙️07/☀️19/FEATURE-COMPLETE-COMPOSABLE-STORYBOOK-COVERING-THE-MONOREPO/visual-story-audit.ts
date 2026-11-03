import { chromium } from "@playwright/test";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const directory = resolve(dirname(fileURLToPath(import.meta.url)), "🗑️generated", process.env.VISUAL_AUDIT_DIRECTORY ?? "visual-before");
mkdirSync(directory, { recursive: true });
const base = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:6010/";
const index = await (await fetch(new URL("index.json", base))).json() as { entries: Record<string, { type: string; title: string }> };
const subjects = (process.env.VISUAL_AUDIT_SUBJECTS ?? "Badge,Label,ActionGroup,PresenceBar,TableAvatar").split(",");
const browser = await chromium.launch({ executablePath: process.env.STORYBOOK_CHROMIUM_PATH || undefined, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"] });
try {
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
  for (const subject of subjects) {
    const entries = Object.entries(index.entries).filter(([, entry]) => entry.type === "story" && entry.title === `🖱️ui⚛️react/${subject}`);
    const entry = entries.find(([id]) => id.endsWith("--default")) ?? entries[0];
    if (!entry) throw new Error(`Missing ${subject}`);
    await page.goto(`${base}iframe.html?id=${encodeURIComponent(entry[0])}&viewMode=story`, { waitUntil: "domcontentloaded", timeout: 120000 });
    await page.waitForFunction(id => {
      const render = (globalThis as typeof globalThis & { __STORYBOOK_PREVIEW__?: { currentRender?: { id: string; phase: string } } }).__STORYBOOK_PREVIEW__?.currentRender;
      return render?.id === id && render.phase === "finished";
    }, entry[0], { timeout: 120000 });
    await page.evaluate(() => document.fonts.ready);
    const elements = await page.locator("#storybook-root").evaluate(root => Array.from(root.querySelectorAll("label,input,button,svg,img,[data-slot]")).flatMap(element => {
      const box = element.getBoundingClientRect();
      if (!box.width || !box.height) return [];
      const style = getComputedStyle(element);
      return [{ tag: element.tagName, slot: element.getAttribute("data-slot"), text: element.textContent?.slice(0, 90), class: element.getAttribute("class"), x: box.x, y: box.y, width: box.width, height: box.height, padding: style.padding, gap: style.gap, lineHeight: style.lineHeight, fontSize: style.fontSize, display: style.display, alignItems: style.alignItems }];
    }).slice(0, 180));
    for (const extension of ["json", "png"]) if (resolve(directory, `${subject}.${extension}`).length > 255) throw new Error("Artifact path too long");
    writeFileSync(resolve(directory, `${subject}.json`), JSON.stringify({ id: entry[0], elements }, null, 2));
    await page.screenshot({ path: resolve(directory, `${subject}.png`), animations: "disabled" });
    console.log(`${subject}: ${entry[0]}, ${elements.length} measured nodes`);
  }
} finally { await browser.close(); }
