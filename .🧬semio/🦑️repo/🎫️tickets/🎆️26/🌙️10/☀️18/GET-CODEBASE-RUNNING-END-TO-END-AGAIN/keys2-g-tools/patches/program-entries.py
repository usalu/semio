"""python3 program-entries.py <precision-dir> : lets the `program` read take `entries` (module globs) as its roots instead of the tsconfig include list.
The TypeScript program of explicit files (what `tsc <file> --noEmit` loads, type-only imports and declaration companions included) for projects without a tsconfig of their own (print, presentation, projektetage).
Anchored, idempotent, all-or-nothing: every anchor must occur exactly once (or the edit be present already). The schema is validated before it is written."""
import json, sys
base = sys.argv[1]
engine, schema = base + "/🟦️.ts", base + "/🧬️schema/🔣️.json"
E = open(engine, encoding="utf-8").read()
S = open(schema, encoding="utf-8").read()


def edit(text, anchor, new, marker):
    if marker in text:
        return text
    if text.count(anchor) != 1:
        sys.exit(f"anchor not unique ({text.count(anchor)}): {anchor[:80]}")
    return text.replace(anchor, new)


E = edit(E, "  | {readonly program: {readonly tsconfig: string}}\n", "  | {readonly program: {readonly tsconfig: string; readonly entries?: readonly string[]}}\n", "readonly entries?: readonly string[]")
E = edit(E, "/** ⌨️ The TypeScript program of a tsconfig:", '''/** 🔭️ The TypeScript source files the entry globs select: positive globs walked from their literal directory, negations applied, hidden and vendored trees skipped; a pattern without wildcards names one file. */
export function entryFilesV1(globs: readonly string[], host: Pick<InputPrecisionHostV1, "listDirectory" | "exists">): string[] {
  const strip = (glob: string) => normalize(glob.replace(WORKSPACE, "")), excluded = excludesV1(globs.filter((glob) => glob.startsWith("!")).map((glob) => strip(glob.slice(1)))), found = new Set<string>();
  for (const pattern of globs.filter((glob) => !glob.startsWith("!")).map(strip)) {
    const stop = pattern.search(GLOB_CHARACTERS);
    if (stop < 0) {
      if (SOURCE.test(pattern) && host.exists(pattern) && !excluded(pattern)) found.add(pattern);
      continue;
    }
    const pending = [normalize(pattern.slice(0, Math.max(0, pattern.lastIndexOf("/", stop))))];
    while (pending.length) {
      const directory = pending.pop()!;
      for (const entry of host.listDirectory(directory)) {
        const path = directory ? `${directory}/${entry.name}` : entry.name;
        if (SKIPPED.has(entry.name) || entry.name.startsWith(".") || excluded(path)) continue;
        if (entry.directory) pending.push(path);
        else if (SOURCE.test(path) && globMatchesV1(pattern, path)) found.add(path);
      }
    }
  }
  return sorted([...found]);
}

/** ⌨️ The TypeScript program of a tsconfig:''', "export function entryFilesV1")
E = edit(E, "export function programInputsV1(tsconfig: string, host: Pick<InputPrecisionHostV1, \"readText\" | \"exists\" | \"listDirectory\" | \"program\">): Promise<{patterns: string[]; files: string[]; roots: string[]}> {\n  const key = normalize(tsconfig);",
         "export function programInputsV1(tsconfig: string, host: Pick<InputPrecisionHostV1, \"readText\" | \"exists\" | \"listDirectory\" | \"program\">, entries?: readonly string[]): Promise<{patterns: string[]; files: string[]; roots: string[]}> {\n  const key = entries ? `${normalize(tsconfig)}\\0${JSON.stringify(entries)}` : normalize(tsconfig);", "entries ? `${normalize(tsconfig)}")
E = edit(E, "    known = deriveProgramV1(key, host);", "    known = deriveProgramV1(normalize(tsconfig), host, entries);", "deriveProgramV1(normalize(tsconfig), host, entries)")
E = edit(E, "async function deriveProgramV1(tsconfig: string, host: Pick<InputPrecisionHostV1, \"readText\" | \"exists\" | \"listDirectory\" | \"program\">): Promise<{patterns: string[]; files: string[]; roots: string[]}> {",
         "async function deriveProgramV1(tsconfig: string, host: Pick<InputPrecisionHostV1, \"readText\" | \"exists\" | \"listDirectory\" | \"program\">, entries?: readonly string[]): Promise<{patterns: string[]; files: string[]; roots: string[]}> {", "async function deriveProgramV1(tsconfig: string, host: Pick<InputPrecisionHostV1, \"readText\" | \"exists\" | \"listDirectory\" | \"program\">, entries?: readonly string[])")
