// 🔬 Runs the command-router validator over every tracked or untracked 📜️script.ts and prints rejected paths. usage: bun census.ts [out.txt]
import {readFileSync, writeFileSync} from "node:fs";
import {execSync} from "node:child_process";
import {ecmaCommandRouterModule} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🛣️ecma-command-routes/🟦️.ts";
import {loadCatalogTaxonomy} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
const grammar = (loadCatalogTaxonomy() as any).packageGlueGrammar.typescript;
const files = execSync("git ls-files --cached --others --exclude-standard -z", {cwd: "/Users/ueli/Documents/semio", maxBuffer: 1 << 28}).toString().split("\0").filter((p) => p.endsWith("/📜️script.ts") || p === "📜️script.ts");
const bad: string[] = [];
for (const p of files) { let s: string; try { s = readFileSync(p, "utf8"); } catch { continue; } if (!ecmaCommandRouterModule(s, grammar)) bad.push(p); }
console.log(JSON.stringify({total: files.length, rejected: bad.length}));
if (process.argv[2]) writeFileSync(process.argv[2], bad.join("\n") + "\n");
