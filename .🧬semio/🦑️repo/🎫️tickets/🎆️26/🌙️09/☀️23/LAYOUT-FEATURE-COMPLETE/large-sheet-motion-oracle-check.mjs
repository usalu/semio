/** 🔍️ Checks that the large-sheet viewport oracle compares the entire exported document. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { JSDOM } from "jsdom";

const journey = await import("../../☀️16/LAYOUT-PLUGIN-END-TO-END/browser-large-sheet.mjs");
const { largeSheetPreview } = JSON.parse(readFileSync(new URL("../../☀️16/LAYOUT-PLUGIN-END-TO-END/browser-journey.json", import.meta.url), "utf8"));
const fixture = JSON.parse(readFileSync(largeSheetPreview.fixture, "utf8"));
const { width, height, frames } = fixture.document.pages[0];
const paths = frames.map(({ bounds: { x, y, w, h } }) =>
  '<path fill="black" d="M' + x + ' ' + y + ' L' + (x + w) + ' ' + y + ' L' +
    (x + w) + ' ' + (y + h) + ' L' + x + ' ' + (y + h) + ' Z"/>',
);
const svg = content => Buffer.from('<svg xmlns="http://www.w3.org/2000/svg" width="' +
  width + '" height="' + height + '" viewBox="0 0 ' + width + ' ' + height + '">' + content.join("") + "</svg>");
const before = svg(paths);
assert.equal(journey.verifyLargeSheetSvg(before, fixture).rectangleCount, frames.length);
assert.equal(journey.verifyLargeSheetUnchanged(before, svg([...paths])), true);
for (const changed of [
  paths.slice(1),
  [...paths, paths[0]],
  [paths[1], paths[0], ...paths.slice(2)],
  [paths[0].replace('fill="black"', 'fill="red"'), ...paths.slice(1)],
]) {
  const next = svg(changed);
  const { window } = new JSDOM(next.toString("utf8"), { contentType: "image/svg+xml" });
  try {
    assert.equal(window.document.documentElement.localName, "svg");
    assert.equal(window.document.querySelectorAll("path").length, changed.length);
  } finally {
    window.close();
  }
  assert.throws(() => journey.verifyLargeSheetUnchanged(before, next), /keeps the full SVG/);
}
console.log("[DEBUG] Large-sheet SVG motion oracle passed five neutral cases");
