import { probe } from "./border-gap-probe.js";
const sleep = t => new Promise(r => setTimeout(r, t));
const fmt = p => [p.edges, p.count, p.violations.map(v => v.distance + " " + v.a + " || " + v.b)];
export const walk = async (mark = false) => {
  const out = [];
  for (const t of ["overview", "spaces", "users", "connections", "documents", "events", "observability"]) {
    document.getElementById("admin-tab-" + t)?.click(); await sleep(1200);
    out.push([t, ...fmt(probe({ mark }))]);
    if (t === "spaces") {
      document.querySelector('[data-row-id="space:sp-1"]')?.click(); await sleep(1200);
      out.push(["spaces+expanded", ...fmt(probe({ mark }))]);
      document.getElementById("admin-space-create-open")?.click(); await sleep(900);
      out.push(["spaces+dialog", ...fmt(probe({ mark }))]);
      document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", keyCode: 27, bubbles: true })); await sleep(500);
    }
  }
  return out;
};
