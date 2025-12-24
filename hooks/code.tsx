#!/usr/bin/env tsx
// #region Header

// hooks/code.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 Ueli Saluz
import { execSync } from "child_process";
import { existsSync, readFileSync, unlinkSync, writeFileSync } from "fs";
import { Box, render, Text } from "ink";
import { basename, dirname, join } from "path";
import React from "react";
import * as ts from "typescript";
import { fileURLToPath } from "url";

type CodeLanguage = "typescript" | "python" | "csharp" | "meta";
type CodeIssueKind =
  | "comment"
  | "temporary_log"
  | "missing_license_header"
  | "invalid_header_format"
  | "header_filepath_mismatch"
  | "extra_dev_docs"
  | "region_name_missing"
  | "region_mismatch"
  | "region_unclosed"
  | "region_duplicate_sibling"
  | "region_empty"
  | "unreadable_file"
  | "forbidden_import"
  | "forbidden_terminology";
type RegionNode = { name: string; line: number; children: RegionNode[] };
type CodeIssue = { path: string; language: CodeLanguage; kind: CodeIssueKind; line: number; column: number; message: string; reason: string; solution: string; excerpt?: string };
type CodeReport = { timestamp: string; status: "success" | "error"; summary: { filesScanned: number; issues: number; byKind: Record<CodeIssueKind, number>; byLanguage: Record<CodeLanguage, number> }; issues: CodeIssue[] };

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, "..");
const reportPath = join(rootDir, "reports", "code.json");
const fix = process.argv.includes("--fix");

type LicenseType = "LGPL" | "AGPL";

const LGPL_LICENSE_LINES = [
  "This program is free software: you can redistribute it and/or modify",
  "it under the terms of the GNU Lesser General Public License as",
  "published by the Free Software Foundation, either version 3 of the",
  "License, or (at your option) any later version.",
  "",
  "This program is distributed in the hope that it will be useful,",
  "but WITHOUT ANY WARRANTY; without even the implied warranty of",
  "MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the",
  "GNU Lesser General Public License for more details.",
  "",
  "You should have received a copy of the GNU Lesser General Public License",
  "along with this program.  If not, see <https://www.gnu.org/licenses/>.",
];

const AGPL_LICENSE_LINES = [
  "This program is free software: you can redistribute it and/or modify",
  "it under the terms of the GNU Affero General Public License as",
  "published by the Free Software Foundation, either version 3 of the",
  "License, or (at your option) any later version.",
  "",
  "This program is distributed in the hope that it will be useful,",
  "but WITHOUT ANY WARRANTY; without even the implied warranty of",
  "MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the",
  "GNU Affero General Public License for more details.",
  "",
  "You should have received a copy of the GNU Affero General Public License",
  "along with this program.  If not, see <https://www.gnu.org/licenses/>.",
];

const CONTRIBUTOR = "Ueli Saluz <ueli@semio-tech.com>";

function getDefaultIssueReason(kind: CodeIssueKind): string {
  if (kind === "comment") return "Code is never documented inline; documentation lives exclusively in the root README.md and AGENTS.md across the four required perspectives (Products, Components, SRS Business Logic/UI/UX, Codebase).";
  if (kind === "temporary_log") return "Only temporary diagnostics prefixed with [DEBUG] are allowed; they must be removed to keep Sketchpad runtime output clean.";
  if (kind === "missing_license_header") return "Every source file must include an SPDX license header and header region per code hygiene requirements.";
  if (kind === "invalid_header_format") return "Header regions must follow the required format so tooling can verify file path, contributor, and license consistently.";
  if (kind === "header_filepath_mismatch") return "Header filepaths must match the actual file path for traceability across the repo.";
  if (kind === "extra_dev_docs") return "Developer documentation is centralized in the root README.md and AGENTS.md; extra docs are forbidden.";
  if (kind === "region_name_missing") return "Regions must be named and nested to keep file structure navigable and consistent.";
  if (kind === "region_mismatch") return "Region blocks must be properly nested and closed with matching named end markers.";
  if (kind === "region_unclosed") return "Every opened region must be closed with a matching named end marker.";
  if (kind === "region_duplicate_sibling") return "Sibling regions must have unique names so region structure stays unambiguous.";
  if (kind === "region_empty") return "Empty regions are forbidden so regions always represent real code content.";
  if (kind === "unreadable_file") return "Unreadable files cannot be scanned for code hygiene requirements.";
  if (kind === "forbidden_import") return "Shared UI elements stay domain-neutral, only elements.tsx may reexport third-party dependencies, and Sketchpad scaffolding must remain decoupled from app internals.";
  return "Shared UI elements must remain domain-neutral and avoid app-specific terminology.";
}

function getDefaultIssueSolution(kind: CodeIssueKind): string {
  if (kind === "comment") return "Remove the inline comment and document the guidance in README.md under Products and Components, plus AGENTS.md under SRS Business Logic/UI/UX and Codebase.";
  if (kind === "temporary_log") return "Remove the temporary log or replace it with a warning or error if it is required.";
  if (kind === "missing_license_header") return "Add the SPDX header and header region using the code fix hook.";
  if (kind === "invalid_header_format") return "Regenerate the header with the fix hook to match the required format.";
  if (kind === "header_filepath_mismatch") return "Update the header filepath or regenerate the header to match the file location.";
  if (kind === "extra_dev_docs") return "Move the content into root README.md and AGENTS.md in the required four documentation sections, then remove the extra file.";
  if (kind === "region_name_missing") return "Add a matching name to the region and endregion markers.";
  if (kind === "region_mismatch") return "Fix the region names and nesting so each endregion matches the open region.";
  if (kind === "region_unclosed") return "Add the matching endregion marker with the same name.";
  if (kind === "region_duplicate_sibling") return "Rename one region or merge them into a single region.";
  if (kind === "region_empty") return "Remove the empty region or move relevant code into it.";
  if (kind === "unreadable_file") return "Fix file permissions or encoding so the scanner can read the file.";
  if (kind === "forbidden_import") return "Move shared logic into elements/shared/semio or app-specific modules, then update the import to the allowed path.";
  return "Replace domain terms with neutral wording or move the string into app-specific modules.";
}

