/** ♿️ ARIA-mirror LIVENESS recon — lane `wgpu-wheel-zoom-a11y-live` (2026-09-14).
 *
 * Separates the two things the battery's `status-a11y-i18n / accessibility:live` red can mean:
 *
 *   A. the RENDERER's published accessibility tree changed and the DOM mirror beside the canvas did
 *      not follow (a mirror defect — the mirror refreshes only on some turns), or
 *   B. the published tree itself never changed, because the guest never republished that window's
 *      retained document (a producer/settle defect, `wgpu-host-settle-pump`'s lane).
 *
 * It therefore samples BOTH sides at the same instants — `dumpAccessibility()` (the renderer's own
 * projection of its retained trees) and `#semio-wgpu-accessibility` (the DOM the reader reads) —
 * around two gestures:
 *
 *   • a CHEAP document change (a panel toggle) that costs the producer nothing, so the mirror's own
 *     liveness is measured without any evaluation in the way;
 *   • the battery's own gesture, `shell.example.<example>`, sampled for the whole budget together
 *     with the shell's `renderSurface` / `render begin` traces per window, so "the mirror is late"
 *     and "the document never arrived" are told apart by name.
 *
 * The wgpu tick is INPUT-DRIVEN: every wait nudges the pointer 1 px.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-wheel-a11y/aria-1 bun 🐍️wgpu-aria-live-probe.mjs
 * @see 🐍️wgpu-status-a11y-i18n-probe.mjs, 📓️wgpu-a11y-status-i18n-runtime-2026-09-14.md
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const baseUrl = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-wheel-a11y/aria");
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT ?? 200);
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 60);
const watchSeconds = Number(process.env.SEMIO_PROBE_WATCH ?? 200);
const example = process.env.SEMIO_PROBE_EXAMPLE ?? "sphere-cut-with-torus";
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
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 6000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

const has = (needle) => lines.filter((line) => line.includes(needle));
const countOf = (needle) => has(needle).length;

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

const liveWindowIds = async () => (await introspect("dumpStructure", undefined))?.windowIds ?? [];

/** ♿️ The DOM subtree a screen reader reads, reduced to what a comparison needs. */
const mirror = () =>
  page
    .evaluate(() => {
      const root = document.getElementById("semio-wgpu-accessibility");
      if (!root) return null;
      return {
        attr: root.getAttribute("data-node-count"),
        nodeCount: Number(root.dataset.nodeCount ?? 0),
        windows: (root.dataset.windows ?? "").split(" ").filter(Boolean),
        labels: Array.from(root.children).map((node) => `${node.dataset.window ?? "?"}:${node.getAttribute("aria-label") ?? ""}`),
      };
    })
    .catch(() => null);

/** ♿️ The RENDERER's own projection at the same instant — what the mirror would paint if it pulled now. */
const dumpLabels = (dump) => (dump?.windows ?? []).flatMap((entry) => (entry.nodes ?? []).map((node) => `${entry.windowId}:${node.label ?? ""}`));

const sample = async (label) => {
  const dump = await introspect("dumpAccessibility", undefined);
  const dom = await mirror();
  const entry = {
    label,
    t: at(),
    dumpNodes: (dump?.windows ?? []).reduce((sum, w) => sum + (w.nodes?.length ?? 0), 0),
    dumpWindows: Object.fromEntries((dump?.windows ?? []).map((w) => [w.windowId, w.nodes?.length ?? 0])),
    dumpLabels: dumpLabels(dump),
    mirrorNodes: dom?.nodeCount ?? 0,
    mirrorWindows: dom?.windows ?? [],
    mirrorLabels: dom?.labels ?? [],
    mainRenders: countOf("wgpu-bridge renderSurface surface=procedural-main"),
    previewRenders: countOf("wgpu-bridge renderSurface surface=procedural-preview"),
    mainIngress: has("ui-doc ingress window=procedural-main").map((line) => line.slice(line.indexOf("generation="))),
    frameBuilds: countOf("frame build admitted"),
    lastGate: (has("os_host frame gate").at(-1) ?? "").slice(0, 200),
  };
  entry.mirrorTracksDump = JSON.stringify(entry.mirrorLabels) === JSON.stringify(entry.dumpLabels);
  return entry;
};

