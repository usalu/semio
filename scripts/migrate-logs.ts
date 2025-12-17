// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2024 Ueli Saluz

import * as fs from "fs";
import * as path from "path";
import matter from "gray-matter";

//#region Types
interface LegacyFileEntry {
  path: string;
  lines?: { added: number; removed: number };
}
interface LegacyFiles {
  updated?: (string | LegacyFileEntry)[];
  created?: (string | LegacyFileEntry)[];
  removed?: (string | LegacyFileEntry)[];
  read?: string[];
}
interface LegacyIteration {
  prompt: string;
  date: string;
  model?: string;
  author?: string;
  commit?: string;
  finished?: string;
  files?: LegacyFiles;
}
interface LegacyFrontmatter {
  slug: string;
  summary?: string;
  status?: "open" | "finished";
  author?: string;
  created?: string;
  base?: string;
  finished?: string;
  files?: LegacyFiles;
  lines?: { added: number; removed: number };
  iterations?: LegacyIteration[];
}
interface NewFileEntry {
  path: string;
  lines?: { added: number; removed: number };
}
interface NewFiles {
  updated: NewFileEntry[];
  created: NewFileEntry[];
  removed: NewFileEntry[];
}
interface NewIteration {
  prompt: string;
  date: { started: string; ended?: string };
  model?: string;
  author?: string;
  commit?: string;
  files?: NewFiles;
  lines?: { added: number; removed: number };
}
interface NewFrontmatter {
  slug: string;
  summary: string;
  status: "open" | "finished";
  author: string;
  date: { created: string; finished?: string };
  commit: string;
  model?: string;
  iterations: NewIteration[];
  files?: NewFiles;
  lines?: { added: number; removed: number };
}
//#endregion

//#region Utilities
const LOG_ROOT = path.join(process.cwd(), "log");
const DEFAULT_AUTHOR = "Unknown <unknown@example.com>";
const DEFAULT_COMMIT = "0000000000000000000000000000000000000000";

function normalizeFileEntry(entry: string | LegacyFileEntry): NewFileEntry {
  if (typeof entry === "string") return { path: entry };
  return { path: entry.path, lines: entry.lines };
}

function normalizeFiles(files?: LegacyFiles): NewFiles | undefined {
  if (!files) return undefined;
  const updated = (files.updated ?? []).map(normalizeFileEntry);
  const created = (files.created ?? []).map(normalizeFileEntry);
  const removed = (files.removed ?? []).map(normalizeFileEntry);
  if (updated.length === 0 && created.length === 0 && removed.length === 0) return undefined;
  return { updated, created, removed };
}

function computeIterationLines(files?: NewFiles): { added: number; removed: number } | undefined {
  if (!files) return undefined;
  let added = 0;
  let removed = 0;
  for (const f of [...files.updated, ...files.created, ...files.removed])
    if (f.lines) {
      added += f.lines.added;
      removed += f.lines.removed;
    }
  if (added === 0 && removed === 0) return undefined;
  return { added, removed };
}

function migrateIteration(legacy: LegacyIteration): NewIteration {
  const files = normalizeFiles(legacy.files);
  const lines = legacy.files ? computeIterationLines(files) : undefined;
  const result: NewIteration = {
    prompt: legacy.prompt,
    date: { started: legacy.date, ended: legacy.finished },
  };
  if (legacy.model) result.model = legacy.model;
  if (legacy.author) result.author = legacy.author;
  if (legacy.commit) result.commit = legacy.commit;
  if (files) result.files = files;
  if (lines) result.lines = lines;
  if (!result.date.ended) delete result.date.ended;
  return result;
}

