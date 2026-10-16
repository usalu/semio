import { readFileSync } from "node:fs";
import { pathEmojiStatuteFindings } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🪪️identity/🛣️path/🟦️.ts";
const paths = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
const entries = paths.map((path) => ({ path, nodeKind: "directory" as const }));
for (const p of paths) {
  const f = pathEmojiStatuteFindings([{ path: p, nodeKind: "directory" }], []);
  if (f.length) console.log(p.slice(p.lastIndexOf("/") + 1), JSON.stringify(f.map((x) => x.kind)));
}
