#!/usr/bin/env python3
"""H55 landing patch: output-determinism contract and Cargo identity in the unit key, module-remap outcome. usage: apply.py <library root> [--check]
Every edit is anchored and idempotent-refusing: an anchor that is missing or twice present aborts before anything is written. Files are written atomically (temp + rename)."""
import json, os, sys, hashlib

root = sys.argv[1]
check = "--check" in sys.argv
U = os.path.join(root, "⚡️caching", "🦀️cargo", "🗃️unit-cache")
P = os.path.join(root, "🏗️build", "🌐️remote-execution", "🚀️offload", "🗺️paths", "🟦️.ts")
files = {}

def load(path):
    if path not in files:
        files[path] = open(path, encoding="utf8").read()
    return files[path]

def once(path, old, new):
    text = load(path)
    if text.count(old) != 1:
        sys.exit("anchor not unique (%d) in %s: %s" % (text.count(old), path, old[:80]))
    files[path] = text.replace(old, new)

def between(path, start, end, new):
    text = load(path)
    a = text.index(start)
    b = text.index(end, a)
    files[path] = text[:a] + new + text[b:]

schema = os.path.join(U, "🧬️schema", "🔣️.json")
once(schema, '''          "extraToken": "@@semio-extra@@",
          "metadataCharacters": 16
''', '''          "extraToken": "@@semio-extra@@",
          "metadataCharacters": 16,
          "outputDeterminism": {
            "version": "2",
            "rules": [
              "front-end-threads=1",
              "dep-info-canonical",
              "streams-canonical",
              "offload-front-end-threads=1",
              "module-remap-or-unshared",
              "cargo-identity-keyed"
            ]
          }
''')
once(schema, '^(toolchain|source|platform|roots|extern:.+|env:.+|args\\\\.[a-z]+)$', '^(toolchain|source|determinism|cargo|platform|roots|extern:.+|env:.+|args\\\\.[a-z]+)$')

key = os.path.join(U, "🔏️key", "🟦️.ts")
between(key, '/** 🔑️ The first-level key: toolchain', '/** 🧩️ Names the parts', '''/** ⚖️ The output-determinism contract the compiler outputs of this code obey (`key.outputDeterminism` of the policy): one front-end thread, canonical dep-info and streams, a module remap that holds or an unshared unit, Cargo's unit identity keyed. Part of every compilation key, so an entry written by code under another contract is never read and never competes with the outputs of this one: raising the version retires all earlier entries at once (they age out), and the first write of a key is a write under the current contract.
 * @see ../🧬️schema/🔣️.json */
export const OUTPUT_DETERMINISM_V1:Readonly<{version:string;rules:readonly string[]}>=policy.key.outputDeterminism;

/** 🔑️ The first-level key: toolchain, normalized command line, root source bytes, dependency artifact bytes, selected environment and, for a compilation (never for a build script run, whose outputs do not depend on a compiler front end), the output-determinism contract version and Cargo's own unit identity (`cargoIdentity`, the `-C extra-filename` of the command line: it is embedded in the metadata of the output, so two Cargo identities are two outputs). */
export function unitKeyV1(input:Readonly<{toolchain:string;normalized:readonly string[];sourceSha256:string;externs:readonly Readonly<{name:string;sha256:string}>[];environment:readonly (readonly [string,string])[];cargoIdentity?:string}>):string{
 const compiled=!input.sourceSha256.startsWith("run:");
 return sha256V1(JSON.stringify([policy.namespace,input.toolchain,input.normalized,input.sourceSha256,input.externs.map(row=>[row.name,row.sha256]),input.environment,...compiled?[["determinism",OUTPUT_DETERMINISM_V1.version],...input.cargoIdentity?[["cargo",input.cargoIdentity]]:[]]:[]]));
}

''')
once(key, '"environment:"+part(input.environment)].join(" ");', '"environment:"+part(input.environment),"determinism:"+OUTPUT_DETERMINISM_V1.version,"cargo:"+part(input.cargoIdentity??"")].join(" ");')
once(key, ' components.toolchain=part(input.toolchain);components.source=input.sourceSha256.slice(0,width);\n', ' components.toolchain=part(input.toolchain);components.source=input.sourceSha256.slice(0,width);\n if(!input.sourceSha256.startsWith("run:")){components.determinism=part(OUTPUT_DETERMINISM_V1.version);if(input.cargoIdentity)components.cargo=part(input.cargoIdentity);}\n')
once(key, '`toolchain`, `source`, `platform`, `roots`,', '`toolchain`, `source`, `determinism`, `cargo`, `platform`, `roots`,')

