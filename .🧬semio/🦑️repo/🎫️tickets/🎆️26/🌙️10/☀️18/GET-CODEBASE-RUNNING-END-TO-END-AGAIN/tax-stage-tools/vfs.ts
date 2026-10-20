// 🧮️ Virtual overlay over the live tree: stages reads and writes in memory and emits an anchored operation manifest. Nothing here writes into the repository.
import {createHash} from "node:crypto";
import {existsSync, mkdirSync, readFileSync, readdirSync, statSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";

export const REPO = "/Users/ueli/Documents/semio";
export const C = `${REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`;
export const TOOLS = `${C}/tax-stage-tools`;
export const STAGED = `${TOOLS}/staged`;
export const LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
export const sha = (data: string | Uint8Array): string => createHash("sha256").update(data).digest("hex");

export type Edit = {old: string; new: string; count: number};
export type Op = {path: string; kind: "write" | "delete" | "patch"; base: string | null; group: "core"; blob?: string; sha?: string; edits?: Edit[]};
const isManifest = (p: string): boolean => /(^|\/)package\.json$/u.test(p);

export class Vfs {
  readonly overlay = new Map<string, string | null>();
  readonly bases = new Map<string, string | null>();
  private readonly edits = new Map<string, Edit[]>();
  private readonly whole = new Set<string>();
  private base(p: string): string | null {
    if (!this.bases.has(p)) this.bases.set(p, existsSync(join(REPO, p)) && statSync(join(REPO, p)).isFile() ? sha(readFileSync(join(REPO, p))) : null);
    return this.bases.get(p)!;
  }
  exists(p: string): boolean {
    if (this.overlay.has(p)) return this.overlay.get(p) !== null;
    return existsSync(join(REPO, p)) && statSync(join(REPO, p)).isFile();
  }
  isDirectory(p: string): boolean {
    for (const [k, v] of this.overlay) if (v !== null && k.startsWith(p + "/")) return true;
    return existsSync(join(REPO, p)) && statSync(join(REPO, p)).isDirectory();
  }
  read(p: string): string {
    if (this.overlay.has(p)) { const v = this.overlay.get(p); if (v === null) throw new Error(`deleted: ${p}`); return v!; }
    return readFileSync(join(REPO, p), "utf8");
  }
  write(p: string, text: string): void { this.base(p); this.whole.add(p); this.overlay.set(p, text); }
  remove(p: string): void { this.base(p); this.whole.add(p); this.overlay.set(p, null); }
  list(dir: string): string[] {
    const out = new Set<string>();
    try { for (const name of readdirSync(join(REPO, dir))) out.add(name); } catch { /* absent */ }
    for (const [k, v] of this.overlay) if (k.startsWith(dir + "/")) { const name = k.slice(dir.length + 1).split("/")[0]!; if (v !== null) out.add(name); }
    for (const [k, v] of this.overlay) if (v === null && k.startsWith(dir + "/") && !k.slice(dir.length + 1).includes("/")) out.delete(k.slice(dir.length + 1));
    return [...out];
  }
  edit(p: string, from: string, to: string, count = 1): void {
    const text = this.read(p), found = text.split(from).length - 1;
    if (found !== count) throw new Error(`anchor in ${p} occurs ${found} times (expected ${count}): ${from.slice(0, 100)}`);
    this.base(p);
    this.overlay.set(p, text.split(from).join(to));
    this.edits.set(p, [...(this.edits.get(p) ?? []), {old: from, new: to, count}]);
  }
  changed(): string[] {
    return [...this.overlay.keys()].filter((p) => { const v = this.overlay.get(p); const b = this.base(p); return v === null ? b !== null : b === null || sha(v) !== b; }).sort();
  }
  deferred(): {path: string; before: string; after: string}[] {
    return this.changed().filter(isManifest).map((p) => ({path: p, before: readFileSync(join(REPO, p), "utf8"), after: this.overlay.get(p) ?? ""}));
  }
  ops(): Op[] {
    mkdirSync(`${STAGED}/blobs`, {recursive: true});
    return this.changed().filter((p) => !isManifest(p)).map((p) => {
      const v = this.overlay.get(p)!;
      const group = "core" as const;
      if (v === null) return {path: p, kind: "delete", base: this.base(p), group};
      if (!this.whole.has(p) && this.edits.has(p) && this.base(p) !== null) return {path: p, kind: "patch", base: this.base(p), group, sha: sha(v), edits: this.edits.get(p)!};
      const derived = this.base(p) !== null ? lineAnchoredEdits(readFileSync(join(REPO, p), "utf8"), v) : null;
      if (derived) return {path: p, kind: "patch", base: this.base(p), group, sha: sha(v), edits: derived};
      const h = sha(v);
      writeFileSync(`${STAGED}/blobs/${h}`, v);
      return {path: p, kind: "write", base: this.base(p), group, blob: h, sha: h};
    });
  }
}

const occurrences = (text: string, part: string): number => text.split(part).length - 1;

/** 🩹 Derives unique-anchor line patches from two texts through a line diff; null when the files are too large or no padding makes the patches replay exactly. */
export function lineAnchoredEdits(before: string, after: string, maximumHunks = 40): Edit[] | null {
  const a = before.split("\n"), b = after.split("\n");
  if (a.length > 4000 || b.length > 4000) return null;
  let head = 0;
  while (head < a.length && head < b.length && a[head] === b[head]) head++;
  let tail = 0;
  while (tail < a.length - head && tail < b.length - head && a[a.length - 1 - tail] === b[b.length - 1 - tail]) tail++;
  const n = a.length - head - tail, m = b.length - head - tail;
  const table = new Uint16Array((n + 1) * (m + 1)), width = m + 1;
  for (let i = n - 1; i >= 0; i--) for (let j = m - 1; j >= 0; j--) table[i * width + j] = a[head + i] === b[head + j] ? table[(i + 1) * width + j + 1]! + 1 : Math.max(table[(i + 1) * width + j]!, table[i * width + j + 1]!);
  const hunks: [number, number, number, number][] = [];
  let i = 0, j = 0, open: [number, number] | null = null;
  const close = (): void => { if (open) { hunks.push([head + open[0], head + i, head + open[1], head + j]); open = null; } };
  while (i < n || j < m) {
    if (i < n && j < m && a[head + i] === b[head + j]) { close(); i++; j++; continue; }
    open ??= [i, j];
    if (j < m && (i === n || table[i * width + j + 1]! >= table[(i + 1) * width + j]!)) j++; else i++;
  }
  close();
  if (!hunks.length || hunks.length > maximumHunks) return null;
  for (const pad of [0, 1, 2, 4, 8, 16, 32]) {
    const edits = hunks.map(([i1, i2, j1, j2]): Edit => ({old: a.slice(Math.max(0, i1 - pad), Math.min(a.length, i2 + pad)).join("\n"), new: b.slice(Math.max(0, j1 - pad), Math.min(b.length, j2 + pad)).join("\n"), count: 1}));
    let text = before, exact = true;
    for (const edit of edits) {
      if (!edit.old || occurrences(text, edit.old) !== 1) { exact = false; break; }
      text = text.split(edit.old).join(edit.new);
    }
    if (exact && text === after) return edits;
  }
  return null;
}

export const rel = (fromFile: string, toRepoPath: string): string => {
  const a = fromFile.split("/").slice(0, -1), b = toRepoPath.split("/");
  let i = 0;
  while (i < a.length && i < b.length - 1 && a[i] === b[i]) i++;
  const up = a.length - i, down = b.slice(i);
  return (up ? "../".repeat(up) : "./") + down.join("/");
};
