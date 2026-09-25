// WG8: inspects the test-layout rule on exactly the wgpu Shell sources T12 attributed to WG7/WG8.
import { readdirSync, readFileSync, statSync } from "node:fs";
import { inspectTestLayoutSources, testTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio/";
const shell = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell";
const walk = (dir: string): string[] => readdirSync(repoRoot + dir).flatMap((name) => (statSync(`${repoRoot}${dir}/${name}`).isDirectory() ? [`${dir}/${name}`, ...walk(`${dir}/${name}`)] : [`${dir}/${name}`]));
const tree = walk(shell);
const directories = tree.filter((path) => statSync(repoRoot + path).isDirectory());
const paths = tree.filter((path) => path.endsWith(".rs"));
const taxonomy = testTaxonomy(repoRoot);
const findings = inspectTestLayoutSources(taxonomy, paths.map((path) => ({ path, source: readFileSync(repoRoot + path, "utf8") })), directories);
console.log(`findings=${findings.length}`);
for (const finding of findings) console.log(JSON.stringify(finding));
