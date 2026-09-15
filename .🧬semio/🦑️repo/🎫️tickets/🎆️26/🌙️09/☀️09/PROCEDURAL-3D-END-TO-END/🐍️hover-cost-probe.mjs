/** 🐁️ What ONE hover and ONE select GESTURE cost the React shell — refresh passes, the scope each pass
 * was opened with, the guest body renders that pass asked for, and the wall time from the pointer
 * event to quiescence.
 *
 * 🩺️ Nothing here is inferred. The host prints its own `[DEBUG] refreshUi sections {scope,asked,…}`
 * line per pass and the guest prints `[DEBUG] gen3d render body=<key>` per rendered body, both gated on
 * the runtime-diagnostics switch this probe arms in `localStorage` before the page boots
 * (`RUNTIME_DIAGNOSTICS_KEY`, `🏛️ShellHost/🟦️.tsx`). A gesture's cost is the delta of those counters
 * across it, and the scope is read out of the host's own line rather than guessed from the count.
 *
 * 🧭️ Gesture points are swept off the preview canvas until the pane publishes a `hoverTarget`, the
 * same way `🐍️interaction-matrix-probe.mjs` finds geometry — a fixed centre point misses on most
 * examples, and a hover that touches nothing costs nothing and would measure nothing.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6024/?plugin=generation3d SEMIO_PROBE_OUT=scope/before bun 🐍️hover-cost-probe.mjs
 * Env: SEMIO_PROBE_EXAMPLE (oracle label, default Box Shell Preview), SEMIO_PROBE_HOVERS, SEMIO_PROBE_SELECTS, SEMIO_PROBE_MESH_WAIT.
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { ORACLES, meshStatsScript } from "./🐍️example-oracle.mjs";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6024/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "scope/cost");
const wantedExample = process.env.SEMIO_PROBE_EXAMPLE ?? "Box Shell Preview";
const hoverCount = Number(process.env.SEMIO_PROBE_HOVERS ?? 8);
const selectCount = Number(process.env.SEMIO_PROBE_SELECTS ?? 4);
const meshWait = Number(process.env.SEMIO_PROBE_MESH_WAIT ?? 90);
mkdirSync(outDir, { recursive: true });

const oracle = Object.values(ORACLES).find((entry) => entry.label === wantedExample || entry.slug === wantedExample);
if (!oracle) throw new Error(`no oracle named ${wantedExample}`);

const lines = [];
const faults = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.addInitScript(() => {
  try {
    globalThis.localStorage?.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
  } catch {
    /* a sandboxed tab has no storage; the build env switch is then the only arming path */
  }
});
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1400)}`));
page.on("pageerror", (e) => {
  faults.push({ t: Date.now() - t0, message: String(e?.message ?? e).slice(0, 300) });
  lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 600)}`);
});

/** 📏️ The host's own refresh-pass line, parsed back into the scope it was opened with. */
const REFRESH_LINE = "[DEBUG] refreshUi sections ";
const RENDER_LINE = "[DEBUG] gen3d render body=";

const parseRefresh = (line) => {
  const at = line.indexOf(REFRESH_LINE);
  if (at < 0) return null;
  try {
    return JSON.parse(line.slice(at + REFRESH_LINE.length));
  } catch {
    return null;
  }
};

/** 🧮️ Everything the console has said so far, as counters — a gesture's cost is the delta. */
const counters = () => {
  const refreshes = lines.map(parseRefresh).filter(Boolean);
  const renders = lines.filter((line) => line.includes(RENDER_LINE)).map((line) => line.slice(line.indexOf(RENDER_LINE) + RENDER_LINE.length).split(" ")[0]);
  return { refreshes, renders, raw: lines.length };
};

/** ⏳️ Quiescence: no new console line for `quiet` ms, or `limit` ms elapsed. A gesture's wall cost is
 * measured to this point, so a pass that lands late is still counted against the gesture that caused it. */
const settle = async (quiet = 600, limit = 12_000) => {
  const started = Date.now();
  let last = lines.length;
  let lastChange = Date.now();
  while (Date.now() - started < limit) {
    await page.waitForTimeout(100);
    if (lines.length !== last) {
      last = lines.length;
      lastChange = Date.now();
    } else if (Date.now() - lastChange >= quiet) break;
  }
  return Date.now() - started;
};

const paneSnap = async () => {
  const stats = await page.evaluate(meshStatsScript, oracle.previewMeshId);
  const dom = await page.evaluate(() => {
    const parse = (raw) => {
      try {
        return JSON.parse(raw ?? "null");
      } catch {
        return null;
      }
    };
    const panes = [...document.querySelectorAll("[data-meshes-json], [data-status-json]")].map((el) => {
      const status = parse(el.getAttribute("data-status-json"));
      return { selection: parse(el.getAttribute("data-selection-json")), phase: status?.phase ?? null, ratio: status?.progress?.ratio ?? null, fault: status?.fault?.code ?? null };
    });
    const combo = document.querySelector('[role="combobox"]');
    return { panes, example: combo?.innerText?.replace(/\s+/g, " ").trim() ?? null, inspector: [...document.querySelectorAll('[data-slot="panel"] [id*="procedural-play-inspector"]')].length };
  });
  return { ...dom, panes: dom.panes.map((pane, index) => ({ ...(stats[index] ?? {}), ...pane })) };
};
const previewPane = (snap) => snap.panes.find((pane) => pane.surfaceId && pane.surfaceId.endsWith("-preview")) ?? null;
const settledPane = (pane) => Boolean(pane) && pane.phase === "idle" && pane.ratio === 1 && !pane.fault;

