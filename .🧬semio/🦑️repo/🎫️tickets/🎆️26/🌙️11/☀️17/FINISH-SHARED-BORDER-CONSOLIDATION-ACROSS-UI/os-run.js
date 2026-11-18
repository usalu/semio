/** 🔭️ Story runner for the os border probe: `window.__runAll(ids, {mark, wait})` renders each Storybook story through the preview channel and probes it; returns one line per story. */
const probeUrl = new URL("./border-gap-probe.js", import.meta.url).href;
const { probe } = await import(probeUrl + "?t=" + Date.now());
const render = async (id, wait) => {
  const ch = window.__STORYBOOK_ADDONS_CHANNEL__;
  await new Promise(res => {
    let done = false;
    const f = () => { if (done) return; done = true; for (const e of ["storyRendered", "storyErrored", "storyThrewException", "storyMissing"]) ch.off(e, f); res(); };
    for (const e of ["storyRendered", "storyErrored", "storyThrewException", "storyMissing"]) ch.on(e, f);
    ch.emit("setCurrentStory", { storyId: id, viewMode: "story" });
    setTimeout(f, 25000);
  });
  await new Promise(r => setTimeout(r, wait));
};
window.__runAll = async (ids, { mark = false, wait = 700, prefix = "" } = {}) => {
  const out = [];
  for (const id of ids) {
    try {
      await render(id, wait);
      const r = probe({ mark });
      const root = document.getElementById("storybook-root");
      out.push(`${r.count} ${id.slice(prefix.length)} w${innerWidth} html${root?.innerHTML.length ?? 0} art${r.art?.length ?? 0}${r.violations.length ? "\n   " + r.violations.slice(0, 8).map(v => v.distance + " " + v.a + " || " + v.b).join("\n   ") : ""}`);
    } catch (e) { out.push(`ERR ${id} ${e.message}`); }
  }
  return out.join("\n");
};
window.__probe = probe;
