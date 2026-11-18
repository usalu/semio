import * as h from "./staffage-helpers.js";
const sleep = h.sleep;
const radio = async (name, wait = 3500) => { const b = [...document.querySelectorAll("[role=radio]")].find(x => x.textContent.trim() === name); if (!b) return "no " + name; b.click(); await sleep(wait); return "ok"; };
const stageSeams = () => {
  const st = document.querySelector("[data-model-stage]");
  if (!st) return null;
  const r = st.getBoundingClientRect(), w = [];
  for (let e = st.parentElement; e && w.length < 4; e = e.parentElement) { const c = getComputedStyle(e), b = e.getBoundingClientRect(); w.push([e.tagName, c.borderBottomWidth, c.borderRightWidth, Math.round(b.bottom - r.bottom), Math.round(b.right - r.right)].join(" ")); }
  const cs = e => { const c = getComputedStyle(e); return [c.borderTopWidth, c.borderRightWidth, c.borderBottomWidth, c.borderLeftWidth].join("/"); };
  return { state: st.dataset.state, framed: st.hasAttribute("data-framed"), root: cs(st), blocks: [...st.querySelectorAll(".semio-stage-block")].map(b => b.className.replace(/semio-stage-/g, "") + " " + cs(b) + (b.hidden ? " hidden" : "")), parents: w };
};
export async function view(label, fn) {
  const pre = fn ? await fn() : "";
  const out = await h.go(null, { wait: 600 });
  return label + " " + out + " " + pre + " " + JSON.stringify(stageSeams());
}
export { radio, stageSeams, h };
