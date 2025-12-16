// SPDX-License-Identifier: AGPL-3.0-or-later
import { execSync } from "child_process";
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, unlinkSync, writeFileSync } from "fs";
import { dirname, join } from "path";
const matter = require("gray-matter");

//#region Types
interface LogLines {
  added: number;
  removed: number;
}

interface FileWithLines {
  path: string;
  lines?: LogLines;
}

interface IterationFiles {
  updated?: FileWithLines[];
  removed?: FileWithLines[];
  created?: FileWithLines[];
}

interface LogIteration {
  prompt: string;
  date: string;
  finished?: string;
  model: string;
  author?: string;
  commit?: string;
  files?: IterationFiles;
}

interface LogFrontmatter {
  slug: string;
  summary: string;
  status?: "open" | "finished";
  author?: string;
  created?: string;
  base?: string;
  finished?: string;
  files?: { updated?: string[]; created?: string[]; removed?: string[] };
  lines?: LogLines;
  iterations?: LogIteration[];
}

interface Log {
  frontmatter: LogFrontmatter;
  content: string;
  path: string;
}

interface LogCreateInput {
  slug: string;
  summary: string;
  content?: string;
  date?: Date;
  author?: string;
}

interface LogUpdateInput {
  summary?: string;
  content?: string;
  model: string;
  prompt: string;
  files: { updated?: string[]; created?: string[]; removed?: string[] };
}

interface LogListOptions {
  year?: number;
  month?: number;
  day?: number;
  slug?: string;
}

interface LogSearchOptions extends LogListOptions {
  query?: string;
  limit?: number;
}

interface LegacyLogInput {
  prompt: string;
  date: string;
  model?: string;
}

interface LegacyLogFiles {
  read?: string[];
  updated?: string[];
  removed?: string[];
  created?: string[];
}

interface LegacyLogFrontmatter {
  slug: string;
  author?: string;
  summary: string;
  model?: string;
  input?: LegacyLogInput[];
  commit?: string;
  files?: LegacyLogFiles;
  lines?: LogLines;
  prompts?: string[];
  date?: string | { created?: string; updated?: string };
  affectedFiles?: string[];
  stats?: { base?: string; addedLines?: number; removedLines?: number; affectedFiles?: string[] };
}
//#endregion

//#region Ticket API Aliases
export const createTicket = createLog;
export const readTicket = readLog;
export const startTicketIteration = updateLog;
export const finishTicketIteration = finishIteration;
export const deleteTicket = deleteLog;
export const listTickets = listLogs;
export const searchTickets = searchLogs;
//#endregion

//#region Configuration
const LOG_ROOT = join(process.cwd(), "log");

function sanitizeForMatter(value: any): any {
  if (value === undefined) return undefined;
  if (value === null) return null;
  if (Array.isArray(value)) return value.map((v) => sanitizeForMatter(v)).filter((v) => v !== undefined);
  if (typeof value === "object") {
    const out: any = {};
    for (const [key, v] of Object.entries(value)) {
      const sanitized = sanitizeForMatter(v);
      if (sanitized !== undefined) out[key] = sanitized;
    }
    return out;
  }
  return value;
}

function getGitConfig(key: string): string {
  try {
    return execSync(`git config --get ${key}`, { encoding: "utf-8" }).trim();
  } catch {
    return "";
  }
}

function getDefaultAuthor(): string {
  const name = getGitConfig("user.name");
  const email = getGitConfig("user.email");
  if (name && email) return `${name} <${email}>`;
  if (name) return name;
  if (email) return email;
  return "Unknown";
}

export enum Model {
  CLAUDE_SONNET_4_5 = "claude-sonnet-4.5",
  CLAUDE_OPUS_4_5 = "claude-opus-4.5",
  GPT_5_1_CODEX_MAX = "gpt-5.1-codex-max",
  GPT_5_2_CODEX = "gpt-5.2-codex",
}

function getGitHead(): string {
  return execSync("git rev-parse HEAD", { encoding: "utf-8" }).trim();
}
//#endregion

//#region Path Utilities
function getLogPath(year: number, month: number, day: number, slug: string): string {
  const monthStr = month.toString().padStart(2, "0");
  const dayStr = day.toString().padStart(2, "0");
  return join(LOG_ROOT, year.toString(), monthStr, dayStr, `${slug}.md`);
}

function parseLogPath(path: string): { year: number; month: number; day: number; slug: string } | null {
  const relativePath = path.replace(LOG_ROOT, "").replace(/\\/g, "/");
  const match = relativePath.match(/^\/(\d{4})\/(\d{2})\/(\d{2})\/(.+)\.md$/);
  if (!match) return null;
  return {
    year: parseInt(match[1]),
    month: parseInt(match[2]),
    day: parseInt(match[3]),
    slug: match[4],
  };
}

function ensureDirectoryExists(filePath: string): void {
  const dir = dirname(filePath);
  if (!existsSync(dir)) {
    mkdirSync(dir, { recursive: true });
  }
}
//#endregion

