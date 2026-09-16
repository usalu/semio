/** 🌳️ Energy artifact-tree + inspector panel probe (ticket 26/09/16/ENERGY-3D-MODEL-TREE-INSPECTOR, DoD 2 + 3).
 *
 * Written against the LIVE wave-1 DOM (measured 2026-09-16 with 🐍️energy-dom-dump.mjs), not against a guess:
 *   · the tree panel publishes rows under `panel:energy-model-artifact/…` — section rows keyed
 *     `energy-model-artifact.<family>` and entity rows keyed by the RAW entity id (`40` = a surface, `50` = a window);
 *   · the inspector publishes rows under `panel:energy-model-inspection/…`, control ids shaped
 *     `energy-model-inspection.<kind>.<field>.<control>` (e.g. `…​.surface.name.input`);
 *   · the panel tabs are `framework.panel.artifact` and `framework.panel.inspection` and they share ONE dock anchor —
 *     **only one is mounted at a time**, so opening Inspection UNMOUNTS every `panel:energy-model-artifact/` row. The
 *     first draft of this probe clicked a tree row, switched to Inspection, and then found the next tree row "absent";
 *     every step here therefore re-opens the tab it needs and POLLS for the namespace instead of sleeping a fixed 2.5 s
 *     (a panel body can take well over 2.5 s to publish after its tab is clicked).
 *
 * Steps: boot → artifact tab → assert a row for every family → select a surface (tree click, else a 3d pick) → assert
 * the inspector shows a `.surface.` form → select a fenestration the same way → assert a `.fenestration.` form → find
 * the u-value control by id substring → set SEMIO_PROBE_VALUE → commit → assert the value reads back AND a
 * `[DEBUG] history patch applied` line with a non-empty label appeared → screenshot every step.
 *
 * The commit is tried in two ways and the report says WHICH one worked: (a) playwright `fill` + Enter + blur, then if no
 * history line lands, (b) React's native value setter plus bubbling `input`/`change` events + Enter + blur.
 *
 * Never throws: an absent panel/row/control is a FAIL assertion with a plain-English message, exit code 1.
 *
 * Usage:
 *   cd .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ENERGY-3D-MODEL-TREE-INSPECTOR
 *   SEMIO_PROBE_OUT=energy-panels-1 bun 🐍️energy-panels-probe.mjs
 * Env: SEMIO_PROBE_URL, SEMIO_PROBE_OUT, SEMIO_PROBE_SECONDS (120), SEMIO_PROBE_PANEL_SECONDS (60, per-tab publish budget),
 *      SEMIO_PROBE_TREE_TAB (framework.panel.artifact), SEMIO_PROBE_INSPECTOR_TAB (framework.panel.inspection),
 *      SEMIO_PROBE_TREE_NS (energy-model-artifact), SEMIO_PROBE_INSPECTOR_NS (energy-model-inspection),
 *      SEMIO_PROBE_SURFACE_ROW (40), SEMIO_PROBE_FENESTRATION_ROW (50), SEMIO_PROBE_FENESTRATION_ROW_ALT (51),
 *      SEMIO_PROBE_FIELD (`u.?value|u_value`), SEMIO_PROBE_VALUE (1.5), SEMIO_PROBE_WINDOW_3D (energy.model.3d).
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6106/?plugin=energy";
const bootSeconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 120);
const panelSeconds = Number(process.env.SEMIO_PROBE_PANEL_SECONDS ?? 60);
const treeTab = process.env.SEMIO_PROBE_TREE_TAB ?? "framework.panel.artifact";
const inspectorTab = process.env.SEMIO_PROBE_INSPECTOR_TAB ?? "framework.panel.inspection";
const treeNsHint = process.env.SEMIO_PROBE_TREE_NS ?? "energy-model-artifact";
const inspectorNsHint = process.env.SEMIO_PROBE_INSPECTOR_NS ?? "energy-model-inspection";
const surfaceRowId = process.env.SEMIO_PROBE_SURFACE_ROW ?? "40";
const fenestrationRowId = process.env.SEMIO_PROBE_FENESTRATION_ROW ?? "50";
const altFenestrationRowId = process.env.SEMIO_PROBE_FENESTRATION_ROW_ALT ?? "51";
const fieldPattern = new RegExp(process.env.SEMIO_PROBE_FIELD ?? "u.?value|u_value", "i");
const newValue = process.env.SEMIO_PROBE_VALUE ?? "1.5";
const surface3d = `window:${process.env.SEMIO_PROBE_WINDOW_3D ?? "energy.model.3d"}`;
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "energy-panels");
mkdirSync(outDir, { recursive: true });

const FAMILIES = [
  { key: "zones", re: /\bzones?\b|\bzonen?\b/i },
  { key: "surfaces", re: /\bsurfaces?\b|\bfl[äa]chen?\b|wall|roof|floor|ceiling/i },
  { key: "fenestrations", re: /fenestration|\bwindows?\b|\bfenster\b|glaz/i },
  { key: "materials", re: /\bmaterials?\b|\bmaterialien\b/i },
  { key: "constructions", re: /\bconstructions?\b|\bkonstruktion/i },
];

const lines = [];
const t0 = Date.now();
const report = { probe: "energy-panels", url, treeTab, inspectorTab, treeNsHint, inspectorNsHint, field: String(fieldPattern), newValue, startedAt: new Date().toISOString(), assertions: [], steps: {} };
const flush = () => { writeFileSync(join(outDir, "console.txt"), lines.join("\n")); writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2)); };
const note = (key, value) => { report.steps[key] = value; console.log(`[DEBUG] ${key} ${JSON.stringify(value).slice(0, 900)}`); flush(); };
const assert = (id, ok, detail) => { report.assertions.push({ id, ok: Boolean(ok), detail }); console.log(`${ok ? "PASS" : "FAIL"} ${id} — ${detail}`); flush(); };

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 1200)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const shot = (name) => page.screenshot({ path: join(outDir, `${name}.png`), type: "png" }).catch(() => {});

const shell = () => page.evaluate(() => ({
  ready: document.documentElement.getAttribute("data-semio-os-ready"),
  error: document.documentElement.getAttribute("data-semio-os-error"),
  surfaces: [...document.querySelectorAll("[data-surface-id]")].map((e) => e.getAttribute("data-surface-id")),
  tabs: [...document.querySelectorAll('[id^="framework.panel"]')].map((e) => ({ id: e.id, text: (e.textContent ?? "").trim().slice(0, 24) })),
}));
/** 🔦️ Every `panel:<ns>/<id>` row in the DOM, grouped by namespace — the discovery that survives a panel rename. */
const panelRows = () => page.evaluate(() => {
  const out = {};
  for (const el of document.querySelectorAll('[id^="panel:"]')) {
    const m = /^panel:([^/]+)\/(.*)$/.exec(el.id);
    if (!m) continue;
    const r = el.getBoundingClientRect();
    (out[m[1]] ??= []).push({
      id: m[2], domId: el.id, tag: el.tagName, role: el.getAttribute("role"), selected: el.getAttribute("aria-selected"),
      visible: r.width > 0 && r.height > 0, type: el.getAttribute("type"),
      value: "value" in el ? String(el.value).slice(0, 40) : null,
      text: (el.textContent ?? "").trim().replace(/\s+/g, " ").slice(0, 90),
    });
  }
  return out;
});
/** ⚠️ `exclude` is load-bearing: the two namespaces are `energy-model-artifact` and `energy-model-inspection`, and a
 * fallback regex of /artifact|model/ matched the INSPECTION namespace (it contains "model"). The probe then treated the
 * inspector's rows as the tree's, could not find row "50" in them, and reported "no-tree-row" for a tree that was fine. */
