/** 🧷 Applies a delta plan (new files, manifest operations, text and specifier edits, file moves/deletions) to a root, with the guards of the exports-stage applier. */
import { copyFileSync, existsSync, mkdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { createHash } from "node:crypto";
import type { ManifestOp, NewFile, Edit, FileOp, JsonOp } from "./plan.ts";

export type PlanJson = { manifestOps: ManifestOp[]; newFiles: NewFile[]; edits: Edit[]; fileOps: FileOp[] };
const sha = (b: Buffer | string): string => createHash("sha256").update(b).digest("hex");
const normalizeEmpty = (t: string): string => t.replace(/\{\s*\}/gu, "{}").replace(/\[\s*\]/gu, "[]");

const isSorted = (xs: string[]): boolean => xs.every((x, i) => i === 0 || xs[i - 1]! < x);
const walkTo = (root: unknown, pointer: string[]): unknown => pointer.reduce<unknown>((o, k) => (o as Record<string, unknown> | undefined)?.[k], root);

/** 🧩 Applies structured edits to a JSON document that round-trips as 2-space JSON plus a final newline; sorted arrays and dictionaries stay sorted. */
export function applyJsonOps(text: string, ops: JsonOp[], name: string, problems: string[]): string {
  const root = JSON.parse(text);
  if (JSON.stringify(root, null, 2) + "\n" !== text) { problems.push(`json does not round-trip: ${name}`); return text; }
  for (const op of ops) {
    if (op.kind === "retarget") {
      const hits = new Map<string, number>();
      const walk = (v: unknown): unknown => {
        if (typeof v === "string") { const to = op.map[v]; if (to === undefined) return v; hits.set(v, (hits.get(v) ?? 0) + 1); return to; }
        if (Array.isArray(v)) {
          const strings = v.every(x => typeof x === "string");
          const next = v.map(walk);
          if (!strings) return next;
          const changed = next.some((x, i) => x !== v[i]);
          if (!changed) return next;
          const uniq = [...new Set(next as string[])];
          return isSorted(v as string[]) ? uniq.sort((a, b) => (a < b ? -1 : a > b ? 1 : 0)) : uniq;
        }
        if (v && typeof v === "object") { const o = v as Record<string, unknown>; for (const k of Object.keys(o)) o[k] = walk(o[k]); return o; }
        return v;
      };
      const out = walk(root); Object.assign(root, out as object);
      for (const k of Object.keys(op.map)) if (!hits.has(k)) problems.push(`${name}: retarget source absent: ${k.slice(0, 100)}`);
    } else if (op.kind === "renameKeys") {
      const dict = walkTo(root, op.pointer) as Record<string, Record<string, unknown>> | undefined;
      if (!dict || typeof dict !== "object") { problems.push(`${name}: pointer absent: ${op.pointer.join("/")}`); continue; }
      const keys = Object.keys(dict), sorted = isSorted(keys);
      let entries = Object.entries(dict);
      for (const [from, to] of Object.entries(op.map)) {
        const i = entries.findIndex(([k]) => k === from);
        if (i < 0) { problems.push(`${name}: key to rename is absent: ${from}`); continue; }
        if (entries.some(([k]) => k === to.to)) { problems.push(`${name}: key exists: ${to.to}`); continue; }
        entries[i] = [to.to, { ...entries[i]![1], ...(to.set ?? {}) }];
      }
      if (sorted) entries = entries.sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0));
      for (const k of keys) delete dict[k];
      for (const [k, v] of entries) dict[k] = v;
    } else if (op.kind === "arrayRemove") {
      const arr = walkTo(root, op.pointer) as string[] | undefined;
      if (!Array.isArray(arr)) { problems.push(`${name}: pointer absent: ${op.pointer.join("/")}`); continue; }
      for (const v of op.values) { const i = arr.indexOf(v); if (i < 0) problems.push(`${name}: value to remove is absent: ${v}`); else arr.splice(i, 1); }
    } else if (op.kind === "arrayInsert") {
      const arr = walkTo(root, op.pointer) as string[] | undefined;
      if (!Array.isArray(arr)) { problems.push(`${name}: pointer absent: ${op.pointer.join("/")}`); continue; }
      const at = op.after === undefined ? arr.length : arr.indexOf(op.after) + 1;
      if (op.after !== undefined && at === 0) { problems.push(`${name}: insert anchor absent: ${op.after}`); continue; }
      arr.splice(at, 0, ...op.values.filter(v => !arr.includes(v)));
    } else {
      const arr = walkTo(root, op.pointer) as Record<string, unknown>[] | undefined;
      if (!Array.isArray(arr)) { problems.push(`${name}: pointer absent: ${op.pointer.join("/")}`); continue; }
      const i = arr.findIndex(r => Object.entries(op.where).every(([k, v]) => r[k] === v));
      if (i < 0) problems.push(`${name}: row to drop is absent: ${JSON.stringify(op.where)}`); else arr.splice(i, 1);
    }
  }
  return JSON.stringify(root, null, 2) + "\n";
}

