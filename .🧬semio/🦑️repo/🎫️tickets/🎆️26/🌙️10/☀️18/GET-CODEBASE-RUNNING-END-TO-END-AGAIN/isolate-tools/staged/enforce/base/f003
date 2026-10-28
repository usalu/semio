import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {mkdtempSync,readFileSync,rmSync,mkdirSync} from "node:fs";
import {join} from "node:path";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "./🧬️.json" with {type:"json"};
import {repositoryCompilerReservationV1} from "../🟦️.ts";
import {receiveScriptProcessInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import {openNativeImageStorageV1,retireNativeImageStorageV1} from "@semio-tech/process/artifacts/native-build/storage/image";
import {recoverNativeStorageReservationV1,nativeStorageRetirementPortV1} from "@semio-tech/process/artifacts/native-build/storage";
import {ordinaryNativeStoragePortV1} from "@semio-tech/process/artifacts/native-build/storage";
import type {NativeImageStorageV1} from "@semio-tech/process/artifacts/native-build/storage/platform/contract";

test("neutral unsupported build modes refuse before storage observation",async()=>{
 const control={signal:new AbortController().signal,remainingMilliseconds:()=>30000,publish:async()=>{},yieldContinuation:async()=>{}},root=process.env.SEMIO_TEST_ARTIFACT_DIR!;let observations=0;
 const image:NativeImageStorageV1={root,image:join(root,"unused"),maximumPhysicalBytes:1,filesystemBytes:1,port:ordinaryNativeStoragePortV1(),withExclusiveMaintenance:async()=>{throw Error("provider touched");},recheck:async()=>{observations++;throw Error("provider touched");},close:async()=>{}};
 for(const mode of fixture.rejectedModes){expect(new Ajv({strict:true}).compile({enum:["prototype","release"]})(mode)).toBe(false);await expect(repositoryCompilerReservationV1(image,{version:1,root,leaseDirectory:join(process.cwd(),".cache/b/l/m/data"),capacityBytes:1,maximumWorkspaceBytes:1,maximumArtifactBytes:1},mode as "prototype",control)).rejects.toThrow(/[Bb]uild mode/u);}expect(observations).toBe(0);
});

test("neutral repeated compiler stores recover one charged working set under real bounded backing",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket required");expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const parent=mkdtempSync(join(artifacts,"cs-")),directory=join(parent,"v"),leaseDirectory=join(process.cwd(),".cache/b/l/m/data");
 await receiveScriptProcessInvocation(process.env,async original=>{
  const control=scriptCompilerAdmissionControlV1(original,"repo:compiler:reuse-law",30000),imageOptions={directory,leaseDirectory,filesystemBytes:33554432,maximumPhysicalBytes:34603008,control};const image=await openNativeImageStorageV1(imageOptions),policy={version:1 as const,root:image.root,leaseDirectory,capacityBytes:34603008,maximumWorkspaceBytes:8388608,maximumArtifactBytes:8388608},ids:string[]=[];
  try{const db=new Database(":memory:");db.exec("CREATE TABLE stores(mode TEXT PRIMARY KEY, reservations INTEGER NOT NULL)");
   try{for(const row of fixture.cases){let id:string|undefined;for(let i=0;i<row.repeat;i++){const store=await repositoryCompilerReservationV1(image,policy,row.mode as "prototype"|"release",control);id??=store.allocation.id;expect(store.allocation.id).toBe(id);await store.recheck();if(row.loseReceipt&&i===0)rmSync(join(image.root,"compiler-"+row.mode+".json"));}ids.push(id!);db.query("INSERT INTO stores VALUES(?,?)").run(row.mode,row.reservations);}
    const events=readFileSync(join(image.root,"storage-events.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));for(const row of fixture.cases)expect(events.filter(event=>event.type==="reserved"&&event.allocation.owner==="repo:compiler:"+row.mode).length).toBe((db.query("SELECT reservations FROM stores WHERE mode=?").get(row.mode) as {reservations:number}).reservations);expect(events.filter(event=>event.type==="committed").length).toBe(2);
   }finally{db.close();}
  }finally{const port=nativeStorageRetirementPortV1(image.port,async allocation=>{const mode=allocation.owner.slice("repo:compiler:".length);rmSync(join(image.root,"compiler-"+mode+".json"),{force:true});});for(const id of ids)await(await recoverNativeStorageReservationV1({policy,control,port,id})).retire();await image.close();await retireNativeImageStorageV1({directory,leaseDirectory,filesystemBytes:33554432,maximumPhysicalBytes:34603008,control});rmSync(parent,{recursive:true});}
 });
},35000);

