/** 🖼️ Tiles panel crops of `r3-sheet.ts` sheets into one image: `bun rework-standing-adult-montage.ts <out.png> <cropWidth> <columns> <sheet.png>:<panel,panel> …` (panels front, right, back, left, top, quarter). */
import sharp from "sharp";
const [out, widthText, columnsText, ...entries] = process.argv.slice(2);
const width = Number(widthText), columns = Number(columnsText), height = 700;
const origin: Record<string, [number, number]> = { front: [0, 24], right: [800, 24], back: [1600, 24], left: [0, 764], top: [800, 764], quarter: [1600, 764] };
const tiles: { input: Buffer; left: number; top: number }[] = [];
for (const entry of entries) {
  const [file, panels] = entry.split(":");
  for (const panel of panels!.split(",")) {
    const [x, y] = origin[panel]!;
    const input = await sharp(file).extract({ left: x + (800 - width) / 2, top: y, width, height }).png().toBuffer();
    tiles.push({ input, left: (tiles.length % columns) * width, top: Math.floor(tiles.length / columns) * height });
  }
}
await sharp({ create: { width: width * columns, height: height * Math.ceil(tiles.length / columns), channels: 3, background: "#ffffff" } }).composite(tiles).png().toFile(out!);
console.log(out);
