import { execSync } from "child_process";
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, unlinkSync, writeFileSync } from "fs";
import { dirname, join } from "path";
const matter = require("gray-matter");

//#region Types
interface LogStats {
  base: string;
  affectedFiles: string[];
  addedLines: number;
  removedLines: number;
  updatedAt: string;
}

interface LogFrontmatter {
  date: string;
  slug: string;
  author: string;
  summary: string;
  model: string;
  prompts?: string[];
  stats?: LogStats;
}

interface Log {
  frontmatter: LogFrontmatter;
  content: string;
  path: string;
}

interface LogCreateInput {
  slug: string;
  summary: string;
  prompts: string[];
  model: string;
  content?: string;
  date?: Date;
  author?: string;
}

interface LogUpdateInput {
  summary?: string;
  content?: string;
  model?: string;
  prompts?: string[];
  stats?: LogStats;
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
function normalizeLogFrontmatter(frontmatter: LogFrontmatter): LogFrontmatter {
  if (!frontmatter.prompts) frontmatter.prompts = [];
  return frontmatter;
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
  const frontmatter: LogFrontmatter = {
    date: date.toISOString(),
    slug,
    author: input.author || getDefaultAuthor(),
    summary: input.summary,
    model: input.model,
    prompts: input.prompts,
    stats: {
      base: getGitHead(),
      affectedFiles: [],
      addedLines: 0,
      removedLines: 0,
      updatedAt: date.toISOString(),
    },
  };
  const content = input.content || "# Previously\n\n# Plan\n\n# Changes\n";
  const fileContent = matter.stringify(content, frontmatter);
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
    frontmatter: normalizeLogFrontmatter(parsed.data as LogFrontmatter),
    content: parsed.content,
    path: logPath,
  };
}

