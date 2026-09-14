#!/usr/bin/env bun
/** 🏗️ Extracts the statute and policy catalog of `📜️statutes` from the Go snapshot of `🧩️component.go` into `🧬️schema/🔣️statutes.json`. */
import { readFileSync, writeFileSync } from "node:fs";

const TICKET = import.meta.dir;
const SNAPSHOT = `${TICKET}/🗑️generated/go-snapshot/client/🧩️component.go`;
const OUT = `${TICKET}/../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧬️schema/🔣️statutes.json`;

const source = readFileSync(SNAPSHOT, "utf8");
const lines = source.split("\n");

const constById = new Map<string, string>();
const idByConst = new Map<string, string>();
for (const line of lines) {
  const m = line.match(/^\t(Breach\w+)\s+Statute = "([^"]+)"$/);
  if (m) {
    constById.set(m[1], m[2]);
    idByConst.set(m[1], m[2]);
  }
}

const priorityOf = (raw: string): string => raw.replace(/^BreachPriority/, "").toLowerCase();

type Meta = { id: string; priority: string; reason: string; solution: string; autofixable: boolean };
const metas = new Map<string, Meta>();

const tableStart = lines.findIndex((l) => l.startsWith("var statuteInfoTable = map[Statute]StatuteMeta{"));
if (tableStart < 0) throw new Error("statuteInfoTable not found");
let depth = 1;
let i = tableStart + 1;
while (depth > 0 && i < lines.length) {
  const line = lines[i];
  const pathEmoji = line.match(/^\t(Breach\w+):\s*pathEmojiStatuteMeta\(Breach\w+\),$/);
  if (pathEmoji) {
    const id = constById.get(pathEmoji[1]);
    if (!id) throw new Error(`unknown const ${pathEmoji[1]}`);
    metas.set(id, {
      id,
      priority: "high",
      reason: "Every non-reserved file and folder requires exactly one handpicked, meaningful emoji unique among all siblings; reserved filenames remain literal",
      solution: "Handpick one meaningful sibling-unique emoji, or restore the unprefixed reserved name, and update only resolved path references",
      autofixable: false,
    });
    i += 1;
    continue;
  }
  const entry = line.match(/^\t(Breach\w+):\s*\{$/);
  if (entry) {
    const id = constById.get(entry[1]);
    if (!id) throw new Error(`unknown const ${entry[1]}`);
    const meta: Meta = { id, priority: "low", reason: "", solution: "", autofixable: false };
    let j = i + 1;
    for (; j < lines.length && !/^\t\},$/.test(lines[j]); j += 1) {
      const p = lines[j].match(/^\t\tPriority:\s*(BreachPriority\w+),$/);
      if (p) meta.priority = priorityOf(p[1]);
      const r = lines[j].match(/^\t\tReason:\s*"((?:[^"\\]|\\.)*)",$/);
      if (r) meta.reason = JSON.parse(`"${r[1]}"`);
      const s = lines[j].match(/^\t\tSolution:\s*"((?:[^"\\]|\\.)*)",$/);
      if (s) meta.solution = JSON.parse(`"${s[1]}"`);
      const a = lines[j].match(/^\t\tAutofixable:\s*(true|false),$/);
      if (a) meta.autofixable = a[1] === "true";
    }
    metas.set(id, meta);
    i = j + 1;
    continue;
  }
  if (/^\}$/.test(line)) depth = 0;
  i += 1;
}

for (const id of constById.values()) {
  if (!metas.has(id)) {
    metas.set(id, { id, priority: "low", reason: "Unknown breach", solution: "Fix the breach", autofixable: false });
  }
}

type Territory = { name: string; description: string; scopes?: string[]; groups?: Territory[]; kinds?: string[] };
type Policy = { id: string; name: string; description: string; scopes: string[]; priority: string; groups: Territory[] };

const policiesStart = lines.findIndex((l) => l === "var policies = []PolicyDef{");
if (policiesStart < 0) throw new Error("policies not found");

let cursor = policiesStart + 1;
const literal = (raw: string) => JSON.parse(`"${raw}"`);

const indentOf = (line: string) => (line.match(/^\t*/) ?? [""])[0].length;

function parseStringList(startIndex: number): { value: string[]; next: number } {
  const inline = lines[startIndex].match(/\[\]string\{(.*)\},$/);
  if (inline && inline[1].trim() !== "") {
    return { value: [...inline[1].matchAll(/"((?:[^"\\]|\\.)*)"/g)].map((m) => literal(m[1])), next: startIndex + 1 };
  }
  if (inline) return { value: [], next: startIndex + 1 };
  const value: string[] = [];
  let j = startIndex + 1;
  for (; j < lines.length && !/^\t*\},$/.test(lines[j]); j += 1) {
    const m = lines[j].match(/"((?:[^"\\]|\\.)*)"/);
    if (m) value.push(literal(m[1]));
  }
  return { value, next: j + 1 };
}

function parseKinds(startIndex: number): { value: string[]; next: number } {
  const inline = lines[startIndex].match(/\[\]Statute\{\},$/);
  if (inline) return { value: [], next: startIndex + 1 };
  const value: string[] = [];
  let j = startIndex + 1;
  for (; j < lines.length && !/^\t*\},$/.test(lines[j]); j += 1) {
    const m = lines[j].match(/(Breach\w+),/);
    if (m) {
      const id = constById.get(m[1]);
      if (!id) throw new Error(`unknown statute const ${m[1]}`);
      value.push(id);
    }
  }
  return { value, next: j + 1 };
}

