// 🧩️ Ensures an owner crate declares `FEATURE = []` in [features]: bun ctc-ownerfeat.ts OWNER_MANIFEST FEATURE
import {readFileSync, writeFileSync} from "node:fs";
const [manifestArg, feature] = process.argv.slice(2);
const manifest = manifestArg!.startsWith("/") ? manifestArg! : "/Users/ueli/Documents/semio/" + manifestArg!;
const text = readFileSync(manifest, "utf8");
const parsed = Bun.TOML.parse(text) as any;
if (parsed.features && feature! in parsed.features) { console.log("feature already declared in " + manifest); process.exit(0); }
const lines = text.split("\n");
const at = lines.findIndex(l => l.trim() === "[features]");
if (at >= 0) lines.splice(at + 1, 0, `${feature} = []`);
else {
  let ins = lines.findIndex(l => l.trim() === "[dependencies]");
  if (ins < 0) ins = lines.findIndex(l => l.trim() === "[dev-dependencies]");
  if (ins < 0) ins = lines.length;
  lines.splice(ins, 0, "[features]", `${feature} = []`, "");
}
writeFileSync(manifest, lines.join("\n"));
console.log("declared " + feature + " in " + manifest);
