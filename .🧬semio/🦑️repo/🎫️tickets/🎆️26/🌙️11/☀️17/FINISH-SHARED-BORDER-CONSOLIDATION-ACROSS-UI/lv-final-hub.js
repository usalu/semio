/** 🔭️ Final hub admin gate: every tab (tab list on desktop and tablet, the one select cell on a phone), the expanded space row and the create dialog. */
import { probe } from "./border-gap-probe.js";
const sl = ms => new Promise(x => setTimeout(x, ms));
const read = async label => {
  let r;
  for (let i = 0; i < 25; i++) { r = probe({}); if (!r.pending) break; await sl(400); }
  return `${label} w${innerWidth} count=${r.count} pending=${!!r.pending} edges=${r.edges} ov=${r.overlaps.length}` + r.violations.slice(0, 6).map(v => `\n   ${v.distance} ${v.a.slice(0, 90)} || ${v.b.slice(0, 90)}`).join("");
};
const tabs = ["overview", "spaces", "users", "connections", "documents", "events", "observability"];
const goto = async tab => {
  const direct = document.getElementById("admin-tab-" + tab);
  if (direct && direct.getBoundingClientRect().width > 0) { direct.click(); await sl(1500); return "tab"; }
  const trigger = document.getElementById("admin-tab-switch");
  if (!trigger) return "none";
  trigger.click(); await sl(700);
  const option = [...document.querySelectorAll("[role=option]")].find(o => o.id?.endsWith(tab) || o.getAttribute("data-value") === tab || new RegExp("^" + tab, "i").test(o.textContent.trim()));
  option?.click(); await sl(1500);
  return option ? "select" : "no option";
};
export const rows = [];
export async function run() {
  rows.length = 0; window.__hd = false;
  await sl(2500);
  for (const tab of tabs) {
    const how = await goto(tab);
    rows.push(await read(`${tab}(${how})`));
    if (tab === "spaces") {
      document.querySelector('[data-row-id="space:sp-1"]')?.click(); await sl(1200);
      rows.push(await read("spaces+expanded"));
      document.getElementById("admin-space-create-open")?.click(); await sl(1200);
      rows.push(await read("spaces+dialog"));
      document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", keyCode: 27, bubbles: true })); await sl(800);
    }
  }
  window.__hd = true;
}
