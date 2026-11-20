/** 🧭️ Resumable app-page silhouette sweep: loads the probe from the elements server and keeps per-target progress in sessionStorage across reloads. */
(async () => {
  if (!window.__pumped) {
    window.__pumped = true;
    const pump = ms => new Promise(resolve => { const start = performance.now(), channel = new MessageChannel(); channel.port1.onmessage = () => { if (performance.now() - start >= ms) { channel.port1.close(); resolve(); } else channel.port2.postMessage(0); }; channel.port2.postMessage(0); });
    window.requestAnimationFrame = cb => { pump(16).then(() => cb(performance.now())); return 1; };
    window.cancelAnimationFrame = () => {};
  }
  if (!window.silhouette) (0, eval)(await (await fetch("http://127.0.0.1:5452/@fs/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️19/CLEAN-CLOSED-STATE-SILHOUETTES-ACROSS-UI/silhouette-probe.js?" + Date.now())).text());
  window.appSweep = async (name, limit = 60, progress) => {
    const key = `${name}:${innerWidth}`;
    const state = JSON.parse(sessionStorage.getItem(key) || '{"next":0,"issues":[]}');
    const r = await silhouette.sweep(document, { limit, start: state.next, onResult: (index, report) => { progress?.(index); state.next = index + 1; state.issues = [...new Set([...state.issues, ...report.map(x => `${x.mode} ${x.label.slice(0, 50)} box=${(x.box || "").slice(0, 30)} :: ${x.issues.join("; ").slice(0, 170)}`)])]; sessionStorage.setItem(key, JSON.stringify(state)); } });
    state.t = r.targets; state.done = true; state.f = state.issues.length;
    sessionStorage.setItem(key, JSON.stringify(state));
  };
  window.appState = name => JSON.parse(sessionStorage.getItem(`${name}:${innerWidth}`) || "null");
})();
