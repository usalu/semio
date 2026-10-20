import os, json

E = lambda code: chr(code) + "️"
LIB = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
BUILD = os.path.join(LIB, E(0x1F3D7) + "build")
TEST = os.path.join(BUILD, E(0x1F9EA) + "tests", E(0x1F578) + "graph", E(0x1F3AF) + "source-demand", E(0x1F7E6) + ".ts")
FIXTURE = os.path.join(BUILD, E(0x1F9EA) + "tests", E(0x1F9EB) + "fixtures", E(0x1F578) + "graph", E(0x1F523) + ".json")


def sub(text, old, new, label):
    count = text.count(old)
    if count != 1:
        raise SystemExit("anchor %s found %d times" % (label, count))
    return text.replace(old, new)


EMPTY = '{compile:new Set(),declared:new Set(),command:new Set()}'

CASE_SCHEMA = {
    "type": "object", "additionalProperties": False, "required": ["id", "targets", "generatedDependencies", "refusal"],
    "properties": {
        "id": {"type": "string", "minLength": 1},
        "targets": {"type": "object", "minProperties": 1, "additionalProperties": {"$ref": "#/$defs/roots"}},
        "shared": {"$ref": "#/$defs/roots"},
        "generatedDependencies": {"type": "object", "additionalProperties": {"type": "array", "items": {"type": "string"}}},
        "refusal": {"oneOf": [{"type": "null"}, {"type": "object", "additionalProperties": False, "required": ["target", "importer", "phase"], "properties": {"target": {"type": "string"}, "importer": {"type": "string"}, "phase": {"enum": ["runtime", "compile"]}}}]},
    },
    "$defs": {"roots": {"type": "object", "additionalProperties": False, "properties": {"compile": {"type": "array", "items": {"type": "string"}}, "declared": {"type": "array", "items": {"type": "string"}}, "command": {"type": "array", "items": {"type": "string"}}}}},
}

T = "fixture-shell:typegen"
def case(id, targets, deps, refusal=None, shared=None):
    row = {"id": id, "targets": targets, "generatedDependencies": deps, "refusal": refusal}
    if shared is not None:
        row["shared"] = shared
    return row

CASES = [
    case("sibling-target-consumes-producer-output", {"typegen": {"compile": ["plain.ts"]}, "schema-check": {"compile": ["schema.ts"]}}, {"schema-check": [T]}),
    case("every-consuming-sibling-is-ordered-after-the-producer", {"typegen": {"compile": ["plain.ts"]}, "check": {"compile": ["schema.ts"]}, "test": {"compile": ["schema.ts"]}, "schema-check": {"compile": ["schema.ts"]}}, {"check": [T], "test": [T], "schema-check": [T]}),
    case("producer-consuming-its-own-output-is-a-cycle", {"typegen": {"compile": ["schema.ts"]}, "check": {"compile": ["plain.ts"]}}, {}, {"target": T, "importer": "reducer.ts", "phase": "compile"}),
    case("cycle-is-reported-although-a-sibling-consumes-the-same-closure", {"typegen": {"compile": ["schema.ts"]}, "check": {"compile": ["schema.ts"]}}, {}, {"target": T, "importer": "reducer.ts", "phase": "compile"}),
    case("runtime-command-closure-of-the-producer-is-a-cycle", {"typegen": {"command": ["schema.ts"]}, "check": {"compile": ["plain.ts"]}}, {}, {"target": T, "importer": "reducer.ts", "phase": "runtime"}),
    case("type-only-import-is-erased-at-runtime", {"typegen": {"command": ["typed.ts"]}, "check": {"compile": ["typed.ts"]}}, {"check": [T]}),
    case("type-only-import-is-a-compile-cycle", {"typegen": {"compile": ["typed.ts"]}, "check": {"compile": ["plain.ts"]}}, {}, {"target": T, "importer": "typed.ts", "phase": "compile"}),
    case("requested-named-input-orders-every-other-target", {"typegen": {"compile": ["plain.ts"]}, "check": {"compile": ["plain.ts"]}}, {"check": [T]}, None, {"declared": ["schema.ts"]}),
    case("closure-without-generated-output-adds-nothing", {"typegen": {"compile": ["plain.ts"]}, "check": {"compile": ["plain.ts"]}}, {}),
]

FIXTURE_KEY = {
    "producer": {"project": "fixture-shell", "target": T, "manifest": "shell/project.json", "output": "gen/out.ts"},
    "files": {
        "plain.ts": "export const plain = 1;",
        "reducer.ts": "export * from \"./gen/out.ts\";",
        "schema.ts": "import \"./reducer.ts\"; export const schema = 1;",
        "typed.ts": "import type { Shape } from \"./gen/out.ts\"; export const typed: Shape | number = 1;",
    },
    "generated": "export type Shape = number; export const generated = 1;",
    "caseSchema": CASE_SCHEMA,
    "cases": CASES,
}

