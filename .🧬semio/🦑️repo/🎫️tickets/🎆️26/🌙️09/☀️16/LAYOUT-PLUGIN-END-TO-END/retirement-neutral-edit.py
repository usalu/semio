from pathlib import Path
import json
root=Path.cwd()
base=root/"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/♻️retirement"
fixture=base/"🧪️tests/🔣️.json"
value=json.loads(fixture.read_text())
assert "managerRoleCases" not in value
value["managerRoleCases"]=[{"manager":"bun","roles":["node_modules","w","patches","package.json","bun.lock","bunfig.toml"]},{"manager":"npm","roles":["node_modules","w","patches","package.json","package-lock.json",".npmrc"]}]
fixture.write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n")
test=base/"🧪️tests/🟦️.ts"; old=test.read_text()
imports='import {materializeSchemaV1,schemaViewV1} from "../../../../../../../../🚀️bootstrap/📦️dependencies/🔌️managers/🧩️materialize/🟦️.ts";\nimport {packageManagerRegistryV1} from "../../../../../../../../🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";\n'
marker='const wire={...schema,$defs:{...schema.$defs,SelectedRetirementSnapshot:javascriptInstallSchemaV1.$defs.SelectedRetirementSnapshot,SelectedRetirementResult:javascriptInstallSchemaV1.$defs.SelectedRetirementResult}};'
replacement='const selectedSchema=materializeSchemaV1(schema,schemaViewV1(javascriptInstallSchemaV1.$defs.ManagerProtocol.manager.descriptor)) as typeof schema;\nconst wire={...selectedSchema,$defs:{...selectedSchema.$defs,SelectedRetirementSnapshot:javascriptInstallSchemaV1.$defs.SelectedRetirementSnapshot,SelectedRetirementResult:javascriptInstallSchemaV1.$defs.SelectedRetirementResult}};'
assert old.count(marker)==1
new=old.replace(marker,imports+'\n'+replacement).replace('function sourceFunctions(names:string[]):Record','function sourceFunctions(names:string[],selected=selectedSchema):Record').replace('retirementProtocol:schema.$defs.Protocol.const','retirementProtocol:selected.$defs.Protocol.const,retirementSchema:schema,materializeSchemaV1,schemaViewV1')
new+='''
test("retirement payload roles follow the selected descriptor and independent SQLite",()=>{
 const materialize=sourceFunctions(["materializeRepositoryDependencyRetirementSchemaV1"]).materializeRepositoryDependencyRetirementSchemaV1!,db=new Database(":memory:");
 try{for(const row of fixture.managerRoleCases){
  const manager=packageManagerRegistryV1.get(row.manager),resolved=materialize(manager.descriptor),roles=resolved.$defs.Protocol.const.unreadyPayloadRoles;
  const oracle=db.query("SELECT json_array('node_modules','w','patches','package.json',(SELECT json_extract(value,'$.name') FROM json_each(?1,'$.locks') WHERE json_extract(value,'$.role')='primary'),json_extract(?1,'$.configurations.scope[0]')) roles").get(JSON.stringify(manager.descriptor))as{roles:string};
  expect(roles).toEqual(row.roles);expect(roles).toEqual(JSON.parse(oracle.roles));expect(JSON.stringify(resolved)).not.toContain("{{");
  const api=sourceFunctions(["selectRepositoryDependencyRetirementCandidateV1"],resolved),plan:any=structuredClone(fixture.plan),candidate={prefix:"0".repeat(20),generation:null,status:"unready",receiptFingerprint:null,rootStamp:"current",roles:roles.slice(-2),ownerReferences:[]};
  plan.candidates.push(candidate);plan.generations.names.push(candidate.prefix);expect(api.selectRepositoryDependencyRetirementCandidateV1!(plan)?.prefix).toBe(candidate.prefix);
  candidate.roles=["unknown-manager-payload"];expect(()=>api.selectRepositoryDependencyRetirementCandidateV1!(plan)).toThrow();
 }}finally{db.close();}
});
'''
test.write_text(new)
schema=base/"🧬️schema/🔣️.json"; old=schema.read_text()
assert old.count('"bun.lock"')==1 and old.count('"bunfig.toml"')==1
schema.write_text(old.replace('"bun.lock"','"{{lockFile}}"').replace('"bunfig.toml"','"{{scopeConfig}}"'))
source=base/"🟦️.ts"; old=source.read_text()
marker='const retirementWireSchema={...retirementSchema,$defs:{...retirementSchema.$defs,SelectedRetirementSnapshot:javascriptInstallSchemaV1.$defs.SelectedRetirementSnapshot,SelectedRetirementResult:javascriptInstallSchemaV1.$defs.SelectedRetirementResult}},retirementProtocol=retirementSchema.$defs.Protocol.const;'
assert old.count(marker)==1
imports='import {materializeSchemaV1,schemaViewV1} from "../../../../../../../🚀️bootstrap/📦️dependencies/🔌️managers/🧩️materialize/🟦️.ts";\nimport type {ManagerDescriptorV1} from "../../../../../../../🚀️bootstrap/📦️dependencies/🔌️managers/🟦️.ts";\n'
replacement='const selectedRetirementSchema=materializeRepositoryDependencyRetirementSchemaV1(javascriptInstallSchemaV1.$defs.ManagerProtocol.manager.descriptor),retirementWireSchema={...selectedRetirementSchema,$defs:{...selectedRetirementSchema.$defs,SelectedRetirementSnapshot:javascriptInstallSchemaV1.$defs.SelectedRetirementSnapshot,SelectedRetirementResult:javascriptInstallSchemaV1.$defs.SelectedRetirementResult}},retirementProtocol=selectedRetirementSchema.$defs.Protocol.const;\n/** 🧩️ Resolves exact payload metadata names through the selected manager descriptor. */\nfunction materializeRepositoryDependencyRetirementSchemaV1(descriptor:ManagerDescriptorV1):typeof retirementSchema{return materializeSchemaV1(retirementSchema,schemaViewV1(descriptor)) as typeof retirementSchema;}'
source.write_text(old.replace(marker,imports+'\n'+replacement))
print("[DEBUG] Neutral retirement schema and descriptor-based role law authored")
