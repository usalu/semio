#!/usr/bin/env python3
"""Builds manual-edits.json: anchored text edits for the Bun sites the AST codemod does not own. Run read-only against the live tree."""
import json, os, re, sys
ROOT = os.environ.get("RUNTIME_SEAM_ROOT", "/Users/ueli/Documents/semio")
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
P = "🧰️framework/🔨️modules/🏃️process"
NB = P + "/📦️artifacts/🏗️native-build/💾️storage"
files = {}
def text(path): return open(os.path.join(ROOT, path), encoding="utf8").read()
def entry(path):
    return files.setdefault(path, {"path": path, "edits": [], "imports": []})
def sub(path, old, new, count=1, regex=False):
    """Plain or regex substitution; regex matches become concrete anchored edits."""
    src = text(path); row = entry(path)
    if regex:
        found = {}
        for m in re.finditer(old, src, flags=re.S):
            found.setdefault(m.group(0), []).append(m)
        if not found: raise SystemExit(f"NO MATCH {path}: {old[:80]}")
        for literal, ms in found.items():
            repl = re.sub(old, new, literal, count=1, flags=re.S)
            if literal != repl: row["edits"].append({"old": literal, "new": repl, "count": len(ms)})
        return
    n = src.count(old)
    if n == 0:
        if new in src: return
        raise SystemExit(f"NO MATCH {path}: {old[:80]}")
    if n != count: raise SystemExit(f"COUNT {n} != {count} {path}: {old[:80]}")
    row["edits"].append({"old": old, "new": new, "count": count})
def need(path, *names):
    row = entry(path)
    for name in names:
        if name not in row["imports"]: row["imports"].append(name)

SYSTEM = r'(?:createRequire\(import\.meta\.url\)|process\.getBuiltinModule|systemLibrary)\(\s*"bun:ffi"\s*\)\s*as\s*typeof import\("bun:ffi"\)'
def ffi_loader(path):
    sub(path, SYSTEM, "runtimeNativeSystemV1()", regex=True); need(path, "runtimeNativeSystemV1")
def ffi_guard(path):
    sub(path, r"!process\.versions\.bun", '!runtimeSupportsV1("ffi")', regex=True); need(path, "runtimeSupportsV1")

# --- ffi ---------------------------------------------------------------
sub(P + "/🔒️leases/📁️physical/🟦️.ts", 'const {dlopen,read}=process.getBuiltinModule("bun:ffi") as typeof import("bun:ffi"),errnoName=', 'const {dlopen,read}=runtimeNativeSystemV1(),errnoName=')
sub(P + "/🔒️leases/📁️physical/🟦️.ts", "!process.versions.bun||", '!runtimeSupportsV1("ffi")||'); need(P + "/🔒️leases/📁️physical/🟦️.ts", "runtimeNativeSystemV1", "runtimeSupportsV1")
ex = NB + "/📁️directory/🔀️exchange/🟦️.ts"
sub(ex, 'import type{dlopen as BunDlopen}from"bun:ffi";\n', "")
sub(ex, 'const{dlopen,FFIType,read}=createRequire(import.meta.url)("bun:ffi")as typeof import("bun:ffi");', "const{dlopen,FFIType,read}=runtimeNativeSystemV1();")
sub(ex, "ReturnType<typeof BunDlopen>", "ReturnType<typeof dlopen>"); need(ex, "runtimeNativeSystemV1")
host = NB + "/📁️directory/🪟️host/🟦️.ts"
ffi_loader(host); sub(host, r"\|\|!process\.versions\.bun", '||!runtimeSupportsV1("ffi")', regex=True); need(host, "runtimeSupportsV1")
cr = NB + "/🪟️conditional-retirement/🔌️port/🟦️.ts"; ffi_loader(cr)
ev = NB + "/📤️publication/📚️witness/🍎️events/🟦️.ts"
sub(ev, 'check();if(!process.versions.bun)throw Error("Bun Native volume service observer required");const{dlopen,FFIType}=systemLibrary("bun:ffi")as typeof import("bun:ffi");', 'check();if(!runtimeSupportsV1("ffi"))throw Error("Bun Native volume service observer required");const{dlopen,FFIType}=runtimeNativeSystemV1();'); need(ev, "runtimeNativeSystemV1", "runtimeSupportsV1")
js = P + "/📦️artifacts/🏗️native-build/🎰️jobserver/🟦️.ts"
sub(js, '  const ffi = process.getBuiltinModule("bun:ffi") as typeof import("bun:ffi") | undefined;\n  if (ffi === undefined) throw Error("Win32 jobserver semaphores require the Bun runtime");\n  const { dlopen, FFIType } = ffi;\n', '  if (!runtimeSupportsV1("ffi")) throw Error("Win32 jobserver semaphores require the Bun runtime");\n  const { dlopen, FFIType } = runtimeNativeSystemV1();\n'); need(js, "runtimeNativeSystemV1", "runtimeSupportsV1")
desc = "🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts"
ffi_loader(desc); sub(desc, r"\|\|!process\.versions\.bun", '||!runtimeSupportsV1("ffi")', regex=True); need(desc, "runtimeSupportsV1")
tools = L + "/⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts"; ffi_loader(tools)
obs = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/👁️observe/🟦️.ts"
sub(obs, '  const moduleName = "bun:ffi";\n  return await import(moduleName) as TransactionNativeApi;', '  return runtimeNativeSystemV1() as unknown as TransactionNativeApi;'); need(obs, "runtimeNativeSystemV1")

