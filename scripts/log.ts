import { execSync } from "child_process";
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, unlinkSync, writeFileSync } from "fs";
import { dirname, join } from "path";
const matter = require("gray-matter");

//#region Types
interface LogFrontmatter {
  date: string;
  slug: string;
  author: string;
  summary: string;
  model: string;
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
  model?: string;
  author?: string;
}

interface LogUpdateInput {
  summary?: string;
  content?: string;
  model?: string;
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

function getCurrentModel(): string {
  return process.env.SEMIO_MODEL || "claude-opus-4.5";
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
    model: input.model || getCurrentModel(),
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
    frontmatter: parsed.data as LogFrontmatter,
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
            frontmatter: matterParsed.data as LogFrontmatter,
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

//#region CLI
function printUsage(): void {
  console.log(`
Usage: tsx scripts/log.ts <command> [options]

Commands:
  create <slug> <summary>              Create a new log (slug will be capitalized)
  read <year> <month> <day> <slug>     Read a log
  update <year> <month> <day> <slug>   Update a log (interactive)
  delete <year> <month> <day> <slug>   Delete a log
  list [year] [month] [day]            List logs (optionally filtered)
  search [query] [--limit=N]           Search logs by query (searches slug, summary, content, author)
                                       Optional: --year=YYYY --month=MM --day=DD --limit=N
  migrate                              Migrate old logs to new structure

Examples:
  tsx scripts/log.ts create my-task "Implement new feature"  # Creates MY-TASK.md
  tsx scripts/log.ts read 2025 11 24 MY-TASK
  tsx scripts/log.ts list 2025 11
  tsx scripts/log.ts search "drag drop"
  tsx scripts/log.ts search "test" --limit=5
  tsx scripts/log.ts search --year=2025 --month=12 --limit=10
  tsx scripts/log.ts migrate
`);
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
        const dateArg = rest.find((arg) => arg.startsWith("--date="));
        const date = dateArg ? new Date(dateArg.split("=")[1]) : undefined;
        const log = createLog({ slug, summary, date });
        console.log(`Created log: ${log.path}`);
        console.log(`Summary: ${log.frontmatter.summary}`);
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
        console.log("Update functionality requires interactive input (not implemented in CLI)");
        console.log(`Use: readLog() and updateLog() programmatically`);
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
