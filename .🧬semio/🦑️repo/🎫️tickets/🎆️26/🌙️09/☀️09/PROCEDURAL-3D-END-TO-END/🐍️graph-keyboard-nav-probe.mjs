/** 🧭️ Runtime proof for keyboard navigation of the node graph — keyboard ONLY, no pointer on any verb.
 *
 * Focuses the node-graph canvas (the `role="application"` shell the `react-i18n-a11y-customization`
 * lane added) and drives the five keyboard verbs the `graph-keyboard-nav-appearance-boot` lane
 * declares — `arrowdown`/`arrowup`/`arrowleft`/`arrowright` plus Enter on the canvas — then the two
 * that already existed (`delete`, `mod+z`) and `escape`.
 *
 * Every step is read off the app's OWN published evidence rather than a screenshot:
 *
 * - `invoked` — the `performInvocation` console lines the host emits, so a chord that reached NOTHING
 *   shows as zero lines and is the finding, not a probe failure.
 * - `inspection` — the Inspection panel's rendered rows, which are fed by the framework-owned `graph`
 *   selection (`marks.graph_selection_ids()`). That is the point of routing traversal through
 *   `Emit.interaction_writes`: the panel follows an arrow key the same way it follows a click.
 * - `outlineSelected` — any outline row the shell marks selected.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=keys-1 bun 🐍️graph-keyboard-nav-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", "graph-keyboard", process.env.SEMIO_PROBE_OUT ?? "keys");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 180);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 2000)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`));

const snap = () => page.evaluate(() => {
  const text = (el) => (el?.textContent ?? "").replace(/\s+/gu, " ").trim();
  const shells = [...document.querySelectorAll('[data-ui-surface-shell]')].map((el) => ({
    label: el.getAttribute("aria-label"),
    role: el.getAttribute("role"),
    tabIndex: el.getAttribute("tabindex"),
    keyshortcuts: el.getAttribute("aria-keyshortcuts"),
    focused: el === document.activeElement,
  }));
  return {
    shells,
    activeRole: document.activeElement?.getAttribute?.("role") ?? null,
    activeLabel: document.activeElement?.getAttribute?.("aria-label") ?? null,
    // 🔍️ The Inspection panel's OWN Id field, by element id. It is fed by the framework-owned `graph`
    // selection (`marks.graph_selection_ids()`), so it names WHICH node is selected — the one reading
    // that tells an arrow key that moved the selection from one that merely dispatched an action.
    // Scraping the panel's body text instead answered `null` on every step and made the step vacuous.
    inspection: [...document.querySelectorAll('[id$="procedural-play-inspector.id"], [id$="procedural-play-inspector.value"], [id$="procedural-play-inspector.range"]')].map((el) => `${el.id.split("/").pop()}=${text(el)}`).join(" ") || null,
    inspectorPresent: document.querySelectorAll('[id*="procedural-play-inspector"]').length,
    outlineSelected: [...document.querySelectorAll('[data-slot="window"][id="procedural-main"] [aria-selected="true"], [data-slot="window"][id="procedural-main"] [data-selected="true"], [data-slot="panel"] [aria-selected="true"]')].map(text).filter(Boolean).slice(0, 12),
    historyJson: (() => { const el = document.querySelector("[data-history-json]"); try { return el ? JSON.parse(el.getAttribute("data-history-json")) : null; } catch { return null; } })(),
  };
});

const results = [];
const invocationsSince = (mark) => lines.slice(mark).filter((l) => l.includes("performInvocation") && !l.includes("settled")).map((l) => (l.match(/"actionId":"([^"]+)"/) ?? [])[1]).filter(Boolean);

const step = async (label, key) => {
  const mark = lines.length;
  if (key) {
    await page.keyboard.press(key);
    await page.waitForTimeout(2200);
  }
  const s = await snap();
  const entry = { label, key: key ?? null, at: Date.now() - t0, invoked: invocationsSince(mark), activeRole: s.activeRole, activeLabel: s.activeLabel, inspection: s.inspection, inspectorPresent: s.inspectorPresent, outlineSelected: s.outlineSelected, history: s.historyJson };
  results.push(entry);
  console.log(`[DEBUG] ${label} ${JSON.stringify(entry).slice(0, 800)}`);
  await page.screenshot({ path: join(outDir, `${results.length}-${label.replace(/[^a-z0-9]+/gi, "-")}.png`) });
  return s;
};

await page.goto(url, { waitUntil: "domcontentloaded" });

/** ⏳️ The playground boots ~20 wasm plugins; wait for the node-graph canvas to exist at all. */
for (let i = 0; i < bootWait; i++) {
  const s = await snap();
  if (s.shells.some((shell) => shell.role === "application")) break;
  await page.waitForTimeout(1000);
}
await page.waitForTimeout(6000);