def cut(path, start, end, new, include_end=True):
    src = text(path); i = src.find(start)
    if i < 0:
        if new in src: return
        raise SystemExit(f"NO START {path}: {start[:60]}")
    if src.count(start) != 1: raise SystemExit(f"START NOT UNIQUE {path}: {start[:60]}")
    j = src.index(end, i) + (len(end) if include_end else 0)
    entry(path)["edits"].append({"old": src[i:j], "new": new, "count": 1})

# --- sqlite ------------------------------------------------------------
lease = P + "/🔒️leases/🟦️.ts"
cut(lease, "/** 🗃️ Bun's own SQLite, named through a constant", "  return { exec: sql => database.exec(sql), read: (sql, ...values) => database.prepare(sql).get(...values), close: () => {if(!closed){database.close();closed=true;}} };\n}\n",
 """/** 🗃️ Uses the hosting runtime's system SQLite through the owned runtime connection class. */
async function openDatabase(path: string, readonly=false,create=!readonly): Promise<LeaseDatabase> {
  const database = new SqliteDatabaseV1(path, { readonly, readwrite:!readonly, create });let closed=false;
  return { exec: sql => database.exec(sql), read: (sql, ...values) => {const statement=database.prepare(sql);try{return statement.get(...values) as Record<string, unknown>|undefined;}finally{statement.finalize();}}, close: () => {if(!closed){database.close(true);closed=true;}} };
}
""")
need(lease, "SqliteDatabaseV1")
base = P + "/🔒️leases/📦️namespace/📒️baseline/🟦️.ts"
src = text(base); i = src.index('const builtin="bun:sqlite" as const;\n'); j = src.index("\n}\n", i) + 3
fn = src[i:j]; lines = fn.split("\n")
bun_line = lines[2]; assert bun_line.startswith(" if(process.versions.bun){const{Database}=await import(builtin),"), bun_line[:80]
assert bun_line.endswith("};}")
new_line = " const Database=SqliteDatabaseV1," + bun_line[len(" if(process.versions.bun){const{Database}=await import(builtin),"):-1]
entry(base)["edits"].append({"old": fn, "new": "\n".join([lines[1], new_line, "}", ""]), "count": 1}); need(base, "SqliteDatabaseV1")
ledger = L + "/🏗️build/📥️inputs/💾️storage/📒️ledger/🟨️.mjs"
src = text(ledger); i = src.index("async function databaseV1(path){"); j = src.index("\n}\n", i) + 3
fn = src[i:j]; lines = fn.split("\n"); b = lines[1]
prefix = ' if(process.versions.bun){const name="bun:sqlite",{Database}=await import(name),database=new Database(path,{create:true});'
assert b.startswith(prefix) and b.endswith(";}")
entry(ledger)["edits"].append({"old": fn, "new": "\n".join([lines[0], " const database=new SqliteDatabaseV1(path,{create:true});" + b[len(prefix):-1], "}", ""]), "count": 1}); need(ledger, "SqliteDatabaseV1")
cat = L + "/⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📂️files/🗄️catalog/🟦️.ts"
sub(cat, ',builtin="bun:sqlite"as const;', ';')
src = text(cat); i = src.index("async function database(file:string)"); j = src.index("\n}\n", i) + 3
fn = src[i:j]; lines = fn.split("\n"); b = lines[1]
prefix = ' if(process.versions.bun){const{Database}=await import(builtin),actual=new Database(file,{create:true,readwrite:true}),'
assert b.startswith(prefix) and b.endswith("};}")
entry(cat)["edits"].append({"old": fn, "new": "\n".join([lines[0], " const actual=new SqliteDatabaseV1(file,{create:true,readwrite:true})," + b[len(prefix):-1], "}", ""]), "count": 1}); need(cat, "SqliteDatabaseV1")

