import { probe } from "./border-gap-probe.js";
const sleep = t => new Promise(r => setTimeout(r, t));
const fmt = p => [p.edges, p.count, p.violations.map(x => x.distance + " " + x.a + " || " + x.b).slice(0, 3)];
export const mobile = async () => {
  const out = [];
  const body = document.body;
  const total = body.scrollHeight;
  for (let y = 0; y < total; y += 600) {
    body.scrollTop = y; await sleep(250);
    const p = probe();
    if (p.count || y === 0) out.push([`scroll ${y}`, ...fmt(p)]);
  }
  const ds = [...document.querySelectorAll(".presentation-interactive-disposition")].filter(e => !/ghost|morph-/.test(e.className));
  const seen = new Set();
  window.__mres = out;
  for (const d of ds) {
    const sec = d.closest("section"); const k = (d.className.match(/--kind-(\w+)/) || [])[1]; const key = k;
    const r0 = d.getBoundingClientRect(); if (r0.width < 40 || r0.height < 40) continue;
    d.scrollIntoView({ block: "center" }); await sleep(300);
    const r = d.getBoundingClientRect(); const x = r.left + r.width / 2, y = r.top + Math.min(r.height / 2, 200);
    for (const t of ["pointerdown", "pointerup", "click"]) d.dispatchEvent(new PointerEvent(t, { bubbles: true, clientX: x, clientY: y, button: 0, pointerId: 1, isPrimary: true, pointerType: "touch" }));
    await sleep(500);
    const p = probe();
    const rows = [`sel:${k}:${document.querySelectorAll(".presentation-interactive-disposition--selected").length}`, ...fmt(p)];
    const en = document.querySelector(".presentation-interaction-enlarge");
    if (en) { en.click(); await sleep(900); rows.push(`enl`, ...fmt(probe())); document.querySelector(".presentation-interaction-enlarge")?.click(); await sleep(400); }
    document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", keyCode: 27, bubbles: true })); await sleep(200);
    if (rows[2] || rows[5]) out.push(rows); else if (!seen.has(key)) { seen.add(key); out.push(rows); }
  }
  window.__mdone = 1;
  return out;
};