function createIssue(params: { path: string; language: CodeLanguage; kind: CodeIssueKind; line: number; column: number; message: string; excerpt?: string; reason?: string; solution?: string }): CodeIssue {
  return {
    path: params.path,
    language: params.language,
    kind: params.kind,
    line: params.line,
    column: params.column,
    message: params.message,
    reason: params.reason ?? getDefaultIssueReason(params.kind),
    solution: params.solution ?? getDefaultIssueSolution(params.kind),
    excerpt: params.excerpt,
  };
}

function getLicenseType(filepath: string): LicenseType {
  const normalized = filepath.replace(/\\/g, "/");
  if (normalized.startsWith("js/js/")) return "LGPL";
  if (normalized.startsWith("net/Semio/")) return "LGPL";
  if (normalized.startsWith("net/Semio.Grasshopper/")) return "LGPL";
  if (normalized.startsWith("py/engine/")) return "LGPL";
  return "AGPL";
}

function getLicenseLines(licenseType: LicenseType): string[] {
  return licenseType === "LGPL" ? LGPL_LICENSE_LINES : AGPL_LICENSE_LINES;
}

function getLicenseValidationText(licenseType: LicenseType): string {
  return licenseType === "LGPL" ? "GNU Lesser General Public License" : "GNU Affero General Public License";
}

function generateHeader(filepath: string, year: number, language: CodeLanguage): string[] {
  const prefix = language === "python" ? "# " : "// ";
  const regionStart = language === "csharp" ? "#region Header" : language === "python" ? "# region Header" : "// #region Header";
  const regionEnd = language === "csharp" ? "#endregion Header" : language === "python" ? "# endregion Header" : "// #endregion Header";
  const licenseType = getLicenseType(filepath);
  const licenseLines = getLicenseLines(licenseType);
  return [regionStart, "", `${prefix}${filepath}`, "", `${prefix}${year} ${CONTRIBUTOR}`, "", ...licenseLines.map((line) => (line ? `${prefix}${line}` : "")), "", regionEnd];
}

function parseExistingHeader(lines: string[], language: CodeLanguage, filepath: string): { hasHeaderRegion: boolean; headerEndIndex: number; isValidFormat: boolean; existingFilepath: string | null; existingYear: number | null } {
  const prefix = language === "python" ? "#" : "//";
  const regionStartPattern = language === "csharp" ? /^\s*#\s*region\s+Header\s*$/i : language === "python" ? /^\s*#\s*region\s+Header\s*$/i : /^\s*\/\/\s*#region\s+Header\s*$/i;
  const regionEndPattern = language === "csharp" ? /^\s*#\s*endregion\s+Header\s*$/i : language === "python" ? /^\s*#\s*endregion(\s+Header)?\s*$/i : /^\s*\/\/\s*#endregion\s+Header\s*$/i;
  const validRegionEndPattern = language === "csharp" ? /^\s*#\s*endregion\s+Header\s*$/i : language === "python" ? /^\s*#\s*endregion\s+Header\s*$/i : /^\s*\/\/\s*#endregion\s+Header\s*$/i;
  let hasHeaderRegion = false;
  let headerEndIndex = -1;
  let isValidFormat = false;
  let existingFilepath: string | null = null;
  let existingYear: number | null = null;
  const maxSearch = language === "python" ? 60 : 5;
  for (let i = 0; i < Math.min(lines.length, maxSearch); i++) {
    if (regionStartPattern.test(lines[i] ?? "")) {
      hasHeaderRegion = true;
      for (let j = i + 1; j < Math.min(lines.length, i + 30); j++) {
        if (regionEndPattern.test(lines[j] ?? "")) {
          headerEndIndex = j;
          break;
        }
        const line = lines[j] ?? "";
        const filepathMatch = line.match(new RegExp(`^\\s*${prefix}\\s*([a-zA-Z0-9_/.-]+\\.[a-z]+)\\s*$`));
        if (filepathMatch && !existingFilepath) existingFilepath = filepathMatch[1] ?? null;
        const yearMatch = line.match(new RegExp(`^\\s*${prefix}\\s*(\\d{4})(?:[\\s-])`));
        if (yearMatch && !existingYear) existingYear = parseInt(yearMatch[1] ?? "0", 10);
      }
      break;
    }
  }
  if (hasHeaderRegion && headerEndIndex > 0 && existingFilepath && existingYear) {
    const headerContent = lines.slice(0, headerEndIndex + 1).join("\n");
    const expectedLicenseText = getLicenseValidationText(getLicenseType(filepath));
    isValidFormat = headerContent.includes(expectedLicenseText) && headerContent.includes(CONTRIBUTOR) && validRegionEndPattern.test(lines[headerEndIndex] ?? "");
  }
  return { hasHeaderRegion, headerEndIndex, isValidFormat, existingFilepath, existingYear };
}

function findHeaderRegionStart(lines: string[], language: CodeLanguage): number {
  const regionStartPattern = language === "csharp" ? /^\s*#\s*region\s+Header\s*$/i : language === "python" ? /^\s*#\s*region\s+Header\s*$/i : /^\s*\/\/\s*#region\s+Header\s*$/i;
  const maxSearch = language === "python" ? 60 : 5;
  for (let i = 0; i < Math.min(lines.length, maxSearch); i++) {
    if (regionStartPattern.test(lines[i] ?? "")) return i;
  }
  return -1;
}