//#region Frontmatter Utilities
function normalizeLogFrontmatter(frontmatter: any, context?: { slug?: string; createdFromPath?: string }): LogFrontmatter {
  const now = new Date().toISOString();
  const slug = frontmatter?.slug || context?.slug || "UNKNOWN";
  const summary = frontmatter?.summary || "";

  if (Array.isArray(frontmatter?.iterations)) {
    return {
      slug,
      summary,
      status: frontmatter?.status || "open",
      created: frontmatter?.created || context?.createdFromPath || frontmatter.iterations[0]?.date || now,
      author: frontmatter?.author,
      base: frontmatter?.base,
      finished: frontmatter?.finished,
      files: frontmatter?.files,
      lines: frontmatter?.lines,
      iterations: frontmatter.iterations,
    };
  }

  if (frontmatter?.status || frontmatter?.created || frontmatter?.base || frontmatter?.author) {
    return {
      slug,
      summary,
      status: frontmatter?.status || "open",
      created: frontmatter?.created || context?.createdFromPath || now,
      author: frontmatter?.author,
      base: frontmatter?.base,
      finished: frontmatter?.finished,
      files: frontmatter?.files,
      lines: frontmatter?.lines,
      iterations: undefined,
    };
  }

  const author = frontmatter?.author || "Unknown";
  const model = frontmatter?.model || "unknown";
  const commit = frontmatter?.commit || frontmatter?.stats?.base;
  const lines = frontmatter?.lines || (frontmatter?.stats ? { added: frontmatter?.stats?.addedLines || 0, removed: frontmatter?.stats?.removedLines || 0 } : undefined);

  let legacyInput: LegacyLogInput[] = frontmatter?.input || [];
  if (legacyInput.length === 0 && frontmatter?.prompts?.length > 0) {
    const dateCreated = typeof frontmatter?.date === "string" ? frontmatter.date : frontmatter?.date?.created || context?.createdFromPath || now;
    legacyInput = frontmatter.prompts.map((prompt: string, index: number) => ({
      prompt,
      date: index === 0 ? dateCreated : frontmatter?.date?.updated || dateCreated,
      model: model,
    }));
  }

  let legacyFiles: LegacyLogFiles = frontmatter?.files || {};
  if (!legacyFiles.read && !legacyFiles.updated && !legacyFiles.removed && !legacyFiles.created) {
    const affectedFiles = frontmatter?.affectedFiles || frontmatter?.stats?.affectedFiles || [];
    if (affectedFiles.length > 0) {
      legacyFiles = { updated: affectedFiles };
    }
  }

  const iterations: LogIteration[] = legacyInput.map((input, index) => {
    const isLast = index === legacyInput.length - 1;
    const iteration: LogIteration = {
      prompt: input.prompt,
      date: input.date,
      model: input.model || model,
      author: index === 0 ? author : undefined,
    };

    if (isLast && commit) {
      iteration.commit = commit;
    }

    if (isLast && legacyFiles) {
      const iterationFiles: IterationFiles = {};
      if (legacyFiles.updated?.length) {
        const perFileLines = lines && legacyFiles.updated.length > 0 ? {
          added: Math.round(lines.added / legacyFiles.updated.length),
          removed: Math.round(lines.removed / legacyFiles.updated.length),
        } : undefined;
        iterationFiles.updated = legacyFiles.updated.map((path) => ({
          path,
          lines: perFileLines,
        }));
      }
      if (legacyFiles.removed?.length) {
        iterationFiles.removed = legacyFiles.removed.map((path) => ({ path }));
      }
      if (legacyFiles.created?.length) {
        iterationFiles.created = legacyFiles.created.map((path) => ({ path }));
      }
      if (Object.keys(iterationFiles).length > 0) {
        iteration.files = iterationFiles;
      }
    }

    return iteration;
  });

  return {
    slug,
    summary,
    status: "open",
    created: context?.createdFromPath || now,
    iterations: iterations.length > 0 ? iterations : undefined,
  };
}

function getLatestIterationDate(frontmatter: LogFrontmatter): string {
  if (!frontmatter.iterations || frontmatter.iterations.length === 0) {
    return new Date().toISOString();
  }
  return frontmatter.iterations[frontmatter.iterations.length - 1].date;
}

function getFirstIterationDate(frontmatter: LogFrontmatter): string {
  if (!frontmatter.iterations || frontmatter.iterations.length === 0) {
    return new Date().toISOString();
  }
  return frontmatter.iterations[0].date;
}

function getLatestIteration(frontmatter: LogFrontmatter): LogIteration | undefined {
  if (!frontmatter.iterations || frontmatter.iterations.length === 0) {
    return undefined;
  }
  return frontmatter.iterations[frontmatter.iterations.length - 1];
}

function getIsIterationFinished(iteration: LogIteration): boolean {
  return Boolean(iteration.commit || iteration.finished);
}

function requireFiles(files: { updated?: string[]; created?: string[]; removed?: string[] }): void {
  const hasAny = Boolean((files.updated && files.updated.length) || (files.created && files.created.length) || (files.removed && files.removed.length));
  if (!hasAny) throw new Error("Missing required file flags: provide at least one of --file=, --file-created=, --file-removed=");
}

function buildIterationFilesWithoutLines(files: { updated?: string[]; created?: string[]; removed?: string[] }): IterationFiles | undefined {
  const iterationFiles: IterationFiles = {};
  if (files.updated?.length) iterationFiles.updated = files.updated.map((path) => ({ path }));
  if (files.created?.length) iterationFiles.created = files.created.map((path) => ({ path }));
  if (files.removed?.length) iterationFiles.removed = files.removed.map((path) => ({ path }));
  return Object.keys(iterationFiles).length > 0 ? iterationFiles : undefined;
}
//#endregion

//#region CRUD Operations
export function createLog(input: LogCreateInput): Log {
  const date = input.date || new Date();
  const year = date.getFullYear();
  const month = date.getMonth() + 1;
  const day = date.getDate();
  const slug = input.slug.toUpperCase();
  const logPath = getLogPath(year, month, day, slug);
  if (existsSync(logPath)) {
    throw new Error(`Log already exists: ${logPath}`);
  }
  const now = date.toISOString();
  const frontmatter: LogFrontmatter = {
    slug,
    summary: input.summary,
    status: "open",
    author: input.author || getDefaultAuthor(),
    created: now,
    base: getGitHead(),
  };
  const content = input.content || "# Previously\n\n# Plan\n\n# Changes\n";
  const fileContent = matter.stringify(content, sanitizeForMatter(frontmatter));
  ensureDirectoryExists(logPath);
  writeFileSync(logPath, fileContent, "utf-8");
  return { frontmatter, content, path: logPath };
}

export function readLog(year: number, month: number, day: number, slug: string): Log {
  const logPath = getLogPath(year, month, day, slug);
  if (!existsSync(logPath)) {
    throw new Error(`Log not found: ${logPath}`);
  }
  const fileContent = readFileSync(logPath, "utf-8");
  const parsed = matter(fileContent);
  return {
    frontmatter: normalizeLogFrontmatter(parsed.data as LogFrontmatter, { slug }),
    content: parsed.content,
    path: logPath,
  };
}

