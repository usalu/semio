import { readFileSync, writeFileSync } from "node:fs";
import { Database } from "bun:sqlite";
import fixture from "../🧫️fixtures/📏️counts/🔣️.json";

/** 🔎️ Checks or edits physical SQLite through the independent SQLite implementation. */
export function runBimSqliteOracle(input:string[]):void {
 const file=input.length>=3&&input.at(-2)==="--file"?input.at(-1):undefined,args=file===undefined?input:input.slice(0,-2),[mode,word]=args;
 if(mode!=="words"&&mode!=="edit"||args.length!==(mode==="words"?2:1)||mode==="words"&&!/^[0-9a-f]{16}$/.test(word??""))throw new Error("sqlite-oracle words <binary64-hex> | edit [--file <path>]");
 const db=Database.deserialize(readFileSync(file??0),{safeIntegers:true}),quote=(name:string)=>'"'+name.replaceAll('"','""')+'"';
 try{
  if(db.query<{integrity_check:string},[]>("PRAGMA integrity_check").get()?.integrity_check!=="ok"||db.query("PRAGMA foreign_key_check").all().length)throw new Error("Independent SQLite file integrity refused");
  const tables=db.query<{name:string},[]>("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all();
  const counts:{tableCount:number;totalRows:number;tables:Record<string,number>}=fixture;
  if(tables.length!==counts.tableCount)throw new Error("Independent SQLite table cardinality");
  let totalRows=0,populatedWords=0,declaredRoles=0,populatedRoles=0;
  for(const{name}of tables){const n=db.query<{n:bigint},[]>("SELECT count(*) AS n FROM "+quote(name)).get()!.n;if(n!==BigInt(counts.tables[name]??-1))throw new Error("Independent SQLite row cardinality "+name);totalRows+=Number(n)}
  if(totalRows!==counts.totalRows)throw new Error("Independent SQLite total row cardinality");
  if(mode==="words"){
   const expected=BigInt("0x"+word!),exponent=(expected>>52n)&2047n,mantissa=expected&4503599627370495n,kind=exponent!==2047n?"finite":mantissa!==0n?"nan":expected>>63n?"negativeInfinity":"positiveInfinity";
   const bytes=new DataView(new ArrayBuffer(8));bytes.setBigUint64(0,expected,true);const finiteValue=kind==="finite"?bytes.getFloat64(0,true):undefined;
   for(const{name}of tables){
    const columns=db.query<{name:string},[]>("PRAGMA table_info("+quote(name)+")").all();
    for(const{name:bitsName}of columns)if(bitsName.endsWith("_bits")){
     const base=bitsName.slice(0,-5),className=base+"_class";declaredRoles++;let roleWords=0;
     if(!columns.some(c=>c.name===base)||!columns.some(c=>c.name===className))throw new Error("Independent SQLite binary64 role shape");
     const rows=db.query<{q:number|bigint|null;w:bigint|null;k:string|null},[]>("SELECT "+quote(base)+" AS q,"+quote(bitsName)+" AS w,"+quote(className)+" AS k FROM "+quote(name)).all();
     for(const row of rows){
      if(row.w===null){if(row.q!==null||row.k!==null)throw new Error("Independent absent binary64 role");continue}
      if(typeof row.w!=="bigint"||BigInt.asUintN(64,row.w)!==expected||row.k!==kind)throw new Error("Independent original binary64 word or class disagrees "+name+"."+base);
      if(kind==="finite"){if(typeof row.q!=="number"&&typeof row.q!=="bigint"||Number(row.q)!==finiteValue)throw new Error("Independent finite query disagrees")}else if(row.q!==null)throw new Error("Independent nonfinite query is present");
      populatedWords++;roleWords++;
     }if(roleWords===0)throw new Error("Independent binary64 role has no witness "+name+"."+base);populatedRoles++;
    }
   }
   if(declaredRoles===0||populatedRoles!==declaredRoles||populatedWords===0)throw new Error("Independent word corpus misses declared float roles");
   process.stdout.write(JSON.stringify({schema:"bim.sqlite.oracle/v1",mode,word,tableCount:tables.length,totalRows,populatedWords,declaredRoles,populatedRoles,checked:true})+"\n");
  }else{
   if(db.query<{n:bigint},[]>("SELECT count(*) AS n FROM bim_project").get()!.n!==1n)throw new Error("Independent project cardinality");
   const sql="UPDATE bim_property_value SET value=?,value_bits=?,value_class='finite' WHERE kind='Length' AND name='Length' AND set_id IN(SELECT s.id FROM bim_property_set s JOIN bim_property_element e ON e.id=s.element_id WHERE s.name='set/λ' AND e.map_key='external/element')";
   const bytes=new DataView(new ArrayBuffer(8));bytes.setFloat64(0,9.75,true);
   const changed=db.run(sql,[9.75,bytes.getBigInt64(0,true)]);if(changed.changes!==1)throw new Error("Independent Length cardinality");
   db.run("UPDATE bim_project SET name=?",["SQLite edited BIM"]);const bytesOut=db.serialize();if(file===undefined)process.stdout.write(bytesOut);else{writeFileSync(file,bytesOut);process.stdout.write(JSON.stringify({schema:"bim.sqlite.oracle/v1",mode,tableCount:tables.length,totalRows,checked:true})+"\n")}
  }
 }finally{db.close()}
}

if (import.meta.main) runBimSqliteOracle(process.argv.slice(2));
