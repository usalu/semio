"""Builds staged/plan.json for bzl-default: anchored edits per group (prep, setup, commands, manifest, flip) from the live tree.
Every edit is an exact (old -> new) text replacement with the expected occurrence counts; nothing is written outside staged/."""
import hashlib, json, os, sys

ROOT = os.environ.get("SEMIO_ROOT", "/Users/ueli/Documents/semio")
HERE = os.path.dirname(os.path.abspath(__file__))
R = "🧰️framework/🛍️products/🦑️repo"
L = R + "/🔨️modules/📚️library"
B = L + "/🏗️build"
M = R + "/🔨️modules"
BZ = B + "/🔧️tools/🏗️bazel"
T0 = L + "/⚡️caching/🚀️bootstrap/🛠️tools"

sha = lambda text: hashlib.sha256(text.encode("utf-8")).hexdigest()
files = {}
problems = []


def text_of(path):
    full = os.path.join(ROOT, path)
    if not os.path.isfile(full):
        problems.append("missing file: " + path)
        return None
    return open(full, encoding="utf-8", newline="").read()


def edit(group, path, old, new, counts=None, optional=False, note="", bulk=False, superseded_by=None):
    text = text_of(path)
    if text is None:
        return
    entry = files.setdefault(path, {"path": path, "sha256": sha(text), "edits": []})
    found = text.count(old)
    if found > 1 and not bulk:
        problems.append(f"anchor not unique ({found}) in {path}: {old[:70]!r}")
        return
    if not optional and found == 0 and (new == "" or text.count(new) == 0):
        problems.append(f"anchor not found in {path}: {old[:70]!r}")
        return
    if counts is None:
        counts = [found] if found else [1]
    if old in new and new and text.count(new) > 0:
        counts = [text.count(new)]
        if "already-applied" not in note:
            note = (note + " already-applied").strip()
    entry["edits"].append({"id": f"{group}-{len(entry['edits']) + 1}", "group": group, "old": old, "new": new, "counts": counts, "optional": optional, "note": note, **({"supersededBy": superseded_by} if superseded_by else {})})


def unique_tail(text):
    lines = text.rstrip("\n").split("\n")
    for k in range(1, 12):
        tail = "\n".join(lines[-k:])
        if text.count(tail) == 1:
            return tail
    raise SystemExit("no unique tail")


def insert_after_line_end(group, path, anchor, addition, note=""):
    edit(group, path, anchor, anchor + addition, note=note)


# ---------------------------------------------------------------- prep: default-independent tests, behaviour neutral under both defaults
birth = B + "/📥️caller/🧪️tests/📥️birth/🔣️.json"
for name in ["alias-release", "opaque-tail", "reset", "watch"]:
    edit("prep", birth, f'"name": "{name}",\n      "args": [\n', f'"name": "{name}",\n      "args": [\n        "--tool=nx",\n', note="pin the Nx rows to the tool whose launches they assert")
extra = {"nx-tools-setup": (["--tool=nx", "tools", "setup"], '["nx", "--version"]'), "bazel-tools-setup": (["--tool=bazel", "tools", "setup"], "null"), "bazel-tools-setup-bazel": (["tools", "setup", "bazel"], "null"), "bazel-reset": (["--tool=bazel", "reset"], "null")}
rows = ""
for name, (args, main) in extra.items():
    arguments = ",\n".join(f'        "{a}"' for a in args)
    mainText = main if main == "null" else '[\n        "nx",\n        "--version"\n      ]'
    rows += f',\n    {{\n      "name": "{name}",\n      "args": [\n{arguments}\n      ],\n      "mode": "prototype",\n      "main": {mainText},\n      "javascript": false\n    }}'
dependencies_row = '      "name": "dependencies",\n      "args": [\n        "dependencies",\n        "sync",\n        "--all"\n      ],\n      "mode": "release",\n      "main": null,\n      "javascript": false\n    }\n  ],'
edit("prep", birth, dependencies_row, dependencies_row.replace("    }\n  ],", "    }" + rows + "\n  ],"), note="tools setup never declares an Nx launch for bazel, whatever the default")

