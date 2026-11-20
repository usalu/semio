/** 🐚️ OS parts silhouette sweep: mounts every part of the previous ticket's OS showcase and keeps results per viewport in sessionStorage. */
(async () => {
  const matrix = document.currentScript?.src.endsWith("/__matrix_os_sweep.js");
  if (!window.silhouette) (0, eval)(await (await fetch("http://127.0.0.1:5452/@fs/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️19/CLEAN-CLOSED-STATE-SILHOUETTES-ACROSS-UI/silhouette-probe.js?" + Date.now())).text());
  window.__osLive = await import(matrix ? "/__matrix_os_live" : "/@fs/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI/round3-os-live.tsx");
  window.osRun = async ({visit} = {}) => {
    if (visit) {
      for (const part of __osLive.partNames) await visit(part, () => {
        __osLive.show(part);
        return document.getElementById("os-chrome-live");
      }, 20);
      return;
    }
    const key = "os:" + innerWidth;
    const st = JSON.parse(sessionStorage.getItem(key) || '{"done":[],"t":0,"issues":[]}');
    for (const part of __osLive.partNames) {
      if (st.done.includes(part)) continue;
      try {
        __osLive.show(part);
        await new Promise(r => setTimeout(r, 1200));
        const r = await silhouette.sweep(document.getElementById("os-chrome-live") || document, { limit: 20 });
        st.t += r.targets;
        st.issues.push(...r.report.map(x => `${part} | ${x.mode} ${x.label.slice(0, 40)} box=${(x.box || "").slice(0, 25)} :: ${x.issues.join("; ").slice(0, 160)}`));
      } catch (e) { st.issues.push(`${part} ERROR ${String(e).slice(0, 100)}`); }
      st.done.push(part);
      sessionStorage.setItem(key, JSON.stringify(st));
    }
    st.finished = true;
    sessionStorage.setItem(key, JSON.stringify(st));
  };
  window.osState = () => { const st = JSON.parse(sessionStorage.getItem("os:" + innerWidth) || "null"); return st && { done: st.done.length, t: st.t, f: st.issues.length, finished: !!st.finished }; };
})();
