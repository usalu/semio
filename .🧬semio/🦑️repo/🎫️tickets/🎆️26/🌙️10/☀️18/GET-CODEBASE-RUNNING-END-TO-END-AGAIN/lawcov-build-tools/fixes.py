"""🔧 Authoring aid: writes the anchored production edits (old text, new text, marker) of the six defect fixes into staged/fixes/<group>.json, reading the exact old text from the repository files so an edit that does not match refuses here instead of at apply time."""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
FE = "\ufe0f"
LIB = "/Users/ueli/Documents/semio/🧰" + FE + "framework/🛍" + FE + "products/🦑" + FE + "repo/🔨" + FE + "modules/📚" + FE + "library"
groups = {"main": [], "cache-gc": []}


def real(rel):
    cur = LIB
    parts = rel.split("/")
    for part in parts:
        hit = [e for e in os.listdir(cur) if e.replace(FE, "") == part.replace(FE, "")]
        if not hit:
            raise SystemExit("unresolved " + rel)
        cur = os.path.join(cur, hit[0])
    return cur


def edit(group, rel, old, new, marker, also=()):
    path = real(rel)
    text = open(path, encoding="utf8").read()
    if text.count(old) != 1:
        raise SystemExit("anchor %d times in %s: %s" % (text.count(old), rel, old[:60]))
    if marker not in new or marker in old:
        raise SystemExit("bad marker for " + rel)
    groups[group].append({"file": os.path.relpath(path, LIB), "old": old, "new": new, "marker": marker, "also": [os.path.relpath(real(a), LIB) for a in also]})


IDENTITY = "⚡caching/🚀bootstrap/🛠tools/📦sdk/🪪identity/🟦.ts"
edit("main", IDENTITY,
     '  const hash = createHash("sha256").update(`nx-tooling-install-v3\\0${environment.platform}\\0${environment.architecture}\\0${environment.libc}\\0`);\n  for (const [name, bytes] of files) hash.update(name + "\\0").update(bytes).update("\\0");\n  return hash.digest("hex");',
     '  const hash = createHash("sha256").update("nx-tooling-install-v4\\0"), field = (value: Uint8Array | string) => { const bytes = typeof value === "string" ? Buffer.from(value) : value; hash.update(`${bytes.byteLength}:`).update(bytes); };\n  for (const value of [environment.platform, environment.architecture, environment.libc]) field(value);\n  for (const [name, bytes] of files) { field(name); field(bytes); }\n  return hash.digest("hex");',
     "nx-tooling-install-v4")
edit("main", "⚡caching/🧪tests/🎚build-mode/🟦.ts",
     '  const toolingOracle = new Bun.CryptoHasher("sha256").update(`nx-tooling-install-v3\\0${toolingIdentity.environment.platform}\\0${toolingIdentity.environment.architecture}\\0${toolingIdentity.environment.libc}\\0`);\n  for (const [path, bytes] of toolingFiles) toolingOracle.update(path + "\\0").update(bytes).update("\\0");',
     '  const toolingOracle = new Bun.CryptoHasher("sha256").update("nx-tooling-install-v4\\0"), frame = (value: Uint8Array | string) => { const bytes = typeof value === "string" ? Buffer.from(value) : value; toolingOracle.update(`${bytes.byteLength}:`).update(bytes); };\n  for (const value of [toolingIdentity.environment.platform, toolingIdentity.environment.architecture, toolingIdentity.environment.libc]) frame(value);\n  for (const [path, bytes] of toolingFiles) { frame(path); frame(bytes); }',
     "nx-tooling-install-v4")
edit("main", "⚡caching/🚀bootstrap/🛠tools/🔏authorizers/🧪tests/🟦.ts",
     'independent=new Bun.CryptoHasher("sha256").update("nx-tooling-install-v3\\0neutral\\0neutral\\0\\0");for(const[path,bytes]of files)independent.update(path+"\\0").update(bytes).update("\\0");',
     'independent=new Bun.CryptoHasher("sha256").update("nx-tooling-install-v4\\0").update("7:neutral7:neutral0:");for(const[path,bytes]of files)independent.update(Buffer.byteLength(path)+":"+path+bytes.byteLength+":").update(bytes);',
     "nx-tooling-install-v4")
