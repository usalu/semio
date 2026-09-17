#!/usr/bin/env bun
/** 🩹️ Corrected `[DEBUG]` console stripper (replaces the fused-`if` bug of
 * 26/09/17/DEMONSTRATOR-REMOVE-DEBUG-CONSOLE/🐍️strip-debug-console.mjs): removes the OUTERMOST statement that holds nothing but
 * `[DEBUG]` console calls (a bare call, an `if`/`else`, a loop or a block), together with its own line indentation; a
 * debug-only statement in a single-statement slot that still has live siblings becomes `{}`; expression positions become
 * `undefined`. Usage: bun 🐍️strip-debug-console-fixed.mjs <files...> */
import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";

const CONSOLE = new Set(["log", "debug", "info", "warn", "error"]);

const isDebugCall = (node, sf) =>
  ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && ts.isIdentifier(node.expression.expression) && node.expression.expression.text === "console" && CONSOLE.has(node.expression.name.text) && node.getText(sf).includes("[DEBUG]");

const debugOnly = (stmt, sf) => {
  if (!stmt) return true;
  if (ts.isExpressionStatement(stmt)) return isDebugCall(stmt.expression, sf);
  if (ts.isBlock(stmt)) return stmt.statements.length > 0 && stmt.statements.every((s) => debugOnly(s, sf));
  if (ts.isIfStatement(stmt)) return debugOnly(stmt.thenStatement, sf) && debugOnly(stmt.elseStatement, sf);
  if (ts.isForOfStatement(stmt) || ts.isForInStatement(stmt) || ts.isForStatement(stmt) || ts.isWhileStatement(stmt)) return debugOnly(stmt.statement, sf);
  return false;
};

const isStatementList = (node) => ts.isBlock(node) || ts.isSourceFile(node) || ts.isCaseClause(node) || ts.isDefaultClause(node) || ts.isModuleBlock(node);

function stripFile(path) {
  const original = readFileSync(path, "utf8");
  const sf = ts.createSourceFile(path, original, ts.ScriptTarget.Latest, true, path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const edits = new Map();
  const visit = (node) => {
    if (isDebugCall(node, sf)) {
      if (ts.isExpressionStatement(node.parent)) {
        let target = node.parent;
        while (!isStatementList(target.parent) && debugOnly(target.parent, sf)) target = target.parent;
        if (ts.isBlock(target.parent) && target.parent.statements.every((s) => debugOnly(s, sf)) && !isStatementList(target.parent.parent) && debugOnly(target.parent.parent, sf)) target = target.parent.parent;
        if (isStatementList(target.parent)) {
          let start = target.getStart(sf);
          while (start > 0 && (original[start - 1] === " " || original[start - 1] === "\t")) start--;
          let end = target.getEnd();
          while (end < original.length && (original[end] === " " || original[end] === "\t")) end++;
          if (original[end] === ";") end++;
          if (original[end] === "\r" && original[end + 1] === "\n") end += 2;
          else if (original[end] === "\n") end++;
          const atLineStart = start === 0 || original[start - 1] === "\n";
          edits.set(target.getStart(sf), { start: atLineStart ? start : target.getStart(sf), end: atLineStart ? end : target.getEnd(), text: "" });
        } else {
          edits.set(target.getStart(sf), { start: target.getStart(sf), end: target.getEnd(), text: "{}" });
        }
        return;
      }
      edits.set(node.getStart(sf), { start: node.getStart(sf), end: node.getEnd(), text: ts.isBinaryExpression(node.parent) || ts.isPropertyAssignment(node.parent) ? "(() => undefined)" : "undefined" });
      return;
    }
    ts.forEachChild(node, visit);
  };
  visit(sf);
  const all = [...edits.values()];
  const ordered = all.filter((edit) => !all.some((other) => other !== edit && other.start <= edit.start && other.end >= edit.end)).sort((a, b) => b.start - a.start);
  let source = original;
  for (const edit of ordered) source = source.slice(0, edit.start) + edit.text + source.slice(edit.end);
  if (source !== original) writeFileSync(path, source);
  return ordered.length;
}

let total = 0;
for (const file of process.argv.slice(2)) total += stripFile(file);
console.log(`[DEBUG] strip removed ${total} debug console site(s) across ${process.argv.length - 2} file(s)`);
