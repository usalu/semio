import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules";
const M = "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json";
const { exportCast } = await import(pathToFileURL(resolve(S, "📤️export/🎭️cast/🟦️.ts")).href);
try {
  await exportCast(JSON.parse(readFileSync(M, "utf8")), process.argv.slice(3), process.argv[2]!);
  console.log("ok");
} catch (e: any) {
  console.log(e.stack);
}
