#!/usr/bin/env bun
/** 🏢️ BIM model owning SQLite and Rust package commands, plus the repository test-platform roles (`test oracle|subject|parity [level]`). */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import {runCmd} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import {BundleScript} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {resolve} from "node:path";
import { runBudgetedTestCommand } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { resolveTestLevel, testLevelBudgetMs } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";

import {readFileSync,writeFileSync} from "node:fs";
import {Database} from "bun:sqlite";

/** 🔎️ Independent SQLite interpretation and authoritative edits over original public IO files. */
class SqliteOracle extends BundleScript {
 run(input:string[]):void{
  const file=input.length>=3&&input.at(-2)==="--file"?input.at(-1):undefined,args=file===undefined?input:input.slice(0,-2),[mode,word]=args;
  if(mode!=="words"&&mode!=="edit"||args.length!==(mode==="words"?2:1)||mode==="words"&&!/^[0-9a-f]{16}$/.test(word??""))throw Error("sqlite-oracle words <binary64-hex> | edit [--file <path>]");
  const db=Database.deserialize(readFileSync(file??0),{safeIntegers:true}),quote=(name:string)=>'"'+name.replaceAll('"','""')+'"';
  try{
   if(db.query<{integrity_check:string},[]>("PRAGMA integrity_check").get()?.integrity_check!=="ok"||db.query("PRAGMA foreign_key_check").all().length)throw Error("Independent SQLite file integrity refused");
   const tables=db.query<{name:string},[]>("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all();
   const counts=JSON.parse(readFileSync(resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/📏️counts/🔣️.json"),"utf8"))as{tableCount:number;totalRows:number;tables:Record<string,number>};
   if(tables.length!==counts.tableCount)throw Error("Independent SQLite table cardinality");
   let totalRows=0,populatedWords=0,declaredRoles=0,populatedRoles=0;
   for(const{name}of tables){const n=db.query<{n:bigint},[]>("SELECT count(*) AS n FROM "+quote(name)).get()!.n;if(n!==BigInt(counts.tables[name]??-1))throw Error("Independent SQLite row cardinality "+name);totalRows+=Number(n)}
   if(totalRows!==counts.totalRows)throw Error("Independent SQLite total row cardinality");
   if(mode==="words"){
    const expected=BigInt("0x"+word!),exponent=(expected>>52n)&2047n,mantissa=expected&4503599627370495n,kind=exponent!==2047n?"finite":mantissa!==0n?"nan":expected>>63n?"negativeInfinity":"positiveInfinity";
    const bytes=new DataView(new ArrayBuffer(8));bytes.setBigUint64(0,expected,true);const finiteValue=kind==="finite"?bytes.getFloat64(0,true):undefined;
    for(const{name}of tables){
     const columns=db.query<{name:string},[]>("PRAGMA table_info("+quote(name)+")").all();
     for(const{name:bitsName}of columns)if(bitsName.endsWith("_bits")){
      const base=bitsName.slice(0,-5),className=base+"_class";declaredRoles++;let roleWords=0;
      if(!columns.some(c=>c.name===base)||!columns.some(c=>c.name===className))throw Error("Independent SQLite binary64 role shape");
      const rows=db.query<{q:number|bigint|null;w:bigint|null;k:string|null},[]>("SELECT "+quote(base)+" AS q,"+quote(bitsName)+" AS w,"+quote(className)+" AS k FROM "+quote(name)).all();
      for(const row of rows){
       if(row.w===null){if(row.q!==null||row.k!==null)throw Error("Independent absent binary64 role");continue}
       if(typeof row.w!=="bigint"||BigInt.asUintN(64,row.w)!==expected||row.k!==kind)throw Error("Independent original binary64 word or class disagrees "+name+"."+base);
       if(kind==="finite"){if(typeof row.q!=="number"&&typeof row.q!=="bigint"||Number(row.q)!==finiteValue)throw Error("Independent finite query disagrees")}else if(row.q!==null)throw Error("Independent nonfinite query is present");
       populatedWords++;roleWords++;
      }if(roleWords===0)throw Error("Independent binary64 role has no witness "+name+"."+base);populatedRoles++;
     }
    }
    if(declaredRoles!==184||populatedRoles!==184||populatedWords===0)throw Error("Independent word corpus misses declared float roles");
    process.stdout.write(JSON.stringify({schema:"bim.sqlite.oracle/v1",mode,word,tableCount:tables.length,totalRows,populatedWords,declaredRoles,populatedRoles,checked:true})+"\n");
   }else{
    if(db.query<{n:bigint},[]>("SELECT count(*) AS n FROM bim_project").get()!.n!==1n)throw Error("Independent project cardinality");
    const sql="UPDATE bim_property_value SET value=?,value_bits=?,value_class='finite' WHERE kind='Length' AND name='Length' AND set_id IN(SELECT s.id FROM bim_property_set s JOIN bim_property_element e ON e.id=s.element_id WHERE s.name='set/λ' AND e.map_key='external/element')";
    const bytes=new DataView(new ArrayBuffer(8));bytes.setFloat64(0,9.75,true);
    const changed=db.run(sql,[9.75,bytes.getBigInt64(0,true)]);if(changed.changes!==1)throw Error("Independent Length cardinality");
    db.run("UPDATE bim_project SET name=?",["SQLite edited BIM"]);const bytesOut=db.serialize();if(file===undefined)process.stdout.write(bytesOut);else{writeFileSync(file,bytesOut);process.stdout.write(JSON.stringify({schema:"bim.sqlite.oracle/v1",mode,tableCount:tables.length,totalRows,checked:true})+"\n")}
   }
  }finally{db.close()}
 }
}

const PLATFORM = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts";
const OWNER = "🏙️bim";

/** 💰️ Runs the independent costing implementation against its language-neutral and decimal arithmetic witnesses. */
class CostTests extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("test-costs-source takes no arguments");
    await runBudgetedTestCommand(process.execPath, ["test", resolve(this.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💰️costs/🧪️tests/🟦️.ts")], {cwd: this.repoRoot, budgetMs: testLevelBudgetMs()});
  }
}

/** 📏️ Checks the defining Source schema and owning provider without publishing artifacts. */
class SqliteVerify extends BundleScript { run(args:string[]):void{if(args.length!==1||args[0]!=="source")throw Error("verify-snapshot-sqlite source");const owner=resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any");runCmd("bun",[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--skipLibCheck",resolve(owner,"🧬️schema/🟦️.ts"),resolve(owner,"🧬️schema/📸️snapshot/🟦️.ts"),resolve(owner,"🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts"),resolve(owner,"🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"),resolve(this.root,"📜️script.ts")],{cwd:this.repoRoot});}}

/** 🧪️ Runs one platform role over every case the BIM plugin owns; `--case <slug>` narrows to one. @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts */
function platformRole(role: "oracle" | "subject" | "parity"): new (root: string, repoRoot: string) => BundleScript {
  return class extends BundleScript {
    async run(args: string[]): Promise<void> {
      const { level, rest } = resolveTestLevel(args);
      await runBudgetedTestCommand(process.execPath, [PLATFORM, role, level, "--owner", OWNER, ...rest], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs() });
    }
  };
}

await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-bim-model", {
  commands: { "verify-snapshot-sqlite": SqliteVerify, "sqlite-oracle": SqliteOracle, "test-costs-source": CostTests },
  snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"],
  testCommands: { oracle: platformRole("oracle"), subject: platformRole("subject"), parity: platformRole("parity") },
});
