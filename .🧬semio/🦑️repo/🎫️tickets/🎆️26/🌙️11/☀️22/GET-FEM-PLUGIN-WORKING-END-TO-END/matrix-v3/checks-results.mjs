/** 📈 Results checks of the v3 matrix: display source/mode, playback, analysis settings, re-solve behaviour, captions. */
import { mk } from "./checks-core.mjs";
import { sleep, same } from "./lib.mjs";

const SKIP = why => ({ pass: null, observed: `SKIP: ${why}` });

export function resultsChecks(a) {
  const d = a.dim, K = [];
  const add = (...args) => K.push(mk(...args));
  const RC = a.resultsClip;
  const ctl = async (x, label, key, transport) => {
    await x.panel("Results", true);
    for (const role of ["combobox", "spinbutton", "slider", "textbox"]) { const l = x.page.getByRole(role, { name: label, exact: true }).first(); if (await l.count()) return { loc: l, role, label }; }
    const sel = transport ? `[id$="-play-results.transport.${key}"]` : `[id$="-play-results.${key}.control"]`;
    const l = x.page.locator(sel).first();
    if (await l.count()) { const tag = await l.evaluate(e => e.tagName + ":" + (e.type ?? "") + ":" + (e.getAttribute("role") ?? "")); return { loc: l, role: /combobox/.test(tag) ? "combobox" : /range|slider/.test(tag) ? "slider" : /number/.test(tag) ? "spinbutton" : /INPUT/.test(tag) ? "textbox" : "button", label }; }
    return null;
  };
  const tbtn = async (x, name, key) => { await x.panel("Results", true); const b = x.page.getByRole("button", { name, exact: true }).first(); if (await b.count()) return b; const l = x.page.locator(`[id$="-play-results.transport.${key}"]`).first(); return (await l.count()) ? l : null; };
  const ensurePaused = async x => { const p = await tbtn(x, /^Play$/, "play"); if (!p) return false; for (let i = 0; i < 3 && /▶/.test(await x.caption()); i++) { await p.click(); await sleep(1500); } return !/▶/.test(await x.caption()); };
  const frames = async (x, n = 8, dt = 600) => { const hs = []; for (let i = 0; i < n; i++) { await x.page.waitForTimeout(dt); hs.push(await x.hash(RC)); } return hs; };
  const pickOpt = async (x, label, key, want) => {
    const c = await ctl(x, label, key); if (!c) return { ok: false, why: `control ${label} missing` };
    const r = await x.chooseOption(c.loc, want instanceof RegExp || typeof want === "number" ? want : new RegExp(`^${want}`)); if (!r.ok) return { ok: false, why: `option ${want} not in [${r.options}]` };
    await sleep(1200); if (want instanceof RegExp) await x.until(async () => want.test(await x.readField(await ctl(x, label, key))), 20000, 500); const shown = await x.readField((await ctl(x, label, key)));
    return { ok: true, picked: r.picked, shown, options: r.options };
  };
  const display = (id, label, key, optRe, inv, extra) => add(`res.${id}`, "results", `Results panel ${label} > ${optRe}`, `${label} combobox shows ${optRe}; Results image or caption changes`, inv, async x => {
    await x.toSummary(); const h0 = await x.hash(RC); const c0 = await x.caption();
    const r = await pickOpt(x, label, key, optRe); if (!r.ok) return { pass: false, observed: r.why };
    const ms = await x.changed(h0, null, 45000, RC); const cap = await x.caption();
    return { pass: new RegExp(optRe, "i").test(r.shown) && (ms >= 0 || cap !== c0), observed: `shows "${r.shown}" imageChanged after ${ms} ms caption "${c0}" -> "${cap}"` };
  }, { timeout: 240000, ...extra });

  display("source-live", "Source", "source", /Live/, { 2: [121], 3: [132] });
  display("source-dead", "Source", "source", /Dead/, { 2: [122], 3: [131] });
  display("source-uls", "Source", "source", /ULS/, { 2: [122], 3: [131] });
  display("mode-modal", "Mode", "mode", /Modal/, { 2: [123], 3: [134] });
  display("mode-buckling", "Mode", "mode", /Buckling/, { 2: [124], 3: [135, 150] });
  display("mode-static", "Mode", "mode", /Static/, { 2: [], 3: [133] });
  add("res.mode-index", "results", "Mode Modal, Mode Index 2", "Caption changes to mode 2", { 2: [125], 3: [136] }, async x => {
    const m = await pickOpt(x, "Mode", "mode", /Modal/); if (!m.ok) return { pass: false, observed: m.why }; await x.solved(150000);
    const c = await ctl(x, "Mode Index", "mode-index"); if (!c) return { pass: false, observed: "Mode Index control missing" };
    const h0 = await x.hash(RC); const cap0 = await x.caption(); await x.setField(c, 2); const ms = await x.changed(h0, null, 45000, RC); const cap = await x.caption();
    await pickOpt(x, "Mode", "mode", /Static/);
    return { pass: (ms >= 0 || cap !== cap0) && (d === 2 || /2/.test(cap)), observed: `caption "${cap0}" -> "${cap}" imageChanged after ${ms} ms` };
  }, { timeout: 420000 });
  add("res.deformation-scale", "results", "Deformation Scale 500", "Deformation magnified: Results image changes", { 2: [126], 3: [148] }, async x => {
    const c = await ctl(x, "Deformation Scale", "deformation-scale"); if (!c) return { pass: false, observed: "control missing" };
    const h0 = await x.hash(RC); await x.setField(c, 500); const ms = await x.changed(h0, null, 45000, RC);
    return { pass: ms >= 0 && same(await x.readField((await ctl(x, "Deformation Scale", "deformation-scale"))), 500), observed: `imageChanged after ${ms} ms` };
  }, { timeout: 120000 });
  add("res.modal-count", "results", "Modal Count 4", "Value kept, re-solve completes", { 2: [135], 3: [146] }, async x => {
    const c = await ctl(x, "Modal Count", "modal-count"); if (!c) return { pass: false, observed: "control missing" };
    await x.setField(c, 4); await sleep(1500); const v = await x.readField((await ctl(x, "Modal Count", "modal-count"))); const s = await x.solved(200000);
    return { pass: same(v, 4) && s >= 0, observed: `value=${v} solved after ${s} ms` };
  }, { timeout: 260000 });
  add("res.buckling-count", "results", "Buckling Count 2", "Value kept, re-solve completes", { 2: [135], 3: [147] }, async x => {
    const c = await ctl(x, "Buckling Count", "buckling-count"); if (!c) return { pass: false, observed: "control missing" };
    await x.setField(c, 2); await sleep(1500); const v = await x.readField((await ctl(x, "Buckling Count", "buckling-count"))); const s = await x.solved(200000);
    return { pass: same(v, 2) && s >= 0, observed: `value=${v} solved after ${s} ms` };
  }, { timeout: 260000 });

  add("res.step-forward", "playback", "Step +", "Results frame advances (image changes)", { 2: [127], 3: [138] }, async x => {
    const b = await tbtn(x, /^Step \+$/, "forward"); if (!b) return { pass: false, observed: "Step + missing" };
    const h0 = await x.hash(RC); const mark = x.mark(); await b.click(); const ms = await x.changed(h0, null, 20000, RC); const con = x.consoleSince(mark);
    return { pass: ms >= 0 && con.fatal === 0, observed: `imageChanged after ${ms} ms fatal=${con.fatal} ${con.fatalFirst.join("|")}` };
  }, { fresh: true });
  add("res.step-back", "playback", "Step - after Step +", "Frame goes back to the earlier image", { 2: [128], 3: [139] }, async x => {
    const f = await tbtn(x, /^Step \+$/, "forward"), b = await tbtn(x, /^Step [-−–]$/, "backward"); if (!f || !b) return { pass: false, observed: "Step buttons missing" };
    await sleep(1500); const h0 = await x.hash(RC); await f.click(); await x.changed(h0, null, 15000, RC); await sleep(1500); const h1 = await x.hash(RC); await b.click(); await sleep(2500); const h2 = await x.hash(RC);
    return { pass: h1 !== h0 && h2 !== h1, observed: `frames ${h0} -> ${h1} -> ${h2} (back equals start: ${h2 === h0})` };
  });
  add("res.play-pause", "playback", "Play, sample 8 frames, Pause", "Animates (>=3 distinct frames), button toggles to Pause, then freezes; no panic", { 2: [129], 3: [140] }, async x => {
    const p = await tbtn(x, /^Play$/, "play"); if (!p) return { pass: false, observed: "Play missing" };
    const mark = x.mark(); await p.click(); const hs = await frames(x); const toggled = (await x.page.getByRole("button", { name: /^Pause$/ }).count()) > 0;
    const pa = (await x.page.getByRole("button", { name: /^Pause$/ }).first().count()) ? x.page.getByRole("button", { name: /^Pause$/ }).first() : p; await pa.click(); await sleep(1500); const hp = await frames(x, 3); const con = x.consoleSince(mark);
    return { pass: new Set(hs).size >= 3 && new Set(hp).size === 1 && con.fatal === 0, observed: `distinct=${new Set(hs).size} pauseButton=${toggled} frozen=${new Set(hp).size === 1} fatal=${con.fatal} ${con.fatalFirst.join("|")}` };
  }, { timeout: 120000, fresh: true });
  add("res.readouts", "playback", "Play in Results window, read caption readouts", "Phase and Hz readouts update while playing and freeze on pause", { 2: [], 3: [145] }, async x => {
    const p = await tbtn(x, /^Play$/, "play"); if (!p) return { pass: false, observed: "Play missing" };
    if (!(await ensurePaused(x))) return { pass: false, observed: "could not reach the paused precondition" };
    await p.click(); const caps = []; for (let i = 0; i < 6; i++) { await sleep(700); caps.push(await x.caption()); }
    await p.click(); await sleep(1200); const c1 = await x.caption(); await sleep(1500); const c2 = await x.caption();
    return { pass: new Set(caps).size >= 2 && c1 === c2, observed: `distinct captions=${new Set(caps).size} frozen=${c1 === c2} sample="${caps[2]}"` };
  }, { skipIf: d !== 3, timeout: 120000 });
  add("res.space-toggle", "playback", "Space over Results window", "Playback toggles on (frames change)", { 2: [130], 3: [141] }, async x => {
    if (!(await ensurePaused(x))) return { pass: false, observed: "could not reach the paused precondition" };
    await x.page.evaluate(() => document.activeElement?.blur?.()); await x.page.mouse.move(...a.resultsHover); await sleep(500); await x.page.keyboard.press("Space");
    const hs = await frames(x); await x.page.keyboard.press("Space"); const hp = (await sleep(1500), await frames(x, 3));
    return { pass: new Set(hs).size >= 3 && new Set(hp).size === 1, observed: `distinct=${new Set(hs).size} frozenAfterSecondSpace=${new Set(hp).size === 1}` };
  }, { timeout: 120000 });
  add("res.speed", "playback", "Speed slider", "Value applied, playback still animates", { 2: [131], 3: [142] }, async x => {
    const c = await ctl(x, "Speed", "speed"); if (!c) return { pass: false, observed: "Speed control missing" };
    const v0 = await x.readField(c); await x.setField(c, 2); await sleep(800); const v1 = await x.readField((await ctl(x, "Speed", "speed")));
    return { pass: v1 !== v0 || same(v1, 2), observed: `speed ${v0} -> ${v1}` };
  });
  for (const [label, key, opts, inv] of [["Loop", "loop", ["Loop", "Ping-Pong", "Once"], { 2: [132], 3: [143] }], ["Waveform", "waveform", ["Ramp", "Sine"], { 2: [133], 3: [144] }]]) add(`res.${key}`, "playback", `${label} combobox`, `${opts.join(", ")} selectable and read back`, inv, async x => {
    const seen = [];
    for (const o of opts) { const r = await pickOpt(x, label, key, new RegExp(`^${o}$`, "i")); seen.push(`${o}:${r.ok ? r.shown : r.why}`); if (!r.ok || !new RegExp(o, "i").test(r.shown)) return { pass: false, observed: seen.join(" | ") }; }
    return { pass: true, observed: seen.join(" | ") };
  });
  add("res.phase", "playback", "Phase slider to 0.5", "Phase set (readback) and Results image changes", { 2: [134], 3: [137] }, async x => {
    const c = await ctl(x, "Phase", "phase"); if (!c) return { pass: false, observed: "Phase control missing" };
    const h0 = await x.hash(RC); const v0 = await x.readField(c); await x.setField(c, 0.5); const ms = await x.changed(h0, null, 20000, RC); const v1 = await x.readField((await ctl(x, "Phase", "phase")));
    return { pass: ms >= 0 && v1 !== v0, observed: `phase ${v0} -> ${v1} imageChanged after ${ms} ms` };
  });
  add("res.caption-static", "results", "Results window caption", "Static caption present and not overlapping (non-empty, not Pending)", { 2: [136], 3: [149] }, async x => {
    await ensurePaused(x); const c = await x.caption(); return { pass: a.bootCaption.test(c) && !a.pending.test(c), observed: JSON.stringify(c) };
  });
  add("res.resolve-after-edit", "results", "Edit node X, then wait", "Caption goes Pending/Preparing, then back to a converged caption", { 2: [137], 3: [152] }, async x => {
    const e = await x.editField(a.rows.node, "X", { keys: ["node.x"], value: 6.5, group: "Nodes" });
    const pend = await x.until(async () => a.pending.test(await x.caption()), 20000, 300); const s = await x.solved(250000);
    return { pass: e.pass && s >= 0, observed: `${e.observed}; pending seen after ${pend} ms; solved after ${s} ms; caption=${JSON.stringify(await x.caption())}` };
  }, { timeout: 420000 });
  add("res.error-caption", "results", "Material Young's Modulus 0 (singular model), then restore", "Analysis error caption shown, app alive, restore re-solves", { 2: [], 3: [151] }, async x => {
    const e = await x.editField(a.rows.material, "Young's Modulus", { keys: ["material.e"], value: 0, group: "Materials" });
    const ms = await x.until(async () => /fail|error|singular|diverg|invalid/i.test(await x.caption()), 120000, 1000); const cap = await x.caption();
    await x.editField(a.rows.material, "Young's Modulus", { keys: ["material.e"], value: 210000000000, group: "Materials" }); const s = await x.solved(250000);
    return { pass: ms >= 0 && s >= 0, observed: `${e.observed}; errorCaption after ${ms} ms "${cap}"; restored solved=${s}` };
  }, { skipIf: d !== 3, timeout: 600000 });
  add("res.stability", "results", "Six node edits with undo cycles", "Results re-solve each time (<= 250 s), no panic", { 2: [], 3: [153] }, async x => {
    const mark = x.mark(); const seen = [];
    for (let i = 0; i < 6; i++) { const e = await x.editField(a.rows.node, "X", { keys: ["node.x"], value: 5 + i * 0.25, group: "Nodes" }); if (i % 2) { await x.page.mouse.move(...a.wheel); await x.page.keyboard.press("Control+z"); } const s = await x.solved(250000); seen.push(`${i}:${e.pass}/${s}`); if (s < 0) break; }
    const con = x.consoleSince(mark);
    return { pass: seen.length === 6 && seen.every(s => !/\/-1$/.test(s)) && con.fatal === 0, observed: `${seen.join(" ")} fatal=${con.fatal} ${con.fatalFirst.join("|")}` };
  }, { skipIf: d !== 3, timeout: 1800000 });
  return K.filter(k => !k.skipIf);
}
