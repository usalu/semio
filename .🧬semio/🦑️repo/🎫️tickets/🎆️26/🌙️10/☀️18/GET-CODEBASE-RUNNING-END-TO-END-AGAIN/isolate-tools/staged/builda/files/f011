import {fileURLToPath} from "node:url";
import {acquireResourceLease} from "@semio-tech/process/leases";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import {readFileSync,rmSync,readdirSync} from "node:fs";
import {dirname,join} from "node:path";
import {randomBytes} from "node:crypto";
import {withBuildTestInvocationV1,registerIsolatedBuildTestV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {repositoryBuildReportNameV1,withRepositoryBuildReportV1,type RepositoryBuildReportV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "../../🧪️tests/🧫️fixtures/🕸️graph/🔣️.json" with {type:"json"};
import{declareLibraryBuildTestEnvelopeV1}from"../../../🧪️testing/🪪️native-birth/🟦️.ts";
import{privateRepositoryWorkspaceV1}from"../../../🧪️testing/🧫️private-repository/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1();
declareLibraryBuildTestEnvelopeV1({suite:"build-reporting"});

test("build report names resolve only inside the finite diagnostics artifact owner",()=>{const database=new Database(":memory:"),validate=new Ajv({strict:false}).compile(schema);try{for(const row of fixture.reportPaths){const oracle=database.query("SELECT instr(?, '/')=0 AND instr(?, char(92))=0 AND ? LIKE '%.json' AND substr(?,1,1)<>'.' AS valid").get(row.requested,row.requested,row.requested,row.requested) as {valid:number};expect(Boolean(oracle.valid)).toBe(row.valid);expect(validate(row.requested)).toBe(row.valid);if(row.valid)expect(repositoryBuildReportNameV1(row.requested)).toBe(row.requested);else expect(()=>repositoryBuildReportNameV1(row.requested)).toThrow();}}finally{database.close();}});


test("build reports publish actual bounded complete bytes under original diagnostics custody",async()=>withBuildTestInvocationV1(async original=>{const name="law-"+randomBytes(4).toString("hex")+".json";await withRepositoryBuildReportV1(repository.root,name,fixture.reportValue,original,async artifact=>{const path=artifact.path;try{expect(path).toBe(join(repository.root,".cache/b/o/m/reports",name));const bytes=readFileSync(path,"utf8"),database=new Database(":memory:");try{expect((database.query("SELECT json(?) AS value").get(bytes) as {value:string}).value).toBe(JSON.stringify(fixture.reportValue));}finally{database.close();}expect(readdirSync(dirname(path)).filter(file=>file.startsWith(name+"~"))).toEqual([]);}finally{}});rmSync(join(repository.root,".cache/b/o/m/reports",name),{force:true});}));


test("build reports refuse values without JSON representation before storage admission",async()=>{await expect(withRepositoryBuildReportV1(repository.root,"undefined.json",undefined,undefined as never,async()=>{})).rejects.toThrow("JSON build report value required");await expect(withRepositoryBuildReportV1(repository.root,"bigint.json",1n,undefined as never,async()=>{})).rejects.toThrow();});


test("build report callback holds exact immutable bytes until its reader settles",async()=>withBuildTestInvocationV1(async original=>{
 const name="reader-"+randomBytes(4).toString("hex")+".json",control=scriptCompilerAdmissionControlV1(original,"report:retirement-proof",30000);let entered=false,retained:RepositoryBuildReportV1|undefined;
 await withRepositoryBuildReportV1(repository.root,name,fixture.reportValue,original,async artifact=>{entered=true;retained=artifact;await artifact.recheck();expect(readFileSync(artifact.path,"utf8")).toBe(JSON.stringify(fixture.reportValue,null,2)+"\n");let blocked=false;try{const exclusive=await acquireResourceLease({directory:join(repository.root,".cache/b/l/m/data"),resource:"repository-build-reports",mode:"exclusive",signal:control.signal,control,timeoutMs:0});exclusive.release();}catch(error){if(!String(error).includes("Resource lease timed out"))throw error;blocked=true;}expect(blocked).toBe(true);});
 expect(entered).toBe(true);await expect(retained!.recheck()).rejects.toThrow("Original report callback released");rmSync(join(repository.root,".cache/b/o/m/reports",name),{force:true});
}));

registerIsolatedBuildTestV1(fileURLToPath(import.meta.url),"build report callback failures release original publication custody",60000,checkpoint=>withBuildTestInvocationV1(async original=>{const name="failure-"+randomBytes(4).toString("hex")+".json",control=scriptCompilerAdmissionControlV1(original,"report:error-release",30000),failure=Error("Original report reader failure");try{await expect(withRepositoryBuildReportV1(repository.root,name,fixture.reportValue,original,async()=>{throw failure;})).rejects.toBe(failure);const successor=await acquireResourceLease({directory:join(repository.root,".cache/b/l/m/data"),resource:"repository-build-reports",mode:"exclusive",signal:control.signal,control,timeoutMs:0});successor.release();expect(readdirSync(join(repository.root,".cache/b/o/m/reports")).filter(file=>file.startsWith(name+"~"))).toEqual([]);checkpoint();}finally{rmSync(join(repository.root,".cache/b/o/m/reports",name),{force:true});}}),/Original report reader failure/u);
