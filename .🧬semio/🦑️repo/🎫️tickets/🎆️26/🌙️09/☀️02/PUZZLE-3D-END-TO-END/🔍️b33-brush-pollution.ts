/** 🧪️ Wave B33 pollution probe — isolates what the brush hover-storm leaves behind that stops the NEXT
 * unrelated document mutation (`addTargetVolume` via the volume brush's Alt+click) from landing.
 * Unlike `🔍️browser-probe.ts` this keeps an UNBOUNDED console log with absolute sequence numbers, so a
 * mark taken before a gesture still addresses the lines that gesture produced (the probe's own
 * `consoleBuf` is a 4000-line ring that `shift()`s, which makes every `slice(mark)` read empty once a long
 * run saturates it — every `tail=[]`/`ingressesWhileWaiting=0` in battery #51 is that artifact).
 * Ticket 26/09/02/PUZZLE-3D-END-TO-END.
 *
 * Run: `bun 🔍️b33-brush-pollution.ts [--pollute=brush|hover|arm|none] [--port=6013]`. */
import { chromium } from "playwright";
import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6013";
const pollute = process.argv.find((a) => a.startsWith("--pollute="))?.slice(10) ?? "brush";
const logPath = join(OUT, `b33-pollution-${pollute}-${stamp}.txt`);
writeFileSync(logPath, "");
const t0 = Date.now();
const log = (m: string) => {
  const row = `[${((Date.now() - t0) / 1000).toFixed(1)}s] ${m}`;
  console.log(row);
  appendFileSync(logPath, `${row}\n`);
};

const console_: string[] = [];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (msg) => {
  console_.push(`${msg.type()}: ${msg.text().slice(0, 400)}`);
});
page.on("pageerror", (error) => console_.push(`pageerror: ${String(error).slice(0, 400)}`));

const snapshot = () =>
  page.evaluate(() => ({
    windows: document.querySelectorAll('[data-slot="window"]').length,
    canvases: document.querySelectorAll("canvas").length,
    dialogs: Array.from(document.querySelectorAll('[role="dialog"]')).length,
  }));

const hostState = () =>
  page.evaluate(() => {
    const host = document.querySelector("#puzzle3d-main-perspective [data-surface-id]") as HTMLElement | null;
    const count = (raw: string | null | undefined) => {
      try {
        return (JSON.parse(raw || "[]") as unknown[]).length;
      } catch {
        return -1;
      }
    };
    let interaction: Record<string, unknown> | null = null;
    try {
      interaction = JSON.parse(host?.getAttribute("data-interaction-json") || "null");
    } catch {
      interaction = null;
    }
    return {
      instances: count(host?.getAttribute("data-instances-json")),
      vortices: count(host?.getAttribute("data-vortices-json")),
      volumes: count(host?.getAttribute("data-target-volumes-json")),
      previewLen: (host?.getAttribute("data-brush-preview-json") || "").length,
      interaction,
    };
  });

const settleFor = async <T>(read: () => Promise<T>, settled: (value: T) => boolean, budgetMs = 30000, stepMs = 500) => {
  const start = Date.now();
  let value = await read();
  while (!settled(value) && Date.now() - start < budgetMs) {
    await page.waitForTimeout(stepMs);
    value = await read();
  }
  return { value, waitedMs: Date.now() - start, ok: settled(value) };
};

const unfoldUtilities = async () => {
  await page.locator("canvas").last().click({ position: { x: 80, y: 80 }, timeout: 4000 }).catch(() => {});
  const byId = page.locator('[id="framework.window.puzzle3dMainPerspective.utilityBar.unfold"]').first();
  if (await byId.count()) await byId.click({ timeout: 4000 }).catch(() => {});
  await page.waitForTimeout(800);
};

const armUtility = async (utilityId: string) => {
  await unfoldUtilities();
  const loc = page.locator(`[id="${utilityId}"]`).first();
  const found = await loc.count();
  if (found) await loc.click({ force: true, timeout: 4000 }).catch(() => {});
  await page.waitForTimeout(1800);
  const state = await hostState();
  log(`arm ${utilityId} found=${found} activeUtility=${JSON.stringify((state.interaction as { activeUtility?: string } | null)?.activeUtility ?? null)}`);
  return state;
};

const frameForestTable = async () => {
  const c = page.locator("canvas").last();
  await c.click({ position: { x: 80, y: 80 }, timeout: 4000 }).catch(() => {});
  const frameBtn = page.locator("#puzzle3d-main-perspective").locator('[id^="world3d-frame-instances-"]').first();
  if (await frameBtn.count()) await frameBtn.click({ force: true, timeout: 4000 }).catch(() => {});
  await page.waitForTimeout(1500);
  const box = await c.boundingBox();
  return { box: box ?? { x: 0, y: 0, width: 947, height: 814 }, table: { x: 739, y: 342 } };
};

const vortexHits = () =>
  page.evaluate(() => {
    const host = document.querySelector("#puzzle3d-main-perspective [data-instances-json]") as HTMLElement | null;
    let raw: Array<{ fullId?: string; sx?: number; sy?: number; ndcZ?: number }> = [];
    try {
      raw = JSON.parse(host?.getAttribute("data-vortex-hits") || "[]");
    } catch {
      raw = [];
    }
    return raw.slice(0, 20);
  });

const waitVortices = async () => {
  for (let i = 0; i < 16; i++) {
    const state = await hostState();
    if (state.vortices > 0) return state.vortices;
    await page.waitForTimeout(400);
  }
  return (await hostState()).vortices;
};

