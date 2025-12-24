// #region Header

// scripts/log.tsx

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

// SPDX-License-Identifier: AGPL-3.0-or-later
import { execSync } from "child_process";
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, unlinkSync, writeFileSync } from "fs";
import matter from "gray-matter";
import { Box, render, Text } from "ink";
import { dirname, join } from "path";

//#region Types
interface Lines {
  added: number;
  removed: number;
}

interface FileEntry {
  path: string;
  lines: Lines;
}

interface Files {
  updated: FileEntry[];
  created: FileEntry[];
  removed: FileEntry[];
}

interface IterationDate {
  started: string;
  ended?: string;
}

interface Iteration {
  prompt: string;
  date: IterationDate;
  model: string;
  author: string;
  commit?: string;
  files: Files;
  lines: Lines;
}

interface TicketDate {
  created: string;
  finished?: string;
}

interface TicketFrontmatter {
  slug: string;
  summary?: string;
  prompt: string;
  status: "open" | "finished";
  author: string;
  date: TicketDate;
  commit?: string;
  model?: string;
  iterations: Iteration[];
  files?: Files;
  lines?: Lines;
}

interface Ticket {
  frontmatter: TicketFrontmatter;
  content: string;
  path: string;
}

interface TicketCreateInput {
  slug: string;
  prompt: string;
  content?: string;
}

interface IterationStartInput {
  prompt: string;
  model: string;
  files: { updated?: string[]; created?: string[]; removed?: string[] };
}

interface IterationFinishInput {
  files: { updated?: string[]; created?: string[]; removed?: string[] };
}

interface ListOptions {
  year?: number;
  month?: number;
  day?: number;
  slug?: string;
}

interface SearchOptions extends ListOptions {
  query?: string;
  limit?: number;
}
//#endregion Types

//#region Exports
export type { FileEntry, Files, Iteration, IterationDate, IterationFinishInput, IterationStartInput, Lines, ListOptions, SearchOptions, Ticket, TicketCreateInput, TicketDate, TicketFrontmatter };

export { createTicket, deleteTicket, finishIteration, finishTicket, listTickets, readTicket, reopenTicket, searchTickets, startIteration };
//#endregion Exports

//#region Configuration
const LOG_ROOT = join(process.cwd(), "log");

export enum Model {
  COMPOSER_1 = "composer-1",
  CLAUDE_SONNET_4_5 = "claude-sonnet-4-5",
  CLAUDE_OPUS_4_5 = "claude-opus-4-5",
  GPT_5_1_CODEX_MAX = "gpt-5.1-codex-max",
  GPT_5_2_CODEX = "gpt-5.2-codex",
}

function getGitConfig(key: string): string {
  try {
    return execSync(`git config --get ${key}`, { encoding: "utf-8" }).trim();
  } catch {
    return "";
  }
}

function getGitAuthor(): string {
  const name = getGitConfig("user.name");
  const email = getGitConfig("user.email");
  if (name && email) return `${name} <${email}>`;
  if (name) return name;
  if (email) return email;
  return "Unknown";
}

function getGitHead(): string {
  return execSync("git rev-parse HEAD", { encoding: "utf-8" }).trim();
}
//#endregion Configuration

//#region Path Utilities
function getTicketPath(year: number, month: number, day: number, slug: string): string {
  const monthStr = month.toString().padStart(2, "0");
  const dayStr = day.toString().padStart(2, "0");
  return join(LOG_ROOT, "tickets", year.toString(), monthStr, dayStr, `${slug}.md`);
}

function parseTicketPath(path: string): { year: number; month: number; day: number; slug: string } | null {
  const relativePath = path.replace(LOG_ROOT, "").replace(/\\/g, "/");
  const match = relativePath.match(/^\/tickets\/(\d{4})\/(\d{2})\/(\d{2})\/(.+)\.md$/);
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
  if (!existsSync(dir)) mkdirSync(dir, { recursive: true });
}
//#endregion Path Utilities

//#region Git Stats
function getGitStatusPorcelain(): string {
  return execSync("git status --porcelain", { encoding: "utf-8" });
}

function parseGitPath(rawPath: string): string {
  const trimmed = rawPath.trim();
  if (!trimmed) return "";
  if (trimmed.startsWith('"') && trimmed.endsWith('"')) {
    try {
      return JSON.parse(trimmed);
    } catch {
      return trimmed.slice(1, -1);
    }
  }
  return trimmed;
}

function getUntrackedFiles(): Set<string> {
  const files = new Set<string>();
  for (const line of getGitStatusPorcelain().split(/\r?\n/)) {
    if (!line.startsWith("?? ")) continue;
    const path = parseGitPath(line.slice(3));
    if (path) files.add(path);
  }
  return files;
}

function quoteGitPath(path: string): string {
  return '"' + path.replaceAll('"', '\\"') + '"';
}

function parseGitNumstatOutput(output: string): Lines {
  let added = 0;
  let removed = 0;
  for (const line of output.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed) continue;
    const parts = trimmed.split("\t");
    if (parts.length < 2) continue;
    const a = parts[0] === "-" ? 0 : parseInt(parts[0]);
    const r = parts[1] === "-" ? 0 : parseInt(parts[1]);
    if (!Number.isNaN(a)) added += a;
    if (!Number.isNaN(r)) removed += r;
  }
  return { added, removed };
}

