import{discoverBunWorkspaces}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟦️.ts";
import{javascriptDeclaredScopesV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🟦️.ts";
const a=discoverBunWorkspaces("/Users/ueli/Documents/semio").map((s:any)=>s.directory).sort(),b=javascriptDeclaredScopesV1("/Users/ueli/Documents/semio");
console.log(JSON.stringify(a),JSON.stringify(b),JSON.stringify(a)===JSON.stringify(b));
