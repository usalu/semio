import { execFileSync } from "node:child_process";
import { readFileSync, statSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";

export type FnRow = { file: string; kind: string; name: string; start: number; bodyStart: number; end: number; lines: number; nested?: boolean; generated?: boolean; ageMinutes?: number };

const EXCLUDED = ["/node_modules/", "/target/", "/🗑️generated/", "/🎫️tickets/", "/.cache/", "/dist/"];

/** 🔭️ Lexes Rust source and returns every `fn` item with the line span of its body. See https://doc.rust-lang.org/reference/items/functions.html */
export function rustFunctions(file: string, text: string): FnRow[] {
  const rows: FnRow[] = [];
  const n = text.length;
  const lineAt = new Uint32Array(n + 1);
  let line = 1;
  for (let i = 0; i < n; i++) {
    lineAt[i] = line;
    if (text.charCodeAt(i) === 10) line++;
  }
  lineAt[n] = line;
  type Tok = { t: string; i: number };
  const toks: Tok[] = [];
  let i = 0;
  const isIdStart = (c: number) => (c >= 65 && c <= 90) || (c >= 97 && c <= 122) || c === 95 || c > 127;
  const isId = (c: number) => isIdStart(c) || (c >= 48 && c <= 57);
  while (i < n) {
    const c = text.charCodeAt(i);
    if (c <= 32) { i++; continue; }
    if (c === 47 && text.charCodeAt(i + 1) === 47) { while (i < n && text.charCodeAt(i) !== 10) i++; continue; }
    if (c === 47 && text.charCodeAt(i + 1) === 42) {
      let depth = 1;
      i += 2;
      while (i < n && depth > 0) {
        if (text.charCodeAt(i) === 47 && text.charCodeAt(i + 1) === 42) { depth++; i += 2; }
        else if (text.charCodeAt(i) === 42 && text.charCodeAt(i + 1) === 47) { depth--; i += 2; }
        else i++;
      }
      continue;
    }
    if (c === 34) {
      i++;
      while (i < n && text.charCodeAt(i) !== 34) i += text.charCodeAt(i) === 92 ? 2 : 1;
      i++;
      toks.push({ t: '"', i });
      continue;
    }
    if (isIdStart(c)) {
      let j = i;
      while (j < n && isId(text.charCodeAt(j))) j++;
      const word = text.slice(i, j);
      const nx = text.charCodeAt(j);
      if ((word === "r" || word === "br" || word === "cr") && (nx === 34 || nx === 35)) {
        let k = j;
        let hashes = 0;
        while (text.charCodeAt(k) === 35) { hashes++; k++; }
        if (text.charCodeAt(k) === 34) {
          const close = '"' + "#".repeat(hashes);
          const e = text.indexOf(close, k + 1);
          i = e < 0 ? n : e + close.length;
          toks.push({ t: '"', i });
          continue;
        }
      }
      if ((word === "b" || word === "c") && nx === 34) { i = j; continue; }
      if (word === "b" && nx === 39) { i = j; continue; }
      toks.push({ t: word, i });
      i = j;
      continue;
    }
    if (c === 39) {
      if (text.charCodeAt(i + 1) === 92) {
        i += 2;
        while (i < n && text.charCodeAt(i) !== 39) i++;
        i++;
        continue;
      }
      const cp = text.codePointAt(i + 1) ?? 0;
      const w = cp > 0xffff ? 2 : 1;
      if (text.charCodeAt(i + 1 + w) === 39) { i += 2 + w; continue; }
      i++;
      while (i < n && isId(text.charCodeAt(i))) i++;
      continue;
    }
    toks.push({ t: text[i]!, i });
    i++;
  }
  for (let k = 0; k < toks.length - 1; k++) {
    if (toks[k]!.t !== "fn") continue;
    const nameTok = toks[k + 1]!;
    if (!isIdStart(nameTok.t.charCodeAt(0))) continue;
    let depth = 0;
    let m = k + 2;
    let open = -1;
    for (; m < toks.length; m++) {
      const t = toks[m]!.t;
      if (t === "(" || t === "[") depth++;
      else if (t === ")" || t === "]") depth--;
      else if (depth === 0 && t === ";") break;
      else if (depth === 0 && t === "{") { open = m; break; }
    }
    if (open < 0) continue;
    let braces = 0;
    let close = open;
    for (let q = open; q < toks.length; q++) {
      const t = toks[q]!.t;
      if (t === "{") braces++;
      else if (t === "}") { braces--; if (braces === 0) { close = q; break; } }
    }
    const start = lineAt[toks[k]!.i]!;
    const bodyStart = lineAt[toks[open]!.i]!;
    const end = lineAt[toks[close]!.i]!;
    rows.push({ file, kind: "rust-fn", name: nameTok.t, start, bodyStart, end, lines: end - start + 1 });
  }
  return rows;
}

/** 🔬️ Parses TypeScript with the compiler API and returns every function-like node with a body. See https://github.com/microsoft/TypeScript/wiki/Using-the-Compiler-API */
export function tsFunctions(ts: typeof import("typescript"), file: string, text: string): FnRow[] {
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS);
  const rows: FnRow[] = [];
  const L = (p: number) => sf.getLineAndCharacterOfPosition(p).line + 1;
  const label = (node: import("typescript").Node): string => {
    const named = (node as { name?: import("typescript").Node }).name;
    if (named) return named.getText(sf);
    const p = node.parent;
    if (p && ts.isVariableDeclaration(p)) return p.name.getText(sf);
    if (p && ts.isPropertyAssignment(p)) return p.name.getText(sf);
    if (p && ts.isPropertyDeclaration(p)) return p.name.getText(sf);
    if (p && ts.isCallExpression(p)) return `(callback of ${p.expression.getText(sf).slice(0, 40)})`;
    return "(anonymous)";
  };
  const visit = (node: import("typescript").Node) => {
    const fnLike = ts.isFunctionDeclaration(node) || ts.isFunctionExpression(node) || ts.isArrowFunction(node) || ts.isMethodDeclaration(node) || ts.isConstructorDeclaration(node) || ts.isGetAccessorDeclaration(node) || ts.isSetAccessorDeclaration(node);
    if (fnLike && (node as { body?: import("typescript").Node }).body) {
      const body = (node as unknown as { body: import("typescript").Node }).body;
      const start = L(node.getStart(sf));
      const end = L(node.getEnd());
      rows.push({ file, kind: `ts-${ts.SyntaxKind[node.kind]}`, name: label(node), start, bodyStart: L(body.getStart(sf)), end, lines: end - start + 1 });
    }
    ts.forEachChild(node, visit);
  };
  visit(sf);
  return rows;
}