edit("prep", B + "/🚀️bootstrap/⌨️cli/🧪️tests/🟦️.ts", 'createBuildBootstrapCapabilitiesV1(process.cwd(),["repo",...args],{SEMIO_BUILD_MODE:"prototype"})', 'createBuildBootstrapCapabilitiesV1(process.cwd(),["repo",...args],{SEMIO_BUILD_MODE:"prototype",SEMIO_BUILD_TOOL:"nx"})')
portable = B + "/🧪️tests/🕸️graph/🧭️portable-cli/🟦️.ts"
edit("prep", portable, 'createBuildBootstrapCapabilitiesV1(root,["repo",...args],{SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(custom)})', 'createBuildBootstrapCapabilitiesV1(root,["repo",...args],{SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(custom),SEMIO_BUILD_TOOL:"nx"})')
edit("prep", portable, 'createBuildBootstrapCapabilitiesV1(root,request,{SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(authored)})', 'createBuildBootstrapCapabilitiesV1(root,request,{SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(authored),SEMIO_BUILD_TOOL:"nx"})')
edit("prep", portable, 'import {parseBuildInvocationV1,nxBuildArgumentsV1,resolveBuildInvocationV1} from "../../../🧭️invocation/🟦️.ts";', 'import {parseBuildInvocationV1,nxBuildArgumentsV1,resolveBuildInvocationV1} from "../../../🧭️invocation/🟦️.ts";\nimport buildPolicy from "../../../🔣️policy.json";')
edit("prep", portable, 'expect(parseBuildInvocationV1(["show","projects"],{SEMIO_BUILD_TOOL:"bazel"}).tool).toBe("bazel");', 'expect(parseBuildInvocationV1(["show","projects"],{SEMIO_BUILD_TOOL:"bazel"}).tool).toBe("bazel");\n  expect(parseBuildInvocationV1(["run","app:build"],{}).tool).toBe(buildPolicy.defaultTool);expect(parseBuildInvocationV1(["run","app:build"],{SEMIO_BUILD_TOOL:"nx"}).tool).toBe("nx");expect(parseBuildInvocationV1(["--tool=nx","run","app:build"],{SEMIO_BUILD_TOOL:"bazel"}).tool).toBe("nx");expect(parseBuildInvocationV1(["--tool=bazel","run","app:build"],{SEMIO_BUILD_TOOL:"nx"}).tool).toBe("bazel");')

orchestration = L + "/⚡️caching/🔁️verification/📋️orchestration/🟦️.ts"
edit("prep", orchestration, 'import buildPolicy from "../../../🏗️build/🔣️policy.json" with { type: "json" };\nimport probeSchema', 'import probeSchema')
edit("prep", orchestration, 'const selected = process.env.SEMIO_BUILD_TOOL ?? buildPolicy.defaultTool;', 'const selected = process.env.SEMIO_CACHE_PROBE_TOOL ?? "nx";', note="the probe names its tool; the selected build tool no longer decides which cache is measured", superseded_by="probe")

