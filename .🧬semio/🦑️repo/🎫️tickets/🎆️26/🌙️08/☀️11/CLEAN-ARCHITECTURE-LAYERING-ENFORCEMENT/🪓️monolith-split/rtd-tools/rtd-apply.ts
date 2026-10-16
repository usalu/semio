// ✍️ Applies the RENDERER-TEST-DEPS plan: re-points test-side `ui_wgpu::…` paths and `use` statements to the owning ui producer crate and edits the manifests (dev-dependencies, test-only normal edges). Flags: --apply (default dry run), --only=<crate,…>, --skip=<crate,…>. Report JSON to argv[3].
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {existsSync, readFileSync, realpathSync, writeFileSync} from "node:fs";
import {join, relative, dirname} from "node:path";
import {cfgCompiledWithoutTest, findAliasRefs, inRegions, mask, testRegions, type UseLeaf} from "./rtd-rust.ts";
import {ROOT, verdictOf, type Verdict} from "./rtd-verdict.ts";
import {loadCrate} from "./rtd-world.ts";

const alias = process.env.ALIAS ?? "ui_wgpu";
const APPLY = process.argv.includes("--apply");
const DIFF = process.argv.includes("--diff");
const arg = (name: string) => process.argv.find(a => a.startsWith(`--${name}=`))?.slice(name.length + 3).split(",").filter(Boolean);
const ONLY = arg("only"), SKIP = arg("skip") ?? [];
const plan = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as any[];
const report: any[] = [];
const isUiProducer = (v?: Verdict) => v?.status === "producer" && !!v.ownerPkg?.startsWith("semio-framework-ui") && !!v.ownerPath?.length;
const identOf = (key: string) => key.replace(/-/g, "_");
const MAXW = 250;

const manifestDeps = (manifestRel: string) => {
  const t = Bun.TOML.parse(readFileSync(join(ROOT, manifestRel), "utf8")) as any;
  const dir = join(ROOT, manifestRel, "..");
  const have = new Map<string, string>();
  for (const table of ["dependencies", "dev-dependencies"]) {
    for (const [k, raw] of Object.entries(t[table] ?? {}) as [string, any][]) {
      const p = typeof raw === "object" ? raw.path : undefined;
      if (typeof p === "string") { const c = loadCrate(join(dir, p)); if (c && !have.has(c.pkg)) have.set(c.pkg, k); }
    }
  }
  return {have, dir};
};

type Leaf = {rel: string[]; alias?: string; glob?: boolean};
type TNode = {kids: Map<string, TNode>; here: boolean; alias?: string};
const trie = (leaves: Leaf[]): TNode => {
  const root: TNode = {kids: new Map(), here: false};
  for (const l of leaves) {
    let n = root;
    for (const seg of [...l.rel, ...(l.glob ? ["*"] : [])]) { if (!n.kids.has(seg)) n.kids.set(seg, {kids: new Map(), here: false}); n = n.kids.get(seg)!; }
    n.here = true;
    n.alias = l.alias;
  }
  return root;
};
const renderNode = (name: string, n: TNode): string => {
  const self = n.alias ? `${name} as ${n.alias}` : name;
  if (!n.kids.size) return self;
  const kids = [...n.kids].map(([k, v]) => renderNode(k, v));
  if (n.here) return `${name}::{${[n.alias ? `self as ${n.alias}` : "self", ...kids].join(", ")}}`;
  if (kids.length === 1) return `${name}::${kids[0]}`;
  return `${name}::{${kids.join(", ")}}`;
};
const topItems = (leaves: Leaf[]): string[] => [...trie(leaves).kids].map(([k, v]) => renderNode(k, v));
const renderUse = (vis: string, prefix: string[], leaves: Leaf[], indent: string): string => {
  const items = topItems(leaves);
  if (items.length === 1) return `${vis}use ${prefix.join("::")}::${items[0]};`;
  return wrapUse(`${vis}use ${prefix.join("::")}::`, items, indent);
};

