import {expect, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname, join} from "node:path";
import Ajv from "ajv";
import {minimatch} from "minimatch";
import {commandWordsV1, deriveInputsV1, dispatchBranchesV1, globMatchesV1, packageNameV1, parseJsoncV1, programInputsV1, readInputPrecisionV1, selectRuleV1, type InputPrecisionHostV1} from "../🟦️.ts";
import fixtureSchema from "./🧬️schema/🔣️.json" with {type: "json"};
import fixture from "./🔣️.json" with {type: "json"};
import schema from "../🧬️schema/🔣️.json" with {type: "json"};
import rules from "../🔣️.json" with {type: "json"};

const here = import.meta.dir;
const findRoot = (directory: string): string => (existsSync(join(directory, "nx.json")) ? directory : dirname(directory) === directory ? process.env.SEMIO_WORKSPACE_ROOT ?? "/Users/ueli/Documents/semio" : findRoot(dirname(directory)));
const workspace = findRoot(here);
const files = fixture.workspace.files as Record<string, string>;
const SPEC = /(?:from\s*|import\s*\(\s*|import\s*)["'](\.{1,2}\/[^"']+)["']/gu;
const normalize = (path: string) => path.split("/").reduce<string[]>((parts, part) => (part === ".." ? (parts.pop(), parts) : part === "." || part === "" ? parts : [...parts, part]), []).join("/");

/** 🧰️ The fixture workspace as an engine host: an in-memory tree, a regex relative-import closure and the declared Cargo graph. */
function fixtureHost(): InputPrecisionHostV1 {
  const closure = (entries: readonly string[]) => {
    const seen = new Set<string>(), pending = [...entries];
    while (pending.length) {
      const file = pending.pop()!;
      if (seen.has(file) || !(file in files)) continue;
      seen.add(file);
      for (const match of (files[file] ?? "").matchAll(SPEC)) {
        const base = normalize(`${dirname(file)}/${match[1]}`), hit = [base, `${base}.ts`, `${base}/🟦️.ts`].find((candidate) => candidate in files);
        if (hit) pending.push(hit);
      }
    }
    return [...seen].sort();
  };
  const directories = new Set(Object.keys(files).flatMap((path) => path.split("/").slice(0, -1).map((_, index, parts) => parts.slice(0, index + 1).join("/"))));
  return {
    exists: (path) => path in files || directories.has(path),
    readText: (path) => files[path],
    listDirectory: (path) => {
      const prefix = path ? `${path}/` : "", names = new Map<string, boolean>();
      for (const file of Object.keys(files)) if (file.startsWith(prefix)) { const rest = file.slice(prefix.length), [name] = rest.split("/"); names.set(name!, rest.includes("/")); }
      return [...names].map(([name, directory]) => ({name, directory}));
    },
    kinds: fixture.workspace.kinds as InputPrecisionHostV1["kinds"],
    importClosure: async (entries) => closure(entries),
    cargoManifests: () => Object.keys(files).filter((path) => path.endsWith("Cargo.toml")).map((path) => path.slice(0, -"Cargo.toml".length).replace(/\/$/u, "")),
    cargoRoots: (crate) => (fixture.workspace.cargoRoots as Record<string, string[]>)[crate] ?? [crate],
    cargoSources: (root) => (fixture.workspace.cargoSources as Record<string, string[]>)[root],
    cargoToolchain: fixture.workspace.cargoToolchain,
  };
}

const oracle = JSON.parse(spawnSync("python3", ["-I", join(here, "🐍️oracle/🐍️.py"), join(here, "🔣️.json")], {encoding: "utf8", maxBuffer: 64 * 1024 * 1024}).stdout);
const frozen = fixture.expect as Record<string, any>;
const document = readInputPrecisionV1(fixture.document);

test("the fixture satisfies its closed schema and the document its derivation schema", () => {
  const validate = new Ajv({strict: false});
  expect(validate.validate(fixtureSchema, fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(validate.validate(schema, fixture.document), JSON.stringify(validate.errors)).toBe(true);
  expect(validate.validate(schema, rules), JSON.stringify(validate.errors)).toBe(true);
});

test("input globs agree with the fixture, with minimatch and with the Python oracle", () => {
  fixture.globs.forEach((row, index) => {
    expect(globMatchesV1(row.pattern, row.path), `${row.pattern} ~ ${row.path}`).toBe(row.match);
    expect(minimatch(row.path, row.pattern), `minimatch ${row.pattern} ~ ${row.path}`).toBe(row.match);
    expect(oracle.globs[index]).toBe(row.match);
  });
});

test("script command words, JSONC documents and Cargo package names agree with the fixture and the Python oracle", () => {
  fixture.commands.forEach((row, index) => {
    expect(commandWordsV1(row.command) ?? null).toEqual(row.words as any);
    expect(oracle.commands[index]).toEqual(row.words);
  });
  fixture.jsonc.forEach((row, index) => {
    expect(parseJsoncV1(row.text)).toEqual(row.value);
    expect(oracle.jsonc[index]).toEqual(row.value);
  });
  fixture.packageNames.forEach((row, index) => {
    expect(packageNameV1(row.manifest) ?? null).toBe(row.name);
    expect(oracle.packageNames[index]).toBe(row.name);
    const parsed = Bun.TOML.parse(row.manifest) as {package?: {name?: string}};
    expect(parsed.package?.name ?? null).toBe(row.name);
  });
});

test("dispatcher branches carry the same facts in TypeScript and in the Python oracle", () => {
  fixture.dispatch.forEach((row, index) => {
    const branches = dispatchBranchesV1(row.source);
    expect(JSON.parse(JSON.stringify(branches))).toEqual(oracle.dispatch[index]);
    expect(branches.map((branch) => branch.key)).toEqual(["alpha", "beta", "gamma", "delta"]);
    expect(branches[0]!.crates).toEqual(["crate-a"]);
    expect(branches[3]!.cargo).toBe(false);
  });
});

test("every fixture target derives the frozen narrowest inputs, and unselected targets derive nothing", async () => {
  const host = fixtureHost(), derived: unknown[] = [];
  for (const row of fixture.derive) {
    const rule = selectRuleV1(document, row.identity as any);
    expect(rule, JSON.stringify(row.identity)).toBeDefined();
    derived.push(rule!.evidence === "assumed" ? {rule: rule!.id, assumed: true} : await deriveInputsV1(document, rule!, row.identity as any, host));
  }
  const unselected = fixture.unselected.map((identity) => selectRuleV1(document, identity as any)?.id ?? null);
  if (process.env.SEMIO_FREEZE === "1") {
    writeFileSync(join(here, "🔣️.json"), JSON.stringify({...fixture, expect: {derive: derived, unselected}}, null, 1) + "\n");
    return;
  }
  expect(derived).toEqual(frozen.derive);
  expect(unselected).toEqual(frozen.unselected);
});

test("every narrowed target keeps the retained toolchain identity its replaced groups carried", () => {
  expect(document.retained).toEqual(["sharedGlobals", "{projectRoot}/📋️project.json"]);
  for (const row of frozen.derive as any[]) {
    const rule = document.rules.find((candidate) => candidate.id === row.rule)!;
    if (row.assumed || !rule.replaces.length) continue;
    for (const name of document.retained) expect(row.groups, `${row.rule} keeps ${name}`).toContain(name);
  }
});

test("a document or derivation that cannot be bounded is refused with the stated reason", async () => {
  const host = fixtureHost();
  for (const row of fixture.refusals as any[]) {
    if (row.kind === "document") {
      const mutated = JSON.parse(JSON.stringify(fixture.document));
      if (row.mutate === "wholeRepository") mutated.rules[2].reads.push({globs: ["{workspaceRoot}/**/*"]});
      if (row.mutate === "wholeRepositoryDot") mutated.rules[2].reads.push({globs: ["{workspaceRoot}/./**/*"]});
      if (row.mutate === "unknownProfile") mutated.rules[0].reads = [{profile: "no-such-profile"}];
      if (row.mutate === "duplicateRule") mutated.rules[1].id = mutated.rules[0].id;
      if (row.mutate === "unknownKey") mutated.surprise = true;
      expect(() => readInputPrecisionV1(mutated), row.mutate).toThrow(new RegExp(row.message, "iu"));
    } else {
      const derived = {...document, rules: [{id: row.rule, evidence: "static", description: "refusal", select: {project: row.identity.project}, replaces: [], reads: row.reads}]};
      await expect(deriveInputsV1(derived as any, derived.rules[0] as any, row.identity, host)).rejects.toThrow(new RegExp(row.message, "iu"));
    }
  }
});

test("a tsconfig program equals what tsc --listFilesOnly lists, and the fixture rule never selects an excluded file", async () => {
  const root = mkdtempSync(join(tmpdir(), "input-precision-"));
  try {
    for (const [path, text] of Object.entries(files)) { mkdirSync(dirname(join(root, path)), {recursive: true}); writeFileSync(join(root, path), text); }
    const tsc = join(workspace, "node_modules/typescript/bin/tsc"), run = spawnSync(process.execPath, [tsc, "-p", join(root, "app/tsconfig.json"), "--listFilesOnly"], {encoding: "utf8", cwd: root});
    expect(run.status, run.stderr + run.stdout).toBe(0);
    const listed = run.stdout.split("\n").filter((line) => line.startsWith(root + "/") && !line.includes("/node_modules/")).map((line) => line.slice(root.length + 1)).sort();
    const program = await programInputsV1("app/tsconfig.json", fixtureHost()), declared = new Set([...program.roots, ...program.files.filter((file) => file.endsWith(".ts"))]);
    expect(listed.length).toBeGreaterThan(0);
    for (const file of listed) expect(declared.has(file), `tsc lists ${file}`).toBe(true);
    for (const file of program.roots) expect(listed).toContain(file);
    expect(program.roots).not.toContain("app/src/skip/x.ts");
    expect(program.files).toContain("shared/package.json");
    expect(program.files).toContain("tsconfig.base.json");
  } finally {
    rmSync(root, {recursive: true, force: true});
  }
});

test.skipIf(process.env.SEMIO_INPUT_PRECISION_REPO !== "1")("the repository rules are valid, every owner route matches an owner declaration and no rule selects the whole repository", () => {
  const document = readInputPrecisionV1(rules);
  const owners = spawnSync("git", ["-c", "core.quotepath=false", "grep", "-l", "\"workspaceCommand\"", "--", ":(glob)**/📋️project.json"], {cwd: workspace, encoding: "utf8"}).stdout.split("\n").filter((path) => path && !path.startsWith(".🧬semio/"));
  const declared: string[] = [];
  for (const path of owners) {
    const project = JSON.parse(readFileSync(join(workspace, path), "utf8"));
    for (const [target, value] of Object.entries<any>(project.targets ?? {})) if (value.metadata?.semio?.workspaceCommand) declared.push(JSON.stringify([value.metadata.semio.workspaceCommand, project.name, target, dirname(path)]));
  }
  expect(document.routes.map((route) => JSON.stringify([route.words, route.project, route.target, route.root])).sort()).toEqual(declared.sort());
  expect(document.retained).toEqual(["sharedGlobals", "{projectRoot}/📋️project.json"]);
  expect(document.rules.filter((rule) => rule.evidence === "assumed").every((rule) => rule.description.length >= 40 && rule.reads.length > 0)).toBe(true);
  expect(document.rules.filter((rule) => rule.evidence !== "assumed").every((rule) => rule.reads.length === 0 || rule.reads.some((read) => !("input" in read)))).toBe(true);
});
