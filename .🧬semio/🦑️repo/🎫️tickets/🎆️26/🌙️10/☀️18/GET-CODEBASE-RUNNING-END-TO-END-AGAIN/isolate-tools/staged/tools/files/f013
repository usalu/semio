import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};
const repository=await privateRepositoryWorkspaceV1();
test("tooling headroom returns closed refusal data before publication effects",async()=>{const api=await import("../🟦️.ts"),database=new Database(":memory:");try{for(const row of corpus.observations){const{admitted,...facts}=row;expect(new Ajv({strict:true}).validate(schema.definitions.Observation,facts)).toBe(true);const oracle=database.query<{admitted:number},[number,number]>("select ?1>=?2 admitted").get(row.availableBytes,row.requiredBytes)!;expect(Boolean(oracle.admitted)).toBe(admitted);expect(api.toolingHeadroomAvailableV1(facts)).toBe(admitted);}expect(()=>api.requireToolingPublicationValueV1({kind:"tooling-capacity-refused"}as unknown as import("../🟦️.ts").ToolingHeadroomRefusalV1)).toThrow("Privately issued");}finally{database.close();}});

import{withBuildTestInvocationV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import{scriptCompilerAdmissionControlV1}from"@semio-tech/process/compiler-capacity";
import{scriptInvocationEnvironment}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{captureOwnedCommandV1}from"@semio-tech/process/execution";
import{fileURLToPath}from"node:url";
import{privateRepositoryWorkspaceV1}from"../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
test.skipIf(process.platform!=="darwin")("one actual Original settles private physical refusal recovery and drain outcomes",async()=>withBuildTestInvocationV1(async original=>{const control=scriptCompilerAdmissionControlV1(original,"tooling:isolated-recovery",110000);for(const row of corpus.cases){const output=await captureOwnedCommandV1(process.execPath,[fileURLToPath(new URL("./📜️script.ts",import.meta.url)),row.name],process.cwd(),{control,env:scriptInvocationEnvironment(original,process.env),output:{maximumBytes:1048576,maximumLines:8192}}),result=JSON.parse(output.trim().split("\n").at(-1)!);expect(result.rootSucceeded,row.name+JSON.stringify(result)).toBe(row.success);expect(result.attempts).toBe(row.attempts);expect(result.recoveries).toBe(row.recoveries);expect(result.events.at(-1).startsWith(row.success?"physical-cleanup-settled":"root-refused:")).toBe(true);if(row.name==="pressure-success"){expect(result.refusalShape).toBe(true);expect(result.oracle).toBe("held");expect(result.effects).toBe(1);expect(result.events.indexOf("accepted-drained:1")).toBeLessThan(result.events.indexOf("shared-closed:1"));expect(result.events.indexOf("shared-closed:1")).toBeLessThan(result.events.indexOf("maintenance"));}if(!row.success)expect(result.effects).toBe(row.name==="replay"?1:0);}}),115000);
