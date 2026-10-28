"""[walk-g] writes edits.json of the batch `walks` from the anchored edits below (each `old` must occur exactly once in the live file). usage: make_edits.py (from anywhere)."""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 10)))
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
ENGINE = B + "📥️inputs/🎯️precision/🟦️.ts"
SCHEMA = B + "📥️inputs/🎯️precision/🧬️schema/🔣️.json"
ENGINE_LAW = B + "📥️inputs/🎯️precision/🧪️tests/🟦️.ts"
SCRIPTS = B + "🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
SCRIPTS_D = B + "🕸️graph/📥️inference/🧭️script-inputs/🟨️.d.mts"
ASSEMBLY = B + "🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs"

edits = []


def edit(file, name, old, new):
    edits.append({"file": file, "name": name, "old": old, "new": new})


edit(ENGINE, "host ports",
     "  readonly cargoIdentity?: (crateDirectory: string) => unknown;\n};",
     """  readonly cargoIdentity?: (crateDirectory: string) => unknown;
  readonly crateNames?: () => Promise<ReadonlyMap<string, string>>;
  readonly sourceRoots?: (question: SourceRootsQuestionV1) => Promise<readonly string[]>;
};
/** 🚶️ The question of a repository walk that selects TypeScript files: the files that entry globs name, or that the include globs of a tsconfig select below its excludes. */
export type SourceRootsQuestionV1 = {readonly mode: "entries"; readonly globs: readonly string[]} | {readonly mode: "include"; readonly include: readonly string[]; readonly exclude: readonly string[]};
/** 🚶️ What a repository walk skips: whole paths (below the workspace root) and directory names; symbolic links are always skipped. */
export type ManifestWalkSkipV1 = {readonly paths: readonly string[]; readonly names: readonly string[]};
/** 🚶️ The recipe of one repository walk a derived fact stands for: the crate index of the Cargo manifests, or the TypeScript files a question selects. */
export type DerivedWalkV1 = {readonly fn: "crate-index"; readonly skip: ManifestWalkSkipV1} | {readonly fn: "source-roots"; readonly question: SourceRootsQuestionV1};""")

edit(ENGINE, "crate index",
     """  const index = new Map<string, string>();
  for (const directory of host.cargoManifests()) {
    if (directory.split("/").some((part) => part.startsWith("."))) continue;
    const name = packageNameV1(host.readText(directory ? `${directory}/Cargo.toml` : "Cargo.toml") ?? "");
    if (name && !index.has(name)) index.set(name, directory);
  }
  indexes.set(host, index);
  return index;
}
""",
     """  const index = crateNamesV1(host.cargoManifests(), host.readText);
  indexes.set(host, index);
  return index;
}

/** 🗺️ Maps the crate names of the Cargo manifest directories (in the order given, the first directory of a name wins) to their directory; hidden trees never own a crate. */
export function crateNamesV1(directories: readonly string[], readText: (path: string) => string | undefined): Map<string, string> {
  const index = new Map<string, string>();
  for (const directory of directories) {
    if (directory.split("/").some((part) => part.startsWith("."))) continue;
    const name = packageNameV1(readText(directory ? `${directory}/Cargo.toml` : "Cargo.toml") ?? "");
    if (name && !index.has(name)) index.set(name, directory);
  }
  return index;
}

/** 🦀️ Every directory of the repository that holds a Cargo manifest, sorted: symbolic links, the skipped paths and the skipped directory names are not entered. */
export function cargoManifestDirectoriesV1(list: (directory: string) => readonly {readonly name: string; readonly directory: boolean; readonly file: boolean; readonly link: boolean}[], skip: ManifestWalkSkipV1): string[] {
  const found: string[] = [], paths = new Set(skip.paths), names = new Set(skip.names), walk = (directory: string): void => {
    for (const entry of list(directory)) {
      const path = directory ? `${directory}/${entry.name}` : entry.name;
      if (entry.link || paths.has(path) || names.has(entry.name)) continue;
      if (entry.directory) walk(path);
      else if (entry.file && entry.name === "Cargo.toml") found.push(directory);
    }
  };
  walk("");
  return found.sort();
}

/** 🗺️ The crate index of a host: answered by its derived walk when it has one, otherwise by walking in place. */
export async function crateIndexOfV1(host: Pick<InputPrecisionHostV1, "readText" | "cargoManifests" | "crateNames">): Promise<ReadonlyMap<string, string>> {
  return host.crateNames ? host.crateNames() : crateIndexV1(host);
}
""")