function migrateFrontmatter(legacy: LegacyFrontmatter): NewFrontmatter {
  const now = new Date().toISOString();
  const created = legacy.created ?? now;
  const finished = legacy.status === "finished" ? legacy.finished ?? now : undefined;
  const iterations = (legacy.iterations ?? []).map(migrateIteration);
  const files = normalizeFiles(legacy.files);
  const result: NewFrontmatter = {
    slug: legacy.slug,
    summary: legacy.summary ?? `Migration from ${legacy.slug}`,
    status: legacy.status ?? "finished",
    author: legacy.author ?? DEFAULT_AUTHOR,
    date: { created, finished },
    commit: legacy.base ?? DEFAULT_COMMIT,
    iterations,
  };
  if (files) result.files = files;
  if (legacy.lines) result.lines = legacy.lines;
  if (!result.date.finished) delete result.date.finished;
  return result;
}

function serializeFileEntry(entry: NewFileEntry): string | Record<string, unknown> {
  if (entry.lines) return { path: entry.path, lines: entry.lines };
  return entry.path;
}

function serializeFiles(files: NewFiles): Record<string, unknown> {
  return {
    updated: files.updated.map(serializeFileEntry),
    created: files.created.map(serializeFileEntry),
    removed: files.removed.map(serializeFileEntry),
  };
}

function serializeIteration(iter: NewIteration): Record<string, unknown> {
  const obj: Record<string, unknown> = { prompt: iter.prompt, date: iter.date };
  if (iter.model) obj.model = iter.model;
  if (iter.author) obj.author = iter.author;
  if (iter.commit) obj.commit = iter.commit;
  if (iter.files) obj.files = serializeFiles(iter.files);
  if (iter.lines) obj.lines = iter.lines;
  return obj;
}

function serializeFrontmatter(fm: NewFrontmatter): Record<string, unknown> {
  const obj: Record<string, unknown> = {
    slug: fm.slug,
    summary: fm.summary,
    status: fm.status,
    author: fm.author,
    date: fm.date,
    commit: fm.commit,
  };
  if (fm.model) obj.model = fm.model;
  obj.iterations = fm.iterations.map(serializeIteration);
  if (fm.files) obj.files = serializeFiles(fm.files);
  if (fm.lines) obj.lines = fm.lines;
  return obj;
}
//#endregion

//#region Migration
function migrateFile(filePath: string): { success: boolean; error?: string } {
  try {
    const raw = fs.readFileSync(filePath, "utf-8");
    const { data: legacy, content } = matter(raw);
    const newFm = migrateFrontmatter(legacy as LegacyFrontmatter);
    const serialized = serializeFrontmatter(newFm);
    const newContent = matter.stringify(content, serialized);
    fs.writeFileSync(filePath, newContent, "utf-8");
    return { success: true };
  } catch (e) {
    return { success: false, error: e instanceof Error ? e.message : String(e) };
  }
}

function collectLogFiles(dir: string): string[] {
  const files: string[] = [];
  if (!fs.existsSync(dir)) return files;
  for (const entry of fs.readdirSync(dir, { withFileTypes: true }))
    if (entry.isDirectory()) files.push(...collectLogFiles(path.join(dir, entry.name)));
    else if (entry.isFile() && entry.name.endsWith(".md")) files.push(path.join(dir, entry.name));
  return files;
}

function migrateAll(): void {
  const logFiles = collectLogFiles(LOG_ROOT);
  let successCount = 0;
  let errorCount = 0;
  const errors: { file: string; error: string }[] = [];
  for (const file of logFiles) {
    const rel = path.relative(LOG_ROOT, file);
    const result = migrateFile(file);
    if (result.success) {
      successCount++;
      console.log(`✓ ${rel}`);
    } else {
      errorCount++;
      errors.push({ file: rel, error: result.error ?? "Unknown error" });
      console.error(`✗ ${rel}: ${result.error}`);
    }
  }
  console.log(`\nMigration complete: ${successCount} succeeded, ${errorCount} failed`);
  if (errors.length > 0) {
    console.log("\nErrors:");
    for (const e of errors) console.log(`  ${e.file}: ${e.error}`);
  }
}
//#endregion

//#region CLI
migrateAll();
//#endregion
