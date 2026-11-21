import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import sharp from "sharp";

const wire = JSON.parse(readFileSync("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-png/🧪️tests/fixture.json", "utf8"));
const image = JSON.parse(readFileSync("✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎞️reserved-import/🧪️structured-wire/image-cases.json", "utf8")).cases[0];
for (const { name, payload, offset } of wire.cases) {
  const uri = new URL(payload);
  const [type, base64] = uri.pathname.split(",");
  const accepted = (type === "image/png;base64" || type === "application/octet-stream;base64") && Boolean(base64);
  assert.equal(accepted ? payload.indexOf(",") + 1 : null, offset, name);
  if (accepted) assert.equal(Buffer.from(base64, "base64").toString("hex"), "89504e470d0a1a0a");
}
const { data, info } = await sharp(Buffer.from(image.base64, "base64")).ensureAlpha().raw().toBuffer({ resolveWithObject: true });
assert.deepEqual([info.width, info.height, info.channels], [1, 1, 4]);
assert.deepEqual(Array.from(data), [10, 20, 30, 255]);
assert.ok(Math.ceil(wire.picker.maxFileBytes / 3) * 4 + 37 < 4 * 1024 * 1024);
console.log(`[DEBUG] PNG picker oracle passed ${wire.cases.length} wire cases and independent pixel decoding.`);
