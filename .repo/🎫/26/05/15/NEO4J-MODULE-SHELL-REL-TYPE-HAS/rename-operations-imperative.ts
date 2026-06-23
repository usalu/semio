#!/usr/bin/env bun
/**
 * @emoji 🏷️ Rewrites past-tense concrete `Operation` GraphQL names to imperative verbs and related Input/Edge/Connection ladders (same stem).
 */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = join(import.meta.dir, "..", "..", "..", "..", "..", "..");
const goldenPath = join(REPO, "compose", "client", "schema", "graphql", "schema.golden.graphql");

/** @emoji 🧭 Past-participle operation stem → imperative stem (PascalCase prefix swap). */
export function imperativeOperationStem(past: string): string {
  const rules: [RegExp, string][] = [
    [/^Created/, "Create"],
    [/^Renamed/, "Rename"],
    [/^Updated/, "Update"],
    [/^Added/, "Add"],
    [/^Removed/, "Remove"],
    [/^Deleted/, "Delete"],
    [/^Changed/, "Change"],
    [/^Moved/, "Move"],
    [/^Fixed/, "Fix"],
    [/^Dragged/, "Drag"],
    [/^Flattened/, "Flatten"],
  ];
  for (const [re, rep] of rules) {
    if (re.test(past)) {
      return past.replace(re, rep);
    }
  }
  return past;
}

function collectOperationNames(golden: string): string[] {
  const re = /^type ([A-Za-z0-9_]+) implements Operation/gm;
  const names: string[] = [];
  for (const m of golden.matchAll(re)) {
    names.push(m[1]!);
  }
  return names;
}

/** @emoji 🧭 Build (from,to) pairs longest-first so `XInput` replaces before bare `X`. */
export function buildIdentifierReplacements(operationNames: readonly string[]): { from: string; to: string }[] {
  const pairs: { from: string; to: string }[] = [];
  const suffixes = ["InputConnection", "InputEdge", "Connection", "Edge", "Input", ""] as const;
  for (const old of operationNames) {
    const neu = imperativeOperationStem(old);
    if (neu === old) {
      continue;
    }
    for (const suf of suffixes) {
      const from = old + suf;
      const to = neu + suf;
      pairs.push({ from, to });
    }
  }
  pairs.sort((a, b) => b.from.length - a.from.length || b.to.length - a.to.length);
  return pairs;
}

/** @emoji 🧭 Adds `Gap…` schema-gap Rust identifiers mirroring each concrete operation rename. */
export function expandPairsWithSchemaGapPrefix(pairs: readonly { from: string; to: string }[]): { from: string; to: string }[] {
  const seen = new Set(pairs.map((p) => p.from));
  const extra: { from: string; to: string }[] = [];
  for (const p of pairs) {
    if (p.from.startsWith("Gap")) {
      continue;
    }
    const gf = `Gap${p.from}`;
    const gt = `Gap${p.to}`;
    if (!seen.has(gf)) {
      seen.add(gf);
      extra.push({ from: gf, to: gt });
    }
  }
  const merged = [...pairs, ...extra];
  merged.sort((a, b) => b.from.length - a.from.length || b.to.length - a.to.length);
  return merged;
}

function escapeRegExpLiteral(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/** @emoji 🧭 Identifier-safe replace so `FixedPiece` does not corrupt `CreateFixedPiece`. */
export function applyReplacements(content: string, pairs: readonly { from: string; to: string }[]): string {
  let s = content;
  for (const { from, to } of pairs) {
    const re = new RegExp(`(?<![A-Za-z0-9_])${escapeRegExpLiteral(from)}(?![A-Za-z0-9_])`, "g");
    s = s.replace(re, to);
  }
  return s;
}

function loadPairsFromLog(logPath: string): { from: string; to: string }[] {
  const text = readFileSync(logPath, "utf8");
  const out: { from: string; to: string }[] = [];
  for (const line of text.split(/\n/)) {
    const m = /^(.+?) → (.+)$/.exec(line.trim());
    if (m) {
      out.push({ from: m[1]!, to: m[2]! });
    }
  }
  out.sort((a, b) => b.from.length - a.from.length || b.to.length - a.to.length);
  return out;
}

const extraSyncPaths = [
  join(REPO, "compose", "client", "schema", "graphql", "schema.graphql"),
  join(REPO, "compose", "client", "lib", "js", "index.ts"),
  join(REPO, "compose", "client", "lib", "rs", "lib.rs"),
];

function main(): void {
  const logPath = join(import.meta.dir, "rename-operations-imperative.log.txt");
  const golden = readFileSync(goldenPath, "utf8");
  let pairs = buildIdentifierReplacements(collectOperationNames(golden));
  if (pairs.length === 0 && existsSync(logPath)) {
    pairs = loadPairsFromLog(logPath);
  }
  pairs = expandPairsWithSchemaGapPrefix(pairs);
  if (pairs.length === 0) {
    console.log("[rename-ops] nothing to do (no pairs, no log)");
    return;
  }
  writeFileSync(logPath, pairs.map((p) => `${p.from} → ${p.to}`).join("\n"), "utf8");
  const nextGolden = applyReplacements(golden, pairs);
  if (nextGolden !== golden) {
    writeFileSync(goldenPath, nextGolden, "utf8");
    console.log(`[rename-ops] wrote ${goldenPath}`);
  } else {
    console.log("[rename-ops] golden SDL already matches pairs");
  }
  for (const p of extraSyncPaths) {
    const before = readFileSync(p, "utf8");
    const after = applyReplacements(before, pairs);
    if (after !== before) {
      writeFileSync(p, after, "utf8");
      console.log(`[rename-ops] wrote ${p}`);
    }
  }
  console.log(`[rename-ops] logged ${pairs.length} identifier pairs in ${logPath}`);
}

main();