edit(ENGINE, "crate patterns",
     """export function cratePatternsV1(packages: readonly string[], host: Pick<InputPrecisionHostV1, "readText" | "cargoManifests" | "cargoRoots" | "cargoSources" | "cargoToolchain" | "cargoIdentity">): {patterns: string[]; files: string[]; markers: unknown[]} {
  const index = crateIndexV1(host);
  return""",
     """export function cratePatternsV1(packages: readonly string[], host: Pick<InputPrecisionHostV1, "readText" | "cargoManifests" | "cargoRoots" | "cargoSources" | "cargoToolchain" | "cargoIdentity">, index: ReadonlyMap<string, string> = crateIndexV1(host)): {patterns: string[]; files: string[]; markers: unknown[]} {
  return""")

edit(ENGINE, "walk docs",
     """/** 🔭️ The TypeScript source files the entry globs select: positive globs walked from their literal directory, negations applied, hidden and vendored trees skipped; a pattern without wildcards names one file. */""",
     """/** 🧭️ The TypeScript files a tsconfig's include globs select: positive globs walked from their literal directory, excludes applied, vendored trees skipped; a pattern without wildcards names one file. */
export function includeRootsV1(include: readonly string[], exclude: readonly string[], host: Pick<InputPrecisionHostV1, "listDirectory" | "exists">): string[] {
  const isExcluded = excludesV1(exclude), roots = new Set<string>();
  for (const pattern of include) {
    const stop = pattern.search(/[*?{[]/u), base = stop < 0 ? pattern : pattern.slice(0, pattern.lastIndexOf("/", stop)), pending = [normalize(base)];
    if (stop < 0 && SOURCE.test(pattern) && host.exists(pattern)) roots.add(pattern);
    while (stop >= 0 && pending.length) {
      const directory = pending.pop()!;
      for (const entry of host.listDirectory(directory)) {
        const path = directory ? `${directory}/${entry.name}` : entry.name;
        if (SKIPPED.has(entry.name) || isExcluded(path)) continue;
        if (entry.directory) pending.push(path);
        else if (SOURCE.test(path) && globMatchesV1(pattern, path)) roots.add(path);
      }
    }
  }
  return sorted([...roots]);
}

/** 🔭️ The TypeScript source files the entry globs select: positive globs walked from their literal directory, negations applied, hidden and vendored trees skipped; a pattern without wildcards names one file. */""")

edit(ENGINE, "program memo",
     """export function programInputsV1(tsconfig: string, host: Pick<InputPrecisionHostV1, "readText" | "exists" | "listDirectory" | "program">, entries?: readonly string[]): Promise<{patterns: string[]; files: string[]; roots: string[]}> {
  const key = entries ? `${normalize(tsconfig)}\\0${JSON.stringify(entries)}` : normalize(tsconfig);
  let byConfig = programs.get(host);
  if (!byConfig) programs.set(host, byConfig = new Map());""",
     """export function programInputsV1(tsconfig: string, host: Pick<InputPrecisionHostV1, "readText" | "exists" | "listDirectory" | "program" | "sourceRoots">, entries?: readonly string[]): Promise<{patterns: string[]; files: string[]; roots: string[]}> {
  const key = entries ? `${normalize(tsconfig)}\\0${JSON.stringify(entries)}` : normalize(tsconfig), owner = host.program ?? host;
  let byConfig = programs.get(owner);
  if (!byConfig) programs.set(owner, byConfig = new Map());""")

edit(ENGINE, "program derive head",
     """async function deriveProgramV1(tsconfig: string, host: Pick<InputPrecisionHostV1, "readText" | "exists" | "listDirectory" | "program">, entries?: readonly string[]): Promise<{patterns: string[]; files: string[]; roots: string[]}> {""",
     """async function deriveProgramV1(tsconfig: string, host: Pick<InputPrecisionHostV1, "readText" | "exists" | "listDirectory" | "program" | "sourceRoots">, entries?: readonly string[]): Promise<{patterns: string[]; files: string[]; roots: string[]}> {""")

