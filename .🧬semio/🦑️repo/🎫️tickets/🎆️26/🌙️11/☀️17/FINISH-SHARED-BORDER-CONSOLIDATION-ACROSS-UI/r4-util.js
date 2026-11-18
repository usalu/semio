/** 🔭️ Round 4 helpers: settle, page probe, silhouette geometry dump. */
import * as L from "./lv-course-run4.js";
import * as R from "./round3-course-crawl.js";
export { L, R };
export const settle = async (ms = 3500) => { await new Promise(r => setTimeout(r, ms)); };
export const geo = () => [...document.querySelectorAll("[data-overview-card]")].map(c => { const b = c.getBoundingClientRect(); const s = c.querySelector("svg[data-window-silhouette-border]"); const sb = s?.getBoundingClientRect(); return [+b.top.toFixed(2), +b.bottom.toFixed(2), sb && +sb.top.toFixed(2), sb && +sb.bottom.toFixed(2)].join("/"); });
export const reset = () => { R.rows.clear(); sessionStorage.clear(); };
export const start = (job) => { window.__r4done = false; Promise.resolve().then(job).catch(e => R.rows.push("ERR " + e.message)).finally(() => { window.__r4done = true; }); return "started"; };
export const status = () => `done=${window.__r4done} w${innerWidth} lang=${document.documentElement.lang}\n` + R.rows.all.filter(l => !/^\s+OVL/.test(l)).join("\n");