const canvasBox = async () => {
  const canvas = page.locator("[data-meshes-json] canvas, .semio-world-3d-host canvas").last();
  if (!(await canvas.count())) return null;
  return canvas.boundingBox();
};

/** 🖱️ Up to `wanted` distinct canvas points that are provably over this example's geometry. */
const geometryPoints = async (box, wanted) => {
  const grid = [];
  for (let row = 1; row <= 5; row += 1) for (let col = 1; col <= 7; col += 1) grid.push([box.x + (box.width * col) / 8, box.y + (box.height * row) / 6]);
  grid.sort((a, b) => Math.hypot(a[0] - (box.x + box.width / 2), a[1] - (box.y + box.height / 2)) - Math.hypot(b[0] - (box.x + box.width / 2), b[1] - (box.y + box.height / 2)));
  const found = [];
  for (const [x, y] of grid) {
    if (found.length >= wanted) break;
    await page.mouse.move(x, y);
    await page.waitForTimeout(280);
    const target = previewPane(await paneSnap())?.selection?.hoverTarget ?? null;
    if (target?.id) found.push([Math.round(x), Math.round(y)]);
  }
  return found;
};

const gestures = [];
/** 🎬️ One gesture: park the pointer off the geometry, take the counters, act, settle, take them again. */
const measure = async (kind, index, act) => {
  const before = counters();
  const started = Date.now();
  await act();
  const wall = await settle();
  const after = counters();
  const passes = after.refreshes.slice(before.refreshes.length);
  const renders = after.renders.slice(before.renders.length);
  const row = {
    kind,
    index,
    wallMs: Date.now() - started - 0,
    settleMs: wall,
    refreshPasses: passes.length,
    scopes: passes.map((pass) => pass.scope?.kind ?? "?"),
    askedWindows: passes.flatMap((pass) => pass.asked ?? []),
    panelBodies: passes.flatMap((pass) => (pass.scope?.kind === "partial" ? (pass.scope.panelBodies ?? []) : pass.scope?.kind === "full" ? ["<all>"] : [])),
    windowBodies: passes.flatMap((pass) => (pass.scope?.kind === "partial" ? (pass.scope.windowBodies ?? []) : pass.scope?.kind === "full" ? ["<all>"] : [])),
    guestBodyRenders: renders.length,
    guestBodies: [...new Set(renders)],
  };
  gestures.push(row);
  console.log(`[DEBUG] ${kind}#${index} wall=${row.wallMs}ms passes=${row.refreshPasses} scopes=${row.scopes.join(",")} guestRenders=${row.guestBodyRenders} bodies=${row.guestBodies.join(",")}`);
  return row;
};

await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(20_000);
await page.locator('[id="framework.panel.inspection"]').first().click({ timeout: 8000 }).catch(() => {});
await page.waitForTimeout(1500);

const pickExample = async () => {
  const combo = page.locator('[role="combobox"]').first();
  await combo.click({ timeout: 8000 });
  await page.waitForTimeout(400);
  await page.locator('[role="option"]').filter({ hasText: oracle.label }).first().click({ timeout: 8000 });
};
await pickExample().catch((error) => lines.push(`pick failed ${String(error).slice(0, 200)}`));

let snap = null;
for (let i = 0; i < meshWait; i += 1) {
  await page.waitForTimeout(1000);
  snap = await paneSnap();
  if (snap.example === oracle.label && settledPane(previewPane(snap))) break;
}
const converged = Boolean(snap) && snap.example === oracle.label && settledPane(previewPane(snap));

const box = await canvasBox();
const points = box ? await geometryPoints(box, 3) : [];
const off = box ? [Math.round(box.x + 8), Math.round(box.y + 8)] : [4, 4];
await settle();

for (let i = 0; i < hoverCount && points.length > 0; i += 1) {
  const [x, y] = points[i % points.length];
  await page.mouse.move(off[0], off[1]);
  await settle(400, 6000);
  await measure("hover", i, async () => {
    await page.mouse.move(x, y);
  });
}

for (let i = 0; i < selectCount && points.length > 0; i += 1) {
  const [x, y] = points[i % points.length];
  await measure("select", i, async () => {
    await page.mouse.click(x, y);
  });
  await measure("clear", i, async () => {
    await page.keyboard.press("Escape");
  });
}

const summarize = (kind) => {
  const rows = gestures.filter((row) => row.kind === kind);
  if (rows.length === 0) return null;
  const mean = (pick) => Math.round(rows.reduce((sum, row) => sum + pick(row), 0) / rows.length);
  return {
    gestures: rows.length,
    meanWallMs: mean((row) => row.wallMs),
    meanRefreshPasses: Number((rows.reduce((sum, row) => sum + row.refreshPasses, 0) / rows.length).toFixed(2)),
    meanGuestBodyRenders: Number((rows.reduce((sum, row) => sum + row.guestBodyRenders, 0) / rows.length).toFixed(2)),
    scopes: [...new Set(rows.flatMap((row) => row.scopes))],
    windowBodies: [...new Set(rows.flatMap((row) => row.windowBodies))],
    panelBodies: [...new Set(rows.flatMap((row) => row.panelBodies))],
    guestBodies: [...new Set(rows.flatMap((row) => row.guestBodies))],
  };
};

const report = {
  url,
  example: oracle.label,
  converged,
  geometryPoints: points,
  pageFaults: faults,
  hover: summarize("hover"),
  select: summarize("select"),
  clear: summarize("clear"),
  gestures,
};
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(JSON.stringify({ converged, hover: report.hover, select: report.select, clear: report.clear, faults: faults.length }, null, 2));
await browser.close();