log(`navigating pollute=${pollute}`);
await page.goto(`http://127.0.0.1:${port}/?plugin=puzzle3d`, { waitUntil: "domcontentloaded", timeout: 60000 });
let booted = false;
for (let i = 0; i < 70; i++) {
  await page.waitForTimeout(3000);
  const s = await snapshot();
  if (i % 10 === 9) log(`boot waiting… windows=${s.windows} canvases=${s.canvases} console=${console_.length}`);
  if (s.dialogs) {
    const skip = page.locator('[role="dialog"] button', { hasText: /skip/i }).first();
    if (await skip.count()) await skip.click({ timeout: 2000 }).catch(() => {});
  }
  if (s.windows >= 2 && s.canvases >= 2) {
    booted = true;
    log(`booted windows=${s.windows} canvases=${s.canvases}`);
    break;
  }
}
if (!booted) {
  log(`boot FAILED console tail=${JSON.stringify(console_.slice(-12))}`);
  await browser.close();
  process.exit(1);
}
const skip = page.locator("button", { hasText: /^\s*(x\s*)?skip\s*$/i }).first();
if (await skip.count()) {
  await skip.click({ timeout: 3000 }).catch(() => {});
  await page.waitForTimeout(1500);
}

const framed = await frameForestTable();
log(`framed state=${JSON.stringify(await hostState())}`);

if (pollute !== "none") {
  const brushMark = console_.length;
  await armUtility("brush");
  log(`brush vortices after arm=${await waitVortices()}`);
  const reframed = await frameForestTable();
  framed.box = reframed.box;
  const hits = await vortexHits();
  log(`brush vortex hits=${hits.length} sample=${JSON.stringify(hits.slice(0, 4))}`);
  if (pollute === "brush" || pollute === "hover") {
    const onscreen = hits.filter((h) => (h.sx ?? -1) >= 8 && (h.sy ?? -1) >= 8 && (h.sx ?? 1e9) <= framed.box.width - 8 && (h.sy ?? 1e9) <= framed.box.height - 8);
    const storm = onscreen.length ? onscreen : [{ sx: framed.table.x, sy: framed.table.y }];
    for (let i = 0; i < 70; i++) {
      const hit = storm[i % storm.length];
      await page.mouse.move(framed.box.x + (hit.sx ?? framed.table.x), framed.box.y + (hit.sy ?? framed.table.y), { steps: 1 });
    }
    log(`hover storm done state=${JSON.stringify(await hostState())}`);
  }
  if (pollute === "brush") {
    const aim = hits[0] ?? { sx: framed.table.x, sy: framed.table.y };
    const c = page.locator("canvas").last();
    await c.hover({ position: { x: aim.sx ?? framed.table.x, y: aim.sy ?? framed.table.y }, timeout: 4000 }).catch(() => {});
    await page.waitForTimeout(200);
    await c.click({ position: { x: aim.sx ?? framed.table.x, y: aim.sy ?? framed.table.y }, timeout: 4000 }).catch(() => {});
    await page.waitForTimeout(2500);
    log(`brush click done state=${JSON.stringify(await hostState())}`);
  }
  await page.keyboard.press("Escape").catch(() => {});
  log(`brush phase console lines=${console_.length - brushMark}`);
}

const preVolume = console_.length;
const framed2 = await frameForestTable();
const armed = await armUtility("volumeBrush");
const before = (await hostState()).volumes;
log(`volume before=${before} armed=${JSON.stringify(armed.interaction)}`);
const canvas = page.locator("canvas").last();
const clickMark = console_.length;
await canvas.hover({ position: { x: framed2.table.x, y: framed2.table.y }, timeout: 4000 }).catch(() => {});
await page.keyboard.down("Alt").catch(() => {});
await canvas.click({ position: { x: framed2.table.x, y: framed2.table.y }, modifiers: ["Alt"], timeout: 4000, force: true }).catch(() => {});
await page.keyboard.up("Alt").catch(() => {});
const settle = await settleFor(async () => (await hostState()).volumes, (count) => count > before);
log(`VERDICT add-target-volume ok=${settle.ok} before=${before} after=${settle.value} waitedMs=${settle.waitedMs}`);
log(`state after=${JSON.stringify(await hostState())}`);

const interesting = /addTargetVolume|performInvocation|command ingress|dropped|refus|notice|worldPointerDown|volumeBrush|pointer|ground|admission|queue|budget|starv|lane/i;
const window_ = console_.slice(clickMark).filter((line) => interesting.test(line));
log(`click window: total=${console_.length - clickMark} interesting=${window_.length}`);
for (const line of window_.slice(0, 120)) log(`  · ${line}`);
const histogram = new Map<string, number>();
for (const line of console_.slice(preVolume)) {
  const key = line.replace(/[0-9a-f]{6,}/g, "#").replace(/\d+/g, "n").slice(0, 110);
  histogram.set(key, (histogram.get(key) ?? 0) + 1);
}
log(`--- console histogram since volume phase (${console_.length - preVolume} lines) ---`);
for (const [key, count] of [...histogram.entries()].sort((a, b) => b[1] - a[1]).slice(0, 30)) log(`  ${String(count).padStart(5)} ${key}`);
writeFileSync(join(OUT, `b33-pollution-${pollute}-${stamp}.console.txt`), console_.join("\n"));
log(`console dump → b33-pollution-${pollute}-${stamp}.console.txt (${console_.length} lines)`);
await browser.close();
