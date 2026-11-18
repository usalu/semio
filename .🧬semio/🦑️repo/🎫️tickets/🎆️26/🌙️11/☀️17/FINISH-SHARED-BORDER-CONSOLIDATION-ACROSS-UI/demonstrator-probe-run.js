import { probe } from "./border-gap-probe.js";

/** 🔭️ Skips the landing introduction, waits `ms`, probes and returns a compact JSON string. */
export async function run({ ms = 2500, skip = true, mark = true } = {}) {
  await new Promise(r => setTimeout(r, ms));
  if (skip) {
    const b = [...document.querySelectorAll("button")].find(b => /^(skip|überspringen)/i.test(b.textContent.trim()));
    if (b) { b.click(); await new Promise(r => setTimeout(r, 1200)); }
  }
  const r = probe({ mark });
  return JSON.stringify({ vp: r.viewport, hash: location.hash, count: r.count, art: r.art, v: r.violations.map(v => v.distance + " " + v.a.replace(/\.[^ ]*/g, "") + " || " + v.b.replace(/\.[^ ]*/g, "")) });
}
