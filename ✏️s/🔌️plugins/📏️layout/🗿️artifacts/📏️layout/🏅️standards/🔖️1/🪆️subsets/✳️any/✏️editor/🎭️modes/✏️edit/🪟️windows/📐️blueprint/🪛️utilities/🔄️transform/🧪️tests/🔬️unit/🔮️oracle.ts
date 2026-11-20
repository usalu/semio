/** 🔮️ Independent transform gesture oracle for the shared dispatch cases. */
import { expect, test } from "bun:test";
import { mat2d, vec2 } from "gl-matrix";
import isEqual from "lodash/isEqual";
import reduce from "lodash/reduce";
import sum from "lodash/sum";
import fixture from "./🔣️.json";

for (const row of fixture.cases) {
  test(row.name, () => {
    const kind = row.open[0];
    const openingScope = ["page-1", "frame-1", kind, 0];
    const tailScope = row.tail === null
      ? openingScope
      : [row.page ?? "page-1", row.target ?? "frame-1", row.tail[0], row.pivot ?? 0];
    const factors = [Number(row.open[1]), ...(row.tail === null ? [] : [Number(row.tail[1])])];
    const net = kind === "scale" ? reduce(factors, (product, factor) => product * factor, 1) : sum(factors);
    const moves = kind === "scale" ? net > 0 && net !== 1 : net !== 0;
    const eligible = row.phase === "commit" && !row.locked && !row.missing && (row.revision ?? "rev-1") === "rev-1";
    expect(eligible && isEqual(openingScope, tailScope) && moves ? net : null).toBe(row.committed);
  });
}

for (const row of fixture.previews) {
  test(row.name, () => {
    const [x, y, width, height, rotation] = fixture.previewBase.map(Number) as [number, number, number, number, number];
    const [kind, first, second] = row.motion;
    const matrix = mat2d.create();
    let nextWidth = width;
    let nextHeight = height;
    let nextRotation = rotation;
    switch (kind) {
      case "drag":
        mat2d.translate(matrix, matrix, [Number(first), Number(second)]);
        break;
      case "rotate":
        mat2d.rotate(matrix, matrix, Number(first));
        nextRotation += Number(first);
        break;
      case "scale":
        mat2d.scale(matrix, matrix, [Number(first), Number(second)]);
        nextWidth *= Number(first);
        nextHeight *= Number(second);
        break;
    }
    const center = vec2.transformMat2d(vec2.create(), [x + width / 2, y + height / 2], matrix);
    const result = [center[0] - nextWidth / 2, center[1] - nextHeight / 2, nextWidth, nextHeight, nextRotation];
    if (row.locked || !result.every(Number.isFinite)) {
      expect(row.bounds).toBeNull();
      return;
    }
    expect(row.bounds).not.toBeNull();
    row.bounds!.forEach((value, index) => expect(result[index]).toBeCloseTo(value, 5));
  });
}
