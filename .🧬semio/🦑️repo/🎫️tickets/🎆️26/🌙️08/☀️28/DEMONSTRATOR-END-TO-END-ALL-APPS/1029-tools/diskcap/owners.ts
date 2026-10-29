import {reconcileBuildOwnerRegistry} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts";
const write=process.argv.includes("--write");
const scope="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching";
const r=reconcileBuildOwnerRegistry("/Users/ueli/Documents/semio",{scope:[scope] as never,write});
console.log(JSON.stringify({changed:r.changed,added:r.added,removed:r.removed,renamed:r.renamed,findings:r.findings},null,1));
