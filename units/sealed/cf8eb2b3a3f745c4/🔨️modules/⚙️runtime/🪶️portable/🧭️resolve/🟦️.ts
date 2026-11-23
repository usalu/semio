import { existsSync, readFileSync, realpathSync, statSync } from "node:fs";
import { dirname, isAbsolute, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { runtimeMessageV1 } from "../../🗣️messages/🟦️.ts";

const CONDITIONS = ["import", "node", "default"];
const EXTENSIONS = [".ts", ".tsx", ".mts", ".cts", ".js", ".mjs", ".cjs", ".jsx", ".json"];

function isFile(path: string): boolean { try { return statSync(path).isFile(); } catch { return false; } }
function isDirectory(path: string): boolean { try { return statSync(path).isDirectory(); } catch { return false; } }
function readJson(path: string): any { try { return JSON.parse(readFileSync(path, "utf8")); } catch { return undefined; } }

function probeFile(path: string): string | undefined {
  if (isFile(path)) return path;
  for (const extension of EXTENSIONS) if (isFile(path + extension)) return path + extension;
  return undefined;
}

function probeDirectory(path: string): string | undefined {
  if (!isDirectory(path)) return undefined;
  const manifest = readJson(join(path, "package.json"));
  for (const field of ["module", "main"]) if (typeof manifest?.[field] === "string") { const found = probeFile(resolve(path, manifest[field])) ?? probeDirectoryIndex(resolve(path, manifest[field])); if (found !== undefined) return found; }
  return probeDirectoryIndex(path);
}

function probeDirectoryIndex(path: string): string | undefined {
  for (const extension of EXTENSIONS) if (isFile(join(path, `index${extension}`))) return join(path, `index${extension}`);
  return undefined;
}

function selectTarget(target: unknown): string | undefined {
  if (typeof target === "string") return target;
  if (Array.isArray(target)) { for (const row of target) { const found = selectTarget(row); if (found !== undefined) return found; } return undefined; }
  if (target !== null && typeof target === "object") {
    for (const [condition, value] of Object.entries(target)) if (CONDITIONS.includes(condition)) { const found = selectTarget(value); if (found !== undefined) return found; }
  }
  return undefined;
}

function resolveExports(exports: unknown, subpath: string): string | undefined {
  let table: Record<string, unknown>;
  if (typeof exports === "string" || Array.isArray(exports) || exports === null || typeof exports !== "object" || !Object.keys(exports).some(key => key.startsWith("."))) table = { ".": exports };
  else table = exports as Record<string, unknown>;
  if (subpath in table && !subpath.includes("*")) return selectTarget(table[subpath]);
  let best: { key: string; capture: string } | undefined;
  for (const key of Object.keys(table)) {
    const star = key.indexOf("*");
    if (star < 0) continue;
    const prefix = key.slice(0, star), suffix = key.slice(star + 1);
    if (subpath.startsWith(prefix) && subpath.endsWith(suffix) && subpath.length >= key.length - 1 && (best === undefined || prefix.length > best.key.indexOf("*"))) best = { key, capture: subpath.slice(prefix.length, subpath.length - suffix.length) };
  }
  if (best === undefined) return undefined;
  return selectTarget(table[best.key])?.replaceAll("*", best.capture);
}

/** 🧭️ Resolves a module specifier from a directory or file the way ESM tooling does (relative, absolute, `file:` and package `exports`/`main`), as our own code over `node:fs`. */
export function resolveModuleV1(specifier: string, from: string): string {
  const path = resolveUnresolvedV1(specifier, from);
  return path.startsWith("node:") ? path : realpathSync(path);
}

function resolveUnresolvedV1(specifier: string, from: string): string {
  const base = isDirectory(from) ? from : dirname(from);
  if (specifier.startsWith("file:")) return fileURLToPath(specifier);
  if (specifier.startsWith("node:")) return specifier;
  if (specifier.startsWith(".") || isAbsolute(specifier)) {
    const path = resolve(base, specifier), found = probeFile(path) ?? probeDirectory(path);
    if (found === undefined) throw new Error(runtimeMessageV1("moduleNotFound", { specifier, from: base }));
    return found;
  }
  const parts = specifier.split("/"), name = specifier.startsWith("@") ? parts.slice(0, 2).join("/") : parts[0]!, rest = specifier.slice(name.length), subpath = rest === "" ? "." : `.${rest}`;
  for (let directory = resolve(base); ; directory = dirname(directory)) {
    const root = join(directory, "node_modules", name);
    if (existsSync(root)) {
      const manifest = readJson(join(root, "package.json"));
      if (manifest?.exports !== undefined) {
        const target = resolveExports(manifest.exports, subpath);
        if (target === undefined) throw new Error(runtimeMessageV1("packageSubpath", { subpath, name }));
        return resolve(root, target);
      }
      if (subpath === ".") { const found = probeDirectory(root); if (found !== undefined) return found; }
      else { const path = join(root, subpath), found = probeFile(path) ?? probeDirectory(path); if (found !== undefined) return found; }
    }
    if (dirname(directory) === directory) break;
  }
  throw new Error(runtimeMessageV1("moduleNotFound", { specifier, from: base }));
}
