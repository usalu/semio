# Round-2 manual sites. Executed by manual-gen.py (shares `sub`, `need`, `cut`, `entry`, `text`, `files`, `L`, `P`, `ROOT`).
MOVES = []
DELETES = []
PW = L + "/⚡️caching/🚀️bootstrap"

def extract(path, start, end, to, symbols, reexport=False, prefix=""):
    entry(path).setdefault("extract", []).append({"start": start, "end": end, "to": to, "symbols": symbols, "reexport": reexport, "prefix": prefix})

def drop_import(path, names):
    src = text(path); pattern = re.compile(r'^import\s*(?:type\s*)?\{\s*' + names + r'\s*\}\s*from\s*"[^"]*";[ \t]*\n', re.M)
    found = pattern.findall(src)
    if len(found) != 1: raise SystemExit(f"IMPORT {names} x{len(found)} {path}")
    entry(path)["edits"].append({"old": found[0], "new": "", "count": 1})

# --- vite runtime selector -------------------------------------------------
vite = L + "/⚡️caching/🌐️vite/🟦️.ts"
sub(vite, 'readonly runtime?:"bun"|"node"}', 'readonly runtime?:RuntimeIdV1}')
sub(vite, 'runtime==="node"?"node":runtime==="bun"&&!process.versions.bun?"bun":process.execPath', 'runtime===undefined?process.execPath:runtimeExecutableForV1(runtime)')
need(vite, "RuntimeIdV1", "runtimeExecutableForV1")
vs = L + "/⚡️caching/🌐️vite/🌐️service/🟦️.ts"
sub(vs, '    runtime?: "bun" | "node";', '    runtime?: RuntimeIdV1;'); need(vs, "RuntimeIdV1")

# --- node child programs ----------------------------------------------------
for hr in (L + "/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🟦️.ts", L + "/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts"):
    sub(hr, r'(captureOwnedCommand(?:Result)?V1)\(process\.versions\.bun\?"node":process\.execPath,', r'\1(runtimeExecutableForV1("node"),', regex=True); need(hr, "runtimeExecutableForV1")

# --- artifact registry label -------------------------------------------------
sub(L + "/⚡️caching/📦️artifacts/📇️registry/🟦️.ts", '["bun", "node_modules", "dependency-store"]', '["node-modules", "node_modules", "dependency-store"]')

# --- bundler selector --------------------------------------------------------
bun_bundle = L + "/⚡️caching/📦️bundle/🟦️.ts"
sub(bun_bundle, 'readonly runtime?: "javascript" | "bun"; readonly invocation', 'readonly runtime?: "javascript" | "runtime"; readonly invocation')
sub(bun_bundle, 'if(options.runtime==="bun"){', 'if(options.runtime==="runtime"){')
sub(bun_bundle, '...["bun",...(options.external??[])].map(', '...[...runtimeBuiltinNamesV1(),...(options.external??[])].map('); need(bun_bundle, "runtimeBuiltinNamesV1")
sub(P + "/🧩️artifact-package/🟦️.ts", 'readonly bundleRuntime?: "javascript" | "bun";', 'readonly bundleRuntime?: "javascript" | "runtime";')
phys = L + "/⚡️caching/📦️artifacts/💾️storage/🧪️tests/📥️physical/🟦️.ts"
sub(phys, '["bun","javascript"]as const', '["runtime","javascript"]as const', count=2)
sub(phys, '(runtime==="bun"?"bun":"esbuild")', '(runtime==="runtime"?"runtime":"esbuild")', count=2)

# --- bootstrap watcher command ----------------------------------------------
sub(PW + "/📜️script.ts", '"--","bun","nx","run",watchTarget', '"--",runtimeCommandNameV1(),"nx","run",watchTarget'); need(PW + "/📜️script.ts", "runtimeCommandNameV1")
sub(PW + "/📥️caller/🟦️.ts", '"--","bun","nx","run",invocation.watch', '"--",runtimeCommandNameV1(),"nx","run",invocation.watch'); need(PW + "/📥️caller/🟦️.ts", "runtimeCommandNameV1")

