#!/usr/bin/env tsx
import { execSync } from "child_process";
import { readFileSync, writeFileSync } from "fs";
import { join } from "path";

const rootDir = join(__dirname, "..");
const reportPath = join(rootDir, "reports", "typescript.json");
const codeReportPath = join(rootDir, "reports", "code.json");

console.log("🔍 Running TypeScript compiler check...");

//#region CodeReport
type CodeLanguage = "typescript" | "python" | "csharp";
type CodeIssueKind = "comment" | "missing_license_header" | "region_name_missing" | "region_mismatch" | "region_unclosed" | "unreadable_file";
type CodeIssue = { path: string; language: CodeLanguage; kind: CodeIssueKind; line: number; column: number; message: string; excerpt?: string };
type CodeReport = { timestamp: string; status: "success" | "error"; summary: { filesScanned: number; issues: number; byKind: Record<CodeIssueKind, number>; byLanguage: Record<CodeLanguage, number> }; issues: CodeIssue[] };
function normalizePath(path: string): string {
  return path.replace(/\\/g, "/");
}
function getTrackedFiles(): string[] {
  return execSync("git ls-files", { cwd: rootDir, encoding: "utf-8" }).split(/\r?\n/g).map((line) => line.trim()).filter(Boolean);
}
function getLanguage(path: string): CodeLanguage | null {
  if (path.endsWith(".ts") || path.endsWith(".tsx")) return "typescript";
  if (path.endsWith(".py")) return "python";
  if (path.endsWith(".cs")) return "csharp";
  return null;
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
  const regionRegex = language === "csharp" ? /^\s*#region\b(.*)$/ : language === "python" ? /^\s*#region\b(.*)$/ : /^\s*\/\/\s*#region\b(.*)$/;
  const endRegionRegex = language === "csharp" ? /^\s*#endregion\b(.*)$/ : language === "python" ? /^\s*#endregion\b(.*)$/ : /^\s*\/\/\s*#endregion\b(.*)$/;
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
function isTsRegionLineText(textAfterSlashes: string): boolean {
  const t = textAfterSlashes.trim();
  return t.startsWith("#region") || t.startsWith("#endregion");
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
function scanPythonComments(content: string, lines: string[], path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
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
      const lineText = lines[line - 1] ?? "";
      if (line > licenseHeaderEndLine && /^\s*'''/.test(lineText)) issues.push({ path, language: "python", kind: "comment", line, column: col, message: "Triple-quoted string statement found", excerpt: getExcerpt(lineText) });
      inTripleSingle = true;
      i += 2;
      col += 3;
      continue;
    }
    if (ch === "\"" && next === "\"" && next2 === "\"") {
      const lineText = lines[line - 1] ?? "";
      if (line > licenseHeaderEndLine && /^\s*\"\"\"/.test(lineText)) issues.push({ path, language: "python", kind: "comment", line, column: col, message: "Triple-quoted string statement found", excerpt: getExcerpt(lineText) });
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
      if (after.startsWith("region") || after.startsWith("endregion")) {
        col += 1;
        continue;
      }
      if (after.includes("SPDX-License-Identifier:")) {
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
function createCodeReport(): { report: CodeReport; ok: boolean } {
  const issues: CodeIssue[] = [];
  let filesScanned = 0;
  const byKind: Record<CodeIssueKind, number> = { comment: 0, missing_license_header: 0, region_name_missing: 0, region_mismatch: 0, region_unclosed: 0, unreadable_file: 0 };
  const byLanguage: Record<CodeLanguage, number> = { typescript: 0, python: 0, csharp: 0 };
  for (const path of getTrackedFiles().filter(shouldScan)) {
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
    const lines = splitLines(content);
    const license = getLicenseHeaderEndLine(lines, language);
    if (!license.hasSpdx) {
      const issue: CodeIssue = { path, language, kind: "missing_license_header", line: 1, column: 1, message: "Missing SPDX license header (SPDX-License-Identifier: ...)" };
      issues.push(issue);
      byKind[issue.kind] += 1;
    }
    if (language === "typescript" || language === "csharp") {
      for (const issue of [...parseRegions(lines, language, path), ...scanTsOrCsharpComments(content, lines, language, path, license.headerEndLine)]) {
        issues.push(issue);
        byKind[issue.kind] += 1;
      }
    } else {
      for (const issue of [...parseRegions(lines, language, path), ...scanPythonComments(content, lines, path, license.headerEndLine)]) {
        issues.push(issue);
        byKind[issue.kind] += 1;
      }
    }
  }
  const report: CodeReport = { timestamp: new Date().toISOString(), status: issues.length === 0 ? "success" : "error", summary: { filesScanned, issues: issues.length, byKind, byLanguage }, issues };
  writeFileSync(codeReportPath, JSON.stringify(report, null, 2));
  return { report, ok: issues.length === 0 };
}
//#endregion

console.log("?? Running codebase code-quality scan...");
const code = createCodeReport();
if (code.ok) {
  console.log("Code report passed");
  console.log(`Report: ${codeReportPath}`);
} else {
  console.error("Code report failed");
  console.error(`Report: ${codeReportPath}`);
}

let tsOk = true;
try {
  const output = execSync("npx tsc --noEmit --project tsconfig.json", {
    cwd: rootDir,
    encoding: "utf-8",
  });
  
  const report = {
    timestamp: new Date().toISOString(),
    status: "success",
    errors: [],
  };
  writeFileSync(reportPath, JSON.stringify(report, null, 2));
  
  console.log("✅ TypeScript check passed");
  console.log(`📝 Report: ${reportPath}`);
} catch (error: any) {
  tsOk = false;
  const stderr = error.stderr?.toString() || "";
  const stdout = error.stdout?.toString() || "";
  const output = stdout || stderr;
  
  const report = {
    timestamp: new Date().toISOString(),
    status: "error",
    errors: output.split("\n").filter((line: string) => line.trim()),
  };
  writeFileSync(reportPath, JSON.stringify(report, null, 2));
  
  console.error("❌ TypeScript check failed");
  console.error(`📝 Report: ${reportPath}`);
}
process.exit(tsOk && code.ok ? 0 : 1);
