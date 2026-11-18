/** Round 3 supplemental probe: near-doubles the base probe skips (svg vs css coincident, same-owner svg pairs, tiny spans, box-shadow/outline lines). */
import { probe as base } from "./border-gap-probe.js";
const tone = c => !(c === "transparent" || /rgba\([^)]*,\s*0\)$/.test(c));
export function probe3({ root = document.body, near = 3.2, overlap = 4 } = {}) {
  const vis = el => {
    if (el.closest("[inert]") || el.closest("details:not([open]) > :not(summary)")) return false;
    for (let e = el; e; e = e.parentElement) { const s = getComputedStyle(e); if (s.display === "none" || s.visibility === "hidden" || +s.opacity === 0) return false; }
    const r = el.getBoundingClientRect();
    return r.width > 0 && r.height > 0 && r.bottom > 0 && r.right > 0 && r.top < innerHeight * 3 && r.left < innerWidth;
  };
  const nm = el => `${el.tagName.toLowerCase()}${el.getAttribute?.("data-slot") ? `[${el.getAttribute("data-slot")}]` : ""}.${(typeof el.className === "string" ? el.className : "").split(/\s+/).filter(c => c && !c.includes(":")).slice(0, 3).join(".")} «${(el.textContent || "").trim().replace(/\s+/g, " ").slice(0, 24)}»`;
  const edges = [];
  const add = (e) => edges.push(e);
  for (const el of root.querySelectorAll("*")) {
    if (el.closest("[data-border-probe-art]") || el instanceof SVGElement || !vis(el)) continue;
    const s = getComputedStyle(el), r = el.getBoundingClientRect();
    for (const k of ["Top", "Right", "Bottom", "Left"]) {
      const w = parseFloat(s[`border${k}Width`]);
      if (w > 0 && !["none", "hidden"].includes(s[`border${k}Style`]) && tone(s[`border${k}Color`])) {
        const h = k === "Top" || k === "Bottom";
        add({ el, src: "border", side: k, h, at: k === "Top" ? r.top + w / 2 : k === "Bottom" ? r.bottom - w / 2 : k === "Left" ? r.left + w / 2 : r.right - w / 2, from: h ? r.left : r.top, to: h ? r.right : r.bottom });
      }
    }
    if (s.outlineStyle !== "none" && parseFloat(s.outlineWidth) > 0 && tone(s.outlineColor)) {
      const o = parseFloat(s.outlineWidth), off = parseFloat(s.outlineOffset) || 0;
      for (const [k, h, at, from, to] of [["Top", true, r.top - off - o / 2, r.left, r.right], ["Bottom", true, r.bottom + off + o / 2, r.left, r.right], ["Left", false, r.left - off - o / 2, r.top, r.bottom], ["Right", false, r.right + off + o / 2, r.top, r.bottom]]) add({ el, src: "outline", side: k, h, at, from, to });
    }
    if (s.boxShadow !== "none") {
      for (const part of s.boxShadow.split(/,(?![^(]*\))/)) {
        const m = part.trim().match(/^(rgba?\([^)]*\)|#\w+)\s+(-?[\d.]+)px\s+(-?[\d.]+)px\s+(-?[\d.]+)px\s*(-?[\d.]+)?px?\s*(inset)?/) || part.trim().match(/^(inset\s+)?(rgba?\([^)]*\))\s+(-?[\d.]+)px\s+(-?[\d.]+)px\s+(-?[\d.]+)px\s*(-?[\d.]+)?px?/);
        if (!m) continue;
        const col = m[1] && m[1].startsWith("rgb") ? m[1] : m[2];
        const nums = part.match(/-?[\d.]+px/g)?.map(parseFloat) || [];
        const [x, y, blur, spread = 0] = nums;
        const inset = /inset/.test(part);
        if (!tone(col) || blur > 0.5 || spread <= 0 || spread > 2) continue;
        const sp = spread;
        const sides = [];
        if (x === 0 && y === 0) sides.push("Top", "Right", "Bottom", "Left");
        else { if (x > 0) sides.push(inset ? "Left" : "Right"); if (x < 0) sides.push(inset ? "Right" : "Left"); if (y > 0) sides.push(inset ? "Top" : "Bottom"); if (y < 0) sides.push(inset ? "Bottom" : "Top"); }
        for (const k of sides) {
          const h = k === "Top" || k === "Bottom";
          const out = inset ? -sp / 2 : sp / 2;
          const at = k === "Top" ? r.top - out : k === "Bottom" ? r.bottom + out : k === "Left" ? r.left - out : r.right + out;
          add({ el, src: "shadow" + (inset ? "-inset" : ""), side: k, h, at, from: h ? r.left : r.top, to: h ? r.right : r.bottom });
        }
      }
    }
  }
  for (const path of root.querySelectorAll("path[data-shared-border-segment]")) {
    const svg = path.ownerSVGElement;
    if (!svg || !vis(svg) || svg.closest("[data-border-probe-art]")) continue;
    const r = svg.getBoundingClientRect(), vb = svg.viewBox.baseVal;
    const sx = vb && vb.width ? r.width / vb.width : 1, sy = vb && vb.height ? r.height / vb.height : 1;
    const nums = (path.getAttribute("d") || "").match(/[MLHVZmlhvz]|-?\d*\.?\d+(?:e-?\d+)?/g) || [];
    let x = 0, y = 0, x0 = 0, y0 = 0, cmd = "";
    const segs = [];
    const push = (nx, ny, d) => { if (d) segs.push([x, y, nx, ny]); x = nx; y = ny; };
    for (let i = 0; i < nums.length;) {
      if (/[A-Za-z]/.test(nums[i])) cmd = nums[i++];
      if (cmd === "M") { push(+nums[i++], +nums[i++], false); x0 = x; y0 = y; cmd = "L"; }
      else if (cmd === "L") push(+nums[i++], +nums[i++], true);
      else if (cmd === "H") push(+nums[i++], y, true);
      else if (cmd === "V") push(x, +nums[i++], true);
      else if (cmd === "Z" || cmd === "z") { push(x0, y0, true); cmd = ""; }
      else i++;
    }
    const owner = svg.parentElement || svg;
    for (const [x1, y1, x2, y2] of segs) {
      const ax = r.left + x1 * sx, ay = r.top + y1 * sy, bx = r.left + x2 * sx, by = r.top + y2 * sy;
      if (Math.abs(ay - by) < 0.01 && Math.abs(ax - bx) > 0.5) add({ el: owner, src: "svg", side: "h", h: true, at: ay, from: Math.min(ax, bx), to: Math.max(ax, bx), path });
      else if (Math.abs(ax - bx) < 0.01 && Math.abs(ay - by) > 0.5) add({ el: owner, src: "svg", side: "v", h: false, at: ax, from: Math.min(ay, by), to: Math.max(ay, by), path });
    }
  }
  const out = [], seen = new Set();
  const modal = el => el.closest('[role="dialog"][aria-modal="true"], [role="menu"], [role="listbox"], [popover]:popover-open, [data-slot^="introduction-"]');
  for (const horiz of [true, false]) {
    const list = edges.filter(e => e.h === horiz).sort((a, b) => a.at - b.at);
    for (let i = 0; i < list.length; i++) for (let j = i + 1; j < list.length && list[j].at - list[i].at <= near; j++) {
      const a = list[i], b = list[j];
      if (a.el === b.el && a.src !== "svg") continue;
      if (a.el === b.el && a.path === b.path) continue;
      if (a.el.contains(b.el) && a.src !== "svg" && b.src === "svg" || false) { }
      if (modal(a.el) !== modal(b.el)) continue;
      const o = Math.min(a.to, b.to) - Math.max(a.from, b.from);
      if (o < overlap) continue;
      const d = +(b.at - a.at).toFixed(2);
      if (d > near) continue;
      if (d === 0 && a.src === b.src && a.src === "border" && a.side === b.side) { }
      const key = `${a.src}/${b.src}|${nm(a.el)}|${nm(b.el)}|${horiz}|${d}`;
      if (seen.has(key)) continue;
      seen.add(key);
      out.push({ d, o: +o.toFixed(0), at: +a.at.toFixed(1), kinds: `${a.src}:${a.side}+${b.src}:${b.side}`, a: nm(a.el), b: nm(b.el), els: [a.el, b.el] });
    }
  }
  return out;
}
export function mark(list) {
  document.querySelectorAll("[data-border-probe-mark]").forEach(n => n.remove());
  for (const v of list) for (const el of v.els) { const r = el.getBoundingClientRect(); const m = document.createElement("div"); m.setAttribute("data-border-probe-mark", ""); m.style.cssText = `position:fixed;left:${r.left}px;top:${r.top}px;width:${r.width}px;height:${r.height}px;outline:2px solid magenta;pointer-events:none;z-index:2147483647`; document.body.append(m); }
}
