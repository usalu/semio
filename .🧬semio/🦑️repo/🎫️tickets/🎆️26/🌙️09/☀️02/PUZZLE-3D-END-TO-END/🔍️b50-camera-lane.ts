/** 📷️ Wave B50 — WHERE the camera lane broke between wasm #58 (orbit/pan/zoom moved in ~1 s) and #59
 * (`camera settle … moved=false waitedMs=30184` for all three).
 *
 * The chain is gesture → orbit controls → `dispatchWorldCameraDebounced` → `setCamera` → guest
 * `set-camera` → WindowConfig lane → guest window render → scene spine `cameraJson` → host
 * `data-camera-json` (wave B12 §4.1, wave B15). This probe reads BOTH ends of it on every step:
 *
 * - `data-viewport-camera-json` — the pane's own LIVE rig pose. It moves the moment the drag reaches
 *   `OrbitControls`, with no guest round trip at all. A gesture that leaves it untouched never reached
 *   the controls (a host-side interception — a pick/marquee/relocate grab swallowing the drag).
 * - `data-camera-json` — the GUEST-published pose. It only moves once the round trip lands.
 *
 * Those two attributes split the regression into exactly two readings, with no tap needed:
 * rig moved + published frozen ⇒ the round trip broke; rig frozen ⇒ the gesture was intercepted.
 *
 * It also arms `SEMIO_RUNTIME_DIAGNOSTICS` before the first module evaluates (wave B48) and keeps the
 * whole console tape, so every `setCamera`/refresh line in the window around each gesture is quoted.
 *
 * Run: `bun 🔍️b50-camera-lane.ts [--port=6013] [--label=before] [--boot=180] [--watch=12]`.
 * Output: `🗑️generated/b50-<stamp>-<label>.md` / `.ndjson`. Ticket 26/09/02/PUZZLE-3D-END-TO-END. */
import { chromium } from "playwright";
import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const label = process.argv.find((a) => a.startsWith("--label="))?.slice(8) ?? "run";
const stamp = `${new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19)}-${label}`;
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6013";
const bootSeconds = Number(process.argv.find((a) => a.startsWith("--boot="))?.slice(7) ?? "180");
const watchSeconds = Number(process.argv.find((a) => a.startsWith("--watch="))?.slice(8) ?? "12");
const mdPath = join(OUT, `b50-${stamp}.md`);
const ndPath = join(OUT, `b50-${stamp}.ndjson`);
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
await page.addInitScript(() => {
  try {
    globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
  } catch {
    /* a sandboxed tab throws on localStorage */
  }
});
page.on("console", (msg) => tape.push({ ms: Date.now() - t0, text: msg.text().slice(0, 900) }));
page.on("pageerror", (error) => tape.push({ ms: Date.now() - t0, text: `pageerror: ${String(error).slice(0, 300)}` }));
const mark = () => tape.length;
const since = (from: number, pattern: RegExp) => tape.slice(from).filter((row) => pattern.test(row.text)).map((row) => `+${row.ms}ms ${row.text}`);

const PANE = "puzzle3d-main-perspective";

const poses = async () =>
  page
    .evaluate((pane) => {
      const read = (id: string) => {
        const host = (document.querySelector(`[id="${id}"] [data-camera-json]`) ?? document.querySelector(`[id="framework.window.${id}"] [data-camera-json]`)) as HTMLElement | null;
        return {
          id,
          published: host?.getAttribute("data-camera-json") ?? null,
          viewport: host?.getAttribute("data-viewport-camera-json") ?? null,
          instances: (host?.getAttribute("data-instances-json") ?? "").length,
          selection: (host?.getAttribute("data-selection-json") ?? "").slice(0, 220),
        };
      };
      return [read(pane), read("puzzle3d-main-top")];
    }, PANE)
    .catch(() => [] as { id: string; published: string | null; viewport: string | null; instances: number; selection: string }[]);

const paneOf = async () => (await poses()).find((row) => row.id === PANE) ?? null;

const veilGone = async () =>
  page
    .evaluate(() => {
      const text = document.body?.innerText ?? "";
      return !/Preparing|Loading|Starting/i.test(text.slice(0, 400));
    })
    .catch(() => false);

const settle = async <T>(read: () => Promise<T>, done: (value: T) => boolean, budgetMs: number, stepMs = 250) => {
  const started = Date.now();
  let value = await read();
  while (!done(value) && Date.now() - started < budgetMs) {
    await page.waitForTimeout(stepMs);
    value = await read();
  }
  return { value, waitedMs: Date.now() - started, ok: done(value) };
};

/** 🎠️ Every DISTINCT reactor publication-slot state the tape shows since `from`, with its count — the
 * `window` slot is the WindowConfig lane's own, so a slot frozen at one `ack/rev` pair across a whole
 * gesture is the publication that never landed. Reactor lines are excluded from the per-step console
 * dump because one gesture emits thousands of them. */
