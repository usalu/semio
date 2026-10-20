import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import corpus from"../../../📦️artifacts/🏗️native-build/💾️storage/📜️journal/🧷️descendant/🧪️tests/🔣️.json";
import{nativeStorageStateV1,nativeStorageEventV1,type NativeStorageEventV1}from"../../../📦️artifacts/🏗️native-build/💾️storage/📜️journal/🟦️.ts";
import{persistentStateRecoveryV1,persistentStateChargeOwnerV1}from"../../📥️recovery/🟦️.ts";
import{persistentStatePolicyV1}from"../../📋️layout/🟦️.ts";

test("the half-born journal of the incident resumes its original charge instead of leaving an unfunded poison",()=>{
 const db=new Database(":memory:"),owner=persistentStateChargeOwnerV1(),policy=persistentStatePolicyV1();
 try{
  db.exec("CREATE TABLE expected(name TEXT PRIMARY KEY,action TEXT)");
  let checked=0;
  for(const row of corpus.cases){
   if(!row.recovery)continue;
   let state=nativeStorageStateV1();for(const event of row.journal)state=nativeStorageEventV1(state,event as NativeStorageEventV1,25769803776);
   const charge=state.allocations.find(item=>item.kind==="persistent-state");if(!charge)throw Error("Fixture persistent charge required");
   expect(charge.owner,row.name).toBe(owner);expect(charge.bytes,row.name).toBe(policy.chargeBytes);
   db.query("INSERT INTO expected VALUES(?,?)").run(row.name,row.recovery.expected);
   const observation={version:1,charge:{id:charge.id,owner:charge.owner,kind:charge.kind,bytes:charge.bytes,phase:charge.phase},binding:row.recovery.binding?charge.id:null,intent:row.recovery.intent?charge.id:null,directoryPresent:row.recovery.directoryPresent};
   expect(persistentStateRecoveryV1(observation),row.name).toBe((db.query("SELECT action FROM expected WHERE name=?").get(row.name)as{action:string}).action);checked++;
  }
  expect(checked).toBe(2);
 }finally{db.close();}
});
