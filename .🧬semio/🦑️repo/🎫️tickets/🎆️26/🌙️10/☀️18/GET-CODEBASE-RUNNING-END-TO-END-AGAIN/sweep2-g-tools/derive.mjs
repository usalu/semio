// sweep2-g: derive the input precision of candidate rules with the LIVE engine and the production closure walk, graph-free. usage: bun derive.mjs <extra-rules.json|-> <project> <target> <out-list> [key-list]
// Document = live 🎯️precision/🔣️.json + extra rules (and optional extra profiles under {"profiles":{},"rules":[]}); host = the project-assembly host without program/cargo. Writes key-list ∪ derived files ∪ pattern matches (git ls-files -co) to out-list. RSS guard 1.2 GB.
import {readFileSync, writeFileSync, existsSync, readdirSync} from "node:fs";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
const W = "/Users/ueli/Documents/semio", LIB = join(W, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
process.chdir(W);
const precision = await import(join(LIB, "🏗️build/📥️inputs/🎯️precision/🟦️.ts"));
const inference = await import(join(LIB, "🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
await inference.libraryBootstrap;
const [extraFile, project, target, outFile, keyFile] = process.argv.slice(2);
const live = JSON.parse(readFileSync(join(LIB, "🏗️build/📥️inputs/🎯️precision/🔣️.json"), "utf8"));
const extra = extraFile === "-" ? {} : JSON.parse(readFileSync(extraFile, "utf8"));
const document = precision.readInputPrecisionV1({...live, profiles: {...live.profiles, ...(extra.profiles ?? {})}, rules: [...(extra.replace ? live.rules.filter((rule) => !extra.replace.includes(rule.id)) : live.rules), ...(extra.rules ?? [])]});
const scripts = inference.createScriptInputCache({workspaceRoot: W});
const rss = () => process.memoryUsage().rss / 1048576;
const host = {
  exists: (path) => existsSync(join(W, path)),
  readText: (path) => { try { return readFileSync(join(W, path), "utf8"); } catch { return undefined; } },
  listDirectory: (path) => { try { return readdirSync(join(W, path), {withFileTypes: true}).map((entry) => ({name: entry.name, directory: entry.isDirectory()})); } catch { return []; } },
  kinds: JSON.parse(readFileSync(join(LIB, "🔣️taxonomy.json"), "utf8")).fileKinds,
  importClosure: async (entries) => (await inference.cacheInternals.relativeScriptInputs(entries.map((entry) => join(W, entry)), W, scripts)).map((path) => path.replace(/^\{workspaceRoot\}\//u, "")),
  cargoManifests: () => spawnSync("git", ["-c", "core.quotepath=false", "ls-files", "-z", "--", ":(glob)**/Cargo.toml"], {cwd: W, encoding: "utf8"}).stdout.split("\0").filter(Boolean).map((file) => file === "Cargo.toml" ? "" : file.slice(0, -"/Cargo.toml".length)),
  cargoRoots: () => { throw new Error("cargo reads are not simulated"); },
  cargoSources: () => undefined,
  cargoToolchain: [],
};
const manifestFile = (await import("node:fs")).readdirSync;
void manifestFile;
// the declared target definition of the sealed generation (command, outputs)
const {brotliDecompressSync} = await import("node:zlib");
const generations = W + "/.cache/b/g/m/generations/", cur = JSON.parse(readFileSync(generations + "current.json", "utf8")), manifest = JSON.parse(readFileSync(generations + "m/" + cur.manifest + ".json", "utf8"));
const fragment = JSON.parse(brotliDecompressSync(readFileSync(generations + "o/" + manifest.owners[project].fragment + ".json.br")).toString("utf8"));
const definition = fragment.node.targets[target];
const identity = {project, target, command: definition.options?.command, outputs: []};
const rule = precision.selectRuleV1(document, identity);
if (!rule) { console.error("no rule selects", project, target, identity.command); process.exit(2); }
const derived = await precision.deriveInputsV1(document, rule, identity, host);
const listing = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], {maxBuffer: 256 * 1024 * 1024, encoding: "utf8"}).stdout.split("\0").filter(Boolean).sort();
const set = new Set(keyFile ? readFileSync(keyFile, "utf8").split("\n").filter(Boolean) : []);
const before = set.size;
for (const file of derived.files) if (existsSync(join(W, file))) set.add(file);
const positive = derived.patterns.filter((pattern) => !pattern.startsWith("!")).map((pattern) => pattern.replace("{workspaceRoot}/", "").replace("{projectRoot}", fragment.root));
const negative = derived.patterns.filter((pattern) => pattern.startsWith("!")).map((pattern) => precision.inputGlobRegExpV1(pattern.slice(1)));
for (const pattern of positive) { const expression = precision.inputGlobRegExpV1("{workspaceRoot}/" + pattern); void expression; const re = precision.inputGlobRegExpV1(pattern); for (const file of listing) if (re.test(file)) set.add(file); }
void negative;
writeFileSync(outFile, [...set].sort().join("\n") + "\n");
console.log(JSON.stringify({rule: rule.id, evidence: rule.evidence, drops: derived.drops, reads: derived.reads, groups: derived.groups, files: derived.files.length, patterns: derived.patterns.length, unresolved: derived.unresolved, keyBefore: before, keyAfter: set.size, rssMb: Math.round(rss())}));
process.exit(0);