/** 🔍️ Open the Inspection panel FIRST: it is the app's own published view of the framework-owned
 * `graph` selection (`marks.graph_selection_ids()`), and therefore the observable that proves an
 * arrow key moved the same selection a click moves. It is a panel, not a window, so opening it does
 * not change which window kind owns the chords. */
const inspectionButton = page.locator("button#framework\\.panel\\.inspection");
if (await inspectionButton.count()) {
  /** 🎯️ Clicked at the tab's LEFT edge, not its centre: hovering the top-right panel's cap row reveals
   * the `Collapse` fold control (`button#framework.panel.top-right.fold`) over the trailing ~2/3 of this
   * tab, so a centre click collapses the dock instead of opening Inspection — measured with
   * `elementsFromPoint` in `🐍️panel-tab-occlusion-recon.mjs` and reported as an unfixed chrome defect.
   * The left edge is the tab's own label and resolves to the tab. */
  const box = await inspectionButton.first().boundingBox();
  console.log(`[DEBUG] inspection tab box ${JSON.stringify(box)}`);
  await inspectionButton.first().click({ position: { x: 8, y: Math.round((box?.height ?? 22) / 2) } });
  await page.waitForTimeout(2500);
  console.log("[DEBUG] opened the Inspection panel");
} else console.log("[DEBUG] no Inspection panel button found");

/** 🪟️ Window ownership is the chords' whole scope: the four arrow verbs are declared on the Flow
 * window kind, and `ShellHost` resolves a chord against the FOCUSED window kind's actions only. One
 * click on the canvas makes the flow window active AND focuses the `role="application"` shell; it is
 * the only pointer use in this probe and it touches no verb. */
const focused = await page.evaluate(() => {
  const shell = [...document.querySelectorAll('[data-ui-surface-shell]')].find((el) => /graph|Graph|Knoten/u.test(el.getAttribute("aria-label") ?? ""));
  if (!shell) return { found: false };
  const rect = shell.getBoundingClientRect();
  return { found: true, x: Math.round(rect.x + rect.width * 0.5), y: Math.round(rect.y + rect.height * 0.9), label: shell.getAttribute("aria-label") };
});
console.log(`[DEBUG] canvas ${JSON.stringify(focused)}`);
if (focused.found) {
  await page.mouse.click(focused.x, focused.y);
  await page.waitForTimeout(1500);
  await page.evaluate(() => {
    const shell = [...document.querySelectorAll('[data-ui-surface-shell]')].find((el) => /graph|Graph|Knoten/u.test(el.getAttribute("aria-label") ?? ""));
    shell?.focus?.();
  });
  await page.waitForTimeout(800);
}

await step("baseline", null);
for (const [label, key] of [
  ["arrow-down-1", "ArrowDown"],
  ["arrow-down-2", "ArrowDown"],
  ["arrow-down-3", "ArrowDown"],
  ["arrow-right-downstream", "ArrowRight"],
  ["arrow-left-upstream", "ArrowLeft"],
  ["arrow-up-previous", "ArrowUp"],
  ["enter-activate", "Enter"],
  ["arrow-right-after-activate", "ArrowRight"],
  ["escape-clear", "Escape"],
  ["arrow-down-reenter", "ArrowDown"],
  ["delete-selection", "Delete"],
  ["undo", "Control+z"],
]) {
  await step(label, key);
}

writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] DONE ${results.length} steps; console ${lines.length} lines -> ${outDir}`);
await browser.close();
