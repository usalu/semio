/** 🎨️ Measures the painted fixture in the canvas bitmap. */
import { createRequire } from "node:module";
import { resolve } from "node:path";

const require = createRequire(resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs"));
const { createCanvas, loadImage } = require("@napi-rs/canvas");

export async function paintExtent(png, { color, tolerance }) {
  const image = await loadImage(png);
  const context = createCanvas(image.width, image.height).getContext("2d");
  context.drawImage(image, 0, 0);
  const { data } = context.getImageData(0, 0, image.width, image.height);
  let pixels = 0, left = image.width, top = image.height, right = -1, bottom = -1;
  for (let index = 0; index < data.length; index += 4) {
    if (data[index + 3] !== 255 || !color.every((value, channel) => Math.abs(data[index + channel] - value) <= tolerance)) continue;
    const x = index / 4 % image.width, y = Math.floor(index / 4 / image.width);
    pixels++;
    left = Math.min(left, x); right = Math.max(right, x);
    top = Math.min(top, y); bottom = Math.max(bottom, y);
  }
  return { bitmap: [image.width, image.height], pixels, left, top, width: Math.max(0, right - left + 1), height: Math.max(0, bottom - top + 1) };
}

export function hasZoomed(before, after) {
  return before.bitmap.every((size, axis) => size === after.bitmap[axis]) &&
    25 * after.pixels >= 27 * before.pixels && 20 * after.width >= 21 * before.width && 20 * after.height >= 21 * before.height;
}
