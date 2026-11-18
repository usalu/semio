/** 🔎️ Live border-consolidation probe: every painted border run must be one shared line.
 * Collects painted CSS border sides and shared-border SVG segments in viewport space and reports
 * pairs of parallel, overlapping edges that lie close together but do not coincide (`gap`: two lines with a
 * gap or an inset box-in-box) and coincident CSS borders painted by two owners (`double`).
 * Load in a page served by Vite: `const p = await import("/@fs" + encodeURI("<abs path to this file>")); p.probe()`.
 * Options: `{ gap: 40, overlap: 6, root: document.body, mark: true }`. Elements inside `[data-border-probe-art]` (charts,
 * mockups, diagrams, canvases — content art, not chrome) are skipped and listed in `art`. */
export function probe({ gap = 40, overlap = 6, root = document.body, mark = false } = {}) {
  const edges = [];
  const art = new Set();
  const visible = el => {
    if (el.closest("details:not([open]) > :not(summary)")) return false;
    for (let e = el; e; e = e.parentElement) {
      const s = getComputedStyle(e);
      if (s.display === "none" || s.visibility === "hidden" || +s.opacity === 0) return false;
    }
    const r = el.getBoundingClientRect();
    return r.width > 0 && r.height > 0 && r.bottom > 0 && r.right > 0 && r.top < innerHeight * 3 && r.left < innerWidth;
  };
  const name = el => {
    const slot = el.getAttribute("data-slot");
    const cls = typeof el.className === "string" ? el.className.split(/\s+/).filter(c => c && !c.includes(":")).slice(0, 3).join(".") : "";
    const text = (el.textContent || "").trim().replace(/\s+/g, " ").slice(0, 30);
    return `${el.tagName.toLowerCase()}${slot ? `[slot=${slot}]` : ""}${cls ? "." + cls : ""} «${text}»`;
  };
  const transparent = c => c === "transparent" || /rgba\([^)]*,\s*0\)$/.test(c);
  for (const el of root.querySelectorAll("*")) {
    if (el.closest("[data-border-probe-art]")) { art.add(el.closest("[data-border-probe-art]")); continue; }
    if (el instanceof SVGElement) continue;
    const s = getComputedStyle(el);
    const sides = ["Top", "Right", "Bottom", "Left"].filter(k => parseFloat(s[`border${k}Width`]) > 0 && s[`border${k}Style`] !== "none" && s[`border${k}Style`] !== "hidden" && !transparent(s[`border${k}Color`]));
    if (!sides.length || !visible(el)) continue;
    const r = el.getBoundingClientRect();
    for (const k of sides) {
      const w = parseFloat(s[`border${k}Width`]);
      const h = k === "Top" || k === "Bottom";
      const at = k === "Top" ? r.top + w / 2 : k === "Bottom" ? r.bottom - w / 2 : k === "Left" ? r.left + w / 2 : r.right - w / 2;
      edges.push({ el, kind: "css", side: k.toLowerCase(), h, at, from: h ? r.left : r.top, to: h ? r.right : r.bottom, style: s[`border${k}Style`] });
    }
  }
  for (const path of root.querySelectorAll("path[data-shared-border-segment]")) {
    const svg = path.ownerSVGElement;
    if (!svg || !visible(svg) || svg.closest("[data-border-probe-art]")) continue;
    const r = svg.getBoundingClientRect();
    const vb = svg.viewBox.baseVal;
    const sx = vb && vb.width ? r.width / vb.width : 1, sy = vb && vb.height ? r.height / vb.height : 1;
    const nums = (path.getAttribute("d") || "").match(/[MLHVZmlhvz]|-?\d*\.?\d+(?:e-?\d+)?/g) || [];
    let x = 0, y = 0, sx0 = 0, sy0 = 0, cmd = "";
    const pts = [];
    const push = (nx, ny, draw) => { if (draw) pts.push([x, y, nx, ny]); x = nx; y = ny; };
    for (let i = 0; i < nums.length;) {
      if (/[A-Za-z]/.test(nums[i])) cmd = nums[i++];
      if (cmd === "M") { push(+nums[i++], +nums[i++], false); sx0 = x; sy0 = y; cmd = "L"; }
      else if (cmd === "L") push(+nums[i++], +nums[i++], true);
      else if (cmd === "H") push(+nums[i++], y, true);
      else if (cmd === "V") push(x, +nums[i++], true);
      else if (cmd === "Z" || cmd === "z") { push(sx0, sy0, true); cmd = ""; }
      else i++;
    }
    const owner = svg.parentElement || svg;
    for (const [x1, y1, x2, y2] of pts) {
      const ax = r.left + x1 * sx, ay = r.top + y1 * sy, bx = r.left + x2 * sx, by = r.top + y2 * sy;
      if (Math.abs(ay - by) < 0.01 && Math.abs(ax - bx) > 0.5) edges.push({ el: owner, kind: "svg", side: "h", h: true, at: ay, from: Math.min(ax, bx), to: Math.max(ax, bx) });
      else if (Math.abs(ax - bx) < 0.01 && Math.abs(ay - by) > 0.5) edges.push({ el: owner, kind: "svg", side: "v", h: false, at: ax, from: Math.min(ay, by), to: Math.max(ay, by) });
    }
  }
  const boxes = [...root.querySelectorAll("*")].filter(el => !(el instanceof SVGElement) && !el.hasAttribute("data-border-probe-mark")).map(el => el.getBoundingClientRect()).filter(r => r.width > 0 && r.height > 0);
  const spanned = (a, b) => boxes.some(r => a.h
    ? Math.abs(r.top - a.at) <= 1.5 && Math.abs(r.bottom - b.at) <= 1.5 && Math.min(r.right, a.to, b.to) - Math.max(r.left, a.from, b.from) >= overlap
    : Math.abs(r.left - a.at) <= 1.5 && Math.abs(r.right - b.at) <= 1.5 && Math.min(r.bottom, a.to, b.to) - Math.max(r.top, a.from, b.from) >= overlap);
  const violations = [];
  const seen = new Set();
  const groups = [edges.filter(e => e.h).sort((a, b) => a.at - b.at), edges.filter(e => !e.h).sort((a, b) => a.at - b.at)];
  for (const list of groups) {
    for (let i = 0; i < list.length; i++) {
      for (let j = i + 1; j < list.length && list[j].at - list[i].at <= gap; j++) {
        const a = list[i], b = list[j];
        if (a.el === b.el) continue;
        const o = Math.min(a.to, b.to) - Math.max(a.from, b.from);
        if (o < overlap) continue;
        const d = b.at - a.at;
        let between = false;
        for (let k = i + 1; k < j && !between; k++) {
          const c = list[k];
          between = c.at > a.at + 0.75 && c.at < b.at - 0.75 && Math.min(c.to, a.to, b.to) - Math.max(c.from, a.from, b.from) >= overlap;
        }
        if (between || (d > 0.75 && spanned(a, b))) continue;
        const kind = d <= 0.75 ? (a.kind === "css" && b.kind === "css" ? "double" : null) : "gap";
        if (!kind) continue;
        const key = `${kind}|${name(a.el)}|${name(b.el)}|${a.h}`;
        if (seen.has(key)) continue;
        seen.add(key);
        violations.push({ kind, axis: a.h ? "horizontal" : "vertical", distance: +d.toFixed(2), overlap: +o.toFixed(1), a: `${name(a.el)} ${a.kind}:${a.side}`, b: `${name(b.el)} ${b.kind}:${b.side}`, elements: [a.el, b.el] });
      }
    }
  }
  if (mark) {
    document.querySelectorAll("[data-border-probe-mark]").forEach(n => n.remove());
    for (const v of violations) for (const el of v.elements) {
      const r = el.getBoundingClientRect();
      const m = document.createElement("div");
      m.setAttribute("data-border-probe-mark", "");
      m.style.cssText = `position:fixed;left:${r.left}px;top:${r.top}px;width:${r.width}px;height:${r.height}px;outline:2px solid magenta;pointer-events:none;z-index:2147483647`;
      document.body.append(m);
    }
  }
  return { url: location.href, viewport: `${innerWidth}x${innerHeight}`, edges: edges.length, count: violations.length, violations, art: [...art].map(name) };
}
