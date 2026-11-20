import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import sharp from "sharp";

const law = JSON.parse(readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🎟️png-texture/🧪️tests/cases.json", "utf8"));
const original = Buffer.from(law.data, "base64");
const { data, info } = await sharp(original).ensureAlpha().raw().toBuffer({ resolveWithObject: true });
assert.deepEqual([info.width, info.height, info.channels], [law.width, law.height, 4]);
assert.deepEqual([...data], law.rgba);
const invalidSignature = Buffer.from(original);
invalidSignature[0] ^= 0xff;
await assert.rejects(() => sharp(invalidSignature).raw().toBuffer());
console.log("Sharp matched the neutral embedded PNG pixel and invalid signature laws");
