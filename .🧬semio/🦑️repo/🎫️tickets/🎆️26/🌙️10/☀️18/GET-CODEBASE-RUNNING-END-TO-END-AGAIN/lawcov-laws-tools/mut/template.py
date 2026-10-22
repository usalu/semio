import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

MUT_TS = '''/** @@DOC@@ */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { resolve, sep } from "node:path";
import Ajv from "ajv";
import { testPythonV1 } from "@@TOOLCHAIN@@";
import { policyReadFileSafe, policyReadModuleTreeSource, policyReadWitPackageSource } from "@@LIB@@/🔍discovery/📖source-access/🟦.ts";
import { policyReadRustPolicyFamily, policyReadRustPolicySource } from "@@LIB@@/🔍discovery/📖source-access/🦀rust-evidence/🟦.ts";
import { readSourceFamilySource } from "@@LIB@@/../../../../🔨modules/📚compiler/📦packages/🟦typescript/📖syntax/🌳module-tree/🟦.ts";
import * as law from "../../🟦.ts";
import corpus from "./🔣.json" with { type: "json" };
import corpusSchema from "./🧬schema/🔣.json" with { type: "json" };

const here = import.meta.dir.split(sep);
const root = here.slice(0, here.indexOf("🧰framework")).join(sep);
const readNode = (node: any): string => {
  if (node.parts) return node.parts.map(readNode).join(node.join ?? "\n");
  let text =
    node.reader === "rust-family" ? policyReadRustPolicyFamily(root, node.paths) : node.reader === "rust-source" ? policyReadRustPolicySource(root, node.paths[0]) : node.reader === "module-tree" ? policyReadModuleTreeSource(root, node.paths[0]) : node.reader === "wit-package" ? policyReadWitPackageSource(root, node.paths[0]) : node.reader === "source-family" ? readSourceFamilySource(`${root}/${node.paths[0]}`, { tests: node.tests === true }) : policyReadFileSafe(root, node.paths[0]);
  if (node.names) return node.names.filter((name: string) => text.includes(`${node.prefix}${name}`)).join("\n");
  if (node.slice) text = text.slice(text.indexOf(node.slice[0]), text.indexOf(node.slice[1]));
  return text;
};
const cache = new Map<string, string>();
const source = (name: string): string => {
  if (!cache.has(name)) cache.set(name, readNode((corpus.sources as any)[name]));
  return cache.get(name)!;
};
const outcome = (value: unknown) => (typeof value === "boolean" ? { accepted: value, messages: [] as string[] } : { accepted: (value as unknown[]).length === 0, messages: (value as unknown[]).map(String) });
const invoke = (row: any, overrides: Record<string, string>) => {
  try { return outcome((law as any)[row.fn](...row.args.map((arg: any) => (typeof arg === "string" ? (overrides[arg] ?? source(arg)) : arg.root ? root : arg.lit)))); } catch { return { accepted: false, messages: ["<throws>"] }; }
};
const measure = (row: any) => {
  const base = invoke(row, {});
  const m = row.mutation;
  if (m.op === "none") return { present: true, baseAccepted: base.accepted, mutatedAccepted: base.accepted, added: [] as string[] };
  const text = source(m.target);
  const present = text.includes(m.text);
  const mutated = invoke(row, { [m.target]: m.op === "remove" ? text.split(m.text).join("") : `${text}\\n${m.text}` });
  return { present, baseAccepted: base.accepted, mutatedAccepted: mutated.accepted, added: [...new Set(mutated.messages.filter((message) => !base.messages.includes(message)))].sort() };
};
const modelled = (corpus.cases as any[]).filter((row) => row.oracle !== false);
const model = (() => {
  const run = spawnSync(testPythonV1() ?? "python3", ["-I", resolve(import.meta.dir, "./🐍oracle.py"), resolve(import.meta.dir, "./🔣.json")], { encoding: "utf8", maxBuffer: 1 << 28 });
  expect(run.status, run.stderr).toBe(0);
  return JSON.parse(run.stdout) as Record<string, { present: boolean }>;
})();

test("the corpus satisfies the strict independent validator and names every case once", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(new Set(corpus.cases.map((row) => row.id)).size).toBe(corpus.cases.length);
  expect(Object.keys(model).sort()).toEqual(modelled.map((row) => row.id).sort());
});

test("every named law is exported and every source resolves to non-empty text", () => {
  for (const row of corpus.cases as any[]) expect(typeof (law as any)[row.fn], row.fn).toBe("function");
  for (const name of Object.keys(corpus.sources)) expect(source(name).length, name).toBeGreaterThan(0);
});

for (const row of corpus.cases as any[]) {
  (row.defect ? test.failing : test)(`${row.fn}: ${row.id}`, () => {
    const got = measure(row);
    expect(got, "law versus corpus").toEqual(row.expect);
    if (row.oracle !== false) expect(model[row.id], "independent source reader versus corpus").toEqual({ present: row.expect.present });
  }, 60000);
}
'''

SCHEMA = {
    "definitions": {"node": {"type": "object", "additionalProperties": False, "properties": {
        "join": {"type": "string"}, "parts": {"type": "array", "minItems": 1, "items": {"$ref": "#/definitions/node"}},
        "reader": {"enum": ["rust-source", "rust-family", "module-tree", "file", "source-family", "wit-package"]}, "tests": {"type": "boolean"}, "paths": {"type": "array", "minItems": 1, "items": {"type": "string", "minLength": 1}},
        "slice": {"type": "array", "minItems": 2, "maxItems": 2, "items": {"type": "string"}}, "names": {"type": "array", "items": {"type": "string"}}, "prefix": {"type": "string"}},
        "oneOf": [{"required": ["parts"]}, {"required": ["reader", "paths"]}]}},
    "$schema": "http://json-schema.org/draft-07/schema#", "title": "Source Mutation Corpus", "type": "object", "additionalProperties": False, "required": ["version", "sources", "cases"],
    "properties": {
        "version": {"const": 1},
        "sources": {"type": "object", "minProperties": 1, "additionalProperties": {"$ref": "#/definitions/node"}},
        "cases": {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": False, "required": ["id", "fn", "args", "mutation", "expect"], "properties": {
            "id": {"type": "string", "minLength": 1}, "fn": {"type": "string", "minLength": 1}, "oracle": {"type": "boolean"}, "defect": {"type": "string"},
            "args": {"type": "array", "items": {"oneOf": [{"type": "string"}, {"type": "object"}]}},
            "mutation": {"oneOf": [{"type": "object", "additionalProperties": False, "required": ["op"], "properties": {"op": {"const": "none"}}},
                                   {"type": "object", "additionalProperties": False, "required": ["op", "target", "text"], "properties": {"op": {"enum": ["remove", "append"]}, "target": {"type": "string"}, "text": {"type": "string", "minLength": 1}}}]},
            "expect": {"type": "object", "additionalProperties": False, "required": ["present", "baseAccepted", "mutatedAccepted", "added"], "properties": {"present": {"type": "boolean"}, "baseAccepted": {"type": "boolean"}, "mutatedAccepted": {"type": "boolean"}, "added": {"type": "array", "items": {"type": "string"}}}}}}}}}
