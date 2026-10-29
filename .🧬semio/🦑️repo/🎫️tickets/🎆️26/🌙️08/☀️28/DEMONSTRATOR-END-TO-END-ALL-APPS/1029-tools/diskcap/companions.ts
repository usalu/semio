import {captureNxAuthorizerSourcesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import schema from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧬️schema/🔣️.json" with {type:"json"};
const owner="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/⌨️cli/🟦️.ts";
const row=schema.$defs.RuntimeDataCompanions.const.find(r=>r.owner===owner)!;
const captured=await captureNxAuthorizerSourcesV1(process.cwd(),owner,new AbortController().signal);
const paths=new Set(captured.manifest.sources.map((s:{path:string})=>s.path));
for(const p of row.paths)console.log(paths.has(p)?"captured":"MISSING",p.slice(-80));
console.log("fixtures excluded",!paths.has("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/⌨️cli/🧫️fixtures/🔣️.json"),"old caching manifest absent",!paths.has("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📋️project.json"));
