// #region 🧲Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Source code parsing: sections via region markers, definitions via regex, scope building. Port of repo/go parsing.

// Specs:
// - Detect region markers: #region 🔖Name / #endregion 🔖Name
// - Detect markdown headings for .md/.mdx files
// - Language-specific definition patterns for Go, TS, JS, Python, C#, Rust, Ruby
// - Build scope IDs deterministically
// #endregion 🧲Header

// #region ⚙️Types
import type { Scope } from "./db";

export interface ParsedSection {
  name: string;
  path: string;
  startLine: number;
  endLine: number;
}

export interface ParsedDefinition {
  name: string;
  startLine: number;
  endLine: number;
}
// #endregion ⚙️Types

// 🔬#region 📯RegionMarker
export function parseRegionMarker(
  line: string
): { name: string; isEnd: boolean } | null {
  let trimmed = line.trim();
  trimmed = trimmed.replace(/^\/\/\s*/, "");
  trimmed = trimmed.replace(/^#\s*/, "");
  trimmed = trimmed.replace(/^--\s*/, "");
  trimmed = trimmed.replace(/^\/\*\s*/, "");
  trimmed = trimmed.replace(/\*\/\s*$/, "");
  trimmed = trimmed.trim();
  if (trimmed.startsWith("#region 🔖")) {
    return { name: trimmed.replace("#region 🔖", "").trim(), isEnd: false };
  }
  if (trimmed.startsWith("#endregion 🔖")) {
    return { name: trimmed.replace("#endregion 🔖", "").trim(), isEnd: true };
  }
  return null;
}
// #endregion 📯RegionMarker

// 📰#region 🪁MarkdownHeading
export function parseMarkdownHeading(
  line: string
): { level: number; title: string } | null {
  const trimmed = line.trim();
  if (!trimmed.startsWith("#")) return null;
  let level = 0;
  while (level < trimmed.length && trimmed[level] === "#") level++;
  if (level === 0 || level > 6) return null;
  const name = trimmed.slice(level).trim();
  if (!name) return null;
  return { level, title: name };
}
// #endregion 🪁MarkdownHeading

// 📖#region 🖋️DefinitionPatterns
export function definitionPatterns(ext: string): RegExp[] {
  switch (ext) {
    case ".go":
      return [
        /^\s*func\s+(?:\([^)]*\)\s*)?([A-Za-z0-9_]+)/,
        /^\s*type\s+([A-Za-z0-9_]+)/,
        /^\s*var\s+([A-Za-z0-9_]+)/,
        /^\s*const\s+([A-Za-z0-9_]+)/,
      ];
    case ".ts":
    case ".tsx":
    case ".js":
    case ".jsx":
      return [
        /^\s*(?:export\s+)?(?:async\s+)?function\s+([A-Za-z0-9_]+)/,
        /^\s*(?:export\s+)?class\s+([A-Za-z0-9_]+)/,
        /^\s*(?:export\s+)?interface\s+([A-Za-z0-9_]+)/,
        /^\s*(?:export\s+)?type\s+([A-Za-z0-9_]+)/,
      ];
    case ".py":
      return [/^\s*def\s+([A-Za-z0-9_]+)/, /^\s*class\s+([A-Za-z0-9_]+)/];
    case ".cs":
      return [
        /^\s*(?:public|private|protected|internal)?\s*(?:static\s+)?(?:class|struct|interface|enum|record)\s+([A-Za-z0-9_]+)/,
      ];
    case ".rs":
      return [
        /^\s*(?:pub\s+)?fn\s+([A-Za-z0-9_]+)/,
        /^\s*(?:pub\s+)?struct\s+([A-Za-z0-9_]+)/,
        /^\s*(?:pub\s+)?enum\s+([A-Za-z0-9_]+)/,
        /^\s*(?:pub\s+)?trait\s+([A-Za-z0-9_]+)/,
        /^\s*impl\s+([A-Za-z0-9_]+)/,
      ];
    case ".rb":
      return [
        /^\s*def\s+([A-Za-z0-9_]+)/,
        /^\s*class\s+([A-Za-z0-9_]+)/,
        /^\s*module\s+([A-Za-z0-9_]+)/,
      ];
    default:
      return [];
  }
}
// #endregion 🖋️DefinitionPatterns

