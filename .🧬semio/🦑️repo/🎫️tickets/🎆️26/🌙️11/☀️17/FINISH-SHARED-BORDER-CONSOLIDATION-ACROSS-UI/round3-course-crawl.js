/** Round 3 course crawler: base probe (all pairs, mixed included) + supplemental near-double probe, plus focus/hover/state passes. */
import * as C from "./round2-course-crawl.js";
const DIR = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI";
export const { sleep, click, fill, typeGuesses } = C;
let P3;
const KEY = "__r3rows";
export const rows = { get all() { try { return JSON.parse(sessionStorage.getItem(KEY) || "[]"); } catch { return []; } }, push(x) { const r = this.all; r.push(x); sessionStorage.setItem(KEY, JSON.stringify(r)); }, clear() { sessionStorage.removeItem(KEY); } };
export async function measure(rootEl) {
  P3 ??= await import("/@fs" + encodeURI(DIR + "/round3-course-probe.js") + "?t=" + Date.now());
  const base = await import("/@fs" + encodeURI(DIR + "/border-gap-probe.js") + "?t=" + Date.now());
  const dialog = [...document.querySelectorAll("dialog[open],[data-course-dialog],[role=dialog]")].find(d => d.getBoundingClientRect().width > 0 && !d.closest("[inert]"));
  const el = rootEl || dialog || document.body;
  const off = [...document.querySelectorAll("[data-page]")].filter(e => { const b = e.getBoundingClientRect(); return b.left >= innerWidth - 1 || b.right <= 1 || b.width === 0 || e.closest("[inert]"); });
  off.forEach(e => e.setAttribute("data-border-probe-art", ""));
  let r, n;
  try { r = base.probe({ root: el }); n = P3.probe3({ root: el }); } finally { off.forEach(e => e.removeAttribute("data-border-probe-art")); }
  const f = x => x.replace(/\s+/g, " ").slice(0, 90);
  return {
    base: r.violations.map(v => `${v.distance} ${f(v.a)} || ${f(v.b)}`).filter(x => !(/css:bottom \|\| div\[slot=shared-border-row\].*css:bot/.test(x))),
    near: n.filter(v => !(v.d === 0 && /^svg:[hv]\+svg:[hv]$/.test(v.kinds))).map(v => `${v.d} ${v.kinds} ${f(v.a)} || ${f(v.b)}`),
    overlaps: r.overlaps.map(o => `${o.area} ${f(o.a)} || ${f(o.b)}`),
  };
}
export async function rec(name, rootEl) {
  await Promise.race([new Promise(r => requestAnimationFrame(() => requestAnimationFrame(r))), sleep(600)]);
  await sleep(350);
  let m = await measure(rootEl);
  for (let k = 0; k < 4 && (m.base.length || m.near.length); k++) { await sleep(1200); m = await measure(rootEl); }
  const t = m.base.length + m.near.length;
  rows.push(`${name} => base=${m.base.length} near=${m.near.length} ovl=${m.overlaps.length}` + (t ? "\n    " + [...m.base.slice(0, 5), ...m.near.slice(0, 5).map(x => "NEAR " + x), ...m.overlaps.slice(0, 3).map(x => "OVL " + x)].join("\n    ") : ""));
  return m;
}
export async function stateSweep(name, rootEl) {
  const el = rootEl || C.root();
  const targets = [...el.querySelectorAll("button,a[href],input,select,textarea,summary,[tabindex='0']")].filter(e => !e.closest("[inert]") && e.getBoundingClientRect().width > 0 && e.getBoundingClientRect().top < innerHeight * 2).slice(0, 40);
  let bad = 0;
  const out = [];
  for (const t of targets) {
    try { t.focus({ focusVisible: true, preventScroll: true }); } catch { continue; }
    await sleep(60);
    const n = P3.probe3({ root: el }).filter(v => !(v.d === 0 && /^svg:[hv]\+svg:[hv]$/.test(v.kinds))).filter(v => v.els.includes(t) || v.els.some(e => t.contains(e) || e.contains(t)));
    if (n.length) { bad++; out.push(`${(t.textContent || t.tagName).trim().slice(0, 24)}: ` + n.slice(0, 2).map(v => `${v.d} ${v.kinds} ${v.a.slice(0, 50)}||${v.b.slice(0, 50)}`).join(" ; ")); }
    t.blur();
  }
  rows.push(`${name} focus-sweep(${targets.length}) bad=${bad}` + (bad ? "\n    " + out.slice(0, 6).join("\n    ") : ""));
  return bad;
}
export async function go(hash, wait = 1800) {
  if (document.querySelector(".course-steps")) { click(/^Course overview/); await sleep(1200); }
  if (location.hash !== "#" + hash) { location.hash = "#" + hash; await sleep(wait); }
  await sleep(800);
}
export async function view(hash, name, id) {
  await go(hash);
  for (let k = 0; k < 20; k++) { const e = document.querySelector(`[data-page="${id || hash}"]`); if (e && !e.closest("[inert]") && Math.abs(e.getBoundingClientRect().left) < 2) break; await sleep(400); }
  await rec(name || `#${hash}`);
  await stateSweep(name || `#${hash}`, document.querySelector(`[data-page="${id || hash}"]`) || document.body);
}

export async function runCrawl(id, level, { doFill = true } = {}) {
  const Lv = level[0].toUpperCase() + level.slice(1);
  await go(id, 2200);
  const radio = document.querySelector(`input.course-radio[value="${level}"]`);
  if (radio) { radio.click(); await sleep(300); }
  await rec(`${id}/${level} quiz-page`);
  click(new RegExp(`\\(${Lv}\\)`)) || click(/^(Start|Resume|Continue|Again)/);
  await sleep(1800);
  if (document.querySelector("[data-course-dialog]")) { await rec(`${id}/${level} dialog`); click(/^Discard and start|^Start/); await sleep(2000); }
  if (!document.querySelector(".course-steps")) { rows.push(`${id}/${level}: no run opened`); return; }
  const n = document.querySelectorAll(".course-steps > li button").length;
  for (let i = 0; i < n; i++) {
    document.querySelectorAll(".course-steps > li button")[i]?.click();
    await sleep(700);
    await rec(`${id}/${level} step${i + 1}`);
    await stateSweep(`${id}/${level} step${i + 1}`);
    if (doFill) { const c = await fill(); const g = await typeGuesses(); await sleep(500); await rec(`${id}/${level} step${i + 1} filled(${c}/${g})`); }
  }
}