# --- authorizer compile target ------------------------------------------------
az = PW + "/🛠️tools/🔏️authorizers"
snap = az + "/📦️snapshot/🟦️.ts"
sub(snap, 'target:"node"|"bun",entryRole', 'target:"node"|"runtime",entryRole')
sub(snap, 'if(target!=="node"&&target!=="bun")throw', 'if(target!=="node"&&target!=="runtime")throw')
sub(PW + "/🛠️tools/🟦️.ts", 'control,"bun","program")', 'control,"runtime","program")')
codef = L + "/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🔏️code/🟦️.ts"
sub(codef, 'control,"bun");bytes=result.bytes', 'control,"runtime");bytes=result.bytes')
aztest = az + "/🧪️tests/🟦️.ts"
sub(aztest, 'for(const target of["bun","node"]as const)', 'for(const target of["runtime","node"]as const)', count=2)
sub(aztest, 'control,"bun",role as never', 'control,"runtime",role as never')
sub(aztest, 'control,"bun",row.role as', 'control,"runtime",row.role as')
rec = L + "/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🔏️code/🧬️schema/📜️receipt.json"
sub(rec, '        "name": {\n          "const": "Bun"\n        },', '        "name": {\n          "type": "string",\n          "minLength": 1,\n          "maxLength": 32\n        },')
sub(rec, '        "target": {\n          "const": "bun"\n        },', '        "target": {\n          "enum": [\n            "bun",\n            "node"\n          ]\n        },')

# --- command words -----------------------------------------------------------
sub("🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts", r'runOwnedCommand\("bun", \["x", ("eslint".*?\.\.\.segments)\], this\.root', r'runOwnedCommand(...runtimePackageCommandV1("exec", [\1]), this.root', regex=True)
need("🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts", "runtimePackageCommandV1")
insp = L + "/⚡️caching/🩺️environment/📋️inspection/🟦️.ts"
sub(insp, 'for (const command of ["bun", "node", "cargo", "rustc", "go", "uv", "dotnet", "cmake"]) {', 'for (const command of [...runtimeCommandNamesV1(), "cargo", "rustc", "go", "uv", "dotnet", "cmake"]) {'); need(insp, "runtimeCommandNamesV1")
setup = L + "/🎛️commands/🔧️setup/🟦️.ts"
sub(setup, 'runCmd("bun", ["add", "--no-save", `${platformPkg}@${version}`], { cwd: this.root });', 'runCmd(...runtimePackageCommandV1("add", [`${platformPkg}@${version}`]), { cwd: this.root });'); need(setup, "runtimePackageCommandV1")
proc = L + "/🏃️process/🟦️.ts"
sub(proc, 'if (cmd === "bun" || cmd === process.execPath) {', 'if (isHostRuntimeCommandV1(cmd)) {')
sub(proc, 'spawnSync(process.versions.bun ? process.execPath : "bun", [CARGO_RELAY_SCRIPT', 'spawnSync(runtimeExecutableV1(), [CARGO_RELAY_SCRIPT'); need(proc, "isHostRuntimeCommandV1", "runtimeExecutableV1")
cx = L + "/🏃️process/🦀️cargo/🏃️execution/🟦️.ts"
sub(cx, 'program:operation.continuation.program==="bun"?process.execPath:operation.continuation.program,', 'program:runtimeProgramV1(operation.continuation.program),'); need(cx, "runtimeProgramV1")
rc2 = P + "/🧭️routing/🎛️command/⚙️configuration/🟦️.ts"
sub(rc2, 'command:selection.command==="bun"?process.execPath:selection.command,', 'command:runtimeProgramV1(selection.command),'); need(rc2, "runtimeProgramV1")
ab = L + "/⚖️laws/🧩️plugin-consistency/🚧️authority-bans/🟦️.ts"
sub(ab, 'return server.command === "bun" && args[0]', 'return isRuntimeExecutableV1(server.command) && args[0]'); need(ab, "isRuntimeExecutableV1")

