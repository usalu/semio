#!/usr/bin/env python3
"""Shared helpers of the lawcov-laws generators: emoji path repair, real-path lookup, test-folder emission."""
import json, os

ROOT = "/Users/ueli/Documents/semio"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
TOOLCHAIN = "🧰️framework/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts"
VS = "️"
SKIP = {0x200D, 0xFE0F}


def emoji(c):
    o = ord(c)
    return o in (0x25AA, 0x25AB, 0x25B6, 0x25C0, 0x25FB, 0x25FC, 0x25FD, 0x25FE) or 0x1F000 <= o <= 0x1FAFF or 0x2300 <= o <= 0x23FF or 0x2600 <= o <= 0x27BF or 0x2B00 <= o <= 0x2BFF


def fix(text):
    out = []
    stripped = text.replace(VS, "")
    for i, c in enumerate(stripped):
        out.append(c)
        if emoji(c) and not (i + 1 < len(stripped) and stripped[i + 1] == "‍") and not (i > 0 and stripped[i - 1] == "‍"):
            out.append(VS)
    return "".join(out)


def N(s):
    return s.replace(VS, "")


def real_unit(norm):
    cur = os.path.join(ROOT, fix(LIB))
    for part in N(norm).split("/"):
        cand = [e for e in os.listdir(cur) if N(e) == part]
        assert len(cand) == 1, (cur, part, cand)
        cur = os.path.join(cur, cand[0])
    return cur


def deep(value):
    if isinstance(value, str):
        return fix(value)
    if isinstance(value, list):
        return [deep(v) for v in value]
    if isinstance(value, dict):
        return {deep(k): deep(v) for k, v in value.items()}
    return value


def write_test(unit_norm, test_name, ts, corpus, schema, oracle, fixdeep=True):
    unit = real_unit(unit_norm)
    folder = os.path.join(unit, fix("🧪tests"), fix(test_name))
    os.makedirs(os.path.join(folder, fix("🧬schema")), exist_ok=True)
    rel_toolchain = os.path.relpath(os.path.join(ROOT, fix(TOOLCHAIN)), folder)
    ts = ts.replace("@@TOOLCHAIN@@", rel_toolchain).replace("@@LIB@@", os.path.relpath(os.path.join(ROOT, fix(LIB)), folder))
    files = {
        fix("🟦.ts"): fix(ts),
        fix("🔣.json"): json.dumps(deep(corpus) if fixdeep else corpus, ensure_ascii=False, indent=1) + "\n",
        os.path.join(fix("🧬schema"), fix("🔣.json")): json.dumps(schema, ensure_ascii=False, indent=1) + "\n",
        fix("🐍oracle.py"): fix(oracle),
    }
    for name, text in files.items():
        with open(os.path.join(folder, name), "w", encoding="utf8") as fh:
            fh.write(text)
    return folder, len(os.path.relpath(folder, ROOT)) + 12


def lines(text):
    return text.strip("\n").split("\n")


