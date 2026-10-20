"""Builds the hand-authored part of the cli-zt plan from a tree (live checkout or overlay): anchored edits and full writes, each with the content hash it expects to find. The relative-import part is planned by rel-plan.ts over the tree this plan produces and merged by merge-plan.py. Usage: build-plan.py --root DIR --out plan-hand.json"""
import argparse, hashlib, json, os, re, sys

parser = argparse.ArgumentParser()
parser.add_argument("--root", default=os.getcwd())
parser.add_argument("--out", required=True)
args = parser.parse_args()
here = os.path.dirname(os.path.abspath(__file__))
staged = lambda name: open(os.path.join(here, "staged", name), encoding="utf-8", newline="").read()
sha = lambda text: hashlib.sha256(text.encode("utf-8")).hexdigest()
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = f"{L}/🏗️build"
Z = f"{B}/📥️source/🛗️zero-touch"
R = f"{L}/🎛️commands/🗂️registry"
O = f"{B}/⚙️operations"

def read(path):
    full = os.path.join(args.root, path)
    return open(full, encoding="utf-8", newline="").read() if os.path.exists(full) else None

edits, writes = {}, {}

def write(path, text):
    before = read(path)
    if path in writes or path in edits: sys.exit(f"duplicate target {path}")
    writes[path] = {"path": path, "sha256": None if before is None else sha(before), "afterSha256": sha(text), "content": text}

class Edit:
    def __init__(self, path):
        self.path, self.text = path, read(path)
        if self.text is None: sys.exit(f"missing edit target {path}")
        if path in writes or path in edits: sys.exit(f"duplicate target {path}")
        self.before, self.rows = self.text, []
    def sub(self, old, new, count=1):
        found = self.text.count(old)
        if found != count: sys.exit(f"{self.path}: anchor count {found} != {count}: {old[:80]!r}")
        self.text = self.text.replace(old, new)
        self.rows.append({"old": old, "new": new, "count": count})
        return self
    def done(self):
        edits[self.path] = {"path": self.path, "sha256": sha(self.before), "afterSha256": sha(self.text), "replacements": self.rows}

def json_transform(path, change):
    text = read(path)
    data = json.loads(text)
    if json.dumps(data, ensure_ascii=False, indent=2) + "\n" != text: sys.exit(f"{path}: formatting is not canonical JSON, transform refused")
    change(data)
    write(path, json.dumps(data, ensure_ascii=False, indent=2) + "\n")

# new modules, schema, data and laws
write(f"{R}/🧬️schema/🔣️.json", staged("01-registry-schema.json"))
write(f"{R}/🔣️.json", staged("02-registry.json"))
write(f"{R}/🟦️.ts", staged("03-registry.ts"))
write(f"{R}/🥾️provision/🟦️.ts", staged("04-provision.ts"))
write(f"{R}/🧪️tests/🟦️.ts", staged("11-registry-test.ts"))
write(f"{R}/🧪️tests/🔣️.json", staged("11-registry-fixture.json"))
write(f"{L}/🎛️commands/🧱️scale-fixture/🟦️.ts", staged("05-scale-fixture.ts"))
write(f"{L}/🎛️commands/🧪️test/🚦️root/🟦️.ts", staged("06-root-test.ts"))
write(f"{L}/🎛️commands/✅️verify/🌳️root/🟦️.ts", staged("07-root-verify.ts"))
write(f"{O}/🧪️tests/🛗️requires/🟦️.ts", staged("12-requires-test.ts"))
write(f"{O}/🧪️tests/🛗️requires/🔣️.json", staged("12-requires-fixture.json"))
write(f"{O}/🧪️tests/🧭️help-offline/🟦️.ts", staged("15-help-offline-test.ts"))

# the root script becomes registry driven; its entry law follows
write("📜️script.ts", staged("09-root-script.ts"))
write("🧪️tests/🧭️entrypoint/🟦️.ts", staged("10-entrypoint-test.ts"))
write("🧪️tests/🧭️entrypoint/🧫️fixtures/🔣️.json", staged("10-entrypoint-fixture.json"))