export function updateLog(year: number, month: number, day: number, slug: string, update: LogUpdateInput): Log {
  const log = readLog(year, month, day, slug);
  const newFrontmatter: LogFrontmatter = {
    ...log.frontmatter,
    summary: update.summary ?? log.frontmatter.summary,
    model: update.model ?? log.frontmatter.model,
    prompts: update.prompts ?? log.frontmatter.prompts,
    stats: update.stats ?? log.frontmatter.stats,
  };
  const newContent = update.content ?? log.content;
  const fileContent = matter.stringify(newContent, newFrontmatter);
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
          logs.push({
            frontmatter: normalizeLogFrontmatter(matterParsed.data as LogFrontmatter),
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
  return logs.sort((a, b) => new Date(b.frontmatter.date).getTime() - new Date(a.frontmatter.date).getTime());
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
    if (parsed.data.date) {
      console.log(`Skipping already migrated log: ${entry}`);
      continue;
    }
    const date = new Date(parseInt(year), parseInt(month) - 1, parseInt(day));
    const frontmatter: LogFrontmatter = {
      date: date.toISOString(),
      slug: capitalizedSlug,
      author: getDefaultAuthor(),
      summary: `Migration from ${entry}`,
      model: "unknown",
      prompts: [],
    };
    const newPath = getLogPath(parseInt(year), parseInt(month), parseInt(day), capitalizedSlug);
    ensureDirectoryExists(newPath);
    const fileContent = matter.stringify(parsed.content || content, frontmatter);
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
  return `"${path.replaceAll("\"", "\\\"")}"`;
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
  prompt <slug> <prompt>               Append a user prompt to the latest log for the slug
  files <slug> [paths...]              Update tracked affected files for the latest log for the slug
                                       Optional: --detect (from git), --reset (replace list)
  stats <slug> [--detect]              Update git stats for the latest log for the slug (uses tracked affected files)
  models                               List available model enum values
  read <year> <month> <day> <slug>     Read a log
  update <year> <month> <day> <slug>   Update a log (non-interactive flags)
  delete <year> <month> <day> <slug>   Delete a log
  list [year] [month] [day]            List logs (optionally filtered)
  search [query] [--limit=N]           Search logs by query (searches slug, summary, content, author)
                                       Optional: --year=YYYY --month=MM --day=DD --limit=N
  migrate                              Migrate old logs to new structure

Examples:
  tsx scripts/log.ts create my-task "Implement new feature" --model=${Model.CLAUDE_OPUS_4_5} --prompt="User request..."
  tsx scripts/log.ts prompt MY-TASK "Follow-up user request..."
  tsx scripts/log.ts files MY-TASK scripts/log.ts README.md AGENTS.md
  tsx scripts/log.ts files MY-TASK --detect --reset
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

function ensureLogStats(log: Log): LogStats {
  if (log.frontmatter.stats) return log.frontmatter.stats;
  const now = new Date().toISOString();
  return {
    base: getGitHead(),
    affectedFiles: [],
    addedLines: 0,
    removedLines: 0,
    updatedAt: now,
  };
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
        const prompts = parseFlags(rest, "prompt");
        if (!prompts.length) {
          console.error("Error: Missing prompt(s). Provide at least one --prompt=... flag.");
          printUsage();
          process.exit(1);
        }
        const dateArg = rest.find((arg) => arg.startsWith("--date="));
        const date = dateArg ? new Date(dateArg.split("=")[1]) : undefined;
        const log = createLog({ slug, summary, date, model, prompts });
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
        const prompts = (log.frontmatter.prompts || []).slice();
        prompts.push(prompt);
        updateLog(latest.year, latest.month, latest.day, latest.slug, { prompts });
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
        const currentStats = ensureLogStats(log);
        const detect = rest.includes("--detect");
        const reset = rest.includes("--reset");
        const paths = rest.filter((arg) => !arg.startsWith("--"));
        const base = currentStats.base;
        const detected = detect ? getChangedFilesSince(base) : [];
        const updated = reset ? [...detected, ...paths] : [...currentStats.affectedFiles, ...detected, ...paths];
        const affectedFiles = Array.from(new Set(updated.filter((path) => path.trim()))).sort();
        updateLog(latest.year, latest.month, latest.day, latest.slug, {
          stats: {
            ...currentStats,
            affectedFiles,
            updatedAt: new Date().toISOString(),
          },
        });
        console.log(`Updated affected files for log: ${log.path}`);
        console.log(`Affected files: ${affectedFiles.length}`);
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
        const currentStats = ensureLogStats(log);
        const detect = rest.includes("--detect");
        const affectedFiles = detect ? getChangedFilesSince(currentStats.base) : currentStats.affectedFiles;
        if (!affectedFiles.length) {
          throw new Error(`No affected files tracked for ${latest.slug}. Use: tsx scripts/log.ts files ${latest.slug} --detect OR tsx scripts/log.ts files ${latest.slug} <paths...>`);
        }
        const computed = computeGitStats(currentStats.base, affectedFiles);
        const stats: LogStats = {
          base: currentStats.base,
          affectedFiles: computed.affectedFiles,
          addedLines: computed.added,
          removedLines: computed.removed,
          updatedAt: new Date().toISOString(),
        };
        updateLog(latest.year, latest.month, latest.day, latest.slug, { stats });
        console.log(`Updated stats for log: ${log.path}`);
        console.log(`Affected files: ${stats.affectedFiles.length}`);
        console.log(`Lines: +${stats.addedLines} -${stats.removedLines}`);
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
        console.log(`Date: ${log.frontmatter.date}`);
        console.log(`Author: ${log.frontmatter.author}`);
        console.log(`Summary: ${log.frontmatter.summary}`);
        console.log(`Model: ${log.frontmatter.model}`);
        console.log(`Prompts: ${log.frontmatter.prompts?.length || 0}`);
        if (log.frontmatter.stats) {
          console.log(`Stats base: ${log.frontmatter.stats.base}`);
          console.log(`Stats files: ${log.frontmatter.stats.affectedFiles.length}`);
          console.log(`Stats lines: +${log.frontmatter.stats.addedLines} -${log.frontmatter.stats.removedLines}`);
        }
        console.log(`\nContent:\n${log.content}`);
        break;
      }
      case "update": {
        const [, year, month, day, slug] = args;
        if (!year || !month || !day || !slug) {
          console.error("Error: Missing year, month, day, or slug");
          printUsage();
          process.exit(1);
        }
        const rest = args.slice(5);
        const summary = parseFlag(rest, "summary");
        const modelFlag = parseFlag(rest, "model");
        const model = modelFlag ? validateModel(modelFlag) : undefined;
        const contentFile = parseFlag(rest, "contentFile");
        const content = contentFile ? readFileSync(contentFile, "utf-8") : undefined;
        const prompts = parseFlags(rest, "prompt");
        const update: LogUpdateInput = {};
        if (summary) update.summary = summary;
        if (model) update.model = model;
        if (content !== undefined) update.content = content;
        if (prompts.length) update.prompts = prompts;
        if (!Object.keys(update).length) {
          console.error("Error: No update flags provided");
          printUsage();
          process.exit(1);
        }
        const updated = updateLog(parseInt(year), parseInt(month), parseInt(day), slug, update);
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
        console.log("Migration complete");
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
