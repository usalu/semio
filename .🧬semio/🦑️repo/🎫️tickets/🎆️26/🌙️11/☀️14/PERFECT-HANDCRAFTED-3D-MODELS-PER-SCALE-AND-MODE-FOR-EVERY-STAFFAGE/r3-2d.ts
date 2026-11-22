/** ✏️ Renders the three 2D drawings of one scale alone (side elevation, front elevation, plan), exactly as the product draws them: `evaluate`, `depict` and the SVG export of the staffage modules, rasterised by sharp (librsvg). Two rows: the line view with every stroke, pen and hatch, and the solid view with the tones.
 *
 * Usage: `bun r3-2d.ts <category>/<slug> --scale N [--pose ID] [--clip ID --t S] [--file <asset document json>] [--views light-line,light-solid] [--svg]`
 * Output: `🗑️generated/r3-2d/<category>/<slug>.s<N>[.<pose>|.<clip>-t<S>][.<view>+<view>…].png` (the last part only with `--views`; the `--file` form carries the same suffixes under `r3-2d/file/<folder name of the file>`) (and the SVG of every panel next to it with `--svg`). `--file` draws a candidate asset document as it is instead of the catalog asset.
 * @see ./r3-tools.md
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { basename, dirname, resolve } from "node:path";
import sharp from "sharp";

import { defaultDepictionOptions, depict } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { writeSvg } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🖼️svg/🟦️.ts";
import { VIEWS, loadAsset, type Json } from "./volumes-lib.ts";
import { cli, load, queryOf, r3Out } from "./r3-lib.ts";

const USAGE = "r3-2d.ts <category>/<slug> --scale N [--pose ID] [--clip ID --t S] [--file <asset document json>] [--views light-line,light-solid] [--svg]";

/** 🖼️ One product drawing as SVG text: the given depiction view of a viewpoint at a scale, without legend. */
export function drawingSvg(asset: Json, scale: number, query: Json, view: string, viewpoint: string): string {
  const selector = { kind: "scale", scale: { denominator: scale } };
  const state = evaluate(asset as never, query as never, selector as never);
  const o = defaultDepictionOptions(view as never, "en", scale, viewpoint as never) as Json;
  return writeSvg(depict(state, { ...o, representation: selector, annotations: { ...o.annotations, legend: false } } as never, {}));
}

if (import.meta.main) {
  const { spec, flag, has } = cli(["--scale", "--pose", "--clip", "--t", "--file", "--views"]);
  const file = flag("--file");
  const loaded = file === undefined ? load(spec, USAGE) : { entry: { category: "file", folder: "file", slug: basename(dirname(resolve(file))), path: resolve(file) }, asset: loadAsset(resolve(file)) };
  const { entry, asset } = loaded;
  const scale = Number(flag("--scale"));
  if (asset.representations[String(scale)]?.status !== "authored") {
    process.stderr.write(`scale ${flag("--scale")} has no authored representation (usage: ${USAGE})\n`);
    process.exit(2);
  }
  const query = queryOf(asset, flag("--pose"), flag("--clip"), flag("--t"));
  const rows = (flag("--views") ?? "light-line,light-solid").split(",");
  const [pw, ph, top] = [800, 700, 30];
  const tag = query.kind === "pose" ? (flag("--pose") === undefined ? "" : `.${query.pose}`) : `.${query.clip}-t${query.timeS}`;
  const path = r3Out("r3-2d", entry as never, `${entry.slug}.s${scale}${tag}${flag("--views") === undefined ? "" : `.${rows.join("+")}`}.png`);
  mkdirSync(dirname(path), { recursive: true });
  const parts: { input: Buffer; left: number; top: number }[] = [];
  let failed = 0;
  for (const [r, view] of rows.entries()) for (const [c, viewpoint] of VIEWS.entries()) {
    let svg: string;
    try {
      svg = drawingSvg(asset, scale, query, view, viewpoint);
    } catch (error) {
      failed++;
      const message = String((error as Error).message).replace(/[<&>]/gu, " ").slice(0, 180);
      svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${pw}" height="${ph}"><text x="12" y="60" font-size="15" fill="#b00">${viewpoint} ${view} FAILED:</text><text x="12" y="84" font-size="11" fill="#b00">${message}</text></svg>`;
      process.stderr.write(`${viewpoint} ${view}: ${(error as Error).message}\n`);
    }
    if (has("--svg")) writeFileSync(path.replace(/\.png$/u, `.${viewpoint}.${view}.svg`), svg);
    const panel = await sharp(Buffer.from(svg), { density: 600, limitInputPixels: false }).resize(pw - 16, ph - top - 12, { fit: "contain", background: "#ffffff" }).flatten({ background: "#ffffff" }).png().toBuffer();
    const label = Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="${pw}" height="${top}"><rect width="${pw}" height="${top}" fill="#eeeeea"/><text x="10" y="21" font-family="Helvetica, Arial, sans-serif" font-size="17" fill="#222">${viewpoint === "elevation" ? "side elevation (elevation, seen from the right)" : viewpoint === "front" ? "front elevation (front, seen from ahead)" : "plan (seen from above, ahead = right)"} · ${view}</text></svg>`);
    parts.push({ input: label, left: c * pw, top: r * ph }, { input: panel, left: c * pw + 8, top: r * ph + top + 6 });
  }
  const what = query.kind === "pose" ? `pose ${query.pose}` : `clip ${query.clip} t ${query.timeS} s`;
  const footer = Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="${pw * 3}" height="34"><text x="10" y="23" font-family="Helvetica, Arial, sans-serif" font-size="17" fill="#222">${entry.category}/${entry.slug} · 1:${scale} · ${what} · the product's own 2D drawings (evaluate, depict, SVG export), each panel fitted to its frame (panels are not at one common zoom)</text></svg>`);
  parts.push({ input: footer, left: 0, top: rows.length * ph });
  await sharp({ create: { width: pw * 3, height: rows.length * ph + 34, channels: 3, background: "#ffffff" } }).composite(parts).png().toFile(path);
  process.stdout.write(`${path}\n`);
  process.exitCode = failed > 0 ? 1 : 0;
}
