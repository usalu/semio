const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { solveThermalAt } = await import(`${S}/🔨️modules/🌡️thermal/🟦️.ts`);
const { loadCatalogFromFiles } = await import(`${S}/🔨️modules/🗂️catalog/📥️files/🟦️.ts`);
const cat = await loadCatalogFromFiles();
const [id, clipId] = process.argv.slice(2);
const asset = cat.assets.find((a: any) => a.id === id);
const clip = asset.clips.find((c: any) => c.id === clipId);
const bad = (v: any, p = ""): string[] => (typeof v === "number" ? (Number.isFinite(v) ? [] : [p]) : v && typeof v === "object" ? Object.entries(v).flatMap(([k, x]) => bad(x, p + "." + k)) : []);
for (let t = 0; t <= clip.durationS + 1e-9; t += clip.durationS / 28) {
  const r = solveThermalAt(asset, { kind: "clip", clip: clipId, timeS: t });
  const b = bad(r).slice(0, 4);
  console.log(t.toFixed(2), b.length ? b.join(" ") : "ok");
}
