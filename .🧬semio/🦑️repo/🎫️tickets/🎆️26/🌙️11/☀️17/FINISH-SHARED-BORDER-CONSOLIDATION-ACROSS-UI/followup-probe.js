import { probe } from "./border-gap-probe.js";
const slim = el => `${el.tagName.toLowerCase()}${el.getAttribute("data-slot") ? `[${el.getAttribute("data-slot")}]` : ""}«${(el.textContent || "").trim().replace(/\s+/g, " ").slice(0, 22)}»`;
export async function run({ mark = false, skip = true } = {}) {
  if (skip) { const b = [...document.querySelectorAll("button")].find(x => /^(skip|überspringen)$/i.test(x.textContent.trim())); b?.click(); }
  await new Promise(r => setTimeout(r, 1200));
  for (let i = 0; i < 100 && document.querySelector("[data-shared-border-pending]"); i++) await new Promise(r => setTimeout(r, 100));
  const r = probe({ mark });
  return JSON.stringify({ vp: r.viewport, pending: !!document.querySelector("[data-shared-border-pending]"), count: r.count, v: r.violations.slice(0, 20).map(v => `${v.kind} ${v.distance} ${v.a.slice(-70)} || ${v.b.slice(-70)}`) });
}