export function updateLog(year: number, month: number, day: number, slug: string, update: LogUpdateInput): Log {
  const log = readLog(year, month, day, slug);
  const now = new Date().toISOString();
  requireFiles(update.files);
  const iterations = log.frontmatter.iterations || [];
  const latest = iterations.length ? iterations[iterations.length - 1] : undefined;
  if (latest && !getIsIterationFinished(latest)) {
    throw new Error(`Cannot start a new iteration while the latest iteration is unfinished (date: ${latest.date})`);
  }

  const newIteration: LogIteration = {
    prompt: update.prompt,
    date: now,
    model: update.model,
    files: buildIterationFilesWithoutLines(update.files),
  };

  const newIterations = [...iterations, newIteration];

  const newFrontmatter: LogFrontmatter = {
    ...log.frontmatter,
    summary: update.summary ?? log.frontmatter.summary,
    status: log.frontmatter.status || "open",
    author: log.frontmatter.author || getDefaultAuthor(),
    created: log.frontmatter.created || now,
    base: log.frontmatter.base || getGitHead(),
    iterations: newIterations,
  };
  const newContent = update.content ?? log.content;
  const fileContent = matter.stringify(newContent, sanitizeForMatter(newFrontmatter));
  writeFileSync(log.path, fileContent, "utf-8");
  return { frontmatter: newFrontmatter, content: newContent, path: log.path };
}

export function finishIteration(year: number, month: number, day: number, slug: string, files: { updated?: string[]; created?: string[]; removed?: string[] }): Log {
  const log = readLog(year, month, day, slug);
  const iterations = log.frontmatter.iterations || [];
  requireFiles(files);

  if (iterations.length === 0) {
    throw new Error(`No iterations found for ticket: ${slug}`);
  }

  const lastIteration = iterations[iterations.length - 1];

  if (getIsIterationFinished(lastIteration)) {
    throw new Error(`Latest iteration already finished with commit: ${lastIteration.commit || "unknown"}`);
  }

  const commit = getGitHead();
  const finished = new Date().toISOString();

  const iterationFiles: IterationFiles = {};

  if (files.updated?.length) {
    iterationFiles.updated = files.updated.map((path) => {
      const lines = computeGitStatsForFile(path);
      return { path, lines };
    });
  }

  if (files.created?.length) {
    iterationFiles.created = files.created.map((path) => {
      const lines = computeGitStatsForFile(path);
      return { path, lines };
    });
  }

  if (files.removed?.length) {
    iterationFiles.removed = files.removed.map((path) => {
      const lines = computeGitStatsForFile(path);
      return { path, lines };
    });
  }

  const updatedLastIteration: LogIteration = {
    ...lastIteration,
    commit,
    finished,
    files: Object.keys(iterationFiles).length > 0 ? iterationFiles : undefined,
  };

  const newIterations = [...iterations.slice(0, -1), updatedLastIteration];

  const newFrontmatter: LogFrontmatter = {
    ...log.frontmatter,
    status: log.frontmatter.status || "open",
    author: log.frontmatter.author || getDefaultAuthor(),
    created: log.frontmatter.created || getFirstIterationDate(log.frontmatter),
    base: log.frontmatter.base || getGitHead(),
    iterations: newIterations,
  };

  const fileContent = matter.stringify(log.content, sanitizeForMatter(newFrontmatter));
  writeFileSync(log.path, fileContent, "utf-8");
  return { frontmatter: newFrontmatter, content: log.content, path: log.path };
}

export function finishTicket(year: number, month: number, day: number, slug: string): Log {
  const log = readLog(year, month, day, slug);
  const iterations = log.frontmatter.iterations || [];
  if (iterations.length === 0) {
    throw new Error(`Cannot finish ticket without any iterations: ${slug}`);
  }
  const latest = iterations[iterations.length - 1];
  if (!getIsIterationFinished(latest)) {
    throw new Error(`Cannot finish ticket while the latest iteration is unfinished (date: ${latest.date})`);
  }
  const updatedFiles = new Set<string>();
  const createdFiles = new Set<string>();
  const removedFiles = new Set<string>();
  for (const iteration of iterations) {
    if (iteration.files?.updated?.length) for (const file of iteration.files.updated) updatedFiles.add(file.path);
    if (iteration.files?.created?.length) for (const file of iteration.files.created) createdFiles.add(file.path);
    if (iteration.files?.removed?.length) for (const file of iteration.files.removed) removedFiles.add(file.path);
  }
  const affectedFiles = Array.from(new Set([...updatedFiles, ...createdFiles, ...removedFiles])).sort();
  const base = log.frontmatter.base || getGitHead();
  const stats = computeGitStats(base, affectedFiles);
  const finished = new Date().toISOString();
  const newFrontmatter: LogFrontmatter = {
    ...log.frontmatter,
    status: "finished",
    created: log.frontmatter.created || getFirstIterationDate(log.frontmatter),
    base,
    finished,
    files: {
      updated: Array.from(updatedFiles).sort(),
      created: Array.from(createdFiles).sort(),
      removed: Array.from(removedFiles).sort(),
    },
    lines: { added: stats.added, removed: stats.removed },
  };
  const fileContent = matter.stringify(log.content, sanitizeForMatter(newFrontmatter));
  writeFileSync(log.path, fileContent, "utf-8");
  return { frontmatter: newFrontmatter, content: log.content, path: log.path };
}

export function deleteLog(year: number, month: number, day: number, slug: string): void {
  const logPath = getLogPath(year, month, day, slug);
  if (!existsSync(logPath)) {
    throw new Error(`Log not found: ${logPath}`);
  }
  unlinkSync(logPath);
}

export function listLogs(options: LogListOptions = {}): Log[] {
  const logs: Log[] = [];
  function walk(dir: string): void {
    if (!existsSync(dir)) return;
    const entries = readdirSync(dir);
    for (const entry of entries) {
      const fullPath = join(dir, entry);
      const stat = statSync(fullPath);
      if (stat.isDirectory()) {
        walk(fullPath);
      } else if (entry.endsWith(".md")) {
        const parsed = parseLogPath(fullPath);
        if (!parsed) continue;
        if (options.year !== undefined && parsed.year !== options.year) continue;
        if (options.month !== undefined && parsed.month !== options.month) continue;
        if (options.day !== undefined && parsed.day !== options.day) continue;
        if (options.slug !== undefined && parsed.slug !== options.slug) continue;
        try {
          const fileContent = readFileSync(fullPath, "utf-8");
          const matterParsed = matter(fileContent);
          const createdFromPath = new Date(parsed.year, parsed.month - 1, parsed.day).toISOString();
          logs.push({
            frontmatter: normalizeLogFrontmatter(matterParsed.data as LogFrontmatter, { slug: parsed.slug, createdFromPath }),
            content: matterParsed.content,
            path: fullPath,
          });
        } catch (error) {
          console.error(`Failed to parse ticket: ${fullPath}`, error);
        }
      }
    }
  }
  walk(LOG_ROOT);
  return logs.sort((a, b) => new Date(getLatestIterationDate(b.frontmatter)).getTime() - new Date(getLatestIterationDate(a.frontmatter)).getTime());
}

