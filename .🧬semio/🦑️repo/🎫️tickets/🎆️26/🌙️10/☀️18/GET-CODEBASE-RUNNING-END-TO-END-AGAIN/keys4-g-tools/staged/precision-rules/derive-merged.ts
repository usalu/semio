// derive-merged (also emits `declared` per target = derived files + the expansion of the derived patterns and of the explicit workspace globs the target keeps, the set the read oracle ran with; listing = git, or a walk without .git): bun derive-merged.ts <engine-dir> <rule.json> <project> <targets|ALL> <project.json> <out.json> : like keys2-g derive-rule.ts but the rule document is the LIVE document with the staged rule at its head (the document the graph will read after apply), validated by the live engine; offline, graph-free.
import {existsSync, readFileSync, readdirSync, writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {dirname, join} from "node:path";
const W = process.env.K4_ROOT ?? "/Users/ueli/Documents/semio", LIB = join(W, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const [engineDir, ruleFile, project, targetList, projectJson, out] = process.argv.slice(2);
const engine: any = await import(join(W, engineDir, "🟦️.ts"));
const {realHost} = await import("./realhost.ts");
const {host, scripts, internals} = await realHost("");
(host as any).importClosure = async (entries: readonly string[]) => (await internals.relativeScriptInputs(entries.map((entry) => join(W, entry)), W, scripts)).map((path: string) => path.replace(/^\{workspaceRoot\}\//u, "")).sort();
const inference: any = await import(join(LIB, "🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
const observation: any = await import(join(LIB, "🏗️build/👁️observation/🟨️.mjs"));
const compiler = (await import("node:module")).createRequire(import.meta.url)("typescript");
const sha = (text: string) => new Bun.CryptoHasher("sha256").update(text).digest("hex");
const optionsOf = new Map<string, any>();
const options = (tsconfig: string) => {
  let known = optionsOf.get(tsconfig);
  if (!known) { const path = join(W, tsconfig), read = compiler.readConfigFile(path, compiler.sys.readFile); known = compiler.parseJsonConfigFileContent(read.config, {...compiler.sys, readDirectory: () => []}, dirname(path), undefined, path).options; optionsOf.set(tsconfig, known); }
  return known;
};
const base = observation.observedTypeScriptHost(compiler.sys), prefix = W + "/";
let analysis: any;
(host as any).program = {
  compiler: compiler.version,
  fact: (kind: string, path: string) => { const full = path.startsWith("/") ? path : join(W, path); try { return sha(readFileSync(full, "utf8")); } catch { return null; } },
  imports: async (file: string, tsconfig: string) => {
    const opts = options(tsconfig), source = join(W, file), resolved = new Set<string>(), cache = compiler.createModuleResolutionCache(W, (v: string) => v, opts);
    for (const specifier of observation.observedSourceAnalysisV1(analysis ??= inference.prepareSourceAnalysesV1().module, source).imports) {
      const hit = compiler.resolveModuleName(specifier, source, opts, base, cache).resolvedModule?.resolvedFileName;
      if (hit && hit.startsWith(prefix) && !hit.includes("/node_modules/")) resolved.add(hit.slice(prefix.length));
    }
    return {resolved: [...resolved].sort(), facts: []};
  },
};
const live = JSON.parse(readFileSync(join(W, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🔣️.json"), "utf8"));
const rule = JSON.parse(readFileSync(ruleFile!, "utf8"));
const document = engine.readInputPrecisionV1({...live, rules: [rule, ...live.rules.filter((other: any) => other.id !== rule.id)]});
const targets = JSON.parse(readFileSync(projectJson!, "utf8")).targets as Record<string, any>;
const names = targetList === "ALL" ? (rule.select.targets as string[]) : targetList!.split(",");
const walk = (at: string, found: string[] = []): string[] => { for (const entry of readdirSync(join(W, at), {withFileTypes: true})) { if (["node_modules", ".git", ".cache", "target"].includes(entry.name)) continue; const path = at ? at + "/" + entry.name : entry.name; if (entry.isDirectory()) walk(path, found); else found.push(path); } return found; };
const listing = existsSync(join(W, ".git")) ? spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], {cwd: W, maxBuffer: 512 * 1024 * 1024, encoding: "utf8"}).stdout.split("\0").filter(Boolean) : walk("");
const strip = (pattern: string) => pattern.replace("{workspaceRoot}/", "");
const declaredOf = (derived: any, extras: string[]): string[] => {
  const files = new Set<string>(derived.files);
  for (const pattern of [...extras, ...derived.patterns.filter((p: string) => !p.startsWith("!"))]) { const expression = engine.inputGlobRegExpV1(strip(pattern)); for (const path of listing) if (expression.test(path)) files.add(path); }
  const negatives = derived.patterns.filter((p: string) => p.startsWith("!")).map((p: string) => engine.inputGlobRegExpV1(strip(p.slice(1))));
  if (negatives.length) for (const path of [...files]) if (negatives.some((e: any) => e.test(path))) files.delete(path);
  files.add("bunfig.toml");
  return [...files].sort();
};
const results: any[] = [];
for (const target of names) {
  const started = performance.now(), command = targets[target]?.options?.command;
  try {
    const derived = await engine.deriveInputsV1(document, rule, {project, target, command}, host);
    const extras = ((targets[target]?.inputs ?? []) as unknown[]).filter((input): input is string => typeof input === "string" && input.startsWith("{workspaceRoot}/") && !input.startsWith("!"));
    results.push({target, ms: Math.round(performance.now() - started), declared: declaredOf(derived, extras), patterns: derived.patterns.length, files: derived.files.length, unresolved: derived.unresolved, reads: derived.reads, patternList: derived.patterns, fileList: derived.files, groups: derived.groups, rssMB: Math.round(process.memoryUsage().rss / 1048576)});
  } catch (error) {
    results.push({target, error: (error as Error).message});
  }
  console.log(JSON.stringify({target, files: results.at(-1).files, patterns: results.at(-1).patterns, error: results.at(-1).error, ms: results.at(-1).ms, rssMB: Math.round(process.memoryUsage().rss / 1048576)}));
}
if (out) writeFileSync(out, JSON.stringify({rule: rule.id, results}, null, 1));