NEW_TEST = '''
test("generated task consumption is decided per consuming target",async()=>{
 const row=fixture.generatedCompileOutputs.perTargetConsumption,ts=require("typescript"),source=readInferenceSourceV1("owner-discovery"),tree=ts.createSourceFile("owner.mjs",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),definition=tree.statements.find((node:any)=>ts.isFunctionDeclaration(node)&&node.name?.text==="discoverBuildOwnerAdmissionFactsV1"),first=definition.body.statements.find((node:any)=>ts.isVariableStatement(node)&&node.declarationList.declarations.some((node:any)=>node.name.getText(tree)==="generatedDependencies")),code=source.slice(first.getStart(tree),definition.body.getEnd()-1).replaceAll("import.meta.url",JSON.stringify(import.meta.url)),parser=require(join(findWorkspaceRoot(import.meta.dir),"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📥️inference/🟨️.mjs")),api=await import("../../../🕸️graph/🚪️admission/🟦️.ts"),root="/fixture",producer=row.producer,at=(path:string)=>join(root,path),virtual=new Map(Object.entries(row.files).map(([path,text])=>[at(path),text as string])),valid=new Ajv({strict:true}).compile(row.caseSchema),output=at(producer.output),own=producer.target.slice(producer.target.lastIndexOf(":")+1);
 const roots=(value:any)=>({compile:new Set<string>((value?.compile??[]).map(at)),declared:new Set<string>((value?.declared??[]).map(at)),command:new Set<string>((value?.command??[]).map(at))});
 const run=async(item:any)=>{const files=new Set<string>(),dependencies=new Set<string>(),inputs=new Map<string,Set<string>>(),targets=Object.keys(item.targets),ports={roots:{targets:new Map(targets.map(name=>[name,roots(item.targets[name])])),shared:roots(item.shared)},ownerAdmissionSchema,buildSourceDemandV1:api.buildSourceDemandV1,scripts:{modules:new Set(virtual.keys()),generatedImports:new Map(),generatedOutput:(path:string)=>path===output?producer:undefined},targets,json:{name:producer.project},workspaceRoot:root,context:{},dependencyInputs:inputs,caretInputs:new Set(),addFile:(path:string)=>files.add(path),dependencies,files,byPackage:new Map(),createRequire:()=>()=>ts,observedTypeScriptHost:()=>({fileExists:(path:string)=>virtual.has(path),readFile:(path:string)=>virtual.get(path),directoryExists:()=>true,getCurrentDirectory:()=>root}),prepareSourceAnalysesV1:()=>({module:{}}),observedSourceAnalysisV1:(_provider:unknown,path:string)=>({imports:parser.moduleSourceImports(path,virtual.get(path)),runtimeImports:parser.runtimeModuleSourceImports(path,virtual.get(path))}),observedRequireResolve:(_from:string,specifier:string)=>{if(virtual.has(specifier))return specifier;throw Object.assign(Error("Missing demanded Source"),{code:"MODULE_NOT_FOUND"});},buildImportResourceV1:(path:string,specifier:string)=>({specifier:specifier.startsWith(".")?resolve(dirname(path),specifier):specifier,source:true}),readPhysicalSource:(path:string)=>{if(!virtual.has(path))throw Error("Missing demanded Source");return Buffer.from(virtual.get(path)!);},addDependency:()=>{},addInput:(project:string,input:string)=>{const rows=inputs.get(project)??new Set<string>();rows.add(input);inputs.set(project,rows);},yieldInferenceV1:async()=>{},stage:async()=>{},buildPath:(path:string)=>path.replaceAll("\\\\","/"),join,relative,isAbsolute:require("node:path").isAbsolute};let failure:any;let result:any;try{result=await new Function(...Object.keys(ports),"return(async()=>{"+code+"})()")(...Object.values(ports));}catch(error){failure=error;}return{failure,result,files};};
 const reaches=async(entries:Set<string>,runtime:boolean)=>{if(!entries.size)return false;if(runtime){const built=await build({entryPoints:[...entries],bundle:true,write:false,format:"esm",metafile:true,logLevel:"silent",plugins:[{name:"virtual",setup(builder:any){builder.onResolve({filter:/.*/},(args:any)=>({path:args.kind==="entry-point"?args.path:resolve(args.resolveDir,args.path),namespace:"virtual"}));builder.onLoad({filter:/.*/,namespace:"virtual"},(args:any)=>({contents:args.path===output?row.generated:virtual.get(args.path)!,loader:"ts",resolveDir:dirname(args.path)}));}}]});return Object.keys(built.metafile!.inputs).includes("virtual:"+output);}const seen=new Set<string>(),pending=[...entries];while(pending.length){const path=pending.pop()!;if(seen.has(path))continue;seen.add(path);for(const imported of ts.preProcessFile(virtual.get(path)!).importedFiles.map((file:any)=>file.fileName)){const next=resolve(dirname(path),imported);if(next===output)return true;if(virtual.has(next))pending.push(next);}}return false;};
 for(const item of row.cases){expect(valid(item),item.id).toBe(true);const actual=await run(item),targets=Object.keys(item.targets),consuming=new Map<string,boolean>();for(const name of targets){const value=roots(item.targets[name]);consuming.set(name,await reaches(new Set([...value.compile,...value.declared]),false)||await reaches(value.command,true));}
  const shared=roots(item.shared),sharedHit=await reaches(new Set([...shared.compile,...shared.declared]),false)||await reaches(shared.command,true),cycle=targets.find(name=>name===own&&consuming.get(name));
  expect(Boolean(actual.failure),item.id).toBe(Boolean(cycle));expect(Boolean(item.refusal),item.id).toBe(Boolean(cycle));
  if(cycle){for(const context of["Generated task cannot consume its own output",'"target":"'+item.refusal.target+'"','"importer":"'+item.refusal.importer+'"','"phase":"'+item.refusal.phase+'"'])expect(String(actual.failure),item.id).toContain(context);continue;}
  expect(actual.failure,item.id).toBeUndefined();const expected=Object.fromEntries(targets.filter(name=>name!==own&&(consuming.get(name)||sharedHit)).map(name=>[name,[producer.target]]));expect(item.generatedDependencies,item.id).toEqual(expected);expect(actual.result.generatedDependencies??{},item.id).toEqual(expected);expect(actual.files.has(producer.manifest),item.id).toBe(sharedHit||[...consuming.values()].some(Boolean));
 }
});
'''


