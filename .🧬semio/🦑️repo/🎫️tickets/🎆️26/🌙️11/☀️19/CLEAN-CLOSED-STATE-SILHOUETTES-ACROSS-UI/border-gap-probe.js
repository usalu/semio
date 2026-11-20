/** 🧹️ Removes the magenta marks `probe({ mark: true })` painted. */
export function clear() { document.querySelectorAll("[data-border-probe-mark]").forEach(n => n.remove()); }

/** 🔎️ Live border-consolidation probe: every painted border run must be one shared line.
 * Collects painted CSS border sides and shared-border SVG segments in viewport space and reports
 * pairs of parallel, overlapping edges that lie close together but do not coincide (`gap`: two lines with a
 * gap or an inset box-in-box) and coincident CSS borders painted by two owners (`double`).
 * Load in a page served by Vite: `const p = await import("/@fs" + encodeURI("<abs path to this file>")); p.probe()`.
 * Options: `{ gap: 40, overlap: 6, root: document.body, mark: true }`. Elements inside `[data-border-probe-art]` (charts,
 * mockups, diagrams, canvases — content art, not chrome) are skipped and listed in `art`. Anchored shell panels
 * (`[data-slot="panel"][data-anchor]`) are opaque overlays on the canvas: an edge hidden BENEATH a panel it is not part of (the midpoint
 * of the pair's shared span lies inside that panel's rect) is not a visible line and is not compared. Everything visible is compared:
 * panel vs navbar/footer band, panel vs panel, panel vs content beside it, panel-internal edges. A docked panel is therefore checked
 * against all its neighbours; only what the panel covers is ignored. `panels` in the result counts the anchored panels probed.
 * An open modal layer (dialog, menu, listbox, popover, UIIntroduction) is its own layer: pairs across it are skipped, pairs within it count.
 * `overlaps`: pairs of visible interactive controls (button, a, [role=button], input, select) or text-bearing leaf elements whose rects
 * intersect by more than 2 px of area while neither contains the other (art, inert and behind-glass content excluded). */
