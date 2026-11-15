import os, sys, hashlib, shutil
V = "️"
def e(cp, rest=""): return chr(cp) + V + rest
FW = "/Users/Shared/semio/" + e(0x1F9F0, "framework")
LIB = FW + "/" + "/".join([e(0x1F6CD, "products"), e(0x1F991, "repo"), e(0x1F528, "modules"), e(0x1F4DA, "library")])
BOOT = LIB + "/" + e(0x26A1, "caching") + "/" + e(0x1F680, "bootstrap")
RC = LIB + "/" + e(0x1F3D7, "build") + "/" + e(0x1F310, "remote-cache")
TS = e(0x1F7E6, ".ts")
dry = "--dry" in sys.argv
log = []

def sub(path, old, new, count=1):
    s = open(path, encoding="utf8").read()
    if new in s and old not in s:
        print("already", path[-60:]); return
    n = s.count(old)
    if n != count:
        raise SystemExit(f"anchor count {n} != {count} in {path}: {old[:100]!r}")
    before = hashlib.sha256(s.encode()).hexdigest()
    s2 = s.replace(old, new)
    if not dry: open(path, "w", encoding="utf8").write(s2)
    log.append((path, before, hashlib.sha256(s2.encode()).hexdigest()))
    print("patched" if not dry else "ok(dry)", path[-70:])

# ---- dependency storage: fleet install state
ST = BOOT + "/" + e(0x1F4E6, "dependencies") + "/" + e(0x1F4BE, "storage") + "/" + TS
s0 = open(ST, encoding="utf8").read(); first = s0.split("\n", 1)[0]
ffi = "../../" + e(0x1F6F0, "fleet-fetch") + "/"
sub(ST, first + "\n", first + "\n" + f'import{{fleetFetchContextV1}}from"{ffi}{TS}";\nimport{{fleetInstallAcquireV1,fleetInstallPublishV1}}from"{ffi}{e(0x1F4E6, "install")}/{TS}";\n')
sub(ST, 'for(const row of fresh){await admitJavascriptDependencyGenerationV1(workspace,store,control,row.scope);',
    'for(const row of fresh){await admitJavascriptDependencyGenerationV1(workspace,store,control,row.scope);const fleetRequest=()=>({context:fleetFetchContextV1(process.env,workspace),projectionSha256:adopter.adoptProjectionOfV1(row.plan.directory,row.plan.files),runtime:adoptRuntime,directory:row.plan.directory,roots:adopter.adoptClonedRootsV1(row.plan.members.length),check:checkpoint,scan:()=>scanJavascriptDependencyInstalledPathsV1(requireNativeStoragePathV1(context.root,join(context.root,"j",row.plan.generation.slice(0,20))),control,()=>checkJavascriptDependencyReadContextV1(context,control)),remove:async(path:string)=>{await removeJavascriptDependencyTreeV1(path,async()=>{checkpoint();});},installed:()=>incremental.newestJavascriptDependencyInstalledReceiptV1(store.root,row.plan.generation),event:(event:{stage:string;detail:string;bytes?:number})=>{void writeOwnedCommandOutputV1(process.stderr,"[DATA] Store dependency fleet "+JSON.stringify({version:1,purpose:"current-dependency-fleet-install",scope:row.scope,...event})+"\\n",control).catch(()=>{});}});')
sub(ST, 'checkpoint();if(reused){await fence();', 'checkpoint();if(reused){await fleetInstallPublishV1(fleetRequest()).catch(()=>false);await fence();')
sub(ST, 'const adoptedFrom=await adopter.adoptJavascriptDependencyInstalledTreeV1({storeRoot:store.root,', 'const localAdopted=await adopter.adoptJavascriptDependencyInstalledTreeV1({storeRoot:store.root,')
sub(ST, 'remove:async path=>{await removeJavascriptDependencyTreeV1(path,async()=>{checkpoint();});}});\n if(adoptedFrom)await writeOwnedCommandOutputV1(',
    'remove:async path=>{await removeJavascriptDependencyTreeV1(path,async()=>{checkpoint();});}});\n const fleetInstall=localAdopted?undefined:await fleetInstallAcquireV1(fleetRequest()),adoptedFrom=localAdopted??(fleetInstall?.restored?"fleet":undefined);\n if(adoptedFrom)await writeOwnedCommandOutputV1(')
sub(ST, 'await adoptSeal(row);frame.built.set(row.scope,', 'await adoptSeal(row);if(fleetInstall){await fleetInstall.publish().catch(()=>false);await fleetInstall.release().catch(()=>undefined);}frame.built.set(row.scope,')

