import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { basename } from "node:path";
import { C, root } from "./lib.ts";
const map = JSON.parse(readFileSync(C + "/rmod-tools/crate-map.json", "utf8")) as { crates: Record<string, string>; ids: Record<string, string>; libNames: Record<string, string> };
const esc = (s: string) => s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
const rules: [RegExp, string][] = [];
for (const [o, n] of Object.entries(map.crates)) {
  rules.push([new RegExp(`(?<![A-Za-z0-9_-])${esc(o)}(?![A-Za-z0-9_-])`, "g"), n]);
  rules.push([new RegExp(`(?<![A-Za-z0-9_])${esc(o.replace(/-/g, "_"))}(?![A-Za-z0-9_])`, "g"), n.replace(/-/g, "_")]);
}
for (const [o, n] of Object.entries(map.ids)) {
  rules.push([new RegExp(`(?<=@semio-tech/)${esc(o)}(?=-rs(?![A-Za-z0-9_]))`, "g"), n]);
  rules.push([new RegExp(`(?<![A-Za-z0-9_@/-])${esc(o)}(?![A-Za-z0-9_-])`, "g"), n]);
}
for (const [o, n] of Object.entries(map.libNames)) rules.push([new RegExp(`(?<![A-Za-z0-9_])${esc(o)}(?![A-Za-z0-9_])`, "g"), n]);
export const rename = (s: string): string => { let r = s; for (const [re, n] of rules) r = r.replace(re, n); return r; };
export const touches = (s: string): boolean => rules.some(([re]) => { re.lastIndex = 0; return re.test(s); });

