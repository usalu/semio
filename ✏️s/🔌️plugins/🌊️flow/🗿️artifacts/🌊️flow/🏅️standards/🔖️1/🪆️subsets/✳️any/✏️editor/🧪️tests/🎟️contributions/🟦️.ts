import {existsSync as testingSchemaExists} from "node:fs";
import assert from "node:assert/strict";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";

/** 🧾️ The original contribution worker consumes actual effects before yielding or refusing. */
export function testFlowContributionsReceiver():void{
 const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎟️contributions/🔣️.json",import.meta.url),"utf8"));
assert.equal(testingSchemaExists(new URL("../../🧬️schema/🎟️contributions/🔣️.json",import.meta.url)),false,"Testing examples have no schema authority");
 const grantSchema=JSON.parse(readFileSync(new URL("../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json",import.meta.url),"utf8")),grant=new Ajv({strict:true,allErrors:true}).compile(grantSchema);let assertions=1;
 for(const row of fixture.cases){assert.equal(grant(row.grant),true,JSON.stringify(grant.errors));assert.equal(grant(row.remaining),true,JSON.stringify(grant.errors));assertions+=2;}
 const db=new Database(":memory:");db.run("CREATE TABLE receipt(items INTEGER, copied INTEGER, capacity INTEGER, released INTEGER)");
 const grantAxes=["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"];
 const progressValues=(value:Record<string,number>):number[]=>(fixture.axes as string[]).map(axis=>value[axis]!);const grantValues=(value:Record<string,number>):number[]=>grantAxes.map(axis=>value[axis]!);
 for(const row of fixture.cases){
  db.run("DELETE FROM receipt");db.run("INSERT INTO receipt VALUES (?1,?2,?3,?4)",progressValues(row.initial));
  db.run("UPDATE receipt SET items=items+?1,copied=copied+?2,capacity=capacity+?3,released=released+?4",progressValues(row.progress));
  const reference=db.query("SELECT items AS copiedItems,copied AS copiedBytes,capacity AS retainedCapacityBytes,released AS releasedBytes FROM receipt").get();assert.deepEqual(reference,row.recorded);assertions++;
  const remaining=db.query("SELECT max(0,?1-items) AS maximumItems,max(0,?2-copied) AS maximumCopyBytes,max(0,?3-capacity) AS maximumCapacityBytes,max(0,?4-released) AS maximumReleaseBytes,?5 AS maximumDepth FROM receipt").get(...grantValues(row.grant));assert.deepEqual(remaining,row.remaining);assertions++;
  const fits=fixture.axes.every((axis:string,index:number)=>row.recorded[axis]<=grantValues(row.grant)[index]!);
  assert.equal(row.failed,row.status==="refused"||!fits);assert.equal(row.ready,fits&&row.status==="complete"&&progressValues(row.progress).every(value=>value===0));assertions+=2;
 }
 db.close();
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");
 const receiver=source.slice(source.indexOf("fn flow_contributions_registry_receipt"),source.indexOf("impl FlowContributionsWork"));
 assert.ok(receiver.includes("cx.consume_retained(progress)"),"the original receiver records the actual child receipt");assertions++;
 assert.ok(receiver.indexOf("cx.consume_retained(progress)")<receiver.indexOf("match result"),"success and failed child effects reach the same context before result propagation");assertions++;
 assert.ok(receiver.includes("error.retained_progress()"));assert.ok(receiver.includes("progress==Default::default()"));assertions+=2;
 const work=source.slice(source.indexOf("impl ArtifactCommandWork<semio_framework_plugin::EditorApp<FlowPlayApp>> for FlowContributionsWork"),source.indexOf("struct FlowContributionsJobFactory"));
 assert.ok(work.includes("retire_flow_extension_registries_step(cx.retained_grant())"));assert.ok(work.includes("flow_contributions_registry_receipt(cx,maintenance)?"));assertions+=2;
 assert.ok(work.indexOf("flow_contributions_registry_receipt(cx,maintenance)?")<work.indexOf("set_contributions::install(payload, session)"));assertions++;
 assert.ok(work.includes("flow.registry-retirement-full"));assert.ok(work.includes("flow-contributions-retirement"));assertions+=2;
 const owner=source.match(/struct FlowContributionsWork\s*\{([^}]+)\}/)![1];assert.deepEqual([...owner.matchAll(/^\s*(\w+)\s*:/gm)].map(match=>match[1]),fixture.ownerFields);assertions++;
 assert.ok(work.includes("semio_framework_plugin::ArtifactInstanceOperationOwnerAliasRetirement::new(original)"));assertions++;
 assert.ok(work.includes("fn close_step(&mut self, grant:semio_framework_value::RetainedCloneGrant)"));assertions++;
 assert.ok(work.includes("owner.step(grant)"));assert.ok(work.includes("owner.terminal_is_empty()"));assertions+=2;
 assert.equal(work.includes("self.instance_owner.take().is_some()"),false);assertions++;
 for(const field of ["copy_byte","capacity_byte","release_byte","depth"]){assert.ok(work.includes("next_close_"+field+"_demand"));assertions++;}
 const instance=source.slice(source.indexOf("impl semio_framework_plugin::ArtifactInstanceOperationOwner for FlowInstanceOperationOwner"),source.indexOf("pub struct FlowPlayApp"));
 assert.ok(instance.includes("session.retirement_step(child)"));assert.ok(instance.includes("session.close_step(child)"));assertions+=2;
 assert.ok(instance.includes("fn retirement_demands"));assert.ok(instance.includes("maximum_depth:grant.maximum_depth-1"));assertions+=2;
 assert.equal(instance.includes("maximum_items: usize, maximum_bytes: usize"),false);assertions++;
 const fields=source.match(/struct FlowInstanceOperationOwner\s*\{([^}]+)\}/)![1];assert.deepEqual([...fields.matchAll(/^\s*(\w+)\s*:/gm)].map(match=>match[1]),fixture.instanceOwnerFields);assertions++;
 for(const mutant of [receiver.replace("cx.consume_retained(progress)","cx.consume_retained(Default::default())"),receiver.replace("error.retained_progress()","Default::default()"),receiver.replace("progress==Default::default()","true")]){assert.equal(mutant.includes("cx.consume_retained(progress)")&&mutant.includes("error.retained_progress()")&&mutant.includes("progress==Default::default()"),false);assertions++;}
 console.log("[DEBUG] original Flow contributions receiving cases="+fixture.cases.length+" assertions="+assertions+" canonicalGrantAjv=true schemaAuthority=absent SQLiteReference=true sourceReceivingOnly=true");
}