edit("main", "⚡caching/🚀bootstrap/🛠tools/📦sdk/📊material/🧪tests/🟦.ts",
     'oracle=new Bun.CryptoHasher("sha256").update("nx-tooling-install-v3\\0darwin\\0arm64\\0\\0");for(const[path,bytes]of files)oracle.update(path+"\\0").update(bytes).update("\\0");',
     'oracle=new Bun.CryptoHasher("sha256").update("nx-tooling-install-v4\\0").update("6:darwin5:arm640:");for(const[path,bytes]of files)oracle.update(Buffer.byteLength(path)+":"+path+bytes.byteLength+":").update(bytes);',
     "nx-tooling-install-v4")
POOL = "⚡caching/💾storage/🔒enforcement/🧵session/🏊pool/🟦.ts"
edit("main", POOL, "inflight=0,idle:unknown,opened=0,closed=0;\n", "inflight=0,idle:unknown,opened=0,closed=0;const dropped=new WeakSet<object>();\n", "const dropped=new WeakSet")
edit("main", POOL, "const drop=async(entry:NonNullable<typeof current>)=>{if(current===entry)current=undefined;try{",
     "const drop=async(entry:NonNullable<typeof current>)=>{if(current===entry)current=undefined;if(dropped.has(entry))return;dropped.add(entry);try{", "if(dropped.has(entry))return;")
edit("main", "⚡caching/🚀bootstrap/🛠tools/🔏authorizers/📦snapshot/🧬schema/🔣.json",
     '"maxLength": 5592408,\n            "pattern": "^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$"',
     '"maxLength": 5592408,\n            "pattern": "^[A-Za-z0-9+/]*={0,2}$"', '"^[A-Za-z0-9+/]*={0,2}$"', also=["⚡caching/🚀bootstrap/🛠tools/🔏authorizers/📦snapshot/🟦.ts"])
edit("main", "⚡caching/🚀bootstrap/🛠tools/🩹patch/🟦.ts",
     'else if(row===""&&before===oldCount&&after===newCount)continue;',
     'else if(row===""&&(index+1===lines.length||before===oldCount&&after===newCount))continue;', "index+1===lines.length||before")
RUNNER = "🏗build/🧹cache-gc/🏃runner/🟦.ts"
edit("cache-gc", RUNNER,
     'try{names=await readdir(prefix?join(base,...prefix.split("/")):base);}catch{continue;}',
     'try{names=await readdir(prefix?join(base,...prefix.split("/")):base);}catch(error){if(prefix&&(error as NodeJS.ErrnoException).code==="ENOTDIR"){const path=root+":"+prefix;throw Error(cacheTextV1("en","error.container",{path})+"\\n"+cacheTextV1("de","error.container",{path}));}continue;}',
     'error.container', also=["⚡caching/💾storage/🧾ledger/🟦.ts"])
COMMAND = "🏗build/🧹cache-gc/⌨command/🟦.ts"
edit("cache-gc", COMMAND, 'only=values["--area"]?new Set(values["--area"].split(",").filter(Boolean)):undefined;', 'only=values["--area"]?new Set(values["--area"].split(",")):undefined;', 'new Set(values["--area"].split(","))')
edit("cache-gc", COMMAND, 'for(const id of only??[])if(!ids.has(id))throw Error(both("error.area",{id}));', 'for(const id of only??[]){if(!id)throw Error(both("error.areaEmpty"));if(!ids.has(id))throw Error(both("error.area",{id}));}', 'both("error.areaEmpty")')
TEXTS = "⚡caching/💾storage/🧾ledger/📖texts/🔣.json"
path = real(TEXTS)
line = [l for l in open(path, encoding="utf8").read().split("\n") if '"error.area":' in l][0]
edit("cache-gc", TEXTS, line + "\n",
     line + '\n    "error.areaEmpty": { "en": "Empty entry in the --area list", "de": "Leerer Eintrag in der --area-Liste" },\n    "error.container": { "en": "Expected a directory at {path}: a file stands where declared cache homes need a container", "de": "Erwartet wird ein Verzeichnis bei {path}: eine Datei steht dort, wo deklarierte Cache-Bereiche einen Container brauchen" },\n', '"error.areaEmpty"', also=["⚡caching/💾storage/🧾ledger/🟦.ts"])
