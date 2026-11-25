/** 🖼️ Stacks two rendered sheets into one picture, the first on top: `bun pair.ts <top.png> <bottom.png> <out.png>` (run from the folder of the 3D ticket, whose `sharp` it uses), so a drawing before and after a repair is looked at side by side at the full width of the sheet. */
const sharp: any = (await import(Bun.resolveSync("sharp", `${process.cwd()}/`))).default;
const [top, bottom, out] = process.argv.slice(2);
const a = sharp(top!), b = sharp(bottom!);
const [ma, mb] = [await a.metadata(), await b.metadata()];
await sharp({ create: { width: Math.max(ma.width, mb.width), height: ma.height + mb.height, channels: 3, background: "#ffffff" } })
  .composite([{ input: await a.toBuffer(), left: 0, top: 0 }, { input: await b.toBuffer(), left: 0, top: ma.height }])
  .png()
  .toFile(out!);
console.log(out);
