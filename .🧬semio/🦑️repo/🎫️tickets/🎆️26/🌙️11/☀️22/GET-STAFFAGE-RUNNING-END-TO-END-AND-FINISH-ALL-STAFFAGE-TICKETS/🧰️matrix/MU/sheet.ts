/** 🖼️ Contact sheet of product drawings as the matrix evaluates them: `bun sheet.ts <out.png> [--views light-line,light-solid] [--panel 520x380] <id|file.json>|<scale>|<query>…` with a query `pose:<id>`, `clip:<id>@<seconds>` or `trans:<from>><to>@<fraction>`; one row per argument with side elevation, front and plan in each depiction view.
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-2d.ts — the renderer this one follows */
import { join } from "node:path";
import sharp from "sharp";
import { S, readDoc } from "../MR/lib.ts";

const { defaultDepictionOptions, depict } = await import(join(S, "🔨️modules", "🎨️depiction", "🟦️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { writeSvg } = await import(join(S, "🔨️modules", "📤️export", "🖼️svg", "🟦️.ts"));
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => {
  const i = args.indexOf(`--${name}`);
  return i < 0 ? undefined : args.splice(i, 2)[1];
};
const views = (flag("views") ?? "light-solid").split(",");
const [pw, ph] = (flag("panel") ?? "560x400").split("x").map(Number) as [number, number];
const [out, ...rows] = args as [string, ...string[]];
const label = (text: string, width: number): Buffer => Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="22"><rect width="${width}" height="22" fill="#eeeeea"/><text x="6" y="16" font-family="Helvetica" font-size="13" fill="#222">${text.replace(/[<&>]/gu, "›")}</text></svg>`);
const parts: { input: Buffer; left: number; top: number }[] = [];
const names = ["elevation", "front", "plan"];
const width = pw * 3 * views.length;
for (const [r, row] of rows.entries()) {
  const [doc, scale, query] = row.split("|") as [string, string, string];
  const asset = readDoc(doc).doc.asset;
  const m = /^(pose|clip|trans):(.*)$/u.exec(query)!;
  const at = m[2]!.lastIndexOf("@");
  const [name, t] = m[1] === "pose" ? [m[2]!, 0] : [m[2]!.slice(0, at), Number(m[2]!.slice(at + 1))];
  const q = m[1] === "pose" ? { kind: "pose", pose: name } : m[1] === "clip" ? { kind: "clip", clip: name, timeS: t } : { kind: "transition", from: name.split(">")[0], to: name.split(">")[1], durationS: 1, timeS: t };
  const selector = { kind: "scale", scale: { denominator: Number(scale) } };
  parts.push({ input: label(`${doc.split("/").at(-1)} 1:${scale} ${query}`, width), left: 0, top: r * (ph + 22) });
  const state = evaluate(asset, q, selector);
  for (const [k, view] of views.entries())
    for (const [c, viewpoint] of names.entries()) {
      const o = defaultDepictionOptions(view, "en", Number(scale), viewpoint);
      const svg = writeSvg(depict(state, { ...o, representation: selector, annotations: { ...o.annotations, legend: false } }, {}));
      const input = await sharp(Buffer.from(svg), { density: 400, limitInputPixels: false }).resize(pw - 8, ph - 8, { fit: "contain", background: "#ffffff" }).flatten({ background: "#ffffff" }).png().toBuffer();
      parts.push({ input, left: (k * 3 + c) * pw + 4, top: r * (ph + 22) + 24 });
    }
}
await sharp({ create: { width, height: rows.length * (ph + 22), channels: 3, background: "#ffffff" }, limitInputPixels: false }).composite(parts).png().toFile(out);
console.log(out);
