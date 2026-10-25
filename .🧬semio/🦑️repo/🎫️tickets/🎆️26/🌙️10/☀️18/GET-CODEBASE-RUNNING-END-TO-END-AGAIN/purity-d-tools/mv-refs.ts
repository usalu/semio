import { existsSync, mkdirSync, readFileSync, readdirSync, renameSync, statSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { basename, dirname, join, posix, relative, resolve } from "node:path";

/**
 * usage: bun mv-refs.ts <pairs.tsv: old<TAB>new, repo-relative, files or directories> [dry|apply]
 * Moves files/directories and rewrites every string literal whose resolution (relative to its own file, or repo-root relative) hits a moved path.
 * Literals inside moved files are re-resolved from the old location and re-expressed from the new location.
 */
const root = "/Users/ueli/Documents/semio";
const [, , listFile, mode = "dry"] = process.argv;
if (!listFile) throw new Error("usage: mv-refs.ts <pairs.tsv> [dry|apply]");
const nfc = (value: string): string => value.normalize("NFC");
const pairs = readFileSync(listFile, "utf8").split("\n").filter(Boolean).map((line) => line.split("\t").map(nfc) as [string, string]);
const abs = (path: string): string => resolve(root, path);
const rel = (path: string): string => nfc(relative(root, path).split("\\").join("/"));
const isDirectory = (path: string): boolean => { try { return statSync(abs(path)).isDirectory(); } catch { return false; } };
const exists = (path: string): boolean => existsSync(abs(path));
for (const [from, to] of pairs) {
  if (!exists(from)) throw new Error(`missing source: ${from}`);
  if (exists(to)) throw new Error(`target exists: ${to}`);
}
const mapPath = (path: string): string => {
  const normal = nfc(path);
  for (const [from, to] of pairs) {
    if (normal === from) return to;
    if (normal.startsWith(`${from}/`)) return `${to}${normal.slice(from.length)}`;
  }
  return normal;
};
const moved = (path: string): boolean => mapPath(path) !== nfc(path);
const resolves = (path: string): boolean => exists(path);

const needles = new Set<string>();
for (const [from] of pairs) needles.add(basename(from));
const ignoreGlobs = ["!node_modules", "!.git", "!**/🗑️generated/**", "!.🧬semio/🦑️repo/🎫️tickets/**", "!**/target/**", "!**/dist/**", "!bazel-*", "!.cache", "!**/.🧾️store/**", "!**/📤️dist/**"];
const candidates = new Set<string>();
const needleList = [...needles];
for (let offset = 0; offset < needleList.length; offset += 40) {
  const args = ["-l", "-F", "--no-messages", ...ignoreGlobs.flatMap((glob) => ["-g", glob]), ...needleList.slice(offset, offset + 40).flatMap((needle) => ["-e", needle]), "."];
  const found = spawnSync("rg", args, { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 }).stdout;
  for (const line of found.split("\n")) if (line) candidates.add(nfc(line.replace(/^\.\//, "")));
}
const movedFiles: string[] = [];
const walk = (path: string): void => {
  if (isDirectory(path)) {
    for (const entry of readdirSync(abs(path))) {
      if (entry === "node_modules" || entry === ".git" || entry === "target" || entry === "dist") continue;
      walk(nfc(`${path}/${entry}`));
    }
  } else movedFiles.push(path);
};
for (const [from] of pairs) walk(from);
for (const file of movedFiles) candidates.add(file);

const literal = /(["'`])((?:(?!\1)[^\\\n]|\\.)*?)\1/gu;
const edits = new Map<string, string>();
const warnings: string[] = [];
let literalsRewritten = 0;
type Candidate = { readonly target: string; readonly kind: "dir" | "root" };
const skip = process.env.MV_SKIP ? new RegExp(process.env.MV_SKIP, "u") : null;
const rewriteFile = (file: string): void => {
  if (skip?.test(file)) return;
  let text: string;
  try { text = readFileSync(abs(file), "utf8"); } catch { return; }
  if (text.includes("\u0000")) return;
  const newFile = mapPath(file);
  const oldDir = dirname(file), newDir = dirname(newFile);
  const manifest = basename(file) === "Cargo.toml";
  let count = 0;
  const rewritten = text.replace(literal, (whole, quote: string, body: string) => {
    if (body.length === 0 || body.length > 400 || body.includes("${") || body.includes("\\") || /[*?{}]/u.test(body)) return whole;
    const bodyNfc = nfc(body);
    const explicit = bodyNfc.startsWith("./") || bodyNfc.startsWith("../");
    const options: Candidate[] = [];
    if (explicit) options.push({ target: rel(resolve(abs(oldDir), bodyNfc)), kind: "dir" });
    else if (!bodyNfc.startsWith("/") && !/^[a-z][a-z0-9+.-]*:/u.test(bodyNfc) && !bodyNfc.startsWith("@")) {
      options.push({ target: rel(resolve(abs(oldDir), bodyNfc)), kind: "dir" });
      options.push({ target: rel(resolve(root, bodyNfc)), kind: "root" });
    } else return whole;
    const loose = !explicit && !manifest && !bodyNfc.includes("/");
    for (const option of options) {
      if (option.target.startsWith("..") || !resolves(option.target)) continue;
      const newTarget = mapPath(option.target);
      if (loose && newTarget === option.target) continue;
      if (option.kind === "root") {
        if (newTarget === option.target) return whole;
        count += 1;
        return `${quote}${newTarget}${bodyNfc.endsWith("/") && !newTarget.endsWith("/") ? "/" : ""}${quote}`;
      }
      if (newTarget === option.target && newDir === oldDir) return whole;
      let replacement = posix.relative(newDir, newTarget) || ".";
      if (explicit && !replacement.startsWith(".")) replacement = `./${replacement}`;
      if (bodyNfc.endsWith("/") && !replacement.endsWith("/")) replacement += "/";
      if (replacement === bodyNfc) return whole;
      count += 1;
      return `${quote}${replacement}${quote}`;
    }
    return whole;
  });
  if (count > 0) { edits.set(file, rewritten); literalsRewritten += count; }
  if (movedFiles.includes(file) && oldDir !== newDir && /import\.meta\.(dir|dirname|url|path)|__dirname|__file/u.test(text) && /["'`]\.\.["'`]/u.test(text)) warnings.push(`${file}: path arithmetic with ".." segments`);
};
for (const file of [...candidates].sort()) rewriteFile(file);

const recent = [...edits.keys(), ...movedFiles].filter((file) => { try { return Date.now() - statSync(abs(file)).mtimeMs < 30 * 60 * 1000; } catch { return false; } });
console.log(`pairs ${pairs.length} moved-files ${movedFiles.length} candidates ${candidates.size} files-edited ${edits.size} literals ${literalsRewritten} warnings ${warnings.length} recent ${recent.length}`);
for (const line of warnings) console.log("WARN", line);
for (const file of recent) console.log("RECENT", file);
if (process.env.SHOW) for (const [file, text] of edits) console.log("EDIT", file, text.length);
if (mode === "apply") {
  if (recent.length > 0 && !process.env.ALLOW_RECENT) throw new Error("recent peer edits; set ALLOW_RECENT=1 to proceed");
  for (const [file, text] of edits) {
    const target = moved(file) ? abs(mapPath(file)) : abs(file);
    if (moved(file)) continue;
    writeFileSync(target, text);
  }
  const pendingMovedEdits = new Map([...edits].filter(([file]) => moved(file)));
  for (const [from, to] of pairs) {
    mkdirSync(dirname(abs(to)), { recursive: true });
    renameSync(abs(from), abs(to));
  }
  for (const [file, text] of pendingMovedEdits) writeFileSync(abs(mapPath(file)), text);
  console.log("applied");
}
