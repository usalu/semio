import { fileOf, patchView } from "./patch.ts";
const rect = (s: any, x: number, y: number, w: number, h: number, extra: any = {}) => {
  const { points, closed, ...rest } = s;
  return { ...rest, kind: "rect", x, y, w, h, ...extra };
};

const board = fileOf("🪑️objects", "chalkboard");
patchView(board, 500, "elevation", (shapes) =>
  shapes.map((s) => {
    if (s.id === "frame") return { ...s, y: 0.07, h: 1.19 };
    if (s.id === "tray") return { id: "tray", bone: "tray", role: "body", z: 3.2, filled: false, tone: 0.5, kind: "poly", points: [[-2, 0.035], [2, 0.035]], closed: false };
    if (s.id === "leaf") return { ...s, y: 0.07, h: 1.19 };
    return s;
  }),
);
patchView(board, 500, "front", (shapes) =>
  shapes
    .filter((s) => s.id !== "end-tray")
    .map((s) => (s.id === "end-frame" ? { ...s, y: 0, h: 1.26 } : s.id === "end-leaf" ? { ...s, x: -0.04, w: 0.04, y: 0.07, h: 1.19 } : s)),
);
patchView(board, 500, "plan", (shapes) => shapes.map((s) => (s.id === "top-leaf" ? { ...s, y: -0.04, h: 0.04 } : s)));

const crates = fileOf("🪑️objects", "storage-crate-stack");
patchView(crates, 500, "elevation", (shapes) => shapes.map((s) => rect(s, -0.31, 0, 0.62, 0.3, { z: s.z })));
patchView(crates, 500, "front", (shapes) =>
  shapes.map((s, i) => rect(s, -0.2, i * 0.3, 0.4, 0.3, { z: s.z })),
);
patchView(crates, 500, "plan", (shapes) => shapes.filter((s) => s.id === "c5").map((s) => rect(s, -0.31, -0.2, 0.62, 0.4, { r: undefined })));

const table = fileOf("🛋️furniture", "operating-table");
patchView(table, 500, "elevation", (shapes) =>
  shapes.map((s) => {
    if (s.id === "st-base") return rect(s, -0.28, 0, 0.56, 0.105);
    if (s.id === "col") return rect(s, -0.14, 0, 0.28, 0.62);
    if (s.id === "tb-plate") return rect(s, -0.4, 0, 0.8, 0.16);
    return s;
  }),
);
patchView(table, 500, "front", (shapes) =>
  shapes
    .filter((s) => !s.id.startsWith("top-end"))
    .map((s) => {
      if (s.id === "st-base") return rect(s, -0.25, 0, 0.5, 0.105);
      if (s.id === "tb-plate") return rect(s, -0.2, 0, 0.4, 0.16);
      if (s.id === "col") return rect(s, -0.14, 0.16, 0.28, 0.62);
      return s;
    }),
);
patchView(table, 500, "plan", (shapes) => shapes.map((s) => (s.id === "st-base" ? { ...s, y: -0.25, h: 0.5 } : s)));