const pump = async (ms, park) => {
  const deadline = Date.now() + ms;
  let flip = 0;
  const point = park ?? [3, 3];
  while (Date.now() < deadline) {
    await page.waitForTimeout(180);
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

/** 👀️ Watches both sides after a gesture: when did the renderer's projection change, and when did
 * the DOM mirror follow it? `null` means "never, inside the budget". */
const watch = async (label, baseline, seconds) => {
  const baselineDump = JSON.stringify(baseline.dumpLabels);
  const baselineMirror = JSON.stringify(baseline.mirrorLabels);
  const trail = [];
  let dumpChangedAt = null;
  let mirrorChangedAt = null;
  for (let tick = 0; tick < seconds * 2; tick += 1) {
    await pump(500);
    const entry = await sample(`${label}#${tick}`);
    if (dumpChangedAt === null && JSON.stringify(entry.dumpLabels) !== baselineDump) dumpChangedAt = entry.t;
    if (mirrorChangedAt === null && JSON.stringify(entry.mirrorLabels) !== baselineMirror) mirrorChangedAt = entry.t;
    trail.push({ t: entry.t, dumpNodes: entry.dumpNodes, mirrorNodes: entry.mirrorNodes, tracks: entry.mirrorTracksDump, mainRenders: entry.mainRenders, previewRenders: entry.previewRenders, frameBuilds: entry.frameBuilds, mainIngress: entry.mainIngress.length });
    if (dumpChangedAt !== null && mirrorChangedAt !== null) break;
  }
  const final = await sample(`${label}:final`);
  const verdict = dumpChangedAt === null ? "document-never-changed" : mirrorChangedAt === null ? "mirror-never-followed" : "mirror-followed";
  note(`${label}: ${verdict} dumpChangedAt=${dumpChangedAt} mirrorChangedAt=${mirrorChangedAt} lag=${dumpChangedAt !== null && mirrorChangedAt !== null ? mirrorChangedAt - dumpChangedAt : null}`);
  return { label, verdict, dumpChangedAt, mirrorChangedAt, lagMs: dumpChangedAt !== null && mirrorChangedAt !== null ? mirrorChangedAt - dumpChangedAt : null, trail, final };
};

const results = { url: baseUrl, startedAt: new Date().toISOString(), samples: [], watches: [] };
const finish = (code) => {
  results.finishedAt = new Date().toISOString();
  results.consoleCensus = {
    mainRenders: countOf("wgpu-bridge renderSurface surface=procedural-main"),
    previewRenders: countOf("wgpu-bridge renderSurface surface=procedural-preview"),
    mainRenderBegin: countOf("render begin surface=procedural-main"),
    pageErrors: countOf(" pageerror "),
  };
  writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  note(`wrote ${outDir}`);
  return code;
};

await page.goto(baseUrl, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));
let windowIds = [];
for (let second = 0; second < bootSeconds; second += 1) {
  await pump(1000);
  windowIds = await liveWindowIds();
  if (windowIds.length > 0 && has("world3d surface=").length > 0) break;
}
note(`booted windows=${JSON.stringify(windowIds)} seconds=${Math.round(at() / 1000)}`);
await pump(settleSeconds * 1000);

const booted = await sample("booted");
results.samples.push(booted);
note(`booted mirror=${booted.mirrorNodes} dump=${booted.dumpNodes} tracks=${booted.mirrorTracksDump}`);

// ── 1 — a CHEAP document change: a panel toggle costs the producer nothing ─────────────────────
if (process.env.SEMIO_PROBE_SKIP_PANEL !== "1") {
  const navbar = await sweep("navbar", 8, 48, 6, viewport.width - 4, 6);
  const toggle = navbar["ui.panelToggle.details"] ?? navbar["ui.panelToggle.workbench"] ?? navbar["ui.panelToggle.settings"] ?? null;
  const before = await sample("panel-toggle:before");
  results.samples.push(before);
  if (toggle) {
    await page.mouse.click(toggle.point[0], toggle.point[1]);
    note(`clicked ${toggle.id}`);
    results.watches.push({ gesture: toggle.id, ...(await watch("panel-toggle", before, 20)) });
  } else {
    note("no panel toggle found in the navbar sweep");
    results.watches.push({ gesture: null, label: "panel-toggle", verdict: "blocked" });
  }
}

// ── 2 — the battery's own gesture: switch the example from the shell's own picker ──────────────
{
  const navbar = await sweep("navbar (example picker)", 8, 48, 6, viewport.width - 4, 6);
  const trigger = navbar["playground.navbar.fixture"] ?? null;
  const before = await sample("example:before");
  results.samples.push(before);
  let picked = null;
  if (trigger) {
    await page.mouse.click(trigger.point[0], trigger.point[1]);
    await pump(2500);
    const rows = await sweep("example dropdown", 60, Math.round(viewport.height * 0.6), Math.round(viewport.width * 0.5 - 190), Math.round(viewport.width * 0.5 + 190), 10);
    const row = rows[`shell.example.${example}`] ?? null;
    if (row) {
      picked = row.id;
      await page.mouse.click(row.point[0], row.point[1]);
    } else {
      note(`no shell.example.${example} among ${JSON.stringify(Object.keys(rows))}`);
    }
  }
  results.samples.push(await sample("example:picked"));
  results.watches.push({ gesture: picked, ...(await watch("example", before, watchSeconds)) });
}

results.samples.push(await sample("final"));
const code = finish(0);
await browser.close();
process.exit(code);
