import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { checkAssetDetailed } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveThermalAt } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";
import { svgMarkup } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { authoredScales, consistency, sweep } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";

const ids = (process.env.QA2_ASSETS ?? "").split(",").filter(Boolean);
const out = process.env.QA2_OUT!;
const log = process.env.QA2_LOG!;
const doRender = process.env.QA2_RENDER !== "0";
const poseOnly = (process.env.QA2_POSES ?? "").split(",").filter(Boolean);
const fractions = (process.env.QA2_FRACTIONS ?? "0.15,0.5,0.95").split(",").map(Number);
const scaleFilter = (process.env.QA2_SCALES ?? "").split(",").filter(Boolean).map(Number);
const POINTS = ["elevation", "front", "plan"] as const;
writeFileSync(log, "");
const say = (line: string): void => { appendFileSync(log, line + "\n"); };
const num = (v: number): string => (Math.round(v * 1e4) / 1e4).toString();
const sheet = (label: string, scenes: any[]): string => {
  const gap = 6, head = 8;
  const height = head + Math.max(...scenes.map((s) => s.heightMm));
  const width = scenes.reduce((s, c) => s + c.widthMm, 0) + gap * (scenes.length + 1);
  let x = gap;
  const cells = scenes.map((s, i) => { const c = `<text x="${num(x)}" y="5" font-size="2.5" font-family="Helvetica,Arial,sans-serif" fill="#001117">${label} · ${POINTS[i]}</text><g transform="translate(${num(x)} ${num(height - s.heightMm)})">${svgMarkup(s)}</g>`; x += s.widthMm + gap; return c; }).join("");
  return `<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" version="1.1" width="${num(width)}mm" height="${num(height)}mm" viewBox="0 0 ${num(width)} ${num(height)}"><rect width="${num(width)}" height="${num(height)}" fill="#ffffff"/>${cells}</svg>\n`;
};
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
for (const id of ids) {
  const asset = catalog.assets.find((a) => a.id === id)!;
  const report = checkAssetDetailed(asset, { samples: 3 });
  say(`## ${id} check: ${report.problems.length ? "\n   " + report.problems.slice(0, 20).join("\n   ") : "ok"}`);
  const scales = authoredScales(asset);
  for (const scale of scales) for (const f of [...sweep(asset, scale), ...consistency(asset, scale)]) say(`   [${f.check}] 1:${scale} ${f.message}`);
  if (!doRender) continue;
  const dir = `${out}/${id}`;
  mkdirSync(dir, { recursive: true });
  for (const scale of scales) {
    if (scaleFilter.length && !scaleFilter.includes(scale)) continue;
    const queries: { name: string; query: any; thermal?: boolean }[] = [];
    for (const pose of asset.poses.filter((p) => !p.key && (!poseOnly.length || poseOnly.includes(p.id)))) queries.push({ name: pose.id, query: { kind: "pose", pose: pose.id } });
    for (const clip of (poseOnly.length ? [] : asset.clips)) {
      const ks = [...new Set((clip.keys as any[]).map((k) => k.t))].sort((a, b) => a - b);
      const times = new Set<number>();
      for (let i = 0; i < ks.length; i++) { times.add(ks[i]!); if (i + 1 < ks.length) { times.add((ks[i]! + ks[i + 1]!) / 2); times.add(ks[i]! + (ks[i + 1]! - ks[i]!) * 0.1); } }
      for (const f of fractions) times.add(clip.durationS * f);
      let list = [...times].map((t) => Math.min(Math.round(t * 100) / 100, clip.durationS * 0.999)).sort((a, b) => a - b);
      list = [...new Set(list)];
      for (const t of list) queries.push({ name: `${clip.id}@${t}`, query: { kind: "clip", clip: clip.id, timeS: t } });
    }
    queries.push({ name: `thermal-${asset.presentation.defaultPose}`, query: { kind: "pose", pose: asset.presentation.defaultPose }, thermal: true });
    for (const q of queries) {
      try {
        const state = evaluate(asset, q.query, { kind: "scale", scale: { denominator: scale } });
        const view = q.thermal ? "thermal" : "dark-line";
        const analysis = q.thermal ? { thermal: solveThermalAt(state.asset, q.query) } : {};
        const scenes = POINTS.map((at) => { const o = defaultDepictionOptions(view, "en", scale, at); return depict(state, { ...o, transport: [], representation: { kind: "scale", scale: { denominator: scale } }, annotations: { ...o.annotations, legend: false } }, analysis); });
        const svgText = sheet(q.name, scenes);
        writeFileSync(`${dir}/${id}--${q.name}--1-${scale}.svg`, svgText);
        const hs = scenes.map((sc) => Bun.hash(svgMarkup(sc).replace(/<text[^>]*>.*?<\/text>/g, "")).toString(36)).join(" ");
        say(`   hash 1:${scale} ${q.name} ${hs}`);
      } catch (e) { say(`   render 1:${scale} ${q.name}: ${(e as Error).message}`); }
    }
  }
}
say("done");
