/** 🔭️ One staffage view: navigate, click optional radio, wait until the compositor commits (rAF needs a fronted pane), then probe. */
import * as h from "./staffage-helpers.js";
export async function view(route, radio, { settle = 2500, scroll } = {}) {
  location.hash = "#/" + route;
  await h.sleep(settle);
  if (radio) { const b = [...document.querySelectorAll("[role=radio]")].find(x => x.textContent.trim() === radio); if (!b) return route + " no radio"; b.click(); await h.sleep(radio === "3D model" ? 4000 : 2000); }
  if (scroll) { const p = h.page(); if (p) p.scrollTop = p.scrollHeight; await h.sleep(500); }
  let t = 0;
  while (document.querySelector("[data-shared-border-pending]") && t < 36) { window.dispatchEvent(new Event("resize")); await h.sleep(500); t++; }
  const { run } = await import("./staffage-probe-run.js?t=" + Date.now());
  const r = run({ top: 6, mark: false });
  const pend = !!document.querySelector("[data-shared-border-pending]");
  const m = /"count":(\d+)/.exec(r);
  return `${route} ${radio ?? ""} pending=${pend} waited=${t / 2}s stage=${document.querySelector("[data-model-stage]")?.dataset.state} count=${m?.[1]} ${m && m[1] !== "0" ? r.slice(0, 900) : ""}`;
}
