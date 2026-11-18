/** 🔭️ Runner for the os chrome showcase: `await run(part, {mark, wait})` mounts one part of r3fw-live.tsx and probes it.
 * Another agent fronting its own tab stops animation frames and throttles timers of this one mid-run, so waits and frames always run on a `MessageChannel` pump and transitions are disabled. */
const base = import.meta.url.replace(/r3fw-run\.js.*$/, "");
const pump = ms => new Promise(resolve => {
  const start = performance.now();
  const channel = new MessageChannel();
  channel.port1.onmessage = () => { if (performance.now() - start >= ms) { channel.port1.close(); resolve(); } else channel.port2.postMessage(0); };
  channel.port2.postMessage(0);
});
const ensureFrames = async () => {
  if (window.__shimmed) return;
  window.__shimmed = true;
  window.requestAnimationFrame = cb => { pump(16).then(() => cb(performance.now())); return 1; };
  window.cancelAnimationFrame = () => {};
  const style = document.createElement("style");
  style.id = "os-chrome-no-motion";
  style.textContent = "*,*::before,*::after{transition:none !important;animation:none !important}";
  document.head.append(style);
};
export async function run(part, { mark = false, wait = 900 } = {}) {
  await ensureFrames();
  const m = await import(base + "r3fw-live.tsx?t=" + Date.now());
  await m.show(part);
  await pump(wait);
  for (let i = 0; i < 25 && document.querySelector("[data-shared-border-pending]"); i++) await pump(200);
  await pump(300);
  for (let i = 0; i < 25 && document.querySelector("[data-shared-border-pending]"); i++) await pump(200);
  const p = await import(base + "border-gap-probe.js?t=" + Date.now());
  const r = p.probe({ mark });
  return `${r.count} ${part} w${innerWidth} art${r.art.length} edges${r.edges} pn${r.panels} ov${r.overlaps.length}${r.pending ? " PENDING" : ""}` + (r.violations.length ? "\n   " + r.violations.slice(0, 12).map(v => v.distance + " @" + v.at + " " + v.a + " || " + v.b).join("\n   ") : "") + (r.overlaps.length ? "\n   OVERLAP " + r.overlaps.slice(0, 6).map(o => o.area + " " + o.a + " || " + o.b).join("\n   OVERLAP ") : "");
}
export async function runAll(parts, opts) { const out = []; for (const p of parts) { try { out.push(await run(p, opts)); } catch (e) { out.push(`ERR ${p} ${e.message}`); } } return out.join("\n"); }
