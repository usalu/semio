/** 🎯️ Wave B54 — what ONE mutation costs in the browser on the large brush-painted document, split by
 * hop, with the shell's runtime diagnostics armed before boot.
 *
 * Where B44 measured the phases of a mutation it could not actually establish a selection for (§6.2) and
 * B48 measured the refresh hop, this probe measures the MUTATION: it switches the example to Nakagin with
 * the self-verifying switch (B48 §6.3 — every inherited probe clicks blind and can report flagship numbers
 * taken on the one-object document), proves a world selection through `data-selection-json`, and then
 * times `deleteSelection` (the `Delete` key, the battery's own route) and a `setCamera` control against
 * the console tape, censusing:
 *
 * - `command ingress lane` → `command ingress settled` — the GUEST's own command turn;
 * - `applyHostEffects refresh` / `refreshUi lane` / `refreshUi sections` — the host's refresh decision;
 * - `b44.intake` / `b44.project` — host UI intake and projection, if those taps are live.
 *
 * ⚠️ Every fix of wave B54 is GUEST-side Rust (the publication run, the per-object scene invalidation, the
 * O(changed) history projection), so it rides the next wasm build. A run against wasm #61 is a BEFORE
 * measurement by construction.
 *
 * Run: `bun 🔍️b54-mutation-turns.ts [--port=6013] [--label=before] [--boot=240]`.
 * Output: `🗑️generated/b54-<stamp>-<label>.md`. Ticket 26/09/02/PUZZLE-3D-END-TO-END. */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const label = process.argv.find((a) => a.startsWith("--label="))?.slice(8) ?? "run";
const stamp = `${new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19)}-${label}`;
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6013";
const bootBudget = Number(process.argv.find((a) => a.startsWith("--boot="))?.slice(7) ?? "240") * 1000;
const mdPath = join(OUT, `b54-${stamp}.md`);

const t0 = Date.now();
const lines: string[] = [];
const log = (message: string) => {
  const row = `[${((Date.now() - t0) / 1000).toFixed(2)}s] ${message}`;
  lines.push(row);
  console.log(row);
};

const tape: { ms: number; text: string }[] = [];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
/** 🩺️ `runtimeDiagnosticsEnabled` resolves ONCE per page off this key, so it must be written before the
 * first module evaluates — an `evaluate` after `goto` is already too late (B48). */
await page.addInitScript(() => {
  try {
    globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
  } catch {
    /* a sandboxed tab throws on localStorage */
  }
});
page.on("console", (message) => tape.push({ ms: Date.now() - t0, text: message.text().slice(0, 1200) }));
page.on("pageerror", (error) => tape.push({ ms: Date.now() - t0, text: `pageerror: ${String(error).slice(0, 300)}` }));
const mark = () => tape.length;
const since = (from: number, pattern: RegExp) => tape.slice(from).filter((row) => pattern.test(row.text));
const census = (from: number, pattern: RegExp) => since(from, pattern).length;
const quote = (from: number, pattern: RegExp, take: number) =>
  since(from, pattern)
    .slice(0, take)
    .map((row) => `+${row.ms}ms ${row.text}`);

const instances = async () =>
  page
    .evaluate(() => {
      const element = document.querySelector("#puzzle3d-main-perspective [data-instances-json]") as HTMLElement | null;
      const raw = element?.getAttribute("data-instances-json") || "[]";
      let parsed: { id?: string }[] = [];
      try {
        parsed = JSON.parse(raw);
      } catch {
        parsed = [];
      }
      return { bytes: raw.length, count: parsed.length, ids: parsed.map((row) => row.id ?? "?") };
    })
    .catch(() => ({ bytes: 0, count: 0, ids: [] as string[] }));

/** 🔦️ The pane's PAINTED selection — `data-selection-json` (wave B20's `worldSurfaceSelectionDomV1`), the
 * one reader every battery selection verdict uses. */
const painted = async () =>
  page
    .evaluate(() =>
      Array.from(document.querySelectorAll("[data-selection-json]")).flatMap((element) => {
        try {
          const value = JSON.parse(element.getAttribute("data-selection-json") || "{}") as { selectedIds?: string[] };
          return value.selectedIds ?? [];
        } catch {
          return [] as string[];
        }
      }),
    )
    .catch(() => [] as string[]);

const cameraOf = async () =>
  page.evaluate(() => document.querySelector("#puzzle3d-main-perspective [data-camera-json]")?.getAttribute("data-camera-json") ?? null).catch(() => null);

