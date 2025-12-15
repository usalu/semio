import { execSync } from "child_process";
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, unlinkSync, writeFileSync } from "fs";
import { dirname, join } from "path";
const matter = require("gray-matter");

//#region Types
interface LogInput {
  prompt: string;
  date: string;
}

interface LogLines {
  added: number;
  removed: number;
}

interface LogFiles {
  read?: string[];
  updated?: string[];
  removed?: string[];
  created?: string[];
}

interface LogFrontmatter {
  slug: string;
  author: string;
  summary: string;
  model: string;
  input?: LogInput[];
  commit?: string;
  files?: LogFiles;
  lines?: LogLines;
}

interface Log {
  frontmatter: LogFrontmatter;
  content: string;
  path: string;
}

interface LogCreateInput {
  slug: string;
  summary: string;
  prompt: string;
  model: string;
  content?: string;
  date?: Date;
  author?: string;
  files?: LogFiles;
}

interface LogUpdateInput {
  summary?: string;
  content?: string;
  model?: string;
  prompt?: string;
  commit?: string;
  files?: LogFiles;
  lines?: LogLines;
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
  const author = frontmatter?.author || "Unknown";
  const summary = frontmatter?.summary || "";
  const model = frontmatter?.model || "unknown";
  const commit = frontmatter?.commit || frontmatter?.stats?.base || "unknown";
  const lines = frontmatter?.lines || (frontmatter?.stats ? { added: frontmatter?.stats?.addedLines || 0, removed: frontmatter?.stats?.removedLines || 0 } : { added: 0, removed: 0 });

  // Handle input migration from legacy prompts format
  let input: LogInput[] = frontmatter?.input || [];
  if (input.length === 0 && frontmatter?.prompts?.length > 0) {
    // Migrate from legacy prompts array to input array
    const dateCreated = typeof frontmatter?.date === "string" ? frontmatter.date : frontmatter?.date?.created || context?.createdFromPath || now;
    input = frontmatter.prompts.map((prompt: string, index: number) => ({
      prompt,
      date: index === 0 ? dateCreated : frontmatter?.date?.updated || dateCreated,
    }));
  }

  // Handle files migration from legacy affectedFiles format
  let files: LogFiles = frontmatter?.files || {};
  if (!files.read && !files.updated && !files.removed && !files.created) {
    const legacyFiles = frontmatter?.affectedFiles || frontmatter?.stats?.affectedFiles || [];
    if (legacyFiles.length > 0) {
      // Migrate legacy affectedFiles to files.updated
      files = { updated: legacyFiles };
    }
  }

  const migrated: LogFrontmatter = {
    slug,
    author,
    summary,
    model,
    input,
    commit,
    files,
    lines,
  };
  return migrated;
}

function getLatestInputDate(frontmatter: LogFrontmatter): string {
  if (!frontmatter.input || frontmatter.input.length === 0) {
    return new Date().toISOString();
  }
  return frontmatter.input[frontmatter.input.length - 1].date;
}

