/** 🧪 Core checks of the v3 matrix: shell, windows, language, roles, examples, utilities, selection, delete/history, import/export, clipboard. */
import { writeFileSync, statSync } from "node:fs";
import { esc, sleep } from "./lib.mjs";

const SKIP = (why) => ({ pass: null, observed: `SKIP: ${why}` });
const sum = o => Object.values(o ?? {}).reduce((a, b) => a + b, 0);
export const mk = (id, area, action, expected, inv, run, o = {}) => ({ id, area, action, expected, inv, run, ...o });

export function coreChecks(a) {
  const d = a.dim, K = [];
  const add = (...args) => K.push(mk(...args));
  const modelHashChange = async (x, fn, ms = 15000) => { const h = await x.hash(); await fn(); return x.changed(h, null, ms); };

  add("shell.editor-role", "shell", "Boot: Role group", "Editor role pressed, editing UI (Actions, Utilities) shown", { 2: [1], 3: [1] }, async x => {
    const pressed = await x.isPressed("Editor");
    const actions = await x.modelPanel().getByRole("button", { name: /^Actions/ }).count();
    const utils = await x.modelPanel().getByRole("button", { name: /^Utilities/ }).count();
    return { pass: pressed === true && actions > 0 && utils > 0, observed: `Editor pressed=${pressed} Actions=${actions} Utilities=${utils}` };
  });
  add("shell.model-window", "shell", "Window Model at boot", "Model tab, tabpanel and view drawn (more than 4 distinct colours)", { 2: [2], 3: [2] }, async x => {
    const tab = await x.page.getByRole("tab", { name: "Model" }).count(), view = await x.page.locator(`[aria-label="${a.viewLabel}"]`).count();
    const colors = await x.drawn();
    return { pass: tab > 0 && view > 0 && colors > 4, observed: `tab=${tab} view=${view} distinctColors=${colors}` };
  });
  add("shell.results-window", "shell", "Window Results at boot", "Results tab, tabpanel and view drawn, caption shown", { 2: [3], 3: [3] }, async x => {
    const tab = await x.page.getByRole("tab", { name: "Results" }).count();
    const colors = await x.drawn(a.resultsClip), cap = await x.caption();
    return { pass: tab > 0 && colors > 4 && !!cap, observed: `tab=${tab} distinctColors=${colors} caption=${JSON.stringify(cap)}` };
  });
  add("shell.default-layout", "shell", "Boot layout", "Model tab left of Results tab", { 2: [], 3: [4] }, async x => {
    const t = await x.tabs(), xm = Number((t.find(s => /^Model@/.test(s)) ?? "@9999").split("@")[1]), xr = Number((t.find(s => /^Results@/.test(s)) ?? "@-1").split("@")[1]);
    return { pass: xm < xr, observed: `tabs=${t.join(",")}` };
  });
  add("shell.artifact-panel", "shell", "Bottom bar Artifact", "Workbench tree with group headers and counts", { 2: [4, 85], 3: [5, 124] }, async x => {
    await x.panel("Artifact", true); await x.page.waitForTimeout(1500);
    const found = []; for (const g of a.groups) found.push(`${g}=${await x.groupCount(g)}`);
    return { pass: found.every(s => !/=null$/.test(s)), observed: found.join(" ") };
  });
  add("shell.artifact-chrome", "shell", "Open Artifact, hit-test window chrome", "Model tab Focus and Close buttons stay reachable (not covered by the panel)", { 2: [5], 3: [] }, async x => {
    await x.panel("Artifact", true);
    const res = [];
    for (const n of ["Focus", "Close"]) {
      const b = x.winButtons("Model").getByRole("button", { name: n, exact: true }).first();
      const ok = (await b.count()) ? await b.evaluate(el => { const r = el.getBoundingClientRect(); const t = document.elementFromPoint(r.x + r.width / 2, r.y + r.height / 2); return !!t && (el.contains(t) || t.contains(el)); }) : false;
      res.push(`${n}=${ok}`);
    }
    return { pass: res.every(s => /true/.test(s)), observed: res.join(" ") };
  });
  add("shell.inspection-panel", "shell", "Tree row select, Inspection panel", "Entity form shown for the selected node", { 2: [6], 3: [6] }, async x => {
    const r = await x.selectRow(a.rows.node, { group: "Nodes" });
    const t = await x.insp();
    return { pass: r.ok && !r.sel.summary, observed: `select ok=${r.ok} inspection="${t.slice(0, 140)}"` };
  });
  add("shell.summary", "shell", "Nothing selected", "Summary lists schema and entity counts", { 2: [84], 3: [55] }, async x => {
    const c = await x.counts();
    return { pass: !!c && Object.keys(c).length >= 5, observed: JSON.stringify(c) };
  });
  add("shell.results-panel", "shell", "Bottom bar Results", "Display, Playback and Analysis controls present", { 2: [7], 3: [7] }, async x => {
    await x.panel("Results", true); await x.page.waitForTimeout(1200);
    const need = ["Source", "Mode", "Mode Index", "Step", "Step +", "Speed", "Loop", "Waveform", "Modal Count", "Buckling Count", "Deformation Scale"];
    const t = await x.text();
    const present = []; for (const n of need) { if (new RegExp(esc(n)).test(t) || await x.page.getByRole("button", { name: n }).count() || await x.page.getByRole("combobox", { name: n }).count() || await x.page.getByRole("spinbutton", { name: n }).count()) present.push(n); }
    const missing = need.filter(n => !present.includes(n));
    return { pass: missing.length === 0 && /Play|Pause/.test(t), observed: `missing=[${missing}] hasPlay=${/Play|Pause/.test(t)}` };
  });
  add("shell.history-panel", "shell", "Bottom bar History", "Actions (Undo, Redo, Commit Checkpoint, Create Alternative) and Commands lists", { 2: [8], 3: [48, 54] }, async x => {
    await x.panel("History", true); await x.page.waitForTimeout(1200);
    const t = await x.page.getByRole("tree").filter({ hasText: /Commit Checkpoint/ }).last().innerText().catch(() => "");
    const need = ["Undo", "Redo", "Commit Checkpoint", "Create Alternative", "Commands"];
    const miss = need.filter(n => !new RegExp(esc(n), "i").test(t));
    return { pass: miss.length === 0, observed: `missing=[${miss}] text="${t.replace(/\n/g, "|").slice(0, 200)}"` };
  });
  add("shell.settings-panel", "shell", "Top Settings", "Driver, General, App sections", { 2: [9], 3: [164] }, async x => {
    await x.panel("Settings", true); await x.page.waitForTimeout(1500);
    const t = await x.text(); const miss = ["Driver", "General", "App"].filter(n => !new RegExp(`\\b${n}\\b`).test(t));
    await x.panel("Settings", false);
    return { pass: miss.length === 0, observed: `missing=[${miss}]` };
  });
  add("shell.command-menu", "shell", "Top Command", "Menu with Layout, App, Appearance, Language tabs", { 2: [14], 3: [168] }, async x => {
    await x.panel("Command", true); await x.page.waitForTimeout(1500);
    const t = await x.text(); const miss = ["Layout", "App", "Appearance", "Language"].filter(n => !new RegExp(`\\b${n}\\b`).test(t));
    await x.panel("Command", false);
    return { pass: miss.length === 0, observed: `missing=[${miss}]` };
  });
  add("shell.command-locale", "shell", "Command > Language > Set Locale", "Locale picker input has non-zero size and is usable", { 2: [], 3: [169] }, async x => {
    await x.panel("Command", true); await x.page.waitForTimeout(1200);
    await x.page.getByRole("button", { name: "Language", exact: true }).first().click({ timeout: 5000 }).catch(() => {});
    await x.page.getByRole("button", { name: /^Set Locale/ }).first().click({ timeout: 5000 }).catch(() => {}); await x.page.waitForTimeout(1200);
    const box = await x.page.locator('[id*="setLocale.arg.locale.input"]').first().boundingBox().catch(() => null);
    await x.panel("Command", false);
    return { pass: !!box && box.width > 4 && box.height > 4, observed: `input box=${JSON.stringify(box)}` };
  }, { skipIf: d !== 3 });
  add("shell.fullscreen", "shell", "Top Fullscreen button", "document.fullscreenElement set, then cleared by second click", { 2: [13], 3: [163] }, async x => {
    await x.btn(/^Fullscreen$/, false).first().click(); await x.page.waitForTimeout(2500);
    const on = await x.page.evaluate(() => !!document.fullscreenElement);
    await x.btn(/^(Exit )?Fullscreen$/, false).first().click(); await x.page.waitForTimeout(2500);
    const off = await x.page.evaluate(() => !document.fullscreenElement);
    return { pass: on && off, observed: `fullscreen on=${on} off=${off}` };
  });

  add("win.focus", "windows", "Model window Focus", "Other window hidden (Results tab removed)", { 2: [11], 3: [41, 162] }, async x => {
    await x.panel("Artifact", false); await x.panel("Inspection", false); await x.panel("History", false); await x.panel("Results", false);
    await x.winButtons("Model").getByRole("button", { name: "Focus", exact: true }).click({ timeout: 8000 }); await x.page.waitForTimeout(2500);
    const left = await x.page.getByRole("tab", { name: "Results" }).count();
    return { pass: left === 0 || !(await x.page.getByRole("tab", { name: "Results" }).first().isVisible()), observed: `Results tabs visible=${left}`, ok: true };
  }, { fresh: true });
  add("win.focus-restore", "windows", "Focus again", "Both windows back", { 2: [11], 3: [41] }, async x => {
    await x.winButtons("Model").getByRole("button", { name: /Focus|Restore|Unfocus/ }).first().click({ timeout: 8000 }); await x.page.waitForTimeout(2500);
    const back = await x.page.getByRole("tab", { name: "Results" }).count();
    return { pass: back > 0, observed: `Results tabs=${back}; tabs=${(await x.tabs()).join(",")}` };
  });
  add("win.close", "windows", "Results window Close", "Results window removed", { 2: [10], 3: [160] }, async x => {
    const before = await x.tabs();
    await x.winButtons("Results").getByRole("button", { name: "Close", exact: true }).click({ timeout: 8000 }); await x.page.waitForTimeout(2500);
    const after = await x.tabs();
    return { pass: after.length < before.length, observed: `${before.join(",")} -> ${after.join(",")}` };
  });
  add("win.reset-panels", "windows", "Command > Layout > Reset panels", "Closed Results window restored", { 2: [12], 3: [] }, async x => {
    await x.panel("Command", true); await x.page.waitForTimeout(1200);
    await x.page.getByRole("button", { name: "Reset panels", exact: true }).first().click({ timeout: 6000 }); await x.page.waitForTimeout(2500);
    let back = await x.page.getByRole("tab", { name: "Results" }).count();
    if (!back) { await x.btn(/Reset panels/, false).first().click({ timeout: 4000 }).catch(() => {}); await x.page.waitForTimeout(2500); back = await x.page.getByRole("tab", { name: "Results" }).count(); }
    return { pass: back > 0, observed: `Results tabs=${back}; tabs=${(await x.tabs()).join(",")}` };
  });
  add("win.reopen-display", "windows", "Display panel: drag window entry back", "Closed window returns", { 2: [], 3: [161] }, async x => {
    await x.panel("Settings", false);
    if ((await x.page.getByRole("tab", { name: "Results" }).count()) > 0) await x.winButtons("Results").getByRole("button", { name: "Close", exact: true }).click().catch(() => {});
    await x.page.waitForTimeout(1500);
    await x.panel("Display", true); await x.page.waitForTimeout(1500);
    const entry = x.page.getByRole("button", { name: /^Results Reorder$/ }).first();
    if (!(await entry.count())) return { pass: false, observed: "no Results section in the Display Windows tree" };
    const leafOf = () => entry.locator("xpath=ancestor::*[@role='tree'][1]").getByRole("treeitem").filter({ hasText: /^Results$/ }).first();
    if (!(await leafOf().count())) { await entry.click(); await x.page.waitForTimeout(1200); }
    const b = await leafOf().boundingBox();
    if (!b) return { pass: false, observed: "Results section has no draggable window entry" };
    await x.rawDrag(b.x + b.width / 2, b.y + b.height / 2, 1500, 500);
    await x.until(async () => (await x.page.getByRole("tab", { name: "Results" }).count()) > 0, 8000, 400);
    const back = await x.page.getByRole("tab", { name: "Results" }).count();
    return { pass: back > 0, observed: `Results tabs=${back}; tabs=${(await x.tabs()).join(",")}; single first drag` };
  });

  add("lang.de", "language", "Settings > Language > Deutsch", "All UI labels German, English main labels gone", { 2: [15], 3: [165] }, async x => {
    await x.setLanguage("Deutsch");
    const t = await x.text(); const german = [...new Set((t.match(new RegExp(a.german.source, "gi")) ?? []).map(s => s.toLowerCase()))];
    const english = [...new Set(t.match(/\b(Inspection|Utilities|History|Results|Command)\b/g) ?? [])];
    return { pass: german.length >= 2 && english.length <= 1, observed: `german=${german} englishLeft=${english}` };
  }, { fresh: true, after: async x => x.setLanguage("English").catch(() => {}) });
  add("lang.en", "language", "Settings > Language > English", "English labels return", { 2: [15], 3: [165] }, async x => {
    await x.setLanguage("English");
    const ok = (await x.btn("Inspection").count()) > 0 || /Inspection/.test(await x.text());
    return { pass: ok, observed: `Inspection label back=${ok}` };
  });
  add("theme.dark", "settings", "Settings > Appearance > Dark, then System", "Page becomes dark (dark pixel share rises) and is restored", { 2: [16], 3: [166] }, async x => {
    const full = { x: 0, y: 0, width: 1600, height: 1000 };
    const share = async () => { const t = await x.analyze(full, "r+g+b<330"); return t.n / (t.width * t.height); };
    await x.panel("Settings", true); await x.page.waitForTimeout(1500); const s0 = await share();
    const cb = x.page.getByRole("combobox", { name: /^(Appearance|Erscheinungsbild)$/ }).first(); if (!(await cb.count())) return { pass: false, observed: "Appearance combobox missing" };
    const r1 = await x.chooseOption(cb, /^Dark$/i); await x.page.waitForTimeout(2500); const s1 = await share();
    const r2 = await x.chooseOption(x.page.getByRole("combobox", { name: /^(Appearance|Erscheinungsbild)$/ }).first(), /^System$/i); await x.page.waitForTimeout(2500); const s2 = await share();
    return { pass: r1.ok && r2.ok && s1 > s0 + 0.3 && Math.abs(s2 - s0) < 0.15, observed: `options=[${r1.options}] dark share ${s0.toFixed(2)} -> ${s1.toFixed(2)} -> restored ${s2.toFixed(2)}` };
  });

  add("role.viewer-editor", "roles", "Role Viewer, then Editor", "Viewer layout (no Inspection, Results, enabled Utilities or editing action; the palette keeps only the read-only Copy and document transfer verbs), then Editor restores editing UI", { 2: [20, "V2D-01", "V2D-02", "V2D-03", "V2D-07"], 3: [158, 159, "V3D-01", "V3D-02", "V3D-03"] }, async x => {
    await x.panel("Inspection", false); await x.panel("History", false);
    await x.btn("Viewer").first().click(); await x.page.waitForTimeout(4000);
    const v = { pressed: await x.isPressed("Viewer"), results: await x.page.getByRole("tab", { name: "Results" }).count(), insp: await x.page.getByRole("button", { name: "Inspection", exact: true }).count() };
    const utilBtn = x.page.getByRole("button", { name: /^Utilities/ }); v.utilsEnabled = (await utilBtn.count()) > 0 && await utilBtn.first().isEnabled();
    const actionsBtn = x.page.getByRole("button", { name: /^Actions/ }); let items = [];
    if (await actionsBtn.count()) {
      await actionsBtn.first().click(); await x.page.waitForTimeout(1500);
      items = (await x.modelPanel().getByRole("treeitem").allTextContents()).map(t => t.trim());
      await actionsBtn.first().click(); await x.page.waitForTimeout(500);
    }
    const READ_ONLY = ["Copy", "Export Document", "Import Document…", "Dokument exportieren", "Dokument importieren…", "Kopieren"];
    v.editing = items.filter(t => !READ_ONLY.includes(t));
    const edBtn = x.btn("Editor").first(); const disabled = await edBtn.isDisabled().catch(() => true);
    if (!disabled) { await edBtn.click(); await x.until(async () => (await x.isPressed("Editor")) === true && (await x.page.getByRole("button", { name: /^Actions/ }).count()) > 0, 60000, 500); }
    const e = { pressed: await x.isPressed("Editor"), actions: await x.page.getByRole("button", { name: /^Actions/ }).count() };
    return { pass: v.pressed === true && v.editing.length === 0 && !v.utilsEnabled && v.results === 0 && v.insp === 0 && !disabled && e.pressed === true && e.actions > 0, observed: `viewer=${JSON.stringify(v)} palette=[${items}] editorDisabled=${disabled} editor=${JSON.stringify(e)}` };
  }, { fresh: true });

  for (const ex of a.examples) {
    add(`example.${ex.toLowerCase().replace(/ /g, "-")}`, "examples", `Example combobox > ${ex}`, `${ex} loaded: combobox shows it, entity counts change, Results re-solve`, d === 2 ? { 2: [18, 109, 138].slice(0, 2) } : { 3: ex === "Demo" ? [114, 154, 155] : ex === "House" ? [114, 156] : [115, 157] }, async x => {
      const cb = x.page.getByRole("combobox", { name: "Example" }).first();
      const c0 = await x.counts();
      const r = await x.chooseOption(cb, new RegExp(`^${esc(ex)}$`));
      if (!r.ok) return { pass: false, observed: `option ${ex} missing in [${r.options}]` };
      await x.page.waitForTimeout(5000);
      const shown = ((await cb.textContent()) ?? "").trim();
      const c1 = await x.counts(); const solved = await x.solved(ex === "House" ? 400000 : 200000);
      return { pass: shown.includes(ex) && !!c1 && sum(c1) > 0 && solved >= 0, observed: `combobox="${shown}" options=[${r.options}] counts ${JSON.stringify(c0)} -> ${JSON.stringify(c1)} solved=${solved} ms caption=${JSON.stringify(await x.caption())}` };
    }, { timeout: ex === "House" ? 600000 : 360000, slow: ex !== "Demo", fresh: ex === "Demo" });
  }
  add("example.no-example-then-demo", "examples", "Example > No example, then Demo", "Model cleared (0 nodes), then Demo restored and Results recover", { 2: [19], 3: [154] }, async x => {
    const cb = x.page.getByRole("combobox", { name: "Example" }).first();
    const demo = await x.counts();
    const r0 = await x.chooseOption(cb, /^No example$/i);
    const clearedMs = await x.until(async () => { const c = await x.counts(); return !!c && (c.Nodes ?? 0) === 0; }, 90000, 1000);
    const c0 = await x.counts(); const cleared = clearedMs >= 0;
    const r1 = await x.chooseOption(cb, /^Demo$/);
    const restoredMs = await x.until(async () => { const c = await x.counts(); return !!c && JSON.stringify(c) === JSON.stringify(demo); }, 90000, 1000);
    const c1 = await x.counts(); const solved = await x.solved(200000);
    return { pass: r0.ok && r1.ok && cleared && restoredMs >= 0 && solved >= 0, observed: `Demo=${JSON.stringify(demo)} cleared=${cleared} in ${clearedMs} ms (${JSON.stringify(c0)}) restored=${restoredMs >= 0} in ${restoredMs} ms (${JSON.stringify(c1)}) solved=${solved} caption=${JSON.stringify(await x.caption())}` };
  }, { timeout: 420000, fresh: true });
  add("example.listing", "examples", "Example combobox open", `Lists No example and ${a.examples.join(", ")}`, { 2: [18], 3: [154] }, async x => {
    const o = await x.options(x.page.getByRole("combobox", { name: "Example" }).first()); await x.page.keyboard.press("Escape");
    const miss = a.examples.filter(e => !o.includes(e));
    return { pass: miss.length === 0 && o.some(s => /no example/i.test(s)), observed: `options=[${o}] missing=[${miss}]` };
  });

  if (d === 2) {
    add("util.direct-select-armed", "utilities", "Open Utilities at boot", "Direct Select shown pressed (implicit default)", { 2: [21], 3: [] }, async x => {
      await x.utilsOpen(); const p = await x.pressedTools();
      return { pass: p.includes("Direct Select"), observed: `pressed=[${p}]` };
    }, { fresh: true });
    add("util.direct-click", "utilities", "Direct Select: click node", "Node selected", { 2: [22, 32], 3: [] }, async x => {
      await x.util("Direct Select").catch(() => {}); await x.toSummary();
      const r = await x.pickAt(a.nodeA); const s = await x.selection();
      return { pass: r.ok && s.n === 1, observed: `pick=${JSON.stringify(r)} selection=${JSON.stringify(s)}` };
    });
  }

  add("util.hover-highlight", "selection", "Hover 1.5 s over entity", "Model image changes (highlight) vs hover over empty canvas", { 2: [40], 3: [25] }, async x => {
    await x.toSummary(); const tree = { x: 0, y: 30, width: 300, height: 900 }; const seen = [];
    for (const spot of [a.nodeA.at, a.nodeB.at, a.nodeC.at]) {
      await x.hover(...a.empty, 1500); const h0 = await x.hash(), t0 = await x.hash(tree);
      await x.hover(...spot, 2000); const h1 = await x.hash(), t1 = await x.hash(tree);
      seen.push(`${spot}: viewport ${h0 !== h1 ? "changed" : "same"}, tree ${t0 !== t1 ? "changed" : "same"}`);
      if (h0 !== h1 || t0 !== t1) return { pass: true, observed: seen.join("; ") };
    }
    return { pass: false, observed: seen.join("; ") };
  });
  add("sel.click-replace", "selection", "Click entity A then entity B", "Selection replaced: only B selected", { 2: [32], 3: [14] }, async x => {
    await x.toSummary(); const A = await x.pickAt(a.nodeA); const idA = (await x.selIds())[0]; const B = await x.pickAt(a.nodeB, { differs: true }); const s = await x.selection(); const ids = await x.selIds();
    return { pass: A.ok && B.ok && s.n === 1 && ids.length === 1 && ids[0] !== idA, observed: `A=${A.ok}(${idA}) B=${B.ok} selected=[${ids}] selection=${JSON.stringify(s)}` };
  });
  add("sel.shift-additive", "selection", "Pick A, Shift+click B", "A kept and B added (selection lists both)", { 2: [33], 3: [29] }, async x => {
    await x.toSummary(); await x.pickAt(a.nodeB); const idB = (await x.selIds())[0]; await x.toSummary();
    await x.pickAt(a.nodeA); const idA = (await x.selIds())[0]; await x.hoverClick(...a.nodeB.at, { mod: "Shift" });
    const ms = await x.until(async () => { const i = await x.selIds(); return i.length >= 2 && i.includes(idA); }, 10000, 300); const s = await x.selection(); const ids = await x.selIds();
    return { pass: ms >= 0, observed: `A=${idA} B=${idB} selected=[${ids}] n=${s.n} (${ms} ms)` };
  });
  add("sel.ctrl-subtractive", "selection", "Ctrl+click entity B added before", "B (and what it added) removed from the selection, A kept", { 2: [34], 3: [29] }, async x => {
    const before = await x.selIds(); const idA = before[0];
    await x.hoverClick(...a.nodeB.at, { mod: "Control" });
    const ms = await x.until(async () => { const i = await x.selIds(); return i.length < before.length && i.includes(idA); }, 10000, 300);
    return { pass: ms >= 0 && before.length >= 2, observed: `before=[${before}] after=[${await x.selIds()}] (${ms} ms); expected fewer entities, A kept` };
  });
  add("sel.shift-ctrl-invertive", "selection", "Shift+Ctrl+click B twice", "Toggles: B added, then removed", { 2: [35], 3: [29] }, async x => {
    await x.toSummary(); await x.pickAt(a.nodeA); const base = await x.selIds();
    await x.hoverClick(...a.nodeB.at, { mod: ["Shift", "Control"] }); const m1 = await x.until(async () => (await x.selIds()).length > base.length, 10000, 300); const s1 = await x.selIds();
    await x.hoverClick(...a.nodeB.at, { mod: ["Shift", "Control"] }); const m2 = await x.until(async () => (await x.selIds()).length < s1.length, 10000, 300); const s2 = await x.selIds();
    return { pass: m1 >= 0 && m2 >= 0, observed: `base=[${base}] first=[${s1}] second=[${s2}]` };
  });
  add("sel.empty-click", "selection", "Click empty canvas", "Selection cleared, Summary shown", { 2: [41], 3: [30] }, async x => {
    await x.pickAt(a.nodeA); const ok = await x.toSummary();
    return { pass: ok, observed: `summary=${ok}` };
  });
  if (d === 2) {
    for (const [mode, inv, steps] of [["Selective", 36], ["Additive", 37], ["Subtractive", 38], ["Invertive", 39]]) add(`sel.mode-${mode.toLowerCase()}`, "selection", `Utilities Mode radio ${mode}, then click`, mode === "Selective" ? "Click replaces selection" : mode === "Additive" ? "Click adds to selection (2)" : mode === "Subtractive" ? "Click on selected entity removes it (1)" : "Click on selected entity toggles it off, unselected on", { 2: [inv], 3: [] }, async x => {
      await x.util("Direct Select").catch(() => {}); await x.toSummary(); await x.radio(mode === "Subtractive" ? "Additive" : mode);
      const count = async n => { await x.until(async () => (await x.selection()).n === n, 25000, 400); return x.selection(); };
      await x.pickAt(a.nodeA);
      if (mode === "Selective") { await x.pickAt(a.nodeB, { differs: true }); const s = await x.selection(); return { pass: s.n === 1, observed: `selection=${JSON.stringify(s)}` }; }
      await x.hoverClick(...a.nodeB.at); const s = await count(2);
      if (mode === "Additive") return { pass: s.n === 2, observed: `selection=${JSON.stringify(s)}` };
      if (mode === "Subtractive") return { pass: s.n === 2 && (await x.radio("Subtractive"), await x.hoverClick(...a.nodeB.at), (await count(1)).n === 1), observed: `Additive A+B -> ${s.n}, Subtractive click B -> ${JSON.stringify(await x.selection())}` };
      await x.hoverClick(...a.nodeB.at); const s2 = await count(1);
      return { pass: s.n === 2 && s2.n === 1, observed: `toggle on=${s.n} off=${s2.n}` };
    }, { after: x => x.radio("Selective") });
  }
  add("sel.marquee", "selection", "Marquee Select drag", "Entities inside rectangle selected (N SELECTED header with list)", { 2: [23, 43], 3: [24, 28] }, async x => {
    await x.toSummary();
    if (d === 3) {
      await x.drag(...a.marquee, { n: 6, dt: 150, mod: "Shift" });
      const ms = await x.until(async () => (await x.selection()).n >= 2, 25000); const s = await x.selection();
      return { pass: ms >= 0 && /selected/i.test(await x.insp()), observed: `Shift+left drag (3D marquee is a host gesture, no Utilities entry): selection=${JSON.stringify(s)} ms=${ms}` };
    }
    await x.utilsOpen();
    if (!(await x.btn("Marquee Select").count())) return { pass: false, observed: "Marquee Select button missing" };
    await x.util("Marquee Select"); const armed = await x.pressedTools(); await x.drag(...a.marquee, { n: 6, dt: 150 });
    const ms = await x.until(async () => (await x.selection()).n >= 2, 25000); const s = await x.selection();
    return { pass: ms >= 0 && /selected/i.test(await x.insp()), observed: `armed=[${armed}] radios=[${await x.radioChecked()}] selection=${JSON.stringify(s)} ms=${ms}` };
  });
  add("sel.marquee-tiny", "selection", "Marquee drag of 2 px on entity", "Acts as a click: that entity alone selected", { 2: [42], 3: [24] }, async x => {
    await x.toSummary(); const [px, py] = a.nodeA.at;
    if (d === 3) {
      await x.hover(px, py, 1500); await x.page.keyboard.down("Shift");
      await x.page.mouse.down(); await x.page.mouse.move(px + 2, py + 1, { steps: 2 }); await x.page.mouse.up(); await x.page.keyboard.up("Shift");
      const ms = await x.until(async () => (await x.selIds()).length >= 1 && !(await x.selection()).summary, 15000, 400); const ids = await x.selIds();
      return { pass: ms >= 0 && ids.length === 1, observed: `Shift+2 px drag selected=[${ids}] (${ms} ms)` };
    }
    await x.utilsOpen(); if (!(await x.btn("Marquee Select").count())) return SKIP("no Marquee Select");
    await x.util("Marquee Select"); await x.hover(px, py, 1500);
    await x.page.mouse.down(); await x.page.mouse.move(px + 2, py + 1, { steps: 2 }); await x.page.mouse.up(); await x.until(async () => (await x.selection()).n >= 1, 25000, 400);
    const s = await x.selection();
    return { pass: s.n === 1, observed: `selection=${JSON.stringify(s)}` };
  });
  add("sel.lasso", "selection", "Lasso Select polygon", "Entities inside polygon selected", { 2: [24], 3: [] }, async x => {
    await x.toSummary(); if (!(await x.btn("Lasso Select").count())) { await x.utilsOpen(); }
    if (!(await x.btn("Lasso Select").count())) return SKIP("no Lasso Select button");
    await x.util("Lasso Select"); await x.lasso(a.lasso);
    const ms = await x.until(async () => (await x.selection()).n >= 2, 25000); const s = await x.selection();
    return { pass: ms >= 0, observed: `selection=${JSON.stringify(s)}` };
  });
  add("util.pan", "utilities", "Pan utility, slow drag +150,+100", "Model view moves with the drag", { 2: [25], 3: [] }, async x => {
    await x.toSummary(); await x.utilsOpen();
    if (!(await x.btn("Pan").count())) return SKIP("no Pan utility");
    await x.util("Pan"); const h0 = await x.hash();
    const [ex, ey] = a.empty; await x.drag(ex - 100, ey, ex + 50, ey + 100, { n: 10, dt: 500, hover: 500 });
    const ms = await x.changed(h0, null, 8000);
    return { pass: ms >= 0, observed: `view changed after ${ms} ms` };
  }, { after: async x => { x.needsReload = true; } });
  add("util.wheel-zoom", "utilities", "Mouse wheel over Model", "View zooms", { 2: [17], 3: [38] }, async x => {
    await x.toSummary(); const h0 = await x.hash(); await x.page.mouse.move(...a.wheel); await x.page.mouse.wheel(0, -500);
    const ms = await x.changed(h0, null, 8000);
    return { pass: ms >= 0, observed: `view changed after ${ms} ms` };
  }, { after: async x => { if (d === 2) x.needsReload = true; } });

  if (d === 3) {
    const cam = (id, inv, action, expected, button, mod) => add(`cam.${id}`, "camera", action, expected, { 2: [], 3: [inv] }, async x => {
      await x.toSummary(); const h0 = await x.hash(); const [px, py] = a.empty;
      await x.drag(px - 100, py + 250, px + 20, py + 330, { n: 10, dt: 120, button, mod });
      const ms = await x.changed(h0, null, 6000);
      return { pass: ms >= 0, observed: `view changed after ${ms} ms` };
    });
    cam("orbit-middle", 33, "Middle drag in Model", "Camera orbits", "middle");
    cam("orbit-left", 34, "Left drag on ground", "Camera orbits", "left");
    cam("pan-shift-middle", 36, "Shift + middle drag", "Camera pans", "middle", "Shift");
    cam("pan-right", 37, "Right drag", "Camera pans", "right");
    add("cam.orbit-left-transform", "camera", "Transform armed, left drag on ground", "Camera orbits", { 2: [], 3: [35] }, async x => {
      await x.util("Transform"); const h0 = await x.hash(); await x.drag(500, 700, 560, 740, { n: 10, dt: 120 });
      const ms = await x.changed(h0, null, 6000); return { pass: ms >= 0, observed: `view changed after ${ms} ms` };
    });
    add("cam.set-camera", "camera", "Action Set Camera", "No fem3d control (host gesture): gesture-driven camera change covered by cam.* checks", { 2: [], 3: [123] }, async () => SKIP("host gesture only; covered by cam.orbit-middle, cam.pan-*, util.wheel-zoom"));
    add("view.per-window-state", "windows", "Orbit Model camera, compare Results view", "Results window camera/state independent of Model window", { 2: [], 3: [167] }, async x => {
      await x.toSummary(); await x.canvasLayout(); await x.solved(200000);
      const r0 = await x.stable(a.resultsClip), m0 = await x.stable();
      await x.drag(500, 600, 560, 650, { n: 8, dt: 120, button: "middle" });
      const m1 = await x.stable(), r1 = await x.stable(a.resultsClip);
      return { pass: m0 !== m1 && r0 === r1, observed: `model ${m0}->${m1} results ${r0}->${r1}` };
    }, { fresh: true });
  }

  const gumClip = a.gumClip ?? a.modelClip;
  const gumballBase = async x => {
    await x.toSummary(); const r = await x.pickAt(a.nodeA);
    if (!(await x.pressedTools()).includes("Transform")) await x.util("Transform");
    await x.page.waitForTimeout(1500);
    if (d === 2) await x.until(async () => (await x.analyze(gumClip, "r>200&&g<90&&b<90")).n > 15, 30000, 700);
    if (d === 3) { await x.optionsLayout(); await x.stable(gumClip); }
    return r;
  };
  add("util.transform-gumball", "utilities", "Transform utility with selection", "Gumball drawn at selection, option checkboxes shown", { 2: [26], 3: [8, 15] }, async x => {
    const r = await gumballBase(x);
    const opts = ["Move", "Rotate", "Scale Axes", "Uniform Scale", ...(d === 3 ? ["Move Planes"] : [])];
    const seen = []; for (const o of opts) if (await x.page.getByRole("checkbox", { name: o, exact: true }).count()) seen.push(o);
    const dr = await x.analyze(gumClip, "r>200&&g<90&&b<90");
    return { pass: r.ok && seen.length === opts.length && dr.n > 15, observed: `pick=${r.ok} options=[${seen}] redPixels=${dr.n}` };
  }, { fresh: d === 3 });
  const OPTS = d === 3 ? [["Move", 9], ["Move Planes", 10], ["Rotate", 11], ["Scale Axes", 12], ["Uniform Scale", 13]] : [["Move", 30], ["Rotate", 30], ["Scale Axes", 30], ["Uniform Scale", 30]];
  for (const [o, n] of OPTS) add(`util.option-${o.toLowerCase().replace(/ /g, "-")}`, "utilities", `Uncheck Transform option ${o}`, "aria-checked false, handle group hidden (image changes), restored when re-checked", { 2: [30], 3: [n] }, async x => {
    await gumballBase(x);
    const box = x.page.getByRole("checkbox", { name: o, exact: true }).first();
    if (!(await box.count())) return { pass: false, observed: `checkbox ${o} missing` };
    if (d === 2) {
      const G = "(r>200&&g<90&&b<90)||(r>230&&g>120&&g<190&&b<70)||(r<110&&g>170&&b>150&&b<230)", n = async () => (await x.analyze(gumClip, G)).n, settle = async want => x.until(async () => want(await n()), 20000, 600);
      const n0 = await n(); await x.tap(box); await settle(v => v < n0 - 15); const nOff = await n(); const off = !(await x.isOn(box));
      await x.tap(box); await settle(v => Math.abs(v - n0) <= 15); const nOn = await n(); const on = await x.isOn(box);
      return { pass: off && on && nOff < n0 - 15 && Math.abs(nOn - n0) <= 15, observed: `unchecked=${off} gumball pixels ${n0} -> ${nOff} rechecked=${on} -> ${nOn}` };
    }
    const h0 = await x.stable(gumClip); await x.tap(box); await x.changed(h0, gumClip, 15000);
    const off = !(await x.isOn(box)); const h1 = await x.stable(gumClip);
    await x.tap(box); await x.changed(h1, gumClip, 15000); const on = await x.isOn(box); const h2 = await x.stable(gumClip);
    return { pass: off && on && h0 !== h1 && h0 === h2, observed: `unchecked=${off} imageChanged=${h0 !== h1} rechecked=${on} imageRestored=${h0 === h2}` };
  });

  const nHist = async x => (await x.commands()).length;
  const gumballDrag = async (x, option, how, dx, dy = 0, mod) => {
    await x.panel("History", false); await x.panel("Inspection", true);
    const all = d === 3 ? ["Move", "Move Planes", "Rotate", "Scale Axes", "Uniform Scale"] : ["Move", "Rotate", "Scale Axes", "Uniform Scale"];
    const hh = await x.handleOf(option, gumClip);
    if (!hh) return { pass: false, observed: `no gumball handle found for ${option} (diff of image with option on/off empty)` };
    await x.soloOption(option, all);
    const p = hh[how] ?? [hh.cx, hh.cy]; const gx = hh.clip.x + p[0], gy = hh.clip.y + p[1];
    const h0 = await x.hash(gumClip); const n0 = await nHist(x); await x.panel("History", false);
    const mark = x.mark();
    await x.drag(gx, gy, gx + dx, gy + dy, { n: 12, dt: 150, hover: 1200 });
    await x.page.waitForTimeout(1500);
    const n1 = await (async () => { const ms = await x.until(async () => (await nHist(x)) > n0, 60000, 500); await x.page.waitForTimeout(4000); return { ms, n: await nHist(x) }; })();
    await x.panel("History", false); await x.panel("Inspection", true);
    const h1 = await x.hash(gumClip); const con = x.consoleSince(mark);
    return { pass: n1.n > n0 && h0 !== h1 && con.fatal === 0, observed: `handle@${Math.round(gx)},${Math.round(gy)} commands ${n0}->${n1.n} imageChanged=${h0 !== h1} fatal=${con.fatal} ${con.fatalFirst.join("|")}`, gx, gy, h0 };
  };
  add("gumball.translate", "utilities", "Drag move handle +60 px", "Entity moves, History gains a command, Results re-solve", { 2: [27], 3: [16] }, async x => {
    await gumballBase(x); const r = await gumballDrag(x, "Move", "right", 60); r.observed += ` solved=${await x.solved(150000)}`; return r;
  }, { timeout: 300000, fresh: true });
  add("gumball.guide-cleared", "utilities", "View after translate drag", "No leftover guide line across the viewport", { 2: [], 3: [23] }, async x => {
    const t = await x.analyze(gumClip, "r>200&&g<110&&b<90"); const l = t.run;
    return { pass: l < 250, observed: `longest red/orange horizontal run=${l}px` };
  });
  add("gumball.undo-step", "utilities", "Ctrl+Z after the gumball drag", "Gesture reverted in one undo step", { 2: [], 3: [22] }, async x => {
    const n0 = await nHist(x); await x.page.mouse.move(...a.wheel); await x.page.keyboard.press("Control+z");
    const ms = await x.until(async () => (await nHist(x)) < n0 || (await nHist(x)) !== n0, 15000, 500);
    return { pass: ms >= 0, observed: `commands ${n0} -> ${await nHist(x)} (${ms} ms)` };
  });
  add("gumball.plane", "utilities", "Drag plane handle (Move Planes)", "Geometry moves on the plane", { 2: [], 3: [17] }, async x => {
    await gumballBase(x); return gumballDrag(x, "Move Planes", "quad", 40, -30);
  }, { skipIf: d !== 3, timeout: 300000, fresh: true });
  const dist = (p, q) => Math.hypot(...p.map((v, i) => v - q[i]));
  const mid = (p, q) => p.map((v, i) => (v + q[i]) / 2);
  const readTree = async x => {
    await x.panel("Artifact", true); await x.page.waitForTimeout(1500);
    const r = { p: await x.nodePositions(), h: await x.treeHeadCounts() };
    await x.ensureInsp(); await x.page.waitForTimeout(1000); return r;
  };
  const gumballGeometry = async (x, option, how, dx, kind) => {
    await x.toSummary(); const c0 = await x.counts(); const t0 = await readTree(x); const heads0 = t0.h;
    const pair = await x.pickPair(a.nodeA, a.nodeB); if (!pair.ok) return { pass: false, observed: `precondition: two nodes not selected ${JSON.stringify(pair)}` };
    if (!(await x.pressedTools()).includes("Transform")) await x.util("Transform");
    await x.page.waitForTimeout(1500);
    if (d === 2) await x.until(async () => (await x.analyze(gumClip, "r>200&&g<90&&b<90")).n > 15, 30000, 700);
    if (d === 3) { await x.optionsLayout(); await x.stable(gumClip); }
    const p0 = t0.p;
    const r = await gumballDrag(x, option, how, dx); if (r.pass === false && !/commands/.test(r.observed)) return r;
    const known = Object.keys(p0).length >= 2;
    let p1 = p0, settled = false, last = "";
    const moved = () => Object.keys(p0).filter(k => p1[k] && dist(p0[k], p1[k]) > 1e-6);
    if (known) await x.until(async () => { p1 = (await readTree(x)).p; const key = JSON.stringify(p1); settled = moved().length > 0 && key === last; last = key; return settled; }, 90000, 1500);
    const s = await x.solved(240000); const heads1 = (await readTree(x)).h; await x.toSummary(); const c1 = await x.counts();
    const con = x.consoleSince(x.mark?.() ?? 0);
    const same = Object.keys(heads0).length > 0 && JSON.stringify(heads0) === JSON.stringify(heads1) && (!c0 || !c1 || sum(c0) === sum(c1));
    let geo = true, why = "";
    if (known) {
      const M = moved(); const c = pts => pts[0].map((_, i) => pts.reduce((a, q) => a + q[i], 0) / pts.length);
      const A0 = M.map(k => p0[k]), A1 = M.map(k => p1[k]); const cen = M.length ? dist(c(A0), c(A1)) : Infinity;
      const keysSame = Object.keys(p0).length === Object.keys(p1).length;
      const rat = []; for (let i = 0; i < M.length; i++) for (let j = i + 1; j < M.length; j++) { const d0 = dist(A0[i], A0[j]); if (d0 > 1e-6) rat.push(dist(A1[i], A1[j]) / d0); }
      const lo = Math.min(...rat), hi = Math.max(...rat), tol = 0.01;
      if (kind === "rotate") { geo = settled && M.length >= 2 && keysSame && cen < 0.03 && lo > 1 - tol && hi < 1 + tol; why = `moved ${M.length} nodes [${M}] centroidShift=${cen.toFixed(3)} distance ratios ${lo.toFixed(4)}..${hi.toFixed(4)} nodes ${Object.keys(p0).length}->${Object.keys(p1).length}`; }
      else { geo = settled && M.length >= 2 && keysSame && cen < 0.03 && hi - lo < tol && Math.abs(lo - 1) > 0.01; why = `moved ${M.length} nodes [${M}] centroidShift=${cen.toFixed(3)} uniform factor ${lo.toFixed(3)}..${hi.toFixed(3)}`; }
    } else why = "tree gave no node coordinates";
    known || (geo = false);
    return { pass: r.h0 !== undefined && same && geo && s >= 0 && con.fatal === 0, observed: `${r.observed} ${why} entities ${JSON.stringify(heads0)}->${JSON.stringify(heads1)} summary ${c0 ? sum(c0) : "?"}->${c1 ? sum(c1) : "?"} solved=${s}` };
  };
  add("gumball.rotate-small", "utilities", "Select two nodes, drag rotate ring about 18 px", "Selection rotates about its centroid (distance and centroid kept, others fixed), entity counts unchanged, Results re-solve", { 2: [28], 3: [18] }, async x => gumballGeometry(x, "Rotate", "bottom", 18, "rotate"), { timeout: 700000, fresh: true });
  add("gumball.rotate-large", "utilities", "Select two nodes, drag rotate ring about 60 px", "Positions map exactly by a rotation, every entity stays in the model (counts unchanged), no analysis failure", { 2: [28], 3: [19] }, async x => {
    const r = await gumballGeometry(x, "Rotate", "bottom", 60, "rotate"); return { pass: r.pass && !/failed/i.test(await x.caption()), observed: `${r.observed} caption=${JSON.stringify(await x.caption())}` };
  }, { timeout: 700000, fresh: true });
  add("gumball.scale-axis", "utilities", "Drag axis scale cube", "Selection scales", { 2: [29], 3: [20] }, async x => {
    await gumballBase(x); return gumballDrag(x, "Scale Axes", "right", 40);
  }, { timeout: 300000, fresh: true });
  add("gumball.scale-uniform", "utilities", "Select two nodes, drag uniform scale handle", "Selection scales uniformly about its centroid (direction and centroid kept, spacing changes), others fixed", { 2: [29], 3: [21] }, async x => gumballGeometry(x, "Uniform Scale", "right", 30, "scale"), { timeout: 700000, fresh: true });
  add("gumball.pointer-cancel", "utilities", "Pointer cancel during gumball drag", "Selection kept, no panic", { 2: [31], 3: [] }, async x => {
    await x.toSummary(); const r = await x.pickAt(a.nodeA); await x.util("Transform").catch(() => {});
    const hh = await x.handleOf("Move"); if (!hh) return { pass: false, observed: "no gumball handle" };
    const gx = hh.clip.x + hh.right[0], gy = hh.clip.y + hh.right[1]; const mark = x.mark();
    await x.hover(gx, gy, 1200); await x.page.mouse.down(); await x.page.mouse.move(gx + 20, gy, { steps: 5 });
    await x.page.evaluate(() => { const c = document.querySelector("canvas"); c?.dispatchEvent(new PointerEvent("pointercancel", { bubbles: true, pointerId: 1 })); });
    await x.page.mouse.up(); await x.page.waitForTimeout(2500); const s = await x.selection(); const con = x.consoleSince(mark);
    return { pass: r.ok && s.n >= 1 && con.fatal === 0, observed: `selection=${JSON.stringify(s)} fatal=${con.fatal}` };
  }, { skipIf: d !== 2 });
  add("focus.entity", "inspection", "Inspection Focus on node", "Camera recentres (Model image changes after the view was moved)", { 2: [55], 3: [39, 40] }, async x => {
    await x.panel("Inspection", true);
    const r = await x.selectRow(a.rows.node, { group: "Nodes" });
    await x.page.mouse.move(...a.wheel); await x.page.mouse.wheel(0, 700); await x.page.waitForTimeout(2500); const h0 = await x.hash();
    await x.inspTree().getByRole("button", { name: "Focus", exact: true }).first().click({ timeout: 6000 });
    const ms = await x.changed(h0, null, 8000);
    return { pass: r.ok && ms >= 0, observed: `select=${r.ok} view changed after ${ms} ms` };
  }, { fresh: true });

  const cnt = async x => { const c = await x.counts(); return c; };
  const delDef = (id, action, expected, inv, how) => add(id, "delete", action, expected, inv, async x => {
    const c0 = await cnt(x); if (!c0) return { pass: false, observed: "no summary counts" };
    const mark = x.mark(); const picked = await how(x); if (picked && picked.ok === false) return { pass: false, observed: `precondition: ${JSON.stringify(picked)}` };
    const ms = await x.until(async () => { const k = await x.summaryCounts(); return Object.keys(k).length && sum(k) < sum(c0); }, 40000, 500);
    const c1 = await x.counts(); const con = x.consoleSince(mark);
    return { pass: !!c1 && sum(c1) < sum(c0) && con.fatal === 0, observed: `entities ${sum(c0)} -> ${c1 ? sum(c1) : "?"} (${ms} ms) fatal=${con.fatal}` };
  }, { timeout: 120000 });
  add("del.reference", "delete", "Fresh baseline", "Demo counts captured", { 2: [], 3: [] }, async x => ({ pass: !!(await cnt(x)), observed: JSON.stringify(await x.counts()) }), { fresh: true });
  delDef("del.key-canvas", "Canvas pick then Delete key", "Entity removed", { 2: [47], 3: [42] }, async x => { const r = await x.pickAt(a.nodeA); await x.page.mouse.move(...a.nodeA.at); await x.page.keyboard.press("Delete"); return r; });
  const nodesNow = async x => { await x.panel("Artifact", true); return x.groupCount("Nodes"); };

  const keyHist = async (x, keys) => {
    await x.panel("Artifact", true); const g0 = await x.groupCount("Nodes");
    const ad = await x.runAction("addNode", { x: 3.5, y: 3.5, z: 0 }, { verify: { pre: async () => g0, post: async p => (await x.groupCount("Nodes")) === p + 1 } });
    if (!ad.pass) return { pass: false, observed: `precondition addNode failed: ${ad.observed}` };
    await x.page.evaluate(() => document.activeElement?.blur?.()); await x.page.mouse.move(...a.wheel); await sleep(800);
    const seen = [];
    for (const [key, want] of keys) {
      await x.page.keyboard.press(key);
      const ms = await x.until(async () => (await nodesNow(x)) === g0 + want, 40000, 1000); seen.push(`${key} -> nodes ${await nodesNow(x)} (want ${g0 + want}, ${ms} ms)`);
      if (ms < 0) return { pass: false, observed: seen.join("; ") };
      await x.page.mouse.move(...a.wheel);
    }
    return { pass: true, observed: `base ${g0}; ${seen.join("; ")}` };
  };
  add("hist.undo-key", "history", "Ctrl+Z after an edit", "The added node is removed", { 2: [112], 3: [45] }, async x => keyHist(x, [["Control+z", 0]]));
  add("hist.redo-shift", "history", "Ctrl+Z then Ctrl+Shift+Z", "Edit re-applied", { 2: [113], 3: [46] }, async x => keyHist(x, [["Control+z", 0], ["Control+Shift+z", 1]]));
  add("hist.redo-y", "history", "Ctrl+Z then Ctrl+Y", "Edit re-applied", { 2: [114], 3: [47] }, async x => keyHist(x, [["Control+z", 0], ["Control+y", 1]]));
  const histEdit = async x => {
    await x.panel("Artifact", true); const g0 = await x.groupCount("Nodes");
    const ad = await x.runAction("addNode", { x: 2.5, y: 2.5, z: 0 }, { verify: { pre: async () => g0, post: async p => (await x.groupCount("Nodes")) === p + 1 } });
    return { ad, g0 };
  };
  add("hist.buttons", "history", "History panel Undo then Redo buttons", "Undo removes the node just added, Redo restores it", { 2: [115, 116], 3: [48] }, async x => {
    const { ad, g0 } = await histEdit(x); if (!ad.pass) return { pass: false, observed: `precondition addNode failed: ${ad.observed}` };
    const press = async n => { await x.panel("History", true); await x.historyPanel().getByRole("button", { name: n, exact: true }).first().click({ timeout: 8000 }); };
    const nodes = async () => { const v = await x.groupCount("Nodes"); await x.panel("History", true); return v; };
    await press("Undo"); const um = await x.until(async () => (await nodes()) === g0, 30000, 1000); const u = await nodes();
    await press("Redo"); const rm = await x.until(async () => (await nodes()) === g0 + 1, 30000, 1000); const r = await nodes();
    return { pass: um >= 0 && rm >= 0, observed: `nodes base ${g0} after edit ${g0 + 1} after Undo ${u} (${um} ms) after Redo ${r} (${rm} ms)` };
  });
  add("hist.command-log", "history", "History Commands list after edits", "Rows listed for the executed edits", { 2: [8], 3: [54] }, async x => {
    const { ad } = await histEdit(x); if (!ad.pass) return { pass: false, observed: `precondition addNode failed: ${ad.observed}` };
    let c = []; await x.until(async () => { c = await x.commands(); return c.some(r => /Create node/i.test(r)); }, 20000, 800);
    return { pass: c.some(r => /Create node/i.test(r)), observed: `${c.length} rows: ${c.slice(0, 5).join(" | ")}` };
  });
  add("hist.palette-undo-redo", "history", "Palette Undo and Redo", "Edit reverted and re-applied", { 2: [115, 116], 3: [121] }, async x => {
    await x.toSummary(); const base = sum(await x.summaryCounts()); const g = await x.groupCount("Nodes");
    const ad = await x.runAction("addNode", { x: 1.5, y: 1.5, z: 0 }, { verify: { pre: async () => x.groupCount("Nodes"), post: async p => (await x.groupCount("Nodes")) === p + 1 } });
    const un = await x.runAction("undo", {}, { verify: { pre: async () => 0, post: async () => (await x.groupCount("Nodes")) === g } });
    const re = await x.runAction("redo", {}, { verify: { pre: async () => 0, post: async () => (await x.groupCount("Nodes")) === g + 1 } });
    return { pass: ad.pass && un.pass && re.pass, observed: `add:${ad.pass} undo:${un.pass} redo:${re.pass} base=${base}` };
  });
  delDef("del.key-tree", "Tree row select then Delete key", "Entity removed", { 2: [48], 3: [42] }, async x => { const r = await x.selectRow(a.rows.element2, { group: "Elements" }); await x.page.evaluate(() => document.activeElement?.blur?.()); await x.page.mouse.move(...a.wheel); await x.page.waitForTimeout(800); await x.page.keyboard.press("Delete"); return r; });
  delDef("del.backspace-tree", "Tree row select then Backspace", "Entity removed", { 2: [49], 3: [43] }, async x => { const r = await x.selectRow(a.rows.element, { group: "Elements" }); await x.page.evaluate(() => document.activeElement?.blur?.()); await x.page.mouse.move(...a.wheel); await x.page.waitForTimeout(800); await x.page.keyboard.press("Backspace"); return r; });
  delDef("del.backspace-canvas", "Canvas pick then Backspace", "Entity removed", { 2: [50], 3: [43] }, async x => { const r = await x.pickAt(a.nodeB); await x.page.mouse.move(...a.nodeB.at); await x.page.keyboard.press("Backspace"); return r; });
  delDef("del.inspection-element", "Inspection Delete on element", "Element removed", { 2: [51], 3: [44] }, async x => { const r = await x.selectRow(a.rows.element3 ?? a.rows.element2, { group: "Elements" }); await x.btn("Delete").last().click({ timeout: 6000 }); return r; });
  delDef("del.inspection-node", "Inspection Delete on node", "Node and dependents removed", { 2: [52], 3: [] }, async x => { const r = await x.selectRow(a.rows.node2, { group: "Nodes" }); await x.btn("Delete").last().click({ timeout: 6000 }); return r; });
  const ENT = d === 2 ? [["support", "Supports"], ["loadNodal", "Load Cases"], ["region", "Regions"], ["combination", "Combinations"], ["loadCase", "Load Cases"], ["section", "Sections"], ["material", "Materials"]] : [["support", "Supports"], ["loadNodal", "Load Cases"], ["solid", "Solids"], ["combination", "Combinations"], ["loadCase", "Load Cases"], ["section", "Sections"], ["material", "Materials"]];
  const REFUSAL = { material: [/Nothing was removed: the material .* is still used by/, /Nichts wurde entfernt: Das Material .* wird noch von/], section: [/Nothing was removed: the section .* is still used by/, /Nichts wurde entfernt: Der Querschnitt .* wird noch von/] };
  for (const [k, g] of ENT) {
    if (REFUSAL[k]) add(`del.inspection-${k}`, "delete", `Inspection Delete on a referenced ${k}`, `Refused with an accessible EN notice naming the referrers; nothing removed`, { 2: [53], 3: [] }, async x => {
      const c0 = await cnt(x); if (!c0) return { pass: false, observed: "no summary counts" };
      const mark = x.mark(); const r = await x.selectRow(a.rows[k], { group: g }); if (r.ok === false) return { pass: false, observed: `precondition: ${JSON.stringify(r)}` };
      await x.btn("Delete").last().click({ timeout: 6000 });
      const notice = await x.waitAlert(REFUSAL[k][0], 20000); const c1 = await x.counts(); const con = x.consoleSince(mark);
      return { pass: !!notice && !!c1 && sum(c1) === sum(c0) && con.fatal === 0, observed: `notice=${JSON.stringify(notice)} entities ${sum(c0)} -> ${c1 ? sum(c1) : "?"} fatal=${con.fatal}` };
    }, { timeout: 120000 });
    else if (k === "loadNodal") add(`del.inspection-${k}`, "delete", "Inspection Delete on a nodal load", "The load row leaves its load case (the case count is unchanged)", { 2: [53], 3: [] }, async x => {
      const mark = x.mark(); const r = await x.selectRow(a.rows[k], { group: g }); if (r.ok === false) return { pass: false, observed: `precondition: ${JSON.stringify(r)}` };
      const rows = () => x.rowLoc(a.rows[k]).count(); const n0 = await rows();
      await x.btn("Delete").last().click({ timeout: 6000 });
      const ms = await x.until(async () => (await rows()) < n0, 40000, 500); const con = x.consoleSince(mark);
      return { pass: ms >= 0 && con.fatal === 0, observed: `load rows ${n0} -> ${await rows()} (${ms} ms) fatal=${con.fatal}` };
    }, { timeout: 120000 });
    else delDef(`del.inspection-${k}`, `Inspection Delete on ${k}`, `${k} removed`, { 2: [53], 3: [] }, async x => { const r = await x.selectRow(k === "support" ? a.rows.supportB ?? a.rows.support : a.rows[k], { group: g }); await x.btn("Delete").last().click({ timeout: 6000 }); return r; });
  }
  add("del.palette-remove-selection", "delete", "Palette Remove Selection with element selected", "Selection removed (count drops)", { 2: [105], 3: [109] }, async x => {
    await x.selectRow(a.rows.element2, { group: "Elements" }); const g = await x.groupCount("Elements");
    const r = await x.runAction("removeSelection", {}, { verify: { pre: async () => 0, post: async () => (await x.groupCount("Elements")) < g } });
    return r;
  }, { timeout: 120000, fresh: true });
  add("del.combination-term-zero", "delete", "Combination term factor 0", "The term leaves the Inspection and the other term stays", { 2: [54], 3: [93] }, async x => {
    await x.selectRow(a.rows.combination, { group: "Combinations" });
    const f = await x.field(/dead/i, ["combination.term.dead"]); if (!f) return { pass: false, observed: "no term factor field" };
    const inputs = () => x.inspTree().locator("input").count(); const n0 = await inputs(); await x.setField(f, 0);
    const ms = await x.until(async () => (await inputs()) === n0 - 1 && !(await x.field(/dead/i, ["combination.term.dead"])), 30000, 500);
    return { pass: ms >= 0 && (await x.field(/live/i, ["combination.term.live"])) !== null, observed: `inputs ${n0} -> ${await inputs()}, term removed after ${ms} ms` };
  }, { fresh: true });

  const ckp = (id, action, expected, inv, run) => add(id, "history", action, expected, inv, run, { timeout: 120000 });
  ckp("ckp.commit", "Palette Commit Checkpoint", "Checkpoint created (History lists it, no error)", { 2: [117], 3: [49, 50] }, async x => {
    const n0 = await nHist(x), t0 = await x.text(); const r = await x.runAction("commitCheckpoint", {});
    await sleep(2000); const n1 = await nHist(x); const t1 = await x.text();
    return { pass: r.pass && (n1 > n0 || t1 !== t0), observed: `${r.observed}; commands ${n0}->${n1}; pageTextChanged=${t1 !== t0}` };
  });
  ckp("ckp.create-alternative", "Palette Create Alternative", "Alternative created (no vcs error)", { 2: [118], 3: [51] }, async x => {
    const r = await x.runAction("createAlternative", {}); await sleep(2500);
    const bad = x.logs.filter(l => /no checkpoint for alternative/.test(l.text)).length;
    return { pass: r.pass && bad === 0, observed: `${r.observed}; 'no checkpoint for alternative' errors=${bad}` };
  });
  ckp("ckp.switch-alternative", "Palette Switch Alternative", "The History alternative list marks the other alternative current after the switch", { 2: [119], 3: [52] }, async x => {
    let at = "start";
    try {
      const alts = () => x.page.evaluate(() => [...document.querySelectorAll("[id*='framework.history.alternative.']")].map(e => ({ id: e.id.split("alternative.")[1], current: /\bCurrent\b/.test(e.textContent ?? "") })));
      const history = async () => { at = "history panel"; await x.panel("History", true); await sleep(800); return alts(); };
      let rows = await history();
      if (rows.length < 2) {
        at = "commit"; await x.runAction("commitCheckpoint", {}); await sleep(2000);
        at = "create"; const c = await x.runAction("createAlternative", {}); if (!c.pass) return { pass: false, observed: `createAlternative: ${c.observed}` }; await sleep(2500);
        await history(); at = "settle"; await x.until(async () => { const r = await alts(); return r.length >= 2 && r.some(a2 => a2.current); }, 30000, 700); rows = await alts();
      }
      const from = rows.find(r => r.current), to = rows.find(r => !r.current);
      if (!from || !to) return { pass: false, observed: `alternatives ${JSON.stringify(rows)}` };
      at = "switch"; const r = await x.runAction("switchAlternative", { alternativeId: to.id });
      await history(); at = "verify"; const ms = await x.until(async () => (await alts()).find(a2 => a2.id === to.id)?.current === true, 30000, 600);
      return { pass: r.pass && ms >= 0, observed: `${from.id} -> ${to.id} current after ${ms} ms; ${r.observed}` };
    } catch (e) { return { pass: false, observed: `step "${at}": ${String(e.message ?? e).split("\n")[0].slice(0, 200)}` }; }
  });
  ckp("ckp.checkout", "Palette Checkout Checkpoint", "Checkpoint restored without error", { 2: [120], 3: [53] }, async x => {
    const r = await x.runAction("checkoutCheckpoint", {}); return { pass: r.pass, observed: r.observed };
  });

  add("io.export", "import-export", "Palette Export Document", "Download starts, file is a non-empty .semio-archive", { 2: [110], 3: [118] }, async x => {
    const dl = x.page.waitForEvent("download", { timeout: 60000 }).catch(() => null);
    await x.runAction("exportDocument", {}); const d1 = await dl;
    if (!d1) return { pass: false, observed: "no download event" };
    const path = `${x.dirs.out}/export-${a.key}.semio-archive`; await d1.saveAs(path); x.exported = path;
    const size = statSync(path).size; return { pass: size > 0 && /\.semio-archive$/.test(d1.suggestedFilename()), observed: `${d1.suggestedFilename()} ${size} bytes` };
  }, { timeout: 120000 });
  add("io.import-probe-json", "import-export", "Import Document with a JSON probe", "Rejected with a message, model unchanged, app alive", { 2: [111], 3: [119] }, async x => {
    const path = `${x.dirs.out}/probe-${a.key}.json`; writeFileSync(path, JSON.stringify({ probe: true }));
    const c0 = await x.counts(); const mark = x.mark();
    const picked = await x.importFile(path); if (!picked.ok) return { pass: false, observed: picked.why };
    const alert = await x.waitAlert(/./, 15000); const warned = x.logs.slice(mark).some(l => /import-unreadable|import-failed/.test(l.text)); const msg = alert; const con = x.consoleSince(mark);
    await x.toSummary(); const c1 = await x.counts();
    return { pass: JSON.stringify(c0) === JSON.stringify(c1) && !!msg && warned, observed: `alert=${JSON.stringify(msg)} refusalLogged=${warned} consoleErrors=${con.errors} ${con.first.join("|").slice(0, 160)} counts same=${JSON.stringify(c0) === JSON.stringify(c1)}` };
  });
  add("io.import-archive", "import-export", "Import the exported archive", "Archive restored (counts equal), edits still work", { 2: [111], 3: [120] }, async x => {
    if (!x.exported) return { pass: false, observed: "no exported archive (io.export failed or not run)" };
    const c0 = await x.counts(); const mark = x.mark();
    const picked = await x.importFile(x.exported); if (!picked.ok) return { pass: false, observed: picked.why }; await sleep(6000);
    await x.solved(200000); await x.toSummary(); const c1 = await x.counts();
    const edit = await x.editField(a.rows.node, "X", { keys: ["node.x"], value: 6.25, group: "Nodes" }); const con = x.consoleSince(mark);
    return { pass: JSON.stringify(c0) === JSON.stringify(c1) && edit.pass && con.fatal === 0, observed: `counts ${JSON.stringify(c0)} -> ${JSON.stringify(c1)}; edit: ${edit.observed}; fatal=${con.fatal} ${con.fatalFirst.join("|")}` };
  }, { timeout: 420000 });

  add("clip.copy-paste", "clipboard", "Select element, Copy, Paste", "Element count +1", { 2: [107], 3: [116] }, async x => {
    await x.selectRow(a.rows.element, { group: "Elements" }); const e0 = await x.groupCount("Elements");
    const c = await x.runAction("copy", {}); const mark = x.mark();
    const p = await x.runAction("paste", {}, { verify: { pre: async () => 0, post: async () => (await x.groupCount("Elements")) === e0 + 1 }, timeout: 120000 });
    return { pass: c.pass && p.pass, observed: `copy: ${c.observed} | paste: ${p.observed}` };
  }, { timeout: 420000 });
  add("clip.cut-paste", "clipboard", "Select element, Cut, Paste", "Element removed on Cut, returns on Paste", { 2: [107], 3: [117] }, async x => {
    await x.selectRow(a.rows.element2, { group: "Elements" }); const e0 = await x.groupCount("Elements");
    const c = await x.runAction("cut", {}, { verify: { pre: async () => 0, post: async () => (await x.groupCount("Elements")) === e0 - 1 }, timeout: 120000 });
    const p = await x.runAction("paste", {}, { verify: { pre: async () => 0, post: async () => (await x.groupCount("Elements")) === e0 }, timeout: 120000 });
    return { pass: c.pass && p.pass, observed: `cut: ${c.observed} | paste: ${p.observed}` };
  }, { timeout: 420000 });

  return K.filter(k => !k.skipIf).filter(k => !k.skip);
}
