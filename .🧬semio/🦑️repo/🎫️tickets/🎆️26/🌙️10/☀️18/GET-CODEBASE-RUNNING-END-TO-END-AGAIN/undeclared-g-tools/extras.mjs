// undeclared-g extras: expands a simulated declaration (workspace globs + module entries) to files, the way the repository would key it: globs by the precision engine's own matcher over git ls-files (+ generated mirrors), module entries by the static import closure (bun's scanner) plus, with "configs": true, the F1 rule (staged resolutionConfigsV1) for those modules. usage: bun extras.mjs <spec.json> <out.files>
import {readFileSync, writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {dirname, join} from "node:path";
import {staticClosure, WORKSPACE} from "./static-closure.mjs";
const {inputGlobRegExpV1} = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🟦️.ts");
const staged = await import("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/undeclared-g/staged/script-inputs.f1.mjs");
const {parserTypescriptV1} = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/📦️parsers/🟨️.mjs");
const [specFile, out] = process.argv.slice(2), spec = JSON.parse(readFileSync(specFile, "utf8")), files = new Set();
const listing = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], {cwd: WORKSPACE, maxBuffer: 256 * 1024 * 1024, encoding: "utf8"}).stdout.split("\0").filter(Boolean);
for (const glob of spec.globs ?? []) { const expression = inputGlobRegExpV1(glob.replace("{workspaceRoot}/", "")); for (const path of listing) if (expression.test(path)) files.add(path); }
for (const path of spec.files ?? []) files.add(path);
const modules = new Set();
for (const entry of spec.modules ?? []) for (const path of staticClosure([entry.replace("{workspaceRoot}/", "")])) { files.add(path); modules.add(join(WORKSPACE, path)); }
if (spec.configs) { const compiler = parserTypescriptV1(); for (const config of staged.resolutionConfigsV1(compiler, compiler.sys, WORKSPACE, modules)) files.add(config.replace(WORKSPACE + "/", "")); }
if (process.memoryUsage().rss > 500 * 1048576) throw new Error("RSS above 500 MB");
writeFileSync(out, [...files].sort().join("\n") + "\n");
console.log(JSON.stringify({files: files.size, modules: modules.size}));
