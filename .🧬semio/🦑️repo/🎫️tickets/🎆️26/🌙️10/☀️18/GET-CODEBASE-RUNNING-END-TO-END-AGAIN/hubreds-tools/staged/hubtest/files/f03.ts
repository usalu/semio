import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import ts from "typescript";
import { describe, expect, it } from "vitest";
import { TEST_LEVEL_BUDGET_MS, resolveTestLevel } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";

const script = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript/📜️script.ts");

describe("hub unit test phase budget", () => {
  it("floors the default unit run at the quick level because its fixed transform and import cost exceeds the fundamental budget", () => {
    const syntax = ts.createSourceFile(script, readFileSync(script, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const owner = syntax.statements.find((node) => ts.isClassDeclaration(node) && node.name?.text === "TestScript");
    expect(owner && ts.isClassDeclaration(owner)).toBe(true);
    const run = owner && ts.isClassDeclaration(owner) ? owner.members.find((node) => ts.isMethodDeclaration(node) && node.name.getText(syntax) === "run") : undefined;
    const calls: ts.CallExpression[] = [];
    const visit = (node: ts.Node): void => { if (ts.isCallExpression(node) && node.expression.getText(syntax) === "resolveTestLevel") calls.push(node); ts.forEachChild(node, visit); };
    if (run) visit(run);
    expect(calls).toHaveLength(1);
    const floor = calls[0]!.arguments[1];
    expect(floor && ts.isStringLiteral(floor)).toBe(true);
    const saved = process.env.SEMIO_TEST_LEVEL;
    try {
      delete process.env.SEMIO_TEST_LEVEL;
      const { level } = resolveTestLevel([], floor && ts.isStringLiteral(floor) ? (floor.text as keyof typeof TEST_LEVEL_BUDGET_MS) : "fundamental");
      expect(TEST_LEVEL_BUDGET_MS[level]).toBeGreaterThanOrEqual(120_000);
    } finally {
      if (saved === undefined) delete process.env.SEMIO_TEST_LEVEL; else process.env.SEMIO_TEST_LEVEL = saved;
    }
  });
});
