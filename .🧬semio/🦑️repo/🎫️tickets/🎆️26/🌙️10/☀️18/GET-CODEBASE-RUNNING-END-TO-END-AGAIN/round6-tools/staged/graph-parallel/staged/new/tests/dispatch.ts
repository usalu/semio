import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";

const source=readFileSync(fileURLToPath(new URL("../../🟨️.mjs",import.meta.url)),"utf8");

test("the project inference body has one definition shared by the process and the workers, and the farm sits in front of it",()=>{
 expect(source.match(/projectWithDefaults\(/gu)?.length).toBe(1);
 const compute=source.indexOf("if(farm){const farmed=await farm.infer("),body=source.indexOf("return inferProjectBodyV1(projectEnvironment,");
 expect(compute).toBeGreaterThan(0);expect(body).toBeGreaterThan(compute);
 expect(source.slice(compute,body)).toContain("farm.replay(farmed)");
 expect(source.match(/inferProjectBodyV1\(/gu)?.length).toBe(2);
});

test("the farm is closed on every path, never serves an owner admission and is told the implementation it must share",()=>{
 expect(source).toContain(".finally(()=>farm?.close())");
 expect(source).toContain("options?.ownerAdmission?undefined:await openProjectFarmV1(");
 expect(source).toContain("identity:revisionIdentity.revision");
 expect(source.match(/openProjectFarmV1\(/gu)?.length).toBe(1);
});

test("the farm adopts what a worker staged before the facts join the project and before the value is used",()=>{
 const glue=readFileSync(fileURLToPath(new URL("../../🏊️project-farm/🟨️.mjs",import.meta.url)),"utf8");
 const adopt=glue.indexOf("adoptStagedGraphFilesV1(workspaceRoot, files)"),replay=glue.indexOf("replayDetachedProjectInferenceV1(table.rows.decode(reply.rowIds)"),value=glue.indexOf("return preimagedProjectValueV1(reply.image);");
 expect(adopt).toBeGreaterThan(0);expect(replay).toBeGreaterThan(adopt);expect(value).toBeGreaterThan(replay);
 expect(glue).toContain("table.rows.learn(reply.freshRows.map(");
 expect(glue.indexOf("table.rows.learn(reply.freshRows.map(")).toBeLessThan(glue.indexOf("adoptStagedGraphFilesV1(workspaceRoot, files)"));
 expect(glue).toContain("opened.capacity.release()");
 const lazy=glue.indexOf("const open = () => opening ??=");
 expect(lazy).toBeGreaterThan(0);expect(glue.indexOf("acquireBuildFarmCapacityV1({")).toBeGreaterThan(lazy);expect(glue.indexOf("createFarmV1({")).toBeGreaterThan(lazy);
});
