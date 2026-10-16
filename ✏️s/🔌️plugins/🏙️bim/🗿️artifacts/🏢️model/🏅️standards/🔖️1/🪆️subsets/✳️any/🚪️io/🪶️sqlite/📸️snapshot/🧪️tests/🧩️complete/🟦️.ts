import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import corruptions from "../🧫️fixtures/🧩️complete/⚠️corruptions/🔣️.json";
import {exportSqliteDatabase,importSqliteDatabase} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import input from "../🧫️fixtures/🧩️complete/🔣️.json";
import documentSchema from "../../../../../🧬️schema/🔣️.json";
import snapshotSchema from "../../../../../🧬️schema/📸️snapshot/🔣️.json";
import type {ModelSnapshot} from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {bimModelSnapshotToSqliteDatabase,bimModelSnapshotFromSqliteDatabase} from "../../🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../../../../../\ud83e\uddf0\ufe0fframework/\ud83d\udd28\ufe0fmodules/\ud83e\uddec\ufe0fschema/\u2705\ufe0fvalidator/\ud83d\udfe6\ufe0f.ts";
import {semioSchemaAjvV1} from "../../../../../../../../../../../../../../\ud83e\uddf0\ufe0fframework/\ud83d\udd28\ufe0fmodules/\ud83e\uddec\ufe0fschema/\ud83d\udd2e\ufe0foracles/\u2705\ufe0fvalidator/\ud83d\udfe6\ufe0f.ts";
const missing=["supports","load_cases","loads","option_groups","design_options","worksets","element_options","element_worksets","space_conditions","clash_sets","rules","issues","issue_comments","components","component_overrides","mep_elements"] as const;
const tables=["bim_support","bim_load_case","bim_structural_load","bim_option_group","bim_design_option","bim_workset","bim_element_option","bim_element_workset","bim_space_conditions","bim_clash_set","bim_rule","bim_issue","bim_issue_comment","bim_component","bim_component_override","bim_mep_element"];
const snapshot=input as unknown as ModelSnapshot;

test("BIM complete original input uses the genuine sixty-five-field schema",()=>{
 expect(Object.keys(input).sort()).toEqual(Object.keys(snapshotSchema.properties).sort());expect(Object.keys(input).length).toBe(65);
 expect(validateJsonSchemaSubset(snapshotSchema,input,snapshotSchema,[documentSchema])).toEqual([]);
 const oracle=semioSchemaAjvV1({strict:true,allErrors:true}).addSchema(documentSchema).compile(snapshotSchema);expect(oracle(input)).toBe(true);
 for(const field of missing)expect(Object.keys(input[field]).length,field).toBeGreaterThan(0);
 console.log("[DEBUG] BIM original complete input actual65fields nonempty16missingmaps both canonical schema and independent Ajv admitted");
});

test("BIM actual semantic projection and reconstruction conserve every original field",async()=>{
 const database=await bimModelSnapshotToSqliteDatabase(snapshot),restored=await bimModelSnapshotFromSqliteDatabase(database);
 expect(restored).toEqual(snapshot);
 const oracle=new Database(":memory:",{safeIntegers:true});try{
  oracle.run(readFileSync(new URL("../../🗄️.sql",import.meta.url),"utf8"));
  for(const table of database.tables)for(const row of table.rows)oracle.run('INSERT INTO "'+table.name.replaceAll('"','""')+'" VALUES('+row.values.map(()=>'?').join(',')+')',row.values.map(value=>value instanceof Uint8Array?Buffer.from(value):value));
  for(let index=0;index<missing.length;index++){const keys=oracle.query<{map_key:string},[]>("SELECT map_key FROM "+tables[index]+" ORDER BY ordinal").all().map(row=>row.map_key);expect(keys).toEqual(Object.keys(input[missing[index]!]).sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b))));}
  expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(oracle.query("SELECT name FROM bim_support WHERE map_key=?").get("support/λ\0")).toEqual({name:"Display α"});
  expect(oracle.query("SELECT component_key,name,value FROM bim_component_override ORDER BY ordinal").all()).toEqual([{component_key:"component/mount",name:"Width",value:"Width * 2 + 0.25"}]);
  expect(oracle.query("SELECT e.map_key,o.map_key AS target FROM bim_element_option e JOIN bim_design_option o ON o.document_id=e.document_id AND o.map_key=e.target_key ORDER BY e.ordinal").all()).toEqual([{map_key:"beam/λ",target:"option/β"}]);
 }finally{oracle.close();}
 console.log("[DEBUG] BIM actual65field semantic roundtrip and independent sixteen named map/ordinal/literal/reference SQL outputs passed; Native receiving custody remains separate");
});

test("BIM actual public pack declares its owning SQLite capability",()=>{
 const provider=readFileSync(new URL("../../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs",import.meta.url),"utf8");
 expect(provider.includes("fn sqlite_snapshot_codec()")).toBe(true);
 expect(provider.includes("Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())")).toBe(true);
});


test("BIM new named child ownership and active variants reject independently witnessed corruption",async()=>{
 const original=await exportSqliteDatabase(await bimModelSnapshotToSqliteDatabase(snapshot));
 for(const law of corruptions){const sqlite=Database.deserialize(original,{safeIntegers:true});try{
  sqlite.run("PRAGMA foreign_keys=OFF");sqlite.run("PRAGMA ignore_check_constraints=ON");sqlite.run(law.sql);
  expect(sqlite.query(law.witness).get(),law.name).toEqual({n:1n});
  await expect(bimModelSnapshotFromSqliteDatabase(await importSqliteDatabase(sqlite.serialize()))).rejects.toThrow();
 }finally{sqlite.close();}}
 console.log("[DEBUG] BIM six original named child/ordinal/variant corruptions independently witnessed by SQLite and refused by actual reconstruction");
},60000);
