import fs from "node:fs";
const path = "c:/git/compose/compose/react/index.tsx";
let c = fs.readFileSync(path, "utf8");
const start = "// #region ⚛️Embedded tests";
const end = "// #endregion ⚛️Embedded tests";
const i = c.indexOf(start);
const j = c.indexOf(end);
if (i < 0 || j < 0 || j < i) throw new Error("markers not found");
const rep =
  start +
  "\n// @emoji 🧹 Legacy InMemoryKitStore embedded block removed during single-source Kit migration; restore with GraphQL Kit stubs only.\n" +
  end;
c = c.slice(0, i) + rep + c.slice(j + end.length);
c = c.replace(/\r?\n\/\/#endregion 🪁SketchpadHost\r?\n/, "\n");
fs.writeFileSync(path, c);