E = edit(E, "  const config = configuration(tsconfig, host), include = config.include ?? [`${config.directory ? `${config.directory}/` : \"\"}**/*`], exclude = config.exclude ?? [];\n  const isExcluded = excludesV1(exclude), roots = new Set<string>(config.files.filter((file) => host.exists(file)));",
         "  const config = configuration(tsconfig, host), include = entries ? [] : config.include ?? [`${config.directory ? `${config.directory}/` : \"\"}**/*`], exclude = entries ? [] : config.exclude ?? [];\n  const isExcluded = excludesV1(exclude), roots = new Set<string>(entries ? entryFilesV1(entries, host) : config.files.filter((file) => host.exists(file)));\n  if (entries && !roots.size) fail(`Program entries select no TypeScript file: ${tsconfig}`);", "entries ? entryFilesV1(entries, host)")
E = edit(E, "receiptKey = JSON.stringify([\"input-precision-program\", 1, tsconfig, program.compiler]);", "receiptKey = JSON.stringify([\"input-precision-program\", 1, tsconfig, program.compiler, ...(entries ? [entries] : [])]);", "...(entries ? [entries] : [])")
E = edit(E, "      const program = await programInputsV1(read.program.tsconfig, host);\n      patterns.push(...program.patterns);\n      files.push(...program.files);\n      name = `program:${read.program.tsconfig}`;",
         "      const program = await programInputsV1(read.program.tsconfig, host, read.program.entries);\n      patterns.push(...program.patterns);\n      files.push(...program.files);\n      name = `program:${read.program.tsconfig}${read.program.entries ? `[${read.program.entries.length}]` : \"\"}`;", "read.program.entries ? `[${read.program.entries.length}]`")

d = json.loads(S)
props = d["definitions"]["ProgramRead"]["properties"]["program"]["properties"]
if "entries" not in props:
    props["entries"] = {"$ref": "#/definitions/Patterns"}
    S2 = json.dumps(d, ensure_ascii=False, indent=2) + "\n"
else:
    S2 = S
json.loads(S2)
open(engine, "w", encoding="utf-8").write(E)
open(schema, "w", encoding="utf-8").write(S2)
print("program entries applied to", base)

tests = base + "/🧪️tests/🟦️.ts"
T = open(tests, encoding="utf-8").read()
T = edit(T, "function tscListed(tree: Record<string, string>): string[] {", "function tscListed(tree: Record<string, string>, entries?: readonly string[]): string[] {", "entries?: readonly string[]): string[] {")
T = edit(T, 'const tsc = join(workspace, "node_modules/typescript/bin/tsc"), run = spawnSync(process.execPath, [tsc, "-p", join(root, "app/tsconfig.json"), "--listFilesOnly"], {encoding: "utf8", cwd: root});',
         'const tsc = join(workspace, "node_modules/typescript/bin/tsc"), run = spawnSync(process.execPath, [tsc, ...(entries ? [...entries.map((entry) => join(root, entry)), "--module", "ESNext", "--moduleResolution", "Bundler", "--allowImportingTsExtensions", "--noEmit", "--skipLibCheck", "--target", "ESNext", "--listFilesOnly"] : ["-p", join(root, "app/tsconfig.json"), "--listFilesOnly"])], {encoding: "utf8", cwd: root});', "...(entries ? [...entries.map(")
T = edit(T, 'test("the program receipt makes a warm run derive nothing,', '''/** 🔭️ A program rooted at entry globs is what `tsc <files> --listFilesOnly` lists for the files they select (type-only imports included), a negation removes a root, a glob that selects no TypeScript file is refused. */
test("a program rooted at entries equals what tsc lists for those files", async () => {
  for (const entries of [["{workspaceRoot}/app/src/main.ts"], ["{workspaceRoot}/app/src/**/*.ts", "!{workspaceRoot}/app/src/skip/**/*"]]) {
    const roots = entryFilesV1(entries, fixtureHost()), listed = tscListed(files, roots), program = await programInputsV1("app/tsconfig.json", fixtureHost(), entries), declared = new Set([...program.roots, ...program.files.filter((file) => file.endsWith(".ts"))]);
    expect(program.roots).toEqual(roots);
    expect([...declared].sort(), entries.join(" ")).toEqual(listed);
    expect(program.patterns).toEqual([]);
    expect(program.files).toContain("shared/package.json");
  }
  expect(entryFilesV1(["{workspaceRoot}/app/src/**/*.ts", "!{workspaceRoot}/app/src/skip/**/*"], fixtureHost())).not.toContain("app/src/skip/x.ts");
  await expect(programInputsV1("app/tsconfig.json", fixtureHost(), ["{workspaceRoot}/app/src/**/*.rs"])).rejects.toThrow(/no TypeScript file/u);
});

test("the program receipt makes a warm run derive nothing,''', "entryFilesV1")
T = edit(T, "deriveInputsV1, dispatchBranchesV1, excludesV1,", "deriveInputsV1, dispatchBranchesV1, entryFilesV1, excludesV1,", "dispatchBranchesV1, entryFilesV1,")
open(tests, "w", encoding="utf-8").write(T)
