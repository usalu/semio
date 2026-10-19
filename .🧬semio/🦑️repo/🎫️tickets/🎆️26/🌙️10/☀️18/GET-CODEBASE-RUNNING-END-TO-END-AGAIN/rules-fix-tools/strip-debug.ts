import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
const [listFile, backupRoot, apply] = process.argv.slice(2);
const files = readFileSync(listFile, "utf8").split("\n").filter(Boolean);
const CALL = /(console\.(?:log|error|warn|info)|eprintln!|println!)\s*\(/g;
function endOfCall(s: string, open: number): number {
  let depth = 0;
  const tpl: number[] = [];
  for (let i = open; i < s.length; i++) {
    const c = s[i];
    if (c === '"' || c === "'" ) {
      const q = c; i++;
      while (i < s.length && s[i] !== q) { if (s[i] === "\\") i++; i++; }
    } else if (c === "`") {
      i++;
      while (i < s.length && s[i] !== "`") {
        if (s[i] === "\\") i++;
        else if (s[i] === "$" && s[i + 1] === "{") {
          let d = 1; i += 2;
          while (i < s.length && d > 0) { if (s[i] === "{") d++; else if (s[i] === "}") d--; if (d > 0) i++; }
        }
        i++;
      }
    } else if (c === "(") depth++;
    else if (c === ")") { depth--; if (depth === 0) return i + 1; }
  }
  throw Error("unbalanced");
}
function matchBack(s: string, close: number): number {
  let depth = 0;
  for (let i = close; i >= 0; i--) { if (s[i] === ")") depth++; else if (s[i] === "(") { depth--; if (depth === 0) return i; } }
  return -1;
}
for (const f of files) {
  const orig = readFileSync(f, "utf8");
  let s = orig;
  const flags: string[] = [];
  let guard = 0;
  while (guard++ < 200) {
    const hit = s.indexOf("[DEBUG] ");
    if (hit < 0) break;
    let start = -1, open = -1;
    CALL.lastIndex = 0;
    let m: RegExpExecArray | null;
    const lineStart = s.lastIndexOf("\n", hit) + 1;
    const windowStart = Math.max(lineStart, 0);
    const seg = s.slice(windowStart, hit);
    const re = new RegExp(CALL.source, "g");
    let last: RegExpExecArray | null = null;
    while ((m = re.exec(seg))) last = m;
    if (!last) { flags.push(`NOCALL line ${s.slice(0, hit).split("\n").length}`); const nl = s.indexOf("\n", hit); s = s.slice(0, hit) + "[DEBUGSKIP] " + s.slice(hit + 8); continue; }
    start = windowStart + last.index; open = start + last[0].length - 1;
    const end = endOfCall(s, open);
    let a = start, b = end;
    if (s[b] === ";") b++;
    const lineEnd = (() => { const i = s.indexOf("\n", b); return i < 0 ? s.length : i; })();
    let prefix = s.slice(s.lastIndexOf("\n", a - 1) + 1, a);
    const suffix = s.slice(b, lineEnd);
    const pt = prefix.trimEnd();
    if (pt.endsWith(")")) {
      const o = matchBack(s, a - (prefix.length - pt.length) - 1);
      const before = s.slice(0, o).trimEnd();
      const kw = /(if|while|for)$/.exec(before);
      if (kw) { a = before.length - kw[1].length; flags.push(`BRACELESS ${kw[1]} line ${s.slice(0, a).split("\n").length}`);
        const eb = before.slice(0, before.length - kw[1].length);
        if (/else\s*$/.test(eb)) { a = eb.replace(/else\s*$/, "").length; flags.push("ELSE"); }
      }
    } else if (/else\s*$/.test(pt)) { a = s.slice(0, a).replace(/else\s*$/, "").length; flags.push("ELSE"); }
    else if (pt.endsWith("{") && /^\s*\}/.test(suffix)) flags.push(`BLOCKONLY line ${s.slice(0, a).split("\n").length}`);
    s = s.slice(0, a) + s.slice(b);
    const ls = s.lastIndexOf("\n", a - 1) + 1;
    const le = (() => { const i = s.indexOf("\n", a); return i < 0 ? s.length : i; })();
    if (s.slice(ls, le).trim() === "") s = s.slice(0, ls) + s.slice(le + 1);
  }
  s = s.replaceAll("[DEBUGSKIP] ", "[DEBUG] ");
  console.log(`${f}: ${flags.join("; ") || "ok"}`);
  if (apply === "apply") {
    const bp = join(backupRoot, f);
    mkdirSync(dirname(bp), { recursive: true });
    writeFileSync(bp, orig);
    writeFileSync(f, s);
  } else {
    const bp = join(backupRoot, "dry", f);
    mkdirSync(dirname(bp), { recursive: true });
    writeFileSync(bp, s);
  }
}