export function searchLogs(options: LogSearchOptions = {}): Log[] {
  const allLogs = listLogs({
    year: options.year,
    month: options.month,
    day: options.day,
    slug: options.slug,
  });

  if (!options.query) {
    return options.limit ? allLogs.slice(0, options.limit) : allLogs;
  }

  const query = options.query.toLowerCase();
  const matchedLogs = allLogs.filter((log) => {
    const slugMatch = log.frontmatter.slug?.toLowerCase().includes(query) ?? false;
    const summaryMatch = log.frontmatter.summary?.toLowerCase().includes(query) ?? false;
    const contentMatch = log.content?.toLowerCase().includes(query) ?? false;
    const authorMatch = (log.frontmatter.author?.toLowerCase().includes(query) ?? false) || (log.frontmatter.iterations?.some((it) => it.author?.toLowerCase().includes(query)) ?? false);
    return slugMatch || summaryMatch || contentMatch || authorMatch;
  });

  return options.limit ? matchedLogs.slice(0, options.limit) : matchedLogs;
}
//#endregion

//#region Migration
export function migrateOldLogs(): void {
  const oldLogPattern = /^(\d{4})-(\d{2})-(\d{2})_(.+)\.md$/;
  if (!existsSync(LOG_ROOT)) return;
  const entries = readdirSync(LOG_ROOT);
  for (const entry of entries) {
    const fullPath = join(LOG_ROOT, entry);
    const stat = statSync(fullPath);
    if (!stat.isFile() || !entry.endsWith(".md")) continue;
    const match = entry.match(oldLogPattern);
    if (!match) continue;
    const [, year, month, day, slug] = match;
    const capitalizedSlug = slug.toUpperCase();
    const content = readFileSync(fullPath, "utf-8");
    const parsed = matter(content);
    if (parsed.data.iterations) {
      console.log(`Skipping already migrated ticket: ${entry}`);
      continue;
    }
    const frontmatter: LogFrontmatter = {
      slug: capitalizedSlug,
      summary: `Migration from ${entry}`,
    };
    const newPath = getLogPath(parseInt(year), parseInt(month), parseInt(day), capitalizedSlug);
    ensureDirectoryExists(newPath);
    const fileContent = matter.stringify(parsed.content || content, sanitizeForMatter(frontmatter));
    writeFileSync(newPath, fileContent, "utf-8");
    unlinkSync(fullPath);
    console.log(`Migrated: ${entry} -> ${newPath}`);
  }
}
//#endregion

//#region Git Stats
function getGitStatusPorcelain(): string {
  return execSync("git status --porcelain", { encoding: "utf-8" });
}

function getGitDiffNameOnly(base: string): string {
  return execSync(`git diff --name-only ${base}`, { encoding: "utf-8" });
}

function parseGitPath(rawPath: string): string {
  const trimmed = rawPath.trim();
  if (!trimmed) return "";
  if (trimmed.startsWith("\"") && trimmed.endsWith("\"")) {
    try {
      return JSON.parse(trimmed);
    } catch {
      return trimmed.slice(1, -1);
    }
  }
  return trimmed;
}

function getChangedFilesSince(base: string): string[] {
  const files = new Set<string>();
  for (const line of getGitDiffNameOnly(base).split(/\r?\n/)) {
    const path = line.trim();
    if (!path) continue;
    files.add(path);
  }
  for (const line of getGitStatusPorcelain().split(/\r?\n/)) {
    if (!line.trim()) continue;
    const status = line.slice(0, 2);
    const rawPath = line.slice(3);
    if (!rawPath) continue;
    if (status.includes("R") || status.includes("C")) {
      const parts = rawPath.split("->").map((part) => part.trim());
      if (parts.length === 2) {
        const renamedPath = parseGitPath(parts[1]);
        if (renamedPath) files.add(renamedPath);
      }
      continue;
    }
    const path = parseGitPath(rawPath);
    if (!path) continue;
    files.add(path);
  }
  return Array.from(files).sort();
}

function quoteGitPath(path: string): string {
  return "\"" + path.replaceAll("\"", "\\\"") + "\"";
}

function execGitDiffNoIndexNumstat(nullPath: string, filePath: string): string {
  try {
    return execSync(`git diff --no-index --numstat -- ${nullPath} ${quoteGitPath(filePath)}`, { encoding: "utf-8" });
  } catch (error: any) {
    if (error && error.stdout) {
      return Buffer.isBuffer(error.stdout) ? error.stdout.toString("utf-8") : String(error.stdout);
    }
    return "";
  }
}

function getUntrackedFiles(): string[] {
  const files: string[] = [];
  for (const line of getGitStatusPorcelain().split(/\r?\n/)) {
    if (!line.startsWith("?? ")) continue;
    const path = parseGitPath(line.slice(3));
    if (!path) continue;
    files.push(path);
  }
  return files;
}

function parseGitNumstatOutput(output: string): { added: number; removed: number } {
  let addedTotal = 0;
  let removedTotal = 0;
  for (const line of output.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed) continue;
    const parts = trimmed.split("\t");
    if (parts.length < 2) continue;
    const added = parts[0] === "-" ? 0 : parseInt(parts[0]);
    const removed = parts[1] === "-" ? 0 : parseInt(parts[1]);
    if (!Number.isNaN(added)) addedTotal += added;
    if (!Number.isNaN(removed)) removedTotal += removed;
  }
  return { added: addedTotal, removed: removedTotal };
}

function computeGitStatsForFile(filePath: string): LogLines {
  const untracked = new Set(getUntrackedFiles());
  const nullPath = process.platform === "win32" ? "NUL" : "/dev/null";

  if (untracked.has(filePath)) {
    const output = execGitDiffNoIndexNumstat(nullPath, filePath);
    return parseGitNumstatOutput(output);
  }

  try {
    const output = execSync(`git diff --numstat HEAD -- ${quoteGitPath(filePath)}`, { encoding: "utf-8" });
    return parseGitNumstatOutput(output);
  } catch {
    return { added: 0, removed: 0 };
  }
}

