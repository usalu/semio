/** 🩺️ Slice B3d — why 🏗️fem's `undo` does not retire an applied edit.
 *
 * The shared bar probe scores fem2d `mutated: true` (`create-node node=id=n12 x=3.5m y=4.5m↶`, edits
 * 0 → 1) and then `undone: false` with the edit count frozen at 1 for the whole 60 s budget, while
 * the same probe, same shell build, retires draw's and forms's edits in seconds. This isolates which
 * of the three undo routes the shell offers actually reaches the fem guest, and what the console
 * says while each one is pressed:
 *   A. the Actions rail's own `action.undo` row (what the bar probe presses),
 *   B. the History panel's `framework.history.undo` button,
 *   C. the applied ledger row's own revert affordance `framework.history.entry.<n>.revert`.
 *
 * Usage: bun 🐍️b3d-fem-undo-diagnose.mjs <variant> <port> <action> [key=value …]
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const OUT = join(dirname(fileURLToPath(import.meta.url)), "🗑️generated");
const [variant, port, action, ...rest] = process.argv.slice(2);
const args = Object.fromEntries(rest.filter((p) => p.includes("=")).map((p) => [p.slice(0, p.indexOf("=")), p.slice(p.indexOf("=") + 1)]));

const read = (page) => page.evaluate(() => {
  const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    checkin: text(document.querySelector("#s-checkin")),
    activeWindow: document.querySelector('[data-slot="window"][data-active="true"]')?.id
      ?? [...document.querySelectorAll('[data-slot="window"]')].map((el) => `${el.id}:${el.getAttribute("data-active")}`).join(","),
    windows: [...document.querySelectorAll('[data-slot="window"]')].map((el) => el.id),
    ledger: [...document.querySelectorAll('[id^="framework.history.entry."]')].map((el) => `${el.id}=${text(el).slice(0, 70)}`),
    undoRows: [...document.querySelectorAll('[id$="undo"], [id$="redo"]')].map((el) => `${el.id}|${el.tagName}|disabled=${el.getAttribute("aria-disabled") ?? el.disabled}`),
  };
});

mkdirSync(OUT, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1500)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`));
const steps = [];
const say = (name, detail) => {
  steps.push({ step: name, ms: Date.now() - t0, detail, consoleFrom: lines.length });
  console.log(name, JSON.stringify(detail).slice(0, 900));
};
const click = async (selector) => {
  if (!(await page.locator(selector).count())) return "absent";
  return page.locator(selector).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).split("\n")[0].slice(0, 110));
};

await page.goto(`http://127.0.0.1:${port}/?plugin=${variant}`, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 180; i++) {
  await page.waitForTimeout(1000);
  const shell = await read(page);
  if (shell.ready && i > 8) break;
}
say("boot", await read(page));

// 🧾️ The ledger and `#s-checkin` only exist once the History panel is mounted, so open it BEFORE
// anything is measured — the first run of this diagnose read an empty check-in and an empty ledger
// and could not tell the three routes apart at all.
await click('[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
await page.waitForTimeout(2000);
for (const toggle of await page.evaluate(() => [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((el) => el.id))) {
  await click(`[id="${toggle}"]`);
  await page.waitForTimeout(2500);
}
await page.waitForTimeout(1500);
await click(`[id="action.${action}"]`);
await page.waitForTimeout(1200);
for (const [key, value] of Object.entries(args)) {
  const input = page.locator(`[id$=".arg.${key}"]:is(input,textarea), [id$="${key}"]:is(input,textarea), [name="${key}"]`).first();
  if (await input.count()) await input.fill(String(value)).catch(() => {});
}
await page.waitForTimeout(400);
say("submit", { submitted: await click(`[id$=".action.${action}.execute"]`) });
await page.waitForTimeout(8000);
const mutated = await read(page);
say("mutated", mutated);

for (const [route, selector] of [
  ["A rail action.undo", '[id="action.undo"]'],
  ["B history button", '[id="framework.history.undo"] button, [id="framework.history.undo"]'],
  ["C ledger revert", '[id^="framework.history.entry."][id$=".revert"]'],
]) {
  const from = lines.length;
  const clicked = await click(selector);
  await page.waitForTimeout(12000);
  const after = await read(page);
  say(`undo ${route}`, { clicked, checkin: after.checkin, ledgerTail: after.ledger.slice(-3), console: lines.slice(from).slice(-14) });
  if (after.checkin !== mutated.checkin) { say("RETIRED BY", { route }); break; }
}

writeFileSync(join(OUT, `b3d-fem-undo-${variant}.txt`), [...steps.map((s) => `${s.ms} ${s.step} ${JSON.stringify(s.detail)}`), "", "# console", ...lines].join("\n"));
await browser.close();