edit(ENGINE, "program roots",
     """  const isExcluded = excludesV1(exclude), roots = new Set<string>(entries ? entryFilesV1(entries, host) : config.files.filter((file) => host.exists(file)));
  if (entries && !roots.size) fail(`Program entries select no TypeScript file: ${tsconfig}`);
  for (const pattern of include) {
    const stop = pattern.search(/[*?{[]/u), base = stop < 0 ? pattern : pattern.slice(0, pattern.lastIndexOf("/", stop)), pending = [normalize(base)];
    if (stop < 0 && SOURCE.test(pattern) && host.exists(pattern)) roots.add(pattern);
    while (stop >= 0 && pending.length) {
      const directory = pending.pop()!;
      for (const entry of host.listDirectory(directory)) {
        const path = directory ? `${directory}/${entry.name}` : entry.name;
        if (SKIPPED.has(entry.name) || isExcluded(path)) continue;
        if (entry.directory) pending.push(path);
        else if (SOURCE.test(path) && globMatchesV1(pattern, path)) roots.add(path);
      }
    }
  }
""",
     """  const isExcluded = excludesV1(exclude), ask = (question: SourceRootsQuestionV1): Promise<readonly string[]> => host.sourceRoots ? host.sourceRoots(question) : Promise.resolve(question.mode === "entries" ? entryFilesV1(question.globs, host) : includeRootsV1(question.include, question.exclude, host));
  const roots = new Set<string>(entries ? await ask({mode: "entries", globs: entries}) : config.files.filter((file) => host.exists(file)));
  if (entries && !roots.size) fail(`Program entries select no TypeScript file: ${tsconfig}`);
  if (include.length) for (const file of await ask({mode: "include", include, exclude})) roots.add(file);
""")

edit(ENGINE, "cargo read",
     "for (const crate of [packages.length ? cratePatternsV1(packages, host) : {patterns: [], files: [], markers: []},",
     "for (const crate of [packages.length ? cratePatternsV1(packages, host, await crateIndexOfV1(host)) : {patterns: [], files: [], markers: []},")
edit(ENGINE, "script read",
     "const crates = facts.crates.length ? cratePatternsV1(facts.crates, host) : {patterns: [], files: [], markers: []}, crateDirs",
     "const crates = facts.crates.length ? cratePatternsV1(facts.crates, host, await crateIndexOfV1(host)) : {patterns: [], files: [], markers: []}, crateDirs")
edit(ENGINE, "dispatch read",
     "named = crates.length ? cratePatternsV1(crates, host) : {patterns: [], files: [], markers: []}, directories = crateDirectories.length",
     "named = crates.length ? cratePatternsV1(crates, host, await crateIndexOfV1(host)) : {patterns: [], files: [], markers: []}, directories = crateDirectories.length")
edit(ENGINE, "router index",
     "facts = literalFactsV1(body), index = crateIndexV1(host);",
     "facts = literalFactsV1(body), index = await crateIndexOfV1(host);")
edit(ENGINE, "router crates",
     "const crates = named.length ? cratePatternsV1(named, host) : {patterns: [], files: [], markers: []}, roots = crateDirs.length",
     "const crates = named.length ? cratePatternsV1(named, host, index) : {patterns: [], files: [], markers: []}, roots = crateDirs.length")

