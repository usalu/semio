// undeclared-g stage-module: copies a library module with every RELATIVE import/export/dynamic-import specifier rewritten to an absolute path of the original location, so a patched copy can run (and be tested) outside the library without touching it. usage: bun stage-module.mjs <live module> <out file> [patch.py]
import {readFileSync, writeFileSync, mkdirSync} from "node:fs";
import {dirname, resolve} from "node:path";
import {spawnSync} from "node:child_process";
const [source, out, patch] = process.argv.slice(2);
const absolute = resolve(source);
writeFileSync(out, readFileSync(absolute));
if (patch) { const run = spawnSync("python3", ["-I", patch, out], {encoding: "utf8"}); process.stdout.write(run.stdout); if (run.status) { process.stderr.write(run.stderr); process.exit(run.status); } }
let text = readFileSync(out, "utf8");
const here = dirname(absolute);
const rewrite = (specifier) => (specifier.startsWith(".") ? resolve(here, specifier) : specifier);
text = text.replace(/((?:from|import)\s*\(?\s*)(["'])(\.{1,2}\/[^"']*)\2/gu, (_, head, quote, specifier) => `${head}${quote}${rewrite(specifier)}${quote}`);
text = text.replace(/(createRequire\(import\.meta\.url\)\()(["'])(\.{1,2}\/[^"']*)\2/gu, (_, head, quote, specifier) => `${head}${quote}${rewrite(specifier)}${quote}`);
text = text.replaceAll("import.meta.url", JSON.stringify("file://" + absolute));
writeFileSync(out, text);