const settle = async <T>(read: () => Promise<T>, done: (value: T) => boolean, budgetMs: number) => {
  const started = Date.now();
  let value = await read();
  while (!done(value) && Date.now() - started < budgetMs) {
    await page.waitForTimeout(300);
    value = await read();
  }
  return { value, waitedMs: Date.now() - started, ok: done(value) };
};

log(`goto http://127.0.0.1:${port}/ diagnostics=armed`);
await page.goto(`http://127.0.0.1:${port}/`, { waitUntil: "domcontentloaded", timeout: 120_000 });
const booted = await settle(instances, (value) => value.count > 0, bootBudget);
log(`boot instances=${booted.value.count} bytes=${booted.value.bytes} waitedMs=${booted.waitedMs}`);
if (!booted.ok) {
  log(`boot FAILED errors=${JSON.stringify(tape.filter((row) => /error|fault|panic|does not provide an export|failed to fetch|500/i.test(row.text)).slice(0, 20).map((row) => `+${row.ms}ms ${row.text}`)).slice(0, 5000)}`);
  writeFileSync(mdPath, `# b54 ${stamp}\n\n\`\`\`\n${lines.join("\n")}\n\`\`\`\n`);
  await browser.close();
  process.exit(1);
}

/** 🔀️ The example picker is a `<button role="combobox">` over a portalled listbox, never a `<select>`, and
 * a hit-tested click on it times out while the element is neither disabled nor covered (B44 §1.0) — so the
 * switch forces the click, WAITS for the options to exist, and verifies the picker's own label changed. A
 * blind click-and-hope switch is what made a whole battery measure the one-object document while reporting
 * on the flagship (B48 §6.3). */
const pickerLabel = async () => page.locator('[id="playground.navbar.fixture"]').first().innerText().catch(() => "");
const switchExample = async (pattern: RegExp) => {
  for (let attempt = 0; attempt < 4; attempt += 1) {
    await page.keyboard.press("Escape").catch(() => {});
    await page.locator('[id="playground.navbar.fixture"]').first().click({ timeout: 6000, force: true }).catch(() => {});
    const option = page.locator('[role="option"]').filter({ hasText: pattern }).first();
    const appeared = await option.waitFor({ state: "visible", timeout: 8000 }).then(() => true).catch(() => false);
    if (!appeared) continue;
    await option.click({ timeout: 6000 }).catch(() => option.click({ timeout: 6000, force: true }).catch(() => {}));
    const settled = await settle(pickerLabel, (text) => pattern.test(text), 10_000);
    log(`example-switch attempt=${attempt} pickerLabel=${JSON.stringify(settled.value)} verified=${settled.ok}`);
    if (settled.ok) return true;
  }
  return false;
};

const switchMark = mark();
const switched = await switchExample(/nakagin/i);
const swapped = await settle(instances, (value) => value.count >= 100, 180_000);
log(`example-switch switched=${switched} instances=${swapped.value.count} bytes=${swapped.value.bytes} waitedMs=${swapped.waitedMs}`);
log(`example-switch guestCommandTurn=${JSON.stringify(quote(switchMark, /command ingress (lane|settled)|performInvocation settled/, 6)).slice(0, 2000)}`);
if (!swapped.ok) {
  log(`example-switch FAILED tail=${JSON.stringify(quote(switchMark, /error|fault|panic|setActiveExample/i).slice(-12)).slice(0, 4000)}`);
  writeFileSync(mdPath, `# b54 ${stamp}\n\n\`\`\`\n${lines.join("\n")}\n\`\`\`\n`);
  await browser.close();
  process.exit(1);
}

/** 🖐️ One world pick per pane fraction until `data-selection-json` names an id — the same route the
 * battery's `ensureWorldSelection` takes, with every attempt's wait recorded so "no selection" and "a slow
 * selection" can never read the same. */
/** 📐️ The pane rect is read through `getBoundingClientRect` rather than Playwright's `boundingBox()`:
 * that call runs an actionability wait and TIMED OUT at 30 s on this pane (2026-09-13) while the locator
 * itself resolved to a visible canvas — the same obstruction class as B44 §1.0's navbar trigger. Every
 * pointer event below is then a raw `page.mouse` event at absolute coordinates, which skips the
 * actionability check entirely. */
const box = await page
  .evaluate(() => {
    const element = Array.from(document.querySelectorAll("#puzzle3d-main-perspective canvas")).pop();
    if (!element) return null;
    const rect = element.getBoundingClientRect();
    return { x: rect.x, y: rect.y, width: rect.width, height: rect.height };
  })
  .catch(() => null);
