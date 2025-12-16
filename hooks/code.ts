#!/usr/bin/env tsx
// SPDX-License-Identifier: AGPL-3.0-only
import { execSync } from "child_process";
import { existsSync, readFileSync, unlinkSync, writeFileSync } from "fs";
import { basename, dirname, join } from "path";
import * as ts from "typescript";

type CodeLanguage = "typescript" | "python" | "csharp" | "meta";
type CodeIssueKind = "comment" | "temporary_log" | "missing_license_header" | "extra_dev_docs" | "region_name_missing" | "region_mismatch" | "region_unclosed" | "unreadable_file";
type CodeIssue = { path: string; language: CodeLanguage; kind: CodeIssueKind; line: number; column: number; message: string; excerpt?: string };
type CodeReport = { timestamp: string; status: "success" | "error"; summary: { filesScanned: number; issues: number; byKind: Record<CodeIssueKind, number>; byLanguage: Record<CodeLanguage, number> }; issues: CodeIssue[] };

const rootDir = join(__dirname, "..");
const reportPath = join(rootDir, "reports", "code.json");
const fix = process.argv.includes("--fix");

function normalizePath(path: string): string {
  return path.replace(/\\/g, "/");
}

function getTrackedFiles(): string[] {
  return execSync("git ls-files", { cwd: rootDir, encoding: "utf-8" }).split(/\r?\n/g).map((line) => line.trim()).filter(Boolean);
}

function getUntrackedFiles(): string[] {
  return execSync("git status --porcelain -uall", { cwd: rootDir, encoding: "utf-8" }).split(/\r?\n/g).map((line) => line.trim()).filter(Boolean).filter((line) => line.startsWith("?? ")).map((line) => line.slice("?? ".length).trim()).filter(Boolean);
}

function getRepoFiles(): string[] {
  return Array.from(new Set([...getTrackedFiles(), ...getUntrackedFiles()].map(normalizePath))).sort();
}

function getPackageRootDirs(repoFiles: string[]): Set<string> {
  const roots = new Set<string>();
  for (const file of repoFiles) {
    if (file.endsWith("/package.json") || file.endsWith("/pyproject.toml") || file.endsWith(".csproj") || file.endsWith(".fsproj") || file.endsWith("/yak.toml")) roots.add(dirname(file).replace(/\\/g, "/"));
  }
  return roots;
}

function isConfigFile(path: string): boolean {
  const base = basename(path);
  if (base.endsWith(".config.ts") || base.endsWith(".config.tsx")) return true;
  if (base === "vite.config.ts" || base === "vite.config.tsx") return true;
  if (base === "vite.test.config.ts" || base === "vite.test.config.tsx") return true;
  if (base === "vitest.config.ts" || base === "vitest.config.tsx") return true;
  if (base === "eslint.config.ts" || base === "eslint.config.tsx") return true;
  if (base === "playwright.config.ts" || base === "playwright.config.tsx") return true;
  if (base === "tailwind.config.ts" || base === "tailwind.config.tsx") return true;
  if (base === "postcss.config.ts" || base === "postcss.config.tsx") return true;
  return false;
}

function getLanguage(path: string): CodeLanguage | null {
  if (path.endsWith(".ts") || path.endsWith(".tsx")) return "typescript";
  if (path.endsWith(".py")) return "python";
  if (path.endsWith(".cs")) return "csharp";
  return null;
}

function isExtraDevDoc(path: string, packageRoots: Set<string>): boolean {
  const normalized = normalizePath(path);
  if (normalized === "README.md" || normalized === "AGENTS.md") return false;
  if (normalized.endsWith("/AGENTS.md")) return true;
  if (normalized.endsWith("/README.md") && packageRoots.has(dirname(normalized))) return false;
  return normalized.endsWith("/README.md");
}

function shouldScan(path: string): boolean {
  const normalized = normalizePath(path);
  if (normalized.startsWith("log/")) return false;
  if (normalized.startsWith("reports/")) return false;
  if (normalized.startsWith("temp/")) return false;
  if (normalized.startsWith("test-results/")) return false;
  if (normalized.startsWith("node_modules/")) return false;
  if (normalized.startsWith(".nx/")) return false;
  if (normalized.startsWith(".github/")) return false;
  return true;
}

function splitLines(text: string): string[] {
  return text.replace(/^\uFEFF/, "").split(/\r?\n/g);
}

function getExcerpt(line: string, max = 160): string {
  const trimmed = line.trim();
  return trimmed.length > max ? `${trimmed.slice(0, max)}...` : trimmed;
}

