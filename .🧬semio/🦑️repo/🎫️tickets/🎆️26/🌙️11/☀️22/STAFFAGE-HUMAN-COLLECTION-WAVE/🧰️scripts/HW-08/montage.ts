/** 🖼️ Rasterises several SVGs with sharp and lays them side by side into one PNG: bun montage.ts out.png a.svg b.svg … [--dpi 120] */
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";
const args = process.argv.slice(2);
const at = args.indexOf("--dpi"); const dpi = at >= 0 ? Number(args[at + 1]) : 120;
const [out, ...files] = args.filter((x, i) => x !== "--dpi" && args[i - 1] !== "--dpi");
const imgs = await Promise.all(files.map(async (f) => { const b = await sharp(f, { density: dpi }).flatten({ background: "#ffffff" }).png().toBuffer(); const m = await sharp(b).metadata(); return { b, w: m.width!, h: m.height! }; }));
const W = imgs.reduce((s, i) => s + i.w, 0), H = Math.max(...imgs.map((i) => i.h));
let x = 0; const comp = imgs.map((i) => { const c = { input: i.b, left: x, top: 0 }; x += i.w; return c; });
await sharp({ create: { width: W, height: H, channels: 3, background: "#ffffff" } }).composite(comp).png().toFile(out!);
console.log(out, W, H);
