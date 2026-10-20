/** 🗒️ Delta plan: manifest operations, new manifests, text edits and file operations of the cycle-break waves, deduplicated and serializable. */
import { readFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";
import { createHash } from "node:crypto";
import { ROOT } from "./lib.ts";

export type ManifestOp = { op: "add" | "remove"; packageJson: string; package: string; field: "exports" | "dependencies" | "devDependencies" | "workspaces"; key: string; value: string; wave: string; why: string };
export type NewFile = { path: string; content: string; wave: string; why: string };
export type JsonOp =
  | { kind: "retarget"; map: Record<string, string> }
  | { kind: "renameKeys"; pointer: string[]; map: Record<string, { to: string; set?: Record<string, string> }> }
  | { kind: "arrayRemove"; pointer: string[]; values: string[] }
  | { kind: "arrayInsert"; pointer: string[]; after?: string; values: string[] }
  | { kind: "dropRows"; pointer: string[]; where: Record<string, string> };
export type Edit = { file: string; from: string; to: string; mode: "specifier" | "text" | "append" | "json"; ops?: JsonOp[]; wave: string; why: string };
export type FileOp = { op: "delete" | "move"; path: string; to?: string; sha256: string; wave: string; why: string };
export type Flag = { wave: string; severity: "blocker" | "note"; message: string };

export class Plan {
  manifestOps = new Map<string, ManifestOp>();
  newFiles = new Map<string, NewFile>();
  edits: Edit[] = [];
  private editKeys = new Set<string>();
  fileOps = new Map<string, FileOp>();
  flags: Flag[] = [];
  flag(wave: string, severity: Flag["severity"], message: string): void { this.flags.push({ wave, severity, message }); }
  manifest(op: ManifestOp): void {
    const k = [op.packageJson, op.field, op.key, op.op].join("\0");
    const have = this.manifestOps.get(k);
    if (have && have.value !== op.value) this.flag(op.wave, "blocker", `conflicting ${op.op} ${op.field}[${op.key}] in ${op.packageJson}: ${have.value} vs ${op.value}`);
    if (!have) this.manifestOps.set(k, op);
  }
  newFile(f: NewFile): void {
    const have = this.newFiles.get(f.path);
    if (have && have.content !== f.content) this.flag(f.wave, "blocker", `conflicting content for new file ${f.path}`);
    if (!have) this.newFiles.set(f.path, f);
  }
  edit(e: Edit): void {
    if (e.from === e.to) return;
    const k = [e.file, e.mode, e.from, e.to].join("\0");
    if (this.editKeys.has(k)) return;
    const clash = this.edits.find(x => x.file === e.file && x.mode === e.mode && x.from === e.from && x.to !== e.to);
    if (clash) { this.flag(e.wave, "blocker", `edit clash in ${e.file}: ${e.from} -> ${clash.to} | ${e.to}`); return; }
    this.editKeys.add(k); this.edits.push(e);
  }
  del(path: string, wave: string, why: string): void {
    if (this.fileOps.has(path)) return;
    const sha256 = existsSync(resolve(ROOT, path)) ? createHash("sha256").update(readFileSync(resolve(ROOT, path))).digest("hex") : "";
    this.fileOps.set(path, { op: "delete", path, sha256, wave, why });
  }
  move(path: string, to: string, wave: string, why: string): void {
    if (this.fileOps.has(path)) return;
    const sha256 = createHash("sha256").update(readFileSync(resolve(ROOT, path))).digest("hex");
    this.fileOps.set(path, { op: "move", path, to, sha256, wave, why });
  }
  toJSON() {
    return {
      schema: "cycle-break/delta@1", stagedAt: new Date().toISOString(),
      manifestOps: [...this.manifestOps.values()], newFiles: [...this.newFiles.values()], edits: this.edits, fileOps: [...this.fileOps.values()], flags: this.flags,
    };
  }
}