# operations catalogue: what every operation needs, and dependencies ensure
requires = {"run": "dependencies", "run-many": "dependencies", "affected": "dependencies", "exec": "dependencies", "watch": "dependencies", "graph": "dependencies", "show": "dependencies", "inputs": "dependencies", "reset": "none", "tools": "none", "preflight": "none", "incremental": "none", "dependencies": "none", "cargo": "none", "cache": "none", "capacity": "none", "remote-execution": "none"}
catalog = Edit(f"{O}/🔣️catalog.json")
for name, value in requires.items():
    catalog.sub(f'      "name": "{name}",\n      "synopsis"', f'      "name": "{name}",\n      "requires": "{value}",\n      "synopsis"')
catalog.sub('"summary": { "en": "Install or lock the workspace dependencies.", "de": "Installiert oder sperrt die Abhängigkeiten des Workspace." }', '"summary": { "en": "Install, lock or ensure the workspace dependencies.", "de": "Installiert, sperrt oder stellt die Abhängigkeiten des Workspace sicher." }')
catalog.sub('''          "summary": { "en": "Regenerate bun.lock after a manifest edit.", "de": "Erzeugt bun.lock nach einer Manifest-Änderung neu." }
        }
      ],''', '''          "summary": { "en": "Regenerate bun.lock after a manifest edit.", "de": "Erzeugt bun.lock nach einer Manifest-Änderung neu." }
        },
        {
          "name": "ensure",
          "synopsis": "dependencies ensure",
          "summary": { "en": "Install the locked dependencies only when they are missing or changed; commands that need them run this first.", "de": "Installiert die gesperrten Abhängigkeiten nur, wenn sie fehlen oder geändert sind; Befehle, die sie brauchen, führen dies zuerst aus." }
        }
      ],''')
catalog.sub('''        { "command": "repo dependencies sync --all", "description": { "en": "Install every owner's dependencies.", "de": "Installiert die Abhängigkeiten jedes Eigentümers." } }
''', '''        { "command": "repo dependencies sync --all", "description": { "en": "Install every owner's dependencies.", "de": "Installiert die Abhängigkeiten jedes Eigentümers." } },
        { "command": "repo dependencies ensure", "description": { "en": "Install the locked dependencies only if they are missing or changed.", "de": "Installiert die gesperrten Abhängigkeiten nur, wenn sie fehlen oder geändert sind." } }
''')
catalog.done()
(Edit(f"{O}/🧬️schema/🔣️.json")
    .sub('''      "required": ["name", "synopsis", "summary", "implementations", "dryRun", "progress", "cancellable", "flags", "examples"],''', '''      "required": ["name", "requires", "synopsis", "summary", "implementations", "dryRun", "progress", "cancellable", "flags", "examples"],''')
    .sub('''        "name": { "$ref": "#/definitions/OperationName" },
        "synopsis": { "type": "string", "minLength": 1 },
        "implementations": {''', '''        "name": { "$ref": "#/definitions/OperationName" },
        "requires": { "$ref": "#/definitions/Requires" },
        "synopsis": { "type": "string", "minLength": 1 },
        "implementations": {''')
    .sub('''    "Implementation": { "enum": ["native", "adapter", "neutral", "missing"] },''', '''    "Implementation": { "enum": ["native", "adapter", "neutral", "missing"] },
    "Requires": { "enum": ["none", "dependencies"] },''')
    .sub('''"cache", "capacity"]''', '''"cache", "capacity", "remote-execution"]''').done())
(Edit(f"{O}/🟦️.ts")
    .sub('''import {advanceScriptInvocation,type ScriptInvocation}''', '''import catalogue from "./🔣️catalog.json" with {type:"json"};
import {advanceScriptInvocation,type ScriptInvocation}''')
    .sub('''/** 🛠️ Names the build tools that implement the neutral operations. */''', '''/** 🛗️ What an operation needs before it starts: \\`none\\` runs on a fresh clone with only Bun, \\`dependencies\\` needs the locked dependencies, which the CLI ensures first; an unknown name needs them, so a typo never skips the provisioning. */
export function buildOperationRequiresV1(name:string):"none"|"dependencies"{return (catalogue.operations as readonly {name:string;requires:"none"|"dependencies"}[]).find(row=>row.name===name)?.requires??"dependencies";}

/** 🛠️ Names the build tools that implement the neutral operations. */'''.replace("\\`", "`")).done())

