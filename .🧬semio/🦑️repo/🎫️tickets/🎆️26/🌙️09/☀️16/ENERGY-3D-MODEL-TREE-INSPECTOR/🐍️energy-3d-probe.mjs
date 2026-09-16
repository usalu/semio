/** 🧊️ Energy 3d model-window probe (ticket 26/09/16/ENERGY-3D-MODEL-TREE-INSPECTOR, DoD 1 + the 3d half of DoD 2).
 *
 * Boots the energy react playground, then:
 *   1. finds the World3d window host — `[data-surface-id="window:<kind>"]` with `data-meshes-json`/`data-instances-json`
 *      (the two attributes `World3dHost` stamps on its root div, `🧰️framework/…/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:6989-7008`);
 *   2. asserts the BESTEST 600 scene carries at least SEMIO_PROBE_MIN_INSTANCES instances (6 surfaces + 2 windows = 8);
 *   3. switches the example through the navbar picker (Escape afterwards — the listbox stays open and covers the middle window)
 *      and asserts meshes/instances actually changed (digest + counts);
 *   4. picks in 3d by clicking the canvas centre (spiralling out if the centre is empty space) and reads the resulting
 *      selection from `data-selection-json` (what the pane PAINTS), `data-interaction-json`, the console, and any
 *      `aria-selected="true"` panel row;
 *   5. screenshots every step.
 *
 * Never throws: a missing window / missing lane is reported as a FAIL assertion with a plain-English message
 * ("window:energy.model.3d absent — the served build has no 3d window") and the process exits 1.
 *
 * Usage:
 *   cd .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ENERGY-3D-MODEL-TREE-INSPECTOR
 *   SEMIO_PROBE_OUT=energy-3d-1 bun 🐍️energy-3d-probe.mjs
 * Env: SEMIO_PROBE_URL (http://127.0.0.1:6106/?plugin=energy), SEMIO_PROBE_OUT, SEMIO_PROBE_SECONDS (boot budget, 120),
 *      SEMIO_PROBE_WINDOW_3D (window kind id, energy.model.3d), SEMIO_PROBE_EXAMPLE (BESTEST 620 — see the note at its declaration),
 *      SEMIO_PROBE_MIN_INSTANCES (8), SEMIO_PROBE_DOMAIN (energyModel).
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6106/?plugin=energy";
const bootSeconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 120);
const windowKind = process.env.SEMIO_PROBE_WINDOW_3D ?? "energy.model.3d";
const surfaceId = `window:${windowKind}`;
// 🔀️ BESTEST 620, not 900: 900 is 600's geometry with heavyweight constructions, so the mesh/instance lanes are
// byte-identical to 600 and the re-render digest check could never differ. 620 keeps 600's envelope but moves the two
// windows from the south wall to the east and west walls, so the fenestration instances really do move.
const example = process.env.SEMIO_PROBE_EXAMPLE ?? "BESTEST 620";
const minInstances = Number(process.env.SEMIO_PROBE_MIN_INSTANCES ?? 8);
const domain = process.env.SEMIO_PROBE_DOMAIN ?? "energyModel";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "energy-3d");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const report = { probe: "energy-3d", url, windowKind, surfaceId, example, minInstances, startedAt: new Date().toISOString(), assertions: [], steps: {} };
const flush = () => {
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
};
const note = (key, value) => { report.steps[key] = value; console.log(`[DEBUG] ${key} ${JSON.stringify(value).slice(0, 900)}`); flush(); };
const assert = (id, ok, detail) => { report.assertions.push({ id, ok: Boolean(ok), detail }); console.log(`${ok ? "PASS" : "FAIL"} ${id} — ${detail}`); flush(); };

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const shot = (name) => page.screenshot({ path: join(outDir, `${name}.png`), type: "png", fullPage: false }).catch(() => {});

/** 🔦️ Every world-ish surface in the DOM plus its lane sizes — used both to find our window and to prove it re-rendered. */
const worlds = () => page.evaluate(() => {
  const digest = (s) => { let h = 7; for (let i = 0; i < s.length; i++) h = (h * 31 + s.charCodeAt(i)) >>> 0; return h; };
  const count = (s) => { try { const v = JSON.parse(s ?? "null"); return Array.isArray(v) ? v.length : null; } catch { return null; } };
  return [...document.querySelectorAll("[data-surface-id]")].map((el) => {
    const meshes = el.getAttribute("data-meshes-json");
    const instances = el.getAttribute("data-instances-json");
    let sample = [];
    try { sample = (JSON.parse(instances ?? "[]") ?? []).slice(0, 12).map((i) => ({ id: i?.id, meshId: i?.meshId, label: i?.label, color: i?.color, objectKind: i?.objectKind })); } catch {}
    let meshIds = [];
    try { meshIds = (JSON.parse(meshes ?? "[]") ?? []).slice(0, 24).map((m) => m?.id); } catch {}
    let status = null;
    try { status = JSON.parse(el.getAttribute("data-status-json") ?? "null"); } catch {}
    const rect = el.getBoundingClientRect();
    return {
      surfaceId: el.getAttribute("data-surface-id"),
      world: meshes !== null || instances !== null,
      canvases: el.querySelectorAll("canvas").length,
      w: Math.round(rect.width), h: Math.round(rect.height),
      meshCount: count(meshes), instanceCount: count(instances),
      meshBytes: meshes?.length ?? 0, instanceBytes: instances?.length ?? 0,
      meshDigest: meshes ? digest(meshes) : null, instanceDigest: instances ? digest(instances) : null,
      meshIds, sample,
      statusPhase: status?.phase ?? null, statusFault: status?.fault?.code ?? null,
      selection: el.getAttribute("data-selection-json")?.slice(0, 600) ?? null,
      interaction: el.getAttribute("data-interaction-json")?.slice(0, 600) ?? null,
      camera: el.getAttribute("data-camera-json")?.slice(0, 300) ?? null,
    };
  });
});
const shell = () => page.evaluate(() => ({
  ready: document.documentElement.getAttribute("data-semio-os-ready"),
  error: document.documentElement.getAttribute("data-semio-os-error"),
  title: document.title,
  combobox: document.querySelector('[role="combobox"]')?.innerText?.replace(/\s+/g, " ").trim() ?? null,
  body: document.body.innerText.replace(/\s+/g, " ").slice(0, 900),
}));
const findWorld = async () => {
  const all = await worlds();
  const exact = all.find((w) => w.surfaceId === surfaceId);
  const fallback = all.find((w) => w.world && w.canvases > 0) ?? all.find((w) => w.world);
  return { all, exact: exact ?? null, fallback: fallback ?? null, target: exact ?? fallback ?? null };
};
/** 🔦️ Selected PANEL rows only — `aria-selected="true"` also marks the window dock tabs (`mode-dock-tab-*`), which are
 * always "selected" and would make a pick assertion pass without any pick happening. */
