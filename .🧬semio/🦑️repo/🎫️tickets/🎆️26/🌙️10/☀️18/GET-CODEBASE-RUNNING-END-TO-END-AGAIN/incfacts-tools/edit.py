"""🧲️ [incfacts] Anchored edits of batch `libfacts`: library files leave project records and become facts of the load authority.
Every anchor must occur exactly once in its file (or exactly the declared number of times), else the script refuses without writing. usage: python3 -I edit.py <root>"""
import os, sys

ROOT = os.path.abspath(sys.argv[1])
INF = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/"
ASSEMBLY = INF + "🧱️project-assembly/"

EDITS = {
    ASSEMBLY + "🟨️.mjs": [
        ('import { memoizedObservedComputationV1, observedSourceAnalysisV1, observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";',
         'import { observedSourceAnalysisV1, observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";'),
        ('import { dirname, isAbsolute, join, relative, resolve } from "node:path";\n', 'import { dirname, isAbsolute, join, relative, resolve } from "node:path";\nimport { fileURLToPath } from "node:url";\n'),
        ('const PRECISION_FILES = ["🏗️build/📥️inputs/🎯️precision/🟦️.ts", "🏗️build/📥️inputs/🎯️precision/🧬️schema/🔣️.json"];\nconst PRECISION_RULES = "🏗️build/📥️inputs/🎯️precision/🔣️.json";\n', 'const PRECISION_RULES = join(resolve(dirname(fileURLToPath(import.meta.url)), "../../../.."), "🏗️build/📥️inputs/🎯️precision/🔣️.json");\n'),
        ('const precisionDocumentsV1 = new Map();\n/** 📏️ The input precision rules as every project that applies them records them: read once per process, and the facts of that read (the engine, its schema and the rules) are replayed into each receiving project, so the rows of a project never depend on which project of the same process asked first (a farm worker infers in a fresh process). */\nexport function readProjectInputPrecisionV1() {\n  return memoizedObservedComputationV1(precisionDocumentsV1, PRECISION_RULES, () => { for (const file of PRECISION_FILES) readFileSync(join(LIBRARY_ROOT, file)); const rules = join(LIBRARY_ROOT, PRECISION_RULES); return existsSync(rules) ? readInputPrecisionV1(JSON.parse(readFileSync(rules, "utf8"))) : null; }) ?? undefined;\n}\n',
         '/** 📥️ Admits the rules file of the loaded implementation, or keeps its refusal for the first project that applies it. */\nfunction readInputPrecisionAuthorityV1() {\n  try { return existsSync(PRECISION_RULES) ? readInputPrecisionV1(JSON.parse(readFileSync(PRECISION_RULES, "utf8"))) : undefined; }\n  catch (error) { return error; }\n}\nconst precisionDocumentV1 = readInputPrecisionAuthorityV1();\n\n/** 📏️ The input precision rules of the loaded implementation, read when this module loads: the implementation revision lists the rules, their schema and the engine, so they are facts of the load authority that every project record inherits by digest (the engine by the stamp of this inferrer), never rows of a project record, and a farm worker sees exactly what its parent sees; an inadmissible rules file refuses where a project applies it; undefined without a rules file.\n * @see ../♻️revision/🟨️.mjs\n * @see ../🔖️stamps/🧬️schema/🔣️.json */\nexport function loadedInputPrecisionV1() {\n  if (precisionDocumentV1 instanceof Error) throw precisionDocumentV1;\n  return precisionDocumentV1;\n}\n'),
        ('  const document = readProjectInputPrecisionV1(), known = precisionContexts.get(scripts);',
         '  const document = loadedInputPrecisionV1(), known = precisionContexts.get(scripts);'),
        ('  let compiler, kinds, manifests;\n', '  let compiler, manifests;\n'),
        ('    get kinds() { return kinds ??= JSON.parse(readFileSync(join(LIBRARY_ROOT, "🔣️taxonomy.json"), "utf8")).fileKinds; },',
         '    get kinds() { return TAXONOMY.fileKinds; },'),
    ],
    ASSEMBLY + "🟨️.d.mts": [
        ('/** 📏️ The input precision rules, read once per process with their facts replayed into every receiving project; undefined without a rules file. */\nexport function readProjectInputPrecisionV1():',
         '/** 📏️ The input precision rules of the loaded implementation, a fact of the load authority and never a row of a project record; undefined without a rules file. */\nexport function loadedInputPrecisionV1():'),
    ],
    INF + "🧭️script-inputs/🟨️.mjs": [
        ('let buildPath, readPhysicalSource, requirePhysicalSourceV1, prepareSourceAnalysesV1, revisionIdentity, register, PROVIDER_PORTS, LIBRARY_ROOT, POLICY, readToml,',
         'let buildPath, readPhysicalSource, requirePhysicalSourceV1, prepareSourceAnalysesV1, revisionIdentity, register, PROVIDER_PORTS, PRESENTATION_RESOURCES, LIBRARY_ROOT, POLICY, WORKSPACE_POLICY, readToml,'),
        ('  ({ buildPath, readPhysicalSource, requirePhysicalSourceV1, LIBRARY_ROOT, POLICY, SCRIPT_PROVIDER_DESIGNATION, SCRIPT_PROVIDER_DESIGNATION_SCHEMA } = registry.coordinates);\n',
         '  ({ buildPath, readPhysicalSource, requirePhysicalSourceV1, LIBRARY_ROOT, POLICY, WORKSPACE_POLICY, SCRIPT_PROVIDER_DESIGNATION, SCRIPT_PROVIDER_DESIGNATION_SCHEMA } = registry.coordinates);\n  PRESENTATION_RESOURCES = new Set(WORKSPACE_POLICY.presentationResources);\n'),
        ('    if(presentationResourcesV1(cache).has(buildPath(relative(workspaceRoot,path))))return;',
         '    if(PRESENTATION_RESOURCES.has(buildPath(relative(workspaceRoot,path))))return;'),
        ('/** 🙈️ The resources the build policy declares presentation-only (progress labels): their edits never change a command\'s behavior, so they are not command inputs. The policy is read once per script input cache and the facts of that read are replayed into every receiving project, so the rows of a project never depend on which project of the same process asked first. */\nfunction presentationResourcesV1(cache) {\n  return memoizedObservedComputationV1(cache.presentation ??= new Map(), "presentation", () => new Set(JSON.parse(readPhysicalSource(join(LIBRARY_ROOT, "🏗️build", "🔣️policy.json")).toString("utf8")).presentationResources));\n}\n',
         ''),
    ],
    INF + "🧬️source-analyses/🟨️.mjs": [
        ('/** 🗺️ The only taxonomy facts graph inference consumes, so edits elsewhere in the taxonomy keep every inference record and parser revision valid. */\nfunction taxonomyProjectionV1(taxonomy) {\n  return { testDeliveryScopeDirectoryNames: taxonomy.testDeliveryScopeDirectoryNames, generatorContracts: taxonomy.generatorContracts ?? {} };',
         '/** 🗺️ The only taxonomy facts graph inference consumes (test delivery scopes, generator contracts and the file kinds the input precision rules read), so edits elsewhere in the taxonomy keep every inference record and parser revision valid. */\nfunction taxonomyProjectionV1(taxonomy) {\n  return { testDeliveryScopeDirectoryNames: taxonomy.testDeliveryScopeDirectoryNames, generatorContracts: taxonomy.generatorContracts ?? {}, fileKinds: taxonomy.fileKinds ?? {} };'),
    ],
    INF + "♻️revision/🟨️.mjs": [
        ('"🏗️build/📥️inputs/🎯️precision/🧬️schema/🔣️.json", ',
         '"🏗️build/📥️inputs/🎯️precision/🟦️.ts", "🏗️build/📥️inputs/🎯️precision/🔣️.json", "🏗️build/📥️inputs/🎯️precision/🧬️schema/🔣️.json", '),
    ],
    INF + "🔖️stamps/🧬️schema/🔣️.json": [
        ('"stamp": "semio.graph-inferrer.source-analyses/1",', '"stamp": "semio.graph-inferrer.source-analyses/2",'),
        ('      "sources": [\n       "🏗️build/🕸️graph/📥️inference/🧱️project-assembly/"\n      ]',
         '      "sources": [\n       "🏗️build/🕸️graph/📥️inference/🧱️project-assembly/",\n       "🏗️build/📥️inputs/🎯️precision/🟦️.ts"\n      ]'),
    ],
    INF + "🧪️tests/🔖️output-stamps/🔣️.json": [
        ('"semio.graph-inferrer.source-analyses/1": {', '"semio.graph-inferrer.source-analyses/2": {'),
    ],
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts": [
        ('"presentationResourcesV1"', '"PRESENTATION_RESOURCES"'),
        ('presentationResourcesV1:()=>new Set()', 'PRESENTATION_RESOURCES:new Set()', 4),
        ('observedFs:{existsSync:virtualExists,readFileSync},readCargoResolutionPolicyV1,resolutionDocument,withHostPlatformOutputsV1:(value:unknown)=>value},true)',
         'observedFs:{existsSync:virtualExists,readFileSync},fileURLToPath:require("node:url").fileURLToPath,readCargoResolutionPolicyV1,resolutionDocument,withHostPlatformOutputsV1:(value:unknown)=>value},true)'),
    ],
}

