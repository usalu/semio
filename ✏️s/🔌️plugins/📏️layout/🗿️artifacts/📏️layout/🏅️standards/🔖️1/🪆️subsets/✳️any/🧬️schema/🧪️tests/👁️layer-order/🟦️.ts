/** 🗂️ Independent stable layer-order oracle for native page resolution. */
import { test, expect } from "bun:test";
import sortBy from "lodash/sortBy";
import vectors from "../../../🧪️tests/🧫️fixtures/👁️layer-order/🔣️.json";
type Document = (typeof vectors.cases)[number]["document"];
type Page = Document["pages"][number];
type Parent = Document["parentPages"][number];

test("resolved parent and local frames follow stable layer ordering", () => {
  for (const vector of vectors.cases) {
    const page = vector.document.pages[0];
    if (!page) throw new Error("Missing fixture page");
    const parent = vector.document.parentPages.find((row) => row.id === page.parentPageId);
    const order = (owner: Page | Parent) => sortBy(owner.frames, (frame) => {
      const rank = owner.layers.findIndex((layer) => layer.id === frame.layerId);
      return rank < 0 ? owner.layers.length : rank;
    });
    const resolve = (owner: Page | Parent, fromParent: boolean) => order(owner).map((frame) => {
      const override = fromParent ? page.overrides.find((row) => row.objectId === frame.id) : undefined;
      const bounds = override?.bounds ?? frame.bounds;
      const visible = override?.visible ?? frame.visible ?? true;
      const layerVisible = owner.layers.find((layer) => layer.id === frame.layerId)?.visible ?? true;
      return { bounds, layerVisible, row: { id: frame.id, parent: fromParent, frameIndex: owner.frames.indexOf(frame), inherited: fromParent && override === undefined, x: bounds.x, visible } };
    });
    const resolved = [...(parent ? resolve(parent, true) : []), ...resolve(page, false)];
    expect(resolved.map(({ row }) => row)).toEqual(vector.expected);
    if (vector.boundedResolution) {
      const { minimumX, rows } = vector.boundedResolution;
      expect(resolved.filter(({ bounds }) => bounds.x >= minimumX).map(({ row }) => [row.id, row.frameIndex])).toEqual(rows);
    }
    const visibleFrames = resolved.filter(({ row, layerVisible }) => row.visible && layerVisible);
    for (const point of [vector.hit, ...(vector.hits ?? [])]) {
      const hit = visibleFrames.filter(({ bounds }) => {
        const dx = point.x - (bounds.x + bounds.w / 2);
        const dy = point.y - (bounds.y + bounds.h / 2);
        const cos = Math.cos(bounds.rotation);
        const sin = Math.sin(bounds.rotation);
        return Math.abs(dx * cos + dy * sin) <= bounds.w / 2 && Math.abs(dy * cos - dx * sin) <= bounds.h / 2;
      }).at(-1);
      expect(hit?.row.id ?? null).toBe(point.expected);
    }
    if (vector.view) {
      const { x, y, zoom, expected } = vector.view;
      const span = 2000 / Math.max(zoom, 0.05);
      const projected = visibleFrames.filter(({ bounds }) => {
        const cos = Math.cos(bounds.rotation);
        const sin = Math.sin(bounds.rotation);
        const corners = [-1, 1].flatMap((horizontal) => [-1, 1].map((vertical) => {
          const dx = horizontal * bounds.w / 2;
          const dy = vertical * bounds.h / 2;
          return [bounds.x + bounds.w / 2 + dx * cos - dy * sin, bounds.y + bounds.h / 2 + dx * sin + dy * cos];
        }));
        const xs = corners.map(([px]) => px!);
        const ys = corners.map(([, py]) => py!);
        return Math.min(...xs) <= x + span && Math.max(...xs) >= x - span && Math.min(...ys) <= y + span && Math.max(...ys) >= y - span;
      }).map(({ row }) => row.id);
      expect(projected).toEqual(expected);
    }
  }
});
