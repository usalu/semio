import { readFileSync } from "node:fs";
import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
const a = JSON.parse(readFileSync("🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/👤️sample-figure/🔣️.json", "utf8")).asset;
const b = structuredClone(a);
for (const s of b.representations["100"].geometry.front) if (s.id === "torso" || s.id === "head") s.filled = false;
const pick = (x: any) => measureVolumes(x, { scale: 100, rules: ["volumes-silhouette"] }).filter((m) => m.name.endsWith(":front") && !m.name.startsWith("hausdorff")).map((m) => `${m.name}=${m.value.toFixed(4)}`).join(" ");
console.log(pick(a)); console.log(pick(b));