const wrapUse = (head: string, items: string[], indent: string): string => {
  const single = `${head}{${items.join(", ")}};`;
  if (indent.length + single.length <= MAXW) return single;
  const lines: string[] = [];
  let cur = "";
  for (const it of items) {
    const next = cur ? `${cur} ${it},` : `${it},`;
    if (indent.length + 4 + next.length > MAXW && cur) { lines.push(cur); cur = `${it},`; } else cur = next;
  }
  if (cur) lines.push(cur);
  return `${head}{\n${lines.map(l => `${indent}    ${l}`).join("\n")}\n${indent}};`;
};

const precedingAttrs = (T: string, start: number): {text: string; from: number} => {
  let from = start;
  const parts: string[] = [];
  for (;;) {
    let k = from;
    while (k > 0 && /\s/.test(T[k - 1]!)) k--;
    if (k > 0 && T[k - 1] === "]") {
      let d = 0, j = k - 1;
      for (; j >= 0; j--) { if (T[j] === "]") d++; else if (T[j] === "[") { d--; if (d === 0) break; } }
      if (j > 0 && T[j - 1] === "#") { parts.unshift(T.slice(j - 1, k)); from = j - 1; continue; }
    }
    break;
  }
  return {text: parts.join("\n"), from};
};

type Applied = {rewritten: number; kept: Map<string, number>; producers: Set<string>; files: Set<string>; useStmts: number; paths: number; lines: string[]};

const rewriteFile = (file: string, wholeFile: boolean, identFor: (pkg: string) => string, out: Applied) => {
  const abs = join(ROOT, file);
  const T = readFileSync(abs, "utf8");
  const M = mask(T);
  const regions = testRegions(M);
  const edits: {start: number; end: number; text: string}[] = [];
  for (const r of findAliasRefs(M, alias)) {
    const reg = inRegions(regions, r.pos);
    const selected = wholeFile || (reg && !cfgCompiledWithoutTest(reg.attr));
    if (!selected) continue;
    if (r.form === "path") {
      const v = verdictOf(r.segs.slice());
      if (!isUiProducer(v)) { out.kept.set(r.segs.slice(0, v.consumed ?? r.segs.length).join("::") + ` [${v.status}${v.ownerPkg ? " " + v.ownerPkg : ""}]`, (out.kept.get(r.segs.slice(0, v.consumed ?? r.segs.length).join("::") + ` [${v.status}${v.ownerPkg ? " " + v.ownerPkg : ""}]`) ?? 0) + 1); continue; }
      const tail = M.slice(r.pos + alias.length);
      let end = r.pos + alias.length, i = 0;
      for (let n = 0; n < v.consumed!; n++) {
        const m = /^\s*::\s*[A-Za-z_][A-Za-z0-9_]*/.exec(tail.slice(i));
        if (!m) throw new Error(`path tail parse failed in ${file}:${r.line}`);
        i += m[0].length;
        end = r.pos + alias.length + i;
      }
      const ident = identFor(v.ownerPkg!);
      edits.push({start: r.pos, end, text: `${ident}::${v.ownerPath!.join("::")}`});
      out.producers.add(v.ownerPkg!);
      out.paths++;
      out.rewritten++;
    } else if (r.form === "use") {
      const leaves = r.tree as UseLeaf[];
      const keep: {rel: string[]; alias?: string; glob?: boolean}[] = [];
      const groups = new Map<string, {rel: string[]; alias?: string; glob?: boolean}[]>();
      let changed = false;
      for (const l of leaves) {
        const relSegs = l.segs.slice(1);
        const v = l.kind === "item" || l.kind === "glob" ? verdictOf(relSegs.slice()) : undefined;
        const asGlob = l.kind === "glob";
        if (v && isUiProducer(v) && (l.kind === "item" ? true : v.consumed === relSegs.length)) {
          const ident = identFor(v.ownerPkg!);
          const rel = [...v.ownerPath!, ...relSegs.slice(v.consumed!)];
          const g = groups.get(ident) ?? [];
          if (!g.some(x => x.rel.join("::") === rel.join("::") && x.alias === l.alias && !!x.glob === asGlob)) g.push({rel, alias: l.alias, glob: asGlob});
          groups.set(ident, g);
          out.producers.add(v.ownerPkg!);
          out.rewritten++;
          changed = true;
        } else {
          keep.push({rel: relSegs, alias: l.alias, glob: asGlob || undefined});
          const label = `${relSegs.join("::")}${asGlob ? "::*" : ""} [${v?.status ?? l.kind}${v?.ownerPkg && v.status !== "aggregate" ? " " + v.ownerPkg : ""}]`;
          out.kept.set(label, (out.kept.get(label) ?? 0) + 1);
        }
      }
      if (!changed) continue;
      const stmtStart = r.stmtStart!, stmtEnd = r.stmtEnd!;
      const lineStart = T.lastIndexOf("\n", stmtStart - 1) + 1;
      const attrs = precedingAttrs(T, stmtStart);
      const lead = T.slice(lineStart, stmtStart);
      const atLineStart = /^\s*$/.test(lead);
      const indent = atLineStart ? lead : "";
      const vis = r.vis ? r.vis + " " : "";
      const stmts: string[] = [];
      if (keep.length) {
        const head = keep.every(k => k.rel[0] === "wgpu") ? [alias, "wgpu"] : [alias];
        const items = keep.map(k => ({...k, rel: head.length === 2 ? k.rel.slice(1) : k.rel}));
        stmts.push(renderUse(vis, head, items, indent));
      }
      for (const [ident, g] of groups) stmts.push(renderUse(vis, [ident], g, indent));
      const sep = atLineStart ? `\n${indent}${attrs.text ? attrs.text.replace(/\n/g, `\n${indent}`) + `\n${indent}` : ""}` : " ";
      edits.push({start: stmtStart, end: stmtEnd, text: stmts.join(sep)});
      out.useStmts++;
    }
  }
  if (!edits.length) return;
  edits.sort((a, b) => b.start - a.start);
  let N = T;
  let last = Infinity;
  for (const e of edits) {
    if (e.end > last) throw new Error(`overlapping edits in ${file}`);
    N = N.slice(0, e.start) + e.text + N.slice(e.end);
    last = e.start;
  }
  if (DIFF) for (const e of edits.slice().reverse().slice(0, Number(process.env.DIFF_MAX ?? 6))) console.log(`  ${file.split("/").slice(-2).join("/")}\n    - ${T.slice(e.start, e.end).replace(/\s+/g, " ").slice(0, 200)}\n    + ${e.text.replace(/\s+/g, " ").slice(0, 260)}`);
  out.files.add(file);
  out.lines.push(`${file}: ${edits.length} edit(s)`);
  if (APPLY) {
    const now = readFileSync(abs, "utf8");
    if (now !== T) throw new Error(`file changed under us: ${file}`);
    writeFileSync(abs, N);
  }
};

