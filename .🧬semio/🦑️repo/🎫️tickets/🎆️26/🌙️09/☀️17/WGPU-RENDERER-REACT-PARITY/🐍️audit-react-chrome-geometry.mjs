/** 🩺️ React chrome geometry dump for the wgpu/React visual-parity audit (📓️audit-visual-parity-puzzle3d.md).
 *
 * Loads the React puzzle3d playground, waits for readiness, optionally opens named panels by clicking
 * their navbar toggle, then walks every interactive/labelled chrome element (button/role=button/role=tab/
 * link/[data-*] chip) and records getBoundingClientRect + computed font/color/background/radius/padding,
 * plus its own trimmed text and a short ancestor class-chain (for region classification during analysis).
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6313/?plugin=puzzle3d \
 *   SEMIO_PROBE_LOCALSTORAGE='{"ui.introduction.seen.s.puzzle.puzzle3d@1/*#editor":"true"}' \
 *   bun 🐍️audit-react-chrome-geometry.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6313/?plugin=puzzle3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "audit-visual");
mkdirSync(outDir, { recursive: true });
const outFile = join(outDir, process.env.SEMIO_PROBE_OUT_FILE ?? "react-geometry.json");

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: Number(process.env.SEMIO_PROBE_DPR ?? 1) });
const seed = process.env.SEMIO_PROBE_LOCALSTORAGE;
if (seed) await page.addInitScript((entries) => { try { for (const [key, value] of Object.entries(entries)) localStorage.setItem(key, value); } catch {} }, JSON.parse(seed));

const consoleLines = [];
page.on("console", (msg) => consoleLines.push(`${msg.type()} ${msg.text()}`));
page.on("pageerror", (err) => consoleLines.push(`pageerror ${String(err)}`));

await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => document.documentElement.getAttribute("data-semio-os-ready") !== null, { timeout: 30000 }).catch(() => {});
await page.waitForTimeout(4000);

const dump = async (label) => {
  const data = await page.evaluate(() => {
    const isVisible = (el) => {
      const r = el.getBoundingClientRect();
      if (r.width <= 0 || r.height <= 0) return false;
      const style = getComputedStyle(el);
      if (style.visibility === "hidden" || style.display === "none" || Number(style.opacity) === 0) return false;
      return true;
    };
    const ownText = (el) => {
      let text = "";
      for (const node of el.childNodes) if (node.nodeType === Node.TEXT_NODE) text += node.textContent;
      return text.trim();
    };
    const classChain = (el, depth) => {
      const chain = [];
      let node = el;
      for (let i = 0; i < depth && node; i++) {
        const cls = typeof node.className === "string" ? node.className : (node.className?.baseVal ?? "");
        chain.push(`${node.tagName?.toLowerCase() ?? ""}${cls ? "." + cls.split(" ").filter(Boolean).slice(0, 3).join(".") : ""}${node.id ? "#" + node.id : ""}`);
        node = node.parentElement;
      }
      return chain.join(" < ");
    };
    const selector = "button, [role='button'], [role='tab'], [role='menuitem'], a[href], input, select, [data-slot], [data-role], [data-state], [data-semio-os-ready], [data-shell-id], [class*='chip' i], [class*='Chip' i], [class*='pill' i], [class*='badge' i]";
    const seen = new Set();
    const nodes = [...document.querySelectorAll(selector)];
    const rows = [];
    for (const el of nodes) {
      if (!isVisible(el)) continue;
      if (seen.has(el)) continue;
      seen.add(el);
      const r = el.getBoundingClientRect();
      const style = getComputedStyle(el);
      const text = ownText(el) || el.getAttribute("aria-label") || el.getAttribute("title") || el.textContent.trim().slice(0, 80);
      rows.push({
        tag: el.tagName.toLowerCase(),
        role: el.getAttribute("role"),
        text,
        rect: { x: Math.round(r.x * 100) / 100, y: Math.round(r.y * 100) / 100, w: Math.round(r.width * 100) / 100, h: Math.round(r.height * 100) / 100 },
        font: { size: style.fontSize, weight: style.fontWeight, family: style.fontFamily.split(",")[0] },
        color: style.color,
        background: style.backgroundColor,
        borderRadius: style.borderRadius,
        padding: style.padding,
        border: style.border,
        dataAttrs: Object.fromEntries([...el.attributes].filter((a) => a.name.startsWith("data-")).map((a) => [a.name, a.value])),
        ancestorChain: classChain(el, 5),
      });
    }
    const html = document.documentElement;
    return {
      ready: html.getAttribute("data-semio-os-ready"),
      error: html.getAttribute("data-semio-os-error"),
      title: document.title,
      bodyBg: getComputedStyle(document.body).backgroundColor,
      htmlBg: getComputedStyle(document.documentElement).backgroundColor,
      canvases: [...document.querySelectorAll("canvas")].map((c) => { const r = c.getBoundingClientRect(); return { w: c.width, h: c.height, rect: { x: r.x, y: r.y, w: r.width, h: r.height } }; }),
      rows,
    };
  });
  return { label, ...data };
};

const snapshots = {};
snapshots.initial = await dump("initial");
await page.screenshot({ path: join(outDir, "react-initial.png"), type: "png" });

// Try to open the Artifact panel and Inspection panel by clicking their navbar toggles (by visible text).
const clickByText = async (text) => {
  const loc = page.locator(`button:has-text("${text}"), [role='button']:has-text("${text}"), [role='tab']:has-text("${text}")`).first();
  if (await loc.count() > 0) {
    await loc.click({ timeout: 5000 }).catch(() => {});
    await page.waitForTimeout(1200);
    return true;
  }
  return false;
};

const artifactOpened = await clickByText("Artifact");
snapshots.artifactPanel = await dump("artifactPanel");
await page.screenshot({ path: join(outDir, "react-artifact-panel.png"), type: "png" });

const inspectionOpened = await clickByText("Inspection");
snapshots.inspectionPanel = await dump("inspectionPanel");
await page.screenshot({ path: join(outDir, "react-inspection-panel.png"), type: "png" });

writeFileSync(outFile, JSON.stringify({ url, artifactOpened, inspectionOpened, snapshots }, null, 2));
writeFileSync(join(outDir, "react-geometry-console.txt"), consoleLines.join("\n"));
console.log("[DEBUG] DONE rows(initial)=", snapshots.initial.rows.length, "artifactOpened=", artifactOpened, "inspectionOpened=", inspectionOpened, "ready=", snapshots.initial.ready, "title=", snapshots.initial.title);
await browser.close();
