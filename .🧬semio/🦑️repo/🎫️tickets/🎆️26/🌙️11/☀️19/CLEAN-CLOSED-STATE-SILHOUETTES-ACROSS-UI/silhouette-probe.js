/** 🫧 Live closure probe: paste into a page, then `silhouette.targets()` lists interactive elements with centres, and after a real hover/focus `silhouette.check(el|index)` verifies the stated box's painted outline is one closed loop. */
(() => {
  const priority = { normal: 0, active: 1, invalid: 2, waiting: 3, loading: 4, introduced: 5, celebrated: 6, focus: 7, hover: 8 };
  const interactiveSelector = 'button, a[href], select, input:not([type="hidden"]), textarea, [contenteditable="true"], [tabindex]:not([tabindex="-1"]), [role="button"], [role="tab"], [role="option"], [role="menuitem"], [role="menuitemradio"], [role="menuitemcheckbox"], [role="treeitem"], [role="switch"], [role="checkbox"], [role="radio"], [data-shared-border-interactive]';
  const boxSelector = '[data-slot="mode-dock-tab"], [data-window-silhouette-chip="true"], [data-shared-border-frame], [data-shared-border-edges], [data-window-silhouette], [data-shared-border-cell], [data-shared-border-cells] > *';
  const visible = el => { const r = el.getBoundingClientRect(); return r.width > 4 && r.height > 4 && r.bottom > 0 && r.right > 0 && r.top < innerHeight && r.left < innerWidth && getComputedStyle(el).visibility !== 'hidden'; };
  const boxOf = el => {
    for (let node = el; node; node = node.parentElement) {
      if (!node.matches(boxSelector)) continue;
      if (node === el || node.matches(interactiveSelector) || node.matches('[data-slot="mode-dock-tab"], [data-window-silhouette-chip="true"]')) return node;
      const inner = [...node.querySelectorAll(interactiveSelector)].filter(visible);
      if (inner.length === 1 && inner[0] === el) return node;
    }
    return null;
  };
  const segments = (statedOnly = false) => {
    const out = [];
    for (const path of document.querySelectorAll('svg[data-shared-border-overlay] path, [data-window-silhouette-border] path')) {
      const state = path.getAttribute('data-shared-border-state') ?? (/window-silhouette-border-(\w+)/.exec(path.getAttribute('class') ?? '')?.[1] ?? 'normal');
      if (statedOnly && state === 'normal') continue;
      const style = getComputedStyle(path), stroke = style.stroke;
      if (!stroke || stroke === 'none' || style.visibility === 'hidden') continue;
      const m = path.getScreenCTM(); if (!m) continue;
      const map = (x, y) => ({ x: m.a * x + m.c * y + m.e, y: m.b * x + m.d * y + m.f });
      const tokens = (path.getAttribute('d') ?? '').match(/[MLHVZmlhvz]|-?\d*\.?\d+(?:e-?\d+)?/g) ?? [];
      let i = 0, cmd = 'M', x = 0, y = 0, sx = 0, sy = 0;
      const num = () => parseFloat(tokens[i++]);
      while (i < tokens.length) {
        if (/[A-Za-z]/.test(tokens[i])) cmd = tokens[i++];
        let nx = x, ny = y;
        if (cmd === 'M') { x = sx = num(); y = sy = num(); cmd = 'L'; continue; }
        if (cmd === 'L') { nx = num(); ny = num(); } else if (cmd === 'H') nx = num(); else if (cmd === 'V') ny = num();
        else if (cmd === 'Z' || cmd === 'z') { nx = sx; ny = sy; }
        else { i++; continue; }
        out.push({ from: map(x, y), to: map(nx, ny), state, stroke, path }); x = nx; y = ny;
        if (cmd === 'Z' || cmd === 'z') cmd = 'M';
      }
    }
    return out;
  };
  const coverage = (segs, a, b, horizontal, tol, gapTolerance = 0.75) => {
    const line = horizontal ? a.y : a.x, lo = Math.min(horizontal ? a.x : a.y, horizontal ? b.x : b.y), hi = Math.max(horizontal ? a.x : a.y, horizontal ? b.x : b.y);
    const spans = segs.filter(s => horizontal ? Math.abs(s.from.y - s.to.y) < 0.01 && Math.abs(s.from.y - line) <= tol : Math.abs(s.from.x - s.to.x) < 0.01 && Math.abs(s.from.x - line) <= tol)
      .map(s => [Math.min(horizontal ? s.from.x : s.from.y, horizontal ? s.to.x : s.to.y), Math.max(horizontal ? s.from.x : s.from.y, horizontal ? s.to.x : s.to.y), s]).sort((p, q) => p[0] - q[0]);
    const gaps = []; let at = lo + tol;
    for (const [s, e] of spans) {
      if (e < lo || s > hi) continue;
      if (s > at + gapTolerance) gaps.push([at, s]);
      at = Math.max(at, e);
    }
    if (at < hi - tol - gapTolerance) gaps.push([at, hi]);
    return { gaps, spans: spans.map(x => x[2]) };
  };
  const sidesOf = ({ left, top, right, bottom }) => ({
    top: [{ x: left, y: top }, { x: right, y: top }, true],
    bottom: [{ x: left, y: bottom }, { x: right, y: bottom }, true],
    left: [{ x: left, y: top }, { x: left, y: bottom }, false],
    right: [{ x: right, y: top }, { x: right, y: bottom }, false],
  });
  /** 🪟 A snapped chip may move by its one-pixel inset, two-pixel snap and subpixel edge alignment. Verify the joins at the actual painted corners. */
  const closesSnappedChip = (segments, measured) => {
    const bounds = { left: Infinity, top: Infinity, right: -Infinity, bottom: -Infinity };
    for (const { from, to } of segments) {
      bounds.left = Math.min(bounds.left, from.x, to.x);
      bounds.top = Math.min(bounds.top, from.y, to.y);
      bounds.right = Math.max(bounds.right, from.x, to.x);
      bounds.bottom = Math.max(bounds.bottom, from.y, to.y);
    }
    return bounds.right > bounds.left && bounds.bottom > bounds.top &&
      Object.keys(bounds).every(side => Math.abs(bounds[side] - measured[side]) <= 3.75) &&
      Object.values(sidesOf(bounds)).every(([a, b, h]) => coverage(segments, a, b, h, 0.02, 0.02).gaps.length === 0);
  };
  const cssLines = el => {
    const cs = getComputedStyle(el), sides = ['Top', 'Right', 'Bottom', 'Left'].map(s => cs[`border${s}Width`] !== '0px' && cs[`border${s}Style`] !== 'none' ? cs[`border${s}Color`] : null);
    const drawn = sides.filter(Boolean);
    return { sides, partialBorder: drawn.length > 0 && (drawn.length < 4 || new Set(drawn).size > 1) && drawn.some(c => c !== drawn[0] || drawn.length < 4) ? sides : null, outline: cs.outlineStyle !== 'none' && cs.outlineWidth !== '0px' ? `${cs.outlineWidth} ${cs.outlineStyle} ${cs.outlineColor}` : null, ring: cs.boxShadow !== 'none' ? cs.boxShadow : null };
  };
  const visibleShadow = value => value.split(/,(?![^(]*\))/).some(part => !/rgba\([^)]*,\s*0\)/.test(part) && /(^|\s)[1-9][\d.]*px/.test(part.replace(/rgba?\([^)]*\)/, '')));
  const label = el => `${el.tagName.toLowerCase()}${el.id ? '#' + el.id : ''}${el.getAttribute('role') ? '[role=' + el.getAttribute('role') + ']' : ''} "${(el.getAttribute('aria-label') ?? el.textContent ?? '').trim().slice(0, 30)}"`;
  let list = [];
  const api = {
    targets(root = document) {
      list = [...root.querySelectorAll(interactiveSelector)].filter(visible).filter(el => !el.closest('[aria-hidden="true"], [inert], [aria-disabled="true"]') && !el.matches(':disabled'));
      return list.map((el, index) => { const r = el.getBoundingClientRect(); return { index, x: Math.round(r.left + r.width / 2), y: Math.round(r.top + r.height / 2), label: label(el) }; });
    },
    check(target, { tol = 1.6, before = null } = {}) {
      const el = typeof target === 'number' ? list[target] : target;
      const box = boxOf(el);
      const result = { label: label(el), box: box ? label(box) : null, issues: [] };
      const stated = segments(true);
      if (!box) {
        const leaks = stated.filter(s => (s.state === 'hover' || s.state === 'focus') && ancestorsOf(el).some(an => an.matches(boxSelector) && an.contains(s.path)));
        if (leaks.length) result.issues.push(`state leaks to ${leaks.length} ancestor segment(s) of an unframed control`);
      }
      if (box) {
        const r = box.getBoundingClientRect();
        if (r.left < 0 || r.top < 0 || r.right > innerWidth || r.bottom > innerHeight) { result.skipped = 'box exceeds viewport'; return result; }
        for (let n = box.parentElement; n; n = n.parentElement) { const cs = getComputedStyle(n); if (!/(auto|scroll|hidden|clip)/.test(cs.overflowX + cs.overflowY)) continue; const c = n.getBoundingClientRect(); if (r.left < c.left - 1 || r.top < c.top - 1 || r.right > c.right + 1 || r.bottom > c.bottom + 1) { result.skipped = 'box clipped by an ancestor'; return result; } }
        const states = new Set(stated.filter(s => box.contains(s.path) || box.parentElement?.contains(s.path)).map(s => s.state));
        const own = stated.filter(s => [s.from, s.to].every(p => p.x >= r.left - tol && p.x <= r.right + tol && p.y >= r.top - tol && p.y <= r.bottom + tol));
        if (!own.length) { result.issues.push('no stated outline on its box'); }
        else {
          const ownState = own.reduce((w, s) => priority[s.state] > priority[w] ? s.state : w, 'normal');
          const strong = stated.filter(s => priority[s.state] >= priority[ownState]);
          const chip = box.hasAttribute('data-window-silhouette-chip');
          const sides = chip && closesSnappedChip(own.filter(s => s.state === ownState), r) ? {} : sidesOf(r);
          for (const [side, [a, b, h]] of Object.entries(sides)) {
            const { gaps } = coverage(strong, a, b, h, tol);
            if (gaps.length) result.issues.push(`${side} open ${gaps.map(g => g.map(v => v.toFixed(1)).join('–')).join(', ')}`);
          }
          const outside = stated.filter(s => s.state === ownState && !own.includes(s) && ancestorsOf(box).some(an => an.contains(s.path) && an.matches(boxSelector)) && [s.from, s.to].some(p => p.x < r.left - 3 || p.x > r.right + 3 || p.y < r.top - 3 || p.y > r.bottom + 3));
          if (outside.length) result.issues.push(`state leaks to ${outside.length} ancestor segment(s)`);
          result.state = ownState; result.seen = [...states];
        }
        if (result.issues.length && box.hasAttribute('data-window-silhouette-chip')) {
          const nearby = stated.filter(({ from, to }) =>
            Math.min(from.x, to.x) <= r.right + 4 && Math.max(from.x, to.x) >= r.left - 4 &&
            Math.min(from.y, to.y) <= r.bottom + 4 && Math.max(from.y, to.y) >= r.top - 4);
          result.diagnostic = {
            box: [r.left, r.top, r.right, r.bottom],
            segments: nearby.slice(0, 24).map(({ from, to, state }) => [state, from.x, from.y, to.x, to.y]),
          };
        }
      }
      for (const node of [...new Set([el, box].filter(Boolean))]) {
        const c = cssLines(node), was = before?.get(node);
        if (c.partialBorder && (!was || JSON.stringify(was.sides) !== JSON.stringify(c.sides))) result.issues.push(`state-dependent partial css border ${c.sides.join(' | ')} on ${label(node)}`);
        if (c.outline && c.outline !== was?.outline) result.issues.push(`css outline ${c.outline}`);
        if (c.ring && node.getAttribute('role') !== 'slider' && c.ring !== was?.ring && visibleShadow(c.ring)) result.issues.push(`css ring ${c.ring.slice(0, 60)}`);
      }
      return result;
    },
    segments,
    async sweep(root = document, { limit = 80, start = 0, onResult = null } = {}) {
      const frame = () => new Promise(r => requestAnimationFrame(() => requestAnimationFrame(r)));
      const pending = el => {
        const owner = el.closest('[data-shared-border-root]');
        return owner ? owner.hasAttribute('data-shared-border-pending') : document.querySelector('[data-shared-border-pending]') !== null;
      };
      const settle = async el => {
        await frame();
        for (let i = 0; i < 20 && pending(el); i++) await frame();
      };
      const found = api.targets(root).slice(0, limit), report = [];
      for (const { index } of found.slice(start)) {
        const el = list[index];
        if (!el.isConnected) continue;
        const before = new Map([el, boxOf(el)].filter(Boolean).map(node => [node, cssLines(node)]));
        el.dispatchEvent(new PointerEvent('pointerover', { bubbles: true, composed: true, pointerType: 'mouse' }));
        await settle(el);
        const hover = api.check(el, { before });
        el.dispatchEvent(new PointerEvent('pointerout', { bubbles: true, composed: true, pointerType: 'mouse', relatedTarget: document.body }));
        el.focus?.({ focusVisible: true, preventScroll: true });
        await settle(el);
        const focus = el.matches(':focus-visible') ? api.check(el, { before }) : null;
        el.blur?.();
        for (const [mode, r] of [['hover', hover], ['focus', focus]]) if (r && r.issues.length) report.push({ mode, ...r });
        onResult?.(index, report);
      }
      await frame();
      return { targets: found.length, failing: report.length, report };
    },
  };
  const ancestorsOf = el => { const out = []; for (let n = el.parentElement; n; n = n.parentElement) out.push(n); return out; };
  window.silhouette = api;
  return 'silhouette probe ready';
})();