export const cmpVer = (a: string, b: string): number => {
  const sp = (v: string) => { const [core, pre = ""] = v.split("+")[0]!.split(/-(.*)/s); return { n: core!.split(".").map((x) => Number.parseInt(x, 10) || 0), pre: pre ?? "" }; };
  const x = sp(a), y = sp(b);
  for (let i = 0; i < Math.max(x.n.length, y.n.length); i++) { const d = (x.n[i] ?? 0) - (y.n[i] ?? 0); if (d !== 0) return d < 0 ? -1 : 1; }
  if (x.pre === y.pre) return 0;
  if (x.pre === "") return 1;
  if (y.pre === "") return -1;
  const p = x.pre.split("."), q = y.pre.split(".");
  for (let i = 0; i < Math.max(p.length, q.length); i++) {
    if (p[i] === undefined) return -1;
    if (q[i] === undefined) return 1;
    const pn = /^\d+$/.test(p[i]!), qn = /^\d+$/.test(q[i]!);
    if (pn && qn) { const d = Number(p[i]) - Number(q[i]); if (d !== 0) return d < 0 ? -1 : 1; }
    else if (pn) return -1;
    else if (qn) return 1;
    else if (p[i]! !== q[i]!) return p[i]! < q[i]! ? -1 : 1;
  }
  return 0;
};
const parseEntry = (e: string) => { const m = e.match(/^"([^" ]+)(?: ([^" ]+))?(?: \(([^)]*)\))?"$/); return m ? { name: m[1]!, version: m[2] ?? "", source: m[3] ?? "" } : { name: e, version: "", source: "" }; };
const cmpId = (a: { name: string; version: string; source: string }, b: { name: string; version: string; source: string }) => (a.name < b.name ? -1 : a.name > b.name ? 1 : a.version !== b.version ? cmpVer(a.version, b.version) : a.source < b.source ? -1 : a.source > b.source ? 1 : 0);

export const lockTransform = (text: string): string => {
  const parts = text.split(/\n(?=\[\[package\]\]\n)/);
  const head = parts[0]!.startsWith("[[package]]") ? null : parts.shift()!;
  const blocks = parts.map((b) => {
    let t = b;
    if (touches(t)) {
      t = rename(t);
      t = t.replace(/dependencies = \[\n((?:  "[^\n]*",?\n)+)\]/g, (_m, inner: string) => {
        const items = inner.trim().split("\n").map((l) => l.trim().replace(/,$/, ""));
        items.sort((x, y) => cmpId(parseEntry(x), parseEntry(y)));
        return "dependencies = [\n" + items.map((x) => `  ${x},`).join("\n") + "\n]";
      });
    }
    return t;
  });
  const key = (b: string) => { const name = b.match(/^name = "([^"]+)"/m)?.[1] ?? ""; const version = b.match(/^version = "([^"]+)"/m)?.[1] ?? ""; const source = b.match(/^source = "([^"]+)"/m)?.[1] ?? ""; return { name, version, source }; };
  const orig = parts.map(key);
  const wasSorted = orig.every((k, i) => i === 0 || cmpId(orig[i - 1]!, k) <= 0);
  const sorted = wasSorted ? [...blocks].sort((a, b) => cmpId(key(a), key(b))) : blocks;
  return (head !== null ? head + "\n" : "") + sorted.join("\n");
};

const keyOf = (o: any): string | null => { for (const k of ["package", "name", "id"]) if (typeof o?.[k] === "string") return k; return null; };
const sortedBy = (xs: any[], f: (x: any) => string) => xs.every((x, i) => i === 0 || f(xs[i - 1]) <= f(x));
const mostlySorted = (xs: any[], f: (x: any) => string) => { let bad = 0; for (let i = 1; i < xs.length; i++) if (f(xs[i - 1]) > f(xs[i])) bad++; return bad <= Math.max(1, Math.floor(xs.length * 0.05)); };
const reinsert = (orig: any[], out: any[], f: (x: any) => string): any[] => {
  const changedIdx = out.map((x, i) => (JSON.stringify(x) !== JSON.stringify(orig[i]) ? i : -1)).filter((i) => i >= 0);
  const keep = out.filter((_x, i) => !changedIdx.includes(i));
  const moved = changedIdx.map((i) => out[i]);
  for (const m of moved) {
    const k = f(m);
    let at = keep.length;
    for (let i = 0; i < keep.length; i++) if (f(keep[i]) > k) { at = i; break; }
    keep.splice(at, 0, m);
  }
  return keep;
};
export const jsonTransform = (text: string): string | null => {
  let doc: any;
  try { doc = JSON.parse(text); } catch { return null; }
  const trailing = text.endsWith("\n") ? "\n" : "";
  const indent = text.match(/^\{\n( +)"/)?.[1]?.length ?? 2;
  if (JSON.stringify(doc, null, indent) + trailing !== text) return null;
  const walk = (n: any): any => {
    if (typeof n === "string") return rename(n);
    if (Array.isArray(n)) {
      const out = n.map(walk);
      const changed = JSON.stringify(out) !== JSON.stringify(n);
      if (changed && out.length > 1) {
        if (n.every((x) => typeof x === "string") && mostlySorted(n, (x) => x)) return reinsert(n, out, (x) => x);
        if (n.every((x) => x && typeof x === "object" && !Array.isArray(x))) {
          const k = keyOf(n[0]);
          if (k && n.every((x) => keyOf(x) === k) && mostlySorted(n, (x) => x[k])) return reinsert(n, out, (x) => x[k]);
        }
      }
      return out;
    }
    if (n && typeof n === "object") { const o: any = {}; for (const [k, v] of Object.entries(n)) o[rename(k)] = walk(v); return o; }
    return n;
  };
  return JSON.stringify(walk(doc), null, indent) + trailing;
};

export const tomlTransform = (text: string): string => {
  const r = rename(text);
  if (r === text) return r;
  const lines = r.split("\n");
  const orig = text.split("\n");
  const sortRuns = (ls: string[], ol: string[]): string[] => {
    const out = ls.slice();
    let i = 0;
    while (i < out.length) {
      if (/^[A-Za-z0-9_-]+ = /.test(out[i]!)) {
        let j = i;
        while (j < out.length && /^[A-Za-z0-9_-]+ = /.test(out[j]!)) j++;
        const run = out.slice(i, j), orun = ol.slice(i, j);
        const nm = (l: string) => l.slice(0, l.indexOf(" = "));
        if (run.length > 1 && run.some((l, k) => l !== orun[k]) && sortedBy(orun, nm)) { run.sort((a, b) => (nm(a) < nm(b) ? -1 : nm(a) > nm(b) ? 1 : 0)); out.splice(i, j - i, ...run); }
        i = j;
      } else i++;
    }
    return out;
  };
  let res = sortRuns(lines, orig);
  const txt = res.join("\n");
  const blocks = txt.split(/\n(?=\[workspace\.dependencies\.)/);
  if (blocks.length > 2) {
    const first = blocks[0]!;
    const rest = blocks.slice(1);
    const hdr = (b: string) => b.match(/^\[workspace\.dependencies\.([^\]]+)\]/)![1]!;
    const oBlocks = text.split(/\n(?=\[workspace\.dependencies\.)/).slice(1);
    const lastTail = (b: string) => b;
    if (sortedBy(oBlocks.map((b) => b.replace(/\n+$/, "")), (b) => hdr(b)) && !sortedBy(rest.map((b) => b.replace(/\n+$/, "")), (b) => hdr(b))) {
      const norm = rest.map((b) => b.replace(/\n+$/, ""));
      norm.sort((a, b) => (hdr(a) < hdr(b) ? -1 : hdr(a) > hdr(b) ? 1 : 0));
      const trail = rest[rest.length - 1]!.match(/\n*$/)![0];
      return first + "\n" + norm.join("\n\n") + trail;
    }
  }
  return txt;
};

export const transform = (path: string, text: string): string | null => {
  if (!touches(text)) return null;
  const b = basename(path);
  if (b === "Cargo.lock") return lockTransform(text);
  if (b === "Cargo.toml") return tomlTransform(text);
  if (path.endsWith("/.vscode/launch.json")) return rename(text);
  if (b.endsWith(".json")) { const j = jsonTransform(text); if (j !== null) return j; return "MANUAL"; }
  return rename(text);
};

if (import.meta.main) {
  const dry = process.argv[2] !== "--apply";
  const listFile = C + "/🗑️generated/rmod-crate-files.txt";
  const files = readFileSync(listFile, "utf8").split("\n").filter(Boolean).map((f) => (f.startsWith("/") ? f : root + "/" + f.replace(/^\.\//, "")));
  let changed = 0; const manual: string[] = []; const same: string[] = [];
  for (const f of files) {
    if (!existsSync(f)) continue;
    const t = readFileSync(f, "utf8");
    const r = transform(f, t);
    if (r === null) { same.push(f); continue; }
    if (r === "MANUAL") { manual.push(f); continue; }
    if (r !== t) { changed++; if (!dry) writeFileSync(f, r); }
  }
  console.log(`${dry ? "dry " : ""}changed ${changed} of ${files.length}; manual ${manual.length}; untouched ${same.length}`);
  for (const m of manual) console.log("MANUAL", m.slice(root.length + 1));
}
