export const page = () => [...document.querySelectorAll('[data-page^="asset:"],[data-page="collections"],[data-page="compare"],[data-page="playground"],[data-page="all"],[data-page^="category:"]')].filter(e => !e.closest("[data-border-probe-art]") && !e.closest("[inert]") && e.scrollHeight > 0).sort((a, b) => b.scrollHeight - a.scrollHeight)[0];
export const sleep = ms => new Promise(r => setTimeout(r, ms));
export async function go(hash, { top = 14, wait = 3000, y = 0, mark = false } = {}) {
  document.querySelectorAll("[data-border-probe-mark]").forEach(n => n.remove());
  if (hash) location.hash = hash;
  await sleep(wait);
  const pg = page();
  if (pg) pg.scrollTop = y === -1 ? pg.scrollHeight : y;
  await sleep(700);
  const { run } = await import(/* @vite-ignore */ `./staffage-probe-run.js?t=${Date.now()}`);
  const errs = [...document.querySelectorAll("[role=alert]")].map(e => e.textContent.slice(0, 80));
  return run({ top, mark }) + ` alerts=${JSON.stringify(errs)} pg=${pg ? pg.dataset.page + ":" + pg.scrollHeight : "-"} y=${pg ? Math.round(pg.scrollTop) : "-"}`;
}
export const click = async (sel, wait = 600) => { const el = [...document.querySelectorAll(sel)].find(e => !e.closest("[inert]") && !e.closest("[data-border-probe-art]")); if (!el) return "no " + sel; el.click(); await sleep(wait); return "ok"; };
export function expand() {
  const pg = page();
  if (!pg) return "no page";
  for (let e = pg; e && e !== document.documentElement; e = e.parentElement) {
    const s = e.style;
    s.overflow = "visible"; s.height = "auto"; s.maxHeight = "none"; s.contain = "none"; s.minHeight = "0"; s.clipPath = "none";
  }
  return `expanded ${pg.dataset.page} h=${pg.scrollHeight} doc=${document.documentElement.scrollHeight}`;
}
export async function chunk(y, { top = 14, mark = false } = {}) {
  document.querySelectorAll("[data-border-probe-mark]").forEach(n => n.remove());
  window.scrollTo(0, y);
  await sleep(500);
  const { run } = await import(/* @vite-ignore */ `./staffage-probe-run.js?t=${Date.now()}`);
  return run({ top, mark }) + ` y=${Math.round(window.scrollY)}/${document.documentElement.scrollHeight}`;
}