ACQ = "⚡caching/🚀bootstrap/🛠tools/📦sdk/📥source/📥acquisition/🟦.ts"
edit("main", ACQ,
     '/** 📥️ Authenticates the locked registry tarball before bounded package decoding. */\nexport async function acquireNxPackageSourceV1(workspace:string,original:ScriptInvocation,lock:unknown,phaseControl:ResourceLeaseControl):Promise<NxPackageArchiveV1>{\n requireIssuedResourceControlInvocationV1(phaseControl,original);checkResourceLeaseControl(phaseControl);',
     '/** 🔌️ Everything acquisition needs from its host: the issued-control admission, the caller authority, a derived control and the network; tests replace them through an explicit parameter. */\nexport type NxPackageSourceHostV1=Readonly<{require(phase:ResourceLeaseControl,original:ScriptInvocation):void;authority(original:ScriptInvocation):Readonly<{repositoryRoot:string;network:Readonly<{offline:boolean}>;child:Readonly<{maximumElapsedMilliseconds:number}>}>;derive(phase:ResourceLeaseControl,milliseconds:number):ResourceLeaseControl;fetch:typeof fetch}>;\n/** 🌐️ The real host: issued controls, the native caller authority and the global fetch. */\nexport const NX_PACKAGE_SOURCE_HOST_V1:NxPackageSourceHostV1=Object.freeze({require:requireIssuedResourceControlInvocationV1,authority:original=>readNativeCallerAuthorityV1(original.capabilities),derive:createRequestedIssuedResourceControlV1,fetch:(input,init)=>fetch(input,init)});\n/** 📥️ Authenticates the locked registry tarball before bounded package decoding. */\nexport async function acquireNxPackageSourceV1(workspace:string,original:ScriptInvocation,lock:unknown,phaseControl:ResourceLeaseControl,host:NxPackageSourceHostV1=NX_PACKAGE_SOURCE_HOST_V1):Promise<NxPackageArchiveV1>{\n host.require(phaseControl,original);checkResourceLeaseControl(phaseControl);',
     "NX_PACKAGE_SOURCE_HOST_V1")
edit("main", ACQ, 'const authority=readNativeCallerAuthorityV1(original.capabilities);', 'const authority=host.authority(original);', "host.authority(original);")
edit("main", ACQ, 'createRequestedIssuedResourceControlV1(phaseControl,authority.child.maximumElapsedMilliseconds)', 'host.derive(phaseControl,authority.child.maximumElapsedMilliseconds)', "host.derive(phaseControl")
edit("main", ACQ, 'if(!response.ok||!response.body)throw Error("Pinned Nx package download refused: "+response.status);', 'if(!response.ok||!response.body){await response.body?.cancel().catch(()=>undefined);throw Error("Pinned Nx package download refused: "+response.status);}', "await response.body?.cancel()")
edit("main", ACQ, 'BigInt(advertised)>BigInt(policy.maximumCompressedBytes)))throw Error("Nx package compressed ceiling exceeded");', 'BigInt(advertised)>BigInt(policy.maximumCompressedBytes))){await response.body.cancel().catch(()=>undefined);throw Error("Nx package compressed ceiling exceeded");}', "await response.body.cancel()")
edit("main", ACQ, 'response=await fetch(', 'response=await host.fetch(', "await host.fetch(")
edit("main", "⚡caching/🚀bootstrap/🛠tools/📦sdk/📥source/🧊pristine/🟦.ts", 'async function restoreNxPackageSourceV1(', 'export async function restoreNxPackageSourceV1(', "export async function restoreNxPackageSourceV1(")

for name, rows in groups.items():
    json.dump(rows, open(os.path.join(HERE, "staged", "fixes", name + ".json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print(name, len(rows))
