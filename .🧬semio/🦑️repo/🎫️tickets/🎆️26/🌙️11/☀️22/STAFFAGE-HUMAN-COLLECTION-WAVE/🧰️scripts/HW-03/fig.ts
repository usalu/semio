/** 🖼️ Figure-only review image: the evaluated drawing of a pose or clip instant at one scale in the three views side by side (shapes filled by tone, painted in the engine's depth order, body pens at their paper width), and the same at true print size (300 dpi) beside it, so a scale is read both enlarged and as printed.
 *  bun fig.ts <file.json> <scale> <pose|clip:id:time> <out.png> [pixelsPerMetre] */
import { S } from "./lib.ts";
const ev = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const sharp = (await import("../../../../../../../../../node_modules/sharp/lib/index.js")).default;
const [file, sc, q, out, ppmArg] = process.argv.slice(2);
const asset = (await Bun.file(file!).json()).asset;
const scale = Number(sc);
const query = q!.startsWith("clip:") ? { kind: "clip", clip: q!.split(":")[1], timeS: Number(q!.split(":")[2]) } : { kind: "pose", pose: q };
const state = ev.evaluate(asset, query, { kind: "scale", scale: { denominator: scale } });
const pens = asset.representations[sc!].pens;
const svgOf = (ppm: number, stroke: boolean): { svg: string; w: number; h: number } => {
  const views = ["elevation", "front", "plan"] as const;
  let x0 = 0;
  const groups: string[] = [];
  let H = 0;
  const boxes = views.map((v) => {
    const pts = state.shapes[v].flatMap((s: any) => s.rings.flat());
    const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
    return { v, minX: Math.min(...xs), maxX: Math.max(...xs), minY: Math.min(...ys), maxY: Math.max(...ys) };
  });
  for (const b of boxes) H = Math.max(H, (b.maxY - b.minY) * ppm);
  for (const b of boxes) {
    const order = [...state.shapes[b.v]].map((s: any) => ({ s, d: ev.shapeDepth(state, s.shape, b.v) })).sort((p, r) => p.s.shape.z - r.s.shape.z || p.d - r.d);
    const paths = order.map(({ s }: any) => {
      const pen = s.shape.penMm ?? pens[s.shape.role] ?? 0.1;
      if (pen === null) return "";
      const d = s.rings.map((r: any[]) => `M${r.map((p) => `${((p.x - b.minX) * ppm + x0 + 10).toFixed(2)},${(H - (p.y - b.minY) * ppm + 10).toFixed(2)}`).join("L")}${s.closed ? "Z" : ""}`).join("");
      const g = Math.round(255 * (0.15 + 0.8 * s.shape.tone));
      const sw = stroke ? Math.max(0.6, ((pen * scale) / 1000) * ppm) : (pen * 300) / 25.4;
      return `<path d="${d}" fill="${s.shape.filled && s.closed ? `rgb(${g},${g},${g})` : "none"}" fill-rule="evenodd" stroke="#111" stroke-width="${sw.toFixed(2)}" stroke-linejoin="round"/>`;
    });
    groups.push(paths.join(""));
    x0 += (b.maxX - b.minX) * ppm + 30;
  }
  return { svg: groups.join(""), w: x0 + 10, h: H + 20 };
};
const big = svgOf(Number(ppmArg ?? 600), true);
const ppmPrint = (300 / 0.0254) / scale;
const small = svgOf(ppmPrint, false);
const W = big.w + small.w + 40, Hh = Math.max(big.h, small.h);
const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${Hh}"><rect width="100%" height="100%" fill="#fff"/><g>${big.svg}</g><g transform="translate(${big.w + 40},${Hh - small.h})">${small.svg}</g></svg>`;
await sharp(Buffer.from(svg), { limitInputPixels: false }).png().toFile(out!);
console.log(out, `${W.toFixed(0)}x${Hh.toFixed(0)}`);
