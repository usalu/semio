/** 🪵️ Slice-B3d sourcing MODULE proof: which of the three `sourcing-module-*` guests actually reached
 * the session, read off the shell rather than off the activation receipt — the Pool window's own rows
 * carry the module each stock kind came from, and the boot ledger's `Set Contributions` entry is the
 * host handing those module payloads to the curation app.
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { writeFileSync } from "node:fs";

const OUT = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/b3d-sourcing-modules.txt";
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6081/?plugin=sourcing";
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 900)}`));
const moduleRequests = [];
page.on("requestfinished", (r) => { if (/plugin-modules/.test(r.url())) moduleRequests.push(decodeURIComponent(r.url().replace(/^https?:\/\/[^/]+/, "")).slice(0, 160)); });
page.setDefaultNavigationTimeout(180_000);
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 180_000 });
for (let i = 0; i < 300; i++) {
  await page.waitForTimeout(1000);
  const ready = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"));
  if (ready && i > 10) break;
}
await page.waitForTimeout(6000);
const shell = await page.evaluate(() => {
  const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    panes: [...document.querySelectorAll("[data-surface-id]")].map((el) => ({ id: el.getAttribute("data-surface-id"), text: text(el).slice(0, 2500) })),
    ledger: [...document.querySelectorAll('[id^="framework.history.entry."]')].map((el) => text(el).slice(0, 120)),
    combobox: [...document.querySelectorAll('[role="combobox"]')].map((el) => text(el)),
  };
});
const MODULES = ["beams", "slabs", "windows"];
const haystack = `${shell.panes.map((p) => p.text).join(" ")} ${moduleRequests.join(" ")}`.toLowerCase();
const verdict = Object.fromEntries(MODULES.map((m) => [m, haystack.includes(m)]));
writeFileSync(OUT, [`# sourcing modules ${url} ${new Date().toISOString()}`, `# verdict ${JSON.stringify(verdict)}`, "", "## shell", JSON.stringify(shell, null, 1), "", "## module requests", ...moduleRequests, "", "## console", ...lines].join("\n"));
console.log("VERDICT", JSON.stringify(verdict), "ready", shell.ready);
console.log("PANES", shell.panes.map((p) => `${p.id}:${p.text.slice(0, 300)}`).join("\n"));
await browser.close();