function getLicenseHeaderEndLine(lines: string[], language: CodeLanguage): { hasSpdx: boolean; headerEndLine: number } {
  let start = 0;
  if (language !== "python" && lines[0]?.startsWith("#!")) start = 1;
  if (language === "python" && (lines[0]?.startsWith("#!") || lines[0]?.startsWith("# -*-"))) start = 1;
  if (language === "python" && start === 1 && /^\s*#\s*coding[:=]/i.test(lines[1] ?? "")) start = 2;
  const headerLines: number[] = [];
  let hasSpdx = false;
  const startsWithLineComment = (line: string): boolean => {
    const trimmed = line.trim();
    if (!trimmed) return true;
    if (language === "python") return trimmed.startsWith("#");
    return trimmed.startsWith("//");
  };
  for (let i = start; i < Math.min(lines.length, start + 50); i++) {
    const line = lines[i] ?? "";
    if (!startsWithLineComment(line)) break;
    headerLines.push(i + 1);
    if (line.includes("SPDX-License-Identifier:")) hasSpdx = true;
  }
  if (hasSpdx && headerLines.length > 0) return { hasSpdx: true, headerEndLine: headerLines[headerLines.length - 1] ?? 0 };
  if (language !== "python") {
    const first = lines[start] ?? "";
    const trimmed = first.trim();
    if (trimmed.startsWith("/*")) {
      let endLine = 0;
      let headerText = "";
      for (let i = start; i < Math.min(lines.length, start + 200); i++) {
        headerText += `${lines[i] ?? ""}\n`;
        if ((lines[i] ?? "").includes("*/")) {
          endLine = i + 1;
          break;
        }
      }
      if (headerText.includes("SPDX-License-Identifier:")) return { hasSpdx: true, headerEndLine: endLine || 1 };
    }
  }
  return { hasSpdx: false, headerEndLine: 0 };
}

function parseRegions(lines: string[], language: CodeLanguage, path: string): CodeIssue[] {
  const issues: CodeIssue[] = [];
  const stack: { name: string; line: number }[] = [];
  const regionRegex = language === "csharp" ? /^\s*#\s*region\b(.*)$/ : language === "python" ? /^\s*#\s*region\b(.*)$/ : /^\s*\/\/\s*#region\b(.*)$/;
  const endRegionRegex = language === "csharp" ? /^\s*#\s*endregion\b(.*)$/ : language === "python" ? /^\s*#\s*endregion\b(.*)$/ : /^\s*\/\/\s*#endregion\b(.*)$/;
  for (let i = 0; i < lines.length; i++) {
    const lineNumber = i + 1;
    const line = lines[i] ?? "";
    const regionMatch = line.match(regionRegex);
    if (regionMatch) {
      const name = (regionMatch[1] ?? "").trim();
      if (!name) issues.push({ path, language, kind: "region_name_missing", line: lineNumber, column: 1, message: "Region is missing a name", excerpt: getExcerpt(line) });
      stack.push({ name, line: lineNumber });
      continue;
    }
    const endMatch = line.match(endRegionRegex);
    if (endMatch) {
      const name = (endMatch[1] ?? "").trim();
      if (!name) issues.push({ path, language, kind: "region_name_missing", line: lineNumber, column: 1, message: "Endregion is missing a name", excerpt: getExcerpt(line) });
      const current = stack.pop();
      if (!current) {
        issues.push({ path, language, kind: "region_mismatch", line: lineNumber, column: 1, message: "Endregion without matching region", excerpt: getExcerpt(line) });
        continue;
      }
      if (current.name !== name) issues.push({ path, language, kind: "region_mismatch", line: lineNumber, column: 1, message: `Region mismatch: opened \"${current.name}\" at line ${current.line}, closed \"${name}\"`, excerpt: getExcerpt(line) });
    }
  }
  for (const open of stack) {
    issues.push({ path, language, kind: "region_unclosed", line: open.line, column: 1, message: `Region \"${open.name}\" not closed`, excerpt: getExcerpt(lines[open.line - 1] ?? "") });
  }
  return issues;
}

function getHeaderRegionLines(lines: string[], language: CodeLanguage): Set<number> {
  const set = new Set<number>();
  const headerNames = new Set(["header", "headers", "fileheader", "file-header"]);
  const regionRegex = language === "csharp" ? /^\s*#\s*region\b(.*)$/ : language === "python" ? /^\s*#\s*region\b(.*)$/ : /^\s*\/\/\s*#region\b(.*)$/;
  const endRegionRegex = language === "csharp" ? /^\s*#\s*endregion\b(.*)$/ : language === "python" ? /^\s*#\s*endregion\b(.*)$/ : /^\s*\/\/\s*#endregion\b(.*)$/;
  const stack: string[] = [];
  for (let i = 0; i < lines.length; i++) {
    const lineNumber = i + 1;
    const line = lines[i] ?? "";
    const startMatch = line.match(regionRegex);
    if (startMatch) {
      const name = (startMatch[1] ?? "").trim().toLowerCase().replace(/\s+/g, "");
      stack.push(name);
      if (stack.some((n) => headerNames.has(n))) set.add(lineNumber);
      continue;
    }
    const endMatch = line.match(endRegionRegex);
    if (endMatch) {
      if (stack.some((n) => headerNames.has(n))) set.add(lineNumber);
      stack.pop();
      continue;
    }
    if (stack.some((n) => headerNames.has(n))) set.add(lineNumber);
  }
  return set;
}

function isTsRegionLineText(textAfterSlashes: string): boolean {
  const t = textAfterSlashes.trim();
  return t.startsWith("#region") || t.startsWith("#endregion");
}

function scanTypescriptComments(content: string, lines: string[], path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
  if (isConfigFile(path)) return issues;
  const sourceFile = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true, path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const headerLines = getHeaderRegionLines(lines, "typescript");
  const scanner = ts.createScanner(ts.ScriptTarget.Latest, false, path.endsWith(".tsx") ? ts.LanguageVariant.JSX : ts.LanguageVariant.Standard, content);
  while (scanner.scan() !== ts.SyntaxKind.EndOfFileToken) {
    const kind = scanner.getToken();
    if (kind !== ts.SyntaxKind.SingleLineCommentTrivia && kind !== ts.SyntaxKind.MultiLineCommentTrivia) continue;
    const pos = scanner.getTokenPos();
    const end = scanner.getTextPos();
    const { line, character } = sourceFile.getLineAndCharacterOfPosition(pos);
    const lineNumber = line + 1;
    const column = character + 1;
    if (lineNumber <= licenseHeaderEndLine) continue;
    const tokenText = content.slice(pos, end);
    if (headerLines.has(lineNumber)) continue;
    if (/todo/i.test(tokenText)) continue;
    if (tokenText.includes("SPDX-License-Identifier:")) continue;
    if (kind === ts.SyntaxKind.SingleLineCommentTrivia && tokenText.startsWith("///") && /<\s*reference\b/i.test(tokenText)) continue;
    const lineText = lines[lineNumber - 1] ?? "";
    if (kind === ts.SyntaxKind.SingleLineCommentTrivia) {
      const after = lineText.slice(Math.max(0, lineText.indexOf("//") + 2));
      if (isTsRegionLineText(after)) continue;
    }
    issues.push({ path, language: "typescript", kind: "comment", line: lineNumber, column, message: kind === ts.SyntaxKind.SingleLineCommentTrivia ? "Line comment found" : "Block comment found", excerpt: getExcerpt(lineText) });
  }
  return issues;
}

function getTypescriptStringLiteralText(node: ts.Node): string | null {
  if (ts.isStringLiteralLike(node)) return node.text;
  if (ts.isNoSubstitutionTemplateLiteral(node)) return node.text;
  if (ts.isTemplateExpression(node)) return [node.head.text, ...node.templateSpans.map((span) => span.literal.text)].join("");
  if (ts.isBinaryExpression(node) && node.operatorToken.kind === ts.SyntaxKind.PlusToken) {
    const left = getTypescriptStringLiteralText(node.left);
    const right = getTypescriptStringLiteralText(node.right);
    if (left !== null && right !== null) return `${left}${right}`;
  }
  return null;
}

function scanTypescriptTemporaryLogs(content: string, lines: string[], path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
  const sourceFile = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true, path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const methods = new Set(["log", "debug", "info", "warn", "error"]);
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node)) {
      const expression = node.expression;
      if (ts.isPropertyAccessExpression(expression) && ts.isIdentifier(expression.expression) && expression.expression.text === "console" && methods.has(expression.name.text)) {
        const debugArg = node.arguments.map((arg) => getTypescriptStringLiteralText(arg)).filter((text): text is string => text !== null).some((text) => text.includes("[DEBUG]"));
        if (debugArg) {
          const { line, character } = sourceFile.getLineAndCharacterOfPosition(node.getStart(sourceFile));
          const lineNumber = line + 1;
          if (lineNumber > licenseHeaderEndLine) issues.push({ path, language: "typescript", kind: "temporary_log", line: lineNumber, column: character + 1, message: "Temporary [DEBUG] console log found", excerpt: getExcerpt(lines[lineNumber - 1] ?? "") });
        }
      }
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return issues;
}

