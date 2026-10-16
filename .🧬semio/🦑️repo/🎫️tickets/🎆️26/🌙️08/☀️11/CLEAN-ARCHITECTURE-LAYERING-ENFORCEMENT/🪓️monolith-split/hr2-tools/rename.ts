/** 🏷️ hr2 folder-rename planner and applier: renames banned-stem directories and rewrites every literal reference in context. Usage: bun rename.ts <plan|apply> <spec.json> [out.json] */
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, readFileSync, renameSync, statSync, writeFileSync, lstatSync, readdirSync } from "node:fs";
import { dirname, join, normalize, relative, resolve, sep } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const BOUND_LEFT = "(?<![A-Za-z0-9_-])";
const BOUND_RIGHT = "(?![A-Za-z0-9_-])";

type Family = { id: string; old: string; new: string; anchors: string[]; scopeAlways?: boolean; forceFiles?: string[]; variableRooted?: boolean; dirs?: string[]; dirFilter?: string; artifactPrefixDepth?: number };
type Spec = { families: Family[] };
type Edit = { start: number; end: number; rule: string };

function git(args: string[]): string[] {
  try {
    return execFileSync("git", ["-c", "core.quotepath=false", ...args], { cwd: ROOT, maxBuffer: 1 << 30, stdio: ["ignore", "pipe", "ignore"] }).toString("utf8").split("\n").filter((x) => x.length > 0);
  } catch (error) {
    const out = (error as { stdout?: Buffer }).stdout;
    return out ? out.toString("utf8").split("\n").filter((x) => x.length > 0) : [];
  }
}

let trackedCache: string[] | null = null;
let untrackedCache: string[] | null = null;
function trackedFiles(): string[] {
  return (trackedCache ??= git(["ls-files"]));
}
function untrackedFiles(): string[] {
  return (untrackedCache ??= git(["ls-files", "-o", "--exclude-standard"]));
}

function candidateFiles(token: string): string[] {
  const tracked = git(["grep", "-l", "-F", token, "--", ".", ":!.🧬semio"]);
  const untracked = untrackedFiles().filter((f) => !f.startsWith(".🧬semio/"));
  const found = new Set<string>(tracked);
  for (const file of untracked) {
    try {
      if (readFileSync(join(ROOT, file), "utf8").includes(token)) found.add(file);
    } catch {}
  }
  return [...found].sort();
}

function discoverDirs(family: Family): string[] {
  const all = new Set<string>();
  const files = [...trackedFiles(), ...untrackedFiles()];
  const needle = `/${family.old}/`;
  for (const file of files) {
    let at = 0;
    while (true) {
      const i = file.indexOf(needle, at);
      if (i < 0) break;
      all.add(file.slice(0, i + needle.length - 1));
      at = i + 1;
    }
  }
  const re = family.dirFilter ? new RegExp(family.dirFilter, "u") : null;
  return [...all].filter((d) => d.endsWith(`/${family.old}`) && (!re || re.test(d))).sort();
}

function decode(buffer: Buffer): string | null {
  if (buffer.includes(0)) return null;
  const text = buffer.toString("utf8");
  return Buffer.from(text, "utf8").equals(buffer) ? text : null;
}

