/** 🎯️ Wave B48 — WHY the guest's selection lane never republishes after a pick on the 180-object
 * Nakagin document, and why `registerBrushMesh` never drains.
 *
 * Where B46 measured the OUTCOME (`data-guest-selection-json` stays `selectedIds:[]` for 150 s), this
 * probe measures the HOP. It arms the shell's own runtime diagnostics in `localStorage` BEFORE the page
 * boots, so the three permanent taps on the refresh lane print for every dispatch:
 *
 * - `[DEBUG] applyHostEffects refresh` — the scope the GUEST declared (`declared`) and the scope the
 *   host actually refreshes with (`scope`), per dispatch.
 * - `[DEBUG] refreshUi lane` — the coalescer's decision (`owed`/`merged`/`pass`/`failed`) and pass count.
 * - `[DEBUG] refreshUi sections` — `asked` (the window instance keys the request carried) against
 *   `changed` (the ones the guest answered with a NEW value), plus each one's hash.
 *
 * Together those three separate the four readings the defect could have: the guest declared the wrong
 * scope, the host dropped it, the host asked for the world body and the guest answered "unchanged", or
 * the host never asked at all.
 *
 * It also counts `registerBrushMesh` ingress per 10-second bucket across the whole run, so
 * "the announce converges" is a curve rather than a single total.
 *
 * Run: `bun 🔍️b48-selection-refresh.ts [--port=6013] [--label=before] [--settle=25] [--census=60]`.
 * Output: `🗑️generated/b48-<stamp>-<label>.md` / `.ndjson`. Ticket 26/09/02/PUZZLE-3D-END-TO-END. */
import { chromium } from "playwright";
import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const label = process.argv.find((a) => a.startsWith("--label="))?.slice(8) ?? "run";
const stamp = `${new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19)}-${label}`;
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6013";
const settleSeconds = Number(process.argv.find((a) => a.startsWith("--settle="))?.slice(9) ?? "25");
const censusSeconds = Number(process.argv.find((a) => a.startsWith("--census="))?.slice(9) ?? "60");
const guestSettleMs = Number(process.argv.find((a) => a.startsWith("--guest-settle="))?.slice(15) ?? "60") * 1000;
const mdPath = join(OUT, `b48-${stamp}.md`);
const ndPath = join(OUT, `b48-${stamp}.ndjson`);
writeFileSync(ndPath, "");

const t0 = Date.now();
const lines: string[] = [];
const log = (m: string) => {
  const row = `[${((Date.now() - t0) / 1000).toFixed(2)}s] ${m}`;
  lines.push(row);
  console.log(row);
};
const emit = (record: Record<string, unknown>) => appendFileSync(ndPath, `${JSON.stringify(record)}\n`);

const tape: { ms: number; text: string }[] = [];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
/** 🩺️ `runtimeDiagnosticsEnabled` resolves ONCE per page off this key, so it has to be written before
 * the first module evaluates — an `evaluate` after `goto` is already too late. */
await page.addInitScript(() => {
  try {
    globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
  } catch {
    /* a sandboxed tab throws on localStorage; the build env flag is the fallback */
  }
});
page.on("console", (msg) => tape.push({ ms: Date.now() - t0, text: msg.text().slice(0, 1400) }));
page.on("pageerror", (error) => tape.push({ ms: Date.now() - t0, text: `pageerror: ${String(error).slice(0, 300)}` }));
const mark = () => tape.length;
const window_ = (from: number) => tape.slice(from).map((row) => row.text);
const matching = (from: number, pattern: RegExp) => tape.slice(from).filter((row) => pattern.test(row.text)).map((row) => `+${row.ms}ms ${row.text}`);

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
      return { bytes: raw.length, count: parsed.length, first: parsed[0]?.id ?? null };
    })
    .catch(() => ({ bytes: 0, count: 0, first: null as string | null }));

const guestSelection = async () =>
  page
    .evaluate(() =>
      Array.from(document.querySelectorAll("[data-guest-selection-json]")).map((element) => ({
        surface: (element.closest("[id]") as HTMLElement | null)?.id ?? "?",
        raw: (element.getAttribute("data-guest-selection-json") || "").slice(0, 800),
      })),
    )
    .catch(() => [] as { surface: string; raw: string }[]);