function computeGitLinesForFile(filePath: string, base?: string): Lines {
  const untracked = getUntrackedFiles();
  const nullPath = process.platform === "win32" ? "NUL" : "/dev/null";
  if (untracked.has(filePath)) {
    try {
      const output = execSync(`git diff --no-index --numstat -- ${nullPath} ${quoteGitPath(filePath)}`, { encoding: "utf-8" });
      return parseGitNumstatOutput(output);
    } catch (error: any) {
      if (error && error.stdout) {
        const stdout = Buffer.isBuffer(error.stdout) ? error.stdout.toString("utf-8") : String(error.stdout);
        return parseGitNumstatOutput(stdout);
      }
      return { added: 0, removed: 0 };
    }
  }
  const ref = base || "HEAD";
  try {
    const output = execSync(`git diff --numstat ${ref} -- ${quoteGitPath(filePath)}`, { encoding: "utf-8" });
    return parseGitNumstatOutput(output);
  } catch {
    return { added: 0, removed: 0 };
  }
}

function computeGitLinesForFiles(filePaths: string[], base?: string): Lines {
  let added = 0;
  let removed = 0;
  for (const filePath of filePaths) {
    const lines = computeGitLinesForFile(filePath, base);
    added += lines.added;
    removed += lines.removed;
  }
  return { added, removed };
}

function buildFilesWithLines(input: { updated?: string[]; created?: string[]; removed?: string[] }, base?: string): Files {
  const updated: FileEntry[] = (input.updated || []).map((path) => ({ path, lines: computeGitLinesForFile(path, base) }));
  const created: FileEntry[] = (input.created || []).map((path) => ({ path, lines: computeGitLinesForFile(path, base) }));
  const removed: FileEntry[] = (input.removed || []).map((path) => ({ path, lines: computeGitLinesForFile(path, base) }));
  return { updated, created, removed };
}

function computeTotalLines(files: Files): Lines {
  let added = 0;
  let removed = 0;
  for (const f of files.updated) {
    added += f.lines.added;
    removed += f.lines.removed;
  }
  for (const f of files.created) {
    added += f.lines.added;
    removed += f.lines.removed;
  }
  for (const f of files.removed) {
    added += f.lines.added;
    removed += f.lines.removed;
  }
  return { added, removed };
}
//#endregion Git Stats

//#region Validation
function requireFiles(files: { updated?: string[]; created?: string[]; removed?: string[] }): void {
  const hasAny = Boolean((files.updated && files.updated.length) || (files.created && files.created.length) || (files.removed && files.removed.length));
  if (!hasAny) throw new Error("Missing required files: provide at least one of --file=, --file-created=, --file-removed=");
}

function validateModel(model: string): string {
  const values = Object.values(Model);
  if (!values.includes(model as any)) throw new Error(`Unknown model: ${model}. Add it to the Model enum in scripts/log.ts.`);
  return model;
}
//#endregion Validation

//#region Serialization
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

function serializeFileEntry(entry: FileEntry): any {
  return { [entry.path]: { lines: entry.lines } };
}

function serializeFiles(files: Files): any {
  return {
    updated: files.updated.map(serializeFileEntry),
    created: files.created.map((f) => f.path),
    removed: files.removed.map((f) => f.path),
  };
}

function serializeIteration(iteration: Iteration): any {
  return {
    prompt: iteration.prompt,
    date: iteration.date,
    model: iteration.model,
    author: iteration.author,
    commit: iteration.commit,
    files: serializeFiles(iteration.files),
    lines: iteration.lines,
  };
}

function serializeFrontmatter(fm: TicketFrontmatter): any {
  const result: any = {
    slug: fm.slug,
    prompt: fm.prompt,
    status: fm.status,
    author: fm.author,
    date: fm.date,
  };
  if (fm.summary) result.summary = fm.summary;
  if (fm.commit) result.commit = fm.commit;
  if (fm.model) result.model = fm.model;
  result.iterations = fm.iterations.map(serializeIteration);
  if (fm.files) result.files = serializeFiles(fm.files);
  if (fm.lines) result.lines = fm.lines;
  return result;
}

function deserializeFileEntry(entry: any): FileEntry {
  if (typeof entry === "string") return { path: entry, lines: { added: 0, removed: 0 } };
  const keys = Object.keys(entry);
  if (keys.length === 1) {
    const path = keys[0];
    const value = entry[path];
    return { path, lines: value?.lines || { added: 0, removed: 0 } };
  }
  return { path: entry.path || "", lines: entry.lines || { added: 0, removed: 0 } };
}

function deserializeFiles(raw: any): Files {
  if (!raw) return { updated: [], created: [], removed: [] };
  const updated = (raw.updated || []).map(deserializeFileEntry);
  const created = (raw.created || []).map((e: any) => (typeof e === "string" ? { path: e, lines: { added: 0, removed: 0 } } : deserializeFileEntry(e)));
  const removed = (raw.removed || []).map((e: any) => (typeof e === "string" ? { path: e, lines: { added: 0, removed: 0 } } : deserializeFileEntry(e)));
  return { updated, created, removed };
}

