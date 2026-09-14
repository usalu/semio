/** ⏳️🔎️ wgpu World3d COMPUTE-STATUS PILL recon — is the pill observable while an example computes?
 *
 * Two lanes reported "the pill never painted" from a session that had NOTHING to compute: the
 * default `?plugin=generation3d` boot carries no example, the producer publishes
 * `phase:"idle" unitsTotal:0` on every frame, and no pill is the CORRECT answer there. This probe
 * measures the two real orderings instead:
 *
 *   • `SEMIO_PROBE_PICK=` (boot axis) — `?example=<id>` in the URL. Measures when the World3d
 *     surface first attaches against when the producer first reports non-idle.
 *   • `SEMIO_PROBE_PICK=1` (live re-evaluation) — boot with NO example, wait until the World3d
 *     surface is live and painting, then drive the shell's own example picker
 *     (`playground.navbar.fixture` → the `shell.example.<id>` row, both located from the shell's own
 *     `os_host pointer hit` trace, never a guessed pixel) and watch the pill appear and vanish.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_PICK=1 bun 🐍️wgpu-status-pill-recon.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const example = process.env.SEMIO_PROBE_EXAMPLE ?? "sphere-cut-with-torus";
const pick = process.env.SEMIO_PROBE_PICK === "1";
const mode = process.env.SEMIO_PROBE_MODE ?? "";
const baseUrl = `${process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d"}${pick ? "" : `&example=${example}`}${mode ? `&mode=${mode}` : ""}`;
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? `wgpu-a11y-runtime/status-${pick ? "picked" : "boot"}-${example}`);
const watchSeconds = Number(process.env.SEMIO_PROBE_WATCH ?? 150);
const viewport = { width: 1440, height: 900 };
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const note = (text) => {
  lines.push(`${at()} PROBE ${text}`);
  console.log(`[DEBUG] ${text}`);
};

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 8000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

const has = (needle) => lines.filter((line) => line.includes(needle));
const stamp = (line) => Number(line.split(" ")[0]);

/** ⏳️ Every pill announcement, parsed. */
const pillTraces = () =>
  has("wgpu world3d status pill").map((line) => ({
    t: stamp(line),
    surface: /surface=(\S+)/.exec(line)?.[1] ?? null,
    phase: /phase=(\S+)/.exec(line)?.[1] ?? null,
    label: /label="((?:[^"\\]|\\.)*)"/.exec(line)?.[1] ?? null,
    ratio: /ratio=(Some\([\d.]+\)|None)/.exec(line)?.[1] ?? null,
    computing: /computing=(true|false)/.exec(line)?.[1] ?? null,
    rect: /rect=(\S+)/.exec(line)?.[1] ?? null,
  }));

/** 🌍️ Every DISTINCT producer status the world3d census carried, with the ms it first appeared. */
const producerStatuses = () => {
  const seen = new Map();
  for (const line of has("world3d surface=")) {
    const status = /status=Some\("((?:[^"\\]|\\.)*)"\)/.exec(line)?.[1];
    if (status === undefined || seen.has(status)) continue;
    seen.set(status, stamp(line));
  }
  return [...seen].map(([status, t]) => ({ t, status }));
};

const pump = async (ms, park) => {
  const deadline = Date.now() + ms;
  let flip = 0;
  const point = park ?? [3, 3];
  while (Date.now() < deadline) {
    await page.waitForTimeout(150);
    flip = 1 - flip;
    await page.mouse.move(point[0] + flip, point[1]).catch(() => {});
  }
};