function fixHeader(content: string, lines: string[], filepath: string, language: CodeLanguage): string {
  const year = new Date().getFullYear();
  const header = parseExistingHeader(lines, language, filepath);
  const newHeader = generateHeader(filepath, year, language);
  let preambleEndAt = 0;
  if ((lines[0] ?? "").startsWith("#!")) preambleEndAt = 1;
  if (language === "python" && preambleEndAt === 1 && /^\s*#\s*coding[:=]/i.test(lines[1] ?? "")) preambleEndAt = 2;
  if (language === "typescript") {
    while ((lines[preambleEndAt] ?? "").trimStart().startsWith("///") && /<\s*reference\b/i.test(lines[preambleEndAt] ?? "")) preambleEndAt += 1;
  }
  const preamble = lines.slice(0, preambleEndAt);
  if (header.hasHeaderRegion && header.headerEndIndex > 0) {
    const headerStart = findHeaderRegionStart(lines, language);
    const afterHeader = lines.slice(header.headerEndIndex + 1);
    const betweenPreambleAndHeader = lines.slice(preambleEndAt, headerStart).filter((line) => line.trim() !== "");
    if (betweenPreambleAndHeader.length === 0) {
      return [...preamble, ...newHeader, ...afterHeader].join("\n");
    }
    return [...preamble, ...newHeader, "", ...betweenPreambleAndHeader, ...afterHeader].join("\n");
  }
  return [...preamble, ...newHeader, "", ...lines.slice(preambleEndAt)].join("\n");
}

function normalizePath(path: string): string {
  return path.replace(/\\/g, "/");
}

function getTrackedFiles(): string[] {
  return execSync("git ls-files", { cwd: rootDir, encoding: "utf-8" })
    .split(/\r?\n/g)
    .map((line) => line.trim())
    .filter(Boolean);
}

function getUntrackedFiles(): string[] {
  return execSync("git status --porcelain -uall", { cwd: rootDir, encoding: "utf-8" })
    .split(/\r?\n/g)
    .map((line) => line.trim())
    .filter(Boolean)
    .filter((line) => line.startsWith("?? "))
    .map((line) => line.slice("?? ".length).trim())
    .filter(Boolean);
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

function isExcludedFromImportRules(path: string): boolean {
  const normalized = normalizePath(path);
  if (normalized.includes(".storybook/")) return true;
  if (normalized.includes(".stories.")) return true;
  if (normalized.endsWith(".test.ts") || normalized.endsWith(".test.tsx")) return true;
  if (isConfigFile(path)) return true;
  if (normalized.endsWith("/dev.ts")) return true;
  if (normalized.endsWith("/site.tsx")) return true;
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
  const stack: RegionNode[] = [];
  const rootChildren: RegionNode[] = [];
  const regionRegex = language === "csharp" ? /^\s*#\s*region\b(.*)$/ : language === "python" ? /^\s*#\s*region\b(.*)$/ : /^\s*\/\/\s*#region\b(.*)$/;
  const endRegionRegex = language === "csharp" ? /^\s*#\s*endregion\b(.*)$/ : language === "python" ? /^\s*#\s*endregion\b(.*)$/ : /^\s*\/\/\s*#endregion\b(.*)$/;
  for (let i = 0; i < lines.length; i++) {
    const lineNumber = i + 1;
    const line = lines[i] ?? "";
    const regionMatch = line.match(regionRegex);
    if (regionMatch) {
      const name = (regionMatch[1] ?? "").trim();
      if (!name) issues.push(createIssue({ path, language, kind: "region_name_missing", line: lineNumber, column: 1, message: "Region is missing a name", excerpt: getExcerpt(line) }));
      const node: RegionNode = { name, line: lineNumber, children: [] };
      const siblings = stack.length > 0 ? (stack[stack.length - 1] as RegionNode).children : rootChildren;
      const duplicate = name !== "" ? siblings.find((s) => s.name === name) : undefined;
      if (duplicate)
        issues.push(createIssue({ path, language, kind: "region_duplicate_sibling", line: lineNumber, column: 1, message: `Duplicate sibling region name "${name}" (first occurrence at line ${duplicate.line})`, excerpt: getExcerpt(line) }));
      siblings.push(node);
      stack.push(node);
      continue;
    }
    const endMatch = line.match(endRegionRegex);
    if (endMatch) {
      const name = (endMatch[1] ?? "").trim();
      if (!name) issues.push(createIssue({ path, language, kind: "region_name_missing", line: lineNumber, column: 1, message: "Endregion is missing a name", excerpt: getExcerpt(line) }));
      const current = stack.pop();
      if (!current) {
        issues.push(createIssue({ path, language, kind: "region_mismatch", line: lineNumber, column: 1, message: "Endregion without matching region", excerpt: getExcerpt(line) }));
        continue;
      }
      if (current.name !== name)
        issues.push(createIssue({ path, language, kind: "region_mismatch", line: lineNumber, column: 1, message: `Region mismatch: opened \"${current.name}\" at line ${current.line}, closed \"${name}\"`, excerpt: getExcerpt(line) }));
    }
  }
  for (const open of stack) {
    issues.push(createIssue({ path, language, kind: "region_unclosed", line: open.line, column: 1, message: `Region \"${open.name}\" not closed`, excerpt: getExcerpt(lines[open.line - 1] ?? "") }));
  }
  return issues;
}

// #region EmptyRegions

function getEmptyRegions(lines: string[], language: CodeLanguage): { regions: { name: string; start: number; end: number }[]; linesToRemove: Set<number> } {
  const regions: { name: string; start: number; end: number }[] = [];
  const linesToRemove = new Set<number>();
  const regionRegex = language === "csharp" ? /^\s*#\s*region\b(.*)$/ : language === "python" ? /^\s*#\s*region\b(.*)$/ : /^\s*\/\/\s*#region\b(.*)$/;
  const endRegionRegex = language === "csharp" ? /^\s*#\s*endregion\b(.*)$/ : language === "python" ? /^\s*#\s*endregion\b(.*)$/ : /^\s*\/\/\s*#endregion\b(.*)$/;
  const stack: { name: string; start: number; hasContent: boolean }[] = [];
  for (let i = 0; i < lines.length; i++) {
    const lineNumber = i + 1;
    const line = lines[i] ?? "";
    const regionMatch = line.match(regionRegex);
    if (regionMatch) {
      const name = (regionMatch[1] ?? "").trim();
      stack.push({ name, start: lineNumber, hasContent: false });
      continue;
    }
    if (endRegionRegex.test(line)) {
      const current = stack.pop();
      if (!current) continue;
      if (!current.hasContent) {
        regions.push({ name: current.name, start: current.start, end: lineNumber });
        for (let lineIndex = current.start; lineIndex <= lineNumber; lineIndex++) {
          linesToRemove.add(lineIndex);
        }
      }
      continue;
    }
    if (line.trim() === "") continue;
    for (const entry of stack) {
      entry.hasContent = true;
    }
  }
  return { regions, linesToRemove };
}

// #endregion EmptyRegions

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

function collectTypescriptComments(content: string, sourceFile: ts.SourceFile): ts.CommentRange[] {
  const seenRanges = new Set<string>();
  const allComments: ts.CommentRange[] = [];
  const addRange = (range: ts.CommentRange): void => {
    const key = `${range.pos}-${range.end}`;
    if (!seenRanges.has(key)) {
      seenRanges.add(key);
      allComments.push(range);
    }
  };
  const collectAt = (pos: number): void => {
    const leading = ts.getLeadingCommentRanges(content, pos);
    if (leading) {
      for (const range of leading) {
        addRange(range);
      }
    }
  };
  collectAt(0);
  const visit = (node: ts.Node): void => {
    collectAt(node.getFullStart());
    const jsDocNodes = ts.getJSDocCommentsAndTags(node);
    for (const doc of jsDocNodes) {
      addRange({ pos: doc.pos, end: doc.end, kind: ts.SyntaxKind.MultiLineCommentTrivia });
    }
    const trailing = ts.getTrailingCommentRanges(content, node.getEnd());
    if (trailing) {
      for (const range of trailing) {
        addRange(range);
      }
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return allComments.sort((a, b) => a.pos - b.pos);
}

function scanTypescriptComments(content: string, lines: string[], path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
  if (isConfigFile(path)) return issues;
  const sourceFile = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true, path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const headerLines = getHeaderRegionLines(lines, "typescript");
  const allComments = collectTypescriptComments(content, sourceFile);
  for (const comment of allComments) {
    const { line, character } = sourceFile.getLineAndCharacterOfPosition(comment.pos);
    const lineNumber = line + 1;
    const column = character + 1;
    if (lineNumber <= licenseHeaderEndLine) continue;
    const tokenText = content.slice(comment.pos, comment.end);
    if (headerLines.has(lineNumber)) continue;
    if (/todo/i.test(tokenText)) continue;
    if (tokenText.includes("SPDX-License-Identifier:")) continue;
    if (comment.kind === ts.SyntaxKind.SingleLineCommentTrivia && tokenText.startsWith("///") && /<\s*reference\b/i.test(tokenText)) continue;
    const lineText = lines[lineNumber - 1] ?? "";
    if (comment.kind === ts.SyntaxKind.SingleLineCommentTrivia) {
      const after = lineText.slice(Math.max(0, lineText.indexOf("//") + 2));
      if (isTsRegionLineText(after)) continue;
    }
    issues.push(createIssue({ path, language: "typescript", kind: "comment", line: lineNumber, column, message: comment.kind === ts.SyntaxKind.SingleLineCommentTrivia ? "Line comment found" : "Block comment found", excerpt: getExcerpt(lineText) }));
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
// #region JsJsRules
// #region JsJsRuleConstants
const jsJsRoot = "js/js/";
const sketchpadAppNames = ["Home", "Kit", "Design", "Type", "Quality", "Docs", "Feedback"];
const sketchpadAppPaths = new Set(sketchpadAppNames.map((name) => `js/js/sketchpad/${name}.tsx`));
const sketchpadImportTargets = new Set(["js/js/sketchpad/elements", "js/js/sketchpad/shared", "js/js/sketchpad/Tutorials", "js/js/semio", "js/js/i18n", ...sketchpadAppNames.map((name) => `js/js/sketchpad/${name}`)]);
const appImportTargets = new Set(["js/js/sketchpad/Sketchpad", ...sketchpadImportTargets]);
// #endregion JsJsRuleConstants
// #region JsJsRuleScans
function scanTypescriptForbiddenImports(content: string, lines: string[], path: string): CodeIssue[] {
  const issues: CodeIssue[] = [];
  const normalizedPath = normalizePath(path);
  if (!normalizedPath.startsWith(jsJsRoot)) return issues;
  if (isExcludedFromImportRules(path)) return issues;
  const isElementsFile = normalizedPath.endsWith("/elements.tsx");
  const isSketchpadFile = normalizedPath === "js/js/sketchpad/Sketchpad.tsx";
  const isAppFile = sketchpadAppPaths.has(normalizedPath);
  const sourceFile = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true, path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const addIssue = (node: ts.Node, message: string, reason: string, solution: string): void => {
    const { line, character } = sourceFile.getLineAndCharacterOfPosition(node.getStart(sourceFile));
    const lineNumber = line + 1;
    issues.push(createIssue({ path, language: "typescript", kind: "forbidden_import", line: lineNumber, column: character + 1, message, reason, solution, excerpt: getExcerpt(lines[lineNumber - 1] ?? "") }));
  };
  const elementsAllowedTargets = new Set(["js/js/i18n", "js/js/semio"]);
  const checkModuleText = (node: ts.Node, moduleText: string): void => {
    if (isElementsFile && moduleText.startsWith(".")) {
      const resolved = normalizePath(join(dirname(normalizedPath), moduleText)).replace(/\.[^.\/]+$/, "");
      if (!elementsAllowedTargets.has(resolved)) {
        addIssue(
          node,
          "Relative imports in elements.tsx must target i18n.ts or semio.ts",
          "elements.tsx is the domain-neutral shared UI library and the only js/js file allowed to import third-party dependencies; it may only import js/js/i18n.ts or js/js/semio.ts.",
          "Move shared functionality into elements.tsx or into js/js/i18n.ts or js/js/semio.ts, then import from the approved path.",
        );
      }
      return;
    }
    if (isSketchpadFile || isAppFile) {
      if (!moduleText.startsWith(".")) return;
      const resolved = normalizePath(join(dirname(normalizedPath), moduleText)).replace(/\.[^.\/]+$/, "");
      if ((isSketchpadFile && !sketchpadImportTargets.has(resolved)) || (isAppFile && !appImportTargets.has(resolved)))
        addIssue(
          node,
          "Relative imports must target elements.tsx, Sketchpad.tsx, semio.ts, or shared.ts",
          "Sketchpad.tsx provides scaffolding and must remain independent of app internals; apps import only shared elements, shared utilities, and core domain modules.",
          "Move shared logic into shared modules (elements/shared/semio) and update the import so Sketchpad.tsx and apps remain decoupled.",
        );
      return;
    }
  };
  const visit = (node: ts.Node): void => {
    if (ts.isImportDeclaration(node) && ts.isStringLiteralLike(node.moduleSpecifier)) checkModuleText(node.moduleSpecifier, node.moduleSpecifier.text);
    if (ts.isExportDeclaration(node) && node.moduleSpecifier && ts.isStringLiteralLike(node.moduleSpecifier)) checkModuleText(node.moduleSpecifier, node.moduleSpecifier.text);
    if (ts.isImportEqualsDeclaration(node) && ts.isExternalModuleReference(node.moduleReference) && ts.isStringLiteralLike(node.moduleReference.expression)) checkModuleText(node.moduleReference.expression, node.moduleReference.expression.text);
    if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword) {
      const arg = node.arguments[0];
      if (arg && ts.isTemplateExpression(arg)) return;
      const argText = getTypescriptStringLiteralText(arg ?? node);
      if (argText !== null) checkModuleText(arg ?? node, argText);
    }
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "require") {
      const argText = getTypescriptStringLiteralText(node.arguments[0] ?? node);
      if (argText !== null) checkModuleText(node.arguments[0] ?? node, argText);
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return issues;
}

function scanTypescriptForbiddenTerminology(content: string, lines: string[], path: string): CodeIssue[] {
  const issues: CodeIssue[] = [];
  const normalizedPath = normalizePath(path);
  if (!normalizedPath.startsWith(jsJsRoot) || !normalizedPath.endsWith("/elements.tsx")) return issues;
  const sourceFile = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true, path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const forbiddenRegex = /\b(kit|design|type|connector|connection|docs|feedback)\b/i;
  const allowedPatterns = [/@dnd-kit/, /\/docs\//, /semio\.sketchpad\.docs\./, /^type$/, /^id$/];
  const isAllowed = (text: string): boolean => allowedPatterns.some((p) => p.test(text));
  const addIssue = (node: ts.Node, term: string): void => {
    const { line, character } = sourceFile.getLineAndCharacterOfPosition(node.getStart(sourceFile));
    const lineNumber = line + 1;
    issues.push(
      createIssue({
        path,
        language: "typescript",
        kind: "forbidden_terminology",
        line: lineNumber,
        column: character + 1,
        message: `Forbidden terminology \"${term}\"`,
        reason: "Shared UI elements must stay domain-neutral and avoid app-specific terminology.",
        solution: "Replace the term with neutral wording or move the string into app-specific modules.",
        excerpt: getExcerpt(lines[lineNumber - 1] ?? ""),
      }),
    );
  };
  const visit = (node: ts.Node): void => {
    if (ts.isJsxText(node)) {
      const match = node.text.match(forbiddenRegex);
      if (match && !isAllowed(node.text)) addIssue(node, match[0]);
    }
    const literalText = getTypescriptStringLiteralText(node);
    if (literalText !== null) {
      const match = literalText.match(forbiddenRegex);
      if (match && !isAllowed(literalText)) addIssue(node, match[0]);
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return issues;
}
// #endregion JsJsRuleScans
// #endregion JsJsRules

function scanTypescriptTemporaryLogs(content: string, lines: string[], path: string, licenseHeaderEndLine: number): CodeIssue[] {
  const issues: CodeIssue[] = [];
  const sourceFile = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true, path.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const methods = new Set(["log", "debug", "info", "warn", "error"]);
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node)) {
      const expression = node.expression;
      if (ts.isPropertyAccessExpression(expression) && ts.isIdentifier(expression.expression) && expression.expression.text === "console" && methods.has(expression.name.text)) {
        const debugArg = node.arguments
          .map((arg) => getTypescriptStringLiteralText(arg))
          .filter((text): text is string => text !== null)
          .some((text) => text.includes("[DEBUG]"));
        if (debugArg) {
          const { line, character } = sourceFile.getLineAndCharacterOfPosition(node.getStart(sourceFile));
          const lineNumber = line + 1;
          if (lineNumber > licenseHeaderEndLine)
            issues.push(createIssue({ path, language: "typescript", kind: "temporary_log", line: lineNumber, column: character + 1, message: "Temporary [DEBUG] console log found", excerpt: getExcerpt(lines[lineNumber - 1] ?? "") }));
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
  const allComments = collectTypescriptComments(content, sourceFile);
  const removals: { start: number; end: number; insertSpace: boolean }[] = [];
  for (const comment of allComments) {
    const { line } = sourceFile.getLineAndCharacterOfPosition(comment.pos);
    const lineNumber = line + 1;
    if (lineNumber <= licenseHeaderEndLine) continue;
    const tokenText = content.slice(comment.pos, comment.end);
    if (headerLines.has(lineNumber)) continue;
    if (/todo/i.test(tokenText)) continue;
    if (tokenText.includes("SPDX-License-Identifier:")) continue;
    if (comment.kind === ts.SyntaxKind.SingleLineCommentTrivia) {
      const lineText = lines[lineNumber - 1] ?? "";
      const after = lineText.slice(Math.max(0, lineText.indexOf("//") + 2));
      if (isTsRegionLineText(after)) continue;
      if (tokenText.startsWith("///") && /<\s*reference\b/i.test(tokenText)) continue;
    }
    const prev = comment.pos > 0 ? (content[comment.pos - 1] ?? "") : "";
    const next = comment.end < content.length ? (content[comment.end] ?? "") : "";
    const insertSpace = /[A-Za-z0-9_$]/.test(prev) && /[A-Za-z0-9_$]/.test(next);
    removals.push({ start: comment.pos, end: comment.end, insertSpace });
  }
  if (removals.length === 0) return content;
  let out = "";
  let index = 0;
  for (const removal of removals) {
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
    if (!inSingle && !inTemplate && ch === '"' && !inDouble) {
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
      if (ch === '"') inDouble = false;
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
      issues.push(createIssue({ path, language, kind: "comment", line, column: col, message: "Line comment found", excerpt: getExcerpt(lineText) }));
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
      issues.push(createIssue({ path, language, kind: "comment", line, column: col, message: "Block comment found", excerpt: getExcerpt(lineText) }));
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
      if (ch === '"' && next === '"' && next2 === '"') {
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
      if (ch === '"') inDouble = false;
      col += 1;
      continue;
    }
    if (ch === "'" && next === "'" && next2 === "'") {
      inTripleSingle = true;
      i += 2;
      col += 3;
      continue;
    }
    if (ch === '"' && next === '"' && next2 === '"') {
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
    if (ch === '"') {
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
      issues.push(createIssue({ path, language: "python", kind: "comment", line, column: col, message: "Line comment found", excerpt: getExcerpt(lineText) }));
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
    issues.push(createIssue({ path, language: "python", kind: "temporary_log", line: lineNumber, column: 1, message: "Temporary [DEBUG] log found", excerpt: getExcerpt(lineText) }));
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
      if (ch === '"' && next === '"') {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === '"') inVerbatimString = false;
      col += 1;
      continue;
    }
    if (inString) {
      if (ch === "\\" && next) {
        i += 1;
        col += 2;
        continue;
      }
      if (ch === '"') inString = false;
      col += 1;
      continue;
    }
    if (ch === "'" && !inChar) {
      inChar = true;
      col += 1;
      continue;
    }
    if (ch === "$" && next === "@" && next2 === '"') {
      inVerbatimString = true;
      i += 2;
      col += 3;
      continue;
    }
    if (ch === "@" && next === "$" && next2 === '"') {
      inVerbatimString = true;
      i += 2;
      col += 3;
      continue;
    }
    if (ch === "@" && next === '"') {
      inVerbatimString = true;
      i += 1;
      col += 2;
      continue;
    }
    if (ch === "$" && next === '"') {
      inString = true;
      i += 1;
      col += 2;
      continue;
    }
    if (ch === '"') {
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
      issues.push(createIssue({ path, language: "csharp", kind: "comment", line, column: col, message: "Line comment found", excerpt: getExcerpt(lineText) }));
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
      issues.push(createIssue({ path, language: "csharp", kind: "comment", line, column: col, message: "Block comment found", excerpt: getExcerpt(lineText) }));
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
    issues.push(createIssue({ path, language: "csharp", kind: "temporary_log", line: lineNumber, column: 1, message: "Temporary [DEBUG] console log found", excerpt: getExcerpt(lineText) }));
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
      if (ch === '"' && next === '"') {
        out += '"';
        i += 1;
        continue;
      }
      if (ch === '"') inVerbatimString = false;
      continue;
    }
    if (inString) {
      out += ch;
      if (ch === "\\" && next) {
        out += next;
        i += 1;
        continue;
      }
      if (ch === '"') inString = false;
      continue;
    }
    if (ch === "'") {
      inChar = true;
      out += ch;
      continue;
    }
    if (ch === "$" && next === "@" && next2 === '"') {
      out += '$@"';
      i += 2;
      inVerbatimString = true;
      continue;
    }
    if (ch === "@" && next === "$" && next2 === '"') {
      out += '@$"';
      i += 2;
      inVerbatimString = true;
      continue;
    }
    if (ch === "@" && next === '"') {
      out += '@"';
      i += 1;
      inVerbatimString = true;
      continue;
    }
    if (ch === "$" && next === '"') {
      out += '$"';
      i += 1;
      inString = true;
      continue;
    }
    if (ch === '"') {
      out += '"';
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
    if (!inSingle && !inTemplate && ch === '"' && !inDouble) {
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
      if (ch === '"') inDouble = false;
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
      if (ch === '"' && next === '"' && next2 === '"') {
        out += '""';
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
      if (ch === '"') inDouble = false;
      continue;
    }
    if (ch === "'" && next === "'" && next2 === "'") {
      out += "'''";
      i += 2;
      inTripleSingle = true;
      continue;
    }
    if (ch === '"' && next === '"' && next2 === '"') {
      out += '"""';
      i += 2;
      inTripleDouble = true;
      continue;
    }
    if (ch === "'") {
      inSingle = true;
      out += ch;
      continue;
    }
    if (ch === '"') {
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
  const byKind: Record<CodeIssueKind, number> = {
    comment: 0,
    temporary_log: 0,
    missing_license_header: 0,
    invalid_header_format: 0,
    header_filepath_mismatch: 0,
    extra_dev_docs: 0,
    region_name_missing: 0,
    region_mismatch: 0,
    region_unclosed: 0,
    region_duplicate_sibling: 0,
    region_empty: 0,
    unreadable_file: 0,
    forbidden_import: 0,
    forbidden_terminology: 0,
  };
  const byLanguage: Record<CodeLanguage, number> = { typescript: 0, python: 0, csharp: 0, meta: 0 };
  const repoFiles = getRepoFiles().filter(shouldScan);
  const packageRoots = getPackageRootDirs(repoFiles);
  for (const docPath of repoFiles.filter((file) => isExtraDevDoc(file, packageRoots))) {
    const absolute = join(rootDir, docPath);
    if (fix) {
      try {
        if (existsSync(absolute)) unlinkSync(absolute);
      } catch {
        issues.push(createIssue({ path: docPath, language: "meta", kind: "extra_dev_docs", line: 1, column: 1, message: "Extra dev doc detected but could not be deleted" }));
        byKind.extra_dev_docs += 1;
        byLanguage.meta += 1;
      }
      continue;
    }
    issues.push(createIssue({ path: docPath, language: "meta", kind: "extra_dev_docs", line: 1, column: 1, message: "Extra README.md/AGENTS.md outside repo root" }));
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
      const issue = createIssue({ path, language, kind: "unreadable_file", line: 1, column: 1, message: "Failed to read file" });
      issues.push(issue);
      byKind[issue.kind] += 1;
      continue;
    }
    let lines = splitLines(content);
    const headerInfo = parseExistingHeader(lines, language, path);
    if (fix && !isConfigFile(path)) {
      const needsHeaderFix = !headerInfo.isValidFormat || headerInfo.existingFilepath !== path;
      if (needsHeaderFix) {
        content = fixHeader(content, lines, path, language);
        writeFileSync(absolute, content, "utf-8");
        lines = splitLines(content);
      }
    }
    let license = getLicenseHeaderEndLine(lines, language);
    if (fix && !isConfigFile(path)) {
      const updated =
        language === "python"
          ? stripPythonComments(content, lines, license.headerEndLine)
          : language === "typescript"
            ? stripTypescriptComments(content, lines, path, license.headerEndLine)
            : stripCsharpComments(content, lines, license.headerEndLine);
      if (updated !== content) {
        writeFileSync(absolute, updated, "utf-8");
        content = updated;
      }
      lines = splitLines(content);
      license = getLicenseHeaderEndLine(lines, language);
    }
    if (!isConfigFile(path)) {
      const updatedHeaderInfo = parseExistingHeader(lines, language, path);
      const expectedLicense = getLicenseType(path) === "LGPL" ? "LGPL-3.0" : "AGPL-3.0";
      if (!updatedHeaderInfo.isValidFormat) {
        const issue = createIssue({ path, language, kind: "invalid_header_format", line: 1, column: 1, message: `Header does not follow the required format (#region Header, filepath, contributor, ${expectedLicense})` });
        issues.push(issue);
        byKind[issue.kind] += 1;
      } else if (updatedHeaderInfo.existingFilepath !== path) {
        const issue = createIssue({ path, language, kind: "header_filepath_mismatch", line: 1, column: 1, message: `Header filepath mismatch: expected "${path}", found "${updatedHeaderInfo.existingFilepath}"` });
        issues.push(issue);
        byKind[issue.kind] += 1;
      }
    }
    // #region EmptyRegionsUsage
    let emptyRegions = getEmptyRegions(lines, language);
    if (fix && !isConfigFile(path) && emptyRegions.regions.length > 0) {
      const updatedLines = lines.filter((_, index) => !emptyRegions.linesToRemove.has(index + 1));
      const updatedContent = updatedLines.join("\n");
      writeFileSync(absolute, updatedContent, "utf-8");
      content = updatedContent;
      lines = updatedLines;
      license = getLicenseHeaderEndLine(lines, language);
      emptyRegions = getEmptyRegions(lines, language);
    }
    for (const region of emptyRegions.regions) {
      const issue = createIssue({ path, language, kind: "region_empty", line: region.start, column: 1, message: `Empty region "${region.name}"`, excerpt: getExcerpt(lines[region.start - 1] ?? "") });
      issues.push(issue);
      byKind[issue.kind] += 1;
    }
    // #endregion EmptyRegionsUsage
    for (const issue of parseRegions(lines, language, path)) {
      issues.push(issue);
      byKind[issue.kind] += 1;
    }
    const tempLogIssues =
      language === "python"
        ? scanPythonTemporaryLogs(lines, path, license.headerEndLine)
        : language === "typescript"
          ? scanTypescriptTemporaryLogs(content, lines, path, license.headerEndLine)
          : scanCsharpTemporaryLogs(lines, path, license.headerEndLine);
    for (const issue of tempLogIssues) {
      issues.push(issue);
      byKind[issue.kind] += 1;
    }
    const commentIssues =
      language === "python"
        ? scanPythonComments(content, lines, path, license.headerEndLine)
        : language === "typescript"
          ? scanTypescriptComments(content, lines, path, license.headerEndLine)
          : scanCsharpComments(content, lines, path, license.headerEndLine);
    for (const issue of commentIssues) {
      issues.push(issue);
      byKind[issue.kind] += 1;
    }
    if (language === "typescript") {
      const forbiddenImportIssues = scanTypescriptForbiddenImports(content, lines, path);
      for (const issue of forbiddenImportIssues) {
        issues.push(issue);
        byKind[issue.kind] += 1;
      }
      const forbiddenTerminologyIssues = scanTypescriptForbiddenTerminology(content, lines, path);
      for (const issue of forbiddenTerminologyIssues) {
        issues.push(issue);
        byKind[issue.kind] += 1;
      }
    }
  }
  const report: CodeReport = { timestamp: new Date().toISOString(), status: issues.length === 0 ? "success" : "error", summary: { filesScanned, issues: issues.length, byKind, byLanguage }, issues };
  writeReport(report);
  if (fix) process.exit(issues.some((issue) => issue.kind === "unreadable_file") ? 1 : 0);
  if (report.status === "success") process.exit(0);
  process.exit(1);
}

function App() {
  const [status, setStatus] = React.useState<"running" | "success" | "error">("running");
  const [report, setReport] = React.useState<CodeReport | null>(null);

  React.useEffect(() => {
    const issues: CodeIssue[] = [];
    let filesScanned = 0;
    const byKind: Record<CodeIssueKind, number> = {
      comment: 0,
      temporary_log: 0,
      missing_license_header: 0,
      invalid_header_format: 0,
      header_filepath_mismatch: 0,
      extra_dev_docs: 0,
      region_name_missing: 0,
      region_mismatch: 0,
      region_unclosed: 0,
      region_duplicate_sibling: 0,
      region_empty: 0,
      unreadable_file: 0,
      forbidden_import: 0,
      forbidden_terminology: 0,
    };
    const byLanguage: Record<CodeLanguage, number> = { typescript: 0, python: 0, csharp: 0, meta: 0 };
    const repoFiles = getRepoFiles().filter(shouldScan);
    const packageRoots = getPackageRootDirs(repoFiles);
    for (const docPath of repoFiles.filter((file) => isExtraDevDoc(file, packageRoots))) {
      const absolute = join(rootDir, docPath);
      if (fix) {
        try {
          if (existsSync(absolute)) unlinkSync(absolute);
        } catch {
          issues.push(createIssue({ path: docPath, language: "meta", kind: "extra_dev_docs", line: 1, column: 1, message: "Extra dev doc detected but could not be deleted" }));
          byKind.extra_dev_docs += 1;
          byLanguage.meta += 1;
        }
        continue;
      }
      issues.push(createIssue({ path: docPath, language: "meta", kind: "extra_dev_docs", line: 1, column: 1, message: "Extra README.md/AGENTS.md outside repo root" }));
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
        const issue = createIssue({ path, language, kind: "unreadable_file", line: 1, column: 1, message: "Failed to read file" });
        issues.push(issue);
        byKind[issue.kind] += 1;
        continue;
      }
      let lines = splitLines(content);
      const headerInfo = parseExistingHeader(lines, language, path);
      if (fix && !isConfigFile(path)) {
        const needsHeaderFix = !headerInfo.isValidFormat || headerInfo.existingFilepath !== path;
        if (needsHeaderFix) {
          content = fixHeader(content, lines, path, language);
          writeFileSync(absolute, content, "utf-8");
          lines = splitLines(content);
        }
      }
      let license = getLicenseHeaderEndLine(lines, language);
      if (fix && !isConfigFile(path)) {
        const updated =
          language === "python"
            ? stripPythonComments(content, lines, license.headerEndLine)
            : language === "typescript"
              ? stripTypescriptComments(content, lines, path, license.headerEndLine)
              : stripCsharpComments(content, lines, license.headerEndLine);
        if (updated !== content) {
          writeFileSync(absolute, updated, "utf-8");
          content = updated;
        }
        lines = splitLines(content);
        license = getLicenseHeaderEndLine(lines, language);
      }
      if (!isConfigFile(path)) {
        const updatedHeaderInfo = parseExistingHeader(lines, language, path);
        const expectedLicense = getLicenseType(path) === "LGPL" ? "LGPL-3.0" : "AGPL-3.0";
        if (!updatedHeaderInfo.isValidFormat) {
          const issue = createIssue({ path, language, kind: "invalid_header_format", line: 1, column: 1, message: `Header does not follow the required format (#region Header, filepath, contributor, ${expectedLicense})` });
          issues.push(issue);
          byKind[issue.kind] += 1;
        } else if (updatedHeaderInfo.existingFilepath !== path) {
          const issue = createIssue({ path, language, kind: "header_filepath_mismatch", line: 1, column: 1, message: `Header filepath mismatch: expected "${path}", found "${updatedHeaderInfo.existingFilepath}"` });
          issues.push(issue);
          byKind[issue.kind] += 1;
        }
      }
      let emptyRegions = getEmptyRegions(lines, language);
      if (fix && !isConfigFile(path) && emptyRegions.regions.length > 0) {
        const updatedLines = lines.filter((_, index) => !emptyRegions.linesToRemove.has(index + 1));
        const updatedContent = updatedLines.join("\n");
        writeFileSync(absolute, updatedContent, "utf-8");
        content = updatedContent;
        lines = updatedLines;
        license = getLicenseHeaderEndLine(lines, language);
        emptyRegions = getEmptyRegions(lines, language);
      }
      for (const region of emptyRegions.regions) {
        const issue = createIssue({ path, language, kind: "region_empty", line: region.start, column: 1, message: `Empty region "${region.name}"`, excerpt: getExcerpt(lines[region.start - 1] ?? "") });
        issues.push(issue);
        byKind[issue.kind] += 1;
      }
      for (const issue of parseRegions(lines, language, path)) {
        issues.push(issue);
        byKind[issue.kind] += 1;
      }
      const tempLogIssues =
        language === "python"
          ? scanPythonTemporaryLogs(lines, path, license.headerEndLine)
          : language === "typescript"
            ? scanTypescriptTemporaryLogs(content, lines, path, license.headerEndLine)
            : scanCsharpTemporaryLogs(lines, path, license.headerEndLine);
      for (const issue of tempLogIssues) {
        issues.push(issue);
        byKind[issue.kind] += 1;
      }
      const commentIssues =
        language === "python"
          ? scanPythonComments(content, lines, path, license.headerEndLine)
          : language === "typescript"
            ? scanTypescriptComments(content, lines, path, license.headerEndLine)
            : scanCsharpComments(content, lines, path, license.headerEndLine);
      for (const issue of commentIssues) {
        issues.push(issue);
        byKind[issue.kind] += 1;
      }
      if (language === "typescript") {
        const forbiddenImportIssues = scanTypescriptForbiddenImports(content, lines, path);
        for (const issue of forbiddenImportIssues) {
          issues.push(issue);
          byKind[issue.kind] += 1;
        }
        const forbiddenTerminologyIssues = scanTypescriptForbiddenTerminology(content, lines, path);
        for (const issue of forbiddenTerminologyIssues) {
          issues.push(issue);
          byKind[issue.kind] += 1;
        }
      }
    }
    const finalReport: CodeReport = { timestamp: new Date().toISOString(), status: issues.length === 0 ? "success" : "error", summary: { filesScanned, issues: issues.length, byKind, byLanguage }, issues };
    writeReport(finalReport);
    setReport(finalReport);
    setStatus(finalReport.status === "success" ? "success" : "error");
    const exitCode = fix ? (issues.some((issue) => issue.kind === "unreadable_file") ? 1 : 0) : finalReport.status === "success" ? 0 : 1;
    setTimeout(() => process.exit(exitCode), 100);
  }, []);

  return (
    <Box flexDirection="column">
      <Text>{fix ? "🔧 Fixing codebase..." : "🔍 Analyzing codebase..."}</Text>
      {report && (
        <>
          <Text>
            {status === "success" ? "✅" : "❌"} {report.summary.filesScanned} files, {report.summary.issues} issues
          </Text>
          <Text dimColor>📝 Report: {reportPath}</Text>
        </>
      )}
    </Box>
  );
}

render(<App />);
