/**
 * 🌳️ F1's DOM half: does a graph-outline ROW carry the selection the guest owns?
 *
 * The Artifact panel lists every node/port/wire of the Flow graph under the framework-owned `graph`
 * interaction domain. A user (and a screen reader) learns which node is selected ONLY if the row says
 * so — `aria-selected="true"`, `data-selected="true"` and a selected row style. This probe opens that
 * panel, walks the graph with the arrow keys, and reads the rows back after every press, so a verdict
 * names the row the shell actually marked rather than the verb that was dispatched.
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const URL = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6023/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "react-gaps2/outline-selection");
mkdirSync(outDir, { recursive: true });
const console_ = [];
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (message) => console_.push(`${message.type()} ${message.text()}`.slice(0, 400)));
page.on("pageerror", (error) => console_.push(`pageerror ${error.message}`.slice(0, 400)));
await page.goto(URL, { waitUntil: "domcontentloaded" });

const snap = () =>
  page.evaluate(() => {
    const rows = [...document.querySelectorAll('[data-slot="panel"] [role="treeitem"]')];
    const label = (el) => (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 40);
    return {
      rows: rows.length,
      selected: rows.filter((el) => el.getAttribute("aria-selected") === "true").map(label),
      dataSelected: rows.filter((el) => el.getAttribute("data-selected") === "true").map(label),
      styled: rows.filter((el) => el.getAttribute("data-selected") === "true").map((el) => getComputedStyle(el).backgroundColor),
      idle: rows.filter((el) => el.getAttribute("aria-selected") !== "true").length,
    };
  });

for (let i = 0; i < 90; i++) {
  const ready = await page.evaluate(() => Boolean(document.querySelector("button#framework\\.panel\\.artifact")));
  if (ready) break;
  await page.waitForTimeout(1000);
}
await page.waitForTimeout(6000);

const artifact = page.locator("button#framework\\.panel\\.artifact");
const box = await artifact.first().boundingBox();
await artifact.first().click({ position: { x: 8, y: Math.round((box?.height ?? 22) / 2) } });
await page.waitForTimeout(3000);

const canvas = await page.evaluate(() => {
  const shell = [...document.querySelectorAll("[data-ui-surface-shell]")].find((el) => /graph|Graph|Knoten/u.test(el.getAttribute("aria-label") ?? ""));
  if (!shell) return { found: false };
  const r = shell.getBoundingClientRect();
  return { found: true, x: Math.round(r.x + r.width * 0.5), y: Math.round(r.y + r.height * 0.9) };
});
if (canvas.found) await page.mouse.click(canvas.x, canvas.y);
await page.waitForTimeout(2000);

const steps = [];
steps.push({ step: "baseline: the outline is mounted with rows", ...(await snap()) });
for (const key of ["ArrowDown", "ArrowDown", "ArrowRight", "ArrowUp"]) {
  await page.keyboard.press(key);
  await page.waitForTimeout(2500);
  steps.push({ step: key, ...(await snap()) });
}
await page.keyboard.press("Escape");
await page.waitForTimeout(2500);
steps.push({ step: "Escape retires the mark", ...(await snap()) });

const marked = steps.filter((s) => s.selected.length > 0);
const distinct = new Set(marked.map((s) => s.selected.join("|")));
const cleared = steps.at(-1);
const result = {
  steps,
  verdicts: [
    { step: "the outline paints rows", ok: steps[0].rows > 0, detail: { rows: steps[0].rows } },
    { step: "a traversal marks a row aria-selected", ok: marked.length > 0, detail: { marked: marked.map((s) => ({ step: s.step, selected: s.selected })) } },
    { step: "the same row carries data-selected", ok: marked.every((s) => s.dataSelected.length === s.selected.length) && marked.length > 0, detail: marked.map((s) => s.dataSelected) },
    { step: "a marked row is visually distinct", ok: marked.every((s) => s.styled.every((color) => color !== "rgba(0, 0, 0, 0)")) && marked.length > 0, detail: marked.map((s) => s.styled) },
    { step: "traversal moves the mark between rows", ok: distinct.size > 1, detail: [...distinct] },
    { step: "exactly one row is marked at a time", ok: marked.every((s) => s.selected.length === 1), detail: marked.map((s) => s.selected.length) },
    { step: "Escape retires the mark", ok: cleared.selected.length === 0, detail: cleared.selected },
    { step: "no page errors", ok: !console_.some((line) => line.startsWith("pageerror")), detail: console_.filter((line) => line.startsWith("pageerror")).slice(0, 4) },
  ],
};
writeFileSync(join(outDir, "results.json"), JSON.stringify(result, null, 2));
writeFileSync(join(outDir, "console.txt"), console_.join("\n"));
for (const verdict of result.verdicts) console.log(`[DEBUG] ${verdict.ok ? "✓" : "✗"} ${verdict.step} ${JSON.stringify(verdict.detail).slice(0, 220)}`);
console.log(`[DEBUG] outline-selection ${result.verdicts.filter((v) => v.ok).length}/${result.verdicts.length}`);
await browser.close();
process.exit(result.verdicts.every((v) => v.ok) ? 0 : 1);
