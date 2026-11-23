/** 🖼️ Finisher contact sheet of one asset file: the default pose as a third-angle sheet (plan, side elevation, front) at every authored scale and the axonometric view of the 1:50 volume, each magnified to the same tile width, three tiles to a row, in one PNG `<out>/<id>.png` (sharp as rasteriser, `rsvg-convert` is not installed on this machine).
 *
 * `repo tools exec bun -- sheets.ts <asset document json> <out dir> [--view light-solid] [--pose id] [--no-axonometric]`
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts */
import { mkdirSync, readFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import sharp from "sharp";
import { runPreview } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const args = process.argv.slice(2);
const flag = (name: string) => (args.includes(name) ? args[args.indexOf(name) + 1] : undefined);
const [file, out] = [args[0]!, args[1]!];
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const view = flag("--view") ?? "light-solid";
const pose = flag("--pose");
const TILE = 620;
const tmp = join(out, `${asset.id}.svg-tmp`);
mkdirSync(tmp, { recursive: true });
const none = { async rasterize() {} };
const tiles: { label: string; png: Buffer; height: number }[] = [];
const add = async (label: string, rest: string[]) => {
  try {
    const paths = await runPreview(["--file", file, "--view", view, ...(pose ? ["--pose", pose] : []), "--language", "en", "--no-png", "--flow", "none", "--out", tmp, ...rest], none as never);
    for (const path of paths.filter((p: string) => p.endsWith(".svg"))) {
      const svg = readFileSync(path);
      const meta = await sharp(svg, { density: 72 }).metadata();
      const density = Math.max(72, Math.min(9600, (72 * TILE) / (meta.width ?? TILE)));
      const png = await sharp(svg, { density }).resize({ width: TILE }).flatten({ background: "#ffffff" }).png().toBuffer();
      tiles.push({ label, png, height: (await sharp(png).metadata()).height ?? 0 });
    }
  } catch (error) {
    console.log(`[DEBUG] ${asset.id} ${label}: ${(error as Error).message.slice(0, 300)}`);
  }
};
for (const scale of ["50", "100", "200", "500"]) {
  if (asset.representations[scale].status !== "authored") { console.log(`[DEBUG] ${asset.id} 1:${scale} ${asset.representations[scale].status}`); continue; }
  await add(`1:${scale} plan / side / front`, ["--scale", scale, "--views", "elevation,front,plan", "--projection", "third"]);
}
if (!args.includes("--no-axonometric")) await add("1:50 axonometric volume", ["--scale", "50", "--viewpoint", "axonometric"]);
const BAR = 40;
const COLUMNS = 3;
const rows: (typeof tiles)[] = [];
for (let i = 0; i < tiles.length; i += COLUMNS) rows.push(tiles.slice(i, i + COLUMNS));
const total = rows.reduce((sum, row) => sum + Math.max(...row.map((t) => t.height)) + BAR, 0);
let y = 0;
const layers: sharp.OverlayOptions[] = [];
for (const row of rows) {
  row.forEach((t, k) => {
    layers.push({ input: Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="${TILE}" height="${BAR}"><rect width="${TILE}" height="${BAR}" fill="#222"/><text x="10" y="28" font-family="Helvetica" font-size="22" fill="#fff">${asset.id} ${t.label}</text></svg>`), left: k * TILE, top: y });
    layers.push({ input: t.png, left: k * TILE, top: y + BAR });
  });
  y += Math.max(...row.map((t) => t.height)) + BAR;
}
const target = join(out, `${asset.id}${pose ? `.${pose}` : ""}.png`);
await sharp({ create: { width: TILE * COLUMNS, height: Math.max(1, total), channels: 3, background: "#ffffff" } }).composite(layers).png().toFile(target);
rmSync(tmp, { recursive: true, force: true });
console.log(`[DEBUG] ${target} ${TILE * COLUMNS}x${total} (${tiles.length} tiles)`);
