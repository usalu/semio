import { mkdir } from "node:fs/promises";
import { basename, dirname, join, resolve } from "node:path";
import { transpilePluginComponentModulesAsync } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";

const repoRoot = resolve(import.meta.dirname, "../../../../../../..");
const artifact = resolve(process.argv[2]);
const output = process.argv[3] ? resolve(process.argv[3]) : join(dirname(artifact), "transpiled");
const name = process.argv[4] ?? basename(artifact, ".wasm");
if (!/^semio_hub_norm(?:_\w+)*_component$/u.test(name)) throw Error("Expected a linked Norm guest");

await mkdir(output, { recursive: true });
await transpilePluginComponentModulesAsync(artifact, output, name, {
  repoRoot,
  mode: "prototype",
  preview2VendorDir: join(repoRoot, ".cache/b/p/m/browser/prototype/modules"),
});
console.log("[DEBUG] Direct browser module", join(output, name));
