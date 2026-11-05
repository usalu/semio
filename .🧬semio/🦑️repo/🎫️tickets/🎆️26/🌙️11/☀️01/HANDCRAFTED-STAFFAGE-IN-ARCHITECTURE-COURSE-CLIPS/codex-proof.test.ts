import { mkdir, readFile, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { it, expect } from "vitest";
import { exportCast } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";
import { createCanvas, loadImage } from "./🗑️generated/t/node_modules/@napi-rs/canvas/index.js";

it("exports course elevation, plan and thermal casts with physical provenance", async () => {
  const root = new URL("../../../../../../../", import.meta.url);
  const manifest = JSON.parse(await readFile(new URL("🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json", root), "utf8"));
  const ids = ["shared-person-relaxed-line", "excel-program-occupant", "shared-radiator-front-winter"];
  const out = fileURLToPath(new URL("./🗑️generated/proof/", import.meta.url));
  const indexes = await exportCast(manifest, ids, out);
  const canvas = createCanvas(1200, 640), context = canvas.getContext("2d");
  context.fillStyle = "#161b22";
  context.fillRect(0, 0, 1200, 640);
  for (const [i, index] of indexes.entries()) {
    expect(index.provenance.asset.id).toBe(manifest.casts[index.cast].asset);
    expect(index.provenance.scale.denominator).toBe(50);
    const image = await loadImage(await readFile(`${out}/${index.cast}/${index.frames[0]}`));
    const scale = Math.min(350 / image.width, 500 / image.height);
    context.drawImage(image, 400 * i + (400 - image.width * scale) / 2, 40 + (500 - image.height * scale) / 2, image.width * scale, image.height * scale);
    context.fillStyle = "#ffffff";
    context.font = "17px sans-serif";
    context.fillText(index.cast, 400 * i + 16, 590, 368);
    context.fillText(`${index.viewpoint} · 1:50 · ${index.heightM.toFixed(2)} m`, 400 * i + 16, 620, 368);
  }
  await mkdir(out, { recursive: true });
  await writeFile(`${out}/proof.png`, await canvas.encode("png"));
}, 120000);
