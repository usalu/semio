/** 🧭 User-journey probe for the procedural 3d playground: boot → every example → generate mode → viewer role → every example.
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6018/?plugin=generation3d SEMIO_PROBE_OUT=journey-1 bun 🐍️journey-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

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
const snap = () => page.evaluate(() => {
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
    return { surfaceId: el.getAttribute("data-surface-id"), meshes, instances, phase: st?.phase, ratio: st?.progress?.ratio, meshesLen: st?.debug?.meshesLen, fault: st?.fault?.code ?? null, nodeStatuses, widgetIds, w: el.offsetWidth, h: el.offsetHeight };
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
/** ⚖️ A step has converged only when the flow window publishes the graph of the example the picker
 * NAMES. Comparing against the eval status map alone declared a stale window converged in 3 s and
 * screenshotted the previous example for the whole journey. */
const converged = (s, expectWidgets) => {
  const main = s.hosts.find((h) => h.surfaceId === "window:procedural-main");
  const preview = s.hosts.find((h) => h.surfaceId && h.surfaceId.endsWith("-preview"));
  const nodes = main?.nodeStatuses ?? [];
  const nodesOk = nodes.length > 0 && nodes.every((st) => st === "ok" || st === "error");
  const graphOk = expectWidgets === undefined ? (main?.widgetIds ?? []).length > 0 : JSON.stringify([...(main?.widgetIds ?? [])].sort()) === JSON.stringify([...expectWidgets].sort());
  return Boolean(preview) && preview.phase === "idle" && preview.ratio === 1 && nodesOk && graphOk;
};
const waitMeshes = async (label, seconds, expectWidgets) => {
  let last = null; let stable = 0; const start = Date.now();
  for (let i = 0; i < seconds; i++) {
    await page.waitForTimeout(1000); last = await snap();
    if (converged(last, expectWidgets)) { stable += 1; if (stable >= 3) break; } else stable = 0;
  }
  const row = { label, t: Date.now() - t0, seconds: (Date.now() - start) / 1000, converged: converged(last, expectWidgets), expectWidgets: expectWidgets ?? null, meshes: last.meshes, example: last.example, hosts: last.hosts, windows: last.windows, modes: last.modes, roles: last.roles, faults: last.faults };
  results.push(row); console.log(`[DEBUG] ${label}: converged=${row.converged} in ${row.seconds.toFixed(0)}s meshes=${row.meshes} example=${JSON.stringify(row.example)} hosts=${JSON.stringify(row.hosts.map((h) => [h.surfaceId, h.meshes, h.phase, h.fault, h.widgetIds ?? h.nodeStatuses]))}`);
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
  await waitMeshes(`edit:${text}`, meshWait, EXPECTED_WIDGETS[text]);
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
s = await waitMeshes("viewer-role", meshWait);
for (const text of await listOptions()) {
  await pick(text);
  await waitMeshes(`view:${text}`, meshWait, EXPECTED_WIDGETS[text]);
}
writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("DONE steps", results.length, "meshSteps", results.filter((r) => r.meshes > 0).length);
await browser.close();
