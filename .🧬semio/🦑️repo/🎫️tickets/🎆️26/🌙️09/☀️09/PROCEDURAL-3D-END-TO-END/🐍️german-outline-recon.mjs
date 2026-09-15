/** 🇩🇪️ Are the graph's OWN rows German where they are painted? `🐍️i18n-a11y-customization-probe.mjs`
 * reads `document.body.innerText` with the Artifact panel CLOSED, so `graph_nodes`/`graph_wires`/
 * `graph_input_port`/`graph_output_port`/`status_ok` were scored missing without ever being on screen.
 * This recon switches the locale, OPENS that panel (checking `data-active-tab-id` rather than trusting
 * a toggle press) and prints the section, node, port and status text in both locales.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6018/?plugin=generation3d bun 🐍️german-outline-recon.mjs
 * @see 🐍️i18n-a11y-customization-probe.mjs, 🗣️terminology/🦀️.rs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "react-reds/german");
mkdirSync(outDir, { recursive: true });
const lines = [];
const report = {};
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (m) => lines.push(`${m.type()} ${m.text().slice(0, 500)}`));
page.on("pageerror", (e) => lines.push(`pageerror ${String(e).slice(0, 500)}`));
const flush = () => { writeFileSync(join(outDir, "recon.json"), JSON.stringify(report, null, 2)); writeFileSync(join(outDir, "console.txt"), lines.join("\n")); };
const say = (key, value) => { report[key] = value; console.log(`[DEBUG] ${key} ${JSON.stringify(value).slice(0, 1600)}`); flush(); };

await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 180; i++) {
  if (await page.evaluate(() => [...document.querySelectorAll("[data-meshes-json]")].some((el) => (JSON.parse(el.getAttribute("data-meshes-json") || "[]") || []).length > 0))) break;
  await page.waitForTimeout(1000);
}

/** 🗂️ Presses the Artifact tab until the dock reports it active — the tab TOGGLES, so one blind press
 * can just as easily close the panel as open it. */
const openArtifact = async () => {
  for (let attempt = 0; attempt < 4; attempt += 1) {
    const active = await page.evaluate(() => document.querySelector('[data-slot="panel"][data-anchor="top-left"]')?.getAttribute("data-active-tab-id") ?? null);
    if (active === "framework.panel.artifact" && (await page.locator('[data-slot="panel"] [role="treeitem"]').count()) > 0) return true;
    await page.locator('button#framework\\.panel\\.artifact').first().click({ position: { x: 8, y: 11 }, timeout: 8000 }).catch(() => {});
    await page.waitForTimeout(2500);
  }
  return false;
};

const readOutline = () => page.evaluate(() => {
  const rows = [...document.querySelectorAll('[data-slot="panel"] [role="treeitem"]')];
  const text = (el) => (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 70);
  return {
    lang: document.documentElement.lang,
    rows: rows.length,
    anchors: [...document.querySelectorAll('[data-slot="panel"]')].map((el) => [el.getAttribute("data-anchor"), el.getAttribute("data-active-tab-id")]),
    sections: rows.filter((el) => /procedural-play-graph\.(nodes|wires)$/.test(el.id ?? "")).map((el) => [el.id, text(el)]),
    nodes: rows.filter((el) => /procedural-play-graph\/[^@]+$/.test(el.id ?? "")).slice(0, 4).map(text),
    ports: rows.filter((el) => /procedural-play-graph\/.*@/.test(el.id ?? "")).slice(0, 6).map(text),
    bodyText: document.body.innerText.replace(/\s+/gu, " ").slice(0, 1200),
  };
});

say("en", { opened: await openArtifact(), ...(await readOutline()) });
await page.screenshot({ path: join(outDir, "en.png") }).catch(() => {});

await page.locator("#framework\\.settings").first().click().catch(() => {});
await page.waitForTimeout(1500);
await page.locator("button#framework\\.settings\\.language").first().click().catch(() => {});
await page.waitForTimeout(900);
await page.locator("[role='option']").filter({ hasText: /Deutsch/ }).first().click().catch((e) => lines.push(`locale ${String(e).slice(0, 200)}`));
await page.waitForTimeout(8000);
say("de", { opened: await openArtifact(), ...(await readOutline()) });
await page.screenshot({ path: join(outDir, "de.png") }).catch(() => {});

flush();
await browser.close();
