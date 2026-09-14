/** 🧭 User-journey probe for the procedural 3d playground: boot → every example → generate mode → viewer role → every example.
 *
 * 🧿 Every picked example is measured against its OWN committed fixture — exact `delivery.meshes`,
 * the delivery triangle/edge floors and the `expect.boundingBoxMin/Max` extent — through
 * `🐍️example-oracle.mjs`, which documents both the fixture schema and the `data-*-json` DOM lanes.
 * Before that, a mesh count was read and printed but never compared to anything, so a preview that
 * published the wrong geometry — or none at all, as four viewer-role rows did — still converged.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6018/?plugin=generation3d SEMIO_PROBE_OUT=journey-1 bun 🐍️journey-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { gradeMeshes, meshStatsScript, oracleForLabel } from "./🐍️example-oracle.mjs";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "journey");
const meshWait = Number(process.env.SEMIO_PROBE_MESH_WAIT ?? 60);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: process.env.SEMIO_PROBE_WEBGPU === "0" ? [] : ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1500)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`));
const domSnap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return s?.slice(0, 200); } };
  const hosts = [...document.querySelectorAll("[data-status-json], [data-meshes-json]")].map((el) => {
    let meshes = 0; try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); meshes = Array.isArray(v) ? v.length : 0; } catch {}
    let instances = 0; try { const v = JSON.parse(el.getAttribute("data-instances-json") ?? "[]"); instances = Array.isArray(v) ? v.length : 0; } catch {}
    const st = parse(el.getAttribute("data-status-json"));
    const nodeStatuses = st && typeof st === "object" && !st.phase ? Object.values(st).map((v) => v?.status ?? "?") : undefined;
    // 🪪️ `data-status-json` is the EVALUATION's per-node status map, which lags the document; the
    // published GRAPH is `data-fixture-json`. Reading the status map as "the graph" is what made this
    // probe call a stale flow window converged (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    const fx = parse(el.getAttribute("data-fixture-json"));
    const widgetIds = fx && Array.isArray(fx.widgets) ? fx.widgets.map((w) => { const inner = w && typeof w === "object" ? Object.values(w)[0] : null; return (w && w.id) ?? (inner && inner.id) ?? null; }).filter(Boolean) : undefined;
    return { surfaceId: el.getAttribute("data-surface-id"), meshes, instances, phase: st?.phase, ratio: st?.progress?.ratio, computing: st?.computing ?? null, cancellable: st?.cancellable ?? null, cancelAction: st?.cancelAction ?? null, statusKeys: st && typeof st === "object" && st.phase ? Object.keys(st) : undefined, meshesLen: st?.debug?.meshesLen, fault: st?.fault?.code ?? null, nodeStatuses, widgetIds, w: el.offsetWidth, h: el.offsetHeight };
  });
  const sel = document.querySelector("select");
  const combo = document.querySelector('[role="combobox"]');
  const pressed = (id) => document.getElementById(id)?.getAttribute("aria-pressed");
  return {
    hosts, meshes: hosts.reduce((n, h) => n + h.meshes, 0),
    example: sel?.selectedOptions[0]?.text ?? combo?.innerText?.replace(/\s+/g, " ").trim() ?? null, options: sel ? [...sel.options].map((o) => ({ value: o.value, text: o.text })) : [],
    windows: [...document.querySelectorAll("[data-window-instance-id]")].map((e) => e.getAttribute("data-window-instance-id")),
    modes: { edit: pressed("playground.navbar.modes.edit"), generate: pressed("playground.navbar.modes.generate") },
    roles: { editor: pressed("playground.navbar.roles.editor"), viewer: pressed("playground.navbar.roles.viewer") },
    faults: [...document.querySelectorAll("[data-fault-code]")].map((e) => e.getAttribute("data-fault-code")).slice(0, 10),
  };
});
/** 🧿 The DOM snapshot with each preview host's DELIVERED mesh numbers attached — mesh/instance
 * counts, the preview mesh's triangle, edge-segment and bounding-box extent. Both queries select
 * `[data-status-json], [data-meshes-json]` and therefore walk the same elements in document order,
 * so the rows line up by index. The position buffers stay in the page: only the reduced numbers
 * cross the bridge, which is what keeps `sphere-cut-with-torus`' 1100-triangle payload affordable
 * once per second. */
const snap = async (previewMeshId = null) => {
  const base = await domSnap();
  const stats = await page.evaluate(meshStatsScript, previewMeshId);
  return { ...base, hosts: base.hosts.map((h, i) => ({ ...h, stats: stats[i] ?? null })) };
};
/** 📚️ The authored widget ids of every bundled example, by picker label — the oracle a pick is
 * measured against (`🧫️fixtures/🎨️example-switch.json` is the language-agnostic twin). */