script = os.path.join(U, "📜️script.ts")
once(script, 'let fixed:Pick<Parameters<typeof unitKeyV1>[0],"toolchain"|"normalized"|"externs">|undefined;', 'let fixed:Pick<Parameters<typeof unitKeyV1>[0],"toolchain"|"normalized"|"externs"|"cargoIdentity">|undefined;')
once(script, 'normalized:normalizeRustcArgumentsV1(args,digests,settings.laneRoot),externs:', 'normalized:normalizeRustcArgumentsV1(args,digests,settings.laneRoot),cargoIdentity:extraFilenameV1(args),externs:')
between(script, ' const moduleRemapped=async()', ' argv=(await moduleRemapped())??argv;\n', ''' let moduleDeclined:string|undefined;
 const moduleRemapped=async():Promise<readonly string[]|undefined>=>{
  const manifest=environment.CARGO_MANIFEST_DIR,workspace=environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||checkoutRootOfScriptV1();
  if(manifest===undefined||!(cwd===manifest||cwd.startsWith(manifest+"/")||cwd.startsWith(manifest+"\\\\")))return undefined;
  try{
   const outcome=(await import("../../../🏗️build/🌐️remote-execution/🚀️offload/🗺️paths/🟦️.ts")).moduleRemapV1(expandRustcArgumentsV1(argv,cwd),environment,cwd,workspace);
   if("declined" in outcome){moduleDeclined=outcome.declined;return undefined;}
   return"arguments" in outcome?outcome.arguments:undefined;
  }catch{moduleDeclined="error";return undefined;}
 };
''')
once(script, '/** 🛠️ Serves one rustc command line from the cache', '''/** 🏠️ The checkout this wrapper script lives in (eight directories above its folder, confirmed by `.cargo/config.toml`), `undefined` when the script is elsewhere: the workspace root of a unit whose launcher did not export `SEMIO_UNIT_CACHE_WORKSPACE_ROOT`, so the module remap of a unit does not depend on how its Cargo process was started. */
function checkoutRootOfScriptV1():string|undefined{
 try{const root=resolve(import.meta.dir,"../../../../../../../..");return existsSync(join(root,".cargo","config.toml"))?root:undefined;}catch{return undefined;}
}

/** 🛠️ Serves one rustc command line from the cache''')
once(script, 'basics=unitBasicsV1({root:settings.root,environment,inner:settings.inner,unit,args,cwd}),prepare=(paths:readonly string[],localKey=key)=>basics?prepareFleetV1(', 'basics=moduleDeclined===undefined?unitBasicsV1({root:settings.root,environment,inner:settings.inner,unit,args,cwd}):undefined,prepare=(paths:readonly string[],localKey=key)=>basics?prepareFleetV1(')
once(script, ' if(!fleet){const why=fleetUnidentifiedReasonV1(settings.root,environment);if(why!==undefined){', ' if(!fleet&&moduleDeclined!==undefined){if(environment.SEMIO_REMOTE_CACHE_ENDPOINT){event("remote-skip",key,0,0,"module-remap-declined: the code lies above the package directory and the remap of its directory was declined ("+moduleDeclined+"), so its recorded names differ from the fleet\'s and the unit is not shared");countSkipV1(workspaceOf,"module-remap-declined");countFallbackV1(settings.root,"module-remap-declined");}}\n else if(!fleet){const why=fleetUnidentifiedReasonV1(settings.root,environment);if(why!==undefined){')

