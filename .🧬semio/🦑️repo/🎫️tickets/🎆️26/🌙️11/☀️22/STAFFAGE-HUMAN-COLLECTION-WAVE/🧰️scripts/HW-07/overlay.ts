/** 🔍️ Overlay of the drawing (blue) and the body volume (red) of one view, scale and pose, as a PNG (sharp), to see where the two disagree; purple is agreement. bun overlay.ts <file> <pose> <scale> <view> <out.png> */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";
const [f, pose, sc, view, out] = process.argv.slice(2);
const a = (await Bun.file(f!).json()).asset;
const st: any = evaluate(a, { kind: "pose", pose } as any, { kind: "scale", scale: { denominator: Number(sc) } } as any);
const proj = (x: number, y: number, z: number): [number, number] => (view === "elevation" ? [x, y] : view === "front" ? [z, y] : [x, z]);
const drawn = st.shapes[view!].filter((e: any) => e.shape.role !== "ground" && e.shape.filled && e.closed).flatMap((e: any) => e.rings.map((r: any) => r.map((p: any) => [p.x, p.y])));
const tris: [number, number][][] = [];
for (const w of worldSolids(a, st.bones, Number(sc)) as any[]) for (let i = 0; i < w.indices.length; i += 3) tris.push([0, 1, 2].map((k) => { const j = w.indices[i + k] * 3; return proj(w.positions[j], w.positions[j + 1], w.positions[j + 2]); }));
const all = [...drawn.flat(), ...tris.flat()];
const minX = Math.min(...all.map((p: any) => p[0])) - 0.05, maxX = Math.max(...all.map((p: any) => p[0])) + 0.05, minY = Math.min(...all.map((p: any) => p[1])) - 0.05, maxY = Math.max(...all.map((p: any) => p[1])) + 0.05;
const k = 900 / Math.max(maxX - minX, maxY - minY);
const P = (p: number[]) => `${((p[0]! - minX) * k).toFixed(1)},${((maxY - p[1]!) * k).toFixed(1)}`;
const W = Math.ceil((maxX - minX) * k), H = Math.ceil((maxY - minY) * k);
const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}"><rect width="100%" height="100%" fill="white"/><g fill="red" fill-opacity="0.45">${tris.map((t) => `<polygon points="${t.map(P).join(" ")}"/>`).join("")}</g><path fill="blue" fill-opacity="0.35" fill-rule="evenodd" d="${drawn.map((r: any) => "M" + r.map(P).join("L") + "Z").join("")}"/><g stroke="black" stroke-width="1">${Array.from({ length: Math.ceil((maxX - minX) / 0.1) }, (_, i) => `<line x1="${(i * 0.1 * k).toFixed(0)}" y1="0" x2="${(i * 0.1 * k).toFixed(0)}" y2="6"/>`).join("")}</g></svg>`;
await sharp(Buffer.from(svg)).png().toFile(out!);
console.log(out, `x ${minX.toFixed(2)}..${maxX.toFixed(2)} y ${minY.toFixed(2)}..${maxY.toFixed(2)}`);