const editManifest = (manifestRel: string, opts: {removeDev: boolean; removeNormal: boolean; add: Map<string, {key: string; path: string}>}) => {
  const abs = join(ROOT, manifestRel);
  const lines = readFileSync(abs, "utf8").split("\n");
  const log: string[] = [];
  let section = "";
  const out: string[] = [];
  let devHeader = -1;
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i]!;
    const h = /^\[([^\]]+)\]\s*$/.exec(line);
    if (h) { section = h[1]!; if (section === "dev-dependencies") devHeader = out.length; }
    const isDev = section === "dev-dependencies" || /\.dev-dependencies$/.test(section);
    const isNormal = section === "dependencies" || (/^target\..*\.dependencies$/.test(section) && !isDev);
    if (new RegExp(`^${alias}\\s*=`).test(line)) {
      if ((isDev && opts.removeDev) || (isNormal && opts.removeNormal)) {
        if (!/\}\s*$/.test(line)) throw new Error(`multi-line ${alias} entry in ${manifestRel}`);
        if (out.length && out[out.length - 1]!.trim().startsWith("#")) log.push(`comment above removed entry kept: ${out[out.length - 1]}`);
        log.push(`removed [${section}] ${line.trim().slice(0, 120)}`);
        continue;
      }
    }
    out.push(line);
  }
  if (opts.add.size) {
    const existing = new Set(out.map(l => /^([A-Za-z0-9_-]+)\s*=/.exec(l)?.[1]).filter(Boolean));
    const newLines = [...opts.add.values()].filter(a => !existing.has(a.key)).sort((a, b) => a.key.localeCompare(b.key)).map(a => `${a.key} = { path = "${a.path}" }`);
    if (newLines.length) {
      const hi = out.findIndex(l => l.trim() === "[dev-dependencies]");
      if (hi >= 0) {
        let end = hi + 1;
        while (end < out.length && !/^\[/.test(out[end]!)) end++;
        let ins = end;
        while (ins > hi + 1 && out[ins - 1]!.trim() === "") ins--;
        out.splice(ins, 0, ...newLines);
      } else {
        while (out.length && out[out.length - 1]!.trim() === "") out.pop();
        out.push("", "[dev-dependencies]", ...newLines, "");
      }
      for (const l of newLines) log.push(`added [dev-dependencies] ${l.slice(0, 140)}`);
    }
  }
  const text = out.join("\n");
  Bun.TOML.parse(text);
  if (APPLY && text !== lines.join("\n")) {
    const now = readFileSync(abs, "utf8");
    if (now !== lines.join("\n")) throw new Error(`manifest changed under us: ${manifestRel}`);
    writeFileSync(abs, text);
  }
  return log;
};

