#!/usr/bin/env bun
/** 📋️ R9 item 1 follow-up: the plugin-registry generator renders one launch row per declared `📋️project.json` target, so every
 * project manifest is an input of its contract. Adds the taxonomy-admissible manifest patterns (`launch-input-prefixes.ts`:
 * literal prefixes down to the deepest opaque root, then `**`) to `generatorContracts["plugin-registry"].inputPatterns`, keeping
 * the list JS-lexically ordered as the taxonomy validator requires. Usage: `bun launch-inputs-taxonomy.ts [--apply]`. */
const ROOT = "/Users/ueli/Documents/semio";
const PATH = `${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`;
const text = await Bun.file(PATH).text();
const taxonomy = JSON.parse(text);
const proc = Bun.spawnSync(["bun", `${ROOT}/.tmp-ticket/wp-r9/launch-input-prefixes.ts`], { cwd: ROOT });
if (proc.exitCode !== 0) throw new Error(proc.stderr.toString());
const { patterns } = JSON.parse(proc.stdout.toString()) as { patterns: string[] };
const current: string[] = taxonomy.generatorContracts["plugin-registry"].inputPatterns;
const next = [...new Set([...current.filter((pattern) => !pattern.endsWith("/📋️project.json") || pattern.startsWith("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/")), ...patterns])].sort();
const start = text.indexOf('"inputPatterns": [', text.indexOf('"plugin-registry": {', text.indexOf('"generatorContracts"')));
const end = text.indexOf("]", start);
const indent = /\n([ ]*)"/u.exec(text.slice(start, end))![1]!;
const closing = text.slice(text.lastIndexOf("\n", end) + 1, end);
const replaced = `${text.slice(0, start)}"inputPatterns": [\n${next.map((pattern) => `${indent}${JSON.stringify(pattern)}`).join(",\n")}\n${closing}${text.slice(end)}`;
const parsed = JSON.parse(replaced);
if (JSON.stringify(parsed.generatorContracts["plugin-registry"].inputPatterns) !== JSON.stringify(next)) throw new Error("rewrite drifted");
console.log(`patterns ${current.length} -> ${next.length}; added ${next.filter((pattern) => !current.includes(pattern)).length}`);
if (process.argv.includes("--apply")) await Bun.write(PATH, replaced);
console.log(process.argv.includes("--apply") ? "applied" : "dry run clean");