edit(SCHEMA, "walk definitions",
     '  "definitions": {\n    "Id": {',
     """  "definitions": {
    "ManifestWalkSkip": {
      "description": "What a Cargo manifest walk does not enter: whole paths below the workspace root and directory names.",
      "type": "object",
      "additionalProperties": false,
      "required": ["paths", "names"],
      "properties": {
        "paths": {"type": "array", "items": {"type": "string", "minLength": 1}},
        "names": {"type": "array", "items": {"type": "string", "minLength": 1}}
      }
    },
    "SourceRootsQuestion": {
      "description": "The question of a TypeScript file walk: the files that entry globs name, or that the include globs of a tsconfig select below its excludes.",
      "oneOf": [
        {"type": "object", "additionalProperties": false, "required": ["mode", "globs"], "properties": {"mode": {"const": "entries"}, "globs": {"type": "array", "items": {"type": "string", "minLength": 1}}}},
        {"type": "object", "additionalProperties": false, "required": ["mode", "include", "exclude"], "properties": {"mode": {"const": "include"}, "include": {"type": "array", "items": {"type": "string", "minLength": 1}}, "exclude": {"type": "array", "items": {"type": "string", "minLength": 1}}}}
      ]
    },
    "DerivedWalk": {
      "description": "The recipe of one repository walk that a derived fact stands for: the crate index of the Cargo manifests or the TypeScript files a question selects; a project that used it holds the digest of its answer instead of the directories the walk read.",
      "oneOf": [
        {"type": "object", "additionalProperties": false, "required": ["fn", "skip"], "properties": {"fn": {"const": "crate-index"}, "skip": {"$ref": "#/definitions/ManifestWalkSkip"}}},
        {"type": "object", "additionalProperties": false, "required": ["fn", "question"], "properties": {"fn": {"const": "source-roots"}, "question": {"$ref": "#/definitions/SourceRootsQuestion"}}}
      ]
    },
    "Id": {""")

edit(ENGINE, "walk reader",
     """const PROGRAM_PHASE = "build:input-precision";""",
     """const PROGRAM_PHASE = "build:input-precision";

/** ✅️ Admits the recipe of a derived repository walk against its closed schema. */
export function readDerivedWalkV1(value: unknown): DerivedWalkV1 {
  const failures = validateJsonSchemaSubset({$ref: "#/definitions/DerivedWalk"}, value, schema);
  if (failures.length) fail(`Derived walk refused: ${failures.slice(0, 3).join("; ")}`);
  return value as DerivedWalkV1;
}""")

# ---- script inputs
edit(SCRIPTS, "import engine",
     'import { routerCommandSpecifiersV1, routerCommandWordV1 } from "./🔀️router/🟨️.mjs";',
     'import { routerCommandSpecifiersV1, routerCommandWordV1 } from "./🔀️router/🟨️.mjs";\nimport { cargoManifestDirectoriesV1, crateNamesV1, entryFilesV1, includeRootsV1, readDerivedWalkV1 } from "../../../📥️inputs/🎯️precision/🟦️.ts";')

edit(SCRIPTS, "walk value",
     """/** 🧩️ The stored observations of every file a command walk visited, by the questions of the walks that visited it""",
     """/** 🔏️ The digest of the answer of a repository walk: what a project that used the walk depends on. */
export function derivedWalkAnswerV1(value) { return createHash("sha256").update(JSON.stringify(value)).digest("hex"); }

/** 🚶️ The answer of one repository walk (the crate index of the Cargo manifests, or the TypeScript files a question selects), read through the observed file system. */
function walkValueV1(recipe, workspaceRoot) {
  const local = path => join(workspaceRoot, path), list = directory => { try { return observedFs.readdirSync(local(directory), { withFileTypes: true }).map(entry => ({ name: entry.name, directory: entry.isDirectory(), file: entry.isFile(), link: entry.isSymbolicLink() })); } catch { return []; } };
  const readText = path => { try { return observedFs.readFileSync(local(path), "utf8"); } catch { return undefined; } };
  if (recipe.fn === "crate-index") return [...crateNamesV1(cargoManifestDirectoriesV1(list, recipe.skip), readText)];
  const host = { exists: path => existsSync(local(path)), listDirectory: directory => list(directory).map(({ name, directory }) => ({ name, directory })) };
  return recipe.question.mode === "entries" ? entryFilesV1(recipe.question.globs, host) : includeRootsV1(recipe.question.include, recipe.question.exclude, host);
}

/** 🪢️ One repository walk as a derived fact: the observations it read are one shared layer, named by the question and the digest of the answer, so a project that used the walk holds that one row instead of the directories the walk listed. */
function derivedWalkV1(workspaceRoot, recipe) {
  const question = JSON.stringify({ derive: 1, workspaceRoot: resolve(workspaceRoot), recipe }), cache = new Map(), value = memoizedObservedComputationV1(cache, question, () => walkValueV1(recipe, workspaceRoot)), stored = cache.get(question);
  if (stored && !walkFactsV1.has(stored.observations)) walkFactsV1.set(stored.observations, { row: { kind: "derived", path: question, fact: derivedWalkAnswerV1(value) } });
  return value;
}

/** 🧩️ The stored observations of every file a command walk visited, by the questions of the walks that visited it""")