once(P, 'import {existsSync,readFileSync,readdirSync,statSync} from "node:fs";', 'import {existsSync,readFileSync,readdirSync,realpathSync,statSync} from "node:fs";')
between(P, '/** 🧭️ The compiler arguments of a unit that Cargo compiles in its package directory', 'const SEARCH_KINDS', '''/** 🧭️ What the module remap of one unit comes to: its `arguments` (the unit's code lives above its manifest and a remap of its own was added), `needless` (the code lies inside the package directory, or an existing remap names it already, so every machine records the same names), or `declined` with the guard that stopped it (`no-workspace`, `reached-files`, `shallow`, `outside-workspace`). A declined unit records the raw package-relative spelling of its sources: another outcome than the one other machines reach, so it is never shared. */
export type ModuleRemapV1=Readonly<{arguments:readonly string[]}|{needless:true}|{declined:"no-workspace"|"reached-files"|"shallow"|"outside-workspace"}>;

/** 🧭️ The compiler arguments of a unit that Cargo compiles in its package directory while its code lives in the module directory above the manifest (a one-line shim includes it), when no remap covers that directory: the directory gets a remap of its own, named by its path below the checkout, placed before Cargo's remaps (the last matching one wins, so the package directory keeps Cargo's name), and a relative source becomes absolute so that every name the compiler records is remapped. A local compile and an offloaded one then record the same names and give the same crate hash on every machine and at every checkout path. A workspace spelled through a link (`/tmp` and `/private/tmp`) names the same directory as the manifest does. The outcome says why a unit got none, never silently. */
export function moduleRemapV1(args:readonly string[],environment:Environment,cwd:string,workspace:string|undefined):ModuleRemapV1{
 const manifest=environment.CARGO_MANIFEST_DIR,pass=scan(args);
 if(manifest===undefined||!isAbsolute(manifest)||"declined" in pass||pass.source===undefined||!within(manifest,cwd))return{needless:true};
 const sourceReal=resolve(cwd,pass.source),reached=reachedFilesV1(sourceReal,undefined,REACHED_FILES,manifest);
 if(reached===undefined)return{declined:"reached-files"};
 const shipped=[sourceReal,...reached].reduce((root,path)=>within(root,path)?root:commonAncestor(root,path),manifest);
 if(within(cwd,shipped)||shipped===manifest)return{needless:true};
 if(workspace===undefined)return{declined:"no-workspace"};
 if(depthOf(shipped)<MINIMUM_SHIPPED_DEPTH)return{declined:"shallow"};
 const base=within(workspace,shipped)?workspace:(()=>{try{const real=realpathSync(workspace);return within(real,shipped)?real:undefined;}catch{return undefined;}})();
 if(base===undefined)return{declined:"outside-workspace"};
 if(remappedNameV1(remapsOfV1(args),shipped)!==shipped)return{needless:true};
 const first=args.findIndex(argument=>argument==="--remap-path-prefix"||argument.startsWith("--remap-path-prefix=")),rewritten=args.map(argument=>argument===pass.source&&!isAbsolute(argument)?sourceReal:argument),at=first<0?rewritten.length:first;
 return{arguments:Object.freeze([...rewritten.slice(0,at),`--remap-path-prefix=${shipped}=${shipped===base?".":relative(base,shipped).split(sep).join("/")}`,...rewritten.slice(at)])};
}

/** 🧭️ The arguments of {@link moduleRemapV1}, `undefined` when the unit needs none or got none. */
export function moduleRemapArgumentsV1(args:readonly string[],environment:Environment,cwd:string,workspace:string|undefined):readonly string[]|undefined{
 const outcome=moduleRemapV1(args,environment,cwd,workspace);
 return"arguments" in outcome?outcome.arguments:undefined;
}

''')

T = os.path.join(U, "🔏️key", "🧪️tests")
oracle = os.path.join(T, "🐍️oracle", "🐍️.py")
once(oracle, '''    print(json.dumps([hashlib.sha256(dumps([policy["namespace"], k["toolchain"], k["normalized"], k["sourceSha256"], [[e["name"], e["sha256"]] for e in k["externs"]], k["environment"]]).encode("utf8")).hexdigest() for k in request["rows"]]))''', '''    def contract(k):
        if k["sourceSha256"].startswith("run:"):
            return []
        return [["determinism", policy["key"]["outputDeterminism"]["version"]]] + ([["cargo", k["cargoIdentity"]]] if k.get("cargoIdentity") else [])
    print(json.dumps([hashlib.sha256(dumps([policy["namespace"], k["toolchain"], k["normalized"], k["sourceSha256"], [[e["name"], e["sha256"]] for e in k["externs"]], k["environment"]] + contract(k)).encode("utf8")).hexdigest() for k in request["rows"]]))''')
