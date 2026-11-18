/** Round 2 course final driver: wraps the crawler probe, hides inert layered panes (pages behind the glass), records rows in sessionStorage. */
import * as C from "./round2-course-crawl.js";
const DIR = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI";
export * from "./round2-course-crawl.js";
export const sleep = C.sleep;
const KEY = "__finalRows";
export const rows = {
  get all() { try { return JSON.parse(sessionStorage.getItem(KEY) || "[]"); } catch { return []; } },
  push(x) { const r = this.all; r.push(x); sessionStorage.setItem(KEY, JSON.stringify(r)); },
  clear() { sessionStorage.removeItem(KEY); },
};
export const pickRoot = () => {
  const dialog = [...document.querySelectorAll("[data-course-dialog],[role=dialog],[role=alertdialog]")].filter(e => e.getBoundingClientRect().width > 0)[0];
  if (dialog) return dialog;
  const pages = [...document.querySelectorAll("[data-page]")].filter(e => {
    if (e.closest("[inert]")) return false;
    const r = e.getBoundingClientRect();
    return r.left >= -1 && r.left < innerWidth && r.top < innerHeight && r.width > 0;
  });
  if (pages.length) return pages[pages.length - 1];
  return document.querySelector("[data-layered-overview]") || document.querySelector("#course-main") || document.body;
};
export async function probeNow({ mark = false, root } = {}) {
  const p = await import("/@fs" + encodeURI(DIR + "/border-gap-probe.js") + "?t=" + Date.now());
  const el = root || window.__rootOverride || pickRoot();
  const hidden = [...document.querySelectorAll("[data-layered-pane][inert]")].filter(n => !el.contains(n) || el === n || n.contains(el) === false);
  const keep = hidden.filter(n => !n.contains(el));
  const prev = keep.map(n => n.style.visibility);
  keep.forEach(n => (n.style.visibility = "hidden"));
  let r;
  try { r = p.probe({ mark, root: el }); } finally { keep.forEach((n, i) => (n.style.visibility = prev[i])); }
  const stack = s => /(course-card-stack|slot=navbar).*svg:[hv]$/.test(s.trim()) || /^footer\.course-footer/.test(s);
  const inside = [], mixed = [];
  for (const v of r.violations) {
    const o = v.distance + " " + v.a.slice(0, 120) + " || " + v.b.slice(0, 120);
    (stack(v.a) && stack(v.b) ? mixed : inside).push(o);
  }
  return { root: el.dataset?.page || el.id || el.tagName, count: r.count, inside, mixed, art: r.art.length, edges: r.edges };
}
export const scrollers = () => [...document.querySelectorAll("*")].filter(e => {
  if (e.closest("[inert]")) return false;
  const s = getComputedStyle(e);
  return /(auto|scroll)/.test(s.overflowY) && e.scrollHeight > e.clientHeight + 8 && e.clientHeight > 100 && e.getBoundingClientRect().width > 0;
});
export async function rec(name, { settle = true, sweep = true } = {}) {
  await Promise.race([new Promise(r => requestAnimationFrame(() => requestAnimationFrame(r))), sleep(600)]);
  await sleep(400);
  const rest = new Set(), mid = new Set();
  let mixed = 0, edges = 0, steps = 0;
  const one = async (isRest) => {
    let r = await probeNow();
    for (let k = 0; k < 4 && settle && r.inside.length; k++) { await sleep(1000); r = await probeNow(); }
    edges = Math.max(edges, r.edges); steps++; mixed = Math.max(mixed, r.mixed.length);
    for (const o of r.inside) (isRest ? rest : mid).add(o);
    return r;
  };
  const sc = sweep ? scrollers() : [];
  const first = await one(true);
  for (const el of sc) {
    const orig = el.scrollTop;
    const max = el.scrollHeight - el.clientHeight;
    const step = Math.max(120, el.clientHeight * 0.7);
    for (let y = step; y < max + step; y += step) {
      const end = y >= max;
      el.scrollTop = Math.min(y, max);
      await sleep(350);
      await one(end);
      if (end) break;
    }
    el.scrollTop = orig;
  }
  const midOnly = [...mid].filter(o => ![...rest].some(q => q.split(" ").slice(1).join(" ") === o.split(" ").slice(1).join(" ")));
  const inside = [...rest];
  const line = `${name} [${innerWidth}] ${first.root} => inside=${inside.length} midscroll=${midOnly.length} mixed=${mixed} edges<=${edges} scrolls=${steps}` + (inside.length ? "\n    " + inside.slice(0, 10).join("\n    ") : "") + (midOnly.length ? "\n    [mid] " + midOnly.slice(0, 4).join("\n    [mid] ") : "");
  rows.push(line);
  return line;
}
export const go = async (hash, wait = 1800) => {
  if (document.querySelector(".course-steps")) { C.click(/^Course overview/); await sleep(1200); }
  location.hash = hash; await sleep(wait);
};

export async function runSteps(name, { doFill = true, from = 0 } = {}) {
  const n = document.querySelectorAll(".course-steps > li button").length;
  for (let i = from; i < n; i++) {
    const b = document.querySelectorAll(".course-steps > li button")[i];
    if (!b) break;
    b.click();
    await sleep(500);
    await rec(`${name} step${i + 1}`);
    if (doFill) {
      const c = await C.fill(), g = await C.typeGuesses();
      await sleep(500);
      await rec(`${name} step${i + 1} filled(${c}/${g})`);
    }
  }
  return n;
}
export async function openQuiz(id, level) {
  if (document.querySelector(".course-steps")) { C.click(/^Course overview/); await sleep(1200); }
  location.hash = "#" + id; await sleep(2200);
  const radio = [...document.querySelectorAll(`input.course-radio[value="${level}"]`)].filter(r => !r.closest("[inert]") && r.getBoundingClientRect().width > 0)[0];
  if (radio) { radio.click(); await sleep(300); }
  await rec(`${id}/${level} quiz-page`);
  const Lv = level[0].toUpperCase() + level.slice(1);
  C.click(new RegExp(`\\(${Lv}\\)`)) || C.click(/^(Start|Resume|Continue|Again)/);
  await sleep(2000);
  if (document.querySelector("[data-course-dialog],[role=alertdialog],[role=dialog]")) {
    await rec(`${id}/${level} dialog`);
    C.click(/^Discard and start|^Start/);
    await sleep(2000);
  }
  return !!document.querySelector(".course-steps");
}
export async function quiz2(id, level, opts) {
  const ok = await openQuiz(id, level);
  if (!ok) { rows.push(`${id}/${level}: no run opened`); return; }
  await runSteps(`${id}/${level} run`, opts);
}