function getFirstInputDate(frontmatter: LogFrontmatter): string {
  if (!frontmatter.input || frontmatter.input.length === 0) {
    return new Date().toISOString();
  }
  return frontmatter.input[0].date;
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
    author: input.author || getDefaultAuthor(),
    summary: input.summary,
    model: input.model,
    input: [{ prompt: input.prompt, date: now }],
    commit: getGitHead(),
    files: input.files || {},
    lines: { added: 0, removed: 0 },
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

  // Add new input if prompt is provided
  const newInput = update.prompt
    ? [...(log.frontmatter.input || []), { prompt: update.prompt, date: now }]
    : log.frontmatter.input;

  const existingFiles = log.frontmatter.files || {};
  const newFiles = update.files
    ? {
        read: update.files.read ?? existingFiles.read,
        updated: update.files.updated ?? existingFiles.updated,
        removed: update.files.removed ?? existingFiles.removed,
        created: update.files.created ?? existingFiles.created,
      }
    : existingFiles;

  const newFrontmatter: LogFrontmatter = {
    ...log.frontmatter,
    summary: update.summary ?? log.frontmatter.summary,
    model: update.model ?? log.frontmatter.model,
    input: newInput,
    commit: update.commit ?? log.frontmatter.commit,
    files: newFiles,
    lines: update.lines ?? log.frontmatter.lines,
  };
  const newContent = update.content ?? log.content;
  const fileContent = matter.stringify(newContent, sanitizeForMatter(newFrontmatter));
  writeFileSync(log.path, fileContent, "utf-8");
  return { frontmatter: newFrontmatter, content: newContent, path: log.path };
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
          console.error(`Failed to parse log: ${fullPath}`, error);
        }
      }
    }
  }
  walk(LOG_ROOT);
  return logs.sort((a, b) => new Date(getLatestInputDate(b.frontmatter)).getTime() - new Date(getLatestInputDate(a.frontmatter)).getTime());
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
    const authorMatch = log.frontmatter.author?.toLowerCase().includes(query) ?? false;
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
    if (parsed.data.input) {
      console.log(`Skipping already migrated log: ${entry}`);
      continue;
    }
    const frontmatter: LogFrontmatter = {
      slug: capitalizedSlug,
      author: getDefaultAuthor(),
      summary: `Migration from ${entry}`,
      model: "unknown",
      input: [],
      files: {},
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
  return `"${path.replace(/"/g, "\\\"")}"`;  // eslint-disable-line no-useless-escape
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
  create <slug> <summary>              Create a new log (slug will be capitalized)
                                       Required: --model=MODEL --prompt="..."
                                       Optional: --file=PATH (can repeat for multiple files)
  update <slug>                        Update an existing log (same API as create)
                                       Optional: --prompt="..." --file=PATH --file-read=PATH
                                                 --file-created=PATH --file-removed=PATH
                                                 --summary="..." --model=MODEL
  prompt <slug> <prompt>               Append a user prompt to the latest log for the slug
  files <slug> [paths...]              Update tracked files for the latest log for the slug
                                       Optional: --reset (replace list)
                                                 --category=read|updated|removed|created (default: updated)
  stats <slug> [--detect]              Update git stats for the latest log for the slug (uses tracked files)
  models                               List available model enum values
  read <year> <month> <day> <slug>     Read a log
  delete <year> <month> <day> <slug>   Delete a log
  list [year] [month] [day]            List logs (optionally filtered)
  search [query] [--limit=N]           Search logs by query (searches slug, summary, content, author)
                                       Optional: --year=YYYY --month=MM --day=DD --limit=N
  migrate                              Migrate all logs to the latest structure

Log Format:
  input: [{prompt, date}]              Array of inputs with prompts and timestamps
  files: {read, updated, removed, created}  Categorized file lists (manually tracked)
  lines: {added, removed}              Line statistics (derived from git)

Examples:
  tsx scripts/log.ts create my-task "Implement new feature" --model=${Model.CLAUDE_OPUS_4_5} --prompt="User request..."
  tsx scripts/log.ts update MY-TASK --prompt="Follow-up request..." --file=scripts/log.ts
  tsx scripts/log.ts prompt MY-TASK "Follow-up user request..."
  tsx scripts/log.ts files MY-TASK scripts/log.ts README.md AGENTS.md
  tsx scripts/log.ts files MY-TASK --reset --category=updated
  tsx scripts/log.ts stats MY-TASK
  tsx scripts/log.ts stats MY-TASK --detect
  tsx scripts/log.ts read 2025 11 24 MY-TASK
  tsx scripts/log.ts list 2025 11
  tsx scripts/log.ts search "drag drop"
  tsx scripts/log.ts search "test" --limit=5
  tsx scripts/log.ts search --year=2025 --month=12 --limit=10
  tsx scripts/log.ts migrate
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
  if (!latest) throw new Error(`No log found for slug: ${normalizedSlug}`);
  const parsed = parseLogPath(latest.path);
  if (!parsed) throw new Error(`Failed to parse log path: ${latest.path}`);
  return parsed;
}

function ensureLogTracking(log: Log): { commit: string; files: LogFiles; lines: LogLines } {
  return {
    commit: log.frontmatter.commit || getGitHead(),
    files: log.frontmatter.files || {},
    lines: log.frontmatter.lines || { added: 0, removed: 0 },
  };
}