const guestSelectedIds = async () =>
  (await guestSelection()).flatMap((row) => {
    try {
      const parsed = JSON.parse(row.raw || "{}") as { selectedIds?: string[]; activeObjectId?: string };
      return [...(parsed.selectedIds ?? []), ...(parsed.activeObjectId ? [parsed.activeObjectId] : [])];
    } catch {
      return [] as string[];
    }
  });

const settle = async <T>(read: () => Promise<T>, done: (value: T) => boolean, budgetMs: number) => {
  const started = Date.now();
  let value = await read();
  while (!done(value) && Date.now() - started < budgetMs) {
    await page.waitForTimeout(300);
    value = await read();
  }
  return { value, waitedMs: Date.now() - started, ok: done(value) };
};

/** 🧱️ `registerBrushMesh` ingress per 10-second bucket, from the whole console tape — a converging
 * announce empties its late buckets, a re-announce loop does not. */
const meshBuckets = (from: number) => {
  const buckets = new Map<number, number>();
  for (const row of tape.slice(from)) {
    if (!/registerBrushMesh/.test(row.text)) continue;
    const bucket = Math.floor(row.ms / 10_000) * 10;
    buckets.set(bucket, (buckets.get(bucket) ?? 0) + 1);
  }
  return [...buckets.entries()].sort((a, b) => a[0] - b[0]).map(([at, count]) => `${at}s:${count}`);
};

log(`goto http://127.0.0.1:${port}/ diagnostics=armed`);
await page.goto(`http://127.0.0.1:${port}/`, { waitUntil: "domcontentloaded", timeout: 120_000 });
const bootBudget = Number(process.argv.find((a) => a.startsWith("--boot="))?.slice(7) ?? "240") * 1000;
const booted = await settle(instances, (value) => value.count > 0, bootBudget);
log(`boot instances=${booted.value.count} bytes=${booted.value.bytes} waitedMs=${booted.waitedMs}`);
if (!booted.ok) {
  // 🧯️ A boot that never published an instance lane says nothing on its own — the reason is always in
  // the console (a module that failed to resolve after a peer's relocation, a guest fault, a plugin that
  // never loaded), so the tape is quoted rather than discarded.
  log(`boot FAILED tail=${JSON.stringify(tape.slice(-40).map((row) => `+${row.ms}ms ${row.text}`)).slice(0, 6000)}`);
  log(`boot FAILED errors=${JSON.stringify(tape.filter((row) => /error|fault|panic|does not provide an export|failed to fetch|500/i.test(row.text)).slice(0, 20).map((row) => `+${row.ms}ms ${row.text}`)).slice(0, 6000)}`);
  writeFileSync(mdPath, `# b48 ${stamp}\n\n${lines.join("\n")}\n`);
  await browser.close();
  process.exit(1);
}

//#region 🔖️ExampleSwitch
/** 🔎️ What the navbar actually offers, for the run where the example switch silently does nothing — a
 * `--inspect-navbar` run answers "which control carries the examples" in one boot instead of one
 * eight-minute battery per guess. */