# --- profile spawn census through the owned spawn family ----------------------------
prof = L + "/🏗️build/⏱️profile/🟨️.mjs"
spec = os.path.relpath(os.path.join(ROOT, "🧰️framework/🔨️modules/⚙️runtime/🚀️process/🟦️.ts"), os.path.dirname(os.path.join(ROOT, prof)))
sub(prof, '  if(typeof Bun!=="undefined"){wrapSync(Bun,"spawnSync",args=>args[0]?.cmd??args[0]);wrapAsync(Bun,"spawn",args=>args[0]?.cmd??args[0]);}', '  require("' + spec + '").runtimeObserveSpawnV1((command,milliseconds)=>{try{profile.event(repoProfileSpawnLabelV1(command),milliseconds);}catch{}});')
sub(prof, "started through `Bun.spawn`, `Bun.spawnSync` and the callable members of", "started through the owned runtime spawn family and the callable members of")

# --- jcoprobe host ------------------------------------------------------------------
jh = "🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe/🌐️browser-host/🟦️.ts"
sub(jh, 'const server = Bun.serve({', 'const server = await runtimeServeV1({')
sub(jh, 'const file = Bun.file(full);', 'const file = runtimeFileV1(full).body();'); need(jh, "runtimeServeV1", "runtimeFileV1")

# --- sealed browser codegen policy ---------------------------------------------------
bb2 = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts"
sub(bb2, 'runtime:{kind:"bun",version:"1.4.2",', 'runtime:{kind:runtimeIdentityV1().id,version:"1.4.2",')
sub(bb2, 'assert.equal(globalThis.Bun?.version, "1.4.2", "browser actor artifact: unqualified codegen runtime");', 'assert.equal(process.versions[${JSON.stringify(runtimeDetectKeyV1())}], "1.4.2", "browser actor artifact: unqualified codegen runtime");'); need(bb2, "runtimeIdentityV1", "runtimeDetectKeyV1")

# --- pinned runtime provisioner reads its runtime name from its manifest ------------------
tb = PW + "/🛠️tools/🟦️bun/🟦️.ts"
sub(tb, 'if (process.versions.bun === version) return process.execPath;', 'if (runtimeIdentityV1().id === manifest.runtime && runtimeIdentityV1().version === version) return runtimeExecutableV1();')
sub(tb, 'join(store.root,"bun")', 'join(store.root,manifest.runtime)')
sub(tb, 'executable = join(directory, process.platform === "win32" ? "bun.exe" : "bun");', 'executable = join(directory, process.platform === "win32" ? manifest.runtime + ".exe" : manifest.runtime);')
sub(tb, 'staged = join(staging, process.platform === "win32" ? "bun.exe" : "bun");', 'staged = join(staging, process.platform === "win32" ? manifest.runtime + ".exe" : manifest.runtime);')
sub(tb, '/bun${process.platform === "win32" ? ".exe" : ""}', '/${manifest.runtime}${process.platform === "win32" ? ".exe" : ""}'); need(tb, "runtimeIdentityV1", "runtimeExecutableV1")
tbs = PW + "/🛠️tools/🟦️bun/🧬️schema/🔣️.json"
sub(tbs, '  "required": [\n    "version",\n    "release",\n    "distributions"\n  ],', '  "required": [\n    "runtime",\n    "version",\n    "release",\n    "distributions"\n  ],')
sub(tbs, '  "properties": {\n    "version": {', '  "properties": {\n    "runtime": {\n      "type": "string",\n      "pattern": "^[a-z]+$"\n    },\n    "version": {')
sub(tbs, '      "const": {\n        "version": "1.4.2",', '      "const": {\n        "runtime": "bun",\n        "version": "1.4.2",')