export function planManifest(text: string, name: string, ops: ManifestOp[], problems: string[]): string {
  const indent = /^(\s+)"/mu.exec(text)?.[1] ?? "  ";
  const obj = JSON.parse(text);
  const re = JSON.stringify(obj, null, indent) + (text.endsWith("\n") ? "\n" : "");
  if (re !== text && normalizeEmpty(re) !== normalizeEmpty(text)) { problems.push(`manifest does not round-trip: ${name}`); return text; }
  const order = (a: ManifestOp, b: ManifestOp): number => (a.op === b.op ? (a.field + a.key < b.field + b.key ? -1 : 1) : a.op === "remove" ? -1 : 1);
  for (const op of [...ops].sort(order)) {
    if (op.field === "workspaces") {
      const list: string[] = obj.workspaces ?? (obj.workspaces = []);
      if (op.op === "remove") { const i = list.indexOf(op.key); if (i < 0) problems.push(`${name}: workspaces entry to remove is absent: ${op.key}`); else list.splice(i, 1); continue; }
      if (list.includes(op.key)) continue;
      const sorted = list.every((k, i) => i === 0 || list[i - 1]! < k);
      if (sorted) { const at = list.findIndex(k => k > op.key); if (at < 0) list.push(op.key); else list.splice(at, 0, op.key); } else list.push(op.key);
      continue;
    }
    let bag = obj[op.field] as Record<string, unknown> | undefined;
    if (op.op === "remove") {
      if (!bag || !(op.key in bag)) { problems.push(`${name}: ${op.field}[${op.key}] to remove is absent`); continue; }
      if (JSON.stringify(bag[op.key]) !== JSON.stringify(op.value)) { problems.push(`${name}: ${op.field}[${op.key}] changed (${JSON.stringify(bag[op.key])} vs ${JSON.stringify(op.value)})`); continue; }
      delete bag[op.key]; continue;
    }
    if (!bag) {
      bag = {};
      const top = Object.entries(obj); for (const [k] of top) delete obj[k];
      const have = Object.fromEntries(top);
      const afterKey = op.field === "devDependencies" && "dependencies" in have ? "dependencies" : "exports" in have ? "exports" : top[top.length - 1]![0];
      for (const [k, v] of top) { obj[k] = v; if (k === afterKey) obj[op.field] = bag; }
      if (!(op.field in obj)) obj[op.field] = bag;
    }
    if (op.key in bag) { if (JSON.stringify(bag[op.key]) !== JSON.stringify(op.value)) problems.push(`${name}: ${op.field}[${op.key}] exists with ${JSON.stringify(bag[op.key])}`); continue; }
    const keys = Object.keys(bag);
    const sorted = keys.every((k, i) => i === 0 || keys[i - 1]! < k);
    const next: [string, unknown][] = Object.entries(bag);
    if (sorted) { const at = next.findIndex(([k]) => k > op.key); if (at < 0) next.push([op.key, op.value]); else next.splice(at, 0, [op.key, op.value]); } else next.push([op.key, op.value]);
    for (const k of keys) delete bag[k];
    for (const [k, v] of next) bag[k] = v;
  }
  return JSON.stringify(obj, null, indent) + (text.endsWith("\n") ? "\n" : "");
}