const nsFor = (rows, hint, includeRe, excludeRe) => {
  if (rows[hint]?.length) return hint;
  return Object.keys(rows).find((ns) => includeRe.test(ns) && !excludeRe.test(ns) && rows[ns].length) ?? null;
};
/** 🗂️ Open a panel tab and POLL until ITS namespace publishes rows — the two panels share one dock anchor, so this is
 * also how we get the tree back after a look at the inspector. The tab is re-clicked every 10 s: the inspection body
 * has been observed taking > 40 s to publish after the first click, and a click that lands while the other panel is
 * still committing can be swallowed. */
const openPanel = async (tabId, nsHint, nsRe, excludeRe) => {
  const tab = page.locator(`[id="${tabId}"]`).first();
  if (!(await tab.count())) return { how: `tab ${tabId} absent`, ns: null, rows: [] };
  await tab.click({ force: true, timeout: 8000 }).catch(() => {});
  for (let i = 0; i < panelSeconds * 2; i++) {
    await page.waitForTimeout(500);
    if (i > 0 && i % 20 === 0) await tab.click({ force: true, timeout: 8000 }).catch(() => {});
    const rows = await panelRows();
    const ns = nsFor(rows, nsHint, nsRe, excludeRe);
    if (ns && rows[ns].length) return { how: `ok after ${(i + 1) * 500}ms${i >= 20 ? ` (${Math.floor(i / 20)} re-click(s))` : ""}`, ns, rows: rows[ns] };
  }
  const rows = await panelRows();
  return { how: `clicked ${tabId} but no namespace matching ${nsRe} (excluding ${excludeRe}) published within ${panelSeconds}s (namespaces seen: ${Object.keys(rows).join(", ") || "none"})`, ns: null, rows: [] };
};
const openTree = () => openPanel(treeTab, treeNsHint, /artifact|artefakt|tree|structure/i, /inspect|eigenschaft/i);
const openInspector = () => openPanel(inspectorTab, inspectorNsHint, /inspect|eigenschaft/i, /artifact|artefakt/i);
const clickRow = async (domId) => {
  const row = page.locator(`[id="${domId}"]`).first();
  if (!(await row.count())) return `absent:${domId}`;
  await row.evaluate((el) => el.scrollIntoView({ block: "center" })).catch(() => {});
  await page.waitForTimeout(250);
  const box = await row.boundingBox();
  if (!box) return `no-box:${domId}`;
  await page.mouse.click(box.x + Math.min(100, box.width / 2), box.y + box.height / 2);
  await page.waitForTimeout(2000);
  return "ok";
};
const findRow = (tree, rowId) => tree.rows.find((r) => r.id === rowId) ?? tree.rows.find((r) => r.id.endsWith(`/${rowId}`) || r.id.endsWith(`.${rowId}`));
/** 🎯️ Select one entity, in four escalating ways, and report WHICH one worked:
 *   1. click its tree row;
 *   2. if the row is not there, click the tree's own "…more" rows and look again — the tree pages a node's children,
 *      and selecting a surface replaces its window rows with `energy-model-artifact.zones.<id>.windows.more`
 *      (measured: 48 rows at boot with `50`/`51` visible → 44 rows with a single `.windows.more` after picking `40`);
 *   3. try the sibling id (a second window of the same surface);
 *   4. fall back to a 3d pick on the World3d canvas. */