# --- rustup TOML needs no provisioned parser ------------------------------------------------
rt = PW + "/🛠️tools/🦀️rustup/📥️installation/🔤️toml/🟦️.ts"
for names in ("requireToolingPublicationValueV1", r"ManagedToolingPublisherV1", "randomBytes", "join", "writeFile,rm,mkdir", "prepareBun", "captureOwnedCommandV1", "readPhysicalCargoSourceV1", "requireNativeStoragePathV1"): drop_import(rt, names)
cut(rt, "/** 🔤️ Uses the pinned system TOML runtime", "finally{await rm(input);}}", """/** 🔤️ Parses TOML through the owned runtime TOML port behind a first-party bounded data port. */
export function rustupTomlValueV1(source:string,workspace:string,store:RepositoryToolingStoreV1,control:ResourceLeaseControl):Promise<unknown>{return withRepositoryToolingOperationV1(workspace,store,control,actual=>rustupTomlValueOwnedV1(source,store,actual));}
async function rustupTomlValueOwnedV1(source:string,store:RepositoryToolingStoreV1,control:ResourceLeaseControl):Promise<unknown>{requireIssuedResourceControlInvocationV1(control,store.invocation);requireIssuedResourceControlRefinementV1(store.control,control);requireNativeStorageReservationV1(store.storage,store.control);if(Buffer.byteLength(source)>schema.definitions.Protocol.const.maximumDistributionManifestBytes)throw Error("Bounded Rustup TOML bytes required");await store.storage.recheck();return runtimeParseTomlV1(source);}""")

files[rt]["before"] = True; need(rt, "runtimeParseTomlV1")

# --- sandbox -----------------------------------------------------------------------------------
SB = P + "/🧪️testing/🏖️sandbox"
root_ = SB + "/📁️root/🟦️.ts"
cut(root_, "/** 🏁️ Registers a callback for the end of the current bun test file", "  } catch {}\n}\n", """/** 🏁️ Registers a callback for the end of the current test file, because some test runners end their process without running exit handlers. */
function atTestEnd(callback: () => void): void {
  try { runtimeTestApiV1()?.afterAll(callback); } catch {}
}
""")
sub(root_, "atBunTestEnd(() => sandbox.dispose());", "atTestEnd(() => sandbox.dispose());"); need(root_, "runtimeTestApiV1")
gd = SB + "/🛡️guard/🟦️.ts"
sub(gd, "/** 🛡️ Inserts the guard preload after the `test` verb of a bun test command when the guard is active and the preload exists next to this module; every other command is returned unchanged. */", "/** 🛡️ Inserts the guard preload into the test command of the hosting runtime when the guard is active and that runtime's adapter directory holds a preload next to this module; every other command is returned unchanged. */")
sub(gd, '  const mode = env[declared.environment.guard] ?? declared.guard.defaultMode, program = basename(command).replace(/\\.exe$/iu, "");\n  if (mode === "off" || args[0] !== "test" || !(program === "bun" || resolve(command) === resolve(process.execPath))) return args;\n  const preload = fileURLToPath(new URL("./🔌️preload/🟦️.ts", import.meta.url));\n  return existsSync(preload) && !args.includes("--preload") ? [args[0], "--preload", preload, ...args.slice(1)] : args;',
    '  const mode = env[declared.environment.guard] ?? declared.guard.defaultMode;\n  if (mode === "off" || !isHostRuntimeCommandV1(command)) return args;\n  const preload = fileURLToPath(new URL(`./${runtimeAdapterNameV1()}/🔌️preload/🟦️.ts`, import.meta.url));\n  return existsSync(preload) ? runtimeTestPreloadV1(args, preload) ?? args : args;')