# --- cache keys and versions (digest-stable on Bun) ----------------------
vite = L + "/⚡️caching/🌐️vite/🟦️.ts"
sub(vite, ":bun:${process.versions.bun}", ":${runtimeKeyV1()}", count=2); need(vite, "runtimeKeyV1")
story = L + "/🏗️build/📖️storybook/🏃️execution/🟦️.ts"
sub(story, ":bun:${process.versions.bun}", ":${runtimeKeyV1()}", count=2); need(story, "runtimeKeyV1")
dist = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🏗️compiler/🟦️.ts"
sub(dist, '":bun:"+process.versions.bun', '":"+runtimeKeyV1()'); need(dist, "runtimeKeyV1")
bz = L + "/🏗️build/🔧️tools/🏗️bazel/📜️script.ts"
sub(bz, "bun:process.versions.bun,mode:", "bun:runtimeIdentityV1().version,mode:"); need(bz, "runtimeIdentityV1")
boot = L + "/🏗️build/🚀️bootstrap/🟦️.ts"
sub(boot, "manifest.patchedDependencies,process.versions.bun,process.platform", "manifest.patchedDependencies,runtimeIdentityV1().version,process.platform"); need(boot, "runtimeIdentityV1")
store = L + "/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts"
sub(store, "sourceRoot,process.versions.bun,process.platform", "sourceRoot,runtimeIdentityV1().version,process.platform")
sub(store, "protocol.ambientFallback,process.versions.bun,process.versions.node", "protocol.ambientFallback,runtimeIdentityV1().version,process.versions.node"); need(store, "runtimeIdentityV1")
gen = L + "/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/💾️storage/🟦️.ts"
sub(gen, 'bun:String(process.versions.bun),revision:String((globalThis as {Bun?:{revision?:string}}).Bun?.revision??"")', 'bun:String(runtimeIdentityV1().version),revision:String(runtimeIdentityV1().revision)'); need(gen, "runtimeIdentityV1")
code = L + "/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🔏️code/🟦️.ts"
sub(code, 'compiler={name:"Bun",version:Bun.version,revision:Bun.revision,target:"bun",format:"esm",minify:true}', 'compiler={name:runtimeIdentityV1().product,version:runtimeIdentityV1().version,revision:runtimeIdentityV1().revision,target:runtimeIdentityV1().id,format:"esm",minify:true}'); need(code, "runtimeIdentityV1")
wgpu = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/"
bb = wgpu + "⚙️browser-build/🟦️.ts"
sub(bb, "actualVersion: string = Bun.version", "actualVersion: string = runtimeIdentityV1().version")
cut(bb, "  const runtime = globalThis as typeof globalThis & {\n", "  };\n", "")
sub(bb, "const result = await runtime.Bun.build({ entrypoints: [entryPath], target: \"browser\", format: \"esm\", define: { \"import.meta.vitest\": \"undefined\" } });", "const result = await runtimeBundleV1({ entrypoints: [entryPath], target: \"browser\", format: \"esm\", define: { \"import.meta.vitest\": \"undefined\" } });")
need(bb, "runtimeIdentityV1", "runtimeBundleV1")
sub(wgpu + "📽️projection/🟦️.ts", "actualVersion: string = Bun.version", "actualVersion: string = runtimeIdentityV1().version"); need(wgpu + "📽️projection/🟦️.ts", "runtimeIdentityV1")
bundle = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts"
sub(bundle, 'if(Bun.version!=="1.4.2"||', 'if(runtimeIdentityV1().version!=="1.4.2"||')
sub(bundle, 'if (typeof Bun === "undefined" || Bun.version !== "1.4.2" || ', 'if (runtimeIdentityV1().version !== "1.4.2" || '); need(bundle, "runtimeIdentityV1")

