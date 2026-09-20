/** 🔁️ Slice PB3 — the FOUR-press history walk the shared bar probe does not do.
 *
 * `🐍️b3a-interaction-probe.mjs` scores mutate → undo → redo. 🖨️raster's trap (PB1 §1.2) lives on
 * the fold's early-return edge, which `redo` takes and which a SECOND `undo` re-enters through the
 * tail cache, so the press order this slice's brief names — paint → undo → redo → undo — is the one
 * that exercises both halves. Keyboard routes only: raster publishes no `action.undo` row and the
 * History panel's own buttons share a dock with the app panels (measured: "Element is not visible").
 *
 * Usage: bun 🐍️pb3-undo-redo-undo.mjs <variant> <port> <rowSelector>
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const OUT = join(dirname(fileURLToPath(import.meta.url)), "🗑️generated");
const [variant, port, rowSelector] = process.argv.slice(2);
const FAULT = /unreachable|trapped|\btrap\b|panicked|fault|refused|dropped action|not-ui-safe|missing-owned|invalid-args|unsupported|pageerror|Uncaught|dispatch-failed/i;
const NOISE = /staged plugin module\(s\) are behind their source|\[stale\]|Failed to load resource: the server responded with a status of 404|Download the (React|Vue) DevTools|typed-operation slots/;

const read = (page) => page.evaluate(() => {
  const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
  const checkin = text(document.querySelector("#s-checkin"));
  const match = checkin.match(/\((\d+)\)/);
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    edits: checkin === "" ? -1 : match ? Number(match[1]) : 0,
    ledger: [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((el) => !el.id.endsWith(".revert")).map((el) => `${el.id.split(".").pop()}:${text(el).slice(0, 48)}`).slice(-3),
    panels: [...document.querySelectorAll('[data-slot="panel"]')].filter((el) => el.offsetParent !== null).map((el) => el.id),
  };
});

mkdirSync(OUT, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 900)}`));
const steps = [];
const say = (step, detail) => { steps.push(`${Date.now() - t0} ${step} ${JSON.stringify(detail)}`); console.log(step, JSON.stringify(detail)); };
const click = async (selector) => {
  if (!(await page.locator(selector).count())) return "absent";
  return page.locator(selector).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).split("\n")[0].slice(0, 90));
};
const raiseHistory = async () => {
  const open = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"]')].filter((el) => el.offsetParent !== null).map((el) => el.id));
  if (open.some((id) => id.includes("framework.panel.history"))) return "already";
  return click('[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
};
const settle = async (predicate, budgetMs = 60_000) => {
  const deadline = Date.now() + budgetMs;
  let state = await read(page);
  while (Date.now() < deadline && !predicate(state)) { await page.waitForTimeout(500); state = await read(page); }
  await page.waitForTimeout(2500);
  return read(page);
};

await page.goto(`http://127.0.0.1:${port}/?plugin=${variant}`, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 180; i++) { await page.waitForTimeout(1000); if ((await read(page)).ready && i > 8) break; }
await page.waitForTimeout(4000);
say("boot", await read(page));

await raiseHistory();
await page.waitForTimeout(1500);
// 🧾️ The baseline edit count has to be taken BEFORE the row is pressed: reading it after the press
// scores the paint against a witness that already contains it (first run: `painted: false` on a run
// whose ledger carried `Add Layer↶`).
const start = await read(page);
await click('[data-slot="panel-tab-button"][id="framework.panel.artifact"], [id="framework.panel.artifact"]');
await page.waitForTimeout(1200);
const pressed = await click(rowSelector);
await page.waitForTimeout(1200);
await raiseHistory();
const mutatedState = await settle((next) => next.edits > start.edits);
say("paint", { pressed, edits: mutatedState.edits, ledger: mutatedState.ledger });

const walk = [];
for (const [name, key, wants] of [
  ["undo", "Meta+z", (next, from) => next.edits < from],
  ["redo", "Meta+Shift+z", (next, from) => next.edits > from],
  ["undo2", "Meta+z", (next, from) => next.edits < from],
]) {
  const from = (await read(page)).edits;
  await page.keyboard.press(key);
  const after = await settle((next) => wants(next, from));
  walk.push({ name, from, to: after.edits, ok: wants(after, from), ledger: after.ledger });
  say(name, walk.at(-1));
}

const faults = lines.filter((line) => FAULT.test(line) && !NOISE.test(line));
const summary = { variant, painted: mutatedState.edits > start.edits, walk: walk.map((w) => `${w.name}:${w.from}->${w.to}:${w.ok}`), allOk: mutatedState.edits > start.edits && walk.every((w) => w.ok), faultLines: faults.length };
writeFileSync(join(OUT, `pb3-undo-redo-undo-${variant}.txt`), [`# summary ${JSON.stringify(summary)}`, "", "## steps", ...steps, "", "## faults", ...faults, "", "## console", ...lines].join("\n"));
console.log("SUMMARY", JSON.stringify(summary));
await browser.close();