const EXPECTED_WIDGETS = {
  "Hexagonal Mushroom Column": ["height", "radius", "sides", "profile", "extrusion-axis", "extrude", "column-preview"],
  "Rectangle Extrude Volume": ["width", "height", "distance", "rect", "vector", "extrude", "volume"],
  "Sphere Cut With Torus": ["slider_2", "brep_prim3d_sphere_3", "brep_prim3d_torus_4", "brep_bool_cut_5", "brep_measure_volume_2", "preview_3"],
  "Box Fillet Preview": ["size", "radius", "box", "fillet", "preview"],
  "Sphere Box Fuse": ["radius", "size", "sphere", "box", "fuse", "preview"],
  "Face Sweep Extrude": ["width", "height", "distance", "rect", "face", "vector", "extrude"],
  "Rectangle Wire Preview": ["width", "height", "rect"],
  "Box Shell Preview": ["size", "thickness", "box", "shell"],
  "No example": [],
};
/** 📈️ THE settled predicate, read off the published `World3dComputeStatusV1` and nothing else, so it
 * means the same thing on all THREE preview windows — edit's `procedural-preview`, generate's
 * `generation3d-generate-preview` and the viewer's `procedural-view-preview`.
 *
 * 🪪️ An empty document settles here too: the ledger's ratio is 1 for a zero total with nothing in
 * flight, which is `idle-empty`. The old predicate reached past the contract into the FLOW window's
 * per-node status map, which the viewer and the generate preview do not have at all — so every
 * `view:*` step and `generate-added` reported `converged=false` for the full 60 s budget while the
 * surface had in fact been idle at `ratio 1` the whole time
 * (`🗑️generated/s4-journey-1/results.json`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
const settled = (h) => Boolean(h) && h.phase === "idle" && h.ratio === 1 && h.computing !== true && !h.fault;
/** ⚖️ A step has converged when EVERY attached preview window is settled, the picker names what was
 * picked, and — only where a flow window exists, i.e. edit mode — that window publishes the graph of
 * the example the picker NAMES. Comparing against the eval status map alone declared a stale window
 * converged in 3 s and screenshotted the previous example for the whole journey, so the graph oracle
 * stays; it is simply not asked of a surface that has no graph window. */
const converged = (s, expectWidgets, expectLabel, oracle, expectSurface) => {
  const main = s.hosts.find((h) => h.surfaceId === "window:procedural-main");
  const previews = s.hosts.filter((h) => h.surfaceId && h.surfaceId.endsWith("-preview"));
  if (previews.length === 0 || !previews.every(settled)) return false;
  // 🚪️ A row that names a MODE's own preview window must not settle on whichever window is still
  // mounted from the previous one. The viewer-role row settled on the edit preview while the viewer's
  // had not mounted yet, so the next pick was made against a surface that then swapped under it and
  // swallowed it (`🗑️generated/react-oracle/journey-1`, `view:No example`).
  if (expectSurface && !previews.some((h) => h.surfaceId === expectSurface)) return false;
  // 🕳️ `No example` is an EMPTY state everywhere: a row that promises no example must leave no mesh,
  // no instance and no selection behind on any preview window.
  if (expectWidgets !== undefined && expectWidgets.length === 0 && previews.some((h) => h.meshes > 0 || h.instances > 0)) return false;
  if (expectLabel !== undefined && s.example !== expectLabel) return false;
  // 🧿 A NON-empty example owes its committed geometry on every preview window it mounts: exactly
  // `delivery.meshes` meshes and instances, the delivery triangle/edge floors, and an extent inside
  // the fixture's own bounding box. Requiring only `settled()` let four viewer-role rows converge on
  // an empty canvas (`🗑️generated/react-verify/scoreboard.json`, 2026-09-14 15:18).
  if (oracle && previews.some((h) => !gradeMeshes(oracle, h.stats).ok)) return false;
  if (!main) return true;
  const nodes = main.nodeStatuses ?? [];
  const nodesOk = nodes.every((st) => st === "ok" || st === "error") && (nodes.length > 0 || (expectWidgets !== undefined && expectWidgets.length === 0));
  const graphOk = expectWidgets === undefined ? (main.widgetIds ?? []).length > 0 : JSON.stringify([...(main.widgetIds ?? [])].sort()) === JSON.stringify([...expectWidgets].sort());
  return nodesOk && graphOk;
};
/** ⏱️ One journey row. `expectMeshes` names the example whose committed fixture this row is held to;
 * pass `null` for the rows that mount a preview no fixture describes (generate mode's own window
 * before a generation is evaluated). */
