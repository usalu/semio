/** 🔎 Recon: what locale/appearance/dock controls the React shell exposes on 6018, and what ARIA the
 * generation3d surfaces carry today. Read-only — clicks nothing that mutates the document. */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", "react-i18n-a11y", process.env.SEMIO_PROBE_OUT ?? "recon");
mkdirSync(outDir, { recursive: true });
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const log = [];
page.on("console", (m) => { if (/error|warn/i.test(m.type())) log.push(`${m.type()} ${m.text().slice(0, 400)}`); });
await page.goto(url, { waitUntil: "domcontentloaded" });

let booted = false;
for (let i = 0; i < 90; i++) {
  await page.waitForTimeout(1000);
  if (await page.locator('[data-surface-id="window:procedural-preview"]').count()) { booted = true; break; }
}
console.log("[DEBUG] booted", booted);
await page.waitForTimeout(6000);

const census = await page.evaluate(() => {
  const surfaces = [...document.querySelectorAll("[data-surface-id]")].map((el) => ({
    surfaceId: el.getAttribute("data-surface-id"),
    tag: el.tagName,
    role: el.getAttribute("role"),
    ariaLabel: el.getAttribute("aria-label"),
    ariaLive: el.getAttribute("aria-live"),
    tabIndex: el.getAttribute("tabindex"),
  }));
  const canvases = [...document.querySelectorAll("canvas")].map((el) => ({
    role: el.getAttribute("role"),
    ariaLabel: el.getAttribute("aria-label"),
    tabIndex: el.getAttribute("tabindex"),
    parentRole: el.parentElement?.getAttribute("role") ?? null,
    parentLabel: el.parentElement?.getAttribute("aria-label") ?? null,
    w: el.width, h: el.height,
  }));
  const selects = [...document.querySelectorAll("[id^='framework.settings']")].map((el) => ({ id: el.id, tag: el.tagName, text: (el.textContent || "").slice(0, 60) }));
  const buttons = [...document.querySelectorAll("button")].map((el) => ({ id: el.id, label: el.getAttribute("aria-label"), text: (el.innerText || "").replace(/\s+/g, " ").slice(0, 40) })).filter((b) => b.id || b.label || b.text);
  const nodeKeys = [...document.querySelectorAll("[data-ui-node-key]")].map((el) => el.getAttribute("data-ui-node-key"));
  return { surfaces, canvases, selects, buttons: buttons.slice(0, 120), nodeKeyCount: nodeKeys.length, bodyText: document.body.innerText.replace(/\s+/g, " ").slice(0, 4000) };
});

const cdp = await page.context().newCDPSession(page);
await cdp.send("Accessibility.enable");
const { nodes } = await cdp.send("Accessibility.getFullAXTree");
const axe = nodes.map((n) => ({
  role: n.role?.value,
  name: n.name?.value,
  ignored: n.ignored,
  props: Object.fromEntries((n.properties ?? []).map((p) => [p.name, p.value?.value])),
})).filter((n) => !n.ignored && n.role && n.role !== "none");
writeFileSync(join(outDir, "census.json"), JSON.stringify(census, null, 2));
writeFileSync(join(outDir, "axtree.json"), JSON.stringify(axe, null, 2));
writeFileSync(join(outDir, "console.txt"), log.join("\n"));
await page.screenshot({ path: join(outDir, "boot.png"), fullPage: false });
console.log("[DEBUG] surfaces", census.surfaces.length, "canvases", census.canvases.length, "settingsSelects", JSON.stringify(census.selects));
console.log("[DEBUG] canvases", JSON.stringify(census.canvases));
console.log("[DEBUG] bodyText head", census.bodyText.slice(0, 800));
console.log("DONE");
await browser.close();
