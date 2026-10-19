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

export type Op = {path: string; kind: "write" | "delete"; base: string | null; group: "core" | "package-json"; blob?: string; sha?: string};

export class Vfs {
  readonly overlay = new Map<string, string | null>();
  readonly bases = new Map<string, string | null>();
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
  write(p: string, text: string): void { this.base(p); this.overlay.set(p, text); }
  remove(p: string): void { this.base(p); this.overlay.set(p, null); }
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
    this.write(p, text.split(from).join(to));
  }
  changed(): string[] {
    return [...this.overlay.keys()].filter((p) => { const v = this.overlay.get(p); const b = this.base(p); return v === null ? b !== null : b === null || sha(v) !== b; }).sort();
  }
  ops(): Op[] {
    mkdirSync(`${STAGED}/blobs`, {recursive: true});
    return this.changed().map((p) => {
      const v = this.overlay.get(p)!;
      const group = /(^|\/)package\.json$/u.test(p) ? "package-json" as const : "core" as const;
      if (v === null) return {path: p, kind: "delete", base: this.base(p), group};
      const h = sha(v);
      writeFileSync(`${STAGED}/blobs/${h}`, v);
      return {path: p, kind: "write", base: this.base(p), group, blob: h, sha: h};
    });
  }
}

export const rel = (fromFile: string, toRepoPath: string): string => {
  const a = fromFile.split("/").slice(0, -1), b = toRepoPath.split("/");
  let i = 0;
  while (i < a.length && i < b.length - 1 && a[i] === b[i]) i++;
  const up = a.length - i, down = b.slice(i);
  return (up ? "../".repeat(up) : "./") + down.join("/");
};
