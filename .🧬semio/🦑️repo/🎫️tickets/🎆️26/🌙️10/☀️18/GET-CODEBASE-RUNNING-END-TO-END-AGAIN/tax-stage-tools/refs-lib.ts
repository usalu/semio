// 🔗 Reference index and rewriting over the virtual overlay: quoted path references to moved files, in import position or anywhere.
import {existsSync, readFileSync} from "node:fs";
import {dirname, join, posix, relative, resolve} from "node:path";
import {C, REPO, Vfs} from "./vfs.ts";

export type Ref = {file: string; line: number; spec: string; kind: "relative" | "path"};
const index: Record<string, Ref[]> = JSON.parse(readFileSync(`${C}/tax-stage-tools/index/refs.json`, "utf8"));
export const refsOf = (target: string): Ref[] => index[target] ?? [];

const abs = (p: string): string => join(REPO, p);
const IMPORT_CONTEXT = /(?:\bfrom\s*|\bimport\s*\(\s*|\bimport\s*|\brequire\s*\(\s*)$/u;

/** 🔁 Rewrites quoted references that resolve to `from` (relative) or end with its repo path (absolute-ish) into references to `to`. */
export function rewriteReferences(text: string, referencerDir: string, from: string, to: string, importOnly: boolean): {text: string; count: number} {
  let count = 0;
  const out = text.replace(/(["'`])((?:\\.|(?!\1)[^\n\\])*)\1/gu, (m, q: string, v: string, offset: number) => {
    if (v.includes("${")) return m;
    if (importOnly && !IMPORT_CONTEXT.test(text.slice(Math.max(0, offset - 24), offset))) return m;
    const shift = relative(dirname(abs(from)), abs(to)), keepStyle = (): string => { const head = posix.dirname(v); return (head === "." ? (v.startsWith("./") ? "./" : "") : head + "/") + shift; };
    if (v.startsWith(".")) {
      if (resolve(referencerDir, v) !== abs(from)) return m;
      count++;
      return q + keepStyle() + q;
    }
    if (v === from || v.endsWith("/" + from)) { count++; return q + v.slice(0, v.length - from.length) + to + q; }
    if (!/^(?:[A-Za-z]:|\/|@)/u.test(v) && resolve(referencerDir, v) === abs(from)) { count++; return q + keepStyle() + q; }
    return m;
  });
  return {text: out, count};
}

/** 🚚 Scripts whose implementation moved into a module: their old referencing code now lives in the module. */
export const RELOCATED = new Map<string, string>();

/** 🔁 Re-points every referencer of `from` found in the index (and the module that took over a lifted referencer); `skip` files are left alone. */
export function repoint(v: Vfs, from: string, to: string, importOnly: boolean, skip: Set<string> = new Set()): string[] {
  const touched: string[] = [];
  for (const indexed of new Set(refsOf(from).map((r) => r.file))) {
    for (const file of [indexed, RELOCATED.get(indexed)]) {
      if (!file || file === from || skip.has(file) || !v.exists(file)) continue;
      const r = rewriteReferences(v.read(file), dirname(abs(file)), from, to, importOnly);
      if (r.count) { v.write(file, r.text); touched.push(file); }
    }
  }
  return touched;
}

/** 🧭️ Rebases every quoted relative specifier of moved source text from `oldDir` to `newDir` when its target exists in the overlay or on disk. */
export function rebaseOwn(text: string, oldDir: string, newDir: string, v: Vfs): string {
  const present = (target: string): boolean => { const r = relative(REPO, target); return v.exists(r) || v.isDirectory(r) || existsSync(target); };
  return text.replace(/(["'`])(\.{1,2}\/(?:(?!\1)(?:\\u[0-9a-fA-F]{4}|[^\n\\$]))*)\1/gu, (m, q: string, raw: string) => {
    const s = raw.replace(/\\u([0-9a-fA-F]{4})/gu, (_x, h: string) => String.fromCharCode(parseInt(h, 16)));
    const target = resolve(oldDir, s);
    if (!present(target) && !present(target.replace(/\.[^./]+$/u, ""))) return m;
    const r = relative(newDir, target);
    return q + (r.startsWith(".") ? r : "./" + r) + q;
  });
}

export type RefKind = "IMPORT" | "READ" | "READ?" | "SPAWN" | "DATA" | "OTHER";
/** 🏷️ Classifies one indexed reference by the code around it. */
export function classifyRef(r: Ref): RefKind {
  const line = readFileSync(abs(r.file), "utf8").split("\n")[r.line - 1] ?? "";
  const at = line.indexOf(r.spec), before = line.slice(Math.max(0, at - 160), at);
  if (/\.(json|toml|md)$/u.test(r.file)) return "DATA";
  if (/(?:\bfrom\s*|\bimport\s*\(\s*|\bimport\s*)["'`]$/u.test(line.slice(Math.max(0, at - 24), at))) return "IMPORT";
  if (/readFile|Bun\.file|createSourceFile|\.text\(\)|statSync|existsSync/u.test(before)) {
    const after = line.slice(at + r.spec.length, at + r.spec.length + 60), close = line[at - 1] ?? "";
    const pre = before.replace(/["'`]$/u, ""), bound = /new URL\(\s*$/u.test(pre) && new RegExp("^" + close.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&") + "\\s*,\\s*import\\.meta\\.url").test(after) || /(?:join|resolve)\(\s*(?:import\.meta\.dir|importMetaDirectory)\s*,\s*$/u.test(pre);
    return bound ? "READ" : "READ?";
  }
  if (/runOwnedCommand|captureOwned|spawn|execPath|runCmd|"bun"|Command|execute/u.test(before)) return "SPAWN";
  return "OTHER";
}

/** 🔁 Re-points only the indexed references of the given kinds, line by line, so that spawn references in the same file stay on the script. */
export function repointKinds(v: Vfs, from: string, to: string, kinds: ReadonlySet<RefKind>, accept: (file: string) => boolean = () => true): string[] {
  const touched: string[] = [], byFile = new Map<string, Set<number>>();
  for (const r of refsOf(from)) {
    if (r.file === from || !v.exists(r.file) || !accept(r.file) || !kinds.has(classifyRef(r))) continue;
    (byFile.get(r.file) ?? byFile.set(r.file, new Set()).get(r.file)!).add(r.line);
  }
  for (const [file, lines] of byFile) {
    const rows = v.read(file).split("\n");
    let count = 0;
    for (const n of lines) { const r = rewriteReferences(rows[n - 1]!, dirname(abs(file)), from, to, false); if (r.count) { rows[n - 1] = r.text; count += r.count; } }
    if (count) { v.write(file, rows.join("\n")); touched.push(file); }
  }
  return touched;
}

/** 🔁 Re-points the indexed references of `from` at explicitly named `<file substring>:<line>` sites that the classifier cannot bind to a base directory. */
export function repointSites(v: Vfs, from: string, to: string, sites: readonly string[]): string[] {
  const touched: string[] = [];
  for (const site of sites) {
    const cut = site.lastIndexOf(":"), where = site.slice(0, cut), line = Number(site.slice(cut + 1));
    const hit = refsOf(from).find((r) => r.file.includes(where) && r.line === line);
    if (!hit) throw new Error(`no indexed reference at ${site}`);
    const rows = v.read(hit.file).split("\n"), r = rewriteReferences(rows[line - 1]!, dirname(abs(hit.file)), from, to, false);
    if (!r.count) throw new Error(`reference at ${site} did not rewrite`);
    rows[line - 1] = r.text;
    v.write(hit.file, rows.join("\n"));
    touched.push(hit.file);
  }
  return touched;
}
