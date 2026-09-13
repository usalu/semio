/** ⌨️ Runtime proof for the `editor-verbs-cancel-undo` lane's keyboard surface and its undo step.
 *
 * Boots the procedural playground and drives ONLY the keyboard — no clicks on the verbs themselves —
 * for the five chords the editor declares (`mod+alt+d` cycleShowMode, `mod+alt+k` cycleLodMode,
 * `mod+shift+g` addGeneration, `mod+period` cancelPreviewEval, `mod+z`/`mod+shift+z` undo/redo),
 * reading each verb's OWN published evidence rather than a screenshot:
 *
 * - `cycleShowMode`/`cycleLodMode` — the preview window's `data-status-json` and the config the
 *   ladder writes, so a chord that lands on a mode the picker does not offer is visible.
 * - `addGeneration` + `mod+z` + `mod+shift+z` — the history chrome's own `data-history-json`
 *   (`canUndo`/`canRedo`/`cursor`), so the undo step is read off the framework's history, not
 *   inferred from geometry.
 *
 * 🪪️ A chord that reaches NOTHING is the finding, not a probe failure: every step records the
 * `performInvocation` console lines the host emits for it, so an unbound chord shows as zero lines.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=keys-1 bun 🐍️editor-verbs-keyboard-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", "editor-verbs", process.env.SEMIO_PROBE_OUT ?? "keys");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 180);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 2000)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`));

/** 📈️ Every surface the keyboard lane is written over, straight off the published DOM contracts. */
const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-status-json]")].map((el) => ({
    surfaceId: el.getAttribute("data-surface-id"),
    showMode: el.getAttribute("data-show-mode"),
    lodMode: el.getAttribute("data-lod-mode"),
    status: parse(el.getAttribute("data-status-json")),
  }));
  const history = document.querySelector("[data-history-json]");
  const cancel = document.querySelector('[data-slot="world-compute-cancel"]');
  return {
    hosts,
    history: history ? parse(history.getAttribute("data-history-json")) : null,
    cancelButton: cancel ? { action: cancel.getAttribute("data-cancel-action") } : null,
    modes: [...document.querySelectorAll("[data-show-mode],[data-lod-mode]")].map((el) => ({
      slot: el.getAttribute("data-slot"), show: el.getAttribute("data-show-mode"), lod: el.getAttribute("data-lod-mode"),
    })),
  };
});

const results = [];
const invocationsSince = (mark) => lines.slice(mark).filter((l) => l.includes("performInvocation") && !l.includes("settled")).map((l) => (l.match(/"actionId":"([^"]+)"/) ?? [])[1]).filter(Boolean);

const step = async (label, chord) => {
  const mark = lines.length;
  if (chord) {
    await page.keyboard.press(chord);
    await page.waitForTimeout(2500);
  }
  const s = await snap();
  const entry = { label, chord: chord ?? null, at: Date.now() - t0, invoked: invocationsSince(mark), history: s.history, modes: s.modes, cancelButton: s.cancelButton };
  results.push(entry);
  console.log(`[DEBUG] ${label} ${JSON.stringify(entry).slice(0, 900)}`);
  await page.screenshot({ path: join(outDir, `${results.length}-${label.replace(/[^a-z0-9]+/gi, "-")}.png`) });
  return s;
};

await page.goto(url, { waitUntil: "domcontentloaded" });

/** ⏳️ The playground boots ~20 wasm plugins; wait for the preview to publish a status at all. */
for (let i = 0; i < bootWait; i++) {
  const s = await snap();
  if (s.hosts.some((h) => h.status)) break;
  await page.waitForTimeout(1000);
}
await page.waitForTimeout(4000);
/** 🖱️ Focus the shell so chords reach the app, without clicking any verb under test.
 * `SEMIO_PROBE_FOCUS` picks the window to focus first: a keybinding for a WINDOW-OWNED action
 * (`window_kind_action_refs`) only resolves while that window has focus, so the focused window is
 * part of the result, not an incidental detail. */
const focusSelector = process.env.SEMIO_PROBE_FOCUS;
if (focusSelector) {
  const target = await page.$(focusSelector);
  const box = target ? await target.boundingBox() : null;
  if (!box) console.log(`[DEBUG] focus selector matched nothing clickable: ${focusSelector}`);
  else {
    await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
    console.log(`[DEBUG] focused ${focusSelector} at ${Math.round(box.x + box.width / 2)},${Math.round(box.y + box.height / 2)}`);
  }
} else await page.mouse.click(720, 500);
await page.waitForTimeout(1500);

/** ⌨️ `SEMIO_PROBE_CHORDS="label=Chord,label=Chord"` overrides the default walk, so a chord that
 * reached nothing can be retried under a different Playwright spelling without a new probe. */
const walk = process.env.SEMIO_PROBE_CHORDS
  ? process.env.SEMIO_PROBE_CHORDS.split(",").map((entry) => entry.split("="))
  : [["cycle-show-mode", "Control+Alt+d"], ["cycle-show-mode-again", "Control+Alt+d"], ["cycle-lod-mode", "Control+Alt+k"], ["add-generation", "Control+Shift+g"], ["undo", "Control+z"], ["redo", "Control+Shift+z"], ["cancel-preview-eval", "Control+."]];

await step("baseline", null);
for (const [label, chord] of walk) await step(label, chord);

writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] DONE ${results.length} steps; console ${lines.length} lines -> ${outDir}`);
await browser.close();