# dependencies ensure and the catalogue-derived provisioning gate of the build CLI
(Edit(f"{B}/🧭️invocation/🟦️.ts")
    .sub('export type BuildDependencyInvocationV1=Readonly<{mode:"sync"|"lock";selection:', 'export type BuildDependencyInvocationV1=Readonly<{mode:"sync"|"lock"|"ensure";selection:')
    .sub('''/** 🗝️ Admits only the schema-defined whole or scoped installer operation. */''', '''/** 🗝️ Admits only the schema-defined whole or scoped installer operation, or the selection-free ensure request. */''')
    .sub('''const [command,mode,selection,owner]=args,value=selection==="--all"&&args.length===3?''', '''const [command,mode,selection,owner]=args,value=mode==="ensure"?(args.length===2?{mode,selection:{kind:"all"}}:undefined):selection==="--all"&&args.length===3?''')
    .sub('''throw Error("Expected repo dependencies sync|lock --all|--scope <owner>");''', '''throw Error("Expected repo dependencies sync|lock --all|--scope <owner>, or repo dependencies ensure");''').done())
(Edit(f"{B}/🧬️schema/📥️dependencies.json")
    .sub('''        "sync",
        "lock"
''', '''        "sync",
        "lock",
        "ensure"
''').done())
(Edit(f"{B}/📜️script.ts")
    .sub('''  if(args[prefix]==="capacity"){const{runCapacityCommandV1}''', '''  if(args[prefix]==="capacity"){const{buildInvocationRequestsHelpV1}=await import("./🧭️invocation/🟦️.ts");if(buildInvocationRequestsHelpV1(args.slice(prefix))){const{buildHelpForArgumentsV1}=await import("./⚙️operations/📖️help/🟦️.ts");console.log(buildHelpForArgumentsV1(args.slice(prefix),environment));return;}const{runCapacityCommandV1}''')
    .sub('''      const {mode,selection}=readBuildDependencyInvocationV1(resolved.args),{SyncScript,RefreshLockScript}=await import(''', '''      const {mode,selection}=readBuildDependencyInvocationV1(resolved.args);
      if(mode==="ensure"){const {ensureBuildEnvironmentV1}=await import("./🚀️bootstrap/🟦️.ts");await ensureBuildEnvironmentV1(root,invocation,phaseControl);return;}
      const {SyncScript,RefreshLockScript}=await import(''')
    .sub('''    if(!["tools","reset","cache","remote-execution"].includes(command)){const {ensureBuildEnvironmentV1}=await import("./🚀️bootstrap/🟦️.ts");await ensureBuildEnvironmentV1(root,invocation,phaseControl);}''', '''    const {buildOperationRequiresV1}=await import("./⚙️operations/🟦️.ts");
    if(buildOperationRequiresV1(command)==="dependencies"){const {ensureBuildEnvironmentV1}=await import("./🚀️bootstrap/🟦️.ts");await ensureBuildEnvironmentV1(root,invocation,phaseControl);}''').done())

# the Nx argument translation must not validate the target of an inputs help request
(Edit(f"{L}/⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/🟦️.ts")
    .sub('''if(args[0]==="inputs"){if(args.length!==2||''', '''if(args[0]==="inputs"){if(args.includes("--help")||args.includes("-h"))return undefined;if(args.length!==2||''').done())

# a local function named require is no module require
symlinks = Edit(f"{L}/🧹️normalization/➰️symlink-targets/🟦️.ts")
text = symlinks.text
count = len(re.findall(r'(?<![A-Za-z0-9_$.])require\((?=")', text))
symlinks.sub("const require = (value: string): void =>", "const expectLexeme = (value: string): void =>")
for token in sorted(set(re.findall(r'(?<![A-Za-z0-9_$.])require\(("[^"]*")\)', symlinks.text))):
    symlinks.sub(f"require({token})", f"expectLexeme({token})", symlinks.text.count(f"require({token})"))
if re.search(r'(?<![A-Za-z0-9_$.])require\(', symlinks.text): sys.exit("a require call remains in symlink-targets")
symlinks.done()

