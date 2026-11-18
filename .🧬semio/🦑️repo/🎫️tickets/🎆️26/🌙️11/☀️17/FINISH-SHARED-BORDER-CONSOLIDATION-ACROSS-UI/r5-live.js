/** 🔎️ Installs the focus reader and the base probe on window. */
const D = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI/";
export async function install() {
  const p = await import("/@fs" + encodeURI(D + "border-gap-probe.js") + "?t=" + Date.now());
  window.__m = await import("/@fs" + encodeURI(D + "r5-find.js") + "?t=" + Date.now());
  window.__probe = () => { const r = p.probe({ mark: false }); return JSON.stringify({ w: innerWidth, count: r.count, v: r.violations.map(v => v.distance + " " + v.a + " || " + v.b), pending: !!document.querySelector("[data-shared-border-pending]") }); };
  window.__f = () => { const e = document.activeElement; const c = getComputedStyle(e); return JSON.stringify({ tag: e.tagName, cls: (e.className + "").slice(0, 40), txt: (e.textContent || "").trim().slice(0, 24), fv: e.matches(":focus-visible"), outline: c.outlineStyle, shadow: c.boxShadow.slice(0, 40), deco: c.textDecorationLine, bc: c.borderInlineStartColor + "|" + c.borderInlineEndColor, nextbc: e.nextElementSibling ? getComputedStyle(e.nextElementSibling).borderInlineStartColor : "", segs: [...document.querySelectorAll('[data-shared-border-state="focus"]')].map(s => getComputedStyle(s).strokeWidth + "/" + getComputedStyle(s).stroke + (s.hasAttribute("data-focus-on-accent") ? "/onaccent" : "")).slice(0, 4) }); };
  window.__tabTo = (sel, nth = 0) => { const els = [...document.querySelectorAll(sel)].filter(e => !e.closest("[inert]") && e.getBoundingClientRect().width > 0); const i = els[nth]; i.scrollIntoView({ block: "center" }); const all = [...document.querySelectorAll("button,a[href],select,input,[tabindex]")].filter(e => !e.closest("[inert]") && e.getBoundingClientRect().width > 0 && e.tabIndex >= 0); all[all.indexOf(i) - 1].focus(); return "prev focused"; };
  return "ok";
}