LAW = ASSEMBLY + "🧪️tests/🟦️.ts"
LAW_TEXT = '''/** 🎯️ Law: library files are facts of the load authority, never rows of a project record. The input precision rules and their schema are read when the project assembly loads and listed by the implementation revision (the engine by the stamp of the project assembly), the file kinds the rules read come from the taxonomy projection and the presentation resources from the loaded build policy; two projects of one process therefore apply the same rules without recording a library file, whichever asked first, exactly as a farm worker in a fresh process. Independent oracles: the sha256 and the parsed taxonomy kinds Python computes from the files. */
import{expect,test}from"bun:test";
import{dirname,join,resolve,sep}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{captureObservedSourceInputsV1,observeDetachedV1}from"../../../../👁️observation/🟨️.mjs";
import{classifyImplementationRowsV1,createScriptInputCache,libraryBootstrap,loadedImplementationRowsV1}from"../../🟨️.mjs";
import{TAXONOMY}from"../../🧬️source-analyses/🟨️.mjs";
import{relativeScriptInputs}from"../../🧭️script-inputs/🟨️.mjs";
import{loadedInputPrecisionV1}from"../🟨️.mjs";
import presentation from"../../🧪️tests/🙈️presentation-resources/🔣️.json";

const here=dirname(fileURLToPath(import.meta.url)),library=resolve(here,"../../../../.."),workspace=resolve(library,"../../../../.."),python=testPythonV1();
const local=(path:string)=>path.split(sep).join("/").slice(workspace.length+1);
const precision=["🏗️build/📥️inputs/🎯️precision/🧬️schema/🔣️.json","🏗️build/📥️inputs/🎯️precision/🔣️.json"].map(path=>join(library,path)),engine=join(library,"🏗️build/📥️inputs/🎯️precision/🟦️.ts");
const ask=(code:string,...args:string[])=>JSON.parse(Bun.spawnSync([python!,"-I","-c",code,...args]).stdout.toString());
const libraryRows=(rows:readonly{kind:string;path:string}[])=>rows.filter(row=>typeof row.path==="string"&&row.path.startsWith(library));

test("two projects of one process apply the precision rules without recording a library file, and the load authority holds the rules and their schema at the independent digests",async()=>{
 await libraryBootstrap;
 const project=async()=>{const{value,rows,layers}=await observeDetachedV1(async()=>loadedInputPrecisionV1());return{value,library:libraryRows([...rows,...layers.flat()] as{kind:string;path:string}[])};};
 const first=await project(),second=await project();
 expect(first.value).toBeDefined();expect(second.value).toBe(first.value);
 expect(first.library).toEqual([]);expect(second.library).toEqual([]);
 const{authorityObservations}=await captureObservedSourceInputsV1(async()=>undefined,undefined,true),expected=ask("import hashlib,json,sys;print(json.dumps([hashlib.sha256(open(p,'rb').read()).hexdigest() for p in sys.argv[1:]]))",...precision);
 expect(precision.map(file=>(authorityObservations as{kind:string;path:string;fact:{sha256?:string}}[]).find(row=>row.kind==="file"&&row.path===file)?.fact?.sha256)).toEqual(expected);
},60000);

test("the implementation revision lists the rules and their schema as content facts of every owner and the engine under a stamp",()=>{
 const rows=loadedImplementationRowsV1.map(row=>({kind:"digest",path:local(row.path),fact:row.fact})),kept=new Set(classifyImplementationRowsV1(rows as never).filter(row=>row.kind!=="stamp").map(row=>row.path));
 for(const file of[...precision,engine])expect(rows.some(row=>row.path===local(file)),local(file)).toBe(true);
 for(const file of precision)expect(kept.has(local(file)),local(file)).toBe(true);
 expect(kept.has(local(engine))).toBe(false);
});

test("the file kinds the rules read are the taxonomy kinds an independent parser reads",async()=>{
 await libraryBootstrap;
 expect(TAXONOMY.fileKinds).toEqual(ask("import json,sys;print(json.dumps(json.load(open(sys.argv[1],encoding='utf-8'))['fileKinds']))",join(library,"🔣️taxonomy.json")));
});

test("a command walk that passes a presentation resource records no fact of the build policy that declares it",async()=>{
 await libraryBootstrap;
 const{value,rows,layers}=await observeDetachedV1(async()=>relativeScriptInputs([join(workspace,presentation.entry)],workspace,createScriptInputCache({workspaceRoot:workspace}) as never,false));
 for(const file of presentation.excluded)expect(value).not.toContain("{workspaceRoot}/"+file);
 expect(libraryRows([...rows,...layers.flat()] as{kind:string;path:string}[]).filter(row=>row.path===join(library,"🏗️build/🔣️policy.json"))).toEqual([]);
},120000);
'''

out = {}
for rel, edits in EDITS.items():
    path = os.path.join(ROOT, rel)
    text = open(path, encoding="utf8").read()
    for old, new, *expected in edits:
        count = text.count(old)
        if count != (expected[0] if expected else 1): sys.exit(f"refused: anchor found {count} times in {rel}: {old[:90]!r}")
        text = text.replace(old, new)
    out[path] = text
law = os.path.join(ROOT, LAW)
if "readProjectInputPrecisionV1" not in open(law, encoding="utf8").read(): sys.exit("refused: unknown shape of " + LAW)
out[law] = LAW_TEXT
for path, text in out.items():
    with open(path, "w", encoding="utf8", newline="") as handle: handle.write(text)
print(f"[incfacts] edited {len(out)} files under {ROOT}")