need(gd, "isHostRuntimeCommandV1", "runtimeAdapterNameV1", "runtimeTestPreloadV1")
pre = SB + "/🛡️guard/🔌️preload/🟦️.ts"
sub(pre, 'fs: join(here, "../🩻️fs/🟦️.ts"), "fs/promises": join(here, "../🩻️fs-promises/🟦️.ts"), child_process: join(here, "../🩻️child-process/🟦️.ts"),', 'fs: join(here, "../../🩻️fs/🟦️.ts"), "fs/promises": join(here, "../../🩻️fs-promises/🟦️.ts"), child_process: join(here, "../../🩻️child-process/🟦️.ts"),')
sub(pre, 'const guardRoot = join(here, "../..");', 'const guardRoot = join(here, "../../..");')
MOVES.append({"from": pre, "to": SB + "/🛡️guard/🍞️bun/🔌️preload/🟦️.ts"})
MOVES.append({"from": SB + "/🛡️guard/🩻️sqlite/🟦️.ts", "to": SB + "/🛡️guard/🍞️bun/🩻️sqlite/🟦️.ts"})
sbt = SB + "/🧪️tests/🟦️.ts"
sub(sbt, 'const preload = join(root, "🛡️guard/🔌️preload/🟦️.ts");', 'const preload = join(root, "🛡️guard/🍞️bun/🔌️preload/🟦️.ts");', count=2)
sub(sbt, 'join(root, "🛡️guard/🩻️sqlite/🟦️.ts")', 'join(root, "🛡️guard/🍞️bun/🩻️sqlite/🟦️.ts")')
bf = P + "/🧪️testing/🟨️bun/📁️files/🟦️.ts"
sub(bf, '||!process.versions.bun||!isAbsolute(selection.cwd)', '||!runtimeSupportsV1("test")||!isAbsolute(selection.cwd)')
sub(bf, 'captureOwnedCommandResultV1(process.execPath,["test",file,...selection.arguments]', 'captureOwnedCommandResultV1(process.execPath,[...runtimeTestArgumentsV1([file]),...selection.arguments]'); need(bf, "runtimeSupportsV1", "runtimeTestArgumentsV1")

# --- ambient type shim belongs to the Bun adapter -----------------------------------------------
AMB = "🧰️framework/📦️packages/🟦️typescript/🌿️ambient"
MOVES.append({"from": AMB + "/🗄️bun-sqlite-statement/🟦️.d.ts", "to": AMB + "/🍞️bun/🗄️sqlite-statement/🟦️.d.ts"})

