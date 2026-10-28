import {expect, test} from "bun:test";
import {existsSync, readFileSync} from "node:fs";
import {dirname, join} from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import {deriveInputsV1, innerCommandWordsV1, readInputPrecisionV1, routerBranchV1, routerClassBodyV1, routerClassNameV1, type InputPrecisionHostV1} from "../../🟦️.ts";
import document from "../../🔣️.json" with {type: "json"};
import corpus from "./🔣️.json" with {type: "json"};
import corpusSchema from "./🧬️schema/🔣️.json" with {type: "json"};

const findRoot = (directory: string): string => (existsSync(join(directory, "nx.json")) ? directory : dirname(directory) === directory ? process.env.SEMIO_WORKSPACE_ROOT ?? "/Users/ueli/Documents/semio" : findRoot(dirname(directory)));
const workspace = findRoot(import.meta.dir);
const tree = corpus.tree as Record<string, string>;
const directories = new Set(Object.keys(tree).flatMap((path) => path.split("/").slice(0, -1).map((_, index, parts) => parts.slice(0, index + 1).join("/"))));
const SPEC = /(?:from\s*|import\s*)["'](\.{1,2}\/[^"']+)["']/gu;
const normalize = (path: string) => path.split("/").reduce<string[]>((parts, part) => (part === ".." ? (parts.pop(), parts) : part === "." || part === "" ? parts : [...parts, part]), []).join("/");
const host = {
  exists: (path: string) => path in tree || directories.has(path),
  readText: (path: string) => tree[path],
  listDirectory: (path: string) => {
    const prefix = path ? `${path}/` : "", names = new Map<string, boolean>();
    for (const file of Object.keys(tree)) if (file.startsWith(prefix)) { const rest = file.slice(prefix.length), [name] = rest.split("/"); names.set(name!, rest.includes("/")); }
    return [...names].map(([name, directory]) => ({name, directory}));
  },
  importClosure: async (entries: readonly string[]) => {
    const seen = new Set<string>(), pending = [...entries];
    while (pending.length) {
      const file = pending.pop()!;
      if (seen.has(file) || !(file in tree)) continue;
      seen.add(file);
      for (const match of tree[file]!.matchAll(SPEC)) { const target = normalize(`${file.split("/").slice(0, -1).join("/")}/${match[1]}`); if (target in tree) pending.push(target); }
    }
    return [...seen].sort();
  },
  kinds: {},
  cargoManifests: () => Object.keys(tree).filter((path) => path.endsWith("Cargo.toml")).map((path) => path.slice(0, -"Cargo.toml".length).replace(/\/$/u, "")),
  cargoRoots: (crate: string) => (corpus.cargo.roots as Record<string, string[]>)[crate] ?? [crate],
  cargoSources: (root: string) => (corpus.cargo.sources as Record<string, string[]>)[root],
  cargoToolchain: corpus.cargo.toolchain,
} as unknown as InputPrecisionHostV1;
const rule = {id: "router-fixture", description: "fixture", select: {project: "p"}, evidence: "static", replaces: ["default", "^default", "reads"], reads: [{router: {source: "rt/📜️script.ts"}}]};
const branchRule = {...rule, id: "router-branches", reads: [{router: {source: "rt2/📜️script.ts", classes: "rt2/routes/🟦️.ts"}}]};
const derive = (word: string, using = rule, command = `repo ./📜️script.ts ${word}`) => {
  const doc = readInputPrecisionV1({...document, rules: [...document.rules, using]});
  return deriveInputsV1(doc, doc.rules.at(-1)!, {project: "p", target: word, command}, host);
};

/** 🛤️ The router read takes the class registered under the command word of the INNER command: the corpus is strict, the class body equals the class the TypeScript parser finds, literals bound the derivation and what cannot be bounded is refused. */
test("the corpus satisfies its strict schema and the document admits the read", () => {
  expect(new Ajv({strict: true, allErrors: true}).validate(corpusSchema, corpus)).toBe(true);
  expect(() => readInputPrecisionV1({...document, rules: [...document.rules, rule]})).not.toThrow();
  expect(() => readInputPrecisionV1({...document, rules: [...document.rules, {...rule, reads: [{router: {source: "rt/📜️script.ts", extra: 1}}]}]})).toThrow();
});

