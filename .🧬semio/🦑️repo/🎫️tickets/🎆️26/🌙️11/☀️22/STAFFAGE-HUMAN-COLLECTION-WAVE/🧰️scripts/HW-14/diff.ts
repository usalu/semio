/** 🩻️ Drawing against volume per view, as the silhouette guard sees them: SVG+PNG with the drawn area red, the volume blue (overlap purple) at 1:`scale` in the default pose (or --pose). bun diff.ts <file.json> <scale> <out-prefix> [--pose id] */
import { readFileSync, writeFileSync } from "node:fs";
import sharp from "/Users/ueli/Documents/semio/node_modules/sharp/lib/index.js";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { VIEWPOINTS, measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const args = process.argv.slice(2);
const [file, scaleArg, prefix] = args;
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
if (args.includes("--pose")) asset.presentation.defaultPose = args[args.indexOf("--pose") + 1];
const scale = Number(scaleArg);
const m = measureAsset(asset, scale as never, undefined, undefined, featureOf(scale as never));
for (const v of VIEWPOINTS) {
  const x = m.views[v] as any;
  const b = x.box;
  const W = b.maxX - b.minX, H = b.maxY - b.minY, k = 900 / Math.max(W, H);
  const ring = (r: any) => "M" + r.map((p: any) => `${((p.x - b.minX) * k).toFixed(1)},${((b.maxY - p.y) * k).toFixed(1)}`).join("L") + "Z";
  const paths = (groups: any[], fill: string) => `<g fill="${fill}">` + groups.map((g: any) => `<path d="${g.map(ring).join("")}" fill-rule="evenodd"/>`).join("") + "</g>";
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${(W * k).toFixed(0)}" height="${(H * k).toFixed(0)}"><rect width="100%" height="100%" fill="white"/><g opacity="0.55">${paths(x.reference, "rgb(60,90,255)")}</g><g opacity="0.5">${paths([...x.filled, ...x.outlines], "rgb(255,40,40)")}</g></svg>`;
  const out = `${prefix}-${scale}-${v}`;
  writeFileSync(`${out}.svg`, svg);
  await sharp(`${out}.svg`).png().toFile(`${out}.png`);
  if (process.env.CROP && process.env.CROP.startsWith(v + ":")) {
    const [cx, cy, r] = process.env.CROP.slice(v.length + 1).split(",").map(Number) as [number, number, number];
    const vx = (cx - r - b.minX) * k, vy = (b.maxY - cy - r) * k, vs = 2 * r * k;
    const grid = Array.from({ length: 21 }, (_, i) => `<line x1="${vx + (i * vs) / 20}" y1="${vy}" x2="${vx + (i * vs) / 20}" y2="${vy + vs}" stroke="#0003" stroke-width="${vs / 800}"/><line x1="${vx}" y1="${vy + (i * vs) / 20}" x2="${vx + vs}" y2="${vy + (i * vs) / 20}" stroke="#0003" stroke-width="${vs / 800}"/>`).join("");
    const crop = svg.replace(/<svg [^>]*>/, `<svg xmlns="http://www.w3.org/2000/svg" width="800" height="800" viewBox="${vx} ${vy} ${vs} ${vs}">`).replace("</svg>", grid + "</svg>");
    await sharp(Buffer.from(crop)).png().toFile(`${out}-crop.png`);
    console.log(`crop ${out}-crop.png: grid ${(2 * r / 20 * 1000).toFixed(0)} mm, left ${(cx - r).toFixed(3)}, top ${(cy + r).toFixed(3)}`);
  }
  const px = Math.round(W * k), py = Math.round(H * k);
  const raster = async (groups: any[]) => {
    const one = `<svg xmlns="http://www.w3.org/2000/svg" width="${px}" height="${py}"><rect width="100%" height="100%" fill="black"/>${paths(groups, "white")}</svg>`;
    const { data } = await sharp(Buffer.from(one)).greyscale().raw().toBuffer({ resolveWithObject: true });
    return Uint8Array.from(data, (x: number) => (x > 127 ? 1 : 0));
  };
  const [A, B] = [await raster([...x.filled, ...x.outlines]), await raster(x.reference)];
  const dist = (M: Uint8Array) => { const d = new Float64Array(px * py).fill(1e9); for (let i = 0; i < M.length; i++) if (M[i]) d[i] = 0; for (let pass = 0; pass < 2; pass++) for (let j = 0; j < py; j++) for (let i = 0; i < px; i++) { const n = pass === 0 ? j * px + i : (py - 1 - j) * px + (px - 1 - i); const [ii, jj] = [n % px, Math.floor(n / px)]; const s = pass === 0 ? -1 : 1; for (const [di, dj, w] of [[s, 0, 1], [0, s, 1], [s, s, 1.414], [-s, s, 1.414]] as const) { const [a, bb] = [ii + di, jj + dj]; if (a < 0 || bb < 0 || a >= px || bb >= py) continue; d[n] = Math.min(d[n]!, d[bb * px + a]! + w); } } return d; };
  const holeCells = 64 * ((featureOf(scale as never) / 4) * k) ** 2;
  const fillHoles = (M: Uint8Array) => { const out = Uint8Array.from(M); const seen = new Uint8Array(M.length); const comp: number[] = []; for (let s0 = 0; s0 < M.length; s0++) { if (M[s0] || seen[s0]) continue; comp.length = 0; let border = false; const st = [s0]; seen[s0] = 1; while (st.length) { const n = st.pop()!; comp.push(n); const [i, j] = [n % px, Math.floor(n / px)]; if (i === 0 || j === 0 || i === px - 1 || j === py - 1) border = true; for (const [a, bb] of [[i + 1, j], [i - 1, j], [i, j + 1], [i, j - 1]]) { if (a < 0 || bb < 0 || a >= px || bb >= py) continue; const m = bb * px + a; if (!M[m] && !seen[m]) { seen[m] = 1; st.push(m); } } } if (!border && comp.length <= holeCells) for (const n of comp) out[n] = 1; } return out; };
  const edge = (M: Uint8Array) => { const F = fillHoles(M); const E = new Uint8Array(M.length); for (let n = 0; n < F.length; n++) { if (!F[n]) continue; const [i, j] = [n % px, Math.floor(n / px)]; if (i === 0 || j === 0 || i === px - 1 || j === py - 1 || !F[n - 1] || !F[n + 1] || !F[n - px] || !F[n + px]) E[n] = 1; } return E; };
  const [eA, eB] = [edge(A), edge(B)];
  const [dA, dB] = [dist(eA), dist(eB)];
  const worst: string[] = [];
  for (const [M, D, name] of [[eA, dB, "drawn-edge"], [eB, dA, "volume-edge"]] as const) {
    let best = 0, at = 0;
    for (let i = 0; i < M.length; i++) if (M[i] && D[i]! > best) { best = D[i]!; at = i; }
    worst.push(`${name} ${(best / k * 1000).toFixed(0)}mm @(${(b.minX + (at % px) / k).toFixed(3)}, ${(b.maxY - Math.floor(at / px) / k).toFixed(3)})`);
  }
  console.log(`${out}.png iou ${x.iou.toFixed(3)} haus ${(x.hausdorffM * 1000).toFixed(1)} mm  ${worst.join("  ")}`);
}