const waitMeshes = async (label, seconds, expectWidgets, expectLabel, expectMeshes, expectSurface) => {
  const oracle = expectMeshes === null ? null : (oracleForLabel(expectMeshes ?? expectLabel) ?? null);
  let last = null; let stable = 0; const start = Date.now();
  for (let i = 0; i < seconds; i++) {
    await page.waitForTimeout(1000); last = await snap(oracle?.previewMeshId ?? null);
    if (converged(last, expectWidgets, expectLabel, oracle, expectSurface)) { stable += 1; if (stable >= 3) break; } else stable = 0;
  }
  const previews = last.hosts.filter((h) => h.surfaceId && h.surfaceId.endsWith("-preview"));
  const meshGrades = oracle ? previews.map((h) => ({ surfaceId: h.surfaceId, ...gradeMeshes(oracle, h.stats) })) : [];
  const row = { label, t: Date.now() - t0, seconds: (Date.now() - start) / 1000, converged: converged(last, expectWidgets, expectLabel, oracle, expectSurface), expectSurface: expectSurface ?? null, expectWidgets: expectWidgets ?? null, expectLabel: expectLabel ?? null, meshes: last.meshes, example: last.example, hosts: last.hosts, windows: last.windows, modes: last.modes, roles: last.roles, faults: last.faults,
    oracle: oracle ? { slug: oracle.slug, previewMeshId: oracle.previewMeshId, meshes: oracle.meshes, minTriangles: oracle.minTriangles, minEdgeSegments: oracle.minEdgeSegments, boundingBoxMin: oracle.boundingBoxMin, boundingBoxMax: oracle.boundingBoxMax, boundingBoxTolerance: oracle.boundingBoxTolerance } : null,
    measured: previews.map((h) => ({ surfaceId: h.surfaceId, meshCount: h.stats?.meshCount ?? 0, instanceCount: h.stats?.instanceCount ?? 0, meshIds: h.stats?.meshIds ?? [], preview: h.stats?.preview ?? null })),
    meshOk: oracle ? meshGrades.every((g) => g.ok) : null, meshReasons: meshGrades.flatMap((g) => g.reasons.map((r) => `${g.surfaceId}: ${r}`)) };
  results.push(row); console.log(`[DEBUG] ${label}: converged=${row.converged} in ${row.seconds.toFixed(0)}s meshes=${row.meshes} meshOk=${row.meshOk} example=${JSON.stringify(row.example)} oracle=${JSON.stringify(row.oracle && [row.oracle.meshes, row.oracle.previewMeshId])} measured=${JSON.stringify(row.measured.map((m) => [m.surfaceId, m.meshCount, m.preview?.triangles ?? null, m.preview?.bounds ?? null]))} reasons=${JSON.stringify(row.meshReasons)}`);
  await page.screenshot({ path: join(outDir, `${results.length}-${label.replace(/[^a-z0-9]+/gi, "-")}.png`) });
  return last;
};
const results = [];
await page.goto(url, { waitUntil: "domcontentloaded" });
let s = await waitMeshes("boot", meshWait + 30);
const listOptions = async () => {
  const combo = page.locator('[role="combobox"]').first();
  if (!(await combo.count())) return [];
  await combo.click({ timeout: 4000 }); await page.waitForTimeout(400);
  const texts = await page.locator('[role="option"]').allInnerTexts();
  await page.keyboard.press("Escape"); await page.waitForTimeout(200);
  return texts.map((t) => t.replace(/\s+/g, " ").trim()).filter(Boolean);
};
const pick = async (text) => {
  const combo = page.locator('[role="combobox"]').first();
  await combo.click({ timeout: 4000 }); await page.waitForTimeout(400);
  await page.locator('[role="option"]').filter({ hasText: text }).first().click({ timeout: 4000 });
};
const options = await listOptions();
console.log("[DEBUG] options", JSON.stringify(options));
for (const text of options) {
  await pick(text);
  await waitMeshes(`edit:${text}`, meshWait, EXPECTED_WIDGETS[text], text, undefined, "window:procedural-preview");
}
await page.keyboard.press("Meta+Alt+ArrowRight");
await waitMeshes("generate-mode", 10);
{
  const add = page.locator(':text-is("Add Generation"), :text-is("Generation hinzufügen"), [data-action-id="addGeneration"]').first();
  if (await add.count()) { await add.click({ timeout: 4000 }); console.log("[DEBUG] clicked Add Generation"); }
  else console.log("[DEBUG] Add Generation row not found");
  await waitMeshes("generate-added", meshWait);
}
await page.keyboard.press("Meta+Alt+ArrowLeft");
await waitMeshes("back-to-edit", 5);
await page.keyboard.press("Meta+Alt+V");
s = await waitMeshes("viewer-role", meshWait, undefined, undefined, undefined, "window:procedural-view-preview");
for (const text of await listOptions()) {
  await pick(text);
  await waitMeshes(`view:${text}`, meshWait, EXPECTED_WIDGETS[text], text, undefined, "window:procedural-view-preview");
}
writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("DONE steps", results.length, "meshSteps", results.filter((r) => r.meshes > 0).length);
await browser.close();
