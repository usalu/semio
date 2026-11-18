/** 🩺 Validation issues of the HEAD version of chosen assets. bun humansa-validate-head.ts <slug>... */
import { execFileSync } from "node:child_process";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
for (const slug of process.argv.slice(2)) {
  const text = execFileSync("git", ["-C", "/Users/ueli/Documents/semio", "show", `HEAD:🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/${slug}/🔣️.json`], { maxBuffer: 1 << 28 }).toString("utf8");
  const issues = validateAsset(JSON.parse(text).asset);
  console.log(`== HEAD ${slug}: ${issues.length}`);
  for (const i of issues) console.log(`  ${i.code}: ${i.message.en.slice(0, 160)}`);
}