# --- Bun-only embedded programs live in Bun adapter directories ----------------------------------
EV = P + "/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🍎️events"
ev2 = EV + "/🟦️.ts"
sub(ev2, 'const nativeVolumeEventPreparationSourceV1=String.raw`', "/** 🍎️ Closed Bun child program that observes one volume's event service through Bun's FFI; the preparation command starts it with the Bun executable. */\nexport const nativeVolumeEventPreparationSourceV1=String.raw`")
sub(ev2, 'events,closed:true}));`;\n', 'events,closed:true}));`;\n/** 🍎️ Gives the executable that runs the closed volume event program. */\nexport const nativeVolumeEventPreparationProgramV1=():string=>process.versions.bun?process.execPath:"bun";\n')
sub(ev2, 'return Object.freeze({program:process.execPath,args:Object.freeze(["-e"', 'return Object.freeze({program:nativeVolumeEventPreparationProgramV1(),args:Object.freeze(["-e"')
extract(ev2, "/** 🍎️ Closed Bun child program", 'process.versions.bun?process.execPath:"bun";', EV + "/🍞️bun/🟦️.ts", ["nativeVolumeEventPreparationSourceV1", "nativeVolumeEventPreparationProgramV1"])
wt2 = P + "/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🧪️tests/🟦️.ts"
sub(wt2, 'createRequire:()=>()=>library,checkResourceLeaseControl', 'createRequire:()=>()=>library,runtimeSupportsV1:()=>true,runtimeNativeSystemV1:()=>library,checkResourceLeaseControl')
sub(wt2, '"utf8"),tree=ts.createSourceFile("service-child.ts",source,', '"utf8"),payloadSource=await fs.readFile(new URL("../🍎️events/🍞️bun/🟦️.ts",import.meta.url),"utf8"),payloadTree=ts.createSourceFile("service-payload.ts",payloadSource,ts.ScriptTarget.Latest,true),tree=ts.createSourceFile("service-child.ts",source,')
sub(wt2, '"utf8"),tree=ts.createSourceFile("service-worker-input.ts",source,', '"utf8"),payloadSource=await fs.readFile(new URL("../🍎️events/🍞️bun/🟦️.ts",import.meta.url),"utf8"),payloadTree=ts.createSourceFile("service-payload.ts",payloadSource,ts.ScriptTarget.Latest,true),tree=ts.createSourceFile("service-worker-input.ts",source,')
sub(wt2, 'script=tree.statements.find(node=>ts.isVariableStatement(node)&&node.declarationList.declarations.some(value=>ts.isIdentifier(value.name)&&value.name.text==="nativeVolumeEventPreparationSourceV1"))', 'script=payloadTree.statements.find(node=>ts.isVariableStatement(node)&&node.declarationList.declarations.some(value=>ts.isIdentifier(value.name)&&value.name.text==="nativeVolumeEventPreparationSourceV1"))', count=2)
sub(wt2, 'new Function("schema","process",compile(script!.getText(tree))', 'new Function("schema","process","runtimeSupportsV1","nativeVolumeEventPreparationProgramV1",compile(script!.getText(payloadTree).replace(/^export /u,""))', count=2)
sub(wt2, '(schema,{platform:"darwin",versions:{bun:"Source"},execPath:"/bun"})', '(schema,{platform:"darwin",versions:{bun:"Source"},execPath:"/bun"},()=>true,()=>"/bun")')
sub(wt2, '(schema,{platform:"darwin",versions:{bun:"Source"},execPath:process.execPath})', '(schema,{platform:"darwin",versions:{bun:"Source"},execPath:process.execPath},()=>true,()=>process.execPath)')
CL = L + "/🏗️build/📥️source/🔏️closure"
extract(CL + "/🟦️.ts", "/** 🏃️ Runs only checksum-bound captured modules at their original Source URLs. */\nexport const buildSourceClosureLoaderV1=String.raw`", "\n`;", CL + "/🍞️bun/🟦️.ts", ["buildSourceClosureLoaderV1"], reexport=True, prefix='import schema from "../🧬️schema/🔣️.json" with { type: "json" };\n\nconst policy = schema.definitions.Policy.const;\n\n')
BR = L + "/🏗️build/🔧️tools/🏗️bazel/🏃️execution/🏭️runner"
extract(BR + "/🟦️.ts", "/** 🏭️ The self-contained program a hermetic Bazel action runs", "\n`;", BR + "/🍞️bun/🟦️.ts", ["BAZEL_HERMETIC_RUNNER"], reexport=True)

# --- test harnesses that evaluate production text need the owned functions bound --------------------
cache_tests = P + "/📦️artifacts/💾️cache/🧪️tests/🟦️.ts"
sub(cache_tests, r'process:\{platform:"win32",arch:"x64",versions:\{bun:"Source-only"\}\},', r'process:{platform:"win32",arch:"x64",versions:{bun:"Source-only"}},runtimeSupportsV1:()=>true,', regex=True)

