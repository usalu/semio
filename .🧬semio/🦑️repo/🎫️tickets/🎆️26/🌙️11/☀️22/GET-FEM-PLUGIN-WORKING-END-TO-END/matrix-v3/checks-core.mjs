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
    await x.btn("Command").first().click(); await x.page.waitForTimeout(1500);
    const t = await x.text(); const miss = ["Layout", "App", "Appearance", "Language"].filter(n => !new RegExp(`\\b${n}\\b`).test(t));
    await x.page.keyboard.press("Escape");
    return { pass: miss.length === 0, observed: `missing=[${miss}]` };
  });
  add("shell.command-locale", "shell", "Command > Language > Set Locale", "Locale picker input has non-zero size and is usable", { 2: [], 3: [169] }, async x => {
    await x.btn("Command").first().click(); await x.page.waitForTimeout(1200);
    await x.page.getByRole("tab", { name: "Language" }).or(x.page.getByText("Language", { exact: true })).first().click({ timeout: 5000 }).catch(() => {});
    await x.page.getByText("Set Locale", { exact: false }).first().click({ timeout: 5000 }).catch(() => {});
    await x.page.waitForTimeout(1200);
    const box = await x.page.locator('[id*="setLocale.arg.locale.input"]').first().boundingBox().catch(() => null);
    await x.page.keyboard.press("Escape");
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
    await x.btn("Command").first().click(); await x.page.waitForTimeout(1200);
    await x.page.getByText("Reset panels", { exact: true }).first().click({ timeout: 6000 }); await x.page.waitForTimeout(2500);
    let back = await x.page.getByRole("tab", { name: "Results" }).count();
    if (!back) { await x.btn(/Reset panels/, false).first().click({ timeout: 4000 }).catch(() => {}); await x.page.waitForTimeout(2500); back = await x.page.getByRole("tab", { name: "Results" }).count(); }
    return { pass: back > 0, observed: `Results tabs=${back}; tabs=${(await x.tabs()).join(",")}` };
  });
  add("win.reopen-display", "windows", "Display panel: drag window entry back", "Closed window returns", { 2: [], 3: [161] }, async x => {
    await x.panel("Settings", false);
    if ((await x.page.getByRole("tab", { name: "Results" }).count()) > 0) await x.winButtons("Results").getByRole("button", { name: "Close", exact: true }).click().catch(() => {});
    await x.page.waitForTimeout(1500);
    await x.btn("Display").first().click(); await x.page.waitForTimeout(1500);
    const entry = x.page.getByText("Results", { exact: true }).last(); const b = await entry.boundingBox();
    if (!b) return { pass: false, observed: "no Results entry in Display panel" };
    await x.drag(b.x + b.width / 2, b.y + b.height / 2, 1100, 500, { n: 10, dt: 120 });
    await x.page.waitForTimeout(2000);
    const back = await x.page.getByRole("tab", { name: "Results" }).count();
    return { pass: back > 0, observed: `Results tabs=${back}; tabs=${(await x.tabs()).join(",")}` };
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

  add("role.viewer-editor", "roles", "Role Viewer, then Editor", "Viewer layout (no Actions, Utilities, Inspection, Results), then Editor restores editing UI", { 2: [20, "V2D-01", "V2D-02", "V2D-03", "V2D-07"], 3: [158, 159, "V3D-01", "V3D-02", "V3D-03"] }, async x => {
    await x.panel("Inspection", false); await x.panel("History", false);
    await x.btn("Viewer").first().click(); await x.page.waitForTimeout(4000);
    const v = { pressed: await x.isPressed("Viewer"), actions: await x.page.getByRole("button", { name: /^Actions/ }).count(), utils: await x.page.getByRole("button", { name: /^Utilities/ }).count(), results: await x.page.getByRole("tab", { name: "Results" }).count(), insp: await x.page.getByRole("button", { name: "Inspection", exact: true }).count() };
    const edBtn = x.btn("Editor").first(); const disabled = await edBtn.isDisabled().catch(() => true);
    if (!disabled) { await edBtn.click(); await x.page.waitForTimeout(4000); }
    const e = { pressed: await x.isPressed("Editor"), actions: await x.page.getByRole("button", { name: /^Actions/ }).count() };
    return { pass: v.pressed === true && v.actions === 0 && v.utils === 0 && v.results === 0 && !disabled && e.pressed === true && e.actions > 0, observed: `viewer=${JSON.stringify(v)} editorDisabled=${disabled} editor=${JSON.stringify(e)}` };
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
    const r0 = await x.chooseOption(cb, /^No example$/i); await x.page.waitForTimeout(4000);
    const c0 = await x.counts(); const cleared = !c0 || (c0.Nodes ?? 0) === 0;
    const r1 = await x.chooseOption(cb, /^Demo$/); await x.page.waitForTimeout(5000);
    const c1 = await x.counts(); const solved = await x.solved(200000);
    return { pass: r0.ok && r1.ok && cleared && (c1?.Nodes ?? 0) > 0 && solved >= 0, observed: `cleared=${cleared} (${JSON.stringify(c0)}) restored=${JSON.stringify(c1)} solved=${solved} caption=${JSON.stringify(await x.caption())}` };
  }, { timeout: 420000 });
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
    await x.toSummary(); await x.hover(...a.empty, 1200); const h0 = await x.hash();
    await x.hover(...a.nodeA.at, 1800); const h1 = await x.hash();
    return { pass: h0 !== h1, observed: `empty ${h0} entity ${h1}` };
  });
  add("sel.click-replace", "selection", "Click entity A then entity B", "Selection replaced: only B selected", { 2: [32], 3: [14] }, async x => {
    await x.toSummary(); const A = await x.pickAt(a.nodeA); const B = await x.pickAt(a.nodeB); const s = await x.selection();
    return { pass: A.ok && B.ok && s.n === 1, observed: `A=${A.ok} B=${B.ok} selection=${JSON.stringify(s)}` };
  });
  add("sel.shift-additive", "selection", "Shift+click second entity", "Both selected (2 SELECTED)", { 2: [33], 3: [29] }, async x => {
    await x.toSummary(); await x.pickAt(a.nodeA); const r = await x.pickAt(a.nodeB, { mod: "Shift" }); const s = await x.selection();
    return { pass: s.n === 2, observed: `pick=${r.ok} selection=${JSON.stringify(s)}` };
  });
  add("sel.ctrl-subtractive", "selection", "Ctrl+click selected entity", "Entity removed from selection (1 left)", { 2: [34], 3: [29] }, async x => {
    const before = await x.selection();
    await x.hoverClick(...a.nodeB.at, { mod: "Control" }); await x.page.waitForTimeout(3000); const s = await x.selection();
    return { pass: before.n === 2 && s.n === 1, observed: `before=${before.n} after=${JSON.stringify(s)}` };
  });
  add("sel.shift-ctrl-invertive", "selection", "Shift+Ctrl+click entity twice", "Toggles: added, then removed", { 2: [35], 3: [29] }, async x => {
    await x.toSummary(); await x.pickAt(a.nodeA);
    await x.hoverClick(...a.nodeB.at, { mod: ["Shift", "Control"] }); await x.page.waitForTimeout(3000); const s1 = await x.selection();
    await x.hoverClick(...a.nodeB.at, { mod: ["Shift", "Control"] }); await x.page.waitForTimeout(3000); const s2 = await x.selection();
    return { pass: s1.n === 2 && s2.n === 1, observed: `after first=${s1.n} after second=${s2.n}` };
  });
  add("sel.empty-click", "selection", "Click empty canvas", "Selection cleared, Summary shown", { 2: [41], 3: [30] }, async x => {
    await x.pickAt(a.nodeA); const ok = await x.toSummary();
    return { pass: ok, observed: `summary=${ok}` };
  });
  if (d === 2) {
    for (const [mode, inv, steps] of [["Selective", 36], ["Additive", 37], ["Subtractive", 38], ["Invertive", 39]]) add(`sel.mode-${mode.toLowerCase()}`, "selection", `Utilities Mode radio ${mode}, then click`, mode === "Selective" ? "Click replaces selection" : mode === "Additive" ? "Click adds to selection (2)" : mode === "Subtractive" ? "Click on selected entity removes it (1)" : "Click on selected entity toggles it off, unselected on", { 2: [inv], 3: [] }, async x => {
      await x.util("Direct Select").catch(() => {}); await x.toSummary(); await x.radio(mode);
      await x.pickAt(a.nodeA);
      if (mode === "Selective") { await x.pickAt(a.nodeB); const s = await x.selection(); return { pass: s.n === 1, observed: `selection=${JSON.stringify(s)}` }; }
      await x.hoverClick(...a.nodeB.at); await x.page.waitForTimeout(3000); const s = await x.selection();
      if (mode === "Additive") return { pass: s.n === 2, observed: `selection=${JSON.stringify(s)}` };
      if (mode === "Subtractive") { await x.hoverClick(...a.nodeB.at); await x.page.waitForTimeout(3000); const s2 = await x.selection(); return { pass: s2.n === 1, observed: `after first click ${s.n}, second click ${JSON.stringify(s2)}` }; }
      await x.hoverClick(...a.nodeB.at); await x.page.waitForTimeout(3000); const s2 = await x.selection();
      return { pass: s.n === 2 && s2.n === 1, observed: `toggle on=${s.n} off=${s2.n}` };
    });
  }
  add("sel.marquee", "selection", "Marquee Select drag", "Entities inside rectangle selected (N SELECTED header with list)", { 2: [23, 43], 3: [24, 28] }, async x => {
    await x.toSummary(); await x.utilsOpen();
    if (!(await x.btn("Marquee Select").count())) return d === 3 ? { pass: false, observed: "3D Utilities has no Marquee Select; left-drag in Model tried below", extra: await (async () => { await x.drag(...a.marquee); await x.page.waitForTimeout(3000); return (await x.selection()).n; })() } : { pass: false, observed: "Marquee Select button missing" };
    await x.util("Marquee Select"); await x.drag(...a.marquee, { n: 6, dt: 150 });
    const ms = await x.until(async () => (await x.selection()).n >= 2, 25000); const s = await x.selection();
    return { pass: ms >= 0 && /selected/i.test(await x.insp()), observed: `selection=${JSON.stringify(s)} ms=${ms}` };
  });
  add("sel.marquee-tiny", "selection", "Marquee drag of 2 px on entity", "Acts as a click: that entity alone selected", { 2: [42], 3: [] }, async x => {
    await x.toSummary(); if (!(await x.btn("Marquee Select").count())) return SKIP("no Marquee Select");
    await x.util("Marquee Select"); const [px, py] = a.nodeA.at; await x.hover(px, py, 1500);
    await x.page.mouse.down(); await x.page.mouse.move(px + 2, py + 1, { steps: 2 }); await x.page.mouse.up(); await x.page.waitForTimeout(3500);
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
    await x.drag(300, 300, 450, 400, { n: 10, dt: 500, hover: 500 });
    const ms = await x.changed(h0, null, 8000);
    return { pass: ms >= 0, observed: `view changed after ${ms} ms` };
  });
  add("util.wheel-zoom", "utilities", "Mouse wheel over Model", "View zooms", { 2: [17], 3: [38] }, async x => {
    await x.toSummary(); const h0 = await x.hash(); await x.page.mouse.move(...a.wheel); await x.page.mouse.wheel(0, -500);
    const ms = await x.changed(h0, null, 8000);
    return { pass: ms >= 0, observed: `view changed after ${ms} ms` };
  });

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
      await x.toSummary(); const r0 = await x.hash(a.resultsClip), m0 = await x.hash();
      await x.drag(500, 600, 560, 650, { n: 8, dt: 120, button: "middle" });
      const m1 = await x.hash(), r1 = await x.hash(a.resultsClip);
      return { pass: m0 !== m1 && r0 === r1, observed: `model ${m0}->${m1} results ${r0}->${r1}` };
    });
  }

  const gumballBase = async (x, rowRe) => {
    await x.toSummary(); const r = await x.pickAt(a.nodeA); await x.util("Transform"); await x.page.waitForTimeout(1500);
    return r;
  };
  add("util.transform-gumball", "utilities", "Transform utility with selection", "Gumball drawn at selection, option checkboxes shown", { 2: [26], 3: [8, 15] }, async x => {
    const r = await gumballBase(x);
    const opts = ["Move", "Rotate", "Scale Axes", "Uniform Scale", ...(d === 3 ? ["Move Planes"] : [])];
    const seen = []; for (const o of opts) if (await x.page.getByRole("checkbox", { name: o, exact: true }).count()) seen.push(o);
    const dr = await x.analyze(a.modelClip, "r>200&&g<90&&b<90");
    return { pass: r.ok && seen.length === opts.length && dr.n > 15, observed: `pick=${r.ok} options=[${seen}] redPixels=${dr.n}` };
  });
  const OPTS = d === 3 ? [["Move", 9], ["Move Planes", 10], ["Rotate", 11], ["Scale Axes", 12], ["Uniform Scale", 13]] : [["Move", 30], ["Rotate", 30], ["Scale Axes", 30], ["Uniform Scale", 30]];
  for (const [o, n] of OPTS) add(`util.option-${o.toLowerCase().replace(/ /g, "-")}`, "utilities", `Uncheck Transform option ${o}`, "aria-checked false, handle group hidden (image changes), restored when re-checked", { 2: [30], 3: [n] }, async x => {
    const box = x.page.getByRole("checkbox", { name: o, exact: true }).first();
    if (!(await box.count())) return { pass: false, observed: `checkbox ${o} missing` };
    const h0 = await x.hash(); await box.click(); await x.page.waitForTimeout(1200);
    const off = (await box.getAttribute("aria-checked")) === "false" || !(await box.isChecked().catch(() => true)); const h1 = await x.hash();
    await box.click(); await x.page.waitForTimeout(1200); const on = (await box.getAttribute("aria-checked")) === "true" || (await box.isChecked().catch(() => false)); const h2 = await x.hash();
    return { pass: off && on && h0 !== h1 && h0 === h2, observed: `unchecked=${off} imageChanged=${h0 !== h1} rechecked=${on} imageRestored=${h0 === h2}` };
  });

  const nHist = async x => (await x.commands()).length;
  const gumballDrag = async (x, option, how, dx, dy = 0, mod) => {
    await x.panel("History", false); await x.panel("Inspection", true);
    const all = d === 3 ? ["Move", "Move Planes", "Rotate", "Scale Axes", "Uniform Scale"] : ["Move", "Rotate", "Scale Axes", "Uniform Scale"];
    const hh = await x.handleOf(option);
    if (!hh) return { pass: false, observed: `no gumball handle found for ${option} (diff of image with option on/off empty)` };
    await x.soloOption(option, all);
    const p = hh[how] ?? [hh.cx, hh.cy]; const gx = hh.clip.x + p[0], gy = hh.clip.y + p[1];
    const h0 = await x.hash(); const n0 = await nHist(x); await x.panel("History", false);
    const mark = x.mark();
    await x.drag(gx, gy, gx + dx, gy + dy, { n: 12, dt: 150, hover: 1200 });
    await x.page.waitForTimeout(1500);
    const n1 = await (async () => { const ms = await x.until(async () => (await nHist(x)) > n0, 15000, 500); return { ms, n: await nHist(x) }; })();
    const h1 = await x.hash(); const con = x.consoleSince(mark);
    return { pass: n1.ms >= 0 && h0 !== h1 && con.fatal === 0, observed: `handle@${Math.round(gx)},${Math.round(gy)} commands ${n0}->${n1.n} imageChanged=${h0 !== h1} fatal=${con.fatal} ${con.fatalFirst.join("|")}`, gx, gy, h0 };
  };
  add("gumball.translate", "utilities", "Drag move handle +60 px", "Entity moves, History gains a command, Results re-solve", { 2: [27], 3: [16] }, async x => {
    await gumballBase(x); const r = await gumballDrag(x, "Move", "right", 60); r.observed += ` solved=${await x.solved(150000)}`; return r;
  }, { timeout: 300000, fresh: true });
  add("gumball.guide-cleared", "utilities", "View after translate drag", "No leftover guide line across the viewport", { 2: [], 3: [23] }, async x => {
    const t = await x.analyze(a.modelClip, "r>200&&g<110&&b<90"); const l = t.run;
    return { pass: l < 250, observed: `longest red/orange horizontal run=${l}px` };
  });
  add("gumball.undo-step", "utilities", "Ctrl+Z after the gumball drag", "Gesture reverted in one undo step", { 2: [], 3: [22] }, async x => {
    const n0 = await nHist(x); await x.page.mouse.move(...a.wheel); await x.page.keyboard.press("Control+z");
    const ms = await x.until(async () => (await nHist(x)) < n0 || (await nHist(x)) !== n0, 15000, 500);
    return { pass: ms >= 0, observed: `commands ${n0} -> ${await nHist(x)} (${ms} ms)` };
  });
  add("gumball.plane", "utilities", "Drag plane handle (Move Planes)", "Geometry moves on the plane", { 2: [], 3: [17] }, async x => {
    await x.toSummary(); await x.pickAt(a.nodeA); await x.util("Transform").catch(() => {}); return gumballDrag(x, "Move Planes", "quad", 40, -30);
  }, { skipIf: d !== 3, timeout: 300000 });
  add("gumball.rotate-small", "utilities", "Drag rotate ring about 18 px", "Selection rotates, Results re-solve", { 2: [28], 3: [18] }, async x => {
    await x.toSummary(); await x.pickAt(a.nodeA); await x.util("Transform").catch(() => {}); const r = await gumballDrag(x, "Rotate", "bottom", 18); r.observed += ` solved=${await x.solved(150000)}`; return r;
  }, { timeout: 300000 });
  add("gumball.rotate-large", "utilities", "Drag rotate ring about 60 px", "Entity stays in the model (counts unchanged), no analysis failure", { 2: [28], 3: [19] }, async x => {
    await x.toSummary(); const c0 = await x.counts(); await x.pickAt(a.nodeA); await x.util("Transform").catch(() => {}); const r = await gumballDrag(x, "Rotate", "bottom", 60);
    const s = await x.solved(150000); await x.toSummary(); const c1 = await x.counts();
    return { pass: r.pass && sum(c0) === sum(c1) && s >= 0 && !/failed/i.test(await x.caption()), observed: `${r.observed} counts ${sum(c0)}->${sum(c1)} solved=${s} caption=${JSON.stringify(await x.caption())}` };
  }, { timeout: 300000 });
  add("gumball.scale-axis", "utilities", "Drag axis scale cube", "Selection scales", { 2: [29], 3: [20] }, async x => {
    await x.toSummary(); await x.pickAt(a.nodeA); await x.util("Transform").catch(() => {}); return gumballDrag(x, "Scale Axes", "right", 40);
  }, { timeout: 300000 });
  add("gumball.scale-uniform", "utilities", "Drag uniform scale handle", "Selection scales uniformly", { 2: [29], 3: [21] }, async x => {
    await x.toSummary(); await x.pickAt(a.nodeA); await x.util("Transform").catch(() => {}); return gumballDrag(x, "Uniform Scale", "right", 30);
  }, { timeout: 300000 });
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
    await x.btn("Focus").last().click({ timeout: 6000 });
    const ms = await x.changed(h0, null, 8000);
    return { pass: r.ok && ms >= 0, observed: `select=${r.ok} view changed after ${ms} ms` };
  });

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
  add("hist.undo-key", "history", "Ctrl+Z after delete", "Entity restored", { 2: [112], 3: [45] }, async x => {
    const c0 = await x.counts(); await x.page.mouse.move(...a.wheel); await x.page.keyboard.press("Control+z");
    const ms = await x.until(async () => { const k = await x.summaryCounts(); return sum(k) > sum(c0 ?? {}) || (await x.toSummary() && sum(await x.summaryCounts()) > sum(c0 ?? {})); }, 30000, 700);
    return { pass: ms >= 0, observed: `entities ${sum(c0 ?? {})} -> ${sum(await x.summaryCounts())} (${ms} ms)` };
  });
  add("hist.redo-shift", "history", "Ctrl+Shift+Z", "Edit re-applied", { 2: [113], 3: [46] }, async x => {
    const c0 = await x.counts(); await x.page.mouse.move(...a.wheel); await x.page.keyboard.press("Control+Shift+z");
    const ms = await x.until(async () => { await x.toSummary(); return sum(await x.summaryCounts()) < sum(c0 ?? {}); }, 30000, 700);
    return { pass: ms >= 0, observed: `entities ${sum(c0 ?? {})} -> ${sum(await x.summaryCounts())} (${ms} ms)` };
  });
  add("hist.redo-y", "history", "Undo then Ctrl+Y", "Edit re-applied", { 2: [114], 3: [47] }, async x => {
    await x.page.mouse.move(...a.wheel); await x.page.keyboard.press("Control+z"); await sleep(4000); await x.toSummary(); const c0 = await x.counts();
    await x.page.keyboard.press("Control+y");
    const ms = await x.until(async () => { await x.toSummary(); return sum(await x.summaryCounts()) < sum(c0 ?? {}); }, 30000, 700);
    await x.page.keyboard.press("Control+z");
    return { pass: ms >= 0, observed: `entities ${sum(c0 ?? {})} -> ${sum(await x.summaryCounts())} (${ms} ms)` };
  });
  add("hist.buttons", "history", "History panel Undo then Redo buttons", "Counts change back and forth", { 2: [115, 116], 3: [48] }, async x => {
    await x.toSummary(); const c0 = await x.counts(); await x.panel("History", true);
    const act = async n => { const b = x.page.getByRole("tree").filter({ hasText: /Commit Checkpoint/ }).last().getByRole("button", { name: n, exact: true }).first(); await b.click({ timeout: 8000 }); await sleep(3000); };
    const base = sum(c0 ?? {}); await act("Undo"); await x.toSummary(); const u = sum(await x.summaryCounts()); await x.panel("History", true); await act("Redo"); await x.toSummary(); const r = sum(await x.summaryCounts());
    return { pass: u !== base && r === base, observed: `base ${base} after Undo ${u} after Redo ${r}` };
  });
  add("hist.command-log", "history", "History Commands list after edits", "Rows listed for the executed edits", { 2: [8], 3: [54] }, async x => { const c = await x.commands(); return { pass: c.length > 0, observed: `${c.length} rows: ${c.slice(0, 5).join(" | ")}` }; });
  add("hist.palette-undo-redo", "history", "Palette Undo and Redo", "Edit reverted and re-applied", { 2: [115, 116], 3: [121] }, async x => {
    await x.toSummary(); const base = sum(await x.summaryCounts()); const g = await x.groupCount("Nodes");
    const ad = await x.runAction("addNode", { x: 1.5, y: 1.5, z: 0 }, { verify: { pre: async () => x.groupCount("Nodes"), post: async p => (await x.groupCount("Nodes")) === p + 1 } });
    const un = await x.runAction("undo", {}, { verify: { pre: async () => 0, post: async () => (await x.groupCount("Nodes")) === g } });
    const re = await x.runAction("redo", {}, { verify: { pre: async () => 0, post: async () => (await x.groupCount("Nodes")) === g + 1 } });
    return { pass: ad.pass && un.pass && re.pass, observed: `add:${ad.pass} undo:${un.pass} redo:${re.pass} base=${base}` };
  });
  delDef("del.key-tree", "Tree row select then Delete key", "Entity removed", { 2: [48], 3: [42] }, async x => { const r = await x.selectRow(a.rows.element2, { group: "Elements" }); await x.page.keyboard.press("Delete"); return r; });
  delDef("del.backspace-tree", "Tree row select then Backspace", "Entity removed", { 2: [49], 3: [43] }, async x => { const r = await x.selectRow(a.rows.element, { group: "Elements" }); await x.page.keyboard.press("Backspace"); return r; });
  delDef("del.backspace-canvas", "Canvas pick then Backspace", "Entity removed", { 2: [50], 3: [43] }, async x => { const r = await x.pickAt(a.nodeB); await x.page.mouse.move(...a.nodeB.at); await x.page.keyboard.press("Backspace"); return r; });
  delDef("del.inspection-element", "Inspection Delete on element", "Element removed", { 2: [51], 3: [44] }, async x => { const r = await x.selectRow(a.rows.element2, { group: "Elements" }); await x.btn("Delete").last().click({ timeout: 6000 }); return r; });
  delDef("del.inspection-node", "Inspection Delete on node", "Node and dependents removed", { 2: [52], 3: [] }, async x => { const r = await x.selectRow(a.rows.node2, { group: "Nodes" }); await x.btn("Delete").last().click({ timeout: 6000 }); return r; });
  const ENT = d === 2 ? [["support", "Supports"], ["loadNodal", "Load Cases"], ["region", "Regions"], ["combination", "Combinations"], ["loadCase", "Load Cases"], ["section", "Sections"], ["material", "Materials"]] : [["support", "Supports"], ["loadNodal", "Load Cases"], ["solid", "Solids"], ["combination", "Combinations"], ["loadCase", "Load Cases"], ["section", "Sections"], ["material", "Materials"]];
  for (const [k, g] of ENT) delDef(`del.inspection-${k}`, `Inspection Delete on ${k}`, `${k} removed`, { 2: [53], 3: [] }, async x => { const r = await x.selectRow(a.rows[k], { group: g }); await x.btn("Delete").last().click({ timeout: 6000 }); return r; });
  add("del.palette-remove-selection", "delete", "Palette Remove Selection with element selected", "Selection removed (count drops)", { 2: [105], 3: [109] }, async x => {
    await x.selectRow(a.rows.element2, { group: "Elements" }); const g = await x.groupCount("Elements");
    const r = await x.runAction("removeSelection", {}, { verify: { pre: async () => 0, post: async () => (await x.groupCount("Elements")) < g } });
    return r;
  }, { timeout: 120000 });
  add("del.combination-term-zero", "delete", "Combination term factor 0", "Term removed", { 2: [54], 3: [93] }, async x => {
    await x.selectRow(a.rows.combination, { group: "Combinations" });
    const f = await x.field(/dead/i, ["combination.term.dead"]); if (!f) return { pass: false, observed: "no term factor field" };
    const before = (await x.insp()); await x.setField(f, 0); await sleep(3000);
    const ms = await x.until(async () => !/dead/i.test((await x.artTree().innerText().catch(() => "")).split("Combinations")[1] ?? "") || !(await x.field(/dead/i, ["combination.term.dead"])), 20000, 500);
    return { pass: ms >= 0, observed: `term field removed after ${ms} ms` };
  });

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
  ckp("ckp.switch-alternative", "Palette Switch Alternative", "Alternative switched (form offers options, executes without error)", { 2: [119], 3: [52] }, async x => {
    const r = await x.runAction("switchAlternative", {}); return { pass: r.pass, observed: r.observed };
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
    const c0 = await x.counts(); await x.openPalette();
    const item = x.modelPanel().getByRole("button", { name: /^Import Document/ }).first(); await item.click(); await sleep(1500);
    const fi = x.page.locator('input[type="file"]').first(); if (!(await fi.count())) return { pass: false, observed: "no file input in Import form" };
    const mark = x.mark(); await fi.setInputFiles(path); await sleep(800);
    const ex = x.page.locator('[id$="importDocument.execute"]').first(); if (await ex.count()) await ex.click().catch(() => {}); await sleep(4000);
    const t = await x.text(); const msg = /reject|invalid|unsupported|cannot|could not|error|not a|failed/i.exec(t)?.[0]; const con = x.consoleSince(mark);
    await x.toSummary(); const c1 = await x.counts();
    return { pass: JSON.stringify(c0) === JSON.stringify(c1) && (!!msg || con.errors > 0), observed: `message=${msg} consoleErrors=${con.errors} ${con.first.join("|").slice(0, 160)} counts same=${JSON.stringify(c0) === JSON.stringify(c1)}` };
  });
  add("io.import-archive", "import-export", "Import the exported archive", "Archive restored (counts equal), edits still work", { 2: [111], 3: [120] }, async x => {
    if (!x.exported) return { pass: false, observed: "no exported archive (io.export failed or not run)" };
    const c0 = await x.counts(); await x.openPalette();
    await x.modelPanel().getByRole("button", { name: /^Import Document/ }).first().click(); await sleep(1500);
    const fi = x.page.locator('input[type="file"]').first(); if (!(await fi.count())) return { pass: false, observed: "no file input in Import form" };
    const mark = x.mark(); await fi.setInputFiles(x.exported); await sleep(800);
    const ex = x.page.locator('[id$="importDocument.execute"]').first(); if (await ex.count()) await ex.click().catch(() => {}); await sleep(6000);
    await x.solved(200000); await x.toSummary(); const c1 = await x.counts();
    const edit = await x.editField(a.rows.node, "X", { keys: ["node.x"], value: 6.25, group: "Nodes" }); const con = x.consoleSince(mark);
    return { pass: JSON.stringify(c0) === JSON.stringify(c1) && edit.pass && con.fatal === 0, observed: `counts ${JSON.stringify(c0)} -> ${JSON.stringify(c1)}; edit: ${edit.observed}; fatal=${con.fatal} ${con.fatalFirst.join("|")}` };
  }, { timeout: 420000 });

  add("clip.copy-paste", "clipboard", "Select element, Copy, Paste", "Element count +1", { 2: [107], 3: [116] }, async x => {
    await x.selectRow(a.rows.element, { group: "Elements" }); const e0 = await x.groupCount("Elements");
    const c = await x.runAction("copy", {}); const mark = x.mark();
    const p = await x.runAction("paste", {}, { verify: { pre: async () => 0, post: async () => (await x.groupCount("Elements")) === e0 + 1 }, timeout: 40000 });
    return { pass: c.pass && p.pass, observed: `copy: ${c.observed} | paste: ${p.observed}` };
  }, { timeout: 150000 });
  add("clip.cut-paste", "clipboard", "Select element, Cut, Paste", "Element removed on Cut, returns on Paste", { 2: [107], 3: [117] }, async x => {
    await x.selectRow(a.rows.element2, { group: "Elements" }); const e0 = await x.groupCount("Elements");
    const c = await x.runAction("cut", {}, { verify: { pre: async () => 0, post: async () => (await x.groupCount("Elements")) === e0 - 1 }, timeout: 40000 });
    const p = await x.runAction("paste", {}, { verify: { pre: async () => 0, post: async () => (await x.groupCount("Elements")) === e0 }, timeout: 40000 });
    return { pass: c.pass && p.pass, observed: `cut: ${c.observed} | paste: ${p.observed}` };
  }, { timeout: 200000 });

  return K.filter(k => !k.skipIf).filter(k => !k.skip);
}
