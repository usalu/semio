/** 📄️ Independent stable layer-order projection for large-sheet export fixtures. */
import { test, expect } from "bun:test";
import sortBy from "lodash/sortBy";
import vector from "../🧫️fixtures/📄️large-sheet/🔣️.json";

test("large-sheet export projection contains every frame in stable layer order", () => {
  const page = vector.document.pages[0];
  const ordered = sortBy(page.frames, (frame) => page.layers.findIndex((layer) => layer.id === frame.layerId));
  expect(ordered.map((frame) => frame.id)).toEqual(vector.expectedOrder);
  expect(ordered.map((frame) => frame.bounds.x)).toEqual(vector.expectedX);
  expect(ordered.length).toBe(vector.frameCount);
  expect(vector.frameCount).toBeGreaterThan(8);
  expect(page.width).toBe(vector.vectorWidth);
  expect(page.height).toBe(vector.vectorHeight);
});