edit(SCRIPTS, "re-ask",
     """  if (question?.walk !== 1 || !Array.isArray(question.roots)""",
     """  if (question?.derive === 1) {
    let recipe;
    try { recipe = readDerivedWalkV1(question.recipe); } catch { throw refuse(); }
    const asked = await observeDetachedV1(() => derivedWalkV1(question.workspaceRoot, recipe));
    return { fact: derivedWalkAnswerV1(asked.value), rows: support(asked) };
  }
  if (question?.walk !== 1 || !Array.isArray(question.roots)""")

edit(SCRIPTS, "digest query",
     """  ["generated", value => value?.generatedDependencies ?? []],
]);""",
     """  ["generated", value => value?.generatedDependencies ?? []],
  ["digest", derivedWalkAnswerV1],
]);""")

edit(SCRIPTS, "recipe case",
     """    case "target-sources": return targetScriptSources(target, workspaceRoot, scripts, recipe.projectRoot, recipe.nativeManifest);
    default:""",
     """    case "target-sources": return targetScriptSources(target, workspaceRoot, scripts, recipe.projectRoot, recipe.nativeManifest);
    case "crate-index": case "source-roots": return derivedWalkV1(workspaceRoot, readDerivedWalkV1(recipe));
    default:""")

edit(SCRIPTS, "api derive",
     """    async some(list, substrings) {
      const handle = closureHandleOfV1(list);""",
     """    async derive(recipe) {
      if (!late) return derivedWalkV1(workspaceRoot, recipe);
      const key = closureRecipeKeyV1(recipe);
      knownClosureRecipesV1.set(key, recipe); used.set(key, recipe);
      const entry = await ensureClosureEntryV1(workspaceRoot, recipe);
      recordClosureFactV1(JSON.stringify([workspaceRoot, key, ["digest"]]), JSON.stringify(closureAnswerV1(entry, ["digest"])));
      return entry.value;
    },
    async some(list, substrings) {
      const handle = closureHandleOfV1(list);""")

edit(SCRIPTS_D, "types",
     "export function resolveCommandWalkV1(",
     "export function derivedWalkAnswerV1(value: unknown): string;\nexport function resolveCommandWalkV1(")

# ---- project assembly
edit(ASSEMBLY, "import",
     "import { deriveInputsV1, inputPrecisionDecisionV1, inputPrecisionLanguageV1, inputPrecisionTextV1, readInputPrecisionV1, selectRuleV1 } from",
     "import { cargoManifestDirectoriesV1, deriveInputsV1, inputPrecisionDecisionV1, inputPrecisionLanguageV1, inputPrecisionTextV1, readInputPrecisionV1, selectRuleV1 } from")

