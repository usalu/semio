/** 🔍 Why the context menu's `menu.group.*` submenu is reachable by neither pointer nor arrow keys.
 *
 * `📓️window-gaps-followup-2026-09-14.md` §8 left `menus · export-formats` red and called the cause
 * undetermined: Playwright resolves `<button role="menuitem" id="exportDocument">` but `hover`/`click`
 * both time out and an arrow walk never gives it focus. This recon answers both halves with numbers
 * instead of a guess:
 *
 *   - for the submenu panel and the export row: `getBoundingClientRect`, and what `elementFromPoint`
 *     answers at the row's own centre (a hit-target mismatch is exactly what times a Playwright click out);
 *   - every ancestor's computed `overflow-x`/`overflow-y`/`z-index` up to the portal host, because a
 *     side-anchored submenu inside a scrolling parent is clipped away by the parent's own scrollport;
 *   - what `document.activeElement` is after open / ArrowDown / ArrowRight, because a menu that tracks
 *     its active row in React state only never moves DOM focus at all.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=react-sweep/submenu-recon bun 🐍️submenu-reach-recon.mjs
 * @see 🐍️menu-dump-probe.mjs, 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🟦️.tsx
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6023/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "submenu-recon");
mkdirSync(outDir, { recursive: true });
const MAIN = "window:procedural-main";
const lines = [];
const t0 = Date.now();
const report = { url, steps: [] };

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 600)}`));

const say = (step, detail) => {
  report.steps.push({ step, detail, t: Date.now() - t0 });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1200)}`);
};

await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 200; i += 1) {
  await page.waitForTimeout(1000);
  if ((await page.locator(`[data-surface-id="${MAIN}"]`).count()) > 0) break;
}
await page.waitForTimeout(8000);

const box = await page.locator(`[data-surface-id="${MAIN}"]`).first().boundingBox();
await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.82, { button: "right" });
await page.waitForTimeout(1500);

const groups = await page.evaluate(() => [...document.querySelectorAll('[role="menu"] [role="menuitem"]')].map((el) => el.id).filter((id) => id.startsWith("menu.group.")));
say("groups", { groups, activeElement: await page.evaluate(() => document.activeElement?.id ?? document.activeElement?.tagName ?? null) });

const probeGeometry = () =>
  page.evaluate(() => {
    const describe = (el) => {
      if (!el) return null;
      const r = el.getBoundingClientRect();
      return { id: el.id || null, tag: el.tagName, cls: (el.className ?? "").toString().slice(0, 120), rect: { x: Math.round(r.x), y: Math.round(r.y), w: Math.round(r.width), h: Math.round(r.height) } };
    };
    const row = document.getElementById("exportDocument");
    const ancestors = [];
    for (let el = row?.parentElement ?? null; el && ancestors.length < 12; el = el.parentElement) {
      const cs = getComputedStyle(el);
      ancestors.push({ ...describe(el), overflowX: cs.overflowX, overflowY: cs.overflowY, zIndex: cs.zIndex, position: cs.position, clipPath: cs.clipPath });
    }
    const r = row?.getBoundingClientRect();
    const cx = r ? r.x + r.width / 2 : 0;
    const cy = r ? r.y + r.height / 2 : 0;
    const stack = r ? [...document.elementsFromPoint(cx, cy)].slice(0, 5).map(describe) : [];
    return {
      row: describe(row),
      rowVisible: row ? getComputedStyle(row).visibility : null,
      rowInViewport: r ? r.width > 0 && r.height > 0 && r.x >= 0 && r.y >= 0 && r.right <= innerWidth && r.bottom <= innerHeight : false,
      hitAtCentre: stack[0] ?? null,
      hitStack: stack,
      ancestors,
      panelCount: document.querySelectorAll('[role="menu"]').length,
    };
  });

for (const group of groups) {
  await page.locator(`[id="${group}"]`).first().hover({ timeout: 4000 }).catch((e) => lines.push(`hover ${group} ${String(e).replace(/\s+/gu, " ").slice(0, 140)}`));
  await page.waitForTimeout(1200);
  const present = await page.locator('[id="exportDocument"]').count();
  if (present === 0) continue;
  say(`hovered-${group}`, await probeGeometry());
  const clickErr = await page
    .locator('[id="exportDocument"]')
    .first()
    .click({ timeout: 4000 })
    .then(() => null)
    .catch((e) => String(e).replace(/\s+/gu, " ").slice(0, 200));
  say(`click-${group}`, { clickErr });
  break;
}

await page.keyboard.press("Escape");
await page.waitForTimeout(600);
await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.82, { button: "right" });
await page.waitForTimeout(1500);
const focusWalk = [];
focusWalk.push({ after: "open", active: await page.evaluate(() => document.activeElement?.id ?? document.activeElement?.tagName ?? null), activeRow: await page.evaluate(() => document.querySelector('[role="menuitem"][data-active="true"]')?.id ?? null) });
for (const key of ["ArrowDown", "ArrowDown", "ArrowRight", "ArrowDown"]) {
  await page.keyboard.press(key);
  await page.waitForTimeout(400);
  focusWalk.push({ after: key, active: await page.evaluate(() => document.activeElement?.id ?? document.activeElement?.tagName ?? null), activeRow: await page.evaluate(() => document.querySelector('[role="menuitem"][data-active="true"]')?.id ?? null) });
}
say("focus-walk", { focusWalk });

await page.screenshot({ path: join(outDir, "menu.png") });
writeFileSync(join(outDir, "recon.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] SUBMENU RECON DONE -> ${outDir}`);
await browser.close();
