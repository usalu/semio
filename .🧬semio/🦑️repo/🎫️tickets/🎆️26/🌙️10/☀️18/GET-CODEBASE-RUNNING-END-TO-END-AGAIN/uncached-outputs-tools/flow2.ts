import { stripComments, read } from "./closure.ts";
import { WRITE } from "./writes.ts";

export type Finding = { index: number; strong: boolean; line: number; prim: string; arg: string; anchored: boolean; tmp: boolean };
const TMPRE = /mkdtemp|tmpdir|\btmp\b|\btemp\b|scratch|createTemp|makeTemp|withTemp|tempDir|TmpDir|TempDir|Tmp\b|Temp\b/i;
const STRONG = /\b(repoRoot|REPO_ROOT|REPO|projectRoot|workspaceRoot|packageRoot|PACKAGE_ROOT|checkout|sourceRoot)\b|import\.meta\.(dir|dirname|url|path)|process\.cwd\(\)|__dirname|\b(this|context|ctx)\.(root|repoRoot)\b/;
const ANCHOR = /\b(repoRoot|REPO_ROOT|REPO|ROOT|root|projectRoot|workspaceRoot|packageRoot|PACKAGE_ROOT|cwd|baseDir|rootDir|sourceRoot|checkout)\b|import\.meta\.(dir|dirname|url|path)|process\.cwd\(\)|__dirname/;
const IDENT = /[A-Za-z_$][\w$]*/g;

function balanced(s: string, open: number): string {
  let d = 0, q = "";
  for (let i = open; i < s.length; i++) {
    const c = s[i]!;
    if (q) { if (c === "\\") i++; else if (c === q) q = ""; continue; }
    if (c === '"' || c === "'" || c === "`") { q = c; continue; }
    if (c === "(" || c === "[" || c === "{") d++;
    else if (c === ")" || c === "]" || c === "}") { d--; if (d === 0) return s.slice(open + 1, i); }
  }
  return s.slice(open + 1);
}
function firstArg(args: string): string {
  let d = 0, q = "";
  for (let i = 0; i < args.length; i++) {
    const c = args[i]!;
    if (q) { if (c === "\\") i++; else if (c === q) q = ""; continue; }
    if (c === '"' || c === "'" || c === "`") { q = c; continue; }
    if (c === "(" || c === "[" || c === "{") d++;
    else if (c === ")" || c === "]" || c === "}") d--;
    else if (c === "," && d === 0) return args.slice(0, i);
  }
  return args;
}
const FSPRE = /(?:\bfs|fsp|promises|Fs|\bBun)\.$/;
export function analyzeAnchors(abs: string): Finding[] {
  const src = stripComments(read(abs));
  const assigns = new Map<string, string[]>();
  const add = (id: string, e: string) => (assigns.get(id) ?? assigns.set(id, []).get(id)!).push(e.slice(0, 500));
  for (const m of src.matchAll(/\b(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*(?::[^=;]+)?=\s*([^;]*)/g)) add(m[1]!, m[2]!);
  for (const m of src.matchAll(/\b(?:const|let|var)\s+[{\[]([^}\]]*)[}\]]\s*=\s*([^;]*)/g)) for (const id of m[1]!.match(IDENT) ?? []) add(id, m[2]!);
  const anchored = new Set<string>(), tmp = new Set<string>();
  const isTmp = (e: string) => TMPRE.test(e) || (e.match(IDENT) ?? []).some((i) => tmp.has(i));
  const isAnc = (e: string) => !isTmp(e) && (ANCHOR.test(e) || (e.match(IDENT) ?? []).some((i) => anchored.has(i)));
  for (let p = 0; p < 8; p++) {
    let g = false;
    for (const [id, es] of assigns) {
      if (!tmp.has(id) && es.every(isTmp)) { tmp.add(id); g = true; }
      if (!anchored.has(id) && !tmp.has(id) && es.some(isAnc)) { anchored.add(id); g = true; }
    }
    if (!g) break;
  }
  const strongSet = new Set<string>();
  for (let p = 0; p < 8; p++) { let g = false; for (const [id, es] of assigns) if (!strongSet.has(id) && !tmp.has(id) && es.some((e) => !isTmp(e) && (STRONG.test(e) || (e.match(IDENT) ?? []).some((i) => strongSet.has(i))))) { strongSet.add(id); g = true; } if (!g) break; }
  const out: Finding[] = [];
  const re = new RegExp(WRITE.source, "g");
  let m: RegExpExecArray | null;
  while ((m = re.exec(src))) {
    const prim = m[1]!;
    const pre = src.slice(Math.max(0, m.index - 10), m.index);
    if (/[\w$]\.$/.test(pre) && !FSPRE.test(pre) && ["cp", "rm", "rename", "mkdir", "symlink", "unlink", "copyFile", "writeFile", "appendFile", "rmSync"].includes(prim)) continue;
    if (prim === "mkdtempSync" || prim === "mkdtemp") continue;
    const arg = firstArg(balanced(src, m.index + m[0].length - 1)).trim();
    const t = isTmp(arg), a = isAnc(arg);
    out.push({ index: m.index, strong: !t && (STRONG.test(arg) || (arg.match(IDENT) ?? []).some((i) => strongSet.has(i))), line: src.slice(0, m.index).split("\n").length, prim, arg: arg.replace(/\s+/g, " ").slice(0, 140), anchored: a, tmp: t });
  }
  return out;
}
