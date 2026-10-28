import {expect, test} from "bun:test";
import Ajv from "ajv";
import {minimatch} from "minimatch";
import {deriveInputsV1, entryFilesV1, readInputPrecisionV1, type InputPrecisionHostV1} from "../../🟦️.ts";
import document from "../../🔣️.json" with {type: "json"};
import corpus from "./🔣️.json" with {type: "json"};
import corpusSchema from "./🧬️schema/🔣️.json" with {type: "json"};

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
} as unknown as InputPrecisionHostV1;
const stripped = (entry: string) => entry.replace("{workspaceRoot}/", "");
const oracle = (entries: readonly string[]) => Object.keys(tree).filter((path) => /\.[cm]?[jt]sx?$/u.test(path) && !path.split("/").includes("node_modules") && entries.some((entry) => !entry.startsWith("!") && minimatch(path, stripped(entry), {dot: true})) && !entries.some((entry) => entry.startsWith("!") && minimatch(path, stripped(entry.slice(1)), {dot: true}))).sort();
const rule = (entries: readonly string[]) => ({id: "imports-fixture", description: "fixture", select: {project: "p"}, evidence: "static", replaces: ["^default"], reads: [{imports: {entries}}]});

/** 🔭️ The `imports` read selects the module files of its entry globs and keeps the import closure: the corpus is strict, the selection equals minimatch, the closure equals the expectation. */
test("the corpus satisfies its strict schema and the document admits the read", () => {
  const ajv = new Ajv({strict: true, allErrors: true});
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(() => readInputPrecisionV1({...document, rules: [...document.rules, rule(["{workspaceRoot}/pkg/**/*.ts"])]})).not.toThrow();
  expect(() => readInputPrecisionV1({...document, rules: [...document.rules, {...rule(["{workspaceRoot}/pkg/**/*.ts"]), reads: [{imports: {entries: ["{workspaceRoot}/pkg/**/*.ts"], extra: true}}]}]})).toThrow();
});

test("entry selection equals minimatch and the derived files equal the expected closure", async () => {
  for (const row of corpus.cases) {
    expect(entryFilesV1(row.entries, host), row.id).toEqual(row.selected);
    expect(oracle(row.entries), `minimatch ${row.id}`).toEqual(row.selected);
    const doc = readInputPrecisionV1({...document, rules: [...document.rules, rule(row.entries)]});
    const derived = await deriveInputsV1(doc, doc.rules.at(-1)!, {project: "p", target: "t"}, host);
    expect([...derived.files], row.id).toEqual(row.closure);
    expect(derived.drops).toEqual(["^default"]);
  }
});

test("a read that selects no module file is refused", async () => {
  for (const row of corpus.refused) {
    const doc = readInputPrecisionV1({...document, rules: [...document.rules, rule(row.entries)]});
    await expect(deriveInputsV1(doc, doc.rules.at(-1)!, {project: "p", target: "t"}, host), row.id).rejects.toThrow(/no module file/u);
  }
});