function stripTypescriptComments(content: string, lines: string[], path: string, licenseHeaderEndLine: number): string {
  if (isConfigFile(path)) return content;
  const sourceFile = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true, path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const headerLines = getHeaderRegionLines(lines, "typescript");
  const scanner = ts.createScanner(ts.ScriptTarget.Latest, false, path.endsWith(".tsx") ? ts.LanguageVariant.JSX : ts.LanguageVariant.Standard, content);
  const removals: { start: number; end: number; insertSpace: boolean }[] = [];
  while (scanner.scan() !== ts.SyntaxKind.EndOfFileToken) {
    const kind = scanner.getToken();
    if (kind !== ts.SyntaxKind.SingleLineCommentTrivia && kind !== ts.SyntaxKind.MultiLineCommentTrivia) continue;
    const start = scanner.getTokenPos();
    const end = scanner.getTextPos();
    const { line } = sourceFile.getLineAndCharacterOfPosition(start);
    const lineNumber = line + 1;
    if (lineNumber <= licenseHeaderEndLine) continue;
    const tokenText = content.slice(start, end);
    if (headerLines.has(lineNumber)) continue;
    if (/todo/i.test(tokenText)) continue;
    if (tokenText.includes("SPDX-License-Identifier:")) continue;
    if (kind === ts.SyntaxKind.SingleLineCommentTrivia) {
      const lineText = lines[lineNumber - 1] ?? "";
      const after = lineText.slice(Math.max(0, lineText.indexOf("//") + 2));
      if (isTsRegionLineText(after)) continue;
      if (tokenText.startsWith("///") && /<\s*reference\b/i.test(tokenText)) continue;
    }
    const prev = start > 0 ? content[start - 1] ?? "" : "";
    const next = end < content.length ? content[end] ?? "" : "";
    const insertSpace = /[A-Za-z0-9_$]/.test(prev) && /[A-Za-z0-9_$]/.test(next);
    removals.push({ start, end, insertSpace });
  }
  if (removals.length === 0) return content;
  let out = "";
  let index = 0;
  for (const removal of removals.sort((a, b) => a.start - b.start)) {
    if (removal.start < index) continue;
    out += content.slice(index, removal.start);
    if (removal.insertSpace) out += " ";
    index = removal.end;
  }
  out += content.slice(index);
  return out;
}

