#!/usr/bin/env python3
"""undeclared-g F1 patch (idempotent, anchored): the command closure of a script also names the module resolution configuration bun reads for every module it loads.
usage: patch.py <live script-inputs 🟨️.mjs> [--check]   -> rewrites the file in place (or only reports whether it is applied)"""
import sys, re
path = sys.argv[1]
text = open(path, encoding="utf8").read()
MARK = "resolutionConfigsV1"
if MARK in text:
    print("already applied"); sys.exit(0)
if "--check" in sys.argv:
    print("not applied"); sys.exit(1)
anchor_call = "  for (const entry of roots) await visit(entry);\n  const result = [...files].map((file) => closurePathNameV1(cache, workspaceRoot, file)).sort();"
assert text.count(anchor_call) == 1, "call anchor must be unique"
call = "  for (const entry of roots) await visit(entry);\n  for (const config of resolutionConfigsV1(compiler, host, workspaceRoot, compiled)) await visit(config, false);\n  const result = [...files].map((file) => closurePathNameV1(cache, workspaceRoot, file)).sort();"
text = text.replace(anchor_call, call)
anchor_fn = "/** 🧭️ Tracks literal relative command imports through the existing TypeScript tooling boundary"
assert text.count(anchor_fn) == 1, "function anchor must be unique"
fn = '''/** 🧰️ The module resolution configuration bun reads for every module of a command closure (checked with the kernel read oracle: `undeclared-g`): the nearest `package.json` and the nearest `tsconfig.json` above each module directory, and the `tsconfig.json` files an `extends` names; the walk stops below the workspace root, whose own files are shared inputs of every target. Answers through the observed host, so a config that appears or changes re-keys the walk. */
export function resolutionConfigsV1(compiler, host, workspaceRoot, modules) {
  const root = resolve(workspaceRoot), found = new Set(), nearest = new Map();
  const above = (directory, name) => {
    const key = `${name}\\0${directory}`;
    if (!nearest.has(key)) { let at = directory, hit; while (hit === undefined && at.startsWith(root + sep)) { const candidate = join(at, name); if (host.fileExists(candidate)) hit = candidate; at = dirname(at); } nearest.set(key, hit); }
    return nearest.get(key);
  };
  const extend = (config) => {
    if (found.has(config)) return;
    found.add(config);
    const named = compiler.readConfigFile(config, host.readFile).config?.extends;
    for (const specifier of [named].flat().filter((value) => typeof value === "string" && value.startsWith("."))) {
      const target = resolve(dirname(config), specifier), candidate = host.fileExists(target) ? target : `${target}.json`;
      if (candidate.startsWith(root + sep) && host.fileExists(candidate)) extend(candidate);
    }
  };
  for (const directory of new Set([...modules].map((file) => dirname(file)))) {
    const manifest = above(directory, "package.json"), config = above(directory, "tsconfig.json");
    if (manifest) found.add(manifest);
    if (config) extend(config);
  }
  return [...found].sort();
}

'''
text = text.replace(anchor_fn, fn + anchor_fn)
open(path, "w", encoding="utf8").write(text)
print("applied")
