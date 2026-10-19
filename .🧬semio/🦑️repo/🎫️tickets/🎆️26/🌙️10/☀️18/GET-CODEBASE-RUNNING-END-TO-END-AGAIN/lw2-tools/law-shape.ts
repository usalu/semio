
import { policyEmojiPrefixBreaches, policyBannedNameStemBreaches } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🏷️naming/🟦️.ts";
const root = process.cwd();
for (const [name, fn] of [["emoji", policyEmojiPrefixBreaches], ["stems", policyBannedNameStemBreaches]] as const) {
  const rows = (fn as any)(root) as any[];
  console.log(name, rows.length);
  const mine = rows.filter(r => /extension-package|artifact-reference\/🪆️child|packaging\/🔌️provider|🧑‍💻dev\/(🏗️fem|🧱️block|📸️remodel|🧩️puzzle)/u.test(JSON.stringify(r)));
  for (const r of mine.slice(0, 10)) console.log("  MINE", JSON.stringify(r).slice(0, 300));
}
