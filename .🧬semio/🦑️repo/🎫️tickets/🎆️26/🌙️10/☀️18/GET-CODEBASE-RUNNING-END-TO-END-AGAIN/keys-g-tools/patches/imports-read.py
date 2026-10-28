"""python3 imports-read.py <precision-dir> : adds the `imports` read (import closure of the source files matched by entry globs) to the engine and its schema.
Anchored, idempotent, all-or-nothing: every anchor must occur exactly once (or the edit be present already)."""
import json, sys
base = sys.argv[1]
engine, schema = base + "/🟦️.ts", base + "/🧬️schema/🔣️.json"
E = open(engine, encoding="utf-8").read()
S = open(schema, encoding="utf-8").read()

def edit(text, anchor, new, marker):
    if marker in text:
        return text
    if text.count(anchor) != 1:
        sys.exit(f"anchor not unique ({text.count(anchor)}): {anchor[:70]}")
    return text.replace(anchor, new)

E = edit(E, "  | {readonly program: {readonly tsconfig: string}}\n", "  | {readonly program: {readonly tsconfig: string}}\n  | {readonly imports: {readonly entries: readonly string[]}}\n", "readonly imports:")
E = edit(E, "/** 🧮️ Derives the narrowest declared inputs of one target from its rule. */", '''const MODULE = /\\.(?:[cm]?[jt]sx?)$/u;
/** 🔭️ The module files the entry globs select: positive globs walked from their literal directory, negations applied, hidden and vendored trees skipped. */
export function entryFilesV1(globs: readonly string[], host: Pick<InputPrecisionHostV1, "listDirectory" | "exists">): string[] {
  const positive = globs.filter((glob) => !glob.startsWith("!")).map((glob) => glob.replace(WORKSPACE, "")), excluded = excludesV1(globs.filter((glob) => glob.startsWith("!")).map((glob) => glob.slice(1).replace(WORKSPACE, ""))), found = new Set<string>();
  for (const pattern of positive) {
    const stop = pattern.search(/[*?{[]/u), base = stop < 0 ? pattern : pattern.slice(0, pattern.lastIndexOf("/", stop));
    if (stop < 0) {
      if (MODULE.test(pattern) && host.exists(pattern) && !excluded(pattern)) found.add(pattern);
      continue;
    }
    const pending = [normalize(base)];
    while (pending.length) {
      const directory = pending.pop()!;
      for (const entry of host.listDirectory(directory)) {
        const path = directory ? `${directory}/${entry.name}` : entry.name;
        if (SKIPPED.has(entry.name) || excluded(path)) continue;
        if (entry.directory) pending.push(path);
        else if (MODULE.test(path) && globMatchesV1(pattern, path)) found.add(path);
      }
    }
  }
  return sorted([...found]);
}

/** 🧮️ Derives the narrowest declared inputs of one target from its rule. */''', "export function entryFilesV1")
E = edit(E, '''    } else if ("program" in read) {
      const program = await programInputsV1(read.program.tsconfig, host);''', '''    } else if ("imports" in read) {
      const entries = entryFilesV1(read.imports.entries, host);
      if (!entries.length) fail(`Imports read selects no module file: ${identity.project}:${identity.target}`);
      files.push(...(await host.importClosure(entries)));
      name = `imports:${entries.length}`;
    } else if ("program" in read) {
      const program = await programInputsV1(read.program.tsconfig, host);''', 'else if ("imports" in read)')
d = json.loads(S)
defs = d["definitions"]
if "ImportsRead" not in defs:
    defs["ImportsRead"] = {"type": "object", "additionalProperties": False, "required": ["imports"], "properties": {"imports": {"description": "The import closure of the module files the entry globs select: what a package's tests and sources can load, without the package manifests of a tsconfig program.", "type": "object", "additionalProperties": False, "required": ["entries"], "properties": {"entries": {"$ref": "#/definitions/Patterns"}}}}}
    for name in ("Read", "FlatRead"):
        defs[name]["oneOf"].append({"$ref": "#/definitions/ImportsRead"})
    S2 = json.dumps(d, ensure_ascii=False, indent=2) + "\n"
else:
    S2 = S
open(engine, "w", encoding="utf-8").write(E)
open(schema, "w", encoding="utf-8").write(S2)
print("imports read applied to", base)