function deserializeIteration(raw: any): Iteration {
  const date: IterationDate = typeof raw.date === "string" ? { started: raw.date } : { started: raw.date?.started || "", ended: raw.date?.ended };
  return {
    prompt: raw.prompt || "",
    date,
    model: raw.model || "unknown",
    author: raw.author || "Unknown",
    commit: raw.commit,
    files: deserializeFiles(raw.files),
    lines: raw.lines || { added: 0, removed: 0 },
  };
}

function deserializeFrontmatter(raw: any): TicketFrontmatter {
  const date: TicketDate = typeof raw.date === "string" ? { created: raw.date } : { created: raw.date?.created || new Date().toISOString(), finished: raw.date?.finished };
  const summary = typeof raw.summary === "string" && raw.summary.trim() ? raw.summary : undefined;
  const prompt = typeof raw.prompt === "string" ? raw.prompt : "";
  if (!prompt.trim()) throw new Error(`Missing required ticket prompt for: ${raw.slug || "UNKNOWN"}. Add 'prompt' to the ticket frontmatter.`);
  const status: "open" | "finished" = raw.status || "open";
  if (status === "finished" && !summary) throw new Error(`Missing required ticket summary for finished ticket: ${raw.slug || "UNKNOWN"}.`);
  return {
    slug: raw.slug || "UNKNOWN",
    summary,
    prompt,
    status,
    author: raw.author || "Unknown",
    date,
    commit: raw.commit,
    model: raw.model,
    iterations: (raw.iterations || []).map(deserializeIteration),
    files: raw.files ? deserializeFiles(raw.files) : undefined,
    lines: raw.lines,
  };
}
//#endregion Serialization

//#region CRUD Operations
function createTicket(input: TicketCreateInput): Ticket {
  const now = new Date();
  const year = now.getFullYear();
  const month = now.getMonth() + 1;
  const day = now.getDate();
  const slug = input.slug.toUpperCase();
  const ticketPath = getTicketPath(year, month, day, slug);
  if (existsSync(ticketPath)) throw new Error(`Ticket already exists: ${ticketPath}`);
  const frontmatter: TicketFrontmatter = {
    slug,
    prompt: input.prompt,
    status: "open",
    author: getGitAuthor(),
    date: { created: now.toISOString() },
    iterations: [],
  };
  const content = input.content || "# Previously\n\n# Plan\n\n# Changes\n";
  const fileContent = matter.stringify(content, sanitizeForMatter(serializeFrontmatter(frontmatter)));
  ensureDirectoryExists(ticketPath);
  writeFileSync(ticketPath, fileContent, "utf-8");
  return { frontmatter, content, path: ticketPath };
}

function migrateTicketPromptsFromFirstIteration(): { migrated: number; skipped: number } {
  let migrated = 0;
  let skipped = 0;
  const ticketPaths = listTicketPaths();
  for (const ticketPath of ticketPaths) {
    const fileContent = readFileSync(ticketPath, "utf-8");
    const parsed = matter(fileContent);
    const data: any = parsed.data || {};
    const currentPrompt = typeof data.prompt === "string" ? data.prompt.trim() : "";
    if (currentPrompt) {
      skipped++;
      continue;
    }
    const iterations = Array.isArray(data.iterations) ? data.iterations : [];
    const firstIterationPrompt = typeof iterations[0]?.prompt === "string" ? iterations[0].prompt.trim() : "";
    const nextPrompt = firstIterationPrompt || (typeof data.summary === "string" ? data.summary.trim() : "") || (typeof data.slug === "string" ? data.slug.trim() : "") || "UNKNOWN";
    if (!nextPrompt.trim()) throw new Error(`Cannot migrate ticket prompt (empty fallback): ${ticketPath}`);
    const nextData: any = {
      slug: data.slug,
      summary: data.summary,
      prompt: nextPrompt,
      status: data.status,
      author: data.author,
      date: data.date,
      commit: data.commit,
      model: data.model,
      iterations: data.iterations,
      files: data.files,
      lines: data.lines,
    };
    const rewritten = matter.stringify(parsed.content, sanitizeForMatter(nextData));
    writeFileSync(ticketPath, rewritten, "utf-8");
    migrated++;
  }
  return { migrated, skipped };
}

function readTicket(year: number, month: number, day: number, slug: string): Ticket {
  const ticketPath = getTicketPath(year, month, day, slug);
  if (!existsSync(ticketPath)) throw new Error(`Ticket not found: ${ticketPath}`);
  const fileContent = readFileSync(ticketPath, "utf-8");
  const parsed = matter(fileContent);
  return {
    frontmatter: deserializeFrontmatter(parsed.data),
    content: parsed.content,
    path: ticketPath,
  };
}

function writeTicket(ticket: Ticket): void {
  const fileContent = matter.stringify(ticket.content, sanitizeForMatter(serializeFrontmatter(ticket.frontmatter)));
  writeFileSync(ticket.path, fileContent, "utf-8");
}

