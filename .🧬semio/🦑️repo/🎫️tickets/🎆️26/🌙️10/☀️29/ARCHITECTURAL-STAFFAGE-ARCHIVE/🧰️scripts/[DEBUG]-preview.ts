import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const OUT = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/components";
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { depict, defaultDepictionOptions } = await import(`${S}/🔨️modules/🎨️depiction/🟦️.ts`);
const { solveStructure } = await import(`${S}/🔨️modules/🏗️structure/🟦️.ts`);
const { solveThermal } = await import(`${S}/🔨️modules/🌡️thermal/🟦️.ts`);
const doc = JSON.parse(readFileSync(`${S}/🗂️catalog/🧱️components/🔣️.json`, "utf8"));
const only = process.argv[2];
const views = (process.argv[3] ?? "light-solid,forces,moments").split(",");
const den = Number(process.argv[4] ?? 100);
const f = (n: number) => n.toFixed(2);
function svg(scene: any): string {
  const parts: string[] = [];
  const W = scene.widthMm, H = scene.heightMm;
  if (scene.background) parts.push(`<rect width="${f(W)}" height="${f(H)}" fill="${scene.background}"/>`);
  for (const layer of scene.layers) for (const it of layer.items) {
    if (it.kind === "path") {
      const d = it.commands.map((c: any) => c.op === "Z" ? "Z" : c.op === "C" ? `C${f(c.x1)} ${f(c.y1)} ${f(c.x2)} ${f(c.y2)} ${f(c.x)} ${f(c.y)}` : `${c.op}${f(c.x)} ${f(c.y)}`).join(" ");
      const fill = it.fill ? `fill="${it.fill.color}" fill-opacity="${it.fill.opacity ?? 1}"` : `fill="none"`;
      const stroke = it.stroke ? `stroke="${it.stroke.color}" stroke-width="${f(it.stroke.widthMm)}" stroke-linecap="${it.stroke.cap ?? "butt"}"` : "";
      parts.push(`<path d="${d}" ${fill} ${stroke}/>`);
    } else {
      parts.push(`<text x="${f(it.x)}" y="${f(it.y)}" font-size="${f(it.sizeMm)}" fill="${it.color}" text-anchor="${it.anchor}" font-family="DejaVu Sans">${it.text.replace(/&/g, "&amp;").replace(/</g, "&lt;")}</text>`);
    }
  }
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${f(W)}mm" height="${f(H)}mm" viewBox="0 0 ${f(W)} ${f(H)}">${parts.join("")}</svg>`;
}
mkdirSync(OUT, { recursive: true });
for (const asset of doc.assets) {
  if (only && asset.id !== only) continue;
  const queries = [...asset.poses.map((p: any) => ({ kind: "pose", pose: p.id }))];
  for (const q of queries) {
    const state = evaluate(asset, q);
    const thermal = solveThermal(state);
    const structure = solveStructure(state);
    for (const view of views) {
      const scene = depict(state, { ...defaultDepictionOptions(view as any, "en", den), annotations: { dimensions: false, scaleBar: true, legend: true, labels: true, ground: true } }, { thermal, structure });
      writeFileSync(`${OUT}/${asset.id}--${q.pose}--${view}--1-${den}.svg`, svg(scene));
    }
  }
}
console.log("ok");
