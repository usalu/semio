"""🧷 builds staged/runtimelaw/spec.json: the runtime-law fixes that need no design (neutral facades instead of runtime names)."""
import json, os
C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
F = "🧰️framework"
L = F + "/🛍️products/🦑️repo/🔨️modules/📚️library"
P = F + "/🔨️modules/🏃️process"
FACADE = F + "/📦️packages/🟦️typescript/⚙️runtime/"
def imp(file, name, entry):
    return os.path.relpath(FACADE + entry + "/🟦️.ts", os.path.dirname(file))
files = {}
def edit(file, old, new): files.setdefault(file, []).append({"old": old, "new": new})

f = P + "/🏗️build/🟦️typescript/🔁️incremental/🟦️.ts"
edit(f, 'import {isAbsolute,join,resolve} from "node:path";\n', 'import {isAbsolute,join,resolve} from "node:path";\nimport {runtimeCollectGarbageV1} from "%s";\n' % imp(f, 0, "🏷️identity"))
edit(f, 'collect=():void=>{const host=globalThis as {Bun?:{gc(force:boolean):void};gc?:()=>void};if(host.Bun)host.Bun.gc(true);else host.gc?.();},', 'collect=():void=>runtimeCollectGarbageV1(true),')

edit(L + "/⚡️caching/🗂️suites/🟦️.ts", 'resolve(import.meta.dir,"..")', 'resolve(import.meta.dirname,"..")')
edit(L + "/🏗️build/🗂️suites/🟦️.ts", 'resolve(import.meta.dir,"..")', 'resolve(import.meta.dirname,"..")')
edit(L + "/🏗️build/🧾️ci-report/🗂️suites/🟦️.ts", 'resolve(import.meta.dir,"..","..",', 'resolve(import.meta.dirname,"..","..",')

f = L + "/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧊️memo/🟦️.ts"
edit(f, 'import{dirname,join,resolve}from"node:path";\n', 'import{dirname,join,resolve}from"node:path";\nimport{runtimeKeyV1}from"%s";\n' % imp(f, 0, "🏷️identity"))
edit(f, 'String((globalThis as {Bun?:{version?:string}}).Bun?.version??process.version)+"\\0"', 'runtimeKeyV1()+"\\0"')

f = L + "/🏗️build/🏛️sealed/📤️promotion/🟦️.ts"
edit(f, 'import{Database}from"bun:sqlite";\n', 'import{SqliteDatabaseV1}from"%s";\nimport{TranspilerV1}from"%s";\n' % (imp(f, 0, "🗃️sqlite"), imp(f, 0, "🔬️source")))
edit(f, 'InstanceType<typeof Bun.Transpiler>', 'TranspilerV1')
edit(f, 'new Bun.Transpiler({loader,target:"bun"})', 'new TranspilerV1({loader,target:"runtime"})')
edit(f, 'new Database(entry.lock)', 'new SqliteDatabaseV1(entry.lock)')

f = L + "/🏗️build/🏡️resident/📜️journal/⌨️main/🟦️.ts"
edit(f, 'import {runChangeJournalMainV1} from "../🟦️.ts";\n', 'import {runChangeJournalMainV1} from "../🟦️.ts";\nimport {isMainModuleV1} from "%s";\n' % imp(f, 0, "🏁️main"))
edit(f, 'if(import.meta.main)process.exit(', 'if(isMainModuleV1(import.meta.url))process.exit(')

f = L + "/🏗️build/📥️inputs/🔭️read-oracle/🚪️cli/🟦️.ts"
edit(f, 'import {onTerminationV1}', 'import {isMainModuleV1, onTerminationV1}')
edit(f, 'if (import.meta.main) process.exit(', 'if (isMainModuleV1(import.meta.url)) process.exit(')

f = L + "/🏗️build/🚪️door/📜️script.ts"
edit(f, 'import {liveImplementationStartV1} from "../🏛️sealed/🧭️mode/🟦️.ts";\n', 'import {liveImplementationStartV1} from "../🏛️sealed/🧭️mode/🟦️.ts";\nimport {realpathSync} from "node:fs";\n\n/** 🏁️ Tells whether this file is the process entry point; the door cannot import the runtime seam without leaving its startup module budget, so it compares the real paths itself, exactly as the seam does for every other module. */\nconst mainEntryV1=():boolean=>{const entry=process.argv[1];if(entry===undefined||entry==="")return false;try{const canonical=(path:string):string=>process.platform==="win32"?realpathSync(path).toLowerCase():realpathSync(path);return canonical(entry)===canonical(import.meta.filename);}catch{return false;}};\n')
edit(f, 'if(import.meta.main){', 'if(mainEntryV1()){')

f = L + "/🏗️build/🔧️tools/🏗️bazel/📜️script.ts"
edit(f, 'bun:(globalThis as {Bun?:{version?:string}}).Bun?.version??process.version,', 'bun:runtimeIdentityV1().version,')

f = L + "/🟨️.mjs"
edit(f, 'import { isRuntimeBuiltinSpecifierV1 }', 'import { isRuntimeBuiltinSpecifierV1, runtimeIdsV1 }')
edit(f, 'for(const condition of["development","import","default","node","bun"])', 'for(const condition of["development","import","default","node",...runtimeIdsV1()])')