# --- analyzers recognise the owned file reader instead of the Bun global ------------------------------------
sr = L + "/🔍️discovery/🔗️schema-reads/🟦️.ts"
sub(sr, 'return ["JSON", "Bun", "URL", "import"].includes(name)', 'return ["JSON", "URL", "import"].includes(name)')
sub(sr, '      if (owner?.kind === "namespace" && owner.name === "Bun" && property === "file" && args[0]?.kind === "url") return { kind: "text", name: args[0].name };\n', '')
sub(sr, '    if (callee.kind === "namespace" && callee.name === "fs.readFileSync" && args[0]?.kind === "url") return { kind: "text", name: args[0].name };', '    if (callee.kind === "namespace" && (callee.name === "fs.readFileSync" || callee.name === "runtime.file") && args[0]?.kind === "url") return { kind: "text", name: args[0].name };')
sub(sr, '    if (["node:fs", "fs"].includes(module)) return { kind: "namespace", name: name === "*" ? "fs" : `fs.${name}` };', '    if (["node:fs", "fs"].includes(module)) return { kind: "namespace", name: name === "*" ? "fs" : `fs.${name}` };\n    if (/\\/⚙️runtime\\/(?:🌍️serve\\/)?🟦️\\.ts$/u.test(module) && name === "runtimeFileV1") return { kind: "namespace", name: "runtime.file" };')
sub(sr.rsplit("/", 1)[0] + "/🧪️tests/🧫️fixtures/🔣️.json", r'import schema from \"./schema.json\";const fixture=Bun.file(new URL(', r'import schema from \"./schema.json\";import {runtimeFileV1} from \"../../⚙️runtime/🌍️serve/🟦️.ts\";const fixture=runtimeFileV1(new URL(')
rd = L + "/🔍️discovery/🕸️runtime/🟦️.ts"
sub(rd, 'const readers = new Set(["readFileSync", "readFile"]);', 'const readers = new Set(["readFileSync", "readFile", "runtimeFileV1"]);')
sub(rd, '    if ((readers.has(name!) || name === "file") && tokens[index + 1]?.text === "(") {\n      if (name === "file" && tokens[index - 2]?.text !== "Bun") continue;\n', '    if (readers.has(name!) && tokens[index + 1]?.text === "(") {\n')
tl = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🗺️test-layout/🟦️.ts"
sub(tl, 'const fileReader = ts.isIdentifier(call) && fileBinding(call) === 1 || ts.isPropertyAccessExpression(call) && ts.isIdentifier(call.expression) && (fileBinding(call.expression) === 4 && readers.has(call.name.text) || call.expression.text === "Bun" && bound(call.expression) === undefined && call.name.text === "file");', 'const fileReader = ts.isIdentifier(call) && (fileBinding(call) === 1 || call.text === "runtimeFileV1") || ts.isPropertyAccessExpression(call) && ts.isIdentifier(call.expression) && fileBinding(call.expression) === 4 && readers.has(call.name.text);')
sc = L + "/🔍️discovery/🧩️semantic-census/🟦️.ts"
sub(sc, '["import", "default", "types", "bun", "node"].includes(key)', '["import", "default", "types", ...runtimeIdsV1()].includes(key)'); need(sc, "runtimeIdsV1")
cjs = L + "/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs"
sub(cjs, '    "bun:test",\n', '    "node:test",\n')
sub(cjs, '    "bun:sqlite",\n', '    "node:sqlite",\n')
pp = L + "/🧹️normalization/🧾️plan-parsing/🟦️.ts"
sub(pp, 'command.length !== 4 || command[0] !== "bun" ||', 'command.length !== 4 || !isRuntimeExecutableV1(command[0]) ||')
sub(pp, 'verifyCommand.length !== 4 || verifyCommand[0] !== "bun" ||', 'verifyCommand.length !== 4 || !isRuntimeExecutableV1(verifyCommand[0]) ||'); need(pp, "isRuntimeExecutableV1")
TT = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test"
mo = TT + "/🏭️inventory/📋️orchestration/🟦️.ts"
sub(mo, 'if (existsSync(abs)) return { command: "bun", args: [abs,', 'if (existsSync(abs)) return { command: runtimeCommandNameV1(), args: [abs,')
sub(mo, 'if (supplied) return { command: "bun", args: [supplied.script,', 'if (supplied) return { command: runtimeCommandNameV1(), args: [supplied.script,'); need(mo, "runtimeCommandNameV1")
hm = TT + "/🖥️host/🏗️materialization/🟦️.ts"
sub(hm, '    command: "bun",\n    args: [join(repoRoot, REPO_TEST_DOMAIN_REL, "🖥️host", "🟦️.ts")', '    command: runtimeCommandNameV1(),\n    args: [join(repoRoot, REPO_TEST_DOMAIN_REL, "🖥️host", "🟦️.ts")'); need(hm, "runtimeCommandNameV1")
di = TT + "/🩺️environment/📋️inspection/🟦️.ts"
sub(di, '["typescript", "bun", ["--version"]],', '["typescript", runtimeCommandNameV1(), ["--version"]],'); need(di, "runtimeCommandNameV1")
oa = L + "/🔍️discovery/✅️owned-file-approvals/🟦️.ts"
sub(oa, '"--", "bun", ...direct]', '"--", runtimeCommandNameV1(), ...direct]', count=2)
sub(oa, 'const expected = ["bun", ...(args ?? [])];', 'const expected = [runtimeCommandNameV1(), ...(args ?? [])];')
sub(oa, 'return { command: "bun", args, cwd: options.cwd as string };', 'return { command: runtimeCommandNameV1(), args, cwd: options.cwd as string };'); need(oa, "runtimeCommandNameV1")
ld = L + "/🏗️build/📥️inputs/📋️data/📥️load/🟨️.mjs"
sub(ld, 'const blob="Bun"in globalThis,target=blob?URL.createObjectURL(new Blob([sourceBytes],{type:"text/javascript"})):"data:text/javascript;base64,"+sourceBytes.toString("base64");', 'const sourceModule=runtimeSourceUrlV1(sourceBytes),target=sourceModule.url;')
sub(ld, 'finally{if(blob)URL.revokeObjectURL(target);}', 'finally{sourceModule.release();}'); need(ld, "runtimeSourceUrlV1")
sub(bb2, 'runtime: "bun@" + Bun.version,', 'runtime: ${JSON.stringify(runtimeIdentityV1().id)} + "@" + process.versions[${JSON.stringify(runtimeDetectKeyV1())}],')