function startIteration(year: number, month: number, day: number, slug: string, input: IterationStartInput): Ticket {
  requireFiles(input.files);
  const ticket = readTicket(year, month, day, slug);
  const iterations = ticket.frontmatter.iterations;
  if (iterations.length > 0) {
    const last = iterations[iterations.length - 1];
    if (!last.date.ended) throw new Error(`Cannot start a new iteration while the latest iteration is unfinished (started: ${last.date.started})`);
  }
  const now = new Date().toISOString();
  const files = buildFilesWithLines(input.files);
  const lines = computeTotalLines(files);
  const newIteration: Iteration = {
    prompt: input.prompt,
    date: { started: now },
    model: input.model,
    author: getGitAuthor(),
    files,
    lines,
  };
  ticket.frontmatter.iterations.push(newIteration);
  writeTicket(ticket);
  return ticket;
}

function finishIteration(year: number, month: number, day: number, slug: string, input: IterationFinishInput): Ticket {
  requireFiles(input.files);
  const ticket = readTicket(year, month, day, slug);
  const iterations = ticket.frontmatter.iterations;
  if (iterations.length === 0) throw new Error(`No iterations found for ticket: ${slug}`);
  const lastIteration = iterations[iterations.length - 1];
  if (lastIteration.date.ended) throw new Error(`Latest iteration already finished at: ${lastIteration.date.ended}`);
  const now = new Date().toISOString();
  const commit = getGitHead();
  const files = buildFilesWithLines(input.files);
  const lines = computeTotalLines(files);
  lastIteration.date.ended = now;
  lastIteration.commit = commit;
  lastIteration.files = files;
  lastIteration.lines = lines;
  writeTicket(ticket);
  return ticket;
}

function finishTicket(year: number, month: number, day: number, slug: string, summary: string): Ticket {
  if (!summary || !summary.trim()) throw new Error("Missing required ticket summary.");
  const ticket = readTicket(year, month, day, slug);
  const iterations = ticket.frontmatter.iterations;
  if (iterations.length === 0) throw new Error(`Cannot finish ticket without any iterations: ${slug}`);
  const last = iterations[iterations.length - 1];
  if (!last.date.ended) throw new Error(`Cannot finish ticket while the latest iteration is unfinished (started: ${last.date.started})`);
  const updatedPaths = new Set<string>();
  const createdPaths = new Set<string>();
  const removedPaths = new Set<string>();
  for (const iteration of iterations) {
    for (const f of iteration.files.updated) updatedPaths.add(f.path);
    for (const f of iteration.files.created) createdPaths.add(f.path);
    for (const f of iteration.files.removed) removedPaths.add(f.path);
  }
  const allPaths = [...updatedPaths, ...createdPaths, ...removedPaths];
  const base = getGitHead();
  const updated: FileEntry[] = [...updatedPaths].sort().map((path) => ({ path, lines: computeGitLinesForFile(path, base) }));
  const created: FileEntry[] = [...createdPaths].sort().map((path) => ({ path, lines: computeGitLinesForFile(path, base) }));
  const removed: FileEntry[] = [...removedPaths].sort().map((path) => ({ path, lines: computeGitLinesForFile(path, base) }));
  const files: Files = { updated, created, removed };
  const lines = computeGitLinesForFiles(allPaths, base);
  const now = new Date().toISOString();
  ticket.frontmatter.status = "finished";
  ticket.frontmatter.summary = summary;
  ticket.frontmatter.date.finished = now;
  ticket.frontmatter.commit = getGitHead();
  ticket.frontmatter.model = last.model;
  ticket.frontmatter.files = files;
  ticket.frontmatter.lines = lines;
  writeTicket(ticket);
  return ticket;
}

function reopenTicket(year: number, month: number, day: number, slug: string): Ticket {
  const ticket = readTicket(year, month, day, slug);
  if (ticket.frontmatter.status === "open") throw new Error(`Ticket is already open: ${slug}`);
  ticket.frontmatter.status = "open";
  delete ticket.frontmatter.date.finished;
  delete ticket.frontmatter.commit;
  delete ticket.frontmatter.model;
  delete ticket.frontmatter.files;
  delete ticket.frontmatter.lines;
  writeTicket(ticket);
  return ticket;
}

function addPlanToTicket(year: number, month: number, day: number, slug: string, planFilePath: string): Ticket {
  if (!existsSync(planFilePath)) throw new Error(`Plan file not found: ${planFilePath}`);
  const planContent = readFileSync(planFilePath, "utf-8").trim();
  const ticket = readTicket(year, month, day, slug);
  const content = ticket.content;
  const planHeaderRegex = /^# Plan\s*$/m;
  const match = content.match(planHeaderRegex);
  if (!match) throw new Error(`No "# Plan" section found in ticket: ${slug}`);
  const planHeaderIndex = match.index!;
  const afterPlanHeader = planHeaderIndex + match[0].length;
  const nextSectionRegex = /^# /m;
  const remainingContent = content.slice(afterPlanHeader);
  const nextSectionMatch = remainingContent.match(nextSectionRegex);
  let newContent: string;
  if (nextSectionMatch) {
    const nextSectionIndex = afterPlanHeader + nextSectionMatch.index!;
    newContent = content.slice(0, afterPlanHeader) + "\n\n" + planContent + "\n\n" + content.slice(nextSectionIndex);
  } else {
    newContent = content.slice(0, afterPlanHeader) + "\n\n" + planContent + "\n";
  }
  ticket.content = newContent;
  writeTicket(ticket);
  return ticket;
}

