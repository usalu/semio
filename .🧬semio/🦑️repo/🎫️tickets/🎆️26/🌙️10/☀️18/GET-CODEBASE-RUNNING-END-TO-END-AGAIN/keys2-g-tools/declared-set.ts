// declared-set: DERIVED_JSON=<derive-rule output> bun declared-set.ts <engine-dir> <rule.json> <project> <project.json> <target> <out.allow> : the files the narrowed target declares = the rule's derived files + the expansion of the explicit workspace globs the target keeps (named inputs the rule replaces are dropped). Graph-free; streams the file list; RSS guard 500 MB.
import {readFileSync, writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
const W = "/Users/ueli/Documents/semio";
const [engineDir, ruleFile, project, projectJson, target, out] = process.argv.slice(2);
const engine: any = await import(join(W, engineDir!, "🟦️.ts"));
const rule = JSON.parse(readFileSync(ruleFile!, "utf8")), definition = JSON.parse(readFileSync(projectJson!, "utf8")).targets[target!];
const result = JSON.parse(readFileSync(process.env.DERIVED_JSON!, "utf8")).results.find((row: any) => row.target === target);
const files = new Set<string>(result.fileList);
const extras = (definition.inputs as unknown[]).filter((input): input is string => typeof input === "string" && input.startsWith("{workspaceRoot}/") && !input.startsWith("!"));
const listing = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], {cwd: W, maxBuffer: 256 * 1024 * 1024, encoding: "utf8"}).stdout.split("\0").filter(Boolean);
const strip = (pattern: string) => pattern.replace("{workspaceRoot}/", "");
const derivedPatterns: string[] = result.patternList;
for (const pattern of [...extras, ...derivedPatterns.filter((pattern) => !pattern.startsWith("!"))]) {
  const expression = engine.inputGlobRegExpV1(strip(pattern));
  for (const path of listing) if (expression.test(path)) files.add(path);
}
const negatives = derivedPatterns.filter((pattern) => pattern.startsWith("!")).map((pattern) => engine.inputGlobRegExpV1(strip(pattern.slice(1))));
if (negatives.length) for (const path of [...files]) if (negatives.some((expression) => expression.test(path))) files.delete(path);
if (process.memoryUsage().rss > 500 * 1048576) throw new Error("RSS above 500 MB");
writeFileSync(out!, [...files].sort().join("\n") + "\n");
console.log(JSON.stringify({target, derived: result.fileList.length, extraPatterns: extras.length, total: files.size, rule: rule.id}));