ct = "🧰️framework/🔨️modules/⏳️async/🪃️continuation/🟦️.ts"
sub(ct, "{ process?: { versions?: { node?: string; bun?: string } }; setImmediate", "{ process?: { versions?: { node?: string } }; setImmediate")
sub(ct, "if ((host.process?.versions?.node || host.process?.versions?.bun) && typeof", "if (host.process?.versions?.node && typeof")

sub(snap, "const bytes=new Uint8Array(await result.outputs[0]!.arrayBuffer());", "const bytes=await result.outputs[0]!.bytes();")
sub(aztest, 'target==="bun"?process.execPath:"node"', 'target==="runtime"?process.execPath:"node"', count=3)
sub(aztest, 'control,"bun","program"),program=join(output,".sealed-compiler.mjs")', 'control,"runtime","program"),program=join(output,".sealed-compiler.mjs")')

pt = L + "/🏗️build/⏱️profile/🧪️tests/🟦️.ts"
fac = os.path.relpath(os.path.join(ROOT, "🧰️framework/🔨️modules/⚙️runtime/🟦️.ts"), os.path.dirname(os.path.join(ROOT, pt)))
sub(pt, 'program=`const m=await import(${JSON.stringify(module)});await m.profilePhaseV1("test.outer",async()=>{Bun.spawnSync([process.execPath,"-e","0"]);await Bun.sleep(5);});`', 'runtime=new URL("' + fac + '",import.meta.url).href,program=`const m=await import(${JSON.stringify(module)}),r=await import(${JSON.stringify(runtime)});await m.profilePhaseV1("test.outer",async()=>{r.runtimeSpawnSyncV1([process.execPath,"-e","0"]);await r.runtimeSleepV1(5);});`')
