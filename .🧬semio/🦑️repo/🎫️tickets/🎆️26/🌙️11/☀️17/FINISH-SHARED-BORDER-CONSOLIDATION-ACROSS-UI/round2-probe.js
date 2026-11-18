import { probe } from "./round2-probe-core.js";
const slim = el => `${el.tagName.toLowerCase()}${el.getAttribute("data-slot") ? `[${el.getAttribute("data-slot")}]` : ""}«${(el.textContent || "").trim().replace(/\s+/g, " ").slice(0, 24)}»`;
export async function foreground({ mark = false } = {}) {
  for (let i = 0; i < 80 && document.querySelector("[data-shared-border-pending]"); i++) await new Promise(r => setTimeout(r, 100));
  const r = probe({ mark });
  const behind = el => { for (let strip = el.closest("[data-layered-strip]"); strip; strip = strip.parentElement?.closest("[data-layered-strip]")) if (strip.parentElement?.querySelector(":scope > [data-layered-overlay]")) return true; return false; };
  const own = r.violations.filter(v => !v.elements.some(behind));
  return { vp: r.viewport, total: r.count, foreground: own.length, behindGlass: r.count - own.length, v: own.map(v => `${v.kind} ${v.distance} ${v.a.split(" ").slice(-1)}/${slim(v.elements[0])} || ${slim(v.elements[1])} ${v.b.split(" ").slice(-1)}`) };
}
