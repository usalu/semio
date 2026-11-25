/** 🔍️ An enlarged working picture of what an asset document draws against its body volume, for scales whose product sheet is too small to judge: `bun zoom.ts <id|file.json> <scale> <subject text|default> <out.png> [bone-regex]` evaluates the default pose, the pose named as `pose <id>` or else the first subject of the matrix whose id contains the text (that enumeration probes every clip and is slow for a heavy asset) and writes one row of three panels (side elevation, front, plan): the triangles of the body volume of the scale in grey, every drawn shape as a blue outline with a light fill, the shapes of the bones matching the regex in red. Each panel is fitted to the extent of volume and drawing, with a 0.1 m grid; `FOCUS=<metres>` fits each panel to the red shapes plus that margin instead, and `ONLY=1` draws nothing but the volume solids and the shapes of the matching bones. Run it from the suite folder `T`. */
import { join } from "node:path";
import sharp from "sharp";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const [arg, key, text, out, pattern] = process.argv.slice(2);
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { worldSolids, project } = await import(join(S, "🔨️modules", "🗂️catalog", "🧊️volumes", "🌓️silhouette", "🟦️.ts"));
const asset = readDoc(arg!).doc.asset;
const pose = text === "default" ? asset.presentation.defaultPose : asset.poses.find((candidate: any) => `pose ${candidate.id}` === text)?.id;
const subject = pose !== undefined ? { id: `pose ${pose}`, query: { kind: "pose", pose } } : engine.subjects(asset, Number(key)).find((candidate: any) => candidate.id.includes(text!));
if (subject === undefined) throw new Error(`no subject "${text}"`);
const state = evaluate(asset, subject.query, engine.scaleSelector(Number(key)));
const worlds = worldSolids(asset, state.bones, Number(key)) as any[];
const marked = pattern === undefined ? undefined : new RegExp(pattern);
const only = process.env.ONLY !== undefined && marked !== undefined;
const SIZE = 900;
const panels: string[] = [];
for (const [index, view] of (["elevation", "front", "plan"] as const).entries()) {
  const triangles: { x: number; y: number }[][] = [];
  for (const world of worlds) {
    if (world.texture === true || (only && !marked!.test(world.bone ?? ""))) continue;
    const at = (i: number) => project(view, world.positions[3 * i], world.positions[3 * i + 1], world.positions[3 * i + 2]);
    for (let i = 0; i < world.indices.length; i += 3) triangles.push([at(world.indices[i]), at(world.indices[i + 1]), at(world.indices[i + 2])]);
  }
  const drawn = (state.shapes[view] as any[]).filter((placed) => placed.shape.role !== "ground" && (!only || marked!.test(placed.shape.bone ?? "")));
  const focus = process.env.FOCUS !== undefined && marked !== undefined ? drawn.filter((placed) => marked.test(placed.shape.bone ?? "")).flatMap((placed) => placed.rings.flat()) : [];
  const points = focus.length > 0 ? focus : [...triangles.flat(), ...drawn.flatMap((placed) => placed.rings.flat())];
  const margin = focus.length > 0 ? Number(process.env.FOCUS) : 0;
  const [x0, x1, y0, y1] = [Math.min(...points.map((p) => p.x)) - margin, Math.max(...points.map((p) => p.x)) + margin, Math.min(...points.map((p) => p.y)) - margin, Math.max(...points.map((p) => p.y)) + margin];
  const zoom = (SIZE - 60) / Math.max(x1 - x0, y1 - y0, 0.01);
  const px = (p: { x: number; y: number }): string => `${(30 + (p.x - x0) * zoom).toFixed(1)},${(SIZE - 30 - (p.y - y0) * zoom).toFixed(1)}`;
  const grid: string[] = [];
  for (let g = Math.ceil(x0 * 10) / 10; g <= x1; g += 0.1) grid.push(`<line x1="${30 + (g - x0) * zoom}" y1="0" x2="${30 + (g - x0) * zoom}" y2="${SIZE}" stroke="#ddd" stroke-width="${Math.abs(g) < 1e-9 ? 2 : 0.6}"/>`);
  for (let g = Math.ceil(y0 * 10) / 10; g <= y1; g += 0.1) grid.push(`<line x1="0" y1="${SIZE - 30 - (g - y0) * zoom}" x2="${SIZE}" y2="${SIZE - 30 - (g - y0) * zoom}" stroke="#ddd" stroke-width="${Math.abs(g) < 1e-9 ? 2 : 0.6}"/>`);
  const body = triangles.map((t) => `<polygon points="${t.map(px).join(" ")}" fill="#000" fill-opacity="0.16" stroke="none"/>`).join("");
  const shapes = drawn
    .map((placed) => {
      const red = marked !== undefined && marked.test(placed.shape.bone ?? "");
      return placed.rings.map((ring: any[]) => `<${placed.closed === false ? "polyline" : "polygon"} points="${ring.map(px).join(" ")}" fill="${placed.closed === false ? "none" : red ? "#e00" : "#06c"}" fill-opacity="0.12" stroke="${red ? "#e00" : "#06c"}" stroke-width="${red ? 1.6 : 0.9}"/>`).join("");
    })
    .join("");
  panels.push(`<g transform="translate(${index * SIZE},0)" clip-path="url(#panel)"><rect width="${SIZE}" height="${SIZE}" fill="#fff" stroke="#999"/>${grid.join("")}${body}${shapes}<text x="12" y="22" font-family="Helvetica" font-size="18">${view} · 1:${key} · ${subject.id} · zoom ${zoom.toFixed(0)} px/m</text></g>`);
}
const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${SIZE * 3}" height="${SIZE}"><defs><clipPath id="panel"><rect width="${SIZE}" height="${SIZE}"/></clipPath></defs>${panels.join("")}</svg>`;
await sharp(Buffer.from(svg)).png().toFile(out!);
console.log(out);
