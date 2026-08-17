// [DEBUG] one-off repair: package.json "exports" targets must be "./"-relative and must not
// contain ".." — the emoji-prefix rename left them as "../../../<repo-relative-path-to-self>",
// which round-trips to the correct file but is rejected by Node/Bun's exports resolver
// (ERR_INVALID_PACKAGE_TARGET), breaking every bare-specifier workspace import repo-wide.
import { readdirSync, statSync, readFileSync, writeFileSync } from "fs";
import { join, dirname, relative, resolve } from "path";

const root = process.cwd();
const skipDirs = new Set(["node_modules", ".git", "target", "dist", "build", ".🦑️repo", ".repo"]);

const pkgFiles: string[] = [];
function walk(dir: string) {
  for (const entry of readdirSync(dir)) {
    if (skipDirs.has(entry)) continue;
    const full = join(dir, entry);
    const st = statSync(full);
    if (st.isDirectory()) walk(full);
    else if (entry === "package.json") pkgFiles.push(full);
  }
}
walk(root);

let fixedFiles = 0;
let fixedEntries = 0;
const problems: string[] = [];

for (const pkgPath of pkgFiles) {
  const raw = readFileSync(pkgPath, "utf-8");
  let pkg: any;
  try {
    pkg = JSON.parse(raw);
  } catch (e) {
    problems.push(`PARSE ERROR ${pkgPath}: ${(e as Error).message}`);
    continue;
  }
  if (!pkg.exports || typeof pkg.exports !== "object") continue;

  const pkgDir = dirname(pkgPath);
  let changed = false;

  for (const key of Object.keys(pkg.exports)) {
    const target = pkg.exports[key];
    if (typeof target !== "string") continue;
    if (!target.includes("..")) continue;

    const absoluteTarget = resolve(pkgDir, target);
    let rel = relative(pkgDir, absoluteTarget);
    if (!rel.startsWith(".")) rel = "./" + rel;

    if (rel.startsWith("..")) {
      problems.push(`ESCAPES PACKAGE ${pkgPath} [${key}]: ${target} -> ${rel}`);
      continue;
    }

    pkg.exports[key] = rel;
    changed = true;
    fixedEntries++;
  }

  if (changed) {
    writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + "\n", "utf-8");
    fixedFiles++;
  }
}

console.log(`[DEBUG] scanned ${pkgFiles.length} package.json files`);
console.log(`[DEBUG] fixed ${fixedEntries} exports entries across ${fixedFiles} files`);
if (problems.length) {
  console.log(`[DEBUG] ${problems.length} problems needing manual review:`);
  for (const p of problems) console.log("  " + p);
}