log(`perspective canvas box=${JSON.stringify(box)}`);
let selected: string[] = [];
if (box) {
  for (const fraction of [
    { x: 0.5, y: 0.5 },
    { x: 0.45, y: 0.6 },
    { x: 0.55, y: 0.4 },
    { x: 0.5, y: 0.68 },
    { x: 0.42, y: 0.48 },
  ]) {
    if (selected.length) break;
    const pickMark = mark();
    await page.mouse.click(box.x + box.width * fraction.x, box.y + box.height * fraction.y).catch(() => {});
    const landed = await settle(painted, (ids) => ids.length > 0, 45_000);
    log(
      `pick ${fraction.x.toFixed(2)},${fraction.y.toFixed(2)} ids=${JSON.stringify(landed.value).slice(0, 200)} waitedMs=${landed.waitedMs}` +
        ` ingress=${census(pickMark, /actionId":"interactionSelect"/)} settled=${census(pickMark, /performInvocation settled.*interactionSelect/)} refresh=${census(pickMark, /applyHostEffects refresh/)} lane=${census(pickMark, /refreshUi lane/)}`,
    );
    selected = landed.value;
  }
}
log(`selection proven=${selected.length > 0} ids=${JSON.stringify(selected).slice(0, 200)}`);

/** 📷️ The control: a camera move reads no object at all, so whatever it costs is the per-command constant. */
const cameraMark = mark();
const cameraBefore = await cameraOf();
await page.mouse.move((box?.x ?? 0) + (box?.width ?? 600) * 0.5, (box?.y ?? 0) + (box?.height ?? 400) * 0.5).catch(() => {});
await page.mouse.wheel(0, -240).catch(() => {});
const camera = await settle(cameraOf, (value) => value !== cameraBefore, 45_000);
log(
  `setCamera moved=${camera.ok} waitedMs=${camera.waitedMs} ingress=${census(cameraMark, /actionId":"setCamera"/)} settled=${census(cameraMark, /performInvocation settled.*setCamera/)} refresh=${census(cameraMark, /applyHostEffects refresh/)}`,
);

/** 🗑️ The mutation, on the proven selection: `Delete`, exactly the battery's `delete-selection` route. */
const beforeDelete = await instances();
const deleteMark = mark();
await page.keyboard.press("Delete").catch(() => {});
let removal = await settle(instances, (value) => value.count < beforeDelete.count, 45_000);
if (!removal.ok) {
  await page.keyboard.press("Backspace").catch(() => {});
  const retry = await settle(instances, (value) => value.count < beforeDelete.count, 45_000);
  removal = { value: retry.value, waitedMs: removal.waitedMs + retry.waitedMs, ok: retry.ok };
}
log(
  `deleteSelection landed=${removal.ok} before=${beforeDelete.count} after=${removal.value.count}` +
    ` removed=${JSON.stringify(beforeDelete.ids.filter((id) => !removal.value.ids.includes(id))).slice(0, 160)} waitedMs=${removal.waitedMs}`,
);
log(
  `deleteSelection census ingress=${census(deleteMark, /actionId":"deleteSelection"/)} settled=${census(deleteMark, /performInvocation settled.*deleteSelection/)}` +
    ` refresh=${census(deleteMark, /applyHostEffects refresh/)} lane=${census(deleteMark, /refreshUi lane/)} sections=${census(deleteMark, /refreshUi sections/)}` +
    ` intake=${census(deleteMark, /b44\.intake/)} project=${census(deleteMark, /b44\.project/)} moreWork=${census(deleteMark, /more-work/)} lines=${tape.length - deleteMark}`,
);
log(`deleteSelection ingressTape=${JSON.stringify(quote(deleteMark, /command ingress|performInvocation settled/, 8)).slice(0, 2400)}`);
log(`deleteSelection refreshTape=${JSON.stringify(quote(deleteMark, /applyHostEffects refresh|refreshUi lane/, 8)).slice(0, 2400)}`);
log(`deleteSelection faults=${JSON.stringify(quote(deleteMark, /fault|panic|refused|rejected|unreachable/i, 8)).slice(0, 2400)}`);

await page.screenshot({ path: join(OUT, `b54-${stamp}.png`) }).catch(() => {});
writeFileSync(mdPath, `# b54 ${stamp}\n\n## Timeline\n\n\`\`\`\n${lines.join("\n")}\n\`\`\`\n`);
log(`wrote ${mdPath}`);
await browser.close();