for (const c of plan) {
  if (ONLY && !ONLY.some(o => c.crate.endsWith(o))) continue;
  if (SKIP.some(o => c.crate.endsWith(o))) continue;
  const short = c.crate.replace("semio-framework-os-renderer-", "");
  const testOnlyCrate = c.rootCfgTest || /(shell-tests-|-tests$)/.test(c.crate);
  const {have, dir} = manifestDeps(c.manifest);
  const need = new Map<string, {key: string; path: string}>();
  const identFor = (pkg: string): string => {
    const key = have.get(pkg);
    if (key) return identOf(key);
    if (!need.has(pkg)) {
      const crate = [...new Set(c.edits.flatMap((e: any) => e.form === "path" ? [e.verdict] : (e.leaves ?? []).map((l: any) => l.verdict)))].find((v: any) => v?.ownerPkg === pkg) as Verdict | undefined;
      const ownerDir = realpathSync.native(crate!.ownerDir!);
      need.set(pkg, {key: pkg, path: relative(dir, ownerDir).split("\\").join("/")});
    }
    return identOf(pkg);
  };
  const out: Applied = {rewritten: 0, kept: new Map(), producers: new Set(), files: new Set(), useStmts: 0, paths: 0, lines: []};
  const wholeByFile = new Map<string, boolean>();
  for (const e of c.edits) if (e.where === "testfile") wholeByFile.set(e.file, true); else if (!wholeByFile.has(e.file)) wholeByFile.set(e.file, false);
  for (const [file, whole] of wholeByFile) rewriteFile(file, whole, identFor, out);
  const keptTest = [...out.kept.values()].reduce((s, n) => s + n, 0);
  const keepDev = c.testkitTokens > 0;
  const removeNormal = testOnlyCrate && keptTest === 0 && c.normalRefs === 0;
  const hasDevEdge = Object.keys(c.have ?? {}).length >= 0;
  void hasDevEdge;
  const mlog = editManifest(c.manifest, {removeDev: !keepDev && !(c.crate.endsWith("renderer-camera") && keptTest > 0), removeNormal, add: need});
  report.push({crate: c.crate, short, testOnlyCrate, rewritten: out.rewritten, paths: out.paths, useStmts: out.useStmts, files: [...out.files], producers: [...out.producers].sort(), kept: Object.fromEntries(out.kept), keptTest, normalRefs: c.normalRefs, featureGatedRefs: c.featureGatedRefs, testkitTokens: c.testkitTokens, keepDev, removeNormal, manifest: c.manifest, manifestLog: mlog});
  console.log(`${APPLY ? "APPLIED" : "DRY"} ${short}: ${out.rewritten} rewrites (${out.paths} paths, ${out.useStmts} use stmts) in ${out.files.size} files; kept ${keptTest}; ${mlog.length} manifest change(s)`);
}
writeFileSync(process.argv[3]!, JSON.stringify(report, null, 1));
void existsSync; void dirname;
