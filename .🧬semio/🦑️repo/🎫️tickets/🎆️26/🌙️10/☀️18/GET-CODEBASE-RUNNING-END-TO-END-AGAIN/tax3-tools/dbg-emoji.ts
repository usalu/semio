// 🔬 [DEBUG] probes the scenario-id segment gates of projectionSourceAt for a case directory name. usage: bun dbg-emoji.ts <name>
import {pathEmojiStatuteFindings} from "@semio-tech/framework/identity/path";
import {loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const t = loadCatalogTaxonomy() as any;
for (const name of process.argv.slice(2)) {
  console.log(name, JSON.stringify(pathEmojiStatuteFindings([{path: name, nodeKind: "directory"}], t.pathEmojiPolicy.genericEmojiIdentities)));
}
console.log(Object.keys(t.semanticDirectoryKinds).filter((k) => /case|scenario/.test(k)), t.semanticDirectoryKinds["test-case"]);