# the build-surfaces law asked the root script for the lazy repo registration; the registry owns it now
(Edit(f"{L}/🧪️tests/🔧️build-surfaces/🟦️.ts")
    .sub('''  expect(source).toContain('.registerLazy("repo"');''', '''  expect(JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🔣️.json")).commands.some((row: { id: string }) => row.id === "repo")).toBe(true);''').done())

# zero-touch law: command closures
zero = Edit(f"{Z}/🟦️.ts")
tail = '''  return zeroTouchClosureFindingsV1(readZeroTouchRosterV1(roster), tree);
}
'''
zero.sub(tail, tail + staged("08-zero-touch-commands.part.ts"))
zero.done()
json_transform(f"{Z}/🗂️closures/🔣️.json", lambda data: data["closures"].extend(json.loads(staged("14-roster-add.json"))))
json_transform(f"{Z}/🧪️tests/🔣️.json", lambda data: data.__setitem__("commandCases", json.loads(staged("13-command-cases.json"))))
(Edit(f"{B}/🧬️schema/🛗️zero-touch-closure.json")
    .sub('"required": ["version", "packages", "cases", "scans"],', '"required": ["version", "packages", "cases", "commandCases", "scans"],')
    .sub('''    "scans": {
      "type": "array",''', '''    "commandCases": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "required": ["name", "commands", "seams", "files", "closureFiles", "findings"],
        "additionalProperties": false,
        "properties": {
          "name": { "type": "string", "minLength": 1 },
          "commands": {
            "type": "array",
            "minItems": 1,
            "items": {
              "type": "object",
              "required": ["id", "module", "requires"],
              "additionalProperties": false,
              "properties": { "id": { "type": "string", "minLength": 1 }, "module": { "type": "string", "minLength": 1 }, "requires": { "enum": ["none", "dependencies"] } }
            }
          },
          "seams": { "type": "array", "items": { "type": "string", "minLength": 1 } },
          "files": { "type": "object", "additionalProperties": { "type": "string" } },
          "closureFiles": { "type": "object", "additionalProperties": { "type": "integer", "minimum": 0 } },
          "findings": { "type": "array", "items": { "type": "string", "pattern": "^[^|]+\\\\|(workspace-specifier|external-package|seam-static|missing-entry)\\\\|[^|]+\\\\|[^|]+$" } },
          "details": { "type": "object", "additionalProperties": { "type": "string" } }
        }
      }
    },
    "scans": {
      "type": "array",''').done())
