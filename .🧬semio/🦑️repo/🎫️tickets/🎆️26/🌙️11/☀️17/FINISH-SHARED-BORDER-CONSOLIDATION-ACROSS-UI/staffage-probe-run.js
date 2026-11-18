import { probe } from "./border-gap-probe.js";
const short = s => s.replace(/\.(relative|flex|min-h-0|w-full|min-w-0|h-full|inline-flex|items-center|max-w-full)/g, "").replace(/«[^»]*»/g, m => m.slice(0, 22) + "»").slice(0, 110);
const card = s => s.includes("[slot=staffage-card-stack]");
const chrome = s => s.includes("nav[slot=navbar]") || s.includes("footer[slot=shared-border-row]");
export function run({ top = 25, mark = true } = {}) {
  const r = probe({ mark });
  const own = [], fw = [], edge = [];
  for (const v of r.violations) (card(v.a) && card(v.b) ? fw : chrome(v.a) || chrome(v.b) ? edge : own).push(v);
  const group = list => { const g = new Map(); for (const v of list) { const k = `${short(v.a)} || ${short(v.b)}`; const e = g.get(k) ?? { n: 0, d: v.distance, axis: v.axis }; e.n++; g.set(k, e); } return [...g].sort((x, y) => y[1].n - x[1].n).slice(0, top).map(([k, e]) => `${e.n}x ${e.axis[0]} d=${e.d} ${k}`); };
  return JSON.stringify({ u: location.hash || location.pathname, vp: r.viewport, count: r.count, edges: r.edges, own: own.length, card: fw.length, scrollEdge: edge.length, art: r.art.length, top: group(own), edgeTop: group(edge).slice(0, 4), cardTop: group(fw).slice(0, 2) });
}
