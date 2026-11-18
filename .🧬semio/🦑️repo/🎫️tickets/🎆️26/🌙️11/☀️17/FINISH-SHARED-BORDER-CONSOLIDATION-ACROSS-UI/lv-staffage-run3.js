/** 🔭️ Resumable staffage live gate: every route and view, results persisted in sessionStorage per viewport. */
import * as h from "./staffage-helpers.js";
const pump = ms => new Promise(resolve => { const start = performance.now(); const c = new MessageChannel(); c.port1.onmessage = () => { if (performance.now() - start >= ms) { c.port1.close(); resolve(); } else c.port2.postMessage(0); }; c.port2.postMessage(0); });
export const shim = () => {
  if (window.__shimmed) return;
  window.__shimmed = true;
  window.requestAnimationFrame = cb => setTimeout(() => cb(performance.now()), 16);
  window.cancelAnimationFrame = id => clearTimeout(id);
  const style = document.createElement("style");
  style.textContent = "*,*::before,*::after{transition:none !important;animation:none !important}";
  document.head.append(style);
  window.dispatchEvent(new Event("resize"));
};
const K = "__lvstC" + innerWidth;
const get = () => JSON.parse(sessionStorage.getItem(K) || "{}");
const put = (k, v) => { const s = get(); s[k] = v; sessionStorage.setItem(K, JSON.stringify(s)); };
const routes = ["", "all", "animals", "humans/walking-commuter", "animals/domestic-cat", "furniture/cafe-chair", "appliances/circulation-pump", "collections", "compare", "playground"];
const probe = async () => { window.dispatchEvent(new Event("resize")); for (let i = 0; i < 80 && document.querySelector("[data-shared-border-pending]"); i++) await h.sleep(250); const r = await h.go(null, { wait: 500, top: 6 }); const m = /"count":(\d+)/.exec(r); return `${document.querySelector("[data-shared-border-pending]") ? "PENDING " : ""}count=${m ? m[1] : "?"} ${m && m[1] !== "0" ? r.slice(0, 700) : ""}`; };
export async function runAll() {
  shim();
  for (const route of routes) {
    location.hash = "#/" + route;
    await h.sleep(3500);
    if (!get()[route || "overview"]) put(route || "overview", await probe());
    if (route === "collections" && !get()["collections bottom"]) {
      const p = document.querySelector('[data-page="collections"]'); if (p) p.scrollTop = p.scrollHeight;
      await h.sleep(600); put("collections bottom", await probe());
    }
    if (route.includes("/")) for (const name of ["Three views", "3D model"]) {
      const key = `${route} ${name}`;
      if (get()[key]) continue;
      const b = [...document.querySelectorAll("[role=radio]")].find(x => x.textContent.trim() === name);
      if (!b) { put(key, "no radio"); continue; }
      b.click();
      await h.sleep(name === "3D model" ? 5000 : 2500);
      let out = await probe();
      if (name === "Three views") out += " unavailable=" + document.querySelectorAll("[data-notice=unavailable]").length;
      else out += " stage=" + document.querySelector("[data-model-stage]")?.dataset.state;
      put(key, out);
    }
  }
  return JSON.stringify(get(), null, 1);
}