if (process.argv.includes("--inspect-navbar")) {
  const navbar = await page
    .evaluate(() => ({
      byId: Array.from(document.querySelectorAll<HTMLElement>('[id*="navbar"], [id*="fixture"], [id*="example"]')).map((element) => ({ id: element.id, tag: element.tagName, role: element.getAttribute("role"), text: element.innerText.replace(/\n/g, " ").slice(0, 60) })).slice(0, 40),
      selects: Array.from(document.querySelectorAll("select")).map((element) => ({ id: element.id, options: Array.from(element.options).map((option) => option.label || option.value).slice(0, 12) })),
    }))
    .catch(() => ({ byId: [], selects: [] }));
  log(`navbar byId=${JSON.stringify(navbar.byId).slice(0, 4000)}`);
  log(`navbar selects=${JSON.stringify(navbar.selects).slice(0, 2000)}`);
  await page.keyboard.press("Escape").catch(() => {});
  await page.locator('[id="playground.navbar.fixture"]').first().click({ timeout: 6000, force: true }).catch((error) => log(`navbar click failed ${String(error).slice(0, 200)}`));
  await page.waitForTimeout(1200);
  const opened = await page
    .evaluate(() => ({
      options: Array.from(document.querySelectorAll<HTMLElement>('[role="option"]')).map((element) => ({ id: element.id, text: element.innerText.replace(/\n/g, " ").slice(0, 60) })).slice(0, 20),
      listboxes: Array.from(document.querySelectorAll<HTMLElement>('[role="listbox"], [data-slot="popover"], [data-radix-popper-content-wrapper]')).map((element) => ({ id: element.id, slot: element.getAttribute("data-slot"), children: element.childElementCount })).slice(0, 10),
      expanded: document.querySelector('[id="playground.navbar.fixture"]')?.getAttribute("aria-expanded") ?? null,
    }))
    .catch(() => ({ options: [], listboxes: [], expanded: null }));
  log(`navbar opened expanded=${opened.expanded} options=${JSON.stringify(opened.options).slice(0, 2500)}`);
  log(`navbar opened listboxes=${JSON.stringify(opened.listboxes).slice(0, 1200)}`);
  writeFileSync(mdPath, `# b48 ${stamp}\n\n${lines.join("\n")}\n`);
  await browser.close();
  process.exit(0);
}
const switchMark = mark();
/** 🔀️ The example picker is a `<button role="combobox">` over a portalled Radix listbox — never a
 * `<select>` (`--inspect-navbar`, 2026-09-12: `playground.navbar.fixture` is a BUTTON, `selects=[]`, and
 * the three options only exist once it is expanded). So the switch OPENS it, WAITS for the options to
 * exist instead of sleeping a fixed 800 ms, and then verifies the picker's own label changed — a blind
 * click-and-hope switch is what made a whole battery measure the 1-object document while reporting on
 * the flagship (wave B48 §3). */
const pickerLabel = async () => page.locator('[id="playground.navbar.fixture"]').first().innerText().catch(() => "");
const switchExample = async (pattern: RegExp): Promise<boolean> => {
  for (let attempt = 0; attempt < 4; attempt += 1) {
    await page.keyboard.press("Escape").catch(() => {});
    await page.locator('[id="playground.navbar.fixture"]').first().click({ timeout: 6000, force: true }).catch(() => {});
    const option = page.locator('[role="option"]').filter({ hasText: pattern }).first();
    const appeared = await option.waitFor({ state: "visible", timeout: 8000 }).then(() => true).catch(() => false);
    log(`example-switch attempt=${attempt} expanded=${await page.locator('[id="playground.navbar.fixture"]').first().getAttribute("aria-expanded").catch(() => null)} optionVisible=${appeared}`);
    if (!appeared) continue;
    await option.click({ timeout: 6000 }).catch(() => option.click({ timeout: 6000, force: true }).catch(() => {}));
    const settled = await settle(pickerLabel, (text) => pattern.test(text), 10_000);
    log(`example-switch attempt=${attempt} pickerLabel=${JSON.stringify(settled.value)} ok=${settled.ok}`);
    if (settled.ok) return true;
  }
  return false;
};
const switched = await switchExample(/nakagin/i);
const swapped = await settle(instances, (value) => value.count >= 100, 180_000);
log(`example-switch switched=${switched} instances=${swapped.value.count} bytes=${swapped.value.bytes} waitedMs=${swapped.waitedMs}`);
if (!swapped.ok) {
  log(`example-switch FAILED tail=${JSON.stringify(matching(switchMark, /error|fault|panic|setActiveExample|example/i).slice(-15)).slice(0, 4000)}`);
}
emit({ phase: "example-switch", ...swapped.value, waitedMs: swapped.waitedMs });
//#endregion 🔖️ExampleSwitch