const selectedPanelRows = () => page.evaluate(() =>
  [...document.querySelectorAll('[id^="panel:"][aria-selected="true"], [role="treeitem"][aria-selected="true"], [role="row"][aria-selected="true"]')]
    .filter((e) => !/^mode-dock-tab/.test(e.id ?? ""))
    .map((e) => ({ id: e.id, text: e.textContent?.trim().replace(/\s+/g, " ").slice(0, 80) })).slice(0, 20));

try {
  // ── boot ────────────────────────────────────────────────────────────────
  await page.goto(url, { waitUntil: "domcontentloaded" });
  let s = null, w = null;
  for (let i = 0; i < bootSeconds; i++) {
    await page.waitForTimeout(1000);
    s = await shell();
    if (s.error) break;
    w = await findWorld();
    if (s.ready && w.all.length && i > 10) break;
  }
  note("boot", { shell: s, surfaces: w?.all?.map((x) => x.surfaceId) ?? [] });
  await shot("1-boot");
  assert("boot.ready", Boolean(s?.ready) && !s?.error, `data-semio-os-ready=${s?.ready ?? "null"} data-semio-os-error=${s?.error ?? "null"}`);

  // ── A1 the 3d window exists ─────────────────────────────────────────────
  w = await findWorld();
  note("worlds", w.all);
  if (!w.exact && !w.fallback) {
    assert("world3d.present", false, `${surfaceId} absent — the served build publishes no World3d surface at all (surfaces seen: ${w.all.map((x) => x.surfaceId).join(", ") || "none"}). Restage with 📜️activate-energy-react.sh once wave 1 lands.`);
  } else if (!w.exact) {
    assert("world3d.present", false, `${surfaceId} absent — a World3d surface exists under a DIFFERENT id (${w.fallback.surfaceId}); re-run with SEMIO_PROBE_WINDOW_3D=<kind id> if the window kind was renamed.`);
  } else {
    assert("world3d.present", true, `${surfaceId} present, ${w.exact.w}×${w.exact.h}px, ${w.exact.canvases} canvas(es), status=${w.exact.statusPhase ?? "none"} fault=${w.exact.statusFault ?? "none"}`);
  }
  const target = w.target;
  const before = target ? { ...target } : null;

  // ── A2 instance count for BESTEST 600 ───────────────────────────────────
  if (!target) assert("world3d.instances", false, "skipped — no World3d surface to read");
  else if (target.instanceCount === null) assert("world3d.instances", false, `data-instances-json absent or unparseable on ${target.surfaceId} (${target.instanceBytes} bytes) — the scene published no instances lane`);
  else assert("world3d.instances", target.instanceCount >= minInstances, `${target.instanceCount} instances / ${target.meshCount ?? "?"} meshes (need ≥ ${minInstances} = 6 surfaces + 2 windows); ids: ${target.sample.map((i) => i.id).join(", ").slice(0, 200)}`);
  if (target) assert("world3d.canvas", target.canvases > 0, `${target.canvases} canvas element(s) inside the host (0 means the three.js renderer never mounted)`);
  else assert("world3d.canvas", false, "skipped — no World3d surface to read");

  // ── A3 example switch re-renders the scene ──────────────────────────────
  let picked = "skipped";
  const combo = page.locator('[role="combobox"]').first();
  if (await combo.count()) {
    await combo.click({ timeout: 5000 }).catch((e) => (picked = String(e).slice(0, 120)));
    await page.waitForTimeout(700);
    const option = page.locator('[role="option"]').filter({ hasText: example }).first();
    if (await option.count()) picked = await option.click({ timeout: 5000 }).then(() => `ok:${example}`).catch((e) => String(e).slice(0, 120));
    else picked = "no-option";
    // 🧹️ The picker's listbox stays open after a pick and covers the middle window (predecessor ticket pitfall).
    await page.keyboard.press("Escape");
    await page.waitForTimeout(400);
    if (await page.locator('[role="option"]').count()) { await page.mouse.click(800, 760); await page.waitForTimeout(400); }
  } else picked = "no-combobox";
  let after = null;
  for (let i = 0; i < 60; i++) {
    await page.waitForTimeout(500);
    const now = await findWorld();
    after = now.target;
    if (after && before && (after.instanceDigest !== before.instanceDigest || after.meshDigest !== before.meshDigest)) break;
    if (!before) break;
  }
  const shellAfter = await shell();
  note("exampleSwitch", { picked, model: shellAfter.combobox, before: before && { meshCount: before.meshCount, instanceCount: before.instanceCount, meshDigest: before.meshDigest, instanceDigest: before.instanceDigest }, after: after && { meshCount: after.meshCount, instanceCount: after.instanceCount, meshDigest: after.meshDigest, instanceDigest: after.instanceDigest } });
  await shot("2-example-switch");
  if (!before || !after) assert("world3d.exampleRerender", false, `skipped — no World3d surface before/after the switch (picker said ${picked})`);
  else assert("world3d.exampleRerender", after.instanceDigest !== before.instanceDigest || after.meshDigest !== before.meshDigest,
    `picker=${picked}; instances ${before.instanceCount}→${after.instanceCount} (digest ${before.instanceDigest}→${after.instanceDigest}), meshes ${before.meshCount}→${after.meshCount} (digest ${before.meshDigest}→${after.meshDigest})`);

  // ── A4 3d pick ──────────────────────────────────────────────────────────
  const consoleFrom = lines.length;
  let pick = { attempts: [], selection: null, interaction: null, consoleHits: [], selectedRows: [] };
  const host = page.locator(`[data-surface-id="${(after ?? before)?.surfaceId ?? surfaceId}"]`).first();
  if (await host.count()) {
    const box = await host.boundingBox();
    if (box) {
      // spiral out from the centre: the model's centre of view is usually a surface, but a hollow box can show sky there
      const offsets = [[0, 0], [0, -0.15], [0.15, 0], [-0.15, 0], [0, 0.15], [0.22, -0.18], [-0.22, 0.18]];
      for (const [dx, dy] of offsets) {
        const x = box.x + box.width * (0.5 + dx);
        const y = box.y + box.height * (0.5 + dy);
        await page.mouse.click(x, y);
        await page.waitForTimeout(1800);
        const now = (await findWorld()).target;
        let ids = [];
        try { const sel = JSON.parse(now?.selection ?? "null"); ids = sel?.selectedIds ?? sel?.ids ?? []; } catch {}
        pick.attempts.push({ x: Math.round(x), y: Math.round(y), selectedIds: ids, selection: now?.selection ?? null });
        pick.selection = now?.selection ?? null;
        pick.interaction = now?.interaction ?? null;
        if (ids.length) break;
      }
    } else pick.attempts.push({ error: "the world host has no bounding box (zero-size pane)" });
  } else pick.attempts.push({ error: `no element matches [data-surface-id="${surfaceId}"]` });
  pick.consoleHits = lines.slice(consoleFrom).filter((l) => /interactionSelect|interactionHover|interaction-select|worldPick|energyModel/i.test(l)).slice(0, 12).map((l) => l.slice(0, 300));
  pick.selectedRows = await selectedPanelRows();
  note("pick", pick);
  await shot("3-pick");
  const lastIds = pick.attempts.filter((a) => a.selectedIds?.length).pop()?.selectedIds ?? [];
  const domainHit = pick.consoleHits.some((l) => l.includes(domain)) || (pick.interaction ?? "").includes(domain) || (pick.selection ?? "").length > 2;
  assert("world3d.pick", lastIds.length > 0 || pick.selectedRows.length > 0,
    `selectedIds=${JSON.stringify(lastIds)} · aria-selected rows=${JSON.stringify(pick.selectedRows.map((r) => r.id)).slice(0, 200)} · domain '${domain}' seen=${domainHit} · console hits=${pick.consoleHits.length} (${pick.attempts.length} click attempt(s))`);

  // ── faults ──────────────────────────────────────────────────────────────
  // 🧹️ `contributions push refused empty pack` is a pre-existing baseline console error of the served energy build
  // (recorded in 🗑️generated/baseline-3d on 2026-09-16, before any wave-1 edit) — ignore it, keep every other fault.
  const BENIGN = /contributions push refused empty pack/;
  const faults = lines.filter((l) => /pageerror|trapped|panicked|unreachable|shell fault|surface-render|fixed-capacity|dropped action|Unknown action|window-context-required/i.test(l) && !BENIGN.test(l)).map((l) => l.slice(0, 300));
  note("faults", faults.slice(0, 20));
  assert("no.faults", faults.length === 0, `${faults.length} fault line(s)${faults.length ? `: ${faults[0]}` : ""}`);
} catch (error) {
  report.crash = String(error?.stack ?? error).slice(0, 2000);
  assert("probe.completed", false, `probe threw: ${String(error).slice(0, 300)}`);
  await shot("error");
}

report.finishedAt = new Date().toISOString();
report.passed = report.assertions.filter((a) => a.ok).length;
report.failed = report.assertions.filter((a) => !a.ok).length;
report.result = report.failed === 0 ? "PASS" : "FAIL";
flush();
console.log(`RESULT=${report.result} passed=${report.passed} failed=${report.failed} out=${outDir}`);
for (const a of report.assertions.filter((x) => !x.ok)) console.log(`  FAIL ${a.id}: ${a.detail}`);
await browser.close();
process.exitCode = report.failed === 0 ? 0 : 1;
