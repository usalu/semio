import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { GENERATED, ROOT, SKIP_SEGMENTS, posix, readJson } from "./lib.ts";

export type Pkg = { name: string; dir: string; packageJson: string; sourceOwner: string; exports: Record<string, unknown>; deps: Set<string> };

/** Every workspace package manifest of the repository, by name (first manifest wins, duplicates are listed). */
export function packageIndex(): { byName: Map<string, Pkg>; duplicates: Map<string, string[]>; all: Pkg[] } {
  const cache = resolve(GENERATED, "package-json-files.txt");
  let files: string[];
  if (existsSync(cache) && Date.now() - statSync(cache).mtimeMs < 6 * 3600_000) files = readFileSync(cache, "utf8").split("\n").filter(Boolean);
  else {
    const out = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z", "--", "*package.json"], { cwd: ROOT, maxBuffer: 1 << 26, encoding: "utf8" });
    files = out.stdout.split("\0").filter(f => f && f.endsWith("package.json") && !f.startsWith(".🧬semio/") && !SKIP_SEGMENTS.some(s => ("/" + f).includes(s)));
    writeFileSync(cache, files.join("\n") + "\n");
  }
  const byName = new Map<string, Pkg>(), duplicates = new Map<string, string[]>(), all: Pkg[] = [];
  for (const f of files) {
    const abs = resolve(ROOT, f);
    if (!existsSync(abs)) continue;
    let j: any;
    try { j = readJson(abs); } catch { continue; }
    if (typeof j.name !== "string") continue;
    const dir = posix(dirname(f));
    const sourceOwner = dir.includes("/📦️packages/") ? dir.split("/📦️packages/")[0]! : dir;
    const deps = new Set<string>([...Object.keys(j.dependencies ?? {}), ...Object.keys(j.devDependencies ?? {}), ...Object.keys(j.peerDependencies ?? {})]);
    const pkg: Pkg = { name: j.name, dir, packageJson: f, sourceOwner, exports: typeof j.exports === "object" && j.exports ? j.exports : {}, deps };
    all.push(pkg);
    if (byName.has(j.name)) { (duplicates.get(j.name) ?? duplicates.set(j.name, [byName.get(j.name)!.packageJson]).get(j.name)!).push(f); continue; }
    byName.set(j.name, pkg);
  }
  return { byName, duplicates, all };
}

/** Nearest manifest above a file: the package whose dependency list a bare import is checked against. */
export function nearestManifest(file: string, all: Pkg[]): Pkg | undefined {
  let best: Pkg | undefined;
  for (const p of all) if ((file === p.dir || file.startsWith(p.dir + "/")) && (!best || p.dir.length > best.dir.length)) best = p;
  if (best) return best;
  let dir = dirname(file);
  while (dir && dir !== ".") {
    const pj = resolve(ROOT, dir, "package.json");
    if (existsSync(pj)) { try { const j = readJson(pj); return { name: j.name ?? dir, dir, packageJson: posix(relative(ROOT, pj)), sourceOwner: dir, exports: {}, deps: new Set([...Object.keys(j.dependencies ?? {}), ...Object.keys(j.devDependencies ?? {}), ...Object.keys(j.peerDependencies ?? {})]) }; } catch { /* continue */ } }
    dir = dirname(dir);
  }
  return undefined;
}
