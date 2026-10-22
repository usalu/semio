import { randomUUID } from "node:crypto";
import { appendFileSync, existsSync, mkdirSync, readFileSync, statSync, truncateSync } from "node:fs";
import { isAbsolute, join, resolve } from "node:path";
import { CATALOG_LANGUAGES_V1 } from "@P/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { refuseConfigV1 } from "./📖️texts/🟦️.ts";

/** 🧾️ One fact of the local configuration; the shape is the schema of this folder, nothing is ever rewritten. */
export type LocalConfigEventV1 = Readonly<{ version: 1; id: string; at: string; type: "language-chosen"; language: string }> | Readonly<{ version: 1; id: string; at: string; type: "language-cleared" }>;

/** 🧱️ The configuration as the fold of its events. */
export type LocalConfigV1 = Readonly<{ version: 1; language: string | undefined; events: number }>;

const LOCATION = schema.definitions.Location.const;
const ID = new RegExp(schema.definitions.Id.pattern, "u");
const INSTANT = new RegExp(schema.definitions.Instant.pattern, "u");

/** 📍️ The directory of the event log: the one SEMIO_CONFIG_DIRECTORY names, else `semio/config` inside the git directory (a worktree pointer is followed); undefined when neither exists. */
export function localConfigDirectoryV1(root: string, environment: Readonly<Record<string, string | undefined>>): string | undefined {
  const named = environment[LOCATION.environmentVariable];
  if (named !== undefined && named !== "") return isAbsolute(named) ? named : resolve(root, named);
  const git = join(root, ".git");
  if (!existsSync(git)) return undefined;
  if (statSync(git).isDirectory()) return join(git, ...LOCATION.gitSubdirectory);
  const pointer = /^gitdir:\s*(.+)$/mu.exec(readFileSync(git, "utf8"));
  return pointer === null ? undefined : join(resolve(root, pointer[1]!.trim()), ...LOCATION.gitSubdirectory);
}

function isEvent(value: unknown): value is LocalConfigEventV1 {
  const row = value as Record<string, unknown> | null;
  if (typeof row !== "object" || row === null || Array.isArray(row) || row.version !== 1 || typeof row.id !== "string" || !ID.test(row.id) || typeof row.at !== "string" || !INSTANT.test(row.at)) return false;
  const names = Object.keys(row).sort().join();
  if (row.type === "language-chosen") return names === "at,id,language,type,version" && typeof row.language === "string" && (CATALOG_LANGUAGES_V1 as readonly string[]).includes(row.language);
  return row.type === "language-cleared" && names === "at,id,type,version";
}

/** 📜️ Reads the events of a log text: a torn last line (a write that was cut off) is ignored, any other invalid line is refused with its number. */
export function parseLocalConfigLogV1(text: string, file: string): readonly LocalConfigEventV1[] {
  const lines = text.split("\n"), torn = text !== "" && !text.endsWith("\n"), events: LocalConfigEventV1[] = [];
  lines.forEach((line, index) => {
    if (line.trim() === "") return;
    let value: unknown;
    try { value = JSON.parse(line); } catch { if (torn && index === lines.length - 1) return; throw refuseConfigV1("invalid", "log-invalid", { file, line: index + 1 }); }
    if (!isEvent(value)) throw refuseConfigV1("invalid", "log-invalid", { file, line: index + 1 });
    events.push(value);
  });
  return Object.freeze(events);
}

/** 🧮️ Folds the events into the configuration: the last event of the language setting wins, a clearing event removes it. */
export function foldLocalConfigV1(events: readonly LocalConfigEventV1[]): LocalConfigV1 {
  let language: string | undefined;
  for (const event of events) language = event.type === "language-chosen" ? event.language : undefined;
  return Object.freeze({ version: 1, language, events: events.length });
}

/** 📖️ Reads the configuration of a log directory; a directory without a log is the empty configuration. */
export function readLocalConfigV1(directory: string | undefined): Readonly<{ config: LocalConfigV1; file: string | undefined }> {
  if (directory === undefined) return Object.freeze({ config: foldLocalConfigV1([]), file: undefined });
  const file = join(directory, LOCATION.file);
  return Object.freeze({ config: foldLocalConfigV1(existsSync(file) ? parseLocalConfigLogV1(readFileSync(file, "utf8"), file) : []), file });
}

/** 🏗️ Builds one event; the clock and the id are injectable so a test can pin them. */
export function localConfigEventV1(type: LocalConfigEventV1["type"], language?: string, clock: () => Date = () => new Date(), id: () => string = randomUUID): LocalConfigEventV1 {
  const base = { version: 1 as const, id: id(), at: clock().toISOString() };
  if (type === "language-cleared") return Object.freeze({ ...base, type });
  if (language === undefined || !(CATALOG_LANGUAGES_V1 as readonly string[]).includes(language)) throw refuseConfigV1("usage", "unknown-language", { value: String(language), choice: CATALOG_LANGUAGES_V1.join("|") });
  return Object.freeze({ ...base, type, language });
}

/** ➕️ Appends one event as one line and answers the log file; a write that was cut off (a last line without its newline) is an event that never happened, so it is cut away before the new event is appended. */
export function appendLocalConfigEventV1(directory: string, event: LocalConfigEventV1): string {
  mkdirSync(directory, { recursive: true });
  const file = join(directory, LOCATION.file);
  if (existsSync(file)) {
    const text = readFileSync(file, "utf8");
    if (text !== "" && !text.endsWith("\n")) truncateSync(file, Buffer.byteLength(text.slice(0, text.lastIndexOf("\n") + 1)));
  }
  appendFileSync(file, `${JSON.stringify(event)}\n`);
  return file;
}

/** 🗣️ The language the repo config names, or undefined; the CLI door reads it only when no flag and no SEMIO_LANGUAGE decide first. */
export function readConfiguredLanguageV1(root: string, environment: Readonly<Record<string, string | undefined>>): string | undefined {
  return readLocalConfigV1(localConfigDirectoryV1(root, environment)).config.language;
}