edit(ASSEMBLY, "manifest walk",
     """/** 🦀️ Lists the directory of every Cargo manifest the build discovers, skipping generated trees. */
function precisionCargoManifestsV1(workspaceRoot) {
  const found = [], walk = (directory) => {
    for (const entry of readdirSync(join(workspaceRoot, directory), { withFileTypes: true })) {
      const path = buildPath(join(directory, entry.name));
      if (entry.isSymbolicLink() || path === "compose" || path === "temp/compose" || entry.name === ".🧬semio" || POLICY.generatedDirectories.includes(entry.name)) continue;
      if (entry.isDirectory()) walk(path);
      else if (entry.isFile() && entry.name === "Cargo.toml") found.push(directory);
    }
  };
  walk("");
  return found.sort();
}
""",
     """/** 🦀️ What the walk of the Cargo manifests skips: the generated trees and the composition outputs. */
const precisionManifestSkipV1 = () => ({ paths: ["compose", "temp/compose"], names: [".🧬semio", ...POLICY.generatedDirectories] });

/** 🦀️ Lists the directory of every Cargo manifest the build discovers, skipping generated trees. */
function precisionCargoManifestsV1(workspaceRoot) {
  return cargoManifestDirectoriesV1((directory) => readdirSync(join(workspaceRoot, directory), { withFileTypes: true }).map((entry) => ({ name: entry.name, directory: entry.isDirectory(), file: entry.isFile(), link: entry.isSymbolicLink() })), precisionManifestSkipV1());
}

const precisionCrateMapsV1 = new WeakMap();
/** 🚶️ The repository walks of one project's input precision as derived facts: the crate index and the TypeScript files a question selects are answered once per process by their closure entry, and the project records the digest of the answer instead of the directories the walk listed. */
function precisionWalksV1(closures) {
  return {
    crateNames: async () => { const entries = await closures.derive({ fn: "crate-index", skip: precisionManifestSkipV1() }); let map = precisionCrateMapsV1.get(entries); if (!map) precisionCrateMapsV1.set(entries, map = new Map(entries)); return map; },
    sourceRoots: (question) => closures.derive({ fn: "source-roots", question }),
  };
}
""")

edit(ASSEMBLY, "precise head",
     """async function preciseInputsV1(project, name, command, policy, root, workspaceRoot, facts, scripts, intern) {
  const { document, host } = precisionContextV1(workspaceRoot, facts, scripts), outputs =""",
     """async function preciseInputsV1(project, name, command, policy, root, workspaceRoot, facts, scripts, intern, closures) {
  const { document, host: shared } = precisionContextV1(workspaceRoot, facts, scripts), host = closures.late ? Object.assign(Object.create(shared), precisionWalksV1(closures)) : shared, outputs =""")

edit(ASSEMBLY, "precise call",
     "policy = await preciseInputsV1(json.name, name, inherited.options?.command ?? inherited.command, policy, root, workspaceRoot, facts, scripts, internCommandSources);",
     "policy = await preciseInputsV1(json.name, name, inherited.options?.command ?? inherited.command, policy, root, workspaceRoot, facts, scripts, internCommandSources, closures);")

# ---- engine law: ports
edit(ENGINE_LAW, "law import",
     "import {commandWordsV1, deriveInputsV1, dispatchBranchesV1, entryFilesV1,",
     "import {commandWordsV1, crateNamesV1, deriveInputsV1, dispatchBranchesV1, entryFilesV1, includeRootsV1,")

edit(ENGINE_LAW, "law ports",
     """  expect(validate.validate(pointed(join(here, ".."), rules), {...rules, $schema: 1})).toBe(false);
});""",
     """  expect(validate.validate(pointed(join(here, ".."), rules), {...rules, $schema: 1})).toBe(false);
});

test("answering the repository walks through the derived-walk ports derives exactly what walking in place derives", async () => {
  const asked: string[] = [], ports = (base: InputPrecisionHostV1): InputPrecisionHostV1 => ({...base, crateNames: async () => { asked.push("crates"); return crateNamesV1(base.cargoManifests(), base.readText); }, sourceRoots: async (question) => { asked.push(question.mode); return question.mode === "entries" ? entryFilesV1(question.globs, base) : includeRootsV1(question.include, question.exclude, base); }});
  for (const row of fixture.derive) {
    const rule = selectRuleV1(document, row.identity as any)!;
    if (rule.evidence === "assumed") continue;
    expect(await deriveInputsV1(document, rule, row.identity as any, ports(fixtureHost())), JSON.stringify(row.identity)).toEqual(await deriveInputsV1(document, rule, row.identity as any, fixtureHost()));
  }
  expect(asked).toContain("crates");
  const entries = ["{workspaceRoot}/app/src/**/*.ts", "!{workspaceRoot}/app/src/skip/**/*"];
  expect(await programInputsV1("app/tsconfig.json", ports(fixtureHost()), entries)).toEqual(await programInputsV1("app/tsconfig.json", fixtureHost(), entries));
  expect(await programInputsV1("app/tsconfig.json", ports(fixtureHost()))).toEqual(await programInputsV1("app/tsconfig.json", fixtureHost()));
  expect(asked).toContain("entries");
  expect(asked).toContain("include");
});""")