def tests(text):
    text = sub(text, "compileModules:new Set(row.compile.map(path=>join(root,path))),declaredCompileEntries:new Set(),commandModules:new Set(row.runtime.map(path=>join(root,path))),targets:[\"generate\"],".replace('\\"','"'),
        "roots:{targets:new Map([[\"generate\",{compile:new Set(row.compile.map(path=>join(root,path))),declared:new Set(),command:new Set(row.runtime.map(path=>join(root,path)))}]]),shared:" + EMPTY + "},targets:[\"generate\"],".replace('\\"','"'), "traversal-ports")
    text = sub(text, "ports={scripts,compileModules:new Set([entry]),ownerAdmissionSchema,", "ports={scripts,roots:{targets:new Map([[\"test\",{compile:new Set([entry]),declared:new Set(),command:new Set([entry])}]]),shared:" + EMPTY + "},ownerAdmissionSchema,".replace('\\"','"'), "package-ports")
    text = sub(text, "declaredCompileEntries:new Set(),commandModules:new Set([entry]),targets:[\"test\"],".replace('\\"','"'), "targets:[\"test\"],".replace('\\"','"'), "package-targets")
    text = sub(text, "ports={scripts,compileModules:new Set(phase===\"compile\"?[entry]:[]),ownerAdmissionSchema,buildSourceDemandV1:admission.buildSourceDemandV1,declaredCompileEntries:new Set(),commandModules:new Set(executable?[entry]:[]),targets:[target],".replace('\\"','"'),
        "ports={scripts,roots:{targets:new Map([[target,{compile:new Set(phase===\"compile\"?[entry]:[]),declared:new Set(),command:new Set(executable?[entry]:[])}]]),shared:" + EMPTY + "},ownerAdmissionSchema,buildSourceDemandV1:admission.buildSourceDemandV1,targets:[target],".replace('\\"','"'), "compile-ports")
    text = sub(text, "scriptCode=scriptDefinition.getText(scriptTree).replace(/^export /u,\"\")".replace('\\"','"'),
        "helpers=[\"memoDigestV1\",\"closurePathNameV1\"].map(name=>scriptTree.statements.find((node:any)=>ts.isFunctionDeclaration(node)&&node.name?.text===name).getText(scriptTree)).join(\"\\n\"),scriptCode=helpers+\"\\n\"+scriptDefinition.getText(scriptTree).replace(/^export /u,\"\")".replace('\\"','"'), "script-code")
    text = sub(text, "ports={resolve,relative,join,buildPath:", "ports={createHash,resolve,relative,join,buildPath:", "script-ports")
    text = sub(text, "cache={files:new Map(),closures:new Map(),modules:new Set(),resources:new Set(),generatedImports:new Map(),", "cache={files:new Map(),closures:new Map(),paths:new Map(),compiledNames:new WeakMap(),moduleSets:new WeakMap(),declared:new Map(),modules:new Set(),resources:new Set(),generatedImports:new Map(),", "script-cache")
    return text.rstrip("\n") + "\n" + NEW_TEST


def fixture(text):
    anchor = '  "generatedCompileOutputs": {\n'
    block = json.dumps({"perTargetConsumption": FIXTURE_KEY}, indent=2, ensure_ascii=False)
    inner = block[block.index("\n") + 1:block.rindex("\n")]
    inner = "\n".join("  " + line if line else line for line in inner.split("\n"))
    return sub(text, anchor, anchor + inner + ",\n", "fixture")
