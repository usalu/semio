import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";

const source=readFileSync(fileURLToPath(new URL("../../🟨️.mjs",import.meta.url)),"utf8"),glue=readFileSync(fileURLToPath(new URL("../../🏊️project-farm/🟨️.mjs",import.meta.url)),"utf8");

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

test("the workers boot while the owners are still being discovered, and the farm they boot into is the one that is opened",()=>{
 const start=source.indexOf("startProjectFarmV1({"),discovery=source.indexOf("await catalogBuildContributionsV1("),open=source.indexOf("openProjectFarmV1({");
 expect(start).toBeGreaterThan(0);expect(discovery).toBeGreaterThan(start);expect(open).toBeGreaterThan(discovery);
 expect(source.slice(start-60,start)).toContain("options?.ownerAdmission?undefined:");
 expect(source.slice(open,open+400)).toContain("started");
 expect(source.match(/startProjectFarmV1\(/gu)?.length).toBe(1);
});

test("every project is inferred inside one accounting batch, and the cargo receipts the projects deferred are refreshed once, in order, after the last of them",()=>{
 const batch=source.indexOf("const inferProject=({result,name,json})=>withBuildGraphOptionalBatchV1(async()=>{"),end=source.indexOf("},receipts);"),loops=source.indexOf("await Promise.all(workers)"),refresh=source.indexOf("await refreshCargoReceiptsV1(workspaceRoot,receipts,context);");
 expect(batch).toBeGreaterThan(0);expect(end).toBeGreaterThan(batch);expect(loops).toBeGreaterThan(end);expect(refresh).toBeGreaterThan(loops);
 expect(source.match(/withBuildGraphOptionalBatchV1\(/gu)?.length).toBe(1);
 expect(source.match(/refreshCargoReceiptsV1\(/gu)?.length).toBe(1);
});

test("the farm accounts what a worker wrote before the facts join the project and before the value is used, and reserves the bytes of a job before it is sent",()=>{
 const adopt=glue.indexOf("adopt(reply);"),replay=glue.indexOf("replayDetachedProjectInferenceV1(table.rows.decode(reply.rowIds)"),value=glue.indexOf("return preimagedProjectValueV1(reply.image);");
 expect(adopt).toBeGreaterThan(0);expect(replay).toBeGreaterThan(adopt);expect(value).toBeGreaterThan(replay);
 expect(glue).toContain("table.rows.learn(reply.freshRows.map(");
 expect(glue.indexOf("table.rows.learn(reply.freshRows.map(")).toBeLessThan(glue.indexOf("adopt(reply);"));
 const grant=glue.indexOf("grantBuildGraphWorkerBytesV1(workspaceRoot, "),send=glue.indexOf('farm.infer("project"');
 expect(grant).toBeGreaterThan(0);expect(send).toBeGreaterThan(grant);
 expect(glue).toContain("opened.release();");
 expect(glue.match(/pool\.open\(\)/gu)?.length).toBe(1);
 expect(glue).toContain("for (const manifest of reply.receipts) deferBuildGraphReceiptV1(manifest);");
 const lazy=glue.indexOf("const open = () => opening ??="),pool=glue.indexOf("await pool.open()");
 expect(lazy).toBeGreaterThan(0);expect(pool).toBeGreaterThan(lazy);
 expect(glue).not.toContain("adoptStaged");
});

test("only the blocks the quota admitted count as present, and the accounting alone decides about duplicates",()=>{
 const adopt=glue.indexOf("adoptBuildGraphWorkerLeavesV1(workspaceRoot, reply.leaves)"),presence=glue.indexOf("presence.add(join(readRoot, name))");
 expect(adopt).toBeGreaterThan(0);expect(presence).toBeGreaterThan(adopt);
 expect(glue).toContain("for (const name of adoption.accepted)");
 expect(glue).not.toContain("accounted");
});

test("the byte grant of a job is given back after its leaves were adopted, never before",()=>{
 const adopt=glue.indexOf("      adopt(reply);\n"),release=glue.indexOf("const release = grants.get(reply);");
 expect(adopt).toBeGreaterThan(0);expect(release).toBeGreaterThan(adopt);
 expect(glue).toContain("if (release !== undefined) { grants.set(reply, release); outstanding.add(release); }");
 expect(glue).toContain("for (const release of outstanding) release();");
 const infer=glue.slice(glue.indexOf("async infer(job)"),glue.indexOf("replay(reply)"));
 expect(infer).not.toContain("finally");
 expect(infer.match(/release\?\.\(\)/gu)?.length).toBe(2);
});

test("a job without an answer is reported with its reason, counted and capped, and never turns into a silent serial run",()=>{
 expect(glue).toContain('fallback(job.name, outcome.fallback ?? "failure: " + outcome.failure.message); return undefined;');
 const fallback=glue.slice(glue.indexOf("const fallback = (name, reason) => {"),glue.indexOf("return Object.freeze({"));
 expect(fallback).toContain("traffic.fallbacks++");expect(fallback).toContain('if (traffic.fallbacks > policy.maximumFallbacks) throw Error(renderGraphFarmMessagesV1("fallbackLimit"');
 expect(fallback).toContain('say("fallback", fields)');expect(fallback.indexOf("throw")).toBeLessThan(fallback.indexOf('say("fallback"'));
});

test("the parser records the parent sends and keeps are bounded by the declared policy, as a total over all workers",()=>{
 expect(glue).toContain("exportSourceAnalysisRecordsV1(policy.seedMegabytes * MEGABYTE)");
 expect(glue).toContain("recordTotal + row.bytes.byteLength > policy.recordTableMegabytes * MEGABYTE");
 expect(glue).toContain("recordTotal += row.bytes.byteLength;");
 expect(glue).not.toContain("table.recordBytes");
});
