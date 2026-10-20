#!/usr/bin/env python3
"""🔍️ Builds a bun test harness from the (overlay-modified) ide-census test: the real classifier code and the SQLite oracle are sliced out of the test source, not re-implemented.
Usage: census-harness.py OVERLAY_ROOT   -> writes <overlay>/<dashboard-control>/harness.test.ts and prints the path. Run it with `bun test <path>` from OVERLAY_ROOT (cwd = physical census root)."""
import os, re, sys
O = sys.argv[1]
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️dashboard-control"
src = open(os.path.join(O, L, "🟦️.ts"), encoding="utf-8").read()
a = src.index("type CensusKind"); b = src.index("function manifestControlStrings")
block = src[a:b]
c = src.index("    let normalized="); d = src.index("    for(const row of censusFixture.pathCases")
oracle = src[c:d]
head = '''import { expect, test } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { closeSync, constants, existsSync, fstatSync, lstatSync, mkdirSync, mkdtempSync, openSync, readdirSync, readFileSync, readSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import ts from "typescript";
import censusSchema from "./🧬️schema/🔍️ide-census.json" with { type: "json" };
import censusFixture from "./🧫️fixtures/🔍️ide-census.json" with { type: "json" };
'''
body = '''
test("census schema accepts the fixture", () => {
  expect(new Ajv({ strict: false }).validate(censusSchema, censusFixture)).toBe(true);
});
test("census path cases agree between the TypeScript classifier and the SQLite oracle", () => {
  const database = new Database(":memory:");
  try {
''' + oracle + '''
    let checked = 0;
    for (const row of censusFixture.pathCases as readonly any[]) {
      const content = "content" in row ? row.content : undefined;
      expect(retiredIdePath(row.path, row.kind, false, content), row.path).toBe(row.expected);
      expect(classify(row.path, row.kind, false, content), row.path).toBe(row.expected);
      checked++;
    }
    expect(checked).toBe(censusFixture.pathCases.length);
    for (const probe of [[".cursor", "directory", "retired"], [".windsurf", "directory", "retired"], [".github/hooks", "directory", "retired"], [".github/hooks/repo.json", "file", "retired"], [".github/workflows", "directory", "accepted"], [".github/hooks-notes", "directory", "accepted"], ["a/.cursor/plans/x.plan.md", "file", "retired"], [".cursorrules-notes", "file", "accepted"]] as const) {
      expect(retiredIdePath(probe[0], probe[1]), probe[0]).toBe(probe[2]);
      expect(classify(probe[0], probe[1]), probe[0]).toBe(probe[2]);
    }
  } finally { database.close(); }
});
test("a physical walk finds .cursor, .windsurf and .github/hooks and nothing else", () => {
  const base = mkdtempSync(join(process.env.HARNESS_TMP ?? "/private/tmp/ide-free", "walk-"));
  try {
    const make = (rel: string) => { mkdirSync(join(base, rel), { recursive: true }); };
    make("src"); make(".github/workflows"); make("notes");
    const retired: string[] = [], queue = [""];
    const walk = () => { while (queue.length) { const relative = queue.pop()!; for (const entry of readdirSync(join(base, relative), { withFileTypes: true })) { const path = relative ? relative + "/" + entry.name : entry.name, kind: CensusKind = entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : "file", disposition = retiredIdePath(path, kind); if (disposition === "excluded") continue; if (disposition === "retired") { retired.push(path); continue; } if (entry.isDirectory()) queue.push(path); } } };
    walk(); expect(retired).toEqual([]);
    make(".cursor/plans"); make(".windsurf"); make(".github/hooks"); queue.push(""); walk();
    expect(retired.sort()).toEqual([".cursor", ".github/hooks", ".windsurf"]);
  } finally { rmSync(base, { recursive: true, force: true }); }
});
'''
out = os.path.join(O, L, "harness.test.ts")
open(out, "w", encoding="utf-8").write(head + block + body)
print(out)