# ---------------------------------------------------------------- setup: tools setup bazel opens only tools and dependencies
storage = BZ + "/💾️storage/🟦️.ts"
anchor = "actualPhase))));}\n\nconst cachePolicy="
edit("setup", storage, anchor, '''actualPhase))));}
/** 🧳️ The two stores `tools setup bazel` writes: the executable and the module catalog never touch the products partition. */
export type BazelSetupStorageV1=Pick<BazelStorageV1,"tools"|"dependencies"|"paths">;
/** 🛂️ Refuses fabricated setup contexts; the products root is a declared path here and is never opened. */
export async function requireBazelSetupStorageV1(root:string,storage:BazelSetupStorageV1):Promise<void>{for(const id of ["tools","dependencies"] as const){const store=storage[id];requireNativeStorageReservationV1(store.storage,store.control);await store.recheck();if(store.root!==repositoryCachePartitionRootV1(root,id))throw Error("Original declared Bazel partition required");}const roots={tools:storage.tools.root,dependencies:storage.dependencies.root,products:repositoryCachePartitionRootV1(root,"products")},layout=bazelCacheLayoutV1(roots);if(JSON.stringify(layout)!==JSON.stringify(storage.paths))throw Error("Original Bazel partition layout required");for(const key of ["executable","install","modules"] as const)requireNativeStoragePathV1(key==="modules"?roots.dependencies:roots.tools,layout[key]);}
/** 🏬️ Keeps only the tools and dependencies stores alive while Bazel and its fixed module catalog are provisioned. */
export function withBazelSetupStorageV1<T>(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(storage:BazelSetupStorageV1,actualPhase:ResourceLeaseControl)=>Promise<T>):Promise<T>{return withRepositoryCacheStoreV1(root,"tools",invocation,phaseControl,(tools,toolPhase)=>withRepositoryCacheStoreV1(root,"dependencies",invocation,toolPhase,(dependencies,actualPhase)=>body(Object.freeze({tools,dependencies,paths:bazelCacheLayoutV1({tools:tools.root,dependencies:dependencies.root,products:repositoryCachePartitionRootV1(root,"products")})}),actualPhase)));}

const cachePolicy=''', note="setup storage: tools + dependencies only")
adapter = BZ + "/📜️script.ts"
edit("setup", adapter, 'import {withBazelStorageV1,requireBazelStorageV1,bazelCacheFlagsV1,type BazelStorageV1} from "./💾️storage/🟦️.ts";', 'import {withBazelStorageV1,requireBazelStorageV1,requireBazelSetupStorageV1,bazelCacheFlagsV1,type BazelStorageV1,type BazelSetupStorageV1} from "./💾️storage/🟦️.ts";')
edit("setup", adapter, 'storage:BazelStorageV1,phaseControl:ResourceLeaseControl):Promise<{lock:string;repository:string}>', 'storage:BazelSetupStorageV1,phaseControl:ResourceLeaseControl):Promise<{lock:string;repository:string}>')
edit("setup", adapter, 'await requireBazelStorageV1(root,storage);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);const environment=issueBuildTaskEnvironmentV1(root,invocation,{kind:"noop"},[],storage.dependencies.environment)', 'await requireBazelSetupStorageV1(root,storage);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);const environment=issueBuildTaskEnvironmentV1(root,invocation,{kind:"noop"},[],storage.dependencies.environment)')
script = B + "/📜️script.ts"
edit("setup", script, '{withBazelStorageV1}=await import("./🔧️tools/🏗️bazel/💾️storage/🟦️.ts")', '{withBazelSetupStorageV1}=await import("./🔧️tools/🏗️bazel/💾️storage/🟦️.ts")')
edit("setup", script, 'await withBazelStorageV1(root,invocation,phaseControl,async(storage,actualPhase)=>{', 'await withBazelSetupStorageV1(root,invocation,phaseControl,async(storage,actualPhase)=>{')