/** 📊️ Lists tracked and untracked repo sources, excluding vendored, generated and ticket folders. */
export function sourceFiles(root: string): string[] {
  const out = execFileSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], { cwd: root, maxBuffer: 1 << 28 }).toString("utf8");
  return out.split("\0").filter((p) => /\.(rs|ts|tsx|mts|cts)$/.test(p) && !EXCLUDED.some((e) => ("/" + p).includes(e)));
}

if (import.meta.main) {
  const args = process.argv.slice(2);
  const opt = (k: string, d: string) => { const ix = args.indexOf(k); return ix >= 0 ? args[ix + 1]! : d; };
  const root = opt("--root", process.cwd());
  const min = Number(opt("--min", "300"));
  const out = opt("--out", "");
  const tsPath = opt("--typescript", process.env.TYPESCRIPT_PATH ?? "/Users/ueli/.cache/c6fin2/nm/node_modules/typescript");
  const ts = createRequire(import.meta.url)(tsPath) as typeof import("typescript");
  const rows: FnRow[] = [];
  let scanned = 0;
  const now = Date.now();
  for (const rel of sourceFiles(root)) {
    let text: string;
    let mtime = 0;
    try { text = readFileSync(`${root}/${rel}`, "utf8"); mtime = statSync(`${root}/${rel}`).mtimeMs; } catch { continue; }
    let nl = 0;
    for (let q = text.indexOf("\n"); q >= 0 && nl <= min; q = text.indexOf("\n", q + 1)) nl++;
    if (nl < min) continue;
    scanned++;
    const all = rel.endsWith(".rs") ? rustFunctions(rel, text) : tsFunctions(ts, rel, text);
    const generated = /@generated|Generated by|🤖️ Generated/.test(text.slice(0, 600));
    for (const r of all) {
      if (r.lines <= min) continue;
      r.nested = all.some((o) => o !== r && o.start <= r.start && o.end >= r.end && (o.start < r.start || o.end > r.end));
      r.generated = generated;
      r.ageMinutes = Math.round((now - mtime) / 60000);
      rows.push(r);
    }
  }
  rows.sort((a, b) => b.lines - a.lines);
  const outer = rows.filter((r) => !r.nested);
  const md = [
    `# Function length census (> ${min} lines)`,
    "",
    `Tool: \`fnsplit4b-tools/census.ts\` (Rust: own lexer over \`git ls-files -co\`; TS: TypeScript compiler API; \`--json\` writes the rows). Files scanned (> ${min} lines): ${scanned}. Functions found: ${rows.length}, outermost: ${outer.length}, outermost > 600: ${outer.filter((r) => r.lines > 600).length}. Excluded roots: ${EXCLUDED.join(" ")}.`,
    "",
    "| lines | kind | name | gen | age min | file:start |",
    "| ---: | --- | --- | --- | ---: | --- |",
    ...outer.map((r) => `| ${r.lines} | ${r.kind} | \`${r.name}\` | ${r.generated ? "gen" : ""} | ${r.ageMinutes} | \`${r.file}:${r.start}\` |`),
  ].join("\n");
  if (out) writeFileSync(out, md + "\n"); else console.log(md);
  const json = opt("--json", "");
  if (json) writeFileSync(json, JSON.stringify(rows, null, 1));
  console.error(`[census] files=${scanned} functions=${rows.length} outer=${outer.length} outer>600=${outer.filter((r) => r.lines > 600).length}`);
}