tkey = os.path.join(T, "🟦️.ts")
once(tkey, ''' expect(new Set([unitKeyV1(first as never),...variants.map(row=>unitKeyV1(row as never))]).size).toBe(6);
});
''', ''' expect(new Set([unitKeyV1(first as never),...variants.map(row=>unitKeyV1(row as never))]).size).toBe(6);
});

test("the output-determinism contract and Cargo's unit identity are key inputs of a compilation, and of nothing else",()=>{
 const unit={toolchain:"t",normalized:["--crate-name","a"],sourceSha256:"a".repeat(64),externs:[],environment:[]},run={...unit,sourceSha256:"run:"+"b".repeat(64)};
 const keyOf=(row:object)=>ask("keys",{rows:[row]})[0] as string;
 expect(unitKeyV1(unit)).toBe(keyOf(unit));
 expect(unitKeyV1({...unit,cargoIdentity:"-3c9971acb8b0a698"})).toBe(keyOf({...unit,cargoIdentity:"-3c9971acb8b0a698"}));
 expect(new Set([unitKeyV1(unit),unitKeyV1({...unit,cargoIdentity:"-3c9971acb8b0a698"}),unitKeyV1({...unit,cargoIdentity:"-4fe346c9fadf2a8a"})]).size).toBe(3);
 expect(unitKeyV1({...run,cargoIdentity:"-3c9971acb8b0a698"})).toBe(unitKeyV1(run));
 expect(unitKeyV1(run)).toBe(sha256V1(JSON.stringify([UNIT_CACHE_POLICY_V1.namespace,run.toolchain,run.normalized,run.sourceSha256,[],[]])));
 expect(unitKeyV1(unit)).not.toBe(sha256V1(JSON.stringify([UNIT_CACHE_POLICY_V1.namespace,unit.toolchain,unit.normalized,unit.sourceSha256,[],[]])));
});

test("the key changes exactly when the version of the output-determinism contract changes, and the version names the rules it stands for",()=>{
 const contract=UNIT_CACHE_POLICY_V1.key.outputDeterminism,unit={toolchain:"t",normalized:["--crate-name","a"],sourceSha256:"a".repeat(64),externs:[],environment:[]};
 const under=(version:string)=>sha256V1(JSON.stringify([UNIT_CACHE_POLICY_V1.namespace,unit.toolchain,unit.normalized,unit.sourceSha256,[],[],["determinism",version]]));
 expect(unitKeyV1(unit)).toBe(under(contract.version));
 expect(under(String(Number(contract.version)+1))).not.toBe(unitKeyV1(unit));
 const pinned:Record<string,string>={"2":"dd945055956faeb7"};
 expect(sha256V1(JSON.stringify(contract.rules)).slice(0,16),"the rules of a contract changed without a new version: raise key.outputDeterminism.version in the schema and pin its rules here").toBe(pinned[contract.version]);
});
''')

pp = os.path.join(T, "🧩️parts")
once(os.path.join(pp, "🐍️.py"), '''        "environment:" + part(input["environment"]),
    ])''', '''        "environment:" + part(input["environment"]),
        "determinism:" + fixture["determinism"],
        "cargo:" + part(input.get("cargoIdentity", "")),
    ])''')
once(os.path.join(pp, "🟦️.ts"), 'import{unitKeyPartsV1,unitKeyV1}from"../../🟦️.ts";', 'import{unitKeyPartsV1,unitKeyV1,UNIT_CACHE_POLICY_V1}from"../../🟦️.ts";')
once(os.path.join(pp, "🟦️.ts"), 'writeFileSync(file,JSON.stringify(fixture));', 'writeFileSync(file,JSON.stringify({...fixture,determinism:UNIT_CACHE_POLICY_V1.key.outputDeterminism.version}));')
once(os.path.join(pp, "🔣️.json"), '''"SDKROOT", "/other"]]}}
  ]''', '''"SDKROOT", "/other"]]}},
    {"name": "cargo identity", "changes": "cargo", "set": {"cargoIdentity": "-3c9971acb8b0a698"}}
  ]''')

cc = os.path.join(T, "🧬️components")
once(os.path.join(cc, "🐍️.py"), '''    if "platform" in extra:''', '''    if not input["sourceSha256"].startswith("run:"):
        rows["determinism"] = part(fixture["determinism"])
        if input.get("cargoIdentity"): rows["cargo"] = part(input["cargoIdentity"])
    if "platform" in extra:''')
once(os.path.join(cc, "🟦️.ts"), 'import{argumentClassesV1,differingComponentsV1,unitKeyComponentsV1,unitKeyV1}from"../../🟦️.ts";', 'import{argumentClassesV1,differingComponentsV1,unitKeyComponentsV1,unitKeyV1,UNIT_CACHE_POLICY_V1}from"../../🟦️.ts";')
once(os.path.join(cc, "🟦️.ts"), 'writeFileSync(file,JSON.stringify(fixture));', 'writeFileSync(file,JSON.stringify({...fixture,determinism:UNIT_CACHE_POLICY_V1.key.outputDeterminism.version}));')
once(os.path.join(cc, "🔣️.json"), '''    {
      "name": "platform class",''', '''    {
      "name": "cargo identity",
      "changes": [
        "cargo"
      ],
      "set": {
        "cargoIdentity": "-3c9971acb8b0a698"
      }
    },
    {
      "name": "platform class",''')

extra = os.path.join(os.path.dirname(os.path.abspath(__file__)), "extra-edits.py")
if os.path.exists(extra):
    exec(open(extra, encoding="utf8").read())

for path, text in files.items():
    if path.endswith(".json"):
        json.loads(text)
if check:
    print("ok, %d files" % len(files))
    sys.exit(0)
for path, text in files.items():
    temporary = path + ".h55.tmp"
    open(temporary, "w", encoding="utf8").write(text)
    os.replace(temporary, path)
    print("wrote", os.path.relpath(path, root), hashlib.sha256(text.encode("utf8")).hexdigest()[:12])