const selectEntity = async (rowId, tree, altRowId = null) => {
  const attempts = [];
  let row = findRow(tree, rowId);
  for (let round = 0; !row && round < 3; round++) {
    const more = tree.rows.filter((r) => /\.more$/i.test(r.id) || /^(more|mehr|weitere)\b/i.test(r.text));
    if (!more.length) break;
    for (const m of more.slice(0, 4)) {
      attempts.push(`expand ${m.id}: ${await clickRow(m.domId)}`);
      tree = await openTree();
      row = findRow(tree, rowId);
      if (row) break;
    }
  }
  if (!row && altRowId) { row = findRow(tree, altRowId); if (row) attempts.push(`fell back to sibling row ${altRowId}`); }
  if (row) {
    const clicked = await clickRow(row.domId);
    if (clicked === "ok") return { via: "tree", row, clicked, attempts, tree };
    return { via: "tree-failed", row, clicked, attempts, tree, ...(await pick3d()) };
  }
  return { via: "no-tree-row", row: null, clicked: `no row '${rowId}' among ${tree.rows.length} (${tree.rows.slice(0, 10).map((r) => r.id).join(", ")})`, attempts, tree, ...(await pick3d()) };
};
const pick3d = async () => {
  const host = page.locator(`[data-surface-id="${surface3d}"]`).first();
  if (!(await host.count())) return { fallback3d: `${surface3d} absent` };
  const box = await host.boundingBox();
  if (!box) return { fallback3d: "world host has no bounding box" };
  for (const [dx, dy] of [[0, 0], [0, -0.15], [0.15, 0], [-0.15, 0]]) {
    await page.mouse.click(box.x + box.width * (0.5 + dx), box.y + box.height * (0.5 + dy));
    await page.waitForTimeout(1800);
    const sel = await page.evaluate((id) => document.querySelector(`[data-surface-id="${id}"]`)?.getAttribute("data-selection-json") ?? null, surface3d);
    if (sel && /"selectedIds":\s*\[\s*"/.test(sel)) return { fallback3d: `picked via 3d: ${sel.slice(0, 200)}` };
  }
  return { fallback3d: "3d clicks selected nothing" };
};
const inspectorInputs = (ns) => page.evaluate((ns) => [...document.querySelectorAll(`[id^="panel:${ns}/"]`)]
  .filter((el) => ["INPUT", "SELECT", "TEXTAREA"].includes(el.tagName) || el.getAttribute("role") === "spinbutton" || el.getAttribute("role") === "combobox" || el.isContentEditable)
  .map((el) => ({ id: el.id, tag: el.tagName, type: el.getAttribute("type"), role: el.getAttribute("role"), inputmode: el.getAttribute("inputmode"), value: "value" in el ? String(el.value).slice(0, 40) : el.getAttribute("aria-valuenow") })), ns);
const historyLines = (from) => lines.slice(from).filter((l) => l.includes("history patch applied")).map((l) => l.slice(0, 500));
const labelledHistory = (hist) => hist.filter((l) => /"labels":\s*\[\s*"[^"]+"/.test(l));
/** ✍️ React's controlled inputs ignore a raw `el.value = x`; go through the prototype setter so React's own
 * value tracker sees the change, then fire bubbling input + change. Used only when playwright's fill did not commit. */
const nativeSet = (domId, value) => page.evaluate(([domId, value]) => {
  const el = document.getElementById(domId);
  if (!el) return "absent";
  const proto = el instanceof HTMLTextAreaElement ? HTMLTextAreaElement.prototype : el instanceof HTMLSelectElement ? HTMLSelectElement.prototype : HTMLInputElement.prototype;
  const setter = Object.getOwnPropertyDescriptor(proto, "value")?.set;
  if (!setter) return "no-setter";
  el.focus();
  setter.call(el, String(value));
  el.dispatchEvent(new Event("input", { bubbles: true }));
  el.dispatchEvent(new Event("change", { bubbles: true }));
  return "dispatched";
}, [domId, value]);

try {
  await page.goto(url, { waitUntil: "domcontentloaded" });
  let s = null;
  for (let i = 0; i < bootSeconds; i++) { await page.waitForTimeout(1000); s = await shell(); if (s.error) break; if (s.ready && s.surfaces.length && i > 10) break; }
  note("boot", s);
  await shot("1-boot");
  assert("boot.ready", Boolean(s?.ready) && !s?.error, `data-semio-os-ready=${s?.ready ?? "null"} data-semio-os-error=${s?.error ?? "null"}; surfaces=${JSON.stringify(s?.surfaces ?? [])}`);

  // ── A1 the artifact tree panel lists every family ───────────────────────
  let tree = await openTree();
  note("treePanel", { how: tree.how, ns: tree.ns, rowCount: tree.rows.length, rows: tree.rows.slice(0, 80).map((r) => ({ id: r.id, text: r.text, selected: r.selected })) });
  await shot("2-tree");
  if (!tree.ns) {
    assert("tree.panel", false, `no tree rows — ${tree.how}`);
    for (const f of FAMILIES) assert(`tree.family.${f.key}`, false, "skipped — no tree panel");
  } else {
    assert("tree.panel", true, `${tree.rows.length} rows under panel:${tree.ns}/ (${tree.how})`);
    for (const f of FAMILIES) {
      const hits = tree.rows.filter((r) => f.re.test(r.text) || f.re.test(r.id));
      const labelled = hits.filter((r) => /[A-Za-zÄÖÜäöü]{3,}/.test(r.text.replace(/^[0-9\s:·-]+/, "")));
      assert(`tree.family.${f.key}`, hits.length > 0 && labelled.length > 0,
        `${hits.length} row(s) match /${f.re.source}/, ${labelled.length} with a human label; e.g. ${JSON.stringify(labelled.slice(0, 3).map((r) => `${r.id}=${r.text}`))}`);
    }
  }


  // 🧭️ Order matters and is NOT cosmetic: the tree pages a node's children, and selecting surface `40` replaces its two
  // window rows (`50`, `51`) with a single non-expanding `energy-model-artifact.zones.40.windows.more` marker —
  // clicking that marker returns "ok" but the tree stays at 44 rows and `50` never comes back (measured in
  // 🗑️generated/energy-panels-w1f). So the FENESTRATION is selected first, while the tree is still in its pristine
  // 48-row boot state, and the surface second.
  // ── A2 fenestration selection → a fenestration form ─────────────────────
  tree = await openTree();
  const fenPick = tree.ns ? await selectEntity(fenestrationRowId, tree, altFenestrationRowId) : { via: "no-tree", clicked: "skipped" };
  let inspector = await openInspector();
  const fenForm = inspector.rows.filter((r) => /\.fenestration\./i.test(r.id));
  const inputs = inspector.ns ? await inspectorInputs(inspector.ns) : [];
  note("fenestrationInspect", { pick: { ...fenPick, tree: undefined }, how: inspector.how, ns: inspector.ns, rowCount: inspector.rows.length, fenFormRows: fenForm.slice(0, 25).map((r) => ({ id: r.id, text: r.text, value: r.value })), inputs });
  await shot("3-inspector-fenestration");
  if (!inspector.ns) assert("inspector.fenestration", false, `the inspection panel published no rows — ${inspector.how}`);
  else assert("inspector.fenestration", fenForm.length > 0,
    `selected ${fenestrationRowId} via ${fenPick.via} (${fenPick.clicked}); ${fenForm.length} '.fenestration.' form field(s)${fenForm.length ? `: ${JSON.stringify(fenForm.slice(0, 5).map((r) => r.id.split("/").pop()))}` : ` (inspector row ids: ${JSON.stringify(inspector.rows.slice(0, 8).map((r) => r.id.split("/").pop()))})`}`);


  // ── A3 edit the u-value on the still-selected fenestration ──────────────
  const field = inputs.find((c) => fieldPattern.test(c.id ?? ""));
  const historyFrom = lines.length;
  const edit = { field: field ?? null, before: null, after: null, strategy: "skipped", history: [] };
  if (field) {
    const input = page.locator(`[id="${field.id}"]`).first();
    edit.before = await input.inputValue().catch(() => null);
    // (a) the ordinary path: playwright fill dispatches input+change, then Enter/blur commit a stepper
    edit.strategy = await input.fill(String(newValue)).then(() => "fill").catch((e) => `fill-failed:${String(e).slice(0, 120)}`);
    await input.press("Enter").catch(() => {});
    await input.evaluate((el) => el.blur?.()).catch(() => {});
    for (let i = 0; i < 12; i++) { await page.waitForTimeout(500); if (labelledHistory(historyLines(historyFrom)).length) break; }
    // (b) fallback: React's native value setter + bubbling events, in case the control ignores a plain fill
    if (!labelledHistory(historyLines(historyFrom)).length) {
      const dispatched = await nativeSet(field.id, newValue);
      await input.press("Enter").catch(() => {});
      await input.evaluate((el) => el.blur?.()).catch(() => {});
      edit.strategy = `${edit.strategy}+nativeSetter(${dispatched})`;
      for (let i = 0; i < 12; i++) { await page.waitForTimeout(500); if (labelledHistory(historyLines(historyFrom)).length) break; }
    }
    edit.after = await input.inputValue().catch(() => null);
  }
  edit.history = historyLines(historyFrom);
  // 🚨️ The interesting failure mode is not "nothing happened": the inspector DOES dispatch, and the shell REFUSES.
  // Surface the refusal verbatim, otherwise a real "window kind X does not own action Y" reads as a silent no-op.
  edit.refusals = lines.slice(historyFrom).filter((l) => /does not own action|refused: dispatch-failed|\[DEBUG\] action failed/i.test(l)).slice(0, 4).map((l) => l.slice(0, 320));
  edit.dispatched = lines.slice(historyFrom).filter((l) => /performInvocation.*actionId/.test(l)).slice(0, 6).map((l) => l.slice(0, 200));
  const inspectorPostEdit = inspector.ns ? (await panelRows())[inspector.ns] ?? [] : [];
  const inputsPostEdit = inspector.ns ? await inspectorInputs(inspector.ns) : [];
  note("fenestrationEdit", { ...edit, inputsPostEdit, inspectorPostEdit: inspectorPostEdit.filter((r) => /fenestration/i.test(r.id)).slice(0, 25).map((r) => ({ id: r.id, text: r.text, value: r.value })) });
  await shot("4-fenestration-edit");
  if (!field) assert("inspector.fenestrationEdit", false, `no inspector control id matched /${fieldPattern.source}/ — controls under panel:${inspector.ns ?? "?"}/: ${JSON.stringify(inputs.map((c) => c.id.split("/").pop())).slice(0, 400)}`);
  else {
    const readback = String(edit.after ?? "");
    const reflected = readback.startsWith(String(newValue))
      || inputsPostEdit.some((c) => c.id === field.id && String(c.value).startsWith(String(newValue)))
      || inspectorPostEdit.some((r) => r.text.includes(newValue));
    assert("inspector.fenestrationEdit", reflected, `${field.id.split("/").pop()} ${edit.before} → ${edit.after} via ${edit.strategy}; reflected in the inspector=${reflected}${edit.refusals.length ? `; THE SHELL REFUSED THE DISPATCH: ${edit.refusals[0]}` : ""}`);
  }
  const labelled = labelledHistory(edit.history);
  assert("history.entry", labelled.length > 0, `${edit.history.length} '[DEBUG] history patch applied' line(s), ${labelled.length} with a non-empty label${labelled.length ? `; first: ${labelled[0].slice(0, 220)}` : edit.refusals?.length ? ` — the dispatch was REFUSED, not silently dropped: ${edit.refusals[0]}` : " — the edit never reached the retained mutation route (a silent no-op reducer looks exactly like this)"}`);

  // ── A4 surface selection → the inspector shows a surface form ───────────
  tree = await openTree();
  const surfacePick = tree.ns ? await selectEntity(surfaceRowId, tree) : { via: "no-tree", clicked: "skipped" };
  inspector = await openInspector();
  const surfaceForm = inspector.rows.filter((r) => /\.surface\./i.test(r.id));
  note("surfaceInspect", { pick: { ...surfacePick, tree: undefined }, how: inspector.how, ns: inspector.ns, rowCount: inspector.rows.length, surfaceFormRows: surfaceForm.slice(0, 25).map((r) => ({ id: r.id, text: r.text, value: r.value })), rows: inspector.rows.slice(0, 40).map((r) => r.id) });
  await shot("5-inspector-surface");
  if (!inspector.ns) assert("inspector.surface", false, `the inspection panel published no rows — ${inspector.how}`);
  else assert("inspector.surface", surfaceForm.length > 0,
    `selected ${surfaceRowId} via ${surfacePick.via} (${surfacePick.clicked}); inspector has ${inspector.rows.length} rows, ${surfaceForm.length} of them a '.surface.' form field${surfaceForm.length ? `: ${JSON.stringify(surfaceForm.slice(0, 4).map((r) => r.id.split("/").pop()))}` : ` (ids seen: ${JSON.stringify(inspector.rows.slice(0, 6).map((r) => r.id.split("/").pop()))})`}`);

  // 🧹️ `contributions push refused empty pack` is pre-existing baseline noise on every boot of this build.
  const BENIGN = /contributions push refused empty pack/;
  const faults = lines.filter((l) => /pageerror|trapped|panicked|unreachable|dropped action|Unknown action|fixed-capacity|surface-render|window-context-required|not a declared|does not own action|refused: dispatch-failed|\[DEBUG\] action failed/i.test(l) && !BENIGN.test(l)).map((l) => l.slice(0, 300));
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
