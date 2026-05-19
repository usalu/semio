import { readFileSync, writeFileSync } from "node:fs";

const path = "c:/git/semio/semio/client/lib/sketchpad/react/index.tsx";
const lines = readFileSync(path, "utf8").split("\n");
const start = lines.findIndex(
  (line) => line === "// #region 📍Scene" && lines[lines.indexOf(line) + 1]?.includes("Scene MUST render"),
);
const end = lines.findIndex((line, index) => index > start && line === "// #endregion 📍Scene");
if (start < 0 || end < 0) {
  throw new Error(`markers not found start=${start} end=${end}`);
}
const replacement = [
  "// #region 📍Scene",
  "/** 🎬 Design scene is {@link DesignTopologySceneWindow} on @elements/scene via {@link TopologyScenePane}. */",
  "// #endregion 📍Scene",
  "",
];
lines.splice(start, end - start + 1, ...replacement);
writeFileSync(path, lines.join("\n"), "utf8");
console.log(`removed legacy design scene lines ${start + 1}-${end + 1}`);
