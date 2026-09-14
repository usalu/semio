/** 🗣️🔎️ wgpu COMMAND-PALETTE / LOCALE recon — where `mod+p → "Deutsch" → Enter` stops.
 *
 * The palette is the ONLY runtime route to `os.setLocale` on wgpu, and two lanes have now reported
 * "no `setLocale` dispatch" without being able to say WHICH hop dropped it. This probe reads the
 * shell's own painted overlay out of `dumpStructure` after each hop, so every stage is observable
 * from outside the canvas:
 *   1. does the canvas have DOM focus and does a keydown reach it at all (`data-ui-turn` moves)?
 *   2. does `mod+p` PAINT the overlay (`shell_chrome_string("overlay.search.title")` — `Search` /
 *      `Suchen`) — i.e. did `OverlayState::Search` happen?
 *   3. does typing land in `search_query` (the overlay's own query text node)?
 *   4. do the filtered rows contain the locale command (`Set Locale: Deutsch`)?
 *   5. does Enter produce a `setLocale` dispatch in the console?
 *
 * Usage: cd <ticket> && bun 🐍️wgpu-palette-locale-recon.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const baseUrl = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-a11y-runtime/palette");
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT ?? 180);
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 45);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const note = (text) => {
  lines.push(`${at()} PROBE ${text}`);
  console.log(`[DEBUG] ${text}`);
};

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 6000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

const introspect = (kind, windowId) =>
  page
    .evaluate(
      async ([kind, id]) => {
        const beacon = globalThis.semioWgpuIntrospection;
        if (typeof beacon?.[kind] !== "function") return null;
        try {
          const raw = await beacon[kind](id);
          return raw ? JSON.parse(raw) : null;
        } catch (error) {
          return { error: String(error) };
        }
      },
      [kind, windowId],
    )
    .catch(() => null);

const windowIds = async () => (await introspect("dumpStructure", undefined))?.windowIds ?? [];

/** 🔤️ Every text the shell painted across every live window — the overlay lives in chrome, so this is
 * the only DOM-visible witness of the palette. */
const shellTexts = async () => {
  const out = {};
  for (const id of await windowIds()) {
    const dump = await introspect("dumpStructure", id);
    out[id] = (dump?.nodes ?? []).map((node) => node.text).filter((text) => typeof text === "string" && text.length > 0);
  }
  return out;
};

const focusState = () =>
  page
    .evaluate(() => ({
      activeId: document.activeElement?.id ?? null,
      activeTag: document.activeElement?.tagName ?? null,
      uiTurn: document.getElementById("semio-wgpu-canvas")?.dataset.uiTurn ?? null,
    }))
    .catch(() => null);

const pump = async (ms) => {
  const deadline = Date.now() + ms;
  let flip = 0;
  while (Date.now() < deadline) {
    await page.waitForTimeout(180);
    flip = 1 - flip;
    await page.mouse.move(3 + flip, 3).catch(() => {});
  }
};

const dispatchLines = () => lines.filter((line) => line.includes("setLocale") || line.includes("dispatch_normalized_event") || line.includes("os_host dispatch"));

const report = { url: baseUrl, hops: [] };
const hop = async (name, extra = {}) => {
  const texts = await shellTexts();
  const flat = Object.values(texts).flat();
  const entry = {
    hop: name,
    t: at(),
    focus: await focusState(),
    overlayTitle: flat.find((text) => ["Search", "Suchen", "Find in page", "Auf Seite suchen"].includes(text)) ?? null,
    localeRow: flat.filter((text) => text.includes("Deutsch") || text.includes("Set Locale") || text.includes("Sprache")),
    sample: flat.slice(0, 40),
    ...extra,
  };
  report.hops.push(entry);
  note(`${name}: focus=${JSON.stringify(entry.focus)} overlayTitle=${entry.overlayTitle} localeRow=${JSON.stringify(entry.localeRow)}`);
  await page.screenshot({ path: join(outDir, `${name}.png`) }).catch(() => {});
  return entry;
};

await page.goto(baseUrl, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));
for (let second = 0; second < bootSeconds; second += 1) {
  await pump(1000);
  if ((await windowIds()).length > 0) break;
}
await pump(settleSeconds * 1000);
await hop("00-settled");

await page.evaluate(() => document.getElementById("semio-wgpu-canvas")?.focus({ preventScroll: true })).catch(() => {});
await pump(600);
await hop("01-canvas-focused");

/** ⌨️ EXACTLY ONE palette chord. Pressing a second one is what the first run of this recon did, and
 * it poisoned its own evidence: `mod+p` sets `input.focused_id = "shell.search.input"`, every
 * hardcoded shell chord is gated on `!editing`, so the second `mod+p` is NOT a toggle — it falls
 * through to the open palette's own `Char` arm and types a literal `p` into the query. */
{
  const before = lines.length;
  await page.keyboard.press("Control+p");
  await pump(2500);
  await hop("02-Control-p", { newConsole: lines.slice(before).filter((line) => line.includes("key routing")).slice(0, 8) });
}

await page.keyboard.type("Deutsch", { delay: 80 });
await pump(2500);
await hop("03-typed-deutsch");

const beforeEnter = lines.length;
await page.keyboard.press("Enter");
await pump(6000);
await hop("04-after-enter", { newConsole: lines.slice(beforeEnter).slice(0, 40) });

/** ⌨️ Does `mod+p` TOGGLE? React's palette closes on the same chord; here the chord that opened it
 * armed `editing`, so the closing half can never run. Escape is the only way out. */
const beforeToggle = lines.length;
await page.keyboard.press("Control+p");
await pump(2000);
await hop("05-second-chord", { newConsole: lines.slice(beforeToggle).filter((line) => line.includes("key routing")).slice(0, 4) });
await page.keyboard.press("Escape");
await pump(2000);
await hop("06-escape");

writeFileSync(join(outDir, "report.json"), JSON.stringify({ ...report, dispatchLines: dispatchLines().slice(-40) }, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
note("done");
await browser.close();
