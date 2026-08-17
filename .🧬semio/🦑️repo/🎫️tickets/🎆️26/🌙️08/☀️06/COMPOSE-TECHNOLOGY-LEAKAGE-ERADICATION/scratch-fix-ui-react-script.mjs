import { readFileSync, writeFileSync } from "fs";
const path = "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts";
let lines = readFileSync(path, "utf8").split("\n");
lines = lines.filter((l) => !l.includes("compose/client/lib/sketchpad") && !l.includes("compose/client/ui/3dm"));
let src = lines.join("\n");
src = src.replace(
  " * shell, and the compose sketchpad app — walked recursively (mirrors {@link collectUiPrimitivesHits}'s",
  " * shell — walked recursively (mirrors {@link collectUiPrimitivesHits}'s",
);
writeFileSync(path, src);
const left = src.split("\n").filter((l) => /compose\/client|compose sketchpad/.test(l));
console.log(left.length ? left : "ui react script clean of compose paths");