function pathPrefix(text: string, index: number): string {
  const left = text.slice(Math.max(0, index - 400), index);
  const m = /[^\s"'`()<>,;|{}\[\]=]*$/u.exec(left);
  return m ? m[0] : "";
}

const REPO_ROOTS = ["✏️s/", "🧰️framework/", "🌎️hub/", "♻️mit-bestand/", "🏢️semio-tech/", "🎓️teaching/", "👴️leutwiler/", "🧪️tests/", ".vscode/", ".github/"];

function classify(family: Family, dirSet: Set<string>, artifactDirs: string[], file: string, text: string, index: number): string {
  if (family.forceFiles?.includes(file)) return "forced";
  const prefix = pathPrefix(text, index);
  for (const anchor of family.anchors) if (prefix.endsWith(anchor + "/") || prefix.endsWith(anchor)) return "anchor";
  const joined = prefix.replace(/^\/Users\/ueli\/Documents\/semio\//u, "");
  if (prefix.startsWith(".")) {
    const target = normalize(join(dirname(file), prefix.endsWith("/") ? prefix.slice(0, -1) + "/" + family.old : prefix + family.old)).split(sep).join("/");
    return dirSet.has(target) ? "relative" : "foreign-relative";
  }
  if (prefix.startsWith("${") || prefix.startsWith("$")) return family.variableRooted ? "variable-rooted" : "unanchored";
  if (joined.endsWith("/") && (prefix !== joined || REPO_ROOTS.some((root) => joined.startsWith(root)))) {
    return dirSet.has(joined + family.old) ? "absolute" : "foreign-path";
  }
  if (artifactDirs.some((dir) => file.startsWith(dir + "/"))) return "prose-in-artifact";
  return "unanchored";
}

function planFamily(family: Family) {
  const dirs = family.dirs ?? discoverDirs(family);
  const dirSet = new Set(dirs);
  const artifactDirs = [...new Set(dirs.map((d) => d.split("/").slice(0, family.artifactPrefixDepth ?? 6).join("/")))];
  const token = new RegExp(`${BOUND_LEFT}${family.old.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}${BOUND_RIGHT}`, "gu");
  const files = candidateFiles(family.old);
  const perFile: Record<string, Edit[]> = {};
  const skipped: { file: string; rule: string; context: string }[] = [];
  const counts: Record<string, number> = {};
  for (const file of files) {
    const buffer = readFileSync(join(ROOT, file));
    const text = decode(buffer);
    if (text === null) continue;
    token.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = token.exec(text)) !== null) {
      const rule = classify(family, dirSet, artifactDirs, file, text, m.index);
      counts[rule] = (counts[rule] ?? 0) + 1;
      if (rule === "foreign-path" || rule === "foreign-relative" || rule === "unanchored") {
        skipped.push({ file, rule, context: JSON.stringify(text.slice(Math.max(0, m.index - 70), m.index + family.old.length + 30)) });
        continue;
      }
      (perFile[file] ??= []).push({ start: m.index, end: m.index + family.old.length, rule });
    }
  }
  return { id: family.id, old: family.old, new: family.new, dirs, counts, skipped, perFile, files: files.length };
}

function applyFamily(plan: ReturnType<typeof planFamily>, ranges: { from: string; to: string }[]) {
  const newName = plan.new;
  const renamed: string[] = [];
  const written: string[] = [];
  const digests: [string, string][] = [];
  const mapPath = (p: string): string => {
    for (const r of ranges) if (p === r.from || p.startsWith(r.from + "/")) return r.to + p.slice(r.from.length);
    return p;
  };
  for (const dir of plan.dirs) {
    const parent = dirname(dir);
    const target = join(parent, newName);
    if (!existsSync(join(ROOT, dir))) continue;
    if (existsSync(join(ROOT, target))) throw new Error(`target exists: ${target}`);
    ranges.push({ from: dir, to: target });
  }
  for (const [file, edits] of Object.entries(plan.perFile)) {
    const sourcePath = join(ROOT, file);
    const buffer = readFileSync(sourcePath);
    const text = decode(buffer);
    if (text === null) throw new Error(`binary now: ${file}`);
    let out = "";
    let cursor = 0;
    for (const e of edits) {
      if (text.slice(e.start, e.end) !== plan.old) throw new Error(`stale plan for ${file}@${e.start}`);
      out += text.slice(cursor, e.start) + newName;
      cursor = e.end;
    }
    out += text.slice(cursor);
    const reread = readFileSync(sourcePath);
    if (!reread.equals(buffer)) throw new Error(`file changed under us: ${file}`);
    writeFileSync(sourcePath, out);
    written.push(file);
    digests.push([createHash("sha256").update(buffer).digest("hex"), createHash("sha256").update(out).digest("hex")]);
  }
  for (const dir of plan.dirs) {
    const target = join(dirname(dir), newName);
    if (!existsSync(join(ROOT, dir))) continue;
    renameSync(join(ROOT, dir), join(ROOT, target));
    renamed.push(`${dir} -> ${target}`);
  }
  return { renamed, written: written.length, digests, files: written };
}

const [mode, specPath, outPath] = process.argv.slice(2);
const spec: Spec = JSON.parse(readFileSync(specPath!, "utf8"));
const result: unknown[] = [];
for (const family of spec.families) {
  const plan = planFamily(family);
  if (mode === "plan") {
    result.push({ id: plan.id, old: plan.old, new: plan.new, dirs: plan.dirs, files: plan.files, counts: plan.counts, editedFiles: Object.keys(plan.perFile).length, editedList: Object.keys(plan.perFile), skipped: plan.skipped });
    console.log(`${plan.id}: dirs=${plan.dirs.length} candidates=${plan.files} editedFiles=${Object.keys(plan.perFile).length} counts=${JSON.stringify(plan.counts)} skipped=${plan.skipped.length}`);
  } else if (mode === "apply") {
    const ranges: { from: string; to: string }[] = [];
    const done = applyFamily(plan, ranges);
    result.push({ id: plan.id, renamed: done.renamed, written: done.written, files: done.files, digests: done.digests });
    console.log(`${plan.id}: renamed=${done.renamed.length} written=${done.written}`);
  }
}
if (outPath) writeFileSync(outPath, JSON.stringify(result, null, 2));
