import { createHash } from "node:crypto";
import { existsSync, readFileSync, statSync } from "node:fs";
import { dirname, resolve } from "node:path";

const IMPORT = /(?:from|import)\s*\(?\s*["'](\.[^"']*)["']/g;

/** 🔐 Every file reachable from the entries through relative imports, so a change to any kernel, schema or depiction file the checks read changes the hash. */
export function closureOf(entries: readonly string[]): string[] {
  const seen = new Set<string>();
  const walk = (file: string): void => {
    if (seen.has(file) || !existsSync(file) || statSync(file).isDirectory()) return;
    seen.add(file);
    const text = readFileSync(file, "utf8");
    for (const match of text.matchAll(IMPORT)) {
      const target = resolve(dirname(file), match[1]!);
      for (const candidate of [target, `${target}.ts`, `${target}/🟦️.ts`]) if (existsSync(candidate) && statSync(candidate).isFile()) walk(candidate);
    }
  };
  for (const entry of entries) walk(entry);
  return [...seen].sort();
}

/** 🔐 One sha1 over path and content of the whole import closure of the entries. */
export function closureHash(entries: readonly string[]): { hash: string; files: number } {
  const files = closureOf(entries);
  const hash = createHash("sha1");
  for (const file of files) hash.update(file).update("\0").update(readFileSync(file)).update("\0");
  return { hash: hash.digest("hex"), files: files.length };
}