test("the inner command words are the words after the last script token", () => {
  for (const row of corpus.words) expect(innerCommandWordsV1(row.command), row.command).toEqual(row.words as never ?? undefined);
});

const parsed = (source: string, name: string): string => {
  const file = ts.createSourceFile("x.ts", source, ts.ScriptTarget.Latest, true);
  let found = "";
  file.forEachChild((node) => { if (ts.isClassDeclaration(node) && node.name?.text === name) found = source.slice(node.getStart(file), node.end).replace(/^export\s+/u, ""); });
  return found;
};
const registrations = (source: string): [string, string][] => [...source.matchAll(/register\(\s*"([^"]+)"\s*,\s*(\w+)/gu)].map((match) => [match[1]!, match[2]!]);

test("the class body equals the class the TypeScript parser finds, in the fixture and in the real framework router", () => {
  const fixtureSource = tree["rt/📜️script.ts"]!;
  for (const [word, name] of registrations(fixtureSource)) { expect(routerClassNameV1(fixtureSource, word), word).toBe(name); expect(routerClassBodyV1(fixtureSource, name), word).toBe(parsed(fixtureSource, name)); }
  const real = readFileSync(join(workspace, "🧰️framework/📦️packages/🦀️rust/📜️script.ts"), "utf8");
  const rows = registrations(real);
  expect(rows.length).toBeGreaterThan(20);
  for (const [word, name] of rows) expect(routerClassBodyV1(real, routerClassNameV1(real, word)!), word).toBe(parsed(real, name));
  expect(routerClassNameV1(fixtureSource, "test-missing")).toBeUndefined();
  const dashboard = readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🛤️task-routes/🟦️.ts"), "utf8"), test = routerClassBodyV1(dashboard, "TestScript")!;
  expect(test).toBe(parsed(dashboard, "TestScript"));
  for (const word of ["unit", "windows", "dispatch", "language", "process-deadline"]) expect(routerBranchV1(test, word), word).toContain(`"${word}"`);
  expect(routerBranchV1(test, "no-such-branch")).toBeUndefined();
});

test("a registered command is bounded by the packages, files and directories its class names", async () => {
  for (const row of corpus.derived) {
    const derived = await derive(row.word);
    expect([...derived.files], row.word).toEqual(row.files);
    for (const pattern of row.patterns) expect(derived.patterns, `${row.word} ${pattern}`).toContain(pattern);
    expect(derived.patterns.filter((pattern) => !pattern.startsWith("!")).length, `${row.word} no extra pattern`).toBe(row.patterns.length);
    expect(derived.drops).toEqual(["default", "^default", "reads"]);
  }
});

test("a branch of the class narrows the command and the string lists it mentions join its packages", async () => {
  for (const row of corpus.branches) {
    const derived = await derive("t", branchRule, row.command);
    expect([...derived.files], row.command).toEqual(row.files);
    for (const pattern of row.patterns) expect(derived.patterns, `${row.command} ${pattern}`).toContain(pattern);
    expect(derived.patterns.filter((pattern) => !pattern.startsWith("!")).length, `${row.command} no extra pattern`).toBe(row.patterns.length);
  }
  const whole = await derive("t", branchRule, "repo ./📜️script.ts test other");
  expect([...whole.files]).toEqual(["crates/a/src/lib.rs", "crates/b/src/lib.rs", "tests/helper.ts", "tests/unit/🟦️.ts"]);
});

test("a command that cannot be bounded is refused", async () => {
  for (const row of corpus.refused) await expect(derive(row.word), row.word).rejects.toThrow(new RegExp(row.message, "u"));
});
