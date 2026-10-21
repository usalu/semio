import { readFileSync } from "node:fs";
import { isAbsolute, join } from "node:path";
import type { ToolDigestSourceV1, ToolEntryV1, ToolSourceV1 } from "../📖️registry/🟦️.ts";

/** 🔐️ One distribution of a pinned tool for a platform and architecture. */
export type ToolDigestV1 = Readonly<{ platform: string; architecture: string; sha256: string; name: string; bytes: number | null; extras: Readonly<Record<string, unknown>> }>;
/** 📌️ What the repository pins about a tool, read from its existing pin file. */
export type ToolPinV1 = Readonly<{ state: ToolEntryV1["pin"]["state"]; version: string | null; minimum: string | null; digests: readonly ToolDigestV1[] }>;
/** 🔌️ What reading pins touches; tests replace it. */
export type ToolPinPortV1 = Readonly<{ read(path: string): string | undefined }>;

/** 📂️ Reads a file as text, or undefined when it is absent. */
export const systemToolPinPortV1: ToolPinPortV1 = Object.freeze({ read: path => { try { return readFileSync(path, "utf8"); } catch { return undefined; } } });

/** 🧭️ Follows a JSON pointer (`/a/b/0`, `~0` and `~1` escapes) through a parsed document. */
export function toolPointerV1(document: unknown, pointer: string): unknown {
  let value: unknown = document;
  for (const part of pointer.split("/").slice(1).map(segment => segment.replace(/~1/gu, "/").replace(/~0/gu, "~"))) {
    if (typeof value !== "object" || value === null) return undefined;
    value = (value as Record<string, unknown>)[part];
  }
  return value;
}

const textOf = (value: unknown): string | null => {
  if (typeof value === "string") return value;
  if (typeof value === "number") return String(value);
  if (typeof value === "object" && value !== null && ["major", "minor", "patch"].every(key => key in value)) return [(value as Record<string, unknown>).major, (value as Record<string, unknown>).minor, (value as Record<string, unknown>).patch].join(".");
  return null;
};

const matches = (row: unknown, where: Readonly<Record<string, unknown>>): boolean => typeof row === "object" && row !== null && Object.entries(where).every(([key, expected]) => (row as Record<string, unknown>)[key] === expected);

const tomlValue = (text: string, table: string, key: string): string | null => {
  let current = "";
  for (const raw of text.split(/\r?\n/u)) {
    const line = raw.trim();
    const header = /^\[([^\]]+)\]$/u.exec(line);
    if (header) { current = header[1]!.trim(); continue; }
    const pair = /^([A-Za-z0-9_.-]+)\s*=\s*(?:"([^"]*)"|'([^']*)')/u.exec(line);
    if (pair && current === table && pair[1] === key) return pair[2] ?? pair[3] ?? null;
  }
  return null;
};

/** 🔢️ Reads one version or constraint from its source file; null when the file or the value is absent. */
export function readToolSourceV1(source: ToolSourceV1 | undefined, root: string, port: ToolPinPortV1 = systemToolPinPortV1): string | null {
  if (source === undefined) return null;
  const text = port.read(isAbsolute(source.file) ? source.file : join(root, source.file));
  if (text === undefined) return null;
  if (source.format === "text") return text.split(/\r?\n/u)[0]!.trim() || null;
  if (source.format === "toml") return tomlValue(text, source.table, source.key);
  let document: unknown;
  try { document = JSON.parse(text); } catch { return null; }
  if (source.format === "json") return textOf(toolPointerV1(document, source.pointer));
  const rows = toolPointerV1(document, source.pointer);
  const row = Array.isArray(rows) ? rows.find(candidate => matches(candidate, source.where)) : undefined;
  return row === undefined ? null : textOf((row as Record<string, unknown>)[source.field]);
}

const hostPart = (row: Record<string, unknown>, source: ToolDigestSourceV1, index: 0 | 1): string => {
  if (source.host !== undefined) { const host = String(row[source.host]), cut = host.indexOf("-"); return cut < 0 ? (index === 0 ? host : "any") : index === 0 ? host.slice(0, cut) : host.slice(cut + 1); }
  return String(row[(index === 0 ? source.platform : source.architecture)!]);
};

/** 🧱️ Reads the digest table of a pinned tool: every row that matches the filter, with the named fields. */
export function readToolDigestsV1(source: ToolDigestSourceV1 | undefined, root: string, port: ToolPinPortV1 = systemToolPinPortV1): readonly ToolDigestV1[] {
  if (source === undefined) return [];
  const text = port.read(isAbsolute(source.file) ? source.file : join(root, source.file));
  if (text === undefined) return [];
  let rows: unknown;
  try { rows = toolPointerV1(JSON.parse(text), source.pointer); } catch { return []; }
  if (!Array.isArray(rows)) return [];
  return rows.filter(row => matches(row, source.where)).map((row: Record<string, unknown>) => Object.freeze({
    platform: hostPart(row, source, 0), architecture: hostPart(row, source, 1), sha256: String(row[source.sha256]), name: String(row[source.name]),
    bytes: source.bytes !== undefined && typeof row[source.bytes] === "number" ? row[source.bytes] as number : null,
    extras: Object.freeze(Object.fromEntries((source.extras ?? []).map(key => [key, row[key]]))),
  }));
}

/** 🧵️ Reads the whole pin of a tool from the files the repository already keeps. */
export function readToolPinV1(entry: ToolEntryV1, root: string, port: ToolPinPortV1 = systemToolPinPortV1): ToolPinV1 {
  return Object.freeze({ state: entry.pin.state, version: readToolSourceV1(entry.pin.version, root, port), minimum: readToolSourceV1(entry.pin.minimum, root, port), digests: readToolDigestsV1(entry.pin.digests, root, port) });
}

/** 🖥️ Picks the digest for a host; `any` rows match every platform or architecture, and `extras` select between variants such as musl. */
export function toolDigestForHostV1(pin: ToolPinV1, platform: string, architecture: string, extras: Readonly<Record<string, unknown>> = {}): ToolDigestV1 | undefined {
  return pin.digests.find(row => (row.platform === platform || row.platform === "any") && (row.architecture === architecture || row.architecture === "any") && Object.entries(extras).every(([key, value]) => row.extras[key] === value));
}
