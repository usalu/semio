// 🔬 [DEBUG] compares mutationFixturesDeclaredByFeature with the expanded steps of parseFeature for real features. usage: bun dbg-feature.ts <feature>...
import {readFileSync} from "node:fs";
import {mutationFixturesDeclaredByFeature} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🥒️mutation-fixture-coverage/🟦️.ts";
import {parseFeature} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧪️test/🥒️gherkin/🟦️.ts";
const policy = {stepUriPrefix: "shared://🧬️mutations/", bundleFacetDirectoryNames: ["📸️snapshot", "🦠️mutation", "🔺️diff", "🎯️outcome"]};
for (const file of process.argv.slice(2)) {
  const src = readFileSync(file, "utf8");
  const mine = mutationFixturesDeclaredByFeature(src, policy);
  const f = parseFeature(src);
  const theirs = new Set<string>();
  for (const sc of f.scenarios) for (const st of [...f.background, ...sc.steps]) { const t = (st as any).text ?? JSON.stringify(st); const at = t.indexOf(policy.stepUriPrefix); if (at >= 0) { const segs = t.slice(at + policy.stepUriPrefix.length).split(/\s/)[0].split("/"); const i = segs.findIndex((s: string) => policy.bundleFacetDirectoryNames.includes(s)); if (i > 0) theirs.add(segs.slice(0, i).join("/").replaceAll("️", "").normalize("NFC")); } }
  const onlyMine = [...mine].filter((x) => !theirs.has(x)), onlyTheirs = [...theirs].filter((x) => !mine.has(x));
  console.log(file.slice(-70), "mine", mine.size, "theirs", theirs.size, "onlyMine", onlyMine.length, "onlyTheirs", onlyTheirs.length, onlyMine.slice(0, 2), onlyTheirs.slice(0, 2));
}
