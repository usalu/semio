import { readFileSync, writeFileSync } from "node:fs";
const repo = "/Users/ueli/Documents/semio";
const write = process.argv.includes("--write");
const { renderCatalogFiles } = await import(`${repo}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts`);
const { declaredProjectTargets, generateLaunchJson } = await import(`${repo}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts`);
const { playgrounds } = renderCatalogFiles(repo);
const expected = generateLaunchJson(repo, playgrounds, declaredProjectTargets(repo));
const actual = readFileSync(`${repo}/.vscode/launch.json`, "utf8");
if (write && expected !== actual) writeFileSync(`${repo}/.vscode/launch.json`, expected);
const a = actual.split("\n"), e = expected.split("\n");
let first = -1;
for (let i = 0; i < Math.max(a.length, e.length); i++) if (a[i] !== e[i]) { first = i; break; }
console.log(`fresh=${expected === actual} lines actual=${a.length} expected=${e.length} firstDiff=${first}${write && expected !== actual ? " (launch.json re-rendered)" : ""}`);
if (first >= 0) console.log(`actual:   ${JSON.stringify(a.slice(first, first + 3))}\nexpected: ${JSON.stringify(e.slice(first, first + 3))}`);