# --- serve / file / write -------------------------------------------------
rc = L + "/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts"
sub(rc, "Bun.file(entry.path)", "runtimeFileV1(entry.path).body()")
sub(rc, "const server=Bun.serve({hostname:options.host,port:options.port,idleTimeout:60,maxRequestBodySize:options.maximumEntryBytes*2,fetch:request=>handler(request)});", "const server=await runtimeServeV1({hostname:options.host,port:options.port,idleTimeoutSeconds:60,maximumRequestBodyBytes:options.maximumEntryBytes*2,fetch:request=>handler(request)});"); need(rc, "runtimeFileV1", "runtimeServeV1")
sub(story, "const server=Bun.serve({hostname:options.host,port:options.port,async fetch(request){", "const server=await runtimeServeV1({hostname:options.host,port:options.port,async fetch(request){")
sub(story, 'const output=Bun.file(file);if(!await output.exists())return new Response("Not Found",{status:404});return new Response(output);', 'const output=runtimeFileV1(file);if(!await output.exists())return new Response("Not Found",{status:404});return new Response(output.body());'); need(story, "runtimeFileV1", "runtimeServeV1")
dc = L + "/⚡️caching/📦️artifacts/📝️derived-config/🟦️.ts"
sub(dc, 'import { existsSync, readFileSync, readdirSync } from "node:fs";', 'import { existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";')
sub(dc, "await Bun.write(file, next);", "writeFileSync(file, next);")

# --- builtin names left after the codemod --------------------------------
zt = L + "/🏗️build/📥️source/🛗️zero-touch/🟦️.ts"
sub(zt, ' || specifier === "bun" ||', ' ||', count=2)
sub(zt, ' || specifier.startsWith("bun:") ||', ' || isRuntimeBuiltinSpecifierV1(specifier) ||', count=2) if False else None
disc = L + "/🔍️discovery/🕸️runtime/🟦️.ts"
sub(disc, ' || target === "bun" ||', ' ||')
inv = L + "/🕸️dependencies/📇️inventory/🟦️.ts"
sub(inv, 'const DEPENDENCY_JS_BUILTIN_PREFIXES = ["node:", "bun:"];\n', "")
sub(inv, "DEPENDENCY_JS_BUILTIN_PREFIXES.some((prefix) => specifier.startsWith(prefix))", "isRuntimeBuiltinSpecifierV1(specifier)", count=2); need(inv, "isRuntimeBuiltinSpecifierV1")
reg = L + "/🔍️discovery/📇️registry-catalog-inputs/🟦️.ts"
sub(reg, 'platform: unknown = Reflect.get(globalThis, "Bun")): readonly RegistryCompilerImport[] {', 'platform: unknown = undefined): readonly RegistryCompilerImport[] {')
sub(reg, '  if (platform === null || typeof platform !== "object" && typeof platform !== "function") throw new Error("Registry import discovery requires Bun\'s compiler runtime.");', '  if (platform === undefined) return new TranspilerV1({ loader: language }).scanImports(source).map((row) => ({ path: row.path, kind: row.kind }));\n  if (platform === null || typeof platform !== "object" && typeof platform !== "function") throw new Error("Registry import discovery requires Bun\'s compiler runtime.");')
need(reg, "TranspilerV1")

