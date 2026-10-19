import { existsSync, readFileSync, statSync, readdirSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";

export const ROOT = "/Users/ueli/Documents/semio";
export const TICKET = join(import.meta.dir, "..");
export const OWNER = "🧰️framework";

export const git = (args: string[]) => new TextDecoder().decode(Bun.spawnSync(["git", "-c", "core.quotepath=false", ...args], { cwd: ROOT }).stdout);
export const tracked: string[] = git(["ls-files", "-z"]).split("\0").filter(Boolean);
export const trackedSet = new Set(tracked);
export const trackedDirs = new Set<string>();
for (const f of tracked) { let d = dirname(f); while (d !== "." && !trackedDirs.has(d)) { trackedDirs.add(d); d = dirname(d); } }

export const strip = (s: string) => s.replace(/️/g, "");
export const PROJECT = "📋️project.json";
export const projectFiles = tracked.filter((f) => f.endsWith("project.json") && f.split("/").pop() === PROJECT && f.startsWith(OWNER + "/")).filter((f) => !f.split("/").some((s) => ["🧫️fixtures", "🧫️examples", "🧪️fixtures", "🧪️examples", "pkg"].map(strip).includes(strip(s))));

export function readJson(rel: string): any { return JSON.parse(readFileSync(join(ROOT, rel), "utf8")); }
export function projectRootOf(file: string): string { return dirname(file); }

const projectRoots = new Set(tracked.filter((f) => f.endsWith("project.json") && f.split("/").pop() === PROJECT).map((f) => dirname(f)));
export function nearestProjectRoot(file: string): string | null {
  let d = dirname(file);
  while (d !== ".") { if (projectRoots.has(d)) return d; d = dirname(d); }
  return null;
}
export { projectRoots };