const slotStates = (from: number) => {
  const counts = new Map<string, number>();
  let streak = 0;
  for (const row of tape.slice(from)) {
    for (const match of row.text.matchAll(/1:(window|puzzle3d-main-perspective)#g\d+:[^,\]]*/g)) counts.set(match[0], (counts.get(match[0]) ?? 0) + 1);
    const found = /more-work streak=(\d+)/.exec(row.text);
    if (found) streak = Math.max(streak, Number(found[1]));
  }
  const rendered = [...counts.entries()].sort((a, b) => b[1] - a[1]).slice(0, 8).map(([state, count]) => `${state} x${count}`);
  return `maxStreak=${streak} distinct=${counts.size} ${rendered.join(" | ")}`;
};

const canvasBox = async () => {
  const canvas = page.locator(`[id="${PANE}"] canvas, [id="framework.window.${PANE}"] canvas`).first();
  if (await canvas.count()) return await canvas.boundingBox();
  return await page.locator("canvas").last().boundingBox();
};

/** 🛰️ One gesture, then BOTH ends of the lane watched for `watchSeconds`. The rig is sampled
 * immediately after mouseup (before any debounce could have fired) and again at the end, so
 * "the drag reached the controls" and "the round trip landed" are separate facts. */
const gesture = async (name: string, run: (cx: number, cy: number) => Promise<void>) => {
  const box = await canvasBox();
  if (!box) {
    log(`${name} SKIPPED no canvas box`);
    return;
  }
  const cx = box.x + box.width * 0.5;
  const cy = box.y + box.height * 0.35;
  const before = await paneOf();
  const from = mark();
  await run(cx, cy);
  const rigAfter = await paneOf();
  const published = await settle(
    async () => (await paneOf())?.published ?? null,
    (latest) => latest !== (before?.published ?? null),
    watchSeconds * 1000,
  );
  const rigMoved = (rigAfter?.viewport ?? null) !== (before?.viewport ?? null);
  log(
    `${name} rigMoved=${rigMoved} publishedMoved=${published.ok} waitedMs=${published.waitedMs}` +
      ` rigBefore=${String(before?.viewport).slice(0, 96)} rigAfter=${String(rigAfter?.viewport).slice(0, 96)}`,
  );
  log(`${name} publishedBefore=${String(before?.published).slice(0, 120)} publishedAfter=${String(published.value).slice(0, 120)}`);
  log(`${name} slots ${slotStates(from)}`);
  const interesting = since(from, /setCamera|set-camera|refreshUi sections|refreshUi lane|applyHostEffects|windowConfig|window-config|WindowConfig|pageerror/).filter((row) => !row.includes("more-work"));
  log(`${name} console lines=${tape.length - from} matching=${interesting.length}`);
  for (const row of interesting.slice(0, 40)) log(`${name} | ${row}`);
  emit({ step: name, rigMoved, publishedMoved: published.ok, waitedMs: published.waitedMs, before, rigAfter, publishedAfter: published.value, console: interesting.slice(0, 120) });
  await page.keyboard.press("Escape").catch(() => {});
};

log(`goto http://127.0.0.1:${port}/`);
await page.goto(`http://127.0.0.1:${port}/`, { waitUntil: "domcontentloaded", timeout: 120_000 });
const booted = await settle(async () => ((await paneOf())?.published ? await veilGone() : false), (ok) => ok === true, bootSeconds * 1000, 1000);
log(`boot ok=${booted.ok} waitedMs=${booted.waitedMs}`);
const first = await poses();
for (const row of first) log(`pane ${row.id} published=${String(row.published).slice(0, 150)} viewport=${String(row.viewport).slice(0, 150)} instancesBytes=${row.instances}`);
log(`boot slots ${slotStates(0)}`);

await gesture("orbit-alt-right", async (cx, cy) => {
  await page.keyboard.down("Alt").catch(() => {});
  await page.mouse.move(cx, cy);
  await page.mouse.down({ button: "right" });
  await page.mouse.move(cx + 160, cy + 70, { steps: 20 });
  await page.mouse.up({ button: "right" });
  await page.keyboard.up("Alt").catch(() => {});
});

await gesture("pan-shift-right", async (cx, cy) => {
  await page.keyboard.down("Shift").catch(() => {});
  await page.mouse.move(cx, cy);
  await page.mouse.down({ button: "right" });
  await page.mouse.move(cx - 130, cy + 90, { steps: 20 });
  await page.mouse.up({ button: "right" });
  await page.keyboard.up("Shift").catch(() => {});
});

await gesture("zoom-wheel", async (cx, cy) => {
  await page.mouse.move(cx, cy);
  await page.mouse.wheel(0, -700);
});

const finalPoses = await poses();
for (const row of finalPoses) log(`final ${row.id} published=${String(row.published).slice(0, 150)} viewport=${String(row.viewport).slice(0, 150)} selection=${row.selection}`);
log(`tape lines=${tape.length}`);
const cameraTape = tape.filter((row) => /camera/i.test(row.text)).slice(-40);
for (const row of cameraTape) log(`tape | +${row.ms}ms ${row.text}`);

writeFileSync(mdPath, `# Wave B50 camera lane probe — ${stamp}\n\n\`\`\`\n${lines.join("\n")}\n\`\`\`\n`);
log(`wrote ${mdPath}`);
await browser.close();