function computeGitStats(base: string, affectedFiles: string[]): { added: number; removed: number; affectedFiles: string[] } {
  const untracked = new Set(getUntrackedFiles());
  const trackedFiles = affectedFiles.filter((file) => !untracked.has(file));
  const trackedOutput = trackedFiles.length ? execSync(`git diff --numstat ${base} -- ${trackedFiles.map(quoteGitPath).join(" ")}`, { encoding: "utf-8" }) : "";
  const tracked = parseGitNumstatOutput(trackedOutput);
  const nullPath = process.platform === "win32" ? "NUL" : "/dev/null";
  let added = tracked.added;
  let removed = tracked.removed;
  for (const file of affectedFiles) {
    if (!untracked.has(file)) continue;
    const output = execGitDiffNoIndexNumstat(nullPath, file);
    const parsed = parseGitNumstatOutput(output);
    added += parsed.added;
    removed += parsed.removed;
  }
  return { added, removed, affectedFiles };
}
//#endregion

//#region CLI
function printUsage(): void {
  console.log(`
Usage: tsx scripts/log.ts <command> [options]

Commands:
  ticket create <slug> <summary>       Create a ticket (no iterations)
                                       Optional: --date=ISO
  ticket iteration start <slug>        Start a new iteration on a ticket
                                       Required: --model=MODEL --prompt="..." and at least one file flag
                                       Optional: --summary="..."
  ticket iteration finish <slug>       Finish the latest iteration (computes git line stats per file)
                                       Required: at least one file flag
  ticket finish <slug>                 Finish the ticket (requires latest iteration finished)
  models                               List available model enum values
  ticket read <year> <month> <day> <slug>     Read a ticket with all iterations
  ticket delete <year> <month> <day> <slug>   Delete a ticket
  ticket list [year] [month] [day]            List tickets (optionally filtered)
  ticket search [query] [--limit=N]           Search tickets by query (searches slug, summary, content, author)
                                             Optional: --year=YYYY --month=MM --day=DD --limit=N
  migrate                              Migrate all logs to the latest structure

Ticket Format:
  status: "open" | "finished"
  iterations: [{prompt, date, finished, model, commit, files}]  Array of iterations
  files: {updated: string[], created: string[], removed: string[]}
  lines: {added: number, removed: number}

Workflow:
  1. Create a ticket when starting work: ticket create <slug> <summary>
  2. Start the first iteration: ticket iteration start <slug> --model=MODEL --prompt="..." --file=...
  3. Finish the iteration when stopping: ticket iteration finish <slug> --file=...
  4. Finish the ticket: ticket finish <slug>

Examples:
  tsx scripts/log.ts ticket create my-task "Implement new feature"
  tsx scripts/log.ts ticket iteration start MY-TASK --model=${Model.CLAUDE_OPUS_4_5} --prompt="User request..." --file=scripts/log.ts
  tsx scripts/log.ts ticket iteration finish MY-TASK --file=scripts/log.ts --file=README.md
  tsx scripts/log.ts ticket finish MY-TASK
  tsx scripts/log.ts ticket read 2025 11 24 MY-TASK
  tsx scripts/log.ts ticket list 2025 11
  tsx scripts/log.ts ticket search "drag drop"
  tsx scripts/log.ts ticket search "test" --limit=5
  tsx scripts/log.ts migrate

Legacy Aliases:
  create, update, finish, read, delete, list, search
`);
}

function parseFlag(args: string[], name: string): string | undefined {
  const prefix = `--${name}=`;
  const match = args.find((arg) => arg.startsWith(prefix));
  if (!match) return undefined;
  return match.slice(prefix.length);
}

function parseFlags(args: string[], name: string): string[] {
  const prefix = `--${name}=`;
  const values: string[] = [];
  for (const arg of args) {
    if (!arg.startsWith(prefix)) continue;
    values.push(arg.slice(prefix.length));
  }
  return values;
}

function requireFlag(args: string[], name: string): string {
  const value = parseFlag(args, name);
  if (!value) throw new Error(`Missing required flag: --${name}=`);
  return value;
}

function validateModel(model: string): string {
  const values = Object.values(Model);
  if (!values.includes(model as any)) {
    throw new Error(`Unknown model: ${model}. Add it to the Model enum in scripts/log.ts.`);
  }
  return model;
}

function getLatestLogBySlug(slug: string): { year: number; month: number; day: number; slug: string } {
  const normalizedSlug = slug.toUpperCase();
  const logs = listLogs({ slug: normalizedSlug });
  const latest = logs[0];
  if (!latest) throw new Error(`No ticket found for slug: ${normalizedSlug}`);
  const parsed = parseLogPath(latest.path);
  if (!parsed) throw new Error(`Failed to parse ticket path: ${latest.path}`);
  return parsed;
}

function getIterationAuthor(frontmatter: LogFrontmatter): string | undefined {
  if (frontmatter.author) return frontmatter.author;
  if (!frontmatter.iterations || frontmatter.iterations.length === 0) return undefined;
  for (const iteration of frontmatter.iterations) {
    if (iteration.author) return iteration.author;
  }
  return undefined;
}

function getIterationModel(frontmatter: LogFrontmatter): string | undefined {
  if (!frontmatter.iterations || frontmatter.iterations.length === 0) return undefined;
  return frontmatter.iterations[frontmatter.iterations.length - 1].model;
}

function getTotalLines(frontmatter: LogFrontmatter): LogLines {
  let added = 0;
  let removed = 0;
  if (!frontmatter.iterations) return { added, removed };
  for (const iteration of frontmatter.iterations) {
    if (!iteration.files) continue;
    const allFiles = [
      ...(iteration.files.updated || []),
      ...(iteration.files.created || []),
      ...(iteration.files.removed || []),
    ];
    for (const file of allFiles) {
      if (file.lines) {
        added += file.lines.added;
        removed += file.lines.removed;
      }
    }
  }
  return { added, removed };
}

function migrateLogFileFrontmatter(path: string): boolean {
  const fileContent = readFileSync(path, "utf-8");
  const parsed = matter(fileContent);
  const parsedPath = parseLogPath(path);
  const createdFromPath = parsedPath ? new Date(parsedPath.year, parsedPath.month - 1, parsedPath.day).toISOString() : undefined;
  const slug = parsedPath ? parsedPath.slug : path.split(/[\\/]/).pop()?.replace(/\.md$/, "").toUpperCase() || "UNKNOWN";
  const normalized = normalizeLogFrontmatter(parsed.data, { slug, createdFromPath });
  const dataJson = JSON.stringify(parsed.data);
  const normalizedJson = JSON.stringify(normalized);
  if (dataJson === normalizedJson) return false;
  const output = matter.stringify(parsed.content, sanitizeForMatter(normalized));
  writeFileSync(path, output, "utf-8");
  return true;
}