f = F + "/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📌️runtime-pin/⌨️command/🟦️.ts"
edit(f, 'const ffi=await import("bun:ffi");', 'const{runtimeNativeSystemV1}=await import("%s"),ffi=runtimeNativeSystemV1();' % imp(f, 0, "🧲️native"))
edit(f, 'return Bun.which("repo",{PATH:environment.PATH??""})??undefined;', 'const extensions=process.platform==="win32"?(environment.PATHEXT??".EXE;.CMD;.BAT").split(";").filter(Boolean):[""];\n  for(const directory of (environment.PATH??"").split(delimiter)){\n    if(directory==="")continue;\n    for(const extension of extensions){\n      const candidate=join(directory,"repo"+extension);\n      try{if(!lstatSync(candidate).isFile()&&!statSync(candidate).isFile())continue;if(process.platform!=="win32")accessSync(candidate,constants.X_OK);return candidate;}catch{}\n    }\n  }\n  return undefined;')
edit(f, 'import{lstatSync,readFileSync,readdirSync}from"node:fs";', 'import{accessSync,constants,lstatSync,readFileSync,readdirSync,statSync}from"node:fs";')
edit(f, '/** 🔎️ Finds the `repo` launcher on the PATH. */', '/** 🔎️ Finds the `repo` launcher on the PATH with plain file system calls: this module runs in the preload of every process, so it loads neither the runtime adapter nor the runtime seam. */')
edit(f, 'export async function guardRuntimeV1(', '/** 🏷️ The version of the selected pinned runtime that is running this process, read from the `process.versions` entry named by the pin registry, so the preload names no runtime and loads nothing. */\nexport function runningVersionV1(root:string,environment:Readonly<Record<string,string|undefined>>):string{\n  const registry=readRuntimePinRegistryV1(root);\n  return (process.versions as Readonly<Record<string,string|undefined>>)[environment[registry.cache.runtimeEnvironment]||registry.selected]??"";\n}\n\nexport async function guardRuntimeV1(')
edit(f, 'options.running??process.versions.bun??""', 'options.running??runningVersionV1(root,environment)')
edit(f, '"tools","exec","--light","bun","--"', '"tools","exec","--light",check.name,"--"')
M = F + "/🔨️modules/⚙️runtime"
f = M + "/🏷️identity/🟦️.ts"
edit(f, '/** 🔑️ Gives stable identity text for cache keys, such as `bun:1.3.14`. */', '/** 🔤️ Lists the names of the global namespace objects of every registered runtime in registry order: the detection key with a capital first letter, which source analyzers need to recognise runtime members in foreign code. */\nexport function runtimeNamespaceNamesV1(): string[] { return (RUNTIME_REGISTRY_V1.runtimes as readonly { detect: string }[]).map(row => row.detect.charAt(0).toUpperCase() + row.detect.slice(1)); }\n/** 🔑️ Gives stable identity text for cache keys, such as `bun:1.3.14`. */')
edit(F + "/📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts", 'runtimeKeyV1, ', 'runtimeKeyV1, runtimeNamespaceNamesV1, ')
f = M + "/🧪️tests/🟦️.ts"
edit(f, 'runtimeKeyV1, runtimeObserveSpawnV1,', 'runtimeKeyV1, runtimeNamespaceNamesV1, runtimeObserveSpawnV1,')
edit(f, '    expect(runtimeIdsV1()).toEqual(ids);\n', '    expect(runtimeIdsV1()).toEqual(ids);\n    expect(runtimeNamespaceNamesV1()).toEqual(runtimeCommandNamesV1().map(word => word.charAt(0).toUpperCase() + word.slice(1)));\n')
f = F + "/🛍️products/🦑️repo/🔨️modules/🧪️test/🗺️test-layout/🟦️.ts"
edit(f, 'import { runtimeParseTomlV1 } from "../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";\n', 'import { runtimeParseTomlV1 } from "../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";\nimport { runtimeNamespaceNamesV1 } from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";\n')
edit(f, 'call.expression.text === "Bun" && call.name.text === "file"', 'runtimeNamespaceNamesV1().includes(call.expression.text) && call.name.text === "file"')
f = F + "/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📌️runtime-pin/⌨️command/🟦️.ts"
edit(f, 'userCacheDirectory:userCacheDirectoryV1(process.platform,environment,homedir())', 'userCacheDirectory:userCacheDirectoryV1(process.platform,environment,environment.HOME||environment.USERPROFILE||homedir())')
f = F + "/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/📌️runtime-pin-contract/🟦️.ts"
edit(f, 'environment={SEMIO_LANGUAGE:"en",PATH:"/usr/bin:/bin"},host=systemRuntimePinHostV1(directory,environment)', 'environment={SEMIO_LANGUAGE:"en",PATH:"/usr/bin:/bin",HOME:directory,USERPROFILE:directory,LocalAppData:join(directory,"LocalAppData"),XDG_CACHE_HOME:join(directory,".cache")},host=systemRuntimePinHostV1(directory,environment)')

extra = json.load(open(C + "/red-g-tools/staged/bazelattr-spec.json", encoding="utf8"))["files"]
for file, edits in extra.items(): files.setdefault(file, []).extend(edits)
json.dump({"files": files}, open(C + "/red-g-tools/staged/runtimelaw/spec.json", "w"), ensure_ascii=False, indent=1)