tests = BZ + "/💾️storage/🧪️tests/🟦️.ts"
edit("setup", tests, 'import {spawnSync} from "node:child_process";', 'import {spawnSync} from "node:child_process";\nimport ts from "typescript";')
test_text = text_of(tests)
tail = unique_tail(test_text)
new_test = '''

const setupContract={type:"object",required:["opens","full","installSkipped"],additionalProperties:false,properties:{opens:{type:"array",uniqueItems:true,minItems:2,items:{enum:["tools","dependencies","products"]}},full:{type:"array",uniqueItems:true,minItems:3,items:{enum:["tools","dependencies","products"]}},installSkipped:{type:"array",uniqueItems:true,minItems:1,items:{type:"string"}}}};
test("tools setup bazel opens only the tools and dependencies partitions and never installs JavaScript or Nx, read by the TypeScript compiler and a text oracle",()=>{
 expect(new Ajv({strict:true}).validate(setupContract,fixture.setup)).toBe(true);
 const parse=(path:string)=>{const file=join(import.meta.dir,path);return ts.createSourceFile(file,readFileSync(file,"utf8"),ts.ScriptTarget.Latest,true);},declaration=(file:ts.SourceFile,name:string)=>file.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text===name)!;
 const opened=(node:ts.Node)=>{const names:string[]=[],visit=(child:ts.Node)=>{if(ts.isCallExpression(child)&&ts.isIdentifier(child.expression)&&child.expression.text==="withRepositoryCacheStoreV1"&&child.arguments[1]&&ts.isStringLiteral(child.arguments[1]))names.push(child.arguments[1].text);ts.forEachChild(child,visit);};visit(node);return names;};
 const storageFile=parse("../🟦️.ts"),setup=declaration(storageFile,"withBazelSetupStorageV1"),full=declaration(storageFile,"withBazelStorageV1");
 expect(opened(setup)).toEqual(fixture.setup.opens);expect(opened(full)).toEqual(fixture.setup.full);
 for(const node of [setup,full])expect(opened(node)).toEqual([...node.getText().matchAll(/withRepositoryCacheStoreV1\\(root,"(\\w+)"/g)].map(match=>match[1]));
 const entry=parse("../../../../📜️script.ts"),leases=declaration(entry,"runOwnedBuildInLeasesV1"),branches:ts.IfStatement[]=[],visit=(node:ts.Node)=>{if(ts.isIfStatement(node))branches.push(node);ts.forEachChild(node,visit);};visit(leases);
 const tools=branches.find(node=>node.expression.getText()==='command==="tools"')!,guard=branches.find(node=>node.thenStatement.getText().includes("ensureBuildEnvironmentV1"))!;
 expect(tools.getText()).toContain("withBazelSetupStorageV1");expect(tools.getText()).not.toContain("withBazelStorageV1");expect(tools.getText()).not.toContain("ensureBuildEnvironmentV1");
 const skipped:string[]=[],collect=(node:ts.Node)=>{if(ts.isArrayLiteralExpression(node))skipped.push(...node.elements.filter(ts.isStringLiteral).map(element=>element.text));ts.forEachChild(node,collect);};collect(guard.expression);
 expect(skipped.sort()).toEqual([...fixture.setup.installSkipped].sort());expect(guard.expression.getText()).toMatch(/^!\\[.*\\]\\.includes\\(command\\)$/u);
});'''
if "tools setup bazel opens only" not in test_text:
    edit("setup", tests, tail, tail + new_test, note="language-agnostic fixture rows in the storage fixture plus a compiler-API law")
fx = BZ + "/💾️storage/🧪️tests/🧫️fixtures/🔣️.json"
fx_text = text_of(fx)
fx_last = fx_text.rstrip("\n").split("\n")[-2]
if '"setup": {"opens"' not in fx_text:
    edit("setup", fx, fx_last + "\n}", fx_last + ',\n  "setup": {"opens": ["tools", "dependencies"], "full": ["tools", "dependencies", "products"], "installSkipped": ["tools", "reset", "cache"]}\n}', note="setup contract rows")

readme = R + "/README.md"
edit("setup", readme, "See Setup per platform below. |", "See Setup per platform below. `tools setup bazel` needs neither the JavaScript install nor Nx and opens only the tools and dependencies partitions; the products partition is opened by the first run. |")
edit("setup", readme, "Siehe Einrichtung je Plattform unten. |", "Siehe Einrichtung je Plattform unten. `tools setup bazel` braucht weder die JavaScript-Installation noch Nx und öffnet nur die Partitionen für Werkzeuge und Abhängigkeiten; die Produkte-Partition öffnet erst der erste Lauf. |")

