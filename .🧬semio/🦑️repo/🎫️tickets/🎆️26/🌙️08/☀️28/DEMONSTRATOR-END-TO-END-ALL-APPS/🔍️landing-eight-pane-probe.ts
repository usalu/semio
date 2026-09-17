/** 🔍️ Loads the demonstrator landing page once (desktop viewport), skips the intro, and reports when each of the
 * eight panes' first window mounts and whether the two new panes (energie/statik) paint content. Read-only. */
import { chromium } from "playwright";

const BASE = process.env.PROBE_BASE_URL ?? "http://127.0.0.1:6029/";
const BUDGET_MS = Number(process.env.PROBE_BUDGET_MS ?? 330_000);
const WINDOW_BY_PANE: Record<string, string> = {
  generator: "framework.window.proceduralMain", koordinator: "framework.window.cad", aggregator: "framework.window.puzzle3d",
  energie: "framework.window.energyModel3d", aussuchen: "framework.window.sourcing", bearbeiten: "framework.window.process3d",
  verfolgen: "framework.window.gis", statik: "framework.window.fem3dModel",
};
const browser = await chromium.launch({ args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const errors: string[] = [];
page.on("pageerror", (e) => errors.push("PAGEERROR " + String(e).slice(0, 300)));
page.on("console", (m) => { const t = m.text(); if (m.type() === "error" && !t.startsWith("[DEBUG]")) errors.push(t.slice(0, 300)); });
const t0 = Date.now();
await page.goto(BASE, { waitUntil: "domcontentloaded", timeout: 60_000 });
await page.waitForTimeout(8_000);
const skip = page.getByText("Überspringen", { exact: true });
if (await skip.count()) await skip.first().click({ force: true }).catch(() => {});
const cards = await page.$$eval("[data-demonstrator-pane-card]", (els) => els.map((e) => e.getAttribute("data-pane-id")));
console.log("cards:", cards.join(", "));
const seen = new Map<string, string>();
while (Date.now() - t0 < BUDGET_MS && [...seen.values()].filter((v) => v !== "booting").length < 8) {
  const shells = await page.$$eval("[data-shell-id]", (els) => els.map((e) => [e.getAttribute("data-shell-id"), e.hasAttribute("data-shell-ready") ? "ready" : e.hasAttribute("data-shell-error") ? "error:" + (e.getAttribute("data-shell-error") ?? "").slice(0, 120) : e.hasAttribute("data-shell-not-found") ? "not-found" : "booting"] as const));
  for (const [id, state] of shells) if (id && seen.get(id) !== state) { seen.set(id, state); console.log(`${((Date.now() - t0) / 1000).toFixed(0)}s ${id}: ${state}`); }
  await page.waitForTimeout(5_000);
}
const content = await page.evaluate(() => ({
  treeRows: document.querySelectorAll('[role="treeitem"]').length,
  zoneRows: document.querySelectorAll("[data-row-id]").length,
  canvases: document.querySelectorAll("canvas").length,
  world3dHosts: document.querySelectorAll(".semio-world-3d-host").length,
  energyIntro: !!document.body.innerText.match(/Energiemodell|Energie/),
  statikIntro: !!document.body.innerText.match(/Tragwerksmodell|Statik/),
  unknownWindows: [...new Set([...document.querySelectorAll('[id^="framework.window."]')].map((e) => e.id.split(".").slice(0, 3).join(".")))],
}));
console.log("final:", JSON.stringify(Object.fromEntries(seen)));
console.log("content:", JSON.stringify(content));
console.log("non-debug console errors:", errors.length);
for (const e of [...new Set(errors)].slice(0, 6)) console.log("  -", e);
await browser.close();
