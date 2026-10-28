// declare-all: bun declare-all.ts <derive-rule output> <rule.json> <project.json> <out-dir> : writes <out-dir>/<target>.allow for every derived target = the rule's derived files + the expansion of the derived patterns and of the explicit workspace globs the target keeps (named inputs the rule replaces are dropped), with the LIVE precision engine and ONE git listing. Streams, RSS guard 900 MB.
import {readFileSync, writeFileSync, mkdirSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
const W = "/Users/ueli/Documents/semio";
const [derivedFile, ruleFile, projectJson, outDir] = process.argv.slice(2);
const engine: any = await import(join(W, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🟦️.ts"));
const derived = JSON.parse(readFileSync(derivedFile!, "utf8")), rule = JSON.parse(readFileSync(ruleFile!, "utf8")), targets = JSON.parse(readFileSync(projectJson!, "utf8")).targets;
const listing = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], {cwd: W, maxBuffer: 512 * 1024 * 1024, encoding: "utf8"}).stdout.split("\0").filter(Boolean);
mkdirSync(outDir!, {recursive: true});
const strip = (pattern: string) => pattern.replace("{workspaceRoot}/", "");
for (const result of derived.results) {
  if (result.error) { console.log(JSON.stringify({target: result.target, error: result.error})); continue; }
  const files = new Set<string>(result.fileList);
  const extras = ((targets[result.target].inputs ?? []) as unknown[]).filter((input): input is string => typeof input === "string" && input.startsWith("{workspaceRoot}/") && !input.startsWith("!"));
  const patterns: string[] = result.patternList;
  for (const pattern of [...extras, ...patterns.filter((p) => !p.startsWith("!"))]) {
    const expression = engine.inputGlobRegExpV1(strip(pattern));
    for (const path of listing) if (expression.test(path)) files.add(path);
  }
  const negatives = patterns.filter((p) => p.startsWith("!")).map((p) => engine.inputGlobRegExpV1(strip(p.slice(1))));
  if (negatives.length) for (const path of [...files]) if (negatives.some((e) => e.test(path))) files.delete(path);
  files.add("bunfig.toml");
  writeFileSync(join(outDir!, `${result.target}.allow`), [...files].sort().join("\n") + "\n");
  console.log(JSON.stringify({target: result.target, derived: result.fileList.length, extraPatterns: extras.length, total: files.size, rule: rule.id}));
}
if (process.memoryUsage().rss > 900 * 1048576) throw new Error("RSS above 900 MB");