function scanTsOrCsharpComments(content: string, lines: string[], language: CodeLanguage, path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
  let inSingle = false;
  let inDouble = false;
  let inTemplate = false;
  let inBlockComment = false;
  let line = 1;
  let col = 1;
  for (let i = 0; i < content.length; i++) {
    const ch = content[i] ?? "";
    const next = content[i + 1] ?? "";
    if (ch === "\n") {
      line += 1;
      col = 1;
      continue;
    }
    if (inBlockComment) {
      if (ch === "*" && next === "/") {
        inBlockComment = false;
        i += 1;
        col += 2;
        continue;
      }
      col += 1;
      continue;
    }
    if (!inDouble && !inTemplate && ch === "'" && !inSingle) {
      inSingle = true;
      col += 1;
      continue;
    }
    if (inSingle) {
      if (ch === "\\" && next) {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === "'") inSingle = false;
      col += 1;
      continue;
    }
    if (!inSingle && !inTemplate && ch === "\"" && !inDouble) {
      inDouble = true;
      col += 1;
      continue;
    }
    if (inDouble) {
      if (ch === "\\" && next) {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === "\"") inDouble = false;
      col += 1;
      continue;
    }
    if (language === "typescript" && !inSingle && !inDouble && ch === "`" && !inTemplate) {
      inTemplate = true;
      col += 1;
      continue;
    }
    if (inTemplate) {
      if (ch === "\\" && next) {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === "`") inTemplate = false;
      col += 1;
      continue;
    }
    if (ch === "/" && next === "/") {
      const lineText = lines[line - 1] ?? "";
      const after = lineText.slice(Math.max(0, lineText.indexOf("//") + 2));
      if (line <= licenseHeaderEndLine) {
        col += 2;
        continue;
      }
      if (language === "typescript" && isTsRegionLineText(after)) {
        col += 2;
        continue;
      }
      if (after.includes("SPDX-License-Identifier:")) {
        col += 2;
        continue;
      }
      issues.push({ path, language, kind: "comment", line, column: col, message: "Line comment found", excerpt: getExcerpt(lineText) });
      while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
      line += 1;
      col = 1;
      continue;
    }
    if (ch === "/" && next === "*") {
      if (line <= licenseHeaderEndLine) {
        inBlockComment = true;
        i += 1;
        col += 2;
        continue;
      }
      const lineText = lines[line - 1] ?? "";
      issues.push({ path, language, kind: "comment", line, column: col, message: "Block comment found", excerpt: getExcerpt(lineText) });
      inBlockComment = true;
      i += 1;
      col += 2;
      continue;
    }
    col += 1;
  }
  return issues;
}

function isPythonRegionLine(lineText: string): boolean {
  return /^\s*#\s*(?:end)?region\b/i.test(lineText);
}

