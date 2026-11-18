import { probe } from "./border-gap-probe.js";
const sleep = t => new Promise(r => setTimeout(r, t));
const key = k => document.dispatchEvent(new KeyboardEvent("keydown", { key: k === " " ? " " : k, keyCode: k === " " ? 32 : 27, which: k === " " ? 32 : 27, bubbles: true }));
const where = () => decodeURIComponent(location.hash).replace(/^#\/?/, "").replace(/&?kapitel=[^&]*&sequenz=[^&]*&gedanke=[^&]*/, "").slice(0, 60);
const fmt = p => [p.edges, p.count, p.violations.map(x => x.distance + " " + x.a + " || " + x.b).slice(0, 3)];
export const sweep = async (count = 40) => {
  window.__sres = []; window.__sdone = 0; const seenAt = new Set();
  for (let step = 0; step < count; step++) {
    await sleep(1300);
    const at = where();
    if (seenAt.has(at)) break;
    seenAt.add(at);
    const s = [...document.querySelectorAll(".slides section.present")].pop();
    const rows = [["rest", ...fmt(probe())]];
    const ds = [...s.querySelectorAll(".presentation-interactive-disposition")].filter(e => { const r = e.getBoundingClientRect(); return r.width > 40 && r.height > 40 && !/ghost|morph-/.test(e.className); });
    const seen = new Set();
    for (const d of ds) {
      const k = (d.className.match(/--kind-(\w+)/) || [])[1];
      if (seen.has(k)) continue;
      seen.add(k);
      const r = d.getBoundingClientRect(); const x = r.left + r.width / 2, y = r.top + r.height / 2;
      for (const t of ["pointerdown", "pointerup", "click"]) d.dispatchEvent(new PointerEvent(t, { bubbles: true, clientX: x, clientY: y, button: 0, pointerId: 1, isPrimary: true, pointerType: "mouse" }));
      await sleep(500);
      const sel = document.querySelectorAll(".presentation-interactive-disposition--selected").length;
      rows.push([`sel:${k}:${sel}`, ...fmt(probe())]);
      const en = document.querySelector(".presentation-interaction-enlarge");
      if (en) { en.click(); await sleep(900); rows.push([`enl:${k}`, ...fmt(probe())]); document.querySelector(".presentation-interaction-enlarge")?.click(); await sleep(500); }
      key("Escape"); await sleep(250);
    }
    window.__sres.push([at, rows]);
    key(" ");
  }
  window.__sdone = 1;
};
export const report = () => JSON.stringify({ done: window.__sdone, n: window.__sres.length, r: window.__sres.map(([at, rows]) => [at, rows.map(x => x.slice(0, 3).join(":") + (x[3].length ? " !! " + x[3].join(" ## ") : "")).join(" ; ")]) });
