#!/usr/bin/env bun
/** 🏢️ BIM model owning SQLite and Rust package commands, plus the repository test-platform roles (`test oracle|subject|parity [level]`). */
import { runArtifactRustPackageMain,runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import {runCmd} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import {BundleScript,scriptInvocationBudget,type ScriptInvocation} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import {resolve} from "node:path";
import { runBudgetedTestCommand } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { resolveTestLevel, testLevelBudgetMs } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎚️budget/🟦️.ts";

import {readFileSync,writeFileSync} from "node:fs";
import {Database} from "bun:sqlite";

/** 🧩️ Qualifies the complete authored model input and actual owning semantic projection. */
class CompleteSqliteSource extends BundleScript{
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("test-complete-sqlite-source accepts no arguments");
  const source=resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧩️complete/🟦️.ts");
  const sources=[source,resolve(source,"../../🟦️.ts")];
  await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",...sources],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});
  await runBudgetedTestCommand(process.execPath,["test",...sources],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true});
 }
}

/** 🦀️ Qualifies only the complete semantic laws in the actual mounted BIM owner. */
class CompleteSqliteNative extends CompleteSqliteSource {
 async run(args:string[]):Promise<void>{await super.run(args);await runArtifactRustTests("semio-s-artifact-bim-model",this.repoRoot,["--lib","sqlite_snapshot_bim_complete_","--no-fail-fast"],this.invocation);}
}

