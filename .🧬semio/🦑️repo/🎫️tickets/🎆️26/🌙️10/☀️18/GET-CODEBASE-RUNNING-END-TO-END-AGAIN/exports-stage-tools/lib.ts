/** 🧰 Shared paths, tracked-file listing and the workspace package model of the exports-stage tools. */
import { existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { spawnSync } from "node:child_process";

export const ROOT = resolve(import.meta.dir, "../../../../../../../..");
export const TICKET = resolve(import.meta.dir, "..");
export const GENERATED = resolve(TICKET, "🗑️generated/exports-stage");
export const SKIP = ["/node_modules/", "/target/", "/dist/", "/📤️dist/", "/🗑️generated/", "/🤖️generated/", "/.cache/", "/pkg/"];
export const posix = (p: string): string => p.split("\\").join("/");

export type Pkg = { name: string; dir: string; packageJson: string; sourceOwner: string; json: any; exports: Record<string, unknown>; deps: Record<string, string>; dev: Record<string, string>; peer: Record<string, string> };

export function listTracked(pattern: string, ttlMs = 6 * 3600_000): string[] {
  mkdirSync(GENERATED, { recursive: true });
  const cache = resolve(GENERATED, `ls-${pattern.replace(/[^a-z0-9]/giu, "_")}.txt`);
  if (existsSync(cache) && Date.now() - statSync(cache).mtimeMs < ttlMs) return readFileSync(cache, "utf8").split("\n").filter(Boolean);
  const out = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z", "--", pattern], { cwd: ROOT, maxBuffer: 1 << 28, encoding: "utf8" });
  if (out.status !== 0) throw Error("git ls-files failed");
  const files = out.stdout.split("\0").filter(f => f && !f.startsWith(".🧬semio/") && !SKIP.some(s => ("/" + f).includes(s)) && existsSync(resolve(ROOT, f)));
  writeFileSync(cache, files.join("\n") + "\n");
  return files;
}

export function loadPackages(): { all: Pkg[]; byName: Map<string, Pkg>; byJson: Map<string, Pkg> } {
  const files = listTracked("*package.json").filter(f => f.endsWith("package.json"));
  const all: Pkg[] = [], byName = new Map<string, Pkg>(), byJson = new Map<string, Pkg>();
  for (const f of files) {
    let j: any;
    try { j = JSON.parse(readFileSync(resolve(ROOT, f), "utf8")); } catch { continue; }
    const dir = posix(dirname(f));
    const sourceOwner = dir.includes("/📦️packages/") ? dir.split("/📦️packages/")[0]! : dir;
    const pkg: Pkg = { name: j.name ?? "", dir, packageJson: f, sourceOwner, json: j, exports: typeof j.exports === "object" && j.exports ? j.exports : {}, deps: j.dependencies ?? {}, dev: j.devDependencies ?? {}, peer: j.peerDependencies ?? {} };
    all.push(pkg);
    byJson.set(f, pkg);
    if (pkg.name && !byName.has(pkg.name)) byName.set(pkg.name, pkg);
  }
  return { all, byName, byJson };
}

export function listAll(ttlMs = 30 * 60_000): string[] {
  mkdirSync(GENERATED, { recursive: true });
  const cache = resolve(GENERATED, "all-files.txt");
  if (existsSync(cache) && Date.now() - statSync(cache).mtimeMs < ttlMs) return readFileSync(cache, "utf8").split("\n").filter(Boolean);
  const out = spawnSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], { cwd: ROOT, maxBuffer: 1 << 30, encoding: "utf8" });
  if (out.status !== 0) throw Error("git ls-files failed");
  const files = out.stdout.split("\0").filter(f => f && !f.startsWith(".🧬semio/") && existsSync(resolve(ROOT, f)));
  writeFileSync(cache, files.join("\n") + "\n");
  return files;
}