function deleteTicket(year: number, month: number, day: number, slug: string): void {
  const ticketPath = getTicketPath(year, month, day, slug);
  if (!existsSync(ticketPath)) throw new Error(`Ticket not found: ${ticketPath}`);
  unlinkSync(ticketPath);
}

function listTickets(options: ListOptions = {}): Ticket[] {
  const tickets: Ticket[] = [];
  const invalidPaths: string[] = [];
  const paths = listTicketPaths(options);
  for (const path of paths) {
    try {
      const fileContent = readFileSync(path, "utf-8");
      const matterParsed = matter(fileContent);
      tickets.push({
        frontmatter: deserializeFrontmatter(matterParsed.data),
        content: matterParsed.content,
        path,
      });
    } catch {
      invalidPaths.push(path);
    }
  }
  if (invalidPaths.length > 0) {
    const head = invalidPaths.slice(0, 10).join("\n");
    throw new Error(`Failed to parse ${invalidPaths.length} ticket(s). First 10:\n${head}`);
  }
  return tickets.sort((a, b) => new Date(b.frontmatter.date.created).getTime() - new Date(a.frontmatter.date.created).getTime());
}

function listTicketPaths(options: ListOptions = {}): string[] {
  const paths: string[] = [];
  function walk(dir: string): void {
    if (!existsSync(dir)) return;
    const entries = readdirSync(dir);
    for (const entry of entries) {
      const fullPath = join(dir, entry);
      const stat = statSync(fullPath);
      if (stat.isDirectory()) {
        walk(fullPath);
        continue;
      }
      if (!entry.endsWith(".md") || entry === "prompts.md") continue;
      const parsed = parseTicketPath(fullPath);
      if (!parsed) continue;
      if (options.year !== undefined && parsed.year !== options.year) continue;
      if (options.month !== undefined && parsed.month !== options.month) continue;
      if (options.day !== undefined && parsed.day !== options.day) continue;
      if (options.slug !== undefined && parsed.slug !== options.slug) continue;
      paths.push(fullPath);
    }
  }
  walk(join(LOG_ROOT, "tickets"));
  return paths;
}

function searchTickets(options: SearchOptions = {}): Ticket[] {
  const allTickets = listTickets({ year: options.year, month: options.month, day: options.day, slug: options.slug });
  if (!options.query) return options.limit ? allTickets.slice(0, options.limit) : allTickets;
  const query = options.query.toLowerCase();
  const matchedTickets = allTickets.filter((ticket) => {
    const slugMatch = ticket.frontmatter.slug.toLowerCase().includes(query);
    const summaryMatch = (ticket.frontmatter.summary || "").toLowerCase().includes(query);
    const contentMatch = ticket.content.toLowerCase().includes(query);
    const authorMatch = ticket.frontmatter.author.toLowerCase().includes(query);
    return slugMatch || summaryMatch || contentMatch || authorMatch;
  });
  return options.limit ? matchedTickets.slice(0, options.limit) : matchedTickets;
}
//#endregion CRUD Operations

//#region Lookup
function findLatestTicketBySlug(slug: string): { year: number; month: number; day: number; slug: string } {
  const normalizedSlug = slug.toUpperCase();
  const tickets = listTickets({ slug: normalizedSlug });
  const latest = tickets[0];
  if (!latest) throw new Error(`No ticket found for slug: ${normalizedSlug}`);
  const parsed = parseTicketPath(latest.path);
  if (!parsed) throw new Error(`Failed to parse ticket path: ${latest.path}`);
  return parsed;
}
//#endregion Lookup

