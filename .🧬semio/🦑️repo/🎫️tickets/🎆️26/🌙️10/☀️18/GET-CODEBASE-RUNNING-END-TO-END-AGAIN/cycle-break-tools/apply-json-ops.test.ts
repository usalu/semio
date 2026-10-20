/** 🧪 Language-agnostic corpus for the structured JSON edits of the delta applier, cross-checked against Python's json module as the independent canonical formatter. */
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { applyJsonOps } from "./apply-plan.ts";
import type { JsonOp } from "./plan.ts";

type Case = { name: string; doc: unknown; ops: JsonOp[]; expected: unknown; problems: string[] };
const corpus = JSON.parse(readFileSync(resolve(import.meta.dir, "fixtures/json-ops.json"), "utf8")) as { cases: Case[] };
const canonical = (value: unknown): string => spawnSync("python3", ["-I", "-c", "import json,sys; sys.stdout.write(json.dumps(json.load(sys.stdin), indent=2, ensure_ascii=False) + '\\n')"], { input: JSON.stringify(value), encoding: "utf8" }).stdout;

describe("applyJsonOps", () => {
  for (const c of corpus.cases) test(c.name, () => {
    const problems: string[] = [];
    const out = applyJsonOps(canonical(c.doc), c.ops, "doc", problems);
    expect(JSON.parse(out)).toEqual(c.expected);
    expect(out).toBe(canonical(c.expected));
    expect(problems.map(p => p.replace(/^doc: /u, ""))).toEqual(c.problems);
  });
  test("refuses a document that does not round-trip as 2-space JSON", () => {
    const problems: string[] = [];
    expect(applyJsonOps('{"a":1}', [], "doc", problems)).toBe('{"a":1}');
    expect(problems).toEqual(["json does not round-trip: doc"]);
  });
});
