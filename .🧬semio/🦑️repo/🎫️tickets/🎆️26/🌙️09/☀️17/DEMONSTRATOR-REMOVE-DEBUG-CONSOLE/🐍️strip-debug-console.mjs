#!/usr/bin/env bun
/** Drop console.* calls whose source text contains `[DEBUG]`, without breaking if/else structure. */
import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";

const CONSOLE = new Set(["log", "debug", "info", "warn", "error"]);
const files = process.argv.slice(2);
if (files.length === 0) {
  console.error("usage: strip-debug-console.mjs <files...>");
  process.exit(1);
}

function isDebugConsoleCall(node, sourceFile) {
  if (!ts.isCallExpression(node)) return false;
  const expr = node.expression;
  if (!ts.isPropertyAccessExpression(expr)) return false;
  if (!ts.isIdentifier(expr.expression) || expr.expression.text !== "console") return false;
  if (!ts.isIdentifier(expr.name) || !CONSOLE.has(expr.name.text)) return false;
  return node.getText(sourceFile).includes("[DEBUG]");
}

function replaceRange(source, start, end, replacement) {
  return source.slice(0, start) + replacement + source.slice(end);
}

function stripStatementTail(source, stmtEnd) {
  let end = stmtEnd;
  while (end < source.length && (source[end] === " " || source[end] === "\t")) end++;
  if (source[end] === ";") end++;
  if (source[end] === "\r" && source[end + 1] === "\n") end += 2;
  else if (source[end] === "\n") end++;
  return end;
}

function stripFile(path) {
  const original = readFileSync(path, "utf8");
  const sourceFile = ts.createSourceFile(path, original, ts.ScriptTarget.Latest, true, path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  let source = original;
  const removals = [];

  function collect(node) {
    if (isDebugConsoleCall(node, sourceFile)) removals.push(node);
    ts.forEachChild(node, collect);
  }
  collect(sourceFile);
  removals.sort((a, b) => b.getStart(sourceFile) - a.getStart(sourceFile));

  let removed = 0;
  for (const call of removals) {
    const parent = call.parent;
    const start = call.getStart(sourceFile);
    const end = call.getEnd();

    if (ts.isExpressionStatement(parent) && parent.expression === call) {
      const stmtEnd = stripStatementTail(source, parent.getEnd());
      source = replaceRange(source, parent.getStart(sourceFile), stmtEnd, "");
      removed++;
      continue;
    }

    if (ts.isExpressionStatement(parent) && parent.expression === call) {
      const ifParent = parent.parent;
      if (ts.isIfStatement(ifParent) && ifParent.thenStatement === parent) {
        source = replaceRange(source, ifParent.getStart(sourceFile), stripStatementTail(source, ifParent.getEnd()), "");
        removed++;
        continue;
      }
      const stmtEnd = stripStatementTail(source, parent.getEnd());
      source = replaceRange(source, parent.getStart(sourceFile), stmtEnd, "");
      removed++;
      continue;
    }

    if (ts.isBlock(parent) && ts.isIfStatement(parent.parent) && parent.statements.every((stmt) => ts.isExpressionStatement(stmt) && isDebugConsoleCall(stmt.expression, sourceFile))) {
      const ifParent = parent.parent;
      source = replaceRange(source, ifParent.getStart(sourceFile), stripStatementTail(source, ifParent.getEnd()), "");
      removed++;
      continue;
    }

    if (ts.isBinaryExpression(parent) && parent.right === call && parent.operatorToken.kind === ts.SyntaxKind.QuestionQuestionToken) {
      source = replaceRange(source, start, end, "(() => undefined)");
      removed++;
      continue;
    }

    if (ts.isArrowFunction(parent) && parent.body === call) {
      source = replaceRange(source, start, end, "undefined");
      removed++;
      continue;
    }

    if (ts.isPropertyAssignment(parent) && parent.initializer === call) {
      source = replaceRange(source, start, end, "(() => undefined)");
      removed++;
      continue;
    }

    if (ts.isCallExpression(parent) && parent.arguments.includes(call)) {
      source = replaceRange(source, start, end, "undefined");
      removed++;
      continue;
    }

    console.warn(`[strip] ${path}:${sourceFile.getLineAndCharacterOfPosition(start).line + 1} manual: ${call.getText(sourceFile).slice(0, 100)}`);
  }

  if (source !== original) writeFileSync(path, source);
  return removed;
}

let total = 0;
for (const file of files) total += stripFile(file);
console.log(`[strip] removed ${total} debug console call(s) across ${files.length} file(s)`);
