/** 🔭️ Final staffage live gate without the frame shim: every route and view, a reading counts only when no shared border is pending. */
import * as h from "./staffage-helpers.js";
import { probe } from "./border-gap-probe.js";
const routes = ["", "all", "animals", "humans/walking-commuter", "animals/domestic-cat", "furniture/cafe-chair", "appliances/circulation-pump", "collections", "compare", "playground"];
const settle = async () => { for (let i = 0; i < 100 && document.querySelector("[data-shared-border-pending]"); i++) await h.sleep(250); };
const read = async (label) => {
  let out = "";
  for (let k = 0; k < 4; k++) {
    await settle();
    const r = probe({});
    out = `${label} w${innerWidth} count=${r.count} pending=${!!r.pending} edges=${r.edges}` + r.violations.slice(0, 4).map(v => `\n   ${v.distance} ${v.a.slice(0, 90)} || ${v.b.slice(0, 90)}`).join("");
    if (!r.pending) break;
    await h.sleep(800);
  }
  return out;
};
export const rows = [];
export async function runAll(only) {
  rows.length = 0;
  for (const route of routes) {
    if (only && !only.includes(route || "overview")) continue;
    location.hash = "#/" + route;
    await h.sleep(3500);
    rows.push(await read(route || "overview"));
    if (route === "collections") { const p = document.querySelector('[data-page="collections"]'); if (p) p.scrollTop = p.scrollHeight; await h.sleep(600); rows.push(await read("collections-bottom")); }
    if (route.includes("/")) for (const name of ["Three views", "3D model"]) {
      const b = [...document.querySelectorAll("[role=radio]")].find(x => x.textContent.trim() === name && !x.closest("[inert]") && x.getBoundingClientRect().width);
      if (!b) { rows.push(`${route} ${name} no radio`); continue; }
      b.click();
      await h.sleep(2500);
      if (name === "3D model") for (let i = 0; i < 40 && !document.querySelector("[data-model-stage]"); i++) await h.sleep(500);
      if (name === "3D model") await h.sleep(7000);
      rows.push(await read(`${route} ${name}` + (name === "3D model" ? " stage=" + document.querySelector("[data-model-stage]")?.dataset.state : "")));
    }
  }
  window.__sd = true;
}