def std_schema(title, rules, extra=None, expect_item=None):
    case_props = {
        "id": {"type": "string", "minLength": 1},
        "rule": {"enum": rules},
        "files": {"type": "object", "additionalProperties": {"type": "array", "items": {"type": "string"}}},
        "generate": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["path", "count"], "properties": {"path": {"type": "string"}, "lines": {"type": "array", "items": {"type": "string"}}, "count": {"type": "integer", "minimum": 1}}}},
        "expect": {"type": "array", "items": expect_item or {"type": "array", "minItems": 4, "maxItems": 4, "items": [{"type": "string"}, {"type": "string"}, {"type": "string"}, {"type": "integer"}]}},
        "expectCounts": {"type": "object", "additionalProperties": {"type": "integer"}},
        "defect": {"type": "string"},
        "oracle": {"type": "boolean"},
        "kindFilter": {"type": "string"},
        "git": {"type": "boolean"},
        "expectList": {"type": "array", "items": {"type": "string"}},
    }
    case_props.update(extra or {})
    return {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": title,
        "type": "object",
        "additionalProperties": False,
        "required": ["version", "cases"],
        "properties": {
            "version": {"const": 1},
            "cases": {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": False, "required": ["id", "rule", "files"], "oneOf": [{"properties": {"expect": True}, "required": ["expect"]}, {"properties": {"expectCounts": True}, "required": ["expectCounts"]}, {"properties": {"expectList": True}, "required": ["expectList"]}], "properties": case_props}},
        },
    }


STD_TS = '''/** @@DOC@@ */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import { testPythonV1 } from "@@TOOLCHAIN@@";
import { gitSpawnEnv } from "@@LIB@@/🏃process/🌿environment/🌳git/🟦.ts";
@@IMPORTS@@
import corpus from "./🔣.json" with { type: "json" };
import corpusSchema from "./🧬schema/🔣.json" with { type: "json" };

type Breach = { id: string; kind: string; priority?: string; line?: number; summary?: string };
const rules: Record<string, (root: string, row: any) => readonly any[]> = {
@@RULES@@
};
const artifacts = realpathSync(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir()));
const libraryRoot = resolve(import.meta.dir, "@@LIB@@"), libraryRel = "🧰framework/🛍products/🦑repo/🔨modules/📚library";
const admit = (root: string) => {
  for (const rel of ["🔣taxonomy.json", "🧬schema/🔣.json"]) { mkdirSync(dirname(join(root, libraryRel, rel)), { recursive: true }); copyFileSync(join(libraryRoot, rel), join(root, libraryRel, rel)); }
  for (const args of [["init", "--quiet"], ["add", "-A"]]) expect(spawnSync("git", args, { cwd: root, env: gitSpawnEnv(), encoding: "utf8" }).status).toBe(0);
};
const order = (rows: readonly unknown[]) => [...rows].sort((a, b) => { const x = JSON.stringify(a), y = JSON.stringify(b); return x < y ? -1 : x > y ? 1 : 0; });
const triples = (breaches: readonly Breach[]) => order(breaches.map((b) => [b.id, b.kind, b.priority ?? "", b.line ?? 0]));
const counts = (breaches: readonly Breach[]) => { const out: Record<string, number> = {}; for (const b of breaches) out[b.priority ?? ""] = (out[b.priority ?? ""] ?? 0) + 1; return out; };
const observe = (row: any, all: readonly any[]) => { const found = row.kindFilter ? all.filter((b: any) => b.kind.startsWith(row.kindFilter)) : all; return row.expectCounts ? counts(found) : row.expectList ? order(found) : triples(found); };
const wanted = (row: any) => (row.expectCounts ? row.expectCounts : row.expectList ? order(row.expectList) : order(row.expect));
const files = (row: any) => { const out: Record<string, string> = {}; for (const [path, lines] of Object.entries(row.files as Record<string, string[]>)) out[path] = lines.length ? lines.join("\\n") + "\\n" : ""; for (const g of row.generate ?? []) for (let i = 0; i < g.count; i++) out[g.path.replaceAll("{i}", String(i))] = (g.lines ?? []).map((l: string) => l.replaceAll("{i}", String(i))).join("\\n"); return out; };
const run = (row: any) => {
  const root = mkdtempSync(join(artifacts, "lawcov-"));
  try {
    for (const [path, text] of Object.entries(files(row))) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); }
    if (row.git) admit(root);
    return observe(row, rules[row.rule]!(root, row));
  } finally { rmSync(root, { recursive: true, force: true }); }
};
const modelled = (corpus.cases as any[]).filter((row) => row.oracle !== false);
const model = (() => {
  const run = spawnSync(testPythonV1() ?? "python3", ["-I", resolve(import.meta.dir, "./🐍oracle.py"), resolve(import.meta.dir, "./🔣.json")], { encoding: "utf8" });
  expect(run.status, run.stderr).toBe(0);
  return JSON.parse(run.stdout) as Record<string, unknown>;
})();

test("the corpus satisfies the strict independent validator and names every case once", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(new Set(corpus.cases.map((row) => row.id)).size).toBe(corpus.cases.length);
  expect(Object.keys(model).sort()).toEqual(modelled.map((row) => row.id).sort());
});

for (const row of corpus.cases as any[]) {
  (row.defect ? test.failing : test)(`${row.rule}: ${row.id}`, () => {
    const found = run(row);
    expect(found, "law versus corpus").toEqual(wanted(row) as never);
    if (row.oracle !== false) expect(row.expectCounts ? model[row.id] : order(model[row.id] as unknown[]), "oracle versus corpus").toEqual(wanted(row) as never);
  }, row.git ? 120000 : 30000);
}
@@EXTRA@@'''


def std_ts(doc, imports, rules, extra=""):
    return STD_TS.replace("@@DOC@@", doc).replace("@@IMPORTS@@", imports).replace("@@RULES@@", rules).replace("@@EXTRA@@", extra)


TRIPLE = {"type": "array", "minItems": 4, "maxItems": 4, "items": [{"type": "string"}, {"type": "string"}, {"type": "string"}, {"type": "integer"}]}


def pure_schema(title, rules, props, required, expect_item=None):
    row = {"id": {"type": "string", "minLength": 1}, "rule": {"enum": rules}, "oracle": {"type": "boolean"}, "defect": {"type": "string"}, "expect": {"type": "array", "items": expect_item or TRIPLE}}
    row.update(props)
    return {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": title,
        "type": "object",
        "additionalProperties": False,
        "required": ["version", "cases"],
        "properties": {"version": {"const": 1}, "cases": {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": False, "required": ["id", "rule", "expect"] + required, "properties": row}}},
    }


PURE_TS = '''/** @@DOC@@ */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { resolve } from "node:path";
import Ajv from "ajv";
import { testPythonV1 } from "@@TOOLCHAIN@@";
@@IMPORTS@@
import corpus from "./🔣.json" with { type: "json" };
import corpusSchema from "./🧬schema/🔣.json" with { type: "json" };

type Breach = { id: string; kind: string; priority?: string; line?: number };
const rules: Record<string, (row: any) => readonly Breach[]> = {
@@RULES@@
};
const order = (rows: readonly unknown[]) => [...rows].sort((a, b) => { const x = JSON.stringify(a), y = JSON.stringify(b); return x < y ? -1 : x > y ? 1 : 0; });
const triples = (breaches: readonly Breach[]) => order(breaches.map((b) => [b.id, b.kind, b.priority ?? "", b.line ?? 0]));
const modelled = (corpus.cases as any[]).filter((row) => row.oracle !== false);
const model = (() => {
  const run = spawnSync(testPythonV1() ?? "python3", ["-I", resolve(import.meta.dir, "./🐍oracle.py"), resolve(import.meta.dir, "./🔣.json")], { encoding: "utf8" });
  expect(run.status, run.stderr).toBe(0);
  return JSON.parse(run.stdout) as Record<string, unknown[]>;
})();

test("the corpus satisfies the strict independent validator and names every case once", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(new Set(corpus.cases.map((row) => row.id)).size).toBe(corpus.cases.length);
  expect(Object.keys(model).sort()).toEqual(modelled.map((row) => row.id).sort());
});

for (const row of corpus.cases as any[]) {
  (row.defect ? test.failing : test)(`${row.rule}: ${row.id}`, () => {
    expect(triples(rules[row.rule]!(row)), "law versus corpus").toEqual(order(row.expect) as never);
    if (row.oracle !== false) expect(order(model[row.id]!), "oracle versus corpus").toEqual(order(row.expect) as never);
  }, 30000);
}
@@EXTRA@@'''


def pure_ts(doc, imports, rules, extra=""):
    return PURE_TS.replace("@@DOC@@", doc).replace("@@IMPORTS@@", imports).replace("@@RULES@@", rules).replace("@@EXTRA@@", extra)


def pure_oracle_main(dispatch_expr):
    return '''
cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
result = {}
for c in cases:
    if c.get("oracle") is False:
        continue
    result[c["id"]] = %s
output(result)
''' % dispatch_expr


def val_schema(title, rules, props=None):
    row = {
        "id": {"type": "string", "minLength": 1}, "rule": {"enum": rules}, "args": {"type": "array"},
        "files": {"type": "object", "additionalProperties": {"type": "array", "items": {"type": "string"}}},
        "oracle": {"type": "boolean"}, "defect": {"type": "string"}, "git": {"type": "boolean"}, "expect": True,
    }
    row.update(props or {})
    return {
        "$schema": "http://json-schema.org/draft-07/schema#", "title": title, "type": "object", "additionalProperties": False, "required": ["version", "cases"],
        "properties": {"version": {"const": 1}, "cases": {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": False, "required": ["id", "rule", "expect"], "properties": row}}},
    }


VAL_TS = '''/** @@DOC@@ */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import { testPythonV1 } from "@@TOOLCHAIN@@";
import { gitSpawnEnv } from "@@LIB@@/🏃process/🌿environment/🌳git/🟦.ts";
@@IMPORTS@@
import corpus from "./🔣.json" with { type: "json" };
import corpusSchema from "./🧬schema/🔣.json" with { type: "json" };

const rules: Record<string, (root: string, ...args: any[]) => unknown> = {
@@RULES@@
};
const artifacts = realpathSync(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir()));
const libraryRoot = resolve(import.meta.dir, "@@LIB@@"), libraryRel = "🧰framework/🛍products/🦑repo/🔨modules/📚library";
const canon = (value: unknown): unknown => (value === undefined ? null : JSON.parse(JSON.stringify(value)));
const admit = (root: string) => {
  for (const rel of ["🔣taxonomy.json", "🧬schema/🔣.json"]) { mkdirSync(dirname(join(root, libraryRel, rel)), { recursive: true }); copyFileSync(join(libraryRoot, rel), join(root, libraryRel, rel)); }
  for (const args of [["init", "--quiet"], ["add", "-A"]]) expect(spawnSync("git", args, { cwd: root, env: gitSpawnEnv(), encoding: "utf8" }).status).toBe(0);
};
const run = (row: any) => {
  const root = mkdtempSync(join(artifacts, "lawcov-"));
  try {
    for (const [path, lines] of Object.entries((row.files ?? {}) as Record<string, string[]>)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), lines.length ? lines.join("\\n") + "\\n" : ""); }
    if (row.git) admit(root);
    return canon(rules[row.rule]!(root, ...(row.args ?? [])));
  } finally { rmSync(root, { recursive: true, force: true }); }
};
const modelled = (corpus.cases as any[]).filter((row) => row.oracle !== false);
const model = (() => {
  const run = spawnSync(testPythonV1() ?? "python3", ["-I", resolve(import.meta.dir, "./🐍oracle.py"), resolve(import.meta.dir, "./🔣.json")], { encoding: "utf8" });
  expect(run.status, run.stderr).toBe(0);
  return JSON.parse(run.stdout) as Record<string, unknown>;
})();

test("the corpus satisfies the strict independent validator and names every case once", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(new Set(corpus.cases.map((row) => row.id)).size).toBe(corpus.cases.length);
  expect(Object.keys(model).sort()).toEqual(modelled.map((row) => row.id).sort());
});

for (const row of corpus.cases as any[]) {
  (row.defect ? test.failing : test)(`${row.rule}: ${row.id}`, () => {
    expect(run(row), "law versus corpus").toEqual(row.expect);
    if (row.oracle !== false) expect(model[row.id], "oracle versus corpus").toEqual(row.expect);
  }, row.git ? 120000 : 30000);
}
@@EXTRA@@'''


def val_ts(doc, imports, rules, extra=""):
    return VAL_TS.replace("@@DOC@@", doc).replace("@@IMPORTS@@", imports).replace("@@RULES@@", rules).replace("@@EXTRA@@", extra)


def jsort(rows):
    return sorted(rows, key=lambda r: json.dumps(r, ensure_ascii=False, separators=(",", ":")).encode("utf-16-be"))


SORT_TS = "const brs = (breaches: readonly any[]) => breaches.map((x) => [x.id, x.kind, x.priority ?? \"\", x.line ?? 0]).sort((a, b) => { const p = JSON.stringify(a), q = JSON.stringify(b); return p < q ? -1 : p > q ? 1 : 0; });"
