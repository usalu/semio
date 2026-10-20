#!/usr/bin/env python3
"""Authors group 3 (live wiring of unit mode into `typecheck`): new files under staged3/new/ and anchored edits in staged3/edits.json (same engine as group 2)."""
import json,os
HERE=os.path.dirname(os.path.abspath(__file__)); R=os.path.abspath(os.path.join(HERE,*['..']*8)); S=os.path.join(HERE,'staged3')
L='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
CMD=L+'🎛️commands/🪁️typecheck/'
def w(rel,text):
    p=os.path.join(S,'new',rel);os.makedirs(os.path.dirname(p),exist_ok=True);open(p,'w',encoding='utf8').write(text)
w(CMD+'🧬️schema/🎯️units.json',json.dumps({
 "$schema":"https://json-schema.org/draft/2020-12/schema","$id":"https://semio.tech/schemas/repo/typecheck/units-selection","title":"Unit-wise TypeScript Check Selection","type":"object","additionalProperties":False,"required":["project","only","jobs","replan"],
 "properties":{
  "project":{"type":"string","minLength":1,"maxLength":256,"pattern":"^(?![/~\\\\])(?![A-Za-z]:)(?!(.*/)?\\.\\.(/|$))[^\\u0000\\\\]+\\.json$"},
  "only":{"description":"Unit labels (workspace-relative directories) or ids to check together with the units they depend on; empty checks every unit.","type":"array","maxItems":16,"uniqueItems":True,"items":{"type":"string","minLength":1,"maxLength":256}},
  "jobs":{"description":"Compiler processes at once; the host capacity gate may admit fewer.","type":"integer","minimum":1,"maximum":8},
  "replan":{"description":"Ignore remembered import resolutions and derive the unit graph from the sources.","type":"boolean"}}},ensure_ascii=False,indent=2)+"\n")
w(CMD+'🧫️fixtures/🎯️units.json',json.dumps({
 "accepted":[
  {"words":["tsconfig.json"],"selection":{"project":"tsconfig.json","only":[],"jobs":1,"replan":False}},
  {"words":["✏️s/tsconfig.json","--jobs","4","--only","🧰️framework/🔨️modules/🏃️process","--replan"],"selection":{"project":"✏️s/tsconfig.json","only":["🧰️framework/🔨️modules/🏃️process"],"jobs":4,"replan":True}}],
 "rejected":[[],["../outside.json"],["C:/outside.json"],["/outside.json"],["source.ts"],["tsconfig.json","--jobs","0"],["tsconfig.json","--jobs","9"],["tsconfig.json","--jobs","x"],["tsconfig.json","--only"],["tsconfig.json","--only","a","--only","a"],["tsconfig.json","--outDir","elsewhere"]]},ensure_ascii=False,indent=2)+"\n")
w(CMD+'🧪️tests/🟦️.ts','''import {describe,expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv2020 from "ajv/dist/2020";
import {compileJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {readTypecheckUnitsSelectionV1,resolveTypecheckUnitsSelectionV1} from "../🟦️.ts";

const read = (name: string) => JSON.parse(readFileSync(join(import.meta.dir, "..", name), "utf8"));
describe("typecheck units selection", () => {
  const schema = read("🧬️schema/🎯️units.json"), cases = read("🧫️fixtures/🎯️units.json"), own = compileJsonSchemaSubset(schema), oracle = new Ajv2020({strict: false}).compile(schema);
  test("accepted words equal their selection, in the owned validator and in Ajv", () => {
    for (const row of cases.accepted) {expect(oracle(row.selection)).toBe(true); expect(own.validate(row.selection)).toEqual([]); expect(readTypecheckUnitsSelectionV1(row.words)).toEqual(row.selection);}
  });
  test("rejected words are refused by the parser, the owned validator and Ajv", () => {
    for (const words of cases.rejected) {
      expect(() => readTypecheckUnitsSelectionV1(words)).toThrow();
      const loose = {project: words[0], only: [], jobs: 1, replan: false};
      if (!(words[0] ?? "").endsWith(".json") || /^(?:[\\\\/~]|[A-Za-z]:|\\.\\.)/u.test(words[0] ?? "")) {expect(oracle(loose)).toBe(false); expect(own.validate(loose).length).toBeGreaterThan(0);}
    }
  });
  test("a configuration outside the workspace is refused", async () => {
    await expect(resolveTypecheckUnitsSelectionV1(import.meta.dir, ["../../../../../../../../../../../../../../tmp/none.json"])).rejects.toThrow();
  });
});
''')
edits=[]
def ed(id,file,ops,why):
    s=open(os.path.join(R,file),encoding='utf8').read()
    for o in ops:
        o['count']=s.count(o['old']); assert o['count']==o.pop('expect',1),(id,o['old'][:50],o['count'])
    edits.append({'id':id,'file':file,'why':why,'ops':ops})