# ---------------------------------------------------------------- commands: tool-independent by design, stated per command
edit("commands", readme, "| `graph`, `show projects\\|project <name>\\|targets` | neutral | neutral | no | |", "| `graph`, `show projects\\|project <name>\\|targets` | neutral | neutral | no | Reads the portable graph; never starts Nx or Bazel, by design. |")
edit("commands", readme, "| `dependencies sync\\|lock --all\\|--scope <owner>` | neutral | neutral | no | |", "| `dependencies sync\\|lock --all\\|--scope <owner>` | neutral | neutral | no | Bun installs the JavaScript dependencies for both tools, by design. |")
edit("commands", readme, "| `cargo …` | neutral | neutral | no | |", "| `cargo …` | neutral | neutral | no | The repository's compile ingress; Bazel's Cargo actions call it, so it is the leaf of both tools, by design. Run Rust targets with `run <project>:build\\|check\\|test` to use Bazel's cache. |")
edit("commands", readme, "| `graph`, `show projects\\|project <name>\\|targets` | neutral | neutral | nein | |", "| `graph`, `show projects\\|project <name>\\|targets` | neutral | neutral | nein | Liest den portablen Graphen; startet bewusst weder Nx noch Bazel. |")
edit("commands", readme, "| `dependencies sync\\|lock --all\\|--scope <owner>` | neutral | neutral | nein | |", "| `dependencies sync\\|lock --all\\|--scope <owner>` | neutral | neutral | nein | Bun installiert die JavaScript-Abhängigkeiten für beide Werkzeuge, bewusst. |")
edit("commands", readme, "| `cargo …` | neutral | neutral | nein | |", "| `cargo …` | neutral | neutral | nein | Das Eingangstor des Repositorys zum Kompilieren; Bazels Cargo-Actions rufen es auf, es ist also das Blatt beider Werkzeuge, bewusst. Rust-Targets mit `run <project>:build\\|check\\|test` ausführen, um Bazels Cache zu nutzen. |")

# ---------------------------------------------------------------- manifest: needs the dependency lock (bun repo dependencies lock --all afterwards)
edit("manifest", T0 + "/package.json", '    "typescript": "5.9.3",\n    "@semio-tech/framework-filesystem": "workspace:*"\n  },', '    "typescript": "5.9.3"\n  },', note="the dependencies row broke the closed pinned recipe; the devDependencies row stays for the tests that import it")
edit("manifest", "bun.lock", '        "@nxlv/python": "23.0.2",\n        "@semio-tech/framework-filesystem": "workspace:*",\n        "nx": "23.2.0",', '        "@nxlv/python": "23.0.2",\n        "nx": "23.2.0",')