export function probe({ gap = 40, overlap = 6, root = document.body, mark = false } = {}) {
  const edges = [];
  const art = new Set();
  const behindGlass = el => { for (let strip = el.closest("[data-layered-strip]"); strip; strip = strip.parentElement?.closest("[data-layered-strip]")) if (strip.parentElement?.querySelector(":scope > [data-layered-overlay]")) return true; return false; };
  const visible = el => {
    if (el.closest("details:not([open]) > :not(summary)") || el.closest("[data-layered-strip] [inert], [data-layered-strip][inert]") || behindGlass(el)) return false;
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
  const clipOf = el => {
    let c = { left: -Infinity, top: -Infinity, right: Infinity, bottom: Infinity };
    for (let e = el.parentElement; e && e !== document.documentElement; e = e.parentElement) {
      const s = getComputedStyle(e);
      if (s.overflowX === "visible" && s.overflowY === "visible" && s.clipPath === "none") continue;
      const r = e.getBoundingClientRect();
      c = { left: Math.max(c.left, r.left), top: Math.max(c.top, r.top), right: Math.min(c.right, r.right), bottom: Math.min(c.bottom, r.bottom) };
    }
    return c;
  };
  const clipEdge = (edge, c) => {
    const [lo, hi, nlo, nhi] = edge.h ? [c.left, c.right, c.top, c.bottom] : [c.top, c.bottom, c.left, c.right];
    if (edge.at < nlo - 0.5 || edge.at > nhi + 0.5) return null;
    const from = Math.max(edge.from, lo), to = Math.min(edge.to, hi);
    return to - from > 0.5 ? { ...edge, from, to } : null;
  };
  const addEdge = (edge, c) => { const e = clipEdge(edge, c); if (e) edges.push(e); };
  const transparent = c => c === "transparent" || /rgba\([^)]*,\s*0\)$/.test(c);
  for (const el of root.querySelectorAll("*")) {
    if (el.closest("[data-border-probe-art]")) { art.add(el.closest("[data-border-probe-art]")); continue; }
    if (el instanceof SVGElement) continue;
    const s = getComputedStyle(el);
    const sides = ["Top", "Right", "Bottom", "Left"].filter(k => parseFloat(s[`border${k}Width`]) > 0 && s[`border${k}Style`] !== "none" && s[`border${k}Style`] !== "hidden" && !transparent(s[`border${k}Color`]));
    if (!sides.length || !visible(el)) continue;
    const r = el.getBoundingClientRect();
    const clip = clipOf(el);
    for (const k of sides) {
      const w = parseFloat(s[`border${k}Width`]);
      const h = k === "Top" || k === "Bottom";
      const at = k === "Top" ? r.top + w / 2 : k === "Bottom" ? r.bottom - w / 2 : k === "Left" ? r.left + w / 2 : r.right - w / 2;
      addEdge({ el, kind: "css", side: k.toLowerCase(), h, at, from: h ? r.left : r.top, to: h ? r.right : r.bottom, style: s[`border${k}Style`] }, clip);
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
    const clip = clipOf(svg);
    for (const [x1, y1, x2, y2] of pts) {
      const ax = r.left + x1 * sx, ay = r.top + y1 * sy, bx = r.left + x2 * sx, by = r.top + y2 * sy;
      if (Math.abs(ay - by) < 0.01 && Math.abs(ax - bx) > 0.5) addEdge({ el: owner, kind: "svg", side: "h", h: true, at: ay, from: Math.min(ax, bx), to: Math.max(ax, bx) }, clip);
      else if (Math.abs(ax - bx) < 0.01 && Math.abs(ay - by) > 0.5) addEdge({ el: owner, kind: "svg", side: "v", h: false, at: ax, from: Math.min(ay, by), to: Math.max(ay, by) }, clip);
    }
  }
  const clipped = el => {
    const r = el.getBoundingClientRect();
    let top = r.top, bottom = r.bottom, left = r.left, right = r.right;
    for (let p = el.parentElement; p && p !== document.documentElement; p = p.parentElement) {
      const s = getComputedStyle(p);
      if (s.overflowY === "visible" && s.overflowX === "visible") continue;
      const q = p.getBoundingClientRect();
      if (s.overflowY !== "visible") { top = Math.max(top, q.top); bottom = Math.min(bottom, q.bottom); }
      if (s.overflowX !== "visible") { left = Math.max(left, q.left); right = Math.min(right, q.right); }
    }
    return { top, bottom, left, right, width: right - left, height: bottom - top };
  };
  const boxes = [...root.querySelectorAll("*")].filter(el => !(el instanceof SVGElement) && !el.hasAttribute("data-border-probe-mark")).map(clipped).filter(r => r.width > 0 && r.height > 0);
  const spanned = (a, b) => boxes.some(r => a.h
    ? Math.abs(r.top - a.at) <= 2 && Math.abs(r.bottom - b.at) <= 2 && Math.min(r.right, a.to, b.to) - Math.max(r.left, a.from, b.from) >= overlap
    : Math.abs(r.left - a.at) <= 2 && Math.abs(r.right - b.at) <= 2 && Math.min(r.bottom, a.to, b.to) - Math.max(r.top, a.from, b.from) >= overlap);
  const bandOf = el => el.closest('[data-slot="navbar"], [data-slot="footer"]');
  const panelOf = el => el.closest('[data-slot="panel"][data-anchor]');
  const panelEls = [...root.querySelectorAll('[data-slot="panel"][data-anchor]')].filter(visible);
  const panelBoxes = new Map(panelEls.map(el => [el, el.getBoundingClientRect()]));
  const coveredByPanel = (edge, other) => {
    const mid = (Math.max(edge.from, other.from) + Math.min(edge.to, other.to)) / 2;
    const [x, y] = edge.h ? [mid, edge.at] : [edge.at, mid];
    return panelEls.some(panel => !panel.contains(edge.el) && (r => x > r.left + 1.5 && x < r.right - 1.5 && y > r.top + 1.5 && y < r.bottom - 1.5)(panelBoxes.get(panel)));
  };
  const modalOf = el => el.closest('[role="dialog"][aria-modal="true"], [role="menu"], [role="listbox"], [popover]:popover-open, [data-slot^="introduction-"]');
  const layered = (a, b) => modalOf(a.el) !== modalOf(b.el) || coveredByPanel(a, b) || coveredByPanel(b, a);
  const violations = [];
  const seen = new Set();
  const groups = [edges.filter(e => e.h).sort((a, b) => a.at - b.at), edges.filter(e => !e.h).sort((a, b) => a.at - b.at)];
  for (const list of groups) {
    for (let i = 0; i < list.length; i++) {
      for (let j = i + 1; j < list.length && list[j].at - list[i].at <= gap; j++) {
        const a = list[i], b = list[j];
        if (a.el === b.el) continue;
        if (layered(a, b)) continue;
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
        violations.push({ kind, axis: a.h ? "horizontal" : "vertical", at: `${a.at.toFixed(1)}/${b.at.toFixed(1)} span ${Math.max(a.from, b.from).toFixed(0)}-${Math.min(a.to, b.to).toFixed(0)}`, distance: +d.toFixed(2), overlap: +o.toFixed(1), a: `${name(a.el)} ${a.kind}:${a.side}`, b: `${name(b.el)} ${b.kind}:${b.side}`, elements: [a.el, b.el] });
      }
    }
  }
  const overlaps = [];
  const items = [...root.querySelectorAll("*")].filter(el => {
    if (el instanceof SVGElement || el.hasAttribute("data-border-probe-mark") || el.closest("[data-border-probe-art]") || el.closest(".sr-only") || el.matches(".sr-only")) return false;
    const control = el.matches("button, a, [role=button], input, select");
    const leaf = !control && el.children.length === 0 && (el.textContent || "").trim().length > 0;
    if (!control && !leaf) return false;
    if (el.closest("button, a, [role=button]") && !control) return false;
    return visible(el);
  }).map(el => ({ el, r: clipped(el) })).filter(({ r }) => r.width > 0 && r.height > 0 && r.bottom > 0 && r.top < innerHeight && r.right > 0 && r.left < innerWidth).sort((x, y) => x.r.left - y.r.left);
  for (let i = 0; i < items.length; i++) for (let j = i + 1; j < items.length && items[j].r.left < items[i].r.right; j++) {
    const a = items[i], b = items[j];
    if (a.el.contains(b.el) || b.el.contains(a.el)) continue;
    const w = Math.min(a.r.right, b.r.right) - Math.max(a.r.left, b.r.left), h = Math.min(a.r.bottom, b.r.bottom) - Math.max(a.r.top, b.r.top);
    if (w <= 0 || h <= 0 || w * h <= 2) continue;
    if (modalOf(a.el) !== modalOf(b.el) || (panelOf(a.el) !== panelOf(b.el) && !bandOf(a.el) && !bandOf(b.el))) continue;
    overlaps.push({ area: +(w * h).toFixed(1), a: name(a.el), b: name(b.el), elements: [a.el, b.el] });
  }
  if (mark) {
    for (const o of overlaps) violations.push({ elements: o.elements });

    document.querySelectorAll("[data-border-probe-mark]").forEach(n => n.remove());
    for (const v of violations) for (const el of v.elements) {
      const r = el.getBoundingClientRect();
      const m = document.createElement("div");
      m.setAttribute("data-border-probe-mark", "");
      m.style.cssText = `position:fixed;left:${r.left}px;top:${r.top}px;width:${r.width}px;height:${r.height}px;outline:2px solid magenta;pointer-events:none;z-index:2147483647`;
      document.body.append(m);
    }
  }
  return { pending: document.hidden || !!document.querySelector("[data-shared-border-pending]"), url: location.href, viewport: `${innerWidth}x${innerHeight}`, edges: edges.length, count: violations.filter(v => v.kind).length, violations: violations.filter(v => v.kind).map(({ elements, ...v }) => v), overlaps: overlaps.map(({ elements, ...o }) => o), art: [...art].map(name), panels: panelEls.length };
}
