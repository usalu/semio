import { buildSandbox, patchSandboxSources } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🔬️verify/🟦️.ts";
const [dir, ...dirs] = process.argv.slice(2);
buildSandbox("/Users/ueli/Documents/semio", dir!, dirs);
patchSandboxSources(dir!, ["semio_", "serde", "protocol::", "store::", "dsl::", "pack::", "value_derive::", "framework_schema::", "tokio"]);
console.log("ok");