# ---------------------------------------------------------------- flip: Bazel becomes the default
edit("flip", B + "/🔣️policy.json", '"defaultTool": "nx",', '"defaultTool": "bazel",')
catalog = B + "/⚙️operations/🔣️catalog.json"
edit("flip", catalog, "Nx with Bun (default) and Bazel.", "Bazel (default) and Nx with Bun.")
edit("flip", catalog, "Nx mit Bun (Standard) und Bazel.", "Bazel (Standard) und Nx mit Bun.")
edit("flip", catalog, "then the policy default (nx).", "then the policy default (bazel).")
edit("flip", catalog, "dann der Standard der Policy (nx).", "dann der Standard der Policy (bazel).")
help_fixture = B + "/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json"
edit("flip", help_fixture, "Nx with Bun (default) and Bazel.", "Bazel (default) and Nx with Bun.", bulk=True)
edit("flip", help_fixture, "Nx mit Bun (Standard) und Bazel.", "Bazel (Standard) und Nx mit Bun.", bulk=True)
edit("flip", help_fixture, "then the policy default (nx).", "then the policy default (bazel).", bulk=True)
edit("flip", help_fixture, "dann der Standard der Policy (nx).", "dann der Standard der Policy (bazel).", bulk=True)
edit("flip", birth, '    {\n      "name": "bazel",\n      "args": [\n        "--tool=bazel",', '    {\n      "name": "default-bazel-run",\n      "args": [\n        "run",\n        "app:build"\n      ],\n      "mode": "prototype",\n      "main": null,\n      "javascript": false\n    },\n    {\n      "name": "default-bazel-tools-setup",\n      "args": [\n        "tools",\n        "setup"\n      ],\n      "mode": "prototype",\n      "main": null,\n      "javascript": false\n    },\n    {\n      "name": "default-bazel-reset",\n      "args": [\n        "reset"\n      ],\n      "mode": "prototype",\n      "main": null,\n      "javascript": false\n    },\n    {\n      "name": "bazel",\n      "args": [\n        "--tool=bazel",', note="the new default declares no Nx launch for run, tools setup and reset")
edit("flip", M + "/💻️client/🔌️mcp/🗄️repository/🐹️.go", "provision tools with repo tools setup all. Nx remains the default; select Bazel with --tool=bazel.", "provision the default tool with repo tools setup and both tools with repo tools setup all. Bazel is the default; select Nx with --tool=nx.")
edit("flip", M + "/🔌️mcp/📦️packages/🦀️rust/🗄️repository/🦀️.rs", "provision tools with repo tools setup all. Nx remains the default; select Bazel with --tool=bazel.", "provision the default tool with repo tools setup and both tools with repo tools setup all. Bazel is the default; select Nx with --tool=nx.")
edit("flip", M + "/🔌️mcp/🧪️tests/🧫️fixtures/🚪️entrypoint-contract.json", '"--tool=bazel",\n    "Nx remains the default"', '"--tool=nx",\n    "Bazel is the default"')
edit("flip", M + "/🎛️dashboard/🧪️tests/🌀️control-plane/🥒️.feature", "Nx remains the task execution authority.", "The selected build tool remains the task execution authority.")
edit("flip", M + "/🎛️dashboard/🧪️tests/🧊️execution/🥒️.feature", "And Nx remains responsible for build and installation tasks", "And the selected build tool remains responsible for build and installation tasks")
tsreadme = L + "/📦️packages/🟦️typescript/README.md"
edit("flip", tsreadme, "bun repo --tool=bazel run <project>:<task>[:<configuration>]", "bun repo --tool=nx run <project>:<task>[:<configuration>]")
edit("flip", tsreadme, "Nx remains the configured default while Bazel qualification is in progress.", "Bazel is the configured default; Nx stays selectable with `--tool=nx`.")
edit("flip", "🎓️teaching/🛂️proctor/🏗️bootstrap/🧾️product/🟦️.ts", 'bun repo${env.SEMIO_BUILD_TOOL === "bazel" ? " --tool=bazel" : ""} run', 'bun repo${env.SEMIO_BUILD_TOOL === "nx" ? " --tool=nx" : ""} run')