const hitLines = (fromIndex = 0) => {
  const parsed = [];
  for (const line of lines.slice(fromIndex)) {
    const match = /os_host pointer hit x=([-\d.]+) y=([-\d.]+) targets=(\d+)(?:\s+(?!hit=)\w+=\S+)*\s+hit=(.*)$/.exec(line);
    if (!match) continue;
    const some = /Some\(\((\w+), Some\("([^"]+)"\)\)\)/.exec(match[4]);
    parsed.push({ x: Number(match[1]), y: Number(match[2]), targets: Number(match[3]), kind: some?.[1] ?? null, id: some?.[2] ?? null });
  }
  return parsed;
};

/** 🎯️ Sweeps a band and answers the centre of every control the shell's own hit trace named there. */
const sweep = async (label, y0, y1, x0, x1, step) => {
  const mark = lines.length;
  for (let y = y0; y < y1; y += step) {
    for (let x = x0; x < x1; x += 18) {
      await page.mouse.move(x, y);
      await page.waitForTimeout(28);
    }
  }
  const controls = {};
  for (const hit of hitLines(mark)) {
    if (!hit.id) continue;
    const entry = (controls[hit.id] ??= { id: hit.id, x0: hit.x, x1: hit.x, y0: hit.y, y1: hit.y });
    entry.x0 = Math.min(entry.x0, hit.x);
    entry.x1 = Math.max(entry.x1, hit.x);
    entry.y0 = Math.min(entry.y0, hit.y);
    entry.y1 = Math.max(entry.y1, hit.y);
  }
  for (const entry of Object.values(controls)) entry.point = [(entry.x0 + entry.x1) / 2, (entry.y0 + entry.y1) / 2];
  note(`sweep ${label}: ${JSON.stringify(Object.keys(controls))}`);
  return controls;
};

note(`url ${baseUrl} pick=${pick}`);
await page.goto(baseUrl, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));

let picked = null;
if (pick) {
  for (let second = 0; second < 180; second += 1) {
    await pump(1000);
    if (has("world3d surface=").length > 3) break;
  }
  note(`world3d surface live at ${Math.round(at() / 1000)}s`);
  await pump(8000);
  const navbar = await sweep("navbar", 8, 48, 6, viewport.width - 4, 8);
  const trigger = navbar["playground.navbar.fixture"];
  if (!trigger) note("BLOCKED: no playground.navbar.fixture control");
  else {
    await page.mouse.click(trigger.point[0], trigger.point[1]);
    await pump(2500);
    await page.screenshot({ path: join(outDir, "dropdown-open.png") }).catch(() => {});
    const rows = await sweep("dropdown", 60, Math.round(viewport.height * 0.6), Math.round(viewport.width * 0.5 - 190), Math.round(viewport.width * 0.5 + 190), 10);
    const row = rows[`shell.example.${example}`];
    if (!row) note(`BLOCKED: no shell.example.${example} row among ${JSON.stringify(Object.keys(rows))}`);
    else {
      picked = { id: row.id, point: row.point, atMs: at() };
      await page.mouse.click(row.point[0], row.point[1]);
      note(`picked ${row.id} at ${JSON.stringify(row.point)}`);
    }
  }
}

const watchFrom = at();
for (let second = 0; second < watchSeconds; second += 2) {
  await pump(2000);
  const pills = pillTraces();
  if (second % 10 === 0) note(`t=${second}s pillTraces=${pills.length} computing=${pills.filter((row) => row.computing === "true").length} statuses=${producerStatuses().length}`);
  if (pills.some((row) => row.computing === "true") && second % 4 === 0) await page.screenshot({ path: join(outDir, `pill-computing-${second}s.png`) }).catch(() => {});
}

const pills = pillTraces();
const statuses = producerStatuses();
const firstSurface = has("world3d surface=")[0] ?? null;
const nonIdle = statuses.find((row) => !row.status.includes('phase\\":\\"idle') && !row.status.includes('"phase":"idle"'));
const result = {
  url: baseUrl,
  pick,
  picked,
  watchFromMs: watchFrom,
  firstSurfaceAtMs: firstSurface ? stamp(firstSurface) : null,
  firstNonIdleStatusAtMs: nonIdle?.t ?? null,
  pillTraces: pills,
  pillComputing: pills.filter((row) => row.computing === "true").length,
  pillWithRatio: pills.filter((row) => row.computing === "true" && row.ratio && row.ratio !== "None").length,
  cancelDispatches: has("shell world3d cancel").length,
  distinctProducerStatuses: statuses,
};
note(`RESULT firstSurface=${result.firstSurfaceAtMs}ms firstNonIdle=${result.firstNonIdleStatusAtMs}ms pills=${pills.length} computingPills=${result.pillComputing} withRatio=${result.pillWithRatio}`);
writeFileSync(join(outDir, "result.json"), JSON.stringify(result, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
await browser.close();