# --- remaining hand sites -------------------------------------------------
sub(cr, "||!process.versions.bun||", '||!runtimeSupportsV1("ffi")||'); need(cr, "runtimeSupportsV1")
sub(ev, 'const.platform||!process.versions.bun||!Number.isSafeInteger(device)', 'const.platform||!runtimeSupportsV1("ffi")||!Number.isSafeInteger(device)')
tcargo = P + "/🧪️testing/🦀️cargo/🟦️.ts"
sub(tcargo, 'await import("bun:ffi")', "runtimeNativeSystemV1()"); need(tcargo, "runtimeNativeSystemV1")
ready = L + "/🏃️process/🦀️cargo/📥️dependencies/🟢️ready/💾️storage/🟦️.ts"
sub(ready, 'typeof Bun==="undefined"', '!runtimeSupportsV1("which")', count=3); need(ready, "runtimeSupportsV1")
wp = L + "/🏃️process/🕸️wasm/🔣️policy/🟦️.ts"
cut(wp, "  const runtime = (globalThis as { readonly Bun?: unknown }).Bun;\n", '    : (which as (name: string, options: { readonly PATH?: string }) => string | null)("wasm-bindgen", { PATH: env.PATH });\n', """  if (!env.SEMIO_WASM_BINDGEN_BIN && !runtimeSupportsV1("which")) throw new Error("wasm-bindgen resolution requires a runtime executable lookup.");
  const command = env.SEMIO_WASM_BINDGEN_BIN
    ? resolve(repoRoot, env.SEMIO_WASM_BINDGEN_BIN)
    : runtimeWhichV1("wasm-bindgen", { PATH: env.PATH });
"""); need(wp, "runtimeSupportsV1", "runtimeWhichV1")
cp = L + "/🔍️discovery/🚚️cargo-provider/🟦️.ts"
cut(cp, "    const runtime = (globalThis as { readonly Bun?: unknown }).Bun;\n", "    return (toml as { readonly parse: (input: string) => unknown }).parse(source);\n", "    return runtimeParseTomlV1(source);\n"); need(cp, "runtimeParseTomlV1")
ob = L + "/🏗️build/📏️incremental/👁️observers/🟦️.ts"
sub(ob, "const bun=(globalThis as {Bun?:{zstdDecompressSync?:(input:Uint8Array)=>Uint8Array}}).Bun,node=(zlib as unknown as {zstdDecompressSync?:(input:Uint8Array)=>Uint8Array}).zstdDecompressSync,decode=bun?.zstdDecompressSync??node;", "const decode=(zlib as unknown as {zstdDecompressSync?:(input:Uint8Array)=>Uint8Array}).zstdDecompressSync;")
si = L + "/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
sub(si, '  if (!globalThis.Bun?.Glob) throw Error("Declared module globs require Bun.Glob");\n', "")
sub(si, "Glob: globalThis.Bun.Glob, join", "Glob: GlobV1, join"); need(si, "GlobV1")
orch = L + "/🔄️transactions/🧪️verification/📋️orchestration/🟦️.ts"
cut(orch, "  const bun = (\n    globalThis as unknown as {\n", "  ).Bun;\n", "")
sub(orch, 'await bun.build({ entrypoints: [identityPaths.suite!], outdir: bundleRoot, naming: "🟦️.test.js", target: "bun", packages: "external" })', 'await runtimeBundleV1({ entrypoints: [identityPaths.suite!], outdir: bundleRoot, naming: "🟦️.test.js", target: "runtime", packages: "external" })')
sub(orch, 'await bun.build({ entrypoints: [identityPaths.normalization!], outdir: normalizationBundleRoot, naming: "🟦️.js", target: "bun", packages: "external" })', 'await runtimeBundleV1({ entrypoints: [identityPaths.normalization!], outdir: normalizationBundleRoot, naming: "🟦️.js", target: "runtime", packages: "external" })'); need(orch, "runtimeBundleV1")
vsc = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🗃️sqlite-constructor/🟦️.ts"
src = text(vsc); i = src.index("/** 🗃️ The Bun builtin, ASSEMBLED at runtime.")
entry(vsc)["edits"].append({"old": src[i:src.index("//#endregion BackboneVitePlugin")], "new": """/** 📥️ Gives the owned SQLite connection class of the hosting runtime; the facade resolves the runtime's own SQLite lazily, so loading this module under Vite's config bundler never touches a runtime builtin. */
export async function backboneDatabaseCtorLazy(): Promise<typeof SqliteDatabaseV1> {
  return SqliteDatabaseV1;
}
export type BackboneSqliteHandle = InstanceType<typeof SqliteDatabaseV1>;
""", "count": 1}); need(vsc, "SqliteDatabaseV1")
tp = L + "/🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts"
sub(tp, 'types: ["bun", "node"],', 'types: [runtimeTypeEnvironmentV1(), "node"],'); need(tp, "runtimeTypeEnvironmentV1")