//#region 🔖️MeshAnnounceCensus
await page.waitForTimeout(settleSeconds * 1000);
const censusMark = mark();
await page.waitForTimeout(censusSeconds * 1000);
const meshLines = window_(censusMark).filter((text) => /registerBrushMesh/.test(text));
log(`mesh census seconds=${censusSeconds} registerBrushMesh=${meshLines.length} buckets=${JSON.stringify(meshBuckets(switchMark))}`);
log(`mesh census sample=${JSON.stringify(meshLines.slice(0, 3)).slice(0, 900)}`);
emit({ phase: "mesh-census", seconds: censusSeconds, lines: meshLines.length, buckets: meshBuckets(switchMark), sample: meshLines.slice(0, 6) });
//#endregion 🔖️MeshAnnounceCensus

//#region 🔖️PickAndRefreshHop
const canvas = page.locator("#puzzle3d-main-perspective canvas").last();
const box = await canvas.boundingBox();
log(`perspective canvas box=${JSON.stringify(box)}`);
let landed: string | null = null;
if (box) {
  for (const fraction of [
    { x: 0.5, y: 0.5 },
    { x: 0.45, y: 0.6 },
    { x: 0.55, y: 0.4 },
    { x: 0.5, y: 0.65 },
  ]) {
    if (landed) break;
    const pickMark = mark();
    await canvas.click({ position: { x: box.width * fraction.x, y: box.height * fraction.y }, timeout: 6000, force: true }).catch(() => {});
    const settled = await settle(guestSelectedIds, (ids) => ids.length > 0, guestSettleMs);
    const at = `${fraction.x.toFixed(2)},${fraction.y.toFixed(2)}`;
    const rows = window_(pickMark);
    const census = (pattern: RegExp) => rows.filter((text) => pattern.test(text)).length;
    log(`pick ${at} guestIds=${JSON.stringify(settled.value)} waitedMs=${settled.waitedMs} lines=${rows.length}`);
    log(
      `pick ${at} census interactionSelectIngress=${census(/actionId":"interactionSelect"/)} interactionSelectSettled=${census(/performInvocation settled.*interactionSelect/)}` +
        ` registerBrushMesh=${census(/registerBrushMesh/)} applyHostEffectsRefresh=${census(/applyHostEffects refresh/)} refreshUiLane=${census(/refreshUi lane/)} refreshUiSections=${census(/refreshUi sections/)}`,
    );
    log(`pick ${at} declaredScopes=${JSON.stringify(matching(pickMark, /applyHostEffects refresh/).slice(0, 6)).slice(0, 1600)}`);
    log(`pick ${at} laneDecisions=${JSON.stringify(matching(pickMark, /refreshUi lane/).slice(0, 8)).slice(0, 1600)}`);
    log(`pick ${at} sections=${JSON.stringify(matching(pickMark, /refreshUi sections/).slice(0, 8)).slice(0, 2400)}`);
    log(`pick ${at} dropped=${JSON.stringify(matching(pickMark, /refreshUi dropped|did not publish|stopped without publishing|interaction selection lost/).slice(0, 6)).slice(0, 1200)}`);
    log(`pick ${at} guest-selection-json=${JSON.stringify(await guestSelection()).slice(0, 600)}`);
    emit({
      phase: "pick",
      at,
      ids: settled.value,
      waitedMs: settled.waitedMs,
      lines: rows.length,
      sections: matching(pickMark, /refreshUi sections/).slice(0, 12),
      declared: matching(pickMark, /applyHostEffects refresh/).slice(0, 8),
      lane: matching(pickMark, /refreshUi lane/).slice(0, 12),
      dropped: matching(pickMark, /refreshUi dropped|did not publish|stopped without publishing|interaction selection lost/).slice(0, 8),
    });
    if (settled.value.length) landed = settled.value[0] ?? null;
  }
}
log(`pick landed=${landed ?? "none"}`);
//#endregion 🔖️PickAndRefreshHop

await page.screenshot({ path: join(OUT, `b48-${stamp}.png`), fullPage: false }).catch(() => {});
writeFileSync(mdPath, `# b48 ${stamp}\n\n## Timeline\n\n\`\`\`\n${lines.join("\n")}\n\`\`\`\n`);
log(`wrote ${mdPath}`);
await browser.close();
