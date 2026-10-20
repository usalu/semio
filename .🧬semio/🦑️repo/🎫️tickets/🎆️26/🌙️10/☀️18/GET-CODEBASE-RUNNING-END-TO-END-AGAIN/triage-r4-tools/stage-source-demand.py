#!/usr/bin/env python3
"""Stages the source-demand test repairs: reads the live test + fixture, writes patched copies under staged/ (nothing live), plus a run copy with absolute imports for the genuine-birth harness.
python3 stage-source-demand.py            stage + run copy
python3 stage-source-demand.py --apply    write the staged files over the live ones (only after the coordinator announces chain17 green)"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
LIB = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
TEST = LIB + "/🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts"
FIX = LIB + "/🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json"
OUT = HERE + "/staged"

def rep(text, old, new, count=1):
    assert text.count(old) == count, (text.count(old), old[:90])
    return text.replace(old, new)

CLOSURE = """/** 🧩 Extracts the named top-level declarations of one inference Source plus every sibling declaration they reference, except the ports a test supplies, so the extracted code stays complete when a helper is split off. */
const closureCodeV1=(ts:any,tree:any,roots:string[],ports:string[]):string=>{const defs=new Map<string,any>();for(const node of tree.statements){if(ts.isFunctionDeclaration(node)&&node.name)defs.set(node.name.text,node);else if(ts.isVariableStatement(node)&&(node.declarationList.flags&ts.NodeFlags.Const)!==0)for(const declaration of node.declarationList.declarations)if(ts.isIdentifier(declaration.name))defs.set(declaration.name.text,node);}const taken=new Set<string>(),order:any[]=[],visit=(name:string):void=>{if(taken.has(name)||ports.includes(name)||!defs.has(name))return;taken.add(name);const node=defs.get(name),walk=(child:any):void=>{if(ts.isIdentifier(child))visit(child.text);ts.forEachChild(child,walk);};walk(node);order.push(node);};roots.forEach(visit);return [...new Set(order)].map(node=>node.getText(tree).replace(/^export /u,"").replaceAll("import.meta.url",JSON.stringify(import.meta.url))).join("\\n");};
"""

def patch_test(s):
    s = rep(s, 'import {readFileSync} from "node:fs";\n', 'import {readFileSync} from "node:fs";\nimport {readCargoResolutionPolicyV1} from "../../../📥️inputs/🦀️resolution/🟦️.ts";\nimport resolutionDocument from "../../../📥️inputs/🦀️resolution/🔣️.json";\n')
    s = rep(s, 'statements.map((node:any)=>node.getText(parsed).replace(/^export /u,"")).join("\\n")+";return {"+names.join(",")+"};")', 'statements.map((node:any)=>node.getText(parsed).replace(/^export /u,"").replaceAll("import.meta.url",JSON.stringify(import.meta.url))).join("\\n")+";return {"+names.join(",")+"};")')
    s = rep(s, '{...api,...paths,observedFs:{existsSync:virtualExists,readFileSync},withHostPlatformOutputsV1:(value:unknown)=>value},true)', '{...api,...paths,observedFs:{existsSync:virtualExists,readFileSync},readCargoResolutionPolicyV1,resolutionDocument,withHostPlatformOutputsV1:(value:unknown)=>value},true)')
    s = rep(s, 'scriptDefinition=scriptTree.statements.find((node:any)=>ts.isFunctionDeclaration(node)&&node.name?.text==="relativeScriptInputs"),scriptCode=scriptDefinition.getText(scriptTree).replace(/^export /u,"")', 'scriptCode=["memoDigestV1","closurePathNameV1","relativeScriptInputs"].map(name=>scriptTree.statements.find((node:any)=>ts.isFunctionDeclaration(node)&&node.name?.text===name).getText(scriptTree).replace(/^export /u,"")).join("\\n")')
    s = rep(s, 'readPhysicalSource:(path:string)=>{reads.push(path);if(!virtual.has(path))throw Error("Missing actual Source");return Buffer.from(virtual.get(path)!);},', 'readPhysicalSource:(path:string)=>{reads.push(path);if(!virtual.has(path))throw Error("Missing actual Source");return Buffer.from(virtual.get(path)!);},requirePhysicalSourceV1:(path:string)=>{reads.push(path);if(!virtual.has(path))throw Error("Missing actual Source");},presentationResourcesV1:()=>new Set(),createHash,')
    s = rep(s, 'readPhysicalSource:(path:string)=>readFileSync(path),readToml:()=>({}),bindBuildCommandV1:', 'readPhysicalSource:(path:string)=>readFileSync(path),requirePhysicalSourceV1:(path:string)=>{if(!require("node:fs").lstatSync(path).isFile())throw Error("Ordinary command Source required");},presentationResourcesV1:()=>new Set(),readToml:()=>({}),bindBuildCommandV1:', 2)
    s = rep(s, '["inferProjectInputs","deduplicateProjectInputs","internProjectInputGroups"]', '["inferProjectInputs","deduplicateProjectInputs","internProjectInputGroups","cargoManifestInputV1"]')
    s = rep(s, 'definitions.map((node:any)=>node.getText(tree).replace(/^export /u,"")).join("\\n")+";return targetScriptSources;")', 'closureCodeV1(ts,tree,["targetScriptSources"],Object.keys(ports))+";return targetScriptSources;")')
    s = rep(s, 'type SourceDemandRootV1=', CLOSURE + 'type SourceDemandRootV1=')
    s = rep(s, 'definitions.map((node:any)=>node.getText(tree).replace(/^export /u,"").replaceAll("import.meta.url",JSON.stringify(import.meta.url))).join("\\n")+";return {targetScriptSources,createScriptInputCache};")', 'closureCodeV1(ts,tree,["targetScriptSources","createScriptInputCache"],Object.keys(ports))+";return {targetScriptSources,createScriptInputCache};")')
    s = rep(s, 'loaded={inferProjectInputs:inputApi.inferProjectInputs,', 'loaded={inferProjectInputs:inputApi.inferProjectInputs,cargoManifestInputV1:inputApi.cargoManifestInputV1,')
    s = rep(s, 'cache={files:new Map(),closures:new Map(),modules:new Set(),resources:new Set(),generatedImports:new Map(),', 'cache={files:new Map(),closures:new Map(),modules:new Set(),resources:new Set(),generatedImports:new Map(),declared:new Map(),paths:new Map(),compiledNames:new WeakMap(),moduleSets:new WeakMap(),')
    return s

def patch_fixture(s):
    s = rep(s, '"specifier": "@semio-tech/framework-manifest/view-state-and-public-invocation"', '"specifier": "../🛂️manifest/👁️view-state-and-public-invocation/🟦️.ts"')
    s = rep(s, '"specifier": "@semio-tech/framework-manifest/schema"', '"specifier": "../../🛂️manifest/🧬️schema/🟦️.ts"')
    s = rep(s, '        "production",\n        "cargo",\n', '        "production",\n        "cargoToolchain",\n')
    return s

def run_copy(s, origin):
    d = os.path.dirname(origin)
    def spec(m):
        p = os.path.normpath(os.path.join(d, m.group(2)))
        return m.group(1) + p + m.group(3)
    s = re.sub(r'(from ")(\.\.?/[^"]*)(")', spec, s)
    s = re.sub(r'(?m)^(import ")(\.\.?/[^"]*)(")', spec, s)
    s = re.sub(r'(import\(")(\.\.?/[^"]*)("\))', spec, s)
    s = re.sub(r'(?<!")import\.meta\.dir(?!")', lambda m: json.dumps(d, ensure_ascii=False), s)
    s = re.sub(r'(?<!")import\.meta\.url(?!")', lambda m: json.dumps("file://" + origin, ensure_ascii=False), s)
    return s

def main():
    test = patch_test(open(TEST, encoding="utf8").read())
    fix = patch_fixture(open(FIX, encoding="utf8").read())
    json.loads(fix)
    if "--apply" in sys.argv:
        open(TEST, "w", encoding="utf8").write(test)
        open(FIX, "w", encoding="utf8").write(fix)
        print("applied")
        return
    os.makedirs(OUT + "/run", exist_ok=True)
    open(OUT + "/source-demand.ts", "w", encoding="utf8").write(test)
    open(OUT + "/source-demand-fixture.json", "w", encoding="utf8").write(fix)
    runtest = run_copy(test, TEST)
    runtest = runtest.replace(os.path.dirname(TEST) + "/../../🧫️fixtures/🕸️graph/🔣️.json", OUT + "/run/fixture.json")
    runtest = re.sub(r'"[^"]*🧫️fixtures/🕸️graph/🔣️\.json"', json.dumps(OUT + "/run/fixture.json", ensure_ascii=False), runtest)
    open(OUT + "/run/source-demand.ts", "w", encoding="utf8").write(runtest)
    open(OUT + "/run/fixture.json", "w", encoding="utf8").write(fix)
    print("staged", OUT)
main()