ed('typecheck-command-units',CMD+'🟦️.ts',[
 {'old':'import selectionSchema from "./🧬️schema/🎯️project.json" with { type: "json" };\nconst selectionContract=compileJsonSchemaSubset(selectionSchema);\n',
  'new':'import selectionSchema from "./🧬️schema/🎯️project.json" with { type: "json" };\nimport unitsSchema from "./🧬️schema/🎯️units.json" with { type: "json" };\nconst selectionContract=compileJsonSchemaSubset(selectionSchema),unitsContract=compileJsonSchemaSubset(unitsSchema);\n'},
 {'old':'/** 🪁️ Type-checks every `🦑️repo` product TypeScript source.',
  'new':'''/** 🧩️ Reads `typecheck units <config.json> [--only <unit>]... [--jobs 1-8] [--replan]`: the unit-wise development gate. */
export function readTypecheckUnitsSelectionV1(words:readonly string[]):Readonly<{project:string;only:readonly string[];jobs:number;replan:boolean}>{
 const rest=[...words],selection:{project:string|undefined;only:(string|undefined)[];jobs:number;replan:boolean}={project:rest.shift(),only:[],jobs:1,replan:false};
 while(rest.length){const flag=rest.shift();if(flag==="--only")selection.only.push(rest.shift());else if(flag==="--jobs")selection.jobs=Number(rest.shift());else if(flag==="--replan")selection.replan=true;else selection.project=undefined;}
 if(unitsContract.validate(selection).length)throw Error("Expected typecheck units <workspace-relative configuration.json> [--only <unit>]... [--jobs 1-8] [--replan]");return selection as {project:string;only:readonly string[];jobs:number;replan:boolean};
}

/** 📂️ Resolves the unit-wise selection to a real configuration inside the workspace. */
export async function resolveTypecheckUnitsSelectionV1(workspace:string,words:readonly string[]):Promise<Readonly<{cwd:string;project:string;only:readonly string[];jobs:number;replan:boolean}>>{
 const selection=readTypecheckUnitsSelectionV1(words),cwd=await realpath(workspace),project=await realpath(resolve(cwd,selection.project)),distance=relative(cwd,project);
 if(project.length>256||isAbsolute(distance)||distance===".."||distance.startsWith(".."+sep)||!(await stat(project)).isFile())throw Error("TypeScript project must be a real workspace configuration");return{cwd,project,only:selection.only,jobs:selection.jobs,replan:selection.replan};
}

/** 🪁️ Type-checks every `🦑️repo` product TypeScript source.'''},
 {'old':'    const selection=segments[0]==="project"?await resolveTypecheckProjectSelectionV1(this.repoRoot,segments.slice(1)):undefined;\n',
  'new':'''    if(segments[0]==="units"){
      const units=await resolveTypecheckUnitsSelectionV1(this.repoRoot,segments.slice(1)),{typecheckRepositoryUnitsV1}=await import("../../🏗️build/🟦️typescript/🧩️units/🔌️repository/🟦️.ts");
      await typecheckRepositoryUnitsV1({workspace:this.repoRoot,invocation:this.invocation,control:createIssuedResourceControlV1(this.invocation,"typecheck:units",0),cwd:units.cwd,project:units.project,only:units.only,jobs:units.jobs,replan:units.replan,owner:"repo:typescript-units",timeoutMs:1_800_000});return;
    }
    const selection=segments[0]==="project"?await resolveTypecheckProjectSelectionV1(this.repoRoot,segments.slice(1)):undefined;
'''}],
 'Adds the unit-wise development gate `typecheck units`; `typecheck` and `typecheck project` (monolithic release gate) are unchanged.')
ed('typecheck-units-target',L+'📦️packages/🟦️typescript/📋️project.json',[
 {'old':'    "typecheck": {\n',
  'new':'''    "typecheck-units": {
      "metadata": {
        "semio": {
          "execution": { "version": 1, "kind": "source" },
          "cache": { "version": 1, "class": "diagnostic", "reason": "reads-caller-input" },
          "dashboard": {
            "title": { "en": "Check TypeScript unit by unit", "de": "TypeScript einheitenweise prüfen" },
            "description": { "en": "Fast development check: only units whose sources or dependency declarations changed are checked; results are shared between sessions. The monolithic check stays the release gate.", "de": "Schnelle Entwicklungsprüfung: nur Einheiten mit geänderten Quellen oder Abhängigkeitsdeklarationen werden geprüft; Ergebnisse werden zwischen Sitzungen geteilt. Die monolithische Prüfung bleibt das Freigabetor." },
            "verb": "check",
            "parameters": [
              { "id": "config", "title": { "en": "Project configuration", "de": "Projektkonfiguration" }, "description": { "en": "Choose an authored configuration used by the TypeScript compiler.", "de": "Verfasste Konfiguration für den TypeScript-Compiler auswählen." }, "kind": "text", "default": "tsconfig.json", "valuePositional": true, "suggestions": { "source": "configs" } }
            ]
          }
        }
      },
      "executor": "repo:script",
      "cache": false,
      "options": {
        "cwd": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript",
        "command": "bun ./📜️script.ts typecheck units",
        "forwardAllArgs": true
      },
      "outputs": []
    },
    "typecheck": {
'''}],
 'Dashboard target for the unit-wise check; uncached because the unit store is its own content-addressed cache.')
json.dump(edits,open(os.path.join(S,'edits.json'),'w'),ensure_ascii=False,indent=1)
print('group 3:',len(edits),'edit groups;',sum(1 for _,_,f in os.walk(os.path.join(S,'new')) for x in f),'new files')
