import { describe, expect, test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import ts from "typescript";
import fixtures from "../🧫️fixtures/🔣️.json" with { type: "json" };
import { maskCommentsV1, readArgumentV1, scanToolCallsV1, scanToolSpawnsV1, shellCommandsV1, toolScanLanguageV1, toolSpawnScopeV1 } from "../../🟦️.ts";
import { testRipgrepV1 } from "../../../🧪️testing/🧰️toolchain/🟦️.ts";
import { spawnSync } from "node:child_process";

describe("direct spawn scanner", () => {
  for (const row of fixtures.cases) {
    test(`finds exactly the starts of registered tools in ${row.path}`, () => {
      const found = scanToolSpawnsV1(row.path, row.text).map(site => `${site.line} ${site.form} ${site.tool}`).sort();
      expect(found).toEqual([...row.expected].sort());
      if ("scope" in row) expect(scanToolSpawnsV1(row.path, row.text).every(site => site.scope === row.scope)).toBe(true);
    });
  }
  test("reads the first argument across nested brackets, strings and templates", () => {
    expect(readArgumentV1("f(a(b, c), d)", 2)).toEqual({ text: "a(b, c)", end: 9 });
    expect(readArgumentV1("f(`x${y(1, 2)}`, 3)", 2).text).toBe("`x${y(1, 2)}`");
    expect(readArgumentV1("f([1, 2], 3)", 2).text).toBe("[1, 2]");
    expect(readArgumentV1("f(\"a,b\", 3)", 2).text).toBe("\"a,b\"");
  });
  test("separates the commands of a shell line and skips wrappers and assignments", () => {
    expect(shellCommandsV1("A=1 sudo cargo build && cd x; bun test | tee out")).toEqual(["cargo", "cd", "bun", "tee"]);
    expect(shellCommandsV1("exec nice -n 5 go test")).toEqual(["go"]);
  });
  test("classifies scope and language", () => {
    expect(toolSpawnScopeV1("a/🧪️tests/b.ts")).toBe("test");
    expect(toolSpawnScopeV1("a/b.test.ts")).toBe("test");
    expect(toolSpawnScopeV1("a/b_test.go")).toBe("test");
    expect(toolSpawnScopeV1("a/🏭️generator/b.ts")).toBe("prod");
    expect(toolScanLanguageV1("x/📋️project.json")).toBe("declared");
    expect(toolScanLanguageV1("x/🐚️.sh")).toBe("shell");
    expect(toolScanLanguageV1("x/readme.md")).toBeUndefined();
  });
  test("finds the calls of the tool interface with a literal tool id", () => {
    const text = "// runToolV1(\"cargo\")\nawait runToolV1('bun', []);\nrunToolSyncV1(\"git\", a);\nrunToolV1(id, []);\nconst s = `runToolV1(\"go\")`;";
    expect(scanToolCallsV1("a/x.ts", text).map(call => `${call.line} ${call.callee} ${call.tool}`)).toEqual(["2 runToolV1 bun", "3 runToolSyncV1 git"]);
  });
  test("counts the same Rust and Go spawns as ripgrep", () => {
    const rg = testRipgrepV1();
    if (rg === null) return;
    const text = fixtures.cases.filter(row => row.path.endsWith(".rs") || row.path.endsWith(".go")).map(row => row.text).join("\n");
    const count = spawnSync(rg, ["-o", "--no-config", "-e", "Command::new\\(\"[a-z]+\"|exec\\.Command(?:Context)?\\([^\"]*\"[a-z]+\""], { input: text, encoding: "utf8" }).stdout.trim().split("\n").filter(Boolean).length;
    const mine = fixtures.cases.filter(row => row.path.endsWith(".rs") || row.path.endsWith(".go")).flatMap(row => scanToolSpawnsV1(row.path, row.text)).length;
    expect(mine).toBe(count - 1);
  });
});

describe("comment masking", () => {
  const directory = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
  const sources = (() => { const out: string[] = []; for (const name of ["📖️registry", "📌️pin", "🔎️resolve", "🏃️run", "🔍️scan", "🛡️guard", "🌿️environment"]) { try { for (const file of readdirSync(join(directory, name))) if (file.endsWith(".ts")) out.push(join(directory, name, file)); } catch { continue; } } return out; })();
  test("blanks exactly the comments the TypeScript compiler finds and nothing else", () => {
    expect(sources.length).toBeGreaterThan(3);
    for (const path of sources) {
      const text = readFileSync(path, "utf8"), file = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true), ranges = new Map<number, number>();
      const note = (found: ts.CommentRange[] | undefined): void => { for (const range of found ?? []) ranges.set(range.pos, range.end); };
      const visit = (node: ts.Node): void => { note(ts.getLeadingCommentRanges(text, node.getFullStart())); note(ts.getTrailingCommentRanges(text, node.getEnd())); ts.forEachChild(node, visit); };
      visit(file);
      note(ts.getLeadingCommentRanges(text, 0));
      const masked = maskCommentsV1(text, "ts"), expected = text.split("");
      for (const [from, to] of ranges) for (let index = from; index < to; index++) if (expected[index] !== "\n") expected[index] = " ";
      expect(masked).toBe(expected.join(""));
    }
  });
  test("keeps strings in view and blanks them on request", () => {
    const text = "const a = \"// not a comment\"; // real\nconst b = `x ${1} y`;";
    expect(maskCommentsV1(text, "ts")).toBe("const a = \"// not a comment\";        \nconst b = `x ${1} y`;");
    expect(maskCommentsV1(text, "ts", true)).toBe("const a = \"                \";        \nconst b = `        `;");
    expect(maskCommentsV1("let r = r#\"// x\"#; /* a /* b */ c */ Command::new(\"go\")", "rust", true)).toContain("Command::new(\"  \")");
  });
});