/** 🔎️ Independent SQLite interpretation and authoritative edits over original public IO files. */
class SqliteOracle extends BundleScript {
 run(input:string[]):void{
  const file=input.length>=3&&input.at(-2)==="--file"?input.at(-1):undefined,args=file===undefined?input:input.slice(0,-2),[mode,word]=args;
  if(mode!=="words"&&mode!=="edit"||args.length!==(mode==="words"?2:1)||mode==="words"&&!/^[0-9a-f]{16}$/.test(word??""))throw Error("sqlite-oracle words <binary64-hex> | edit [--file <path>]");
  const db=Database.deserialize(readFileSync(file??0),{safeIntegers:true}),quote=(name:string)=>'"'+name.replaceAll('"','""')+'"';
  try{
   if(db.query<{integrity_check:string},[]>("PRAGMA integrity_check").get()?.integrity_check!=="ok"||db.query("PRAGMA foreign_key_check").all().length)throw Error("Independent SQLite file integrity refused");
   const tables=db.query<{name:string},[]>("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all();
   const counts=JSON.parse(readFileSync(resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/📏️counts/🔣️.json"),"utf8"))as{tableCount:number;totalRows:number;tables:Record<string,number>};
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
    if(declaredRoles!==231||populatedRoles!==231||populatedWords===0)throw Error("Independent word corpus misses declared float roles");
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
function platformRole(role: "oracle" | "subject" | "parity"): new (root: string, repoRoot: string, invocation:ScriptInvocation) => BundleScript {
  return class extends BundleScript {
    async run(args: string[]): Promise<void> {
      const { level, rest } = resolveTestLevel(args);
      await runBudgetedTestCommand(process.execPath, [PLATFORM, role, level, "--owner", OWNER, ...rest], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs() });
    }
  };
}

await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-bim-model", { packages: ["semio-s-artifact-bim-model", "semio-s-artifact-bim-model-authored", "semio-s-artifact-bim-model-bim", "semio-s-artifact-bim-model-building-elements", "semio-s-artifact-bim-model-cascade-restore", "semio-s-artifact-bim-model-components", "semio-s-artifact-bim-model-coordinate-tests", "semio-s-artifact-bim-model-coordination", "semio-s-artifact-bim-model-curtain-walls", "semio-s-artifact-bim-model-diff", "semio-s-artifact-bim-model-dispatch-context", "semio-s-artifact-bim-model-entities", "semio-s-artifact-bim-model-examples", "semio-s-artifact-bim-model-families-tests", "semio-s-artifact-bim-model-family-tests", "semio-s-artifact-bim-model-formula", "semio-s-artifact-bim-model-gbxml", "semio-s-artifact-bim-model-gestures", "semio-s-artifact-bim-model-gestures-tests", "semio-s-artifact-bim-model-ifc", "semio-s-artifact-bim-model-ifc-2", "semio-s-artifact-bim-model-incremental-tests", "semio-s-artifact-bim-model-inferences", "semio-s-artifact-bim-model-interaction", "semio-s-artifact-bim-model-io", "semio-s-artifact-bim-model-kit", "semio-s-artifact-bim-model-modify", "semio-s-artifact-bim-model-move-storey", "semio-s-artifact-bim-model-mut-any", "semio-s-artifact-bim-model-mut-any-10", "semio-s-artifact-bim-model-mut-any-100", "semio-s-artifact-bim-model-mut-any-101", "semio-s-artifact-bim-model-mut-any-102", "semio-s-artifact-bim-model-mut-any-103", "semio-s-artifact-bim-model-mut-any-104", "semio-s-artifact-bim-model-mut-any-105", "semio-s-artifact-bim-model-mut-any-106", "semio-s-artifact-bim-model-mut-any-107", "semio-s-artifact-bim-model-mut-any-108", "semio-s-artifact-bim-model-mut-any-109", "semio-s-artifact-bim-model-mut-any-11", "semio-s-artifact-bim-model-mut-any-110", "semio-s-artifact-bim-model-mut-any-111", "semio-s-artifact-bim-model-mut-any-112", "semio-s-artifact-bim-model-mut-any-113", "semio-s-artifact-bim-model-mut-any-114", "semio-s-artifact-bim-model-mut-any-115", "semio-s-artifact-bim-model-mut-any-116", "semio-s-artifact-bim-model-mut-any-117", "semio-s-artifact-bim-model-mut-any-118", "semio-s-artifact-bim-model-mut-any-119", "semio-s-artifact-bim-model-mut-any-12", "semio-s-artifact-bim-model-mut-any-120", "semio-s-artifact-bim-model-mut-any-121", "semio-s-artifact-bim-model-mut-any-122", "semio-s-artifact-bim-model-mut-any-123", "semio-s-artifact-bim-model-mut-any-124", "semio-s-artifact-bim-model-mut-any-125", "semio-s-artifact-bim-model-mut-any-126", "semio-s-artifact-bim-model-mut-any-127", "semio-s-artifact-bim-model-mut-any-128", "semio-s-artifact-bim-model-mut-any-129", "semio-s-artifact-bim-model-mut-any-13", "semio-s-artifact-bim-model-mut-any-130", "semio-s-artifact-bim-model-mut-any-131", "semio-s-artifact-bim-model-mut-any-132", "semio-s-artifact-bim-model-mut-any-133", "semio-s-artifact-bim-model-mut-any-134", "semio-s-artifact-bim-model-mut-any-135", "semio-s-artifact-bim-model-mut-any-136", "semio-s-artifact-bim-model-mut-any-137", "semio-s-artifact-bim-model-mut-any-138", "semio-s-artifact-bim-model-mut-any-139", "semio-s-artifact-bim-model-mut-any-14", "semio-s-artifact-bim-model-mut-any-140", "semio-s-artifact-bim-model-mut-any-141", "semio-s-artifact-bim-model-mut-any-142", "semio-s-artifact-bim-model-mut-any-143", "semio-s-artifact-bim-model-mut-any-144", "semio-s-artifact-bim-model-mut-any-145", "semio-s-artifact-bim-model-mut-any-146", "semio-s-artifact-bim-model-mut-any-147", "semio-s-artifact-bim-model-mut-any-148", "semio-s-artifact-bim-model-mut-any-149", "semio-s-artifact-bim-model-mut-any-15", "semio-s-artifact-bim-model-mut-any-150", "semio-s-artifact-bim-model-mut-any-151", "semio-s-artifact-bim-model-mut-any-152", "semio-s-artifact-bim-model-mut-any-153", "semio-s-artifact-bim-model-mut-any-154", "semio-s-artifact-bim-model-mut-any-155", "semio-s-artifact-bim-model-mut-any-156", "semio-s-artifact-bim-model-mut-any-157", "semio-s-artifact-bim-model-mut-any-158", "semio-s-artifact-bim-model-mut-any-159", "semio-s-artifact-bim-model-mut-any-16", "semio-s-artifact-bim-model-mut-any-160", "semio-s-artifact-bim-model-mut-any-161", "semio-s-artifact-bim-model-mut-any-162", "semio-s-artifact-bim-model-mut-any-163", "semio-s-artifact-bim-model-mut-any-164", "semio-s-artifact-bim-model-mut-any-165", "semio-s-artifact-bim-model-mut-any-166", "semio-s-artifact-bim-model-mut-any-167", "semio-s-artifact-bim-model-mut-any-168", "semio-s-artifact-bim-model-mut-any-169", "semio-s-artifact-bim-model-mut-any-17", "semio-s-artifact-bim-model-mut-any-170", "semio-s-artifact-bim-model-mut-any-171", "semio-s-artifact-bim-model-mut-any-172", "semio-s-artifact-bim-model-mut-any-173", "semio-s-artifact-bim-model-mut-any-174", "semio-s-artifact-bim-model-mut-any-175", "semio-s-artifact-bim-model-mut-any-176", "semio-s-artifact-bim-model-mut-any-177", "semio-s-artifact-bim-model-mut-any-178", "semio-s-artifact-bim-model-mut-any-179", "semio-s-artifact-bim-model-mut-any-18", "semio-s-artifact-bim-model-mut-any-180", "semio-s-artifact-bim-model-mut-any-181", "semio-s-artifact-bim-model-mut-any-182", "semio-s-artifact-bim-model-mut-any-183", "semio-s-artifact-bim-model-mut-any-184", "semio-s-artifact-bim-model-mut-any-185", "semio-s-artifact-bim-model-mut-any-186", "semio-s-artifact-bim-model-mut-any-187", "semio-s-artifact-bim-model-mut-any-188", "semio-s-artifact-bim-model-mut-any-189", "semio-s-artifact-bim-model-mut-any-19", "semio-s-artifact-bim-model-mut-any-190", "semio-s-artifact-bim-model-mut-any-2", "semio-s-artifact-bim-model-mut-any-20", "semio-s-artifact-bim-model-mut-any-21", "semio-s-artifact-bim-model-mut-any-22", "semio-s-artifact-bim-model-mut-any-23", "semio-s-artifact-bim-model-mut-any-24", "semio-s-artifact-bim-model-mut-any-25", "semio-s-artifact-bim-model-mut-any-26", "semio-s-artifact-bim-model-mut-any-27", "semio-s-artifact-bim-model-mut-any-28", "semio-s-artifact-bim-model-mut-any-29", "semio-s-artifact-bim-model-mut-any-3", "semio-s-artifact-bim-model-mut-any-30", "semio-s-artifact-bim-model-mut-any-31", "semio-s-artifact-bim-model-mut-any-32", "semio-s-artifact-bim-model-mut-any-33", "semio-s-artifact-bim-model-mut-any-34", "semio-s-artifact-bim-model-mut-any-35", "semio-s-artifact-bim-model-mut-any-36", "semio-s-artifact-bim-model-mut-any-37", "semio-s-artifact-bim-model-mut-any-38", "semio-s-artifact-bim-model-mut-any-39", "semio-s-artifact-bim-model-mut-any-4", "semio-s-artifact-bim-model-mut-any-40", "semio-s-artifact-bim-model-mut-any-41", "semio-s-artifact-bim-model-mut-any-42", "semio-s-artifact-bim-model-mut-any-43", "semio-s-artifact-bim-model-mut-any-44", "semio-s-artifact-bim-model-mut-any-45", "semio-s-artifact-bim-model-mut-any-46", "semio-s-artifact-bim-model-mut-any-47", "semio-s-artifact-bim-model-mut-any-48", "semio-s-artifact-bim-model-mut-any-49", "semio-s-artifact-bim-model-mut-any-5", "semio-s-artifact-bim-model-mut-any-50", "semio-s-artifact-bim-model-mut-any-51", "semio-s-artifact-bim-model-mut-any-52", "semio-s-artifact-bim-model-mut-any-53", "semio-s-artifact-bim-model-mut-any-54", "semio-s-artifact-bim-model-mut-any-55", "semio-s-artifact-bim-model-mut-any-56", "semio-s-artifact-bim-model-mut-any-57", "semio-s-artifact-bim-model-mut-any-58", "semio-s-artifact-bim-model-mut-any-59", "semio-s-artifact-bim-model-mut-any-6", "semio-s-artifact-bim-model-mut-any-60", "semio-s-artifact-bim-model-mut-any-61", "semio-s-artifact-bim-model-mut-any-62", "semio-s-artifact-bim-model-mut-any-63", "semio-s-artifact-bim-model-mut-any-64", "semio-s-artifact-bim-model-mut-any-65", "semio-s-artifact-bim-model-mut-any-66", "semio-s-artifact-bim-model-mut-any-67", "semio-s-artifact-bim-model-mut-any-68", "semio-s-artifact-bim-model-mut-any-69", "semio-s-artifact-bim-model-mut-any-7", "semio-s-artifact-bim-model-mut-any-70", "semio-s-artifact-bim-model-mut-any-71", "semio-s-artifact-bim-model-mut-any-72", "semio-s-artifact-bim-model-mut-any-73", "semio-s-artifact-bim-model-mut-any-74", "semio-s-artifact-bim-model-mut-any-75", "semio-s-artifact-bim-model-mut-any-76", "semio-s-artifact-bim-model-mut-any-77", "semio-s-artifact-bim-model-mut-any-78", "semio-s-artifact-bim-model-mut-any-79", "semio-s-artifact-bim-model-mut-any-8", "semio-s-artifact-bim-model-mut-any-80", "semio-s-artifact-bim-model-mut-any-81", "semio-s-artifact-bim-model-mut-any-82", "semio-s-artifact-bim-model-mut-any-83", "semio-s-artifact-bim-model-mut-any-84", "semio-s-artifact-bim-model-mut-any-85", "semio-s-artifact-bim-model-mut-any-86", "semio-s-artifact-bim-model-mut-any-87", "semio-s-artifact-bim-model-mut-any-88", "semio-s-artifact-bim-model-mut-any-89", "semio-s-artifact-bim-model-mut-any-9", "semio-s-artifact-bim-model-mut-any-90", "semio-s-artifact-bim-model-mut-any-91", "semio-s-artifact-bim-model-mut-any-92", "semio-s-artifact-bim-model-mut-any-93", "semio-s-artifact-bim-model-mut-any-94", "semio-s-artifact-bim-model-mut-any-95", "semio-s-artifact-bim-model-mut-any-96", "semio-s-artifact-bim-model-mut-any-97", "semio-s-artifact-bim-model-mut-any-98", "semio-s-artifact-bim-model-mut-any-99", "semio-s-artifact-bim-model-plan-linework", "semio-s-artifact-bim-model-quantities", "semio-s-artifact-bim-model-ramp-runs", "semio-s-artifact-bim-model-schema", "semio-s-artifact-bim-model-sheets", "semio-s-artifact-bim-model-snapshot", "semio-s-artifact-bim-model-spaces", "semio-s-artifact-bim-model-storey-levels", "semio-s-artifact-bim-model-svg", "semio-s-artifact-bim-model-tests", "semio-s-artifact-bim-model-tests-2", "semio-s-artifact-bim-model-tests-3", "semio-s-artifact-bim-model-tests-4", "semio-s-artifact-bim-model-tests-5", "semio-s-artifact-bim-model-utilities-tests", "semio-s-artifact-bim-model-v1", "semio-s-artifact-bim-model-viewer", "semio-s-artifact-bim-model-wall-geometry", "semio-s-artifact-bim-model-wall-layout"],
  commands: { "test-complete-sqlite-source":CompleteSqliteSource,"test-complete-sqlite-native":CompleteSqliteNative, "verify-snapshot-sqlite": SqliteVerify, "sqlite-oracle": SqliteOracle, "test-costs-source": CostTests },
  snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"],
  testCommands: { oracle: platformRole("oracle"), subject: platformRole("subject"), parity: platformRole("parity") },
});
