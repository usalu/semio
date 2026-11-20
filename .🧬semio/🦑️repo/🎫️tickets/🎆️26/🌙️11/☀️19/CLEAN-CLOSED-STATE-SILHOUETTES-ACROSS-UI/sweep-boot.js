/** 🧹️ Boots the silhouette sweep in the elements story page: resumable per story through sessionStorage. */
(async () => {
  if (!window.silhouette) {
    const src = await (await fetch("/@fs/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️19/CLEAN-CLOSED-STATE-SILHOUETTES-ACROSS-UI/silhouette-probe.js?" + Date.now())).text();
    (0, eval)(src);
  }
  window.runSweep = async (name, {visit} = {}) => {
    if (visit) {
      for (const key of round3.list()) await visit(key, () => {
        round3.show(key);
        return document.getElementById("round3-live");
      }, 30);
      return;
    }
    const out = JSON.parse(sessionStorage.getItem(name) || '{"stories":{}}');
    for (const key of round3.list()) {
      if (out.stories[key]) continue;
      try {
        round3.show(key);
        await new Promise(r => setTimeout(r, 900));
        const r = await silhouette.sweep(document.getElementById("round3-live"), { limit: 30 });
        out.stories[key] = { t: r.targets, f: r.failing, issues: r.report.map(x => `${x.mode} ${x.label} box=${x.box} :: ${x.issues.join("; ")}`) };
      } catch (e) { out.stories[key] = { error: String(e) }; }
      sessionStorage.setItem(name, JSON.stringify(out));
    }
    out.finished = true;
    sessionStorage.setItem(name, JSON.stringify(out));
  };
  window.summarize = b => {
    const s = Object.entries(b.stories), kinds = {};
    for (const [, v] of s) for (const i of v.issues || []) for (const part of i.split(":: ")[1].split("; ")) {
      const key = part.replace(/[\d.–, ]+$/, "").replace(/ on .*/, "").replace(/rgb\([^)]*\)/g, "C").replace(/\d+ ancestor/, "N ancestor").slice(0, 40);
      kinds[key] = (kinds[key] || 0) + 1;
    }
    return { finished: b.finished, stories: s.length, targets: s.reduce((a, [, v]) => a + (v.t || 0), 0), failing: s.reduce((a, [, v]) => a + (v.f || 0), 0), errors: s.filter(([, v]) => v.error).length, kinds, worst: s.filter(([, v]) => v.f).sort((a, b) => b[1].f - a[1].f).slice(0, 20).map(([k, v]) => `${k}:${v.f}/${v.t}`) };
  };
  window.sweepState = name => summarize(JSON.parse(sessionStorage.getItem(name) || '{"stories":{}}'));
})();