(Edit(f"{Z}/🧪️tests/🟦️.ts")
    .sub('''import { checkZeroTouchClosuresV1, diskZeroTouchTreeV1, readZeroTouchRosterV1, scanModuleSpecifiersV1, workspaceSpecifierTargetV1, zeroTouchClosureFindingsV1, ZERO_TOUCH_CLOSURES_FILE, type ZeroTouchImportKindV1, type ZeroTouchTreeV1 } from "../🟦️.ts";''', '''import { checkZeroTouchClosuresV1, checkZeroTouchCommandsV1, diskZeroTouchTreeV1, readZeroTouchCommandsV1, readZeroTouchRosterV1, scanModuleSpecifiersV1, workspaceSpecifierTargetV1, zeroTouchClosureFindingsV1, zeroTouchCommandFindingsV1, ZERO_TOUCH_CLOSURES_FILE, ZERO_TOUCH_COMMANDS_FILE, type ZeroTouchCommandV1, type ZeroTouchImportKindV1, type ZeroTouchTreeV1 } from "../🟦️.ts";''')
    .sub('''const files = new Set<string>(), pending = roster.closures.map((closure) => closure.entry);''', '''const files = new Set<string>(), pending = [...roster.closures.map((closure) => closure.entry), ...readZeroTouchCommandsV1(tree.read(ZERO_TOUCH_COMMANDS_FILE)!).map((command) => command.module)];''')
    .sub('''test("the real checkout has no import that leaves a zero-touch closure", () => {
  const result = checkZeroTouchClosuresV1(repoRoot);
  expect(result.findings.map(subject)).toEqual([]);
  expect(Object.keys(result.files).sort()).toEqual(readZeroTouchRosterV1(diskZeroTouchTreeV1(repoRoot).read(ZERO_TOUCH_CLOSURES_FILE)!).closures.map((closure) => closure.id).sort());
}, 120_000);
''', '''test("the real checkout has no import that leaves a zero-touch closure", () => {
  const result = checkZeroTouchClosuresV1(repoRoot);
  expect(result.findings.map(subject)).toEqual([]);
  expect(Object.keys(result.files).sort()).toEqual(readZeroTouchRosterV1(diskZeroTouchTreeV1(repoRoot).read(ZERO_TOUCH_CLOSURES_FILE)!).closures.map((closure) => closure.id).sort());
}, 120_000);

type CommandCaseRow = Readonly<{ name: string; commands: readonly ZeroTouchCommandV1[]; seams: readonly string[]; files: Readonly<Record<string, string>>; closureFiles: Readonly<Record<string, number>>; findings: readonly string[]; details?: Readonly<Record<string, string>> }>;

for (const row of fixture.commandCases as unknown as readonly CommandCaseRow[]) {
  test(`zero-touch command closure: ${row.name}`, () => {
    const result = zeroTouchCommandFindingsV1(row.commands, row.seams, treeOf(row.files));
    expect(result.findings.map(subject).sort()).toEqual([...row.findings].sort());
    expect(result.files).toEqual(row.closureFiles);
    for (const [key, detail] of Object.entries(row.details ?? {})) expect(result.findings.find((finding) => subject(finding) === key)?.detail).toBe(detail);
  });
}

test("a command registry row is closed", () => {
  expect(() => readZeroTouchCommandsV1(JSON.stringify({ commands: [] }))).toThrow("Root command registry required");
  expect(() => readZeroTouchCommandsV1(JSON.stringify({ commands: [{ id: "a", module: "a.ts", requires: "maybe" }] }))).toThrow("Root command registry row required");
  expect(() => readZeroTouchCommandsV1(JSON.stringify({ commands: [{ id: "a", module: "a.ts", requires: "none" }, { id: "a", module: "b.ts", requires: "none" }] }))).toThrow("Root command registry row required");
});

test("no command of the root registry reaches an import its declared requirement forbids", () => {
  const result = checkZeroTouchCommandsV1(repoRoot);
  expect(result.findings.map(subject)).toEqual([]);
  expect(Object.keys(result.files).sort()).toEqual(readZeroTouchCommandsV1(diskZeroTouchTreeV1(repoRoot).read(ZERO_TOUCH_COMMANDS_FILE)!).map((command) => `command:${command.id}`).sort());
}, 120_000);
''')
    .sub('''  const visit = (node: ts.Node): void => {
    if (ts.isImportDeclaration(node) && !typeOnly(node.importClause)) add(node.moduleSpecifier, "static");''', '''  const visit = (node: ts.Node): void => {
    if (ts.isIfStatement(node) && node.expression.getText(source) === "import.meta.vitest") { if (node.elseStatement) visit(node.elseStatement); return; }
    if (ts.isImportDeclaration(node) && !typeOnly(node.importClause)) add(node.moduleSpecifier, "static");''')
    .sub('''    else if (ts.isExportDeclaration(node) && !node.isTypeOnly) add(node.moduleSpecifier, "static");''', '''    else if (ts.isExportDeclaration(node) && !node.isTypeOnly && !(node.exportClause !== undefined && ts.isNamedExports(node.exportClause) && node.exportClause.elements.length > 0 && node.exportClause.elements.every((element) => element.isTypeOnly))) add(node.moduleSpecifier, "static");''')
    .sub('''    const ours = scanModuleSpecifiersV1(file, text).map((entry) => `${entry.kind}\\0${entry.specifier}`).sort(), oracle = compilerImports(file, text);''', '''    const runtime = (row: string): boolean => !/^[a-z]+\\0(?:node:|bun:|bun$)/u.test(row), ours = scanModuleSpecifiersV1(file, text).map((entry) => `${entry.kind}\\0${entry.specifier}`).filter(runtime).sort(), oracle = compilerImports(file, text).filter(runtime);''').done())

plan = {"version": 1, "writes": list(writes.values()), "edits": list(edits.values())}
json.dump(plan, open(args.out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps({"writes": len(writes), "edits": len(edits), "replacements": sum(len(e["replacements"]) for e in edits.values())}))