function scanPythonComments(content: string, lines: string[], path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
  if (isConfigFile(path)) return issues;
  const headerLines = getHeaderRegionLines(lines, "python");
  let inSingle = false;
  let inDouble = false;
  let inTripleSingle = false;
  let inTripleDouble = false;
  let line = 1;
  let col = 1;
  for (let i = 0; i < content.length; i++) {
    const ch = content[i] ?? "";
    const next = content[i + 1] ?? "";
    const next2 = content[i + 2] ?? "";
    if (ch === "\n") {
      line += 1;
      col = 1;
      continue;
    }
    if (inTripleSingle) {
      if (ch === "'" && next === "'" && next2 === "'") {
        inTripleSingle = false;
        i += 2;
        col += 3;
        continue;
      }
      col += 1;
      continue;
    }
    if (inTripleDouble) {
      if (ch === "\"" && next === "\"" && next2 === "\"") {
        inTripleDouble = false;
        i += 2;
        col += 3;
        continue;
      }
      col += 1;
      continue;
    }
    if (inSingle) {
      if (ch === "\\" && next) {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === "'") inSingle = false;
      col += 1;
      continue;
    }
    if (inDouble) {
      if (ch === "\\" && next) {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === "\"") inDouble = false;
      col += 1;
      continue;
    }
    if (ch === "'" && next === "'" && next2 === "'") {
      inTripleSingle = true;
      i += 2;
      col += 3;
      continue;
    }
    if (ch === "\"" && next === "\"" && next2 === "\"") {
      inTripleDouble = true;
      i += 2;
      col += 3;
      continue;
    }
    if (ch === "'") {
      inSingle = true;
      col += 1;
      continue;
    }
    if (ch === "\"") {
      inDouble = true;
      col += 1;
      continue;
    }
    if (ch === "#") {
      const lineText = lines[line - 1] ?? "";
      const after = lineText.slice(lineText.indexOf("#") + 1).trim();
      if (line <= licenseHeaderEndLine) {
        col += 1;
        continue;
      }
      if (headerLines.has(line)) {
        col += 1;
        continue;
      }
      if (isPythonRegionLine(lineText)) {
        col += 1;
        continue;
      }
      if (after.includes("SPDX-License-Identifier:")) {
        col += 1;
        continue;
      }
      if (/todo/i.test(after)) {
        col += 1;
        continue;
      }
      issues.push({ path, language: "python", kind: "comment", line, column: col, message: "Line comment found", excerpt: getExcerpt(lineText) });
      while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
      line += 1;
      col = 1;
      continue;
    }
    col += 1;
  }
  return issues;
}