function parseTerritories(startIndex: number): { value: Territory[]; next: number } {
  if (/\[\]Territory\{\},$/.test(lines[startIndex])) return { value: [], next: startIndex + 1 };
  const value: Territory[] = [];
  const openIndent = indentOf(lines[startIndex]);
  let j = startIndex + 1;
  while (j < lines.length) {
    if (indentOf(lines[j]) === openIndent && /^\t*\},$/.test(lines[j])) return { value, next: j + 1 };
    if (/^\t*\{$/.test(lines[j])) {
      const parsed = parseTerritory(j);
      value.push(parsed.value);
      j = parsed.next;
      continue;
    }
    j += 1;
  }
  throw new Error("unterminated territory list");
}

function parseTerritory(startIndex: number): { value: Territory; next: number } {
  const territory: Territory = { name: "", description: "" };
  const openIndent = indentOf(lines[startIndex]);
  let j = startIndex + 1;
  while (j < lines.length) {
    const line = lines[j];
    if (indentOf(line) === openIndent && /^\t*\},$/.test(line)) return { value: territory, next: j + 1 };
    const name = line.match(/^\t*Name:\s*"((?:[^"\\]|\\.)*)",$/);
    if (name) { territory.name = literal(name[1]); j += 1; continue; }
    const description = line.match(/^\t*Description:\s*"((?:[^"\\]|\\.)*)",$/);
    if (description) { territory.description = literal(description[1]); j += 1; continue; }
    if (/^\t*Scopes:\s*\[\]string\{/.test(line)) { const r = parseStringList(j); territory.scopes = r.value; j = r.next; continue; }
    if (/^\t*Groups:\s*\[\]Territory\{/.test(line)) { const r = parseTerritories(j); territory.groups = r.value; j = r.next; continue; }
    if (/^\t*Kinds:\s*\[\]Statute\{/.test(line)) { const r = parseKinds(j); territory.kinds = r.value; j = r.next; continue; }
    j += 1;
  }
  throw new Error("unterminated territory");
}

const policyList: Policy[] = [];
while (cursor < lines.length && lines[cursor] !== "}") {
  if (!/^\t\{$/.test(lines[cursor])) { cursor += 1; continue; }
  const policy: Policy = { id: "", name: "", description: "", scopes: [], priority: "low", groups: [] };
  let j = cursor + 1;
  while (j < lines.length && !/^\t\},$/.test(lines[j])) {
    const line = lines[j];
    const id = line.match(/^\t\tID:\s*"((?:[^"\\]|\\.)*)",$/);
    if (id) { policy.id = literal(id[1]); j += 1; continue; }
    const name = line.match(/^\t\tName:\s*"((?:[^"\\]|\\.)*)",$/);
    if (name) { policy.name = literal(name[1]); j += 1; continue; }
    const description = line.match(/^\t\tDescription:\s*"((?:[^"\\]|\\.)*)",$/);
    if (description) { policy.description = literal(description[1]); j += 1; continue; }
    const priority = line.match(/^\t\tPriority:\s*(BreachPriority\w+),$/);
    if (priority) { policy.priority = priorityOf(priority[1]); j += 1; continue; }
    if (/^\t\tScopes:\s*\[\]string\{/.test(line)) { const r = parseStringList(j); policy.scopes = r.value; j = r.next; continue; }
    if (/^\t\tGroups:\s*\[\]Territory\{/.test(line)) { const r = parseTerritories(j); policy.groups = r.value; j = r.next; continue; }
    j += 1;
  }
  policyList.push(policy);
  cursor = j + 1;
}

const ordered = [...constById.values()];
const statutes = ordered.map((id) => {
  const meta = metas.get(id)!;
  return { id: meta.id, priority: meta.priority, reason: meta.reason, solution: meta.solution, autofixable: meta.autofixable };
});

const claimed = new Set<string>();
const walk = (t: Territory) => {
  for (const kind of t.kinds ?? []) claimed.add(kind);
  for (const child of t.groups ?? []) walk(child);
};
for (const policy of policyList) for (const group of policy.groups) walk(group);

const policyIdOf = new Map<string, string>();
for (const policy of policyList) {
  const seen = new Set<string>();
  const mark = (t: Territory) => {
    for (const kind of t.kinds ?? []) seen.add(kind);
    for (const child of t.groups ?? []) mark(child);
  };
  for (const group of policy.groups) mark(group);
  for (const kind of seen) if (!policyIdOf.has(kind)) policyIdOf.set(kind, policy.id);
}

const document = {
  $schema: "./🔣️.json",
  schemaVersion: 1,
  statutes: statutes.map((s) => ({ kind: s.id, policyId: policyIdOf.get(s.id) ?? "", priority: s.priority, reason: s.reason, solution: s.solution, autofixable: s.autofixable })),
  policies: policyList.map((p) => ({ id: p.id, name: p.name, description: p.description, scopes: p.scopes, priority: p.priority, groups: p.groups, statutes: null })),
};

writeFileSync(OUT, `${JSON.stringify(document, null, 2)}\n`, "utf8");
console.error(`statutes=${statutes.length} policies=${policyList.length} claimed=${claimed.size} unclaimed=${statutes.filter((s) => !claimed.has(s.id)).map((s) => s.id).join(",")}`);