README = "🧰️framework/🛍️products/🦑️repo/README.md"
edit(README, "readme en", "- **Cheap generations after an environment move (graph generations).**", '- **Repository walks of the input precision are derived facts (graph builder memory).** The crate index of the Cargo manifests (a walk of the whole repository: 126 thousand directory listings in the root project) and the TypeScript files that entry globs or the include globs of a tsconfig select (92 thousand listings in the dev composition) were recorded as raw `readdir` rows by every project that used them. They are derived facts now (closure recipes `crate-index` and `source-roots`, `derive: 1` questions of `🧭️script-inputs`, schema `DerivedWalk` of the precision engine): a project holds ONE `closure` row (the digest of the answer), the listings are the support of one derived fact kept once, the walk is asked again only when a listing went stale, and an equal answer keeps every holder valid. Measured on the real tree (one project per run, same scratch store): the root project 206,302 -> 79,250 rows and `s-dev-composition` 183,817 -> 92,188, the live heap after the inference -0.46 / -0.13 GB (-0.19 / -0.13 GB after a collection), the peak resident set 5.93 -> 5.63 GB and 3.58 -> 3.17 GB; the project values are equal field by field to the ones of the direct walk. Oracle: a Python walk of its own copy of the tree after 22 recorded and 40 x 14 seeded edits of directories and files anywhere (`🚶️repository-walks`): the derived answer equals the full walk after every edit, and whenever every supporting row still holds the answer is the one the holders hold.\n\n' + "- **Cheap generations after an environment move (graph generations).**")
edit(README, "readme de", "- **Günstige Generationen nach einer Umgebungsverschiebung (Graph-Generationen).**", '- **Durchläufe des Repositorys in der Input-Präzision sind abgeleitete Fakten (Speicher des Graph-Aufbaus).** Der Crate-Index der Cargo-Manifeste (ein Durchlauf des ganzen Repositorys: 126 Tausend Verzeichnislisten im Wurzelprojekt) und die TypeScript-Dateien, die Entry-Globs oder die Include-Globs einer tsconfig wählen (92 Tausend Listen in der Dev-Komposition), wurden von jedem Projekt, das sie nutzte, als rohe `readdir`-Zeilen aufgezeichnet. Sie sind jetzt abgeleitete Fakten (Abschluss-Rezepte `crate-index` und `source-roots`, `derive: 1`-Fragen von `🧭️script-inputs`, Schema `DerivedWalk` der Präzisions-Engine): ein Projekt hält EINE `closure`-Zeile (den Digest der Antwort), die Listen sind die Stütze eines einzigen abgeleiteten Fakts, der Durchlauf wird nur neu gefragt, wenn eine Liste veraltet ist, und eine gleiche Antwort lässt jeden Halter gültig. Gemessen auf dem echten Baum (ein Projekt je Lauf, derselbe Arbeitsspeicher-Store): das Wurzelprojekt 206.302 -> 79.250 Zeilen und `s-dev-composition` 183.817 -> 92.188, der lebende Heap nach der Ableitung -0,46 / -0,13 GB (-0,19 / -0,13 GB nach einer Sammlung), der Spitzen-Resident 5,93 -> 5,63 GB und 3,58 -> 3,17 GB; die Projektwerte sind Feld für Feld gleich denen des direkten Durchlaufs. Orakel: ein Python-Durchlauf einer eigenen Kopie des Baums nach 22 aufgezeichneten und 40 x 14 gesäten Änderungen von Verzeichnissen und Dateien überall (`🚶️repository-walks`): die abgeleitete Antwort gleicht dem vollen Durchlauf nach jeder Änderung, und solange jede stützende Zeile gilt, ist die Antwort die, die die Halter halten.\n\n' + "- **Günstige Generationen nach einer Umgebungsverschiebung (Graph-Generationen).**")

out = {"edits": edits}
for e in edits:
    text = open(os.path.join(ROOT, e["file"]), encoding="utf8").read()
    n = text.count(e["old"])
    if n != 1:
        sys.exit(f"anchor {e['name']!r}: {n} occurrences in {e['file']}")
json.dump(out, open(os.path.join(HERE, "edits.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("edits", len(edits))