function scanPythonTemporaryLogs(lines: string[], path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
  for (let i = 0; i < lines.length; i++) {
    const lineNumber = i + 1;
    if (lineNumber <= licenseHeaderEndLine) continue;
    const lineText = lines[i] ?? "";
    if (!lineText.includes("[DEBUG]")) continue;
    if (!/\b(print|logging\.(debug|info|warning|error)|logger\.(debug|info|warning|error))\s*\(/.test(lineText)) continue;
    issues.push({ path, language: "python", kind: "temporary_log", line: lineNumber, column: 1, message: "Temporary [DEBUG] log found", excerpt: getExcerpt(lineText) });
  }
  return issues;
}

function scanCsharpComments(content: string, lines: string[], path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
  if (isConfigFile(path)) return issues;
  const headerLines = getHeaderRegionLines(lines, "csharp");
  let inChar = false;
  let inString = false;
  let inVerbatimString = false;
  let inBlockComment = false;
  let line = 1;
  let col = 1;
  for (let i = 0; i < content.length; i++) {
    const ch = content[i] ?? "";
    const next = content[i + 1] ?? "";
    const next2 = content[i + 2] ?? "";
    if (ch === "\n") {
      line += 1;
      col = 1;
      continue;
    }
    if (inBlockComment) {
      if (ch === "*" && next === "/") {
        inBlockComment = false;
        i += 1;
        col += 2;
        continue;
      }
      col += 1;
      continue;
    }
    if (inChar) {
      if (ch === "\\" && next) {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === "'") inChar = false;
      col += 1;
      continue;
    }
    if (inVerbatimString) {
      if (ch === "\"" && next === "\"") {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === "\"") inVerbatimString = false;
      col += 1;
      continue;
    }
    if (inString) {
      if (ch === "\\" && next) {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === "\"") inString = false;
      col += 1;
      continue;
    }
    if (ch === "'" && !inChar) {
      inChar = true;
      col += 1;
      continue;
    }
    if (ch === "$" && next === "@" && next2 === "\"") {
      inVerbatimString = true;
      i += 2;
      col += 3;
      continue;
    }
    if (ch === "@" && next === "$" && next2 === "\"") {
      inVerbatimString = true;
      i += 2;
      col += 3;
      continue;
    }
    if (ch === "@" && next === "\"") {
      inVerbatimString = true;
      i += 1;
      col += 2;
      continue;
    }
    if (ch === "$" && next === "\"") {
      inString = true;
      i += 1;
      col += 2;
      continue;
    }
    if (ch === "\"") {
      inString = true;
      col += 1;
      continue;
    }
    if (ch === "/" && next === "/") {
      const lineText = lines[line - 1] ?? "";
      if (line <= licenseHeaderEndLine) {
        col += 2;
        continue;
      }
      if (headerLines.has(line)) {
        col += 2;
        continue;
      }
      if (/todo/i.test(lineText)) {
        col += 2;
        continue;
      }
      issues.push({ path, language: "csharp", kind: "comment", line, column: col, message: "Line comment found", excerpt: getExcerpt(lineText) });
      while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
      line += 1;
      col = 1;
      continue;
    }
    if (ch === "/" && next === "*") {
      const lineText = lines[line - 1] ?? "";
      if (line <= licenseHeaderEndLine) {
        inBlockComment = true;
        i += 1;
        col += 2;
        continue;
      }
      const endIndex = content.indexOf("*/", i + 2);
      const blockText = endIndex === -1 ? content.slice(i) : content.slice(i, endIndex + 2);
      if (headerLines.has(line) || /todo/i.test(blockText)) {
        inBlockComment = true;
        i += 1;
        col += 2;
        continue;
      }
      issues.push({ path, language: "csharp", kind: "comment", line, column: col, message: "Block comment found", excerpt: getExcerpt(lineText) });
      inBlockComment = true;
      i += 1;
      col += 2;
      continue;
    }
    col += 1;
  }
  return issues;
}

function scanCsharpTemporaryLogs(lines: string[], path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
  const regex = /\b(?:System\.)?Console\.(?:WriteLine|Write|Error\.WriteLine|Error\.Write)\s*\(/;
  for (let i = 0; i < lines.length; i++) {
    const lineNumber = i + 1;
    if (lineNumber <= licenseHeaderEndLine) continue;
    const lineText = lines[i] ?? "";
    if (!regex.test(lineText)) continue;
    if (!lineText.includes("[DEBUG]")) continue;
    issues.push({ path, language: "csharp", kind: "temporary_log", line: lineNumber, column: 1, message: "Temporary [DEBUG] console log found", excerpt: getExcerpt(lineText) });
  }
  return issues;
}

function stripCsharpComments(content: string, lines: string[], licenseHeaderEndLine: number): string {
  const headerLines = getHeaderRegionLines(lines, "csharp");
  let out = "";
  let inChar = false;
  let inString = false;
  let inVerbatimString = false;
  let inBlockCommentKeep = false;
  let inBlockCommentDrop = false;
  let line = 1;
  for (let i = 0; i < content.length; i++) {
    const ch = content[i] ?? "";
    const next = content[i + 1] ?? "";
    const next2 = content[i + 2] ?? "";
    if (ch === "\n") {
      out += "\n";
      line += 1;
      continue;
    }
    if (inBlockCommentKeep) {
      out += ch;
      if (ch === "*" && next === "/") {
        out += "/";
        i += 1;
        inBlockCommentKeep = false;
      }
      continue;
    }
    if (inBlockCommentDrop) {
      if (ch === "*" && next === "/") {
        i += 1;
        inBlockCommentDrop = false;
      }
      continue;
    }
    if (inChar) {
      out += ch;
      if (ch === "\\" && next) {
        out += next;
        i += 1;
        continue;
      }
      if (ch === "'") inChar = false;
      continue;
    }
    if (inVerbatimString) {
      out += ch;
      if (ch === "\"" && next === "\"") {
        out += "\"";
        i += 1;
        continue;
      }
      if (ch === "\"") inVerbatimString = false;
      continue;
    }
    if (inString) {
      out += ch;
      if (ch === "\\" && next) {
        out += next;
        i += 1;
        continue;
      }
      if (ch === "\"") inString = false;
      continue;
    }
    if (ch === "'") {
      inChar = true;
      out += ch;
      continue;
    }
    if (ch === "$" && next === "@" && next2 === "\"") {
      out += "$@\"";
      i += 2;
      inVerbatimString = true;
      continue;
    }
    if (ch === "@" && next === "$" && next2 === "\"") {
      out += "@$\"";
      i += 2;
      inVerbatimString = true;
      continue;
    }
    if (ch === "@" && next === "\"") {
      out += "@\"";
      i += 1;
      inVerbatimString = true;
      continue;
    }
    if (ch === "$" && next === "\"") {
      out += "$\"";
      i += 1;
      inString = true;
      continue;
    }
    if (ch === "\"") {
      out += "\"";
      inString = true;
      continue;
    }
    if (ch === "/" && next === "/") {
      const startIndex = i;
      if (line <= licenseHeaderEndLine) {
        while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
        out += content.slice(startIndex, i);
        i -= 1;
        continue;
      }
      if (headerLines.has(line) || /todo/i.test(lines[line - 1] ?? "")) {
        while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
        out += content.slice(startIndex, i);
        i -= 1;
        continue;
      }
      while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
      i -= 1;
      continue;
    }
    if (ch === "/" && next === "*") {
      const endIndex = content.indexOf("*/", i + 2);
      const blockText = endIndex === -1 ? content.slice(i) : content.slice(i, endIndex + 2);
      if (line <= licenseHeaderEndLine || headerLines.has(line) || /todo/i.test(blockText)) {
        out += "/*";
        i += 1;
        inBlockCommentKeep = true;
        continue;
      }
      i += 1;
      inBlockCommentDrop = true;
      continue;
    }
    out += ch;
  }
  return out;
}

function stripTsOrCsharpComments(content: string, lines: string[], language: CodeLanguage, licenseHeaderEndLine: number): string {
  let out = "";
  let inSingle = false;
  let inDouble = false;
  let inTemplate = false;
  let inBlockCommentKeep = false;
  let inBlockCommentDrop = false;
  let line = 1;
  for (let i = 0; i < content.length; i++) {
    const ch = content[i] ?? "";
    const next = content[i + 1] ?? "";
    if (ch === "\n") {
      out += "\n";
      line += 1;
      continue;
    }
    if (inBlockCommentKeep) {
      out += ch;
      if (ch === "*" && next === "/") {
        out += "/";
        i += 1;
        inBlockCommentKeep = false;
      }
      continue;
    }
    if (inBlockCommentDrop) {
      if (ch === "*" && next === "/") {
        i += 1;
        inBlockCommentDrop = false;
      }
      continue;
    }
    if (!inDouble && !inTemplate && ch === "'" && !inSingle) {
      inSingle = true;
      out += ch;
      continue;
    }
    if (inSingle) {
      out += ch;
      if (ch === "\\" && next) {
        out += next;
        i += 1;
        continue;
      }
      if (ch === "'") inSingle = false;
      continue;
    }
    if (!inSingle && !inTemplate && ch === "\"" && !inDouble) {
      inDouble = true;
      out += ch;
      continue;
    }
    if (inDouble) {
      out += ch;
      if (ch === "\\" && next) {
        out += next;
        i += 1;
        continue;
      }
      if (ch === "\"") inDouble = false;
      continue;
    }
    if (language === "typescript" && !inSingle && !inDouble && ch === "`" && !inTemplate) {
      inTemplate = true;
      out += ch;
      continue;
    }
    if (inTemplate) {
      out += ch;
      if (ch === "\\" && next) {
        out += next;
        i += 1;
        continue;
      }
      if (ch === "`") inTemplate = false;
      continue;
    }
    if (ch === "/" && next === "/") {
      const lineText = lines[line - 1] ?? "";
      const after = lineText.slice(Math.max(0, lineText.indexOf("//") + 2));
      const keep = line <= licenseHeaderEndLine || after.includes("SPDX-License-Identifier:") || (language === "typescript" && isTsRegionLineText(after));
      if (keep) {
        const startIndex = i;
        while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
        out += content.slice(startIndex, i);
        i -= 1;
        continue;
      }
      while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
      i -= 1;
      continue;
    }
    if (ch === "/" && next === "*") {
      if (line <= licenseHeaderEndLine) {
        out += "/*";
        i += 1;
        inBlockCommentKeep = true;
        continue;
      }
      i += 1;
      inBlockCommentDrop = true;
      continue;
    }
    out += ch;
  }
  return out;
}

function stripPythonComments(content: string, lines: string[], licenseHeaderEndLine: number): string {
  const headerLines = getHeaderRegionLines(lines, "python");
  let out = "";
  let inSingle = false;
  let inDouble = false;
  let inTripleSingle = false;
  let inTripleDouble = false;
  let line = 1;
  for (let i = 0; i < content.length; i++) {
    const ch = content[i] ?? "";
    const next = content[i + 1] ?? "";
    const next2 = content[i + 2] ?? "";
    if (ch === "\n") {
      out += "\n";
      line += 1;
      continue;
    }
    if (inTripleSingle) {
      out += ch;
      if (ch === "'" && next === "'" && next2 === "'") {
        out += "''";
        i += 2;
        inTripleSingle = false;
      }
      continue;
    }
    if (inTripleDouble) {
      out += ch;
      if (ch === "\"" && next === "\"" && next2 === "\"") {
        out += "\"\"";
        i += 2;
        inTripleDouble = false;
      }
      continue;
    }
    if (inSingle) {
      out += ch;
      if (ch === "\\" && next) {
        out += next;
        i += 1;
        continue;
      }
      if (ch === "'") inSingle = false;
      continue;
    }
    if (inDouble) {
      out += ch;
      if (ch === "\\" && next) {
        out += next;
        i += 1;
        continue;
      }
      if (ch === "\"") inDouble = false;
      continue;
    }
    if (ch === "'" && next === "'" && next2 === "'") {
      out += "'''";
      i += 2;
      inTripleSingle = true;
      continue;
    }
    if (ch === "\"" && next === "\"" && next2 === "\"") {
      out += "\"\"\"";
      i += 2;
      inTripleDouble = true;
      continue;
    }
    if (ch === "'") {
      inSingle = true;
      out += ch;
      continue;
    }
    if (ch === "\"") {
      inDouble = true;
      out += ch;
      continue;
    }
    if (ch === "#") {
      const lineText = lines[line - 1] ?? "";
      const after = lineText.slice(lineText.indexOf("#") + 1).trim();
      const keep = line <= licenseHeaderEndLine || headerLines.has(line) || after.includes("SPDX-License-Identifier:") || isPythonRegionLine(lineText) || /todo/i.test(after) || lineText.trimStart().startsWith("#!");
      if (keep) {
        const startIndex = i;
        while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
        out += content.slice(startIndex, i);
        i -= 1;
        continue;
      }
      while (i < content.length && (content[i] ?? "") !== "\n") i += 1;
      i -= 1;
      continue;
    }
    out += ch;
  }
  return out;
}

function writeReport(report: CodeReport): void {
  writeFileSync(reportPath, JSON.stringify(report, null, 2));
}

function run(): void {
  const issues: CodeIssue[] = [];
  let filesScanned = 0;
  const byKind: Record<CodeIssueKind, number> = { comment: 0, temporary_log: 0, missing_license_header: 0, extra_dev_docs: 0, region_name_missing: 0, region_mismatch: 0, region_unclosed: 0, unreadable_file: 0 };
  const byLanguage: Record<CodeLanguage, number> = { typescript: 0, python: 0, csharp: 0, meta: 0 };
  const repoFiles = getRepoFiles().filter(shouldScan);
  const packageRoots = getPackageRootDirs(repoFiles);
  for (const docPath of repoFiles.filter((file) => isExtraDevDoc(file, packageRoots))) {
    const absolute = join(rootDir, docPath);
    if (fix) {
      try {
        if (existsSync(absolute)) unlinkSync(absolute);
      } catch {
        issues.push({ path: docPath, language: "meta", kind: "extra_dev_docs", line: 1, column: 1, message: "Extra dev doc detected but could not be deleted" });
        byKind.extra_dev_docs += 1;
        byLanguage.meta += 1;
      }
      continue;
    }
    issues.push({ path: docPath, language: "meta", kind: "extra_dev_docs", line: 1, column: 1, message: "Extra README.md/AGENTS.md outside repo root" });
    byKind.extra_dev_docs += 1;
    byLanguage.meta += 1;
  }
  for (const path of repoFiles) {
    const language = getLanguage(path);
    if (!language) continue;
    filesScanned += 1;
    byLanguage[language] += 1;
    const absolute = join(rootDir, path);
    let content = "";
    try {
      content = readFileSync(absolute, "utf-8");
    } catch {
      const issue: CodeIssue = { path, language, kind: "unreadable_file", line: 1, column: 1, message: "Failed to read file" };
      issues.push(issue);
      byKind[issue.kind] += 1;
      continue;
    }
    let lines = splitLines(content);
    let license = getLicenseHeaderEndLine(lines, language);
    if (fix && !license.hasSpdx) {
      const year = new Date().getFullYear();
      const prefix = language === "python" ? "# " : "// ";
      const insertionStart = (() => {
        let index = 0;
        if ((lines[0] ?? "").startsWith("#!")) index = 1;
        if (language === "python" && index === 1 && /^\s*#\s*coding[:=]/i.test(lines[1] ?? "")) index = 2;
        return index;
      })();
      const insertAt = language === "typescript" ? (() => { let index = insertionStart; while ((lines[index] ?? "").trimStart().startsWith("///") && /<\s*reference\b/i.test(lines[index] ?? "")) index += 1; return index; })() : insertionStart;
      content = [...lines.slice(0, insertAt), `${prefix}SPDX-License-Identifier: AGPL-3.0-only`, `${prefix}Copyright (C) ${year} Ueli Saluz`, ...lines.slice(insertAt)].join("\n");
      writeFileSync(absolute, content, "utf-8");
      lines = splitLines(content);
      license = getLicenseHeaderEndLine(lines, language);
    }
    if (fix) {
      const updated = language === "python" ? isConfigFile(path) ? content : stripPythonComments(content, lines, license.headerEndLine) : language === "typescript" ? stripTypescriptComments(content, lines, path, license.headerEndLine) : isConfigFile(path) ? content : stripCsharpComments(content, lines, license.headerEndLine);
      if (updated !== content) {
        writeFileSync(absolute, updated, "utf-8");
        content = updated;
      }
      lines = splitLines(content);
      license = getLicenseHeaderEndLine(lines, language);
    }
    if (!license.hasSpdx) {
      const issue: CodeIssue = { path, language, kind: "missing_license_header", line: 1, column: 1, message: "Missing SPDX license header (SPDX-License-Identifier: ...)" };
      issues.push(issue);
      byKind[issue.kind] += 1;
    }
    for (const issue of parseRegions(lines, language, path)) {
      issues.push(issue);
      byKind[issue.kind] += 1;
    }
    const tempLogIssues = language === "python" ? scanPythonTemporaryLogs(lines, path, license.headerEndLine) : language === "typescript" ? scanTypescriptTemporaryLogs(content, lines, path, license.headerEndLine) : scanCsharpTemporaryLogs(lines, path, license.headerEndLine);
    for (const issue of tempLogIssues) {
      issues.push(issue);
      byKind[issue.kind] += 1;
    }
    const commentIssues = language === "python" ? scanPythonComments(content, lines, path, license.headerEndLine) : language === "typescript" ? scanTypescriptComments(content, lines, path, license.headerEndLine) : scanCsharpComments(content, lines, path, license.headerEndLine);
    for (const issue of commentIssues) {
      issues.push(issue);
      byKind[issue.kind] += 1;
    }
  }
  const report: CodeReport = { timestamp: new Date().toISOString(), status: issues.length === 0 ? "success" : "error", summary: { filesScanned, issues: issues.length, byKind, byLanguage }, issues };
  writeReport(report);
  if (fix) process.exit(issues.some((issue) => issue.kind === "unreadable_file") ? 1 : 0);
  if (report.status === "success") process.exit(0);
  process.exit(1);
}

console.log(fix ? "?? Fixing codebase comments..." : "?? Analyzing codebase...");
console.log(`?? Report: ${reportPath}`);
run();
