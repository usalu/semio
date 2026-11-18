/** 🔭️ Footer overflow reading: opens the first pane of the demonstrator and reports every footer item box against the footer box. */
const sl = ms => new Promise(x => setTimeout(x, ms));
export async function open() {
  await sl(5000);
  [...document.querySelectorAll("button")].find(b => /^(skip|überspringen)/i.test(b.textContent.trim()))?.click(); await sl(2500);
  const o = [...document.querySelectorAll("button,a")].filter(b => !b.closest("[inert]") && b.getBoundingClientRect().width > 0 && /open|öffnen/i.test(b.textContent));
  o[0]?.click(); await sl(4500);
}
export function footer() {
  const f = document.querySelector("footer[data-slot=footer]");
  if (!f) return "no footer";
  const fr = f.getBoundingClientRect();
  const items = [...f.querySelectorAll("[data-slot=navbar-item],[data-slot=navbar-centered]")].map(i => { const r = i.getBoundingClientRect(); return `${i.firstElementChild?.getAttribute("class")?.slice(0,30) ?? ""}|${i.textContent.trim().slice(0,20)}|${Math.round(r.left)}-${Math.round(r.right)} w${Math.round(r.width)}`; });
  return JSON.stringify({ w: innerWidth, footer: [Math.round(fr.left), Math.round(fr.right)], docW: document.documentElement.scrollWidth, rowScroll: f.firstElementChild && [f.firstElementChild.scrollWidth, f.firstElementChild.clientWidth], items }, null, 1);
}
const wait = ms => new Promise(x => setTimeout(x, ms));
export async function gate(label) {
  const p = await import("./border-gap-probe.js?t=" + Date.now());
  let r;
  for (let i = 0; i < 25; i++) { r = p.probe({}); if (!r.pending) break; await wait(400); }
  const f = document.querySelector("footer[data-slot=footer]");
  const row = f?.firstElementChild;
  const over = [...document.querySelectorAll("*")].filter(e => e.getBoundingClientRect().right > innerWidth + 0.5 && !e.closest("[inert],[aria-hidden=true]") && getComputedStyle(e).position !== "fixed").length;
  return `${label} w${innerWidth} count=${r.count} pending=${!!r.pending} docScroll=${document.documentElement.scrollWidth}/${innerWidth} footerRow=${row ? row.scrollWidth + "/" + row.clientWidth : "-"} beyondRight=${over}` + r.violations.slice(0, 6).map(v => `\n   ${v.distance} ${v.a.slice(0, 90)} || ${v.b.slice(0, 90)}`).join("");
}