//#region CLI
function printUsage(): void {
  const usage = `
Usage: tsx scripts/log.ts <command> [options]

Commands:
  ticket create <slug>                 Create a ticket (no iterations)
                                       Required: --prompt="..."
  ticket iteration start <slug>        Start a new iteration on a ticket
                                       Required: --model=MODEL --prompt="..." and at least one file flag
  ticket iteration finish <slug>       Finish the latest iteration
                                       Required: at least one file flag
  ticket finish <slug>                 Finish the ticket (requires latest iteration finished)
                                       Required: --summary="..."
  ticket reopen <slug>                 Reopen a finished ticket (removes total files/lines)
  ticket plan <slug>                   Add a plan to the ticket from a markdown file
                                       Required: --plan=path/to/plan.md
  ticket migrate prompts               Backfill missing ticket prompts from the first iteration prompt
  ticket read <year> <month> <day> <slug>     Read a ticket
  ticket delete <year> <month> <day> <slug>   Delete a ticket
  ticket list [year] [month] [day]            List tickets (optionally filtered)
  ticket search [query] [--limit=N]           Search tickets
                                             Optional: --year=YYYY --month=MM --day=DD --limit=N
  models                               List available model enum values

File Flags:
  --file=PATH              Add file to updated list
  --file-created=PATH      Add file to created list
  --file-removed=PATH      Add file to removed list

Ticket Schema:
  slug, summary, prompt, status, author, date{created, finished}, commit, model,
  iterations[{prompt, date{started, ended}, model, author, commit, files{...}, lines{...}}],
  files{updated[{path, lines}], created[path], removed[path]}, lines{added, removed}

  - author, commit, date, lines: derived from git (forbidden to set manually)
  - model, files: must be set manually
  - when ticket is finished: files and lines are computed from git

Workflow:
  1. Create a ticket: ticket create <slug> --prompt="..."
  2. Start iteration: ticket iteration start <slug> --model=MODEL --prompt="..." --file=...
  3. Finish iteration: ticket iteration finish <slug> --file=...
  4. Finish ticket: ticket finish <slug> --summary="..."

Examples:
  tsx scripts/log.ts ticket create MY-TASK --prompt="User request..."
  tsx scripts/log.ts ticket iteration start MY-TASK --model=${Model.CLAUDE_OPUS_4_5} --prompt="User request..." --file=scripts/log.ts
  tsx scripts/log.ts ticket iteration finish MY-TASK --file=scripts/log.ts --file=README.md
  tsx scripts/log.ts ticket finish MY-TASK --summary="Implement new feature"
  tsx scripts/log.ts ticket reopen MY-TASK
  tsx scripts/log.ts ticket plan MY-TASK --plan=docs/plan.md
  tsx scripts/log.ts ticket read 2025 12 16 MY-TASK
  tsx scripts/log.ts ticket list 2025 12
  tsx scripts/log.ts ticket search "drag drop"
  tsx scripts/log.ts ticket migrate prompts
  tsx scripts/log.ts models
`;
  render(
    <Box flexDirection="column">
      <Text>{usage}</Text>
    </Box>,
  );
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

function parseFilesFromFlags(flags: string[]): { updated?: string[]; created?: string[]; removed?: string[] } {
  const filesUpdated = parseFlags(flags, "file");
  const filesCreated = parseFlags(flags, "file-created");
  const filesRemoved = parseFlags(flags, "file-removed");
  return {
    updated: filesUpdated.length > 0 ? filesUpdated : undefined,
    created: filesCreated.length > 0 ? filesCreated : undefined,
    removed: filesRemoved.length > 0 ? filesRemoved : undefined,
  };
}

const args = process.argv.slice(2);
const command = args[0];
try {
  switch (command) {
    case "ticket": {
      const [, sub, ...rest] = args;
      if (!sub) {
        render(
          <Box flexDirection="column">
            <Text color="red">Error: Missing ticket command</Text>
          </Box>,
        );
        printUsage();
        process.exit(1);
      }
      if (sub === "create") {
        const [slug, ...flags] = rest;
        if (!slug) {
          render(
            <Box flexDirection="column">
              <Text color="red">Error: Missing slug</Text>
            </Box>,
          );
          printUsage();
          process.exit(1);
        }
        for (const flag of flags) {
          if (!flag.startsWith("--")) throw new Error(`Unexpected argument: ${flag}. Summary is set on ticket finish via --summary=.`);
        }
        const prompt = requireFlag(flags, "prompt");
        const ticket = createTicket({ slug, prompt });
        render(
          <Box flexDirection="column">
            <Text>✅ Created ticket: {ticket.path}</Text>
          </Box>,
        );
        break;
      }

      if (sub === "migrate") {
        const [migrationCommand] = rest;
        if (migrationCommand === "prompts") {
          const result = migrateTicketPromptsFromFirstIteration();
          render(
            <Box flexDirection="column">
              <Text>
                ✅ Migrated ticket prompts: {result.migrated} (skipped: {result.skipped})
              </Text>
            </Box>,
          );
          break;
        }
        render(
          <Box flexDirection="column">
            <Text color="red">Error: Unknown migrate command</Text>
          </Box>,
        );
        printUsage();
        process.exit(1);
        break;
      }
      if (sub === "iteration") {
        const [iterationCommand, slugArg, ...flags] = rest;
        if (iterationCommand === "start") {
          if (!slugArg) {
            render(
              <Box flexDirection="column">
                <Text color="red">Error: Missing slug</Text>
              </Box>,
            );
            printUsage();
            process.exit(1);
          }
          const model = validateModel(requireFlag(flags, "model"));
          const prompt = requireFlag(flags, "prompt");
          const files = parseFilesFromFlags(flags);
          const latest = findLatestTicketBySlug(slugArg);
          const ticket = startIteration(latest.year, latest.month, latest.day, latest.slug, { prompt, model, files });
          render(
            <Box flexDirection="column">
              <Text>
                ✅ Started iteration {ticket.frontmatter.iterations.length} for ticket: {ticket.path}
              </Text>
            </Box>,
          );
          break;
        }
        if (iterationCommand === "finish") {
          if (!slugArg) {
            render(
              <Box flexDirection="column">
                <Text color="red">Error: Missing slug</Text>
              </Box>,
            );
            printUsage();
            process.exit(1);
          }
          const files = parseFilesFromFlags(flags);
          const latest = findLatestTicketBySlug(slugArg);
          const ticket = finishIteration(latest.year, latest.month, latest.day, latest.slug, { files });
          const lastIteration = ticket.frontmatter.iterations[ticket.frontmatter.iterations.length - 1];
          render(
            <Box flexDirection="column">
              <Text>✅ Finished iteration for ticket: {ticket.path}</Text>
              <Text dimColor>Commit: {lastIteration.commit || "none"}</Text>
              <Text dimColor>
                Lines: +{lastIteration.lines.added} -{lastIteration.lines.removed}
              </Text>
            </Box>,
          );
          break;
        }
        render(
          <Box flexDirection="column">
            <Text color="red">Error: Unknown iteration command</Text>
          </Box>,
        );
        printUsage();
        process.exit(1);
        break;
      }
      if (sub === "finish") {
        const [slugArg, ...flags] = rest;
        if (!slugArg) {
          console.error("Error: Missing slug");
          printUsage();
          process.exit(1);
        }
        for (const flag of flags) {
          if (!flag.startsWith("--")) throw new Error(`Unexpected argument: ${flag}`);
        }
        const summary = requireFlag(flags, "summary");
        const latest = findLatestTicketBySlug(slugArg);
        const ticket = finishTicket(latest.year, latest.month, latest.day, latest.slug, summary);
        render(
          <Box flexDirection="column">
            <Text>✅ Finished ticket: {ticket.path}</Text>
            <Text dimColor>Commit: {ticket.frontmatter.commit}</Text>
            {ticket.frontmatter.lines && (
              <Text dimColor>
                Lines: +{ticket.frontmatter.lines.added} -{ticket.frontmatter.lines.removed}
              </Text>
            )}
          </Box>,
        );
        break;
      }
      if (sub === "reopen") {
        const [slugArg] = rest;
        if (!slugArg) {
          render(
            <Box flexDirection="column">
              <Text color="red">Error: Missing slug</Text>
            </Box>,
          );
          printUsage();
          process.exit(1);
        }
        const latest = findLatestTicketBySlug(slugArg);
        const ticket = reopenTicket(latest.year, latest.month, latest.day, latest.slug);
        render(
          <Box flexDirection="column">
            <Text>✅ Reopened ticket: {ticket.path}</Text>
            <Text dimColor>Status: {ticket.frontmatter.status}</Text>
            <Text dimColor>Iterations preserved: {ticket.frontmatter.iterations.length}</Text>
          </Box>,
        );
        break;
      }
      if (sub === "plan") {
        const [slugArg, ...flags] = rest;
        if (!slugArg) {
          render(
            <Box flexDirection="column">
              <Text color="red">Error: Missing slug</Text>
            </Box>,
          );
          printUsage();
          process.exit(1);
        }
        const planPath = requireFlag(flags, "plan");
        const latest = findLatestTicketBySlug(slugArg);
        const ticket = addPlanToTicket(latest.year, latest.month, latest.day, latest.slug, planPath);
        render(
          <Box flexDirection="column">
            <Text>✅ Added plan to ticket: {ticket.path}</Text>
          </Box>,
        );
        break;
      }
      if (sub === "read") {
        const [year, month, day, slug] = rest;
        if (!year || !month || !day || !slug) {
          render(
            <Box flexDirection="column">
              <Text color="red">Error: Missing year, month, day, or slug</Text>
            </Box>,
          );
          printUsage();
          process.exit(1);
        }
        const ticket = readTicket(parseInt(year), parseInt(month), parseInt(day), slug);
        render(
          <Box flexDirection="column">
            <Text>📄 Path: {ticket.path}</Text>
            <Text>Slug: {ticket.frontmatter.slug}</Text>
            <Text>Summary: {ticket.frontmatter.summary || ""}</Text>
            <Text>Status: {ticket.frontmatter.status}</Text>
            <Text>Author: {ticket.frontmatter.author}</Text>
            <Text>Created: {ticket.frontmatter.date.created}</Text>
            {ticket.frontmatter.date.finished && <Text>Finished: {ticket.frontmatter.date.finished}</Text>}
            {ticket.frontmatter.commit && <Text>Commit: {ticket.frontmatter.commit}</Text>}
            {ticket.frontmatter.model && <Text>Model: {ticket.frontmatter.model}</Text>}
            <Text>Iterations: {ticket.frontmatter.iterations.length}</Text>
            {ticket.frontmatter.iterations.map((it, i) => {
              const totalFiles = it.files.updated.length + it.files.created.length + it.files.removed.length;
              return (
                <Box key={i} flexDirection="column" marginTop={1}>
                  <Text dimColor>
                    {" "}
                    [{i + 1}] {it.date.started}
                  </Text>
                  <Text dimColor> Model: {it.model}</Text>
                  <Text dimColor> Author: {it.author}</Text>
                  {it.date.ended && <Text dimColor> Ended: {it.date.ended}</Text>}
                  {it.commit && <Text dimColor> Commit: {it.commit.substring(0, 8)}</Text>}
                  <Text dimColor>
                    {" "}
                    Lines: +{it.lines.added} -{it.lines.removed}
                  </Text>
                  <Text dimColor>
                    {" "}
                    Files: {totalFiles} ({it.files.updated.length} updated, {it.files.created.length} created, {it.files.removed.length} removed)
                  </Text>
                  <Text dimColor>
                    {" "}
                    Prompt: {it.prompt.substring(0, 80)}
                    {it.prompt.length > 80 ? "..." : ""}
                  </Text>
                </Box>
              );
            })}
            {ticket.frontmatter.files && (
              <Box marginTop={1}>
                <Text>Total Files: {ticket.frontmatter.files.updated.length + ticket.frontmatter.files.created.length + ticket.frontmatter.files.removed.length}</Text>
              </Box>
            )}
            {ticket.frontmatter.lines && (
              <Text>
                Total Lines: +{ticket.frontmatter.lines.added} -{ticket.frontmatter.lines.removed}
              </Text>
            )}
            <Box marginTop={1}>
              <Text>Content:</Text>
            </Box>
            <Text dimColor>{ticket.content}</Text>
          </Box>,
        );
        break;
      }
      if (sub === "delete") {
        const [year, month, day, slug] = rest;
        if (!year || !month || !day || !slug) {
          render(
            <Box flexDirection="column">
              <Text color="red">Error: Missing year, month, day, or slug</Text>
            </Box>,
          );
          printUsage();
          process.exit(1);
        }
        deleteTicket(parseInt(year), parseInt(month), parseInt(day), slug);
        render(
          <Box flexDirection="column">
            <Text>
              ✅ Deleted ticket: {year}/{month}/{day}/{slug}
            </Text>
          </Box>,
        );
        break;
      }
      if (sub === "list") {
        const [year, month, day] = rest;
        const options: ListOptions = {};
        if (year) options.year = parseInt(year);
        if (month) options.month = parseInt(month);
        if (day) options.day = parseInt(day);
        const tickets = listTickets(options);
        render(
          <Box flexDirection="column">
            <Text>Found {tickets.length} ticket(s):</Text>
            {tickets.map((ticket) => {
              const parsed = parseTicketPath(ticket.path);
              if (!parsed) return null;
              return (
                <Box key={ticket.path} flexDirection="column" marginTop={1}>
                  <Text>
                    {parsed.year}-{String(parsed.month).padStart(2, "0")}-{String(parsed.day).padStart(2, "0")} {parsed.slug}
                  </Text>
                  <Text dimColor> Summary: {ticket.frontmatter.summary}</Text>
                  <Text dimColor> Status: {ticket.frontmatter.status}</Text>
                  <Text dimColor> Author: {ticket.frontmatter.author}</Text>
                  {ticket.frontmatter.model && <Text dimColor> Model: {ticket.frontmatter.model}</Text>}
                  <Text dimColor> Iterations: {ticket.frontmatter.iterations.length}</Text>
                </Box>
              );
            })}
          </Box>,
        );
        break;
      }
      if (sub === "search") {
        const options: SearchOptions = {};
        let query = "";
        for (const arg of rest) {
          if (arg.startsWith("--year=")) options.year = parseInt(arg.split("=")[1]);
          else if (arg.startsWith("--month=")) options.month = parseInt(arg.split("=")[1]);
          else if (arg.startsWith("--day=")) options.day = parseInt(arg.split("=")[1]);
          else if (arg.startsWith("--limit=")) options.limit = parseInt(arg.split("=")[1]);
          else if (!arg.startsWith("--")) query = arg;
        }
        if (query) options.query = query;
        const tickets = searchTickets(options);
        const limitText = options.limit ? ` (showing first ${options.limit})` : "";
        render(
          <Box flexDirection="column">
            <Text>
              Found {tickets.length} ticket(s){limitText}:
            </Text>
            {tickets.map((ticket) => {
              const parsed = parseTicketPath(ticket.path);
              if (!parsed) return null;
              return (
                <Box key={ticket.path} flexDirection="column" marginTop={1}>
                  <Text>
                    {parsed.year}-{String(parsed.month).padStart(2, "0")}-{String(parsed.day).padStart(2, "0")} {parsed.slug}
                  </Text>
                  <Text dimColor> Summary: {ticket.frontmatter.summary}</Text>
                  <Text dimColor> Status: {ticket.frontmatter.status}</Text>
                  <Text dimColor> Author: {ticket.frontmatter.author}</Text>
                  {ticket.frontmatter.model && <Text dimColor> Model: {ticket.frontmatter.model}</Text>}
                  {options.query && <Text dimColor> Preview: {ticket.content.substring(0, 200).replace(/\n/g, " ")}...</Text>}
                </Box>
              );
            })}
          </Box>,
        );
        break;
      }
      render(
        <Box flexDirection="column">
          <Text color="red">Error: Unknown ticket command</Text>
        </Box>,
      );
      printUsage();
      process.exit(1);
      break;
    }
    case "models": {
      render(
        <Box flexDirection="column">
          <Text>Available models:</Text>
          {Object.values(Model).map((model) => (
            <Text key={model} dimColor>
              {" "}
              {model}
            </Text>
          ))}
        </Box>,
      );
      break;
    }
    default:
      printUsage();
      process.exit(1);
  }
} catch (error) {
  render(
    <Box flexDirection="column">
      <Text color="red">Error: {error instanceof Error ? error.message : String(error)}</Text>
    </Box>,
  );
  process.exit(1);
}
//#endregion CLI
