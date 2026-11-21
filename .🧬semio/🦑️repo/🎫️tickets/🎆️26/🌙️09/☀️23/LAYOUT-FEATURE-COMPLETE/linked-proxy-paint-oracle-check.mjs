/** 🎨️ Checks the visible linked-proxy detector with pixels from the authored Linked Asset. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import sharp from "sharp";
import { linkedProxyColors, countLinkedProxyColors } from "../../☀️16/LAYOUT-PLUGIN-END-TO-END/browser-physical-interactions.mjs";

const linked = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🔗️linked-asset/🗣️.dsl.semio";
const encoded = readFileSync(linked, "utf8").match(/proxy-data-url="data:image\/png;base64,([A-Za-z0-9+/=]+)"/)[1];
const source = Buffer.from(encoded, "base64");
const colors = await linkedProxyColors(source);
assert.equal(colors.length, 2);
const proxy = await sharp(source).resize(64, 64, { kernel: "nearest" }).png().toBuffer();
const capture = overlays => sharp({
  create: { width: 160, height: 96, channels: 3, background: "#ffffff" },
}).composite(overlays).removeAlpha().raw().toBuffer({ resolveWithObject: true });
const original = { input: proxy, left: 0, top: 16 };
const placed = { input: proxy, left: 96, top: 16 };
const before = await capture([original]);
const after = await capture([original, placed]);
const target = [96, 16, 64, 64];
assert.deepEqual(countLinkedProxyColors(before, [0, 0, 160, 96], colors, 16), [1040, 720]);
assert.deepEqual(countLinkedProxyColors(before, target, colors, 16, before), [0, 0]);
assert.deepEqual(countLinkedProxyColors(after, target, colors, 16, before), [1040, 720]);
assert.deepEqual(countLinkedProxyColors(after, [0, 16, 64, 64], colors, 16, before), [0, 0]);
console.log("[DEBUG] Linked-proxy pixel oracle passed four authored-image cases");
