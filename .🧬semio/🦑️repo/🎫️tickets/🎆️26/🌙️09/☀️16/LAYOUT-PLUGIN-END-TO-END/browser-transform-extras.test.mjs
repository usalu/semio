import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";
import { mat2d, vec2 } from "gl-matrix";
import { expectedTransformedGeometry } from "./browser-transform-extras.mjs";

const fixture = JSON.parse(readFileSync(new URL("./browser-transform-extras-cases.json", import.meta.url)));
const almostEqual = (actual, expected) => {
  for (const key of Object.keys(expected)) {
    assert.ok(Math.abs(actual[key] - expected[key]) < 0.00001, key + " expected " + expected[key] + ", read " + actual[key]);
  }
};

for (const { kind, before, ticks, expected } of fixture.cases) {
  test(kind + " inspector geometry agrees with independent affine matrices", () => {
    const matrix = mat2d.create();
    if (kind === "resize") {
      for (const tick of ticks) mat2d.scale(matrix, matrix, [tick.sx, tick.sy]);
    } else if (kind === "rotate") {
      for (const tick of ticks) mat2d.rotate(matrix, matrix, tick.angle);
    }
    const horizontal = vec2.transformMat2d(vec2.create(), [before.width, 0], matrix);
    const vertical = vec2.transformMat2d(vec2.create(), [0, before.height], matrix);
    const width = vec2.length(horizontal);
    const height = vec2.length(vertical);
    const independentlyTransformed = {
      x: before.x + (before.width - width) / 2,
      y: before.y + (before.height - height) / 2,
      width,
      height,
      rotation: before.rotation + Math.atan2(horizontal[1], horizontal[0]),
    };
    almostEqual(independentlyTransformed, expected);
    almostEqual(expectedTransformedGeometry(kind, before, ticks), expected);
  });
}
