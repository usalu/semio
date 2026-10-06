import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { closeSync, existsSync, fsyncSync, mkdirSync, openSync, readFileSync, writeFileSync, writeSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { createRequire } from "node:module";
const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),require=createRequire(join(root,"package.json")),ts=require("typescript"),[command,epoch="1"]=process.argv.slice(2),out=join(ticket,"🗑️generated/wires-document-contract"),owner="✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",testPath=owner+"/🧪️tests/🪪️document-contract/🟦️.ts",sha=(s:string)=>createHash("sha256").update(s).digest("hex");mkdirSync(out,{recursive:true});
const child={childId:"member 🧬\u0000",target:{artifactId:"independent target",dialect:{artifactKind:"s.stdio.semio",standard:"1",subset:"graph"}}},snapshot={wiresFixture:{scene:"neutral values",numbers:[0,-1,0.5],enabled:true},content:child,meta:null},fixture={schemaVersion:1,declaredChildKind:"s.stdio.semio",cases:[
 {id:"complete-snapshot",kind:"snapshot",value:snapshot,accepted:true},{id:"independent-child-identities",kind:"snapshot",value:{wiresFixture:[],content:{...child,childId:"different member"},meta:"ä 🪐\u0000"},accepted:true},
 {id:"missing-content",kind:"snapshot",value:{wiresFixture:null,meta:null},accepted:false},{id:"extra-parent-field",kind:"snapshot",value:{...snapshot,boardFixture:{}},accepted:false},{id:"missing-target",kind:"snapshot",value:{...snapshot,content:{childId:"member"}},accepted:false},{id:"extra-child-field",kind:"snapshot",value:{...snapshot,content:{...child,extra:true}},accepted:false},
 {id:"empty-parent-delta",kind:"diff",value:{},accepted:true},{id:"content-delta-is-child-owned",kind:"diff",value:{content:{}},accepted:false},{id:"fixture-delta-is-absent",kind:"diff",value:{wiresFixture:null},accepted:false},{id:"meta-delta-is-absent",kind:"diff",value:{meta:null},accepted:false},{id:"nonrecord-delta",kind:"diff",value:[],accepted:false},
 ...[{},null,[],true,1,"mutation",{kind:"move-node",nodeId:"n"},{kind:"set-content",content:child}].map((value,index)=>({id:"parent-mutation-refused-"+index,kind:"mutation",value,accepted:false}))
]};
const schema={$schema:"http://json-schema.org/draft-07/schema#",type:"object",additionalProperties:false,required:["schemaVersion","declaredChildKind","cases"],properties:{schemaVersion:{const:1},declaredChildKind:{const:"s.stdio.semio"},cases:{type:"array",minItems:1,items:{type:"object",additionalProperties:false,required:["id","kind","value","accepted"],properties:{id:{type:"string",minLength:1},kind:{enum:["snapshot","diff","mutation"]},value:{},accepted:{type:"boolean"}}}}}};
const corpus=owner+"/🧫️fixtures/🪪️document-contract/🔣️.json",corpusSchema=owner+"/🧫️fixtures/🪪️document-contract/🛂️schema/🔣️.json";
if(command==="publish"){
 const sourcePath=join(out,"source-"+epoch+".json"),sourceRaw=readFileSync(sourcePath,"utf8"),source=JSON.parse(sourceRaw),modelPlanPath=join(out,"model-"+epoch+"/plan.json"),modelPlanRaw=readFileSync(modelPlanPath,"utf8"),modelPlan=JSON.parse(modelPlanRaw),terminalPath=join(out,"model-"+epoch+"/terminal.json"),terminalRaw=readFileSync(terminalPath,"utf8"),terminal=JSON.parse(terminalRaw),proofPath=join(out,"independent-source-"+epoch+".json"),proofRaw=readFileSync(proofPath,"utf8"),proof=JSON.parse(proofRaw),destination=join(out,"publication-"+epoch+".json"),journal=join(out,"publication-"+epoch+"-intent.jsonl"),physical=(path:string):string|null=>existsSync(join(root,path))?readFileSync(join(root,path),"utf8"):null;
 assert.equal(existsSync(destination),false);assert.equal(existsSync(journal),false);assert.equal(proof.ready,true);assert.equal(proof.sourceHash,sha(sourceRaw));assert.equal(proof.planHash,sha(modelPlanRaw));assert.equal(proof.terminalHash,sha(terminalRaw));assert.equal(terminal.exitCode,0);assert.equal(terminal.postGaps.length,0);
 for(const row of source.pairs){assert.ok(physical(row.path)===row.before,row.path);assert.equal(row.before===null?null:sha(row.before),row.beforeHash);assert.equal(sha(row.after),row.afterHash);}for(const row of modelPlan.floor)assert.ok(physical(row.path)===row.before,row.path);
 const descriptor=openSync(journal,"wx");let sourceWrites=0;try{writeSync(descriptor,JSON.stringify({phase:"prepared",sourcePath,sourceRaw,modelPlanPath,modelPlanRaw,terminalPath,terminalRaw,proofPath,proofRaw,producer:{path:import.meta.path,source:readFileSync(import.meta.path,"utf8")},sourceWrites,atomicBatchClaimed:false})+"\n");fsyncSync(descriptor);for(const row of source.pairs){assert.ok(physical(row.path)===row.before,row.path);writeSync(descriptor,JSON.stringify({phase:"intent",row,sourceWrites})+"\n");fsyncSync(descriptor);const path=join(root,row.path);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,row.after);assert.ok(physical(row.path)===row.after,row.path);sourceWrites++;writeSync(descriptor,JSON.stringify({phase:"written",path:row.path,sourceWrites})+"\n");fsyncSync(descriptor);}for(const row of source.pairs)assert.ok(physical(row.path)===row.after,row.path);for(const row of modelPlan.floor.filter((row:any)=>!source.pairs.some((pair:any)=>pair.path===row.path)))assert.ok(physical(row.path)===row.before,row.path);writeSync(descriptor,JSON.stringify({phase:"completed",sourceWrites})+"\n");fsyncSync(descriptor);writeFileSync(destination,JSON.stringify({sourceHash:sha(sourceRaw),proofHash:sha(proofRaw),journalHash:sha(readFileSync(journal,"utf8")),sourceWrites,exactPostimages:source.pairs.length,atomicBatchClaimed:false,wholeRootAccepted:false}));console.log("[DEBUG] Wires document contract published "+JSON.stringify({sourceWrites}));}finally{closeSync(descriptor);}
}else if(command==="stage"){
 const destination=join(out,"source-"+epoch+".json");assert.equal(existsSync(destination),false);const before=readFileSync(join(root,testPath),"utf8"),imports=before.slice(0,before.indexOf('import snapshot from')).replace('import { readFileSync, readdirSync } from "node:fs";\n','').replace('import { join } from "node:path";\n','').replace('import ts from "typescript";\n',''),after=imports+`import fixture from "../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };
import fixtureSchema from "../../🧫️fixtures/🪪️document-contract/🛂️schema/🔣️.json" with { type: "json" };
import { parseWiresArtifact } from "../../🟦️.ts";
import { parseWiresSnapshot } from "../../📸️snapshot/🟦️.ts";
import { decodeWiresJsonSnapshot } from "../../../🚪️io/📸️snapshot/🔣️json/🟦️.ts";

/** 🪪️ Parent document admission agrees with its closed schemas and child ownership. */
export function testWiresDocumentContractOracle(): void {
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema(valueSchema).addSchema(ioSchema).addSchema(childSchema).addSchema(artifactSchema);
  const validateFixture = ajv.compile(fixtureSchema);
  assert.equal(validateFixture(fixture), true, JSON.stringify(validateFixture.errors));
  assert.equal(validateFixture({ ...fixture, extra: true }), false);
  assert.equal(new Set(fixture.cases.map(row => row.id)).size, fixture.cases.length);
  for (const schema of [artifactSchema, snapshotSchema]) assert.equal(schema.properties.content["x-semio-child-kind"], fixture.declaredChildKind);
  assert.deepEqual(Object.keys(diffSchema.properties), []);
  const documents = [[ajv.compile(artifactSchema), parseWiresArtifact], [ajv.compile(snapshotSchema), parseWiresSnapshot]] as const;
  const validateDiff = ajv.compile(diffSchema), validateMutation = ajv.compile(mutationSchema);
  for (const row of fixture.cases) {
    if (row.kind === "snapshot") {
      for (const [validate, parse] of documents) {
        assert.equal(validate(row.value), row.accepted, row.id + ": " + JSON.stringify(validate.errors));
        if (row.accepted) {
          const canonical = decodeWiresJsonSnapshot(row.value);
          assert.deepEqual(parse(canonical), canonical, row.id);
        } else assert.throws(() => parse(decodeWiresJsonSnapshot(row.value)), row.id);
      }
    } else if (row.kind === "diff") assert.equal(validateDiff(row.value), row.accepted, row.id);
    else {
      assert.equal(row.accepted, false, row.id);
      assert.equal(validateMutation(row.value), false, row.id);
    }
  }
}
`;
 const pairs=[{path:testPath,before,after},{path:corpus,before:null,after:JSON.stringify(fixture,null,2)+"\n"},{path:corpusSchema,before:null,after:JSON.stringify(schema,null,2)+"\n"}].map(row=>({...row,beforeHash:row.before===null?null:sha(row.before),afterHash:sha(row.after)}));for(const row of pairs)assert.ok((existsSync(join(root,row.path))?readFileSync(join(root,row.path),"utf8"):null)===row.before,row.path);writeFileSync(destination,JSON.stringify({producer:{path:import.meta.path,source:readFileSync(import.meta.path,"utf8")},pairs,fixture,schema,sourceWrites:0}));console.log("[DEBUG] Wires parent contract staged "+JSON.stringify({pairs:pairs.length,cases:fixture.cases.length}));
}else if(command==="model"){
 const sourcePath=join(out,"source-"+epoch+".json"),raw=readFileSync(sourcePath,"utf8"),source=JSON.parse(raw),mirror=join(out,"model-"+epoch);assert.equal(existsSync(mirror),false);mkdirSync(mirror,{recursive:true});const floor:any[]=[],visited=new Set<string>();
 const capture=(path:string):void=>{if(visited.has(path))return;visited.add(path);const row=source.pairs.find((r:any)=>r.path===path),before=existsSync(join(root,path))?readFileSync(join(root,path),"utf8"):null,body=row?.after??before;assert.ok(body!==null,path);if(row)assert.ok(before===row.before,path);floor.push({path,before,source:body,sourceHash:sha(body)});const destination=join(mirror,path);mkdirSync(dirname(destination),{recursive:true});writeFileSync(destination,body);if(!path.endsWith(".ts"))return;const ast=ts.createSourceFile(path,body,ts.ScriptTarget.Latest,true);for(const node of ast.statements)if((ts.isImportDeclaration(node)||ts.isExportDeclaration(node)) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier) && node.moduleSpecifier.text.startsWith(".")){const target=resolve(root,dirname(path),node.moduleSpecifier.text);assert.ok(target.startsWith(root+"/"));capture(target.slice(root.length+1));}};capture(testPath);
 writeFileSync(join(mirror,"plan.json"),JSON.stringify({sourcePath,sourceRaw:raw,sourceHash:sha(raw),floor,producer:{path:import.meta.path,source:readFileSync(import.meta.path,"utf8")}}));const invocation='import{testWiresDocumentContractOracle}from '+JSON.stringify(join(mirror,testPath))+';testWiresDocumentContractOracle();console.log("[DEBUG] Wires document contract '+source.fixture.cases.length+' cases completed");';const actual=Bun.spawnSync([process.execPath,"-e",invocation],{cwd:root,stdout:"pipe",stderr:"pipe"});writeFileSync(join(mirror,"actual.log"),Buffer.concat([actual.stdout,actual.stderr]));const postGaps=floor.filter(row=>(existsSync(join(root,row.path))?readFileSync(join(root,row.path),"utf8"):null)!==row.before).map(row=>row.path);assert.equal(readFileSync(sourcePath,"utf8"),raw);writeFileSync(join(mirror,"terminal.json"),JSON.stringify({sourceHash:sha(raw),exitCode:actual.exitCode,frames:floor.length,postGaps,completedCases:actual.exitCode===0?source.fixture.cases.length:0,independentAjvExecuted:actual.exitCode===0,sourceWrites:0}));console.log(JSON.stringify({exitCode:actual.exitCode,frames:floor.length,postGaps:postGaps.length}));process.exit(actual.exitCode);
}else throw Error("stage|model");
