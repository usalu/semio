/** 🔭️ Final demonstrator gate: introduction, overview, an opened pane (the OS shell with its credits footer), back; every reading waits for the compositor. */
import { probe } from "./border-gap-probe.js";
const sl = ms => new Promise(x => setTimeout(x, ms));
const read = async label => {
  let r;
  for (let i = 0; i < 25; i++) { r = probe({}); if (!r.pending) break; await sl(400); }
  return `${label} w${innerWidth} count=${r.count} pending=${!!r.pending} edges=${r.edges} ov=${r.overlaps.length}` + r.violations.slice(0, 6).map(v => `\n   ${v.distance} ${v.a.slice(0, 90)} || ${v.b.slice(0, 90)}`).join("");
};
export const rows = [];
export async function run() {
  rows.length = 0; window.__dd = false;
  await sl(5000);
  const skip = [...document.querySelectorAll("button")].find(b => /^(skip|überspringen)/i.test(b.textContent.trim()));
  rows.push(`introduction (skip button ${!!skip}) ` + await read("intro"));
  skip?.click(); await sl(2500);
  rows.push(await read("overview"));
  const open = [...document.querySelectorAll("button,a")].filter(b => !b.closest("[inert]") && b.getBoundingClientRect().width > 0 && /open|öffnen/i.test(b.textContent));
  open[0]?.click(); await sl(4500);
  rows.push(`${location.hash} ` + await read("opened-pane"));
  document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", keyCode: 27, bubbles: true })); await sl(2500);
  rows.push(await read("back"));
  window.__dd = true;
}