export function applyPlan(plan: PlanJson, root: string, opts: { write: boolean; backup?: string }): { problems: string[]; touched: number } {
  const abs = (p: string): string => resolve(root, p);
  const problems: string[] = [];
  const texts = new Map<string, string>();
  const get = (p: string): string | undefined => { if (texts.has(p)) return texts.get(p); if (!existsSync(abs(p))) return undefined; const t = readFileSync(abs(p), "utf8"); texts.set(p, t); return t; };
  const moved = new Map(plan.fileOps.filter(o => o.op === "move").map(o => [o.path, o.to!]));
  for (const o of plan.fileOps) {
    if (!existsSync(abs(o.path))) { problems.push(`${o.op} source vanished: ${o.path}`); continue; }
    if (sha(readFileSync(abs(o.path))) !== o.sha256) problems.push(`${o.op} source changed since staging: ${o.path}`);
    if (o.op === "move" && existsSync(abs(o.to!))) problems.push(`move destination exists: ${o.to}`);
  }
  for (const f of plan.newFiles) if (existsSync(abs(f.path)) && readFileSync(abs(f.path), "utf8") !== f.content) problems.push(`file to create exists with other content: ${f.path}`);
  const byManifest = new Map<string, ManifestOp[]>();
  for (const o of plan.manifestOps) (byManifest.get(o.packageJson) ?? byManifest.set(o.packageJson, []).get(o.packageJson)!).push(o);
  const out = new Map<string, string>();
  for (const [pj, ops] of byManifest) { const t = get(pj); if (t === undefined) { problems.push(`manifest vanished: ${pj}`); continue; } out.set(pj, planManifest(t, pj, ops, problems)); }
  const editsByFile = new Map<string, Edit[]>();
  for (const e of plan.edits) (editsByFile.get(e.file) ?? editsByFile.set(e.file, []).get(e.file)!).push(e);
  for (const [file, edits] of editsByFile) {
    const src = moved.has(file) ? file : file;
    let t = get(src);
    if (t === undefined) { problems.push(`edit target vanished: ${file}`); continue; }
    for (const e of edits) {
      if (e.mode === "json") { t = applyJsonOps(t, e.ops ?? [], file, problems); continue; }
      if (e.mode === "append") { t = t.replace(/\n*$/u, "\n") + e.to; continue; }
      if (e.mode === "specifier") {
        const n = t.split(`"${e.from}"`).length - 1 + t.split(`'${e.from}'`).length - 1;
        if (!n) { problems.push(`${file}: specifier ${e.from} not found`); continue; }
        t = t.replaceAll(`"${e.from}"`, `"${e.to}"`).replaceAll(`'${e.from}'`, `'${e.to}'`);
      } else {
        const i = t.indexOf(e.from);
        if (i < 0) { problems.push(`${file}: text edit anchor not found: ${e.from.slice(0, 90)}`); continue; }
        if (t.indexOf(e.from, i + 1) >= 0) { problems.push(`${file}: text edit anchor not unique: ${e.from.slice(0, 90)}`); continue; }
        t = t.slice(0, i) + e.to + t.slice(i + e.from.length);
      }
    }
    out.set(file, t);
  }
  if (!opts.write || problems.length) return { problems, touched: out.size + plan.newFiles.length + plan.fileOps.length };
  const save = (p: string): void => { if (!opts.backup || !existsSync(abs(p))) return; const name = sha(p).slice(0, 24); mkdirSync(opts.backup, { recursive: true }); copyFileSync(abs(p), resolve(opts.backup, name + ".bak")); const idx = resolve(opts.backup, "index.json"); const cur = existsSync(idx) ? JSON.parse(readFileSync(idx, "utf8")) : {}; cur[name] = p; writeFileSync(idx, JSON.stringify(cur, null, 1)); };
  const write = (p: string, content: string): void => { mkdirSync(dirname(abs(p)), { recursive: true }); const tmp = abs(p) + ".tmp-cycle-break"; writeFileSync(tmp, content); renameSync(tmp, abs(p)); };
  for (const f of plan.newFiles) if (!existsSync(abs(f.path))) write(f.path, f.content);
  for (const [p, t] of out) { save(p); write(p, t); }
  for (const o of plan.fileOps) {
    if (o.op === "delete") { save(o.path); rmSync(abs(o.path)); }
    else { save(o.path); mkdirSync(dirname(abs(o.to!)), { recursive: true }); renameSync(abs(o.path), abs(o.to!)); }
  }
  return { problems, touched: out.size + plan.newFiles.length + plan.fileOps.length };
}