ing = L + "/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🟦️.ts"
sub(ing, 'RUSTC_WRAPPER:"bun"', "RUSTC_WRAPPER:runtimeExecutableV1()"); need(ing, "runtimeExecutableV1")
ts_check = L + "/⚡️caching/📦️artifacts/🟦️typescript/🟦️.ts"
sub(ts_check, 'new Bun.CryptoHasher("sha256").update(distance).digest("hex")', 'createHash("sha256").update(distance).digest("hex")')
sub(ts_check, 'import { typeScriptLibraryArgumentsV1 } from "../../../🏗️build/🟦️typescript/📚️libraries/🟦️.ts";\n', 'import { createHash } from "node:crypto";\nimport { typeScriptLibraryArgumentsV1 } from "../../../🏗️build/🟦️typescript/📚️libraries/🟦️.ts";\n')

wt = L + "/🏗️build/🧭️selection/👁️watch/🟦️.ts"
sub(wt, 'if(!["bun","bun.exe",process.execPath].includes(command[0]??""))return undefined;', 'if(!isRuntimeExecutableV1(command[0]??""))return undefined;'); need(wt, "isRuntimeExecutableV1")
pr = L + "/🏗️build/📥️inputs/🎯️precision/🟦️.ts"
sub(pr, 'if (tokens[0] !== "bun") return undefined;', 'if (!isRuntimeExecutableV1(tokens[0] ?? "")) return undefined;'); need(pr, "isRuntimeExecutableV1")

nxm = L + "/🟨️.mjs"
sub(nxm, '!specifier.includes(":")&&specifier!=="bun"', '!specifier.includes(":")&&!isRuntimeBuiltinSpecifierV1(specifier)'); need(nxm, "isRuntimeBuiltinSpecifierV1")

exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "manual-gen2.py"), encoding="utf8").read())
files[vsc]["before"] = True
json.dump({"files": list(files.values()), "moves": MOVES, "deletes": DELETES}, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "manual-edits.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print({k.split("/")[-2]: len(v["edits"]) for k, v in files.items()})