function getAllFilesFromLogFiles(files: LogFiles): string[] {
  const allFiles = new Set<string>();
  if (files.read) files.read.forEach((f) => allFiles.add(f));
  if (files.updated) files.updated.forEach((f) => allFiles.add(f));
  if (files.removed) files.removed.forEach((f) => allFiles.add(f));
  if (files.created) files.created.forEach((f) => allFiles.add(f));
  return Array.from(allFiles).sort();
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
        console.error(`Failed to migrate log: ${fullPath}`, error);
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
      case "create": {
        const [, slug, summary, ...rest] = args;
        if (!slug || !summary) {
          console.error("Error: Missing slug or summary");
          printUsage();
          process.exit(1);
        }
        const model = validateModel(requireFlag(rest, "model"));
        const prompt = requireFlag(rest, "prompt");
        const dateArg = rest.find((arg) => arg.startsWith("--date="));
        const date = dateArg ? new Date(dateArg.split("=")[1]) : undefined;
        const filesUpdated = parseFlags(rest, "file");
        const files: LogFiles = filesUpdated.length > 0 ? { updated: filesUpdated } : {};
        const log = createLog({ slug, summary, date, model, prompt, files });
        console.log(`Created log: ${log.path}`);
        console.log(`Summary: ${log.frontmatter.summary}`);
        break;
      }
      case "models": {
        console.log(Object.values(Model).join("\n"));
        break;
      }
      case "prompt": {
        const [, slug, ...promptParts] = args;
        if (!slug || !promptParts.length) {
          console.error("Error: Missing slug or prompt");
          printUsage();
          process.exit(1);
        }
        const prompt = promptParts.join(" ");
        const latest = getLatestLogBySlug(slug);
        const log = readLog(latest.year, latest.month, latest.day, latest.slug);
        updateLog(latest.year, latest.month, latest.day, latest.slug, { prompt });
        console.log(`Appended prompt to log: ${log.path}`);
        break;
      }
      case "files": {
        const [, slug, ...rest] = args;
        if (!slug) {
          console.error("Error: Missing slug");
          printUsage();
          process.exit(1);
        }
        const latest = getLatestLogBySlug(slug);
        const log = readLog(latest.year, latest.month, latest.day, latest.slug);
        const tracking = ensureLogTracking(log);
        const reset = rest.includes("--reset");
        const category = parseFlag(rest, "category") || "updated";
        const paths = rest.filter((arg) => !arg.startsWith("--"));
        if (!paths.length) {
          console.error("Error: No file paths provided. Files must be added explicitly via CLI.");
          printUsage();
          process.exit(1);
        }
        const existingFiles = reset ? [] : getAllFilesFromLogFiles(tracking.files);
        const newPaths = [...existingFiles, ...paths];
        const uniquePaths = Array.from(new Set(newPaths.filter((path) => path.trim()))).sort();
        const files: LogFiles = { [category]: uniquePaths };
        updateLog(latest.year, latest.month, latest.day, latest.slug, { files });
        console.log(`Updated files for log: ${log.path}`);
        console.log(`Files (${category}): ${uniquePaths.length}`);
        break;
      }
      case "stats": {
        const [, slug, ...rest] = args;
        if (!slug) {
          console.error("Error: Missing slug");
          printUsage();
          process.exit(1);
        }
        const latest = getLatestLogBySlug(slug);
        const log = readLog(latest.year, latest.month, latest.day, latest.slug);
        const tracking = ensureLogTracking(log);
        if (tracking.commit === "unknown") throw new Error(`Unknown commit for ${latest.slug}. Set the commit in frontmatter before running stats.`);
        const detect = rest.includes("--detect");
        const trackedFiles = getAllFilesFromLogFiles(tracking.files);
        const affectedFiles = detect ? getChangedFilesSince(tracking.commit) : trackedFiles;
        if (!affectedFiles.length) {
          throw new Error(`No files tracked for ${latest.slug}. Use: tsx scripts/log.ts files ${latest.slug} <paths...>`);
        }
        const computed = computeGitStats(tracking.commit, affectedFiles);
        updateLog(latest.year, latest.month, latest.day, latest.slug, {
          lines: { added: computed.added, removed: computed.removed },
        });
        console.log(`Updated stats for log: ${log.path}`);
        console.log(`Files: ${computed.affectedFiles.length}`);
        console.log(`Lines: +${computed.added} -${computed.removed}`);
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
        console.log(`Author: ${log.frontmatter.author}`);
        console.log(`Summary: ${log.frontmatter.summary}`);
        console.log(`Model: ${log.frontmatter.model}`);
        console.log(`Input: ${log.frontmatter.input?.length || 0}`);
        if (log.frontmatter.input?.length) {
          console.log(`  First: ${getFirstInputDate(log.frontmatter)}`);
          console.log(`  Latest: ${getLatestInputDate(log.frontmatter)}`);
        }
        if (log.frontmatter.commit) console.log(`Commit: ${log.frontmatter.commit}`);
        const files = log.frontmatter.files || {};
        const totalFiles = (files.read?.length || 0) + (files.updated?.length || 0) + (files.removed?.length || 0) + (files.created?.length || 0);
        if (totalFiles > 0) {
          console.log(`Files: ${totalFiles}`);
          if (files.read?.length) console.log(`  Read: ${files.read.length}`);
          if (files.updated?.length) console.log(`  Updated: ${files.updated.length}`);
          if (files.removed?.length) console.log(`  Removed: ${files.removed.length}`);
          if (files.created?.length) console.log(`  Created: ${files.created.length}`);
        }
        if (log.frontmatter.lines) console.log(`Lines: +${log.frontmatter.lines.added} -${log.frontmatter.lines.removed}`);
        console.log(`\nContent:\n${log.content}`);
        break;
      }
      case "update": {
        const [, slugArg, ...rest] = args;
        if (!slugArg) {
          console.error("Error: Missing slug");
          printUsage();
          process.exit(1);
        }
        const latest = getLatestLogBySlug(slugArg);
        const summary = parseFlag(rest, "summary");
        const modelFlag = parseFlag(rest, "model");
        const model = modelFlag ? validateModel(modelFlag) : undefined;
        const contentFile = parseFlag(rest, "contentFile");
        const content = contentFile ? readFileSync(contentFile, "utf-8") : undefined;
        const prompt = parseFlag(rest, "prompt");
        const filesUpdated = parseFlags(rest, "file");
        const filesRead = parseFlags(rest, "file-read");
        const filesCreated = parseFlags(rest, "file-created");
        const filesRemoved = parseFlags(rest, "file-removed");
        const update: LogUpdateInput = {};
        if (summary) update.summary = summary;
        if (model) update.model = model;
        if (content !== undefined) update.content = content;
        if (prompt) update.prompt = prompt;
        if (filesUpdated.length || filesRead.length || filesCreated.length || filesRemoved.length) {
          update.files = {};
          if (filesUpdated.length) update.files.updated = filesUpdated;
          if (filesRead.length) update.files.read = filesRead;
          if (filesCreated.length) update.files.created = filesCreated;
          if (filesRemoved.length) update.files.removed = filesRemoved;
        }
        if (!Object.keys(update).length) {
          console.error("Error: No update flags provided");
          printUsage();
          process.exit(1);
        }
        const updated = updateLog(latest.year, latest.month, latest.day, latest.slug, update);
        console.log(`Updated log: ${updated.path}`);
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
        console.log(`Deleted log: ${year}/${month}/${day}/${slug}`);
        break;
      }
      case "list": {
        const [, year, month, day] = args;
        const options: LogListOptions = {};
        if (year) options.year = parseInt(year);
        if (month) options.month = parseInt(month);
        if (day) options.day = parseInt(day);
        const logs = listLogs(options);
        console.log(`\nFound ${logs.length} log(s):\n`);
        for (const log of logs) {
          const parsed = parseLogPath(log.path);
          if (parsed) {
            console.log(`${parsed.year}-${String(parsed.month).padStart(2, "0")}-${String(parsed.day).padStart(2, "0")} ${parsed.slug}`);
            console.log(`  Summary: ${log.frontmatter.summary}`);
            console.log(`  Author: ${log.frontmatter.author}`);
            console.log(`  Model: ${log.frontmatter.model}`);
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
        console.log(`\nFound ${logs.length} log(s)${limitText}:\n`);
        for (const log of logs) {
          const parsed = parseLogPath(log.path);
          if (parsed) {
            console.log(`${parsed.year}-${String(parsed.month).padStart(2, "0")}-${String(parsed.day).padStart(2, "0")} ${parsed.slug}`);
            console.log(`  Summary: ${log.frontmatter.summary}`);
            console.log(`  Author: ${log.frontmatter.author}`);
            console.log(`  Model: ${log.frontmatter.model}`);
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
