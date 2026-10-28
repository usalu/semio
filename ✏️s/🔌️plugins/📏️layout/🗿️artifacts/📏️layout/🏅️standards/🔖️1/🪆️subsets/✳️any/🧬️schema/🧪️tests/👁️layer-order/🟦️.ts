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
    const parent = vector.document.parentPages.find((row) => row.id === page.parentPageId);
    const order = (owner: Page | Parent) => sortBy(owner.frames, (frame) => {
      const rank = owner.layers.findIndex((layer) => layer.id === frame.layerId);
      return rank < 0 ? owner.layers.length : rank;
    });
    const inherited = parent ? order(parent).map((frame) => {
      const override = page.overrides.find((row) => row.objectId === frame.id);
      return { id: frame.id, parent:true, frameIndex:parent!.frames.indexOf(frame), inherited: override === undefined, x: override?.bounds?.x ?? frame.bounds.x, visible: override?.visible ?? frame.visible ?? true };
    }) : [];
    const local = order(page).map((frame) => ({ id: frame.id, parent:false, frameIndex:page.frames.indexOf(frame), inherited: false, x: frame.bounds.x, visible: frame.visible ?? true }));
    expect([...inherited, ...local]).toEqual(vector.expected);
  }
});