# ---- cargo registry crates
CD = LIB + "/" + e(0x1F3C3, "process") + "/" + e(0x1F980, "cargo") + "/" + e(0x1F4E5, "dependencies") + "/" + TS
s1 = open(CD, encoding="utf8").read(); first1 = s1.split("\n", 1)[0]
ffc = "../../../" + e(0x26A1, "caching") + "/" + e(0x1F680, "bootstrap") + "/" + e(0x1F6F0, "fleet-fetch") + "/"
sub(CD, first1 + "\n", first1 + "\n" + f'import{{fleetFetchContextV1}}from"{ffc}{TS}";\nimport{{fleetCargoRegistryAroundV1}}from"{ffc}{e(0x1F980, "crates")}/{TS}";\n')
sub(CD, 'const fetch=()=>captureOwnedCommandV1("cargo",request.args,request.cwd,{control,env:{...environment,CARGO_NET_OFFLINE:String(request.args.includes("--offline"))},output:{maximumBytes:16777216,maximumLines:65536}});',
    'const run=()=>captureOwnedCommandV1("cargo",request.args,request.cwd,{control,env:{...environment,CARGO_NET_OFFLINE:String(request.args.includes("--offline"))},output:{maximumBytes:16777216,maximumLines:65536}}),fetch=()=>request.args.includes("--offline")?run():fleetCargoRegistryAroundV1(fleetFetchContextV1(environment,options.workspace),{home,manifest:request.manifest,workspace:options.workspace},run);')

# ---- launcher (Go)
GO = FW + "/" + "/".join([e(0x1F6CD, "products"), e(0x1F991, "repo"), e(0x1F528, "modules"), e(0x1F680, "launcher"), e(0x1F4E6, "packages"), e(0x1F439, "go")])
GOF = e(0x1F439, ".go")
GOSRC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/H/go"
sub(GO + "/" + e(0x1F4E5, "runtime-provision") + "/" + GOF,
    '\tarchive := filepath.Join(staging, row.Archive)\n\tif err := q.download(ctx, row, link, base, archive); err != nil {\n\t\treturn err\n\t}\n',
    '\tarchive := filepath.Join(staging, row.Archive)\n\tstore := fleetStoreDirectory(q.Root, q.Env)\n\tif !fleetFetchInto(store, row, archive) {\n\t\tif err := q.download(ctx, row, link, base, archive); err != nil {\n\t\t\treturn err\n\t\t}\n\t\tfleetPublish(store, row, archive)\n\t}\n')
if not dry:
    for src, dst in [("fleet-store.go", GO + "/" + e(0x1F6F0, "fleet-store") + "/" + GOF),
                     ("fleet-provision-test.go", FW + "/" + "/".join([e(0x1F6CD, "products"), e(0x1F991, "repo"), e(0x1F528, "modules"), e(0x1F680, "launcher"), e(0x1F9EA, "tests"), e(0x1F6F0, "fleet-provision")]) + "/" + GOF)]:
        os.makedirs(os.path.dirname(dst), exist_ok=True); shutil.copyfile(os.path.join(GOSRC, src), dst); print("landed", dst[-60:])

# ---- server: mirror routes
SV = RC + "/" + e(0x1F5A5, "server") + "/" + TS
sub(SV, 'const RESERVED=new Set(["ac","cas","v1","healthz","status","claim"])', 'const RESERVED=new Set(["ac","cas","v1","healthz","status","claim","mirror"])')
sub(SV, 'claims?:ClaimTableV1;onRequest?:', 'claims?:ClaimTableV1;mirror?:(request:Request)=>Promise<Response|undefined>;onRequest?:')
sub(SV, 'const url=new URL(request.url),method=request.method;\n  if(url.pathname==="/healthz")', 'const url=new URL(request.url),method=request.method;\n  if(options.mirror!==undefined&&url.pathname.startsWith("/mirror/")){const mirrored=await options.mirror(request);if(mirrored)return mirrored;}\n  if(url.pathname==="/healthz")')
sub(SV, 'export type RemoteCacheServerOptionsV1=Readonly<{hostStore?:', 'export type RemoteCacheServerOptionsV1=Readonly<{mirror?:Readonly<{directory:string;maximumBytes?:number}>;hostStore?:')
sub(SV, 'createRemoteCacheHandlerV1({store:opened,claims:table,', 'createRemoteCacheHandlerV1({store:opened,claims:table,...(options.mirror?{mirror:createMirrorHandlerV1(options.mirror)}:{}),')
s2 = open(SV, encoding="utf8").read(); first2 = s2.split("\n", 1)[0]
sub(SV, first2 + "\n", first2 + "\n" + f'import {{createMirrorHandlerV1}} from "../{e(0x1FA9E, "mirror")}/{TS}";\n')

# ---- command: serve flags
CM = RC + "/" + e(0x2328, "command") + "/" + TS
sub(CM, 'serve:{valued:["--port","--host","--directory","--max-bytes","--grace-seconds","--tls-certificate","--tls-key","--lang"],plain:["--read-only"]},', 'serve:{valued:["--port","--host","--directory","--max-bytes","--grace-seconds","--tls-certificate","--tls-key","--lang","--mirror-directory"],plain:["--read-only","--no-mirror"]},')
sub(CM, '  tokens,readOnly,\n', '  tokens,readOnly,\n  ...(parsed.flags.has("--no-mirror")||(plan===undefined&&flags["--mirror-directory"]===undefined)?{}:{mirror:{directory:where(context.root,flags["--mirror-directory"]??join(directory,"mirror"))}}),\n')

for item in log: print(item[0][-50:], item[1][:8], "->", item[2][:8])
