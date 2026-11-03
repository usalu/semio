import { fileOf, patchAnchors } from "./patch.ts";
patchAnchors(fileOf("🛋️furniture", "picnic-table"), (list) =>
  list.map((a) => (a.id === "seat-r" ? { ...a, x: 0.12, y: 0.04 } : a.id === "seat-l" ? { ...a, x: 0.12, y: -0.04 } : a.id === "table-edge" ? { ...a, x: 0.7, y: 0.04 } : a)),
);
patchAnchors(fileOf("🛋️furniture", "folding-deck-chair"), (list) => list.map((a) => (a.id === "foot-front" ? { ...a, x: 0, y: 0 } : a)));
