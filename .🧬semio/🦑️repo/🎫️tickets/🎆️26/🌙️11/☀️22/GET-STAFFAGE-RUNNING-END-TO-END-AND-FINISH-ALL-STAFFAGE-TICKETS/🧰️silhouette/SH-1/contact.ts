/** 🗂️ Contact sheets for looking at repaired people: per person one row with the sheet before and the sheet after at one scale, several people per PNG (a tool of the ticket; sharp composes).
 *  bun contact.ts <look dir> <scale> <out prefix> <per sheet> <slug>… */
import { readdirSync } from "node:fs";
import sharp from "/Users/ueli/Documents/semio/node_modules/sharp/lib/index.js";

const [dir, scale, prefix, perArg, ...slugs] = process.argv.slice(2) as string[];
const per = Number(perArg), cellW = 760, cellH = 430;
const pick = (slug: string, tag: string): string | undefined => { try { const f = readdirSync(`${dir}/${slug}/${tag}`).find((n) => n.endsWith(".png") && n.endsWith(`1-${scale}.png`)); return f === undefined ? undefined : `${dir}/${slug}/${tag}/${f}`; } catch { return undefined; } };
for (let page = 0; page * per < slugs.length; page++) {
  const rows = slugs.slice(page * per, (page + 1) * per);
  const parts: { input: Buffer; left: number; top: number }[] = [];
  for (const [r, slug] of rows.entries()) for (const [c, tag] of ["before", "after"].entries()) {
    const file = pick(slug, tag);
    if (file === undefined) continue;
    parts.push({ input: await sharp(file).resize({ width: cellW - 10, height: cellH - 10, fit: "inside", background: "#ffffff" }).flatten({ background: "#ffffff" }).png().toBuffer(), left: c * cellW + 5, top: r * cellH + 5 });
  }
  const out = `${prefix}-${scale}-${page + 1}.png`;
  await sharp({ create: { width: 2 * cellW, height: rows.length * cellH, channels: 3, background: "#ffffff" } }).composite(parts).png().toFile(out);
  console.log(`${out}: ${rows.join(", ")}`);
}