function migrateAllLogFrontmatter(): { scanned: number; updated: number } {
  let scanned = 0;
  let updated = 0;
  function walk(dir: string): void {
    if (!existsSync(dir)) return;
    for (const entry of readdirSync(dir)) {
      const fullPath = join(dir, entry);
      const stat = statSync(fullPath);
      if (stat.isDirectory()) {
        walk(fullPath);
        continue;
      }
      if (!entry.endsWith(".md")) continue;
      scanned += 1;
      try {
        if (migrateLogFileFrontmatter(fullPath)) updated += 1;
      } catch (error) {
        console.error(`Failed to migrate ticket: ${fullPath}`, error);
      }
    }
  }
  walk(LOG_ROOT);
  return { scanned, updated };
}

if (require.main === module) {
  const args = process.argv.slice(2);
  const command = args[0];
  try {
    switch (command) {
      case "ticket": {
        const [, sub, ...rest] = args;
        if (!sub) {
          console.error("Error: Missing ticket command");
          printUsage();
          process.exit(1);
        }
        if (sub === "create") {
          const [slug, summary, ...flags] = rest;
          if (!slug || !summary) {
            console.error("Error: Missing slug or summary");
            printUsage();
            process.exit(1);
          }
          const dateArg = flags.find((arg) => arg.startsWith("--date="));
          const date = dateArg ? new Date(dateArg.split("=")[1]) : undefined;
          const log = createLog({ slug, summary, date });
          console.log(`Created ticket: ${log.path}`);
          console.log(`Summary: ${log.frontmatter.summary}`);
          break;
        }
        if (sub === "iteration") {
          const [iterationCommand, slugArg, ...flags] = rest;
          if (iterationCommand === "start") {
            if (!slugArg) {
              console.error("Error: Missing slug");
              printUsage();
              process.exit(1);
            }
            const model = validateModel(requireFlag(flags, "model"));
            const prompt = requireFlag(flags, "prompt");
            const summary = parseFlag(flags, "summary");
            const latest = getLatestLogBySlug(slugArg);
            const filesUpdated = parseFlags(flags, "file");
            const filesCreated = parseFlags(flags, "file-created");
            const filesRemoved = parseFlags(flags, "file-removed");
            const files = {
              updated: filesUpdated.length > 0 ? filesUpdated : undefined,
              created: filesCreated.length > 0 ? filesCreated : undefined,
              removed: filesRemoved.length > 0 ? filesRemoved : undefined,
            };
            const update: LogUpdateInput = { model, prompt, files };
            if (summary) update.summary = summary;
            const updated = updateLog(latest.year, latest.month, latest.day, latest.slug, update);
            console.log(`Updated ticket: ${updated.path}`);
            console.log(`Added iteration ${updated.frontmatter.iterations?.length || 0}`);
            break;
          }
          if (iterationCommand === "finish") {
            if (!slugArg) {
              console.error("Error: Missing slug");
              printUsage();
              process.exit(1);
            }
            const latest = getLatestLogBySlug(slugArg);
            const filesUpdated = parseFlags(flags, "file");
            const filesCreated = parseFlags(flags, "file-created");
            const filesRemoved = parseFlags(flags, "file-removed");
            const files = {
              updated: filesUpdated.length > 0 ? filesUpdated : undefined,
              created: filesCreated.length > 0 ? filesCreated : undefined,
              removed: filesRemoved.length > 0 ? filesRemoved : undefined,
            };
            const log = finishIteration(latest.year, latest.month, latest.day, latest.slug, files);
            const lastIteration = getLatestIteration(log.frontmatter);
            console.log(`Finished iteration for ticket: ${log.path}`);
            console.log(`Commit: ${lastIteration?.commit || "none"}`);
            break;
          }
          printUsage();
          process.exit(1);
          break;
        }
        if (sub === "finish") {
          const [slugArg] = rest;
          if (!slugArg) {
            console.error("Error: Missing slug");
            printUsage();
            process.exit(1);
          }
          const latest = getLatestLogBySlug(slugArg);
          const log = finishTicket(latest.year, latest.month, latest.day, latest.slug);
          console.log(`Finished ticket: ${log.path}`);
          break;
        }
        if (sub === "read") {
          const [year, month, day, slug] = rest;
          if (!year || !month || !day || !slug) {
            console.error("Error: Missing year, month, day, or slug");
            printUsage();
            process.exit(1);
          }
          const log = readLog(parseInt(year), parseInt(month), parseInt(day), slug);
          console.log(`\nPath: ${log.path}`);
          console.log(`Summary: ${log.frontmatter.summary}`);
          console.log(`Status: ${log.frontmatter.status || "open"}`);
          const author = getIterationAuthor(log.frontmatter);
          if (author) console.log(`Author: ${author}`);
          if (log.frontmatter.files) {
            console.log(`Files: ${((log.frontmatter.files.updated || []).length + (log.frontmatter.files.created || []).length + (log.frontmatter.files.removed || []).length)}`);
          }
          if (log.frontmatter.lines) {
            console.log(`Lines: +${log.frontmatter.lines.added} -${log.frontmatter.lines.removed}`);
          }
          const iterations = log.frontmatter.iterations || [];
          console.log(`Iterations: ${iterations.length}`);
          if (iterations.length) {
            console.log(`  First: ${getFirstIterationDate(log.frontmatter)}`);
            console.log(`  Latest: ${getLatestIterationDate(log.frontmatter)}`);
          }
          for (let i = 0; i < iterations.length; i++) {
            const it = iterations[i];
            console.log(`\n  [${i + 1}] ${it.date}`);
            console.log(`      Model: ${it.model}`);
            if (it.author) console.log(`      Author: ${it.author}`);
            if (it.finished) console.log(`      Finished: ${it.finished}`);
            if (it.commit) console.log(`      Commit: ${it.commit.substring(0, 8)}`);
            if (it.files) {
              if (it.files.updated?.length) {
                console.log(`      Updated: ${it.files.updated.length} files (+${it.files.updated.reduce((s, f) => s + (f.lines?.added || 0), 0)} -${it.files.updated.reduce((s, f) => s + (f.lines?.removed || 0), 0)})`);
              }
              if (it.files.created?.length) console.log(`      Created: ${it.files.created.length} files`);
              if (it.files.removed?.length) console.log(`      Removed: ${it.files.removed.length} files`);
            }
            console.log(`      Prompt: ${it.prompt.substring(0, 80)}${it.prompt.length > 80 ? "..." : ""}`);
          }
          const totalLines = getTotalLines(log.frontmatter);
          if (totalLines.added || totalLines.removed) {
            console.log(`\nTotal Lines: +${totalLines.added} -${totalLines.removed}`);
          }
          console.log(`\nContent:\n${log.content}`);
          break;
        }
        if (sub === "delete") {
          const [year, month, day, slug] = rest;
          if (!year || !month || !day || !slug) {
            console.error("Error: Missing year, month, day, or slug");
            printUsage();
            process.exit(1);
          }
          deleteLog(parseInt(year), parseInt(month), parseInt(day), slug);
          console.log(`Deleted ticket: ${year}/${month}/${day}/${slug}`);
          break;
        }
        if (sub === "list") {
          const [year, month, day] = rest;
          const options: LogListOptions = {};
          if (year) options.year = parseInt(year);
          if (month) options.month = parseInt(month);
          if (day) options.day = parseInt(day);
          const logs = listLogs(options);
          console.log(`\nFound ${logs.length} ticket(s):\n`);
          for (const log of logs) {
            const parsed = parseLogPath(log.path);
            if (parsed) {
              console.log(`${parsed.year}-${String(parsed.month).padStart(2, "0")}-${String(parsed.day).padStart(2, "0")} ${parsed.slug}`);
              console.log(`  Summary: ${log.frontmatter.summary}`);
              console.log(`  Status: ${log.frontmatter.status || "open"}`);
              const author = getIterationAuthor(log.frontmatter);
              if (author) console.log(`  Author: ${author}`);
              const model = getIterationModel(log.frontmatter);
              if (model) console.log(`  Model: ${model}`);
              console.log(`  Iterations: ${log.frontmatter.iterations?.length || 0}`);
              console.log();
            }
          }
          break;
        }
        if (sub === "search") {
          const options: LogSearchOptions = {};
          let query = "";
          for (const arg of rest) {
            if (arg.startsWith("--year=")) {
              options.year = parseInt(arg.split("=")[1]);
            } else if (arg.startsWith("--month=")) {
              options.month = parseInt(arg.split("=")[1]);
            } else if (arg.startsWith("--day=")) {
              options.day = parseInt(arg.split("=")[1]);
            } else if (arg.startsWith("--limit=")) {
              options.limit = parseInt(arg.split("=")[1]);
            } else if (!arg.startsWith("--")) {
              query = arg;
            }
          }
          if (query) {
            options.query = query;
          }
          const logs = searchLogs(options);
          const limitText = options.limit ? ` (showing first ${options.limit})` : "";
          console.log(`\nFound ${logs.length} ticket(s)${limitText}:\n`);
          for (const log of logs) {
            const parsed = parseLogPath(log.path);
            if (parsed) {
              console.log(`${parsed.year}-${String(parsed.month).padStart(2, "0")}-${String(parsed.day).padStart(2, "0")} ${parsed.slug}`);
              console.log(`  Summary: ${log.frontmatter.summary}`);
              console.log(`  Status: ${log.frontmatter.status || "open"}`);
              const author = getIterationAuthor(log.frontmatter);
              if (author) console.log(`  Author: ${author}`);
              const model = getIterationModel(log.frontmatter);
              if (model) console.log(`  Model: ${model}`);
              if (options.query) {
                const contentPreview = log.content.substring(0, 200).replace(/\n/g, " ");
                console.log(`  Preview: ${contentPreview}...`);
              }
              console.log();
            }
          }
          break;
        }
        printUsage();
        process.exit(1);
        break;
      }
      case "create": {
        const [, slug, summary, ...rest] = args;
        if (!slug || !summary) {
          console.error("Error: Missing slug or summary");
          printUsage();
          process.exit(1);
        }
        const dateArg = rest.find((arg) => arg.startsWith("--date="));
        const date = dateArg ? new Date(dateArg.split("=")[1]) : undefined;
        const log = createLog({ slug, summary, date });
        console.log(`Created ticket: ${log.path}`);
        console.log(`Summary: ${log.frontmatter.summary}`);
        break;
      }
      case "models": {
        console.log(Object.values(Model).join("\n"));
        break;
      }
      case "update": {
        const [, slugArg, ...rest] = args;
        if (!slugArg) {
          console.error("Error: Missing slug");
          printUsage();
          process.exit(1);
        }
        const model = validateModel(requireFlag(rest, "model"));
        const prompt = requireFlag(rest, "prompt");
        const summary = parseFlag(rest, "summary");
        const latest = getLatestLogBySlug(slugArg);
        const filesUpdated = parseFlags(rest, "file");
        const filesCreated = parseFlags(rest, "file-created");
        const filesRemoved = parseFlags(rest, "file-removed");
        const files = {
          updated: filesUpdated.length > 0 ? filesUpdated : undefined,
          created: filesCreated.length > 0 ? filesCreated : undefined,
          removed: filesRemoved.length > 0 ? filesRemoved : undefined,
        };
        const update: LogUpdateInput = { model, prompt, files };
        if (summary) update.summary = summary;
        const updated = updateLog(latest.year, latest.month, latest.day, latest.slug, update);
        console.log(`Updated ticket: ${updated.path}`);
        console.log(`Added iteration ${updated.frontmatter.iterations?.length || 0}`);
        break;
      }
      case "finish": {
        const [, slug, ...rest] = args;
        if (!slug) {
          console.error("Error: Missing slug");
          printUsage();
          process.exit(1);
        }
        const latest = getLatestLogBySlug(slug);
        const filesUpdated = parseFlags(rest, "file");
        const filesCreated = parseFlags(rest, "file-created");
        const filesRemoved = parseFlags(rest, "file-removed");
        const files = {
          updated: filesUpdated.length > 0 ? filesUpdated : undefined,
          created: filesCreated.length > 0 ? filesCreated : undefined,
          removed: filesRemoved.length > 0 ? filesRemoved : undefined,
        };
        const log = finishIteration(latest.year, latest.month, latest.day, latest.slug, files);
        const lastIteration = getLatestIteration(log.frontmatter);
        console.log(`Finished iteration for ticket: ${log.path}`);
        console.log(`Commit: ${lastIteration?.commit || "none"}`);
        if (lastIteration?.files) {
          const totalFiles = (lastIteration.files.updated?.length || 0) +
            (lastIteration.files.created?.length || 0) +
            (lastIteration.files.removed?.length || 0);
          console.log(`Files: ${totalFiles}`);
        }
        break;
      }
      case "read": {
        const [, year, month, day, slug] = args;
        if (!year || !month || !day || !slug) {
          console.error("Error: Missing year, month, day, or slug");
          printUsage();
          process.exit(1);
        }
        const log = readLog(parseInt(year), parseInt(month), parseInt(day), slug);
        console.log(`\nPath: ${log.path}`);
        console.log(`Summary: ${log.frontmatter.summary}`);
        console.log(`Status: ${log.frontmatter.status || "open"}`);
        const author = getIterationAuthor(log.frontmatter);
        if (author) console.log(`Author: ${author}`);
        if (log.frontmatter.files) {
          console.log(`Files: ${((log.frontmatter.files.updated || []).length + (log.frontmatter.files.created || []).length + (log.frontmatter.files.removed || []).length)}`);
        }
        if (log.frontmatter.lines) {
          console.log(`Lines: +${log.frontmatter.lines.added} -${log.frontmatter.lines.removed}`);
        }
        const iterations = log.frontmatter.iterations || [];
        console.log(`Iterations: ${iterations.length}`);
        if (iterations.length) {
          console.log(`  First: ${getFirstIterationDate(log.frontmatter)}`);
          console.log(`  Latest: ${getLatestIterationDate(log.frontmatter)}`);
        }
        for (let i = 0; i < iterations.length; i++) {
          const it = iterations[i];
          console.log(`\n  [${i + 1}] ${it.date}`);
          console.log(`      Model: ${it.model}`);
          if (it.author) console.log(`      Author: ${it.author}`);
          if (it.finished) console.log(`      Finished: ${it.finished}`);
          if (it.commit) console.log(`      Commit: ${it.commit.substring(0, 8)}`);
          if (it.files) {
            if (it.files.updated?.length) {
              console.log(`      Updated: ${it.files.updated.length} files (+${it.files.updated.reduce((s, f) => s + (f.lines?.added || 0), 0)} -${it.files.updated.reduce((s, f) => s + (f.lines?.removed || 0), 0)})`);
            }
            if (it.files.created?.length) console.log(`      Created: ${it.files.created.length} files`);
            if (it.files.removed?.length) console.log(`      Removed: ${it.files.removed.length} files`);
          }
          console.log(`      Prompt: ${it.prompt.substring(0, 80)}${it.prompt.length > 80 ? "..." : ""}`);
        }
        const totalLines = getTotalLines(log.frontmatter);
        if (totalLines.added || totalLines.removed) {
          console.log(`\nTotal Lines: +${totalLines.added} -${totalLines.removed}`);
        }
        console.log(`\nContent:\n${log.content}`);
        break;
      }
      case "delete": {
        const [, year, month, day, slug] = args;
        if (!year || !month || !day || !slug) {
          console.error("Error: Missing year, month, day, or slug");
          printUsage();
          process.exit(1);
        }
        deleteLog(parseInt(year), parseInt(month), parseInt(day), slug);
        console.log(`Deleted ticket: ${year}/${month}/${day}/${slug}`);
        break;
      }
      case "list": {
        const [, year, month, day] = args;
        const options: LogListOptions = {};
        if (year) options.year = parseInt(year);
        if (month) options.month = parseInt(month);
        if (day) options.day = parseInt(day);
        const logs = listLogs(options);
        console.log(`\nFound ${logs.length} ticket(s):\n`);
        for (const log of logs) {
          const parsed = parseLogPath(log.path);
          if (parsed) {
            console.log(`${parsed.year}-${String(parsed.month).padStart(2, "0")}-${String(parsed.day).padStart(2, "0")} ${parsed.slug}`);
            console.log(`  Summary: ${log.frontmatter.summary}`);
            console.log(`  Status: ${log.frontmatter.status || "open"}`);
            const author = getIterationAuthor(log.frontmatter);
            if (author) console.log(`  Author: ${author}`);
            const model = getIterationModel(log.frontmatter);
            if (model) console.log(`  Model: ${model}`);
            console.log(`  Iterations: ${log.frontmatter.iterations?.length || 0}`);
            console.log();
          }
        }
        break;
      }
      case "search": {
        const [, ...rest] = args;
        const options: LogSearchOptions = {};
        let query = "";

        for (const arg of rest) {
          if (arg.startsWith("--year=")) {
            options.year = parseInt(arg.split("=")[1]);
          } else if (arg.startsWith("--month=")) {
            options.month = parseInt(arg.split("=")[1]);
          } else if (arg.startsWith("--day=")) {
            options.day = parseInt(arg.split("=")[1]);
          } else if (arg.startsWith("--limit=")) {
            options.limit = parseInt(arg.split("=")[1]);
          } else if (!arg.startsWith("--")) {
            query = arg;
          }
        }

        if (query) {
          options.query = query;
        }

        const logs = searchLogs(options);
        const limitText = options.limit ? ` (showing first ${options.limit})` : "";
        console.log(`\nFound ${logs.length} ticket(s)${limitText}:\n`);
        for (const log of logs) {
          const parsed = parseLogPath(log.path);
          if (parsed) {
            console.log(`${parsed.year}-${String(parsed.month).padStart(2, "0")}-${String(parsed.day).padStart(2, "0")} ${parsed.slug}`);
            console.log(`  Summary: ${log.frontmatter.summary}`);
            console.log(`  Status: ${log.frontmatter.status || "open"}`);
            const author = getIterationAuthor(log.frontmatter);
            if (author) console.log(`  Author: ${author}`);
            const model = getIterationModel(log.frontmatter);
            if (model) console.log(`  Model: ${model}`);
            if (options.query) {
              const contentPreview = log.content.substring(0, 200).replace(/\n/g, " ");
              console.log(`  Preview: ${contentPreview}...`);
            }
            console.log();
          }
        }
        break;
      }
      case "migrate": {
        migrateOldLogs();
        const migrated = migrateAllLogFrontmatter();
        console.log(`Migration complete (scanned ${migrated.scanned}, updated ${migrated.updated})`);
        break;
      }
      default:
        printUsage();
        process.exit(1);
    }
  } catch (error) {
    console.error("Error:", error instanceof Error ? error.message : error);
    process.exit(1);
  }
}
//#endregion