// 📑#region 🐹ParseSections
export function parseSectionsFromLines(
  lines: string[],
  ext: string
): ParsedSection[] {
  const sections: ParsedSection[] = [];
  interface SectionFrame {
    name: string;
    startLine: number;
    level: number;
    path: string;
  }
  const stack: SectionFrame[] = [];

  for (let i = 0; i < lines.length; i++) {
    const lineNumber = i + 1;
    const line = lines[i];
    const marker = parseRegionMarker(line);
    if (marker) {
      if (marker.isEnd) {
        if (stack.length > 0) {
          const frame = stack.pop()!;
          sections.push({
            name: frame.name,
            path: frame.path,
            startLine: frame.startLine,
            endLine: lineNumber - 1,
          });
        }
      } else {
        const path =
          stack.length > 0
            ? `${stack[stack.length - 1].path}.${marker.name}`
            : marker.name;
        stack.push({ name: marker.name, startLine: lineNumber, level: 0, path });
      }
      continue;
    }
    if (ext === ".md" || ext === ".mdx") {
      const heading = parseMarkdownHeading(line);
      if (heading) {
        while (
          stack.length > 0 &&
          stack[stack.length - 1].level >= heading.level
        ) {
          const frame = stack.pop()!;
          sections.push({
            name: frame.name,
            path: frame.path,
            startLine: frame.startLine,
            endLine: lineNumber - 1,
          });
        }
        const path =
          stack.length > 0
            ? `${stack[stack.length - 1].path}.${heading.title}`
            : heading.title;
        stack.push({
          name: heading.title,
          startLine: lineNumber,
          level: heading.level,
          path,
        });
      }
    }
  }

  for (const frame of stack) {
    sections.push({
      name: frame.name,
      path: frame.path,
      startLine: frame.startLine,
      endLine: lines.length,
    });
  }
  return sections;
}
// #endregion 🐹ParseSections

// 📖#region 💡ParseDefinitions
export function parseDefinitionsFromLines(
  lines: string[],
  patterns: RegExp[]
): ParsedDefinition[] {
  const defs: ParsedDefinition[] = [];
  for (let i = 0; i < lines.length; i++) {
    const lineNumber = i + 1;
    for (const pattern of patterns) {
      const match = lines[i].match(pattern);
      if (match && match.length > 1) {
        defs.push({
          name: match[match.length - 1],
          startLine: lineNumber,
          endLine: lineNumber,
        });
        break;
      }
    }
  }
  return defs;
}
// #endregion 💡ParseDefinitions

// 🔭#region ⛩️ScopeBuilding
export function buildScopeId(
  kind: string,
  filePath: string,
  sectionPath: string,
  definition: string
): string {
  if (kind === "file") return `file:${filePath}`;
  if (kind === "section") return `section:${filePath}#${sectionPath}`;
  if (sectionPath) return `def:${filePath}#${sectionPath}::${definition}`;
  return `def:${filePath}#${definition}`;
}

export function buildScopesForFile(path: string, content: string): Scope[] {
  const lines = content.split("\n");
  const ext = "." + (path.split(".").pop() || "").toLowerCase();
  const now = new Date();
  const entries: Scope[] = [];

  entries.push({
    id: buildScopeId("file", path, "", ""),
    kind: "file",
    file_path: path,
    section_path: "",
    definition_name: "",
    start_line: 1,
    end_line: lines.length,
    updated_at: now,
  });

  const sections = parseSectionsFromLines(lines, ext);
  for (const s of sections) {
    entries.push({
      id: buildScopeId("section", path, s.path, ""),
      kind: "section",
      file_path: path,
      section_path: s.path,
      definition_name: "",
      start_line: s.startLine,
      end_line: s.endLine,
      updated_at: now,
    });
  }

  const sectionByLine: Record<number, string> = {};
  for (const s of sections) {
    for (let line = s.startLine; line <= s.endLine; line++) {
      sectionByLine[line] = s.path;
    }
  }

  const patterns = definitionPatterns(ext);
  const defs = parseDefinitionsFromLines(lines, patterns);
  for (const d of defs) {
    const sp = sectionByLine[d.startLine] || "";
    entries.push({
      id: buildScopeId("definition", path, sp, d.name),
      kind: "definition",
      file_path: path,
      section_path: sp,
      definition_name: d.name,
      start_line: d.startLine,
      end_line: d.endLine,
      updated_at: now,
    });
  }

  return entries;
}
// #endregion ⛩️ScopeBuilding

// #region 🐍DiffParsing
// Unified diff parser ported from Go server.

export interface DiffHunk {
  oldRange: { start: number; end: number };
  newRange: { start: number; end: number };
}

export interface DiffFile {
  path: string;
  hunks: DiffHunk[];
  deleted: boolean;
  created: boolean;
}

const hunkHeaderRe = /@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@/;

export function parseUnifiedDiff(patch: string): DiffFile[] {
  const lines = patch.split("\n");
  const files: DiffFile[] = [];
  let current: DiffFile | null = null;

  for (const line of lines) {
    if (line.startsWith("diff --git ")) {
      const parts = line.split(" ");
      if (parts.length >= 4) {
        const path = parts[3].replace(/^b\//, "");
        current = { path, hunks: [], deleted: false, created: false };
        files.push(current);
      }
      continue;
    }
    if (line.startsWith("+++ ") && current) {
      if (line.includes("/dev/null")) current.deleted = true;
      continue;
    }
    if (line.startsWith("@@ ") && current) {
      const match = line.match(hunkHeaderRe);
      if (match) {
        const oldStart = parseInt(match[1]);
        const oldCount = match[2] ? parseInt(match[2]) : 1;
        const newStart = parseInt(match[3]);
        const newCount = match[4] ? parseInt(match[4]) : 1;
        current.hunks.push({
          oldRange: { start: oldStart, end: oldStart + oldCount - 1 },
          newRange: { start: newStart, end: newStart + newCount - 1 },
        });
      }
    }
  }
  return files;
}
// #endregion 🐍DiffParsing
