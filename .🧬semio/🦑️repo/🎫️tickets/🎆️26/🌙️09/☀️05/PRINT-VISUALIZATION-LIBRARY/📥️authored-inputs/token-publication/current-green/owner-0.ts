import { lstat, readdir } from "node:fs/promises";
import { existsSync, lstatSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";

/** 🪶️ Writes generated text only when its bytes changed, preserving no-op prerequisite mtimes. */
export function writeGeneratedFileIfChanged(path: string, content: string): boolean {
  if (existsSync(path)) {
    const metadata = lstatSync(path);
    if (!metadata.isFile() || metadata.isSymbolicLink()) throw new Error(`Invalid generated file: ${path}`);
    if (readFileSync(path).equals(Buffer.from(content,"utf8"))) return false;
  }
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, content, "utf8");
  return true;
}

/** 🌱️ Makes sure the generated file at `path` exists, empty when it is new, and says whether it was new: a native
 * exporter whose own freshness test compiles its published mirror in (`include_str!`) builds on a checkout that has no
 * mirror yet. */
export function seedGeneratedFile(path: string): boolean {
  if (existsSync(path)) return false;
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, "", "utf8");
  return true;
}

/** 🗂️ Collects regular staged files without following links or retaining compiler directory state. */
export async function collectArtifactFiles(root: string, signal?: AbortSignal): Promise<ReadonlyMap<string, string>> {
  signal?.throwIfAborted();
  const metadata = await lstat(root);
  if (!metadata.isDirectory() || metadata.isSymbolicLink()) throw new Error(`Invalid artifact root: ${root}`);
  const files = new Map<string, string>(), pending = [root];
  while (pending.length) {
    signal?.throwIfAborted();
    const directory = pending.pop()!;
    for (const entry of await readdir(directory, { withFileTypes: true })) {
      signal?.throwIfAborted();
      const path = join(directory, entry.name);
      if (entry.isDirectory()) pending.push(path);
      else if (entry.isFile()) files.set(relative(root, path).replaceAll("\\", "/"), path);
      else throw new Error(`Unsupported artifact file: ${path}`);
    }
  }
  return new Map([...files].sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0));
}