edit("flip", readme, "**Nx with Bun** (default) and **Bazel** (`--tool=bazel`)", "**Bazel** (default) and **Nx with Bun** (`--tool=nx`)")
edit("flip", readme, "then `defaultTool` of the policy (`nx`).", "then `defaultTool` of the policy (`bazel`).")
edit("flip", readme, "(`--tool`, default `nx`).", "(`--tool`, default `bazel`).")
edit("flip", readme, "Nx is the default (`defaultTool`). The repository does not use Nx project discovery;", "Nx is selected with `--tool=nx` (or `SEMIO_BUILD_TOOL=nx`); Bazel is the default (`defaultTool`). The repository does not use Nx project discovery;")
edit("flip", readme, "so each one works with `--tool=nx` (default) and `--tool=bazel`.", "so each one works with `--tool=bazel` (default) and `--tool=nx`.")
edit("flip", readme, "**Nx mit Bun** (Standard) und **Bazel** (`--tool=bazel`)", "**Bazel** (Standard) und **Nx mit Bun** (`--tool=nx`)")
edit("flip", readme, "dann `defaultTool` der Richtlinie (`nx`).", "dann `defaultTool` der Richtlinie (`bazel`).")
edit("flip", readme, "(`--tool`, Standard `nx`).", "(`--tool`, Standard `bazel`).")
edit("flip", readme, "Nx ist der Standard. Das Repository nutzt keine Nx-Projektentdeckung,", "Nx wird mit `--tool=nx` (oder `SEMIO_BUILD_TOOL=nx`) gewählt; Bazel ist der Standard (`defaultTool`). Das Repository nutzt keine Nx-Projektentdeckung,")
edit("flip", readme, "also mit `--tool=nx` (Standard) und `--tool=bazel`.", "also mit `--tool=bazel` (Standard) und `--tool=nx`.")
edit("flip", readme, "then the policy default `nx`.", "then the policy default `bazel`.", counts=[0, 1], optional=True, note="docs-build guide, EN, only once that set is applied")
edit("flip", readme, "| | Nx (default) | Bazel (`--tool=bazel`) |", "| | Nx (`--tool=nx`) | Bazel (default) |", counts=[0, 1], optional=True, note="docs-build guide table header")
edit("flip", readme, "Use Nx for the default flow. Choose Bazel when you want results shared across selections and machines, workers, or remote execution.", "Use Bazel for the default flow: results are shared across selections and machines, and workers and remote execution are available. Select Nx with `--tool=nx` when you want its task runner and cache.", counts=[0, 1], optional=True, note="docs-build guide advice, EN")
edit("flip", readme, "dann der Standard der Richtlinie `nx`.", "dann der Standard der Richtlinie `bazel`.", counts=[0, 1], optional=True, note="docs-build guide, DE")
edit("flip", readme, "| | Nx (Standard) | Bazel (`--tool=bazel`) |", "| | Nx (`--tool=nx`) | Bazel (Standard) |", counts=[0, 1], optional=True, note="docs-build guide table header, DE")
edit("flip", readme, "Nimm Nx für den Standardablauf. Wähle Bazel, wenn du Ergebnisse über Auswahlen und Rechner hinweg teilen, Worker oder Remote-Ausführung nutzen willst.", "Nimm Bazel für den Standardablauf: Ergebnisse werden über Auswahlen und Rechner hinweg geteilt, Worker und Remote-Ausführung stehen bereit. Wähle Nx mit `--tool=nx`, wenn du seinen Task-Runner und Cache willst.", counts=[0, 1], optional=True, note="docs-build guide advice, DE")

root_readme = "README.md"
edit("flip", root_readme, "Nx is the default backend and Bazel is selectable with `--tool=bazel` (or `SEMIO_BUILD_TOOL=bazel`, inherited by nested tasks)", "Bazel is the default backend and Nx is selectable with `--tool=nx` (or `SEMIO_BUILD_TOOL=nx`, inherited by nested tasks)")
edit("flip", root_readme, "Nx (default) and Bazel execute the same graph.", "Bazel (default) and Nx execute the same graph.")
edit("flip", root_readme, "Nx remains the default build tool (`defaultTool` in", "Bazel is the default build tool (`defaultTool` in")
edit("flip", root_readme, "Bazel qualification does not change the default automatically.", "Nx stays selectable with `--tool=nx` (or `SEMIO_BUILD_TOOL=nx`).")

exec(compile(open(os.path.join(HERE, "plan-round2.py"), encoding="utf-8").read(), "plan-round2.py", "exec"))

if problems:
    print("\n".join(problems))
    sys.exit(1)
plan = {"version": 1, "files": sorted(files.values(), key=lambda row: row["path"]), "newFiles": newfiles}
os.makedirs(os.path.join(HERE, "staged"), exist_ok=True)
json.dump(plan, open(os.path.join(HERE, "staged", "plan.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
groups = {}
for row in plan["files"]:
    for e in row["edits"]:
        groups.setdefault(e["group"], set()).add(row["path"])
print({g: len(p) for g, p in groups.items()}, "files:", len(plan["files"]), "edits:", sum(len(r["edits"]) for r in plan["files"]))
