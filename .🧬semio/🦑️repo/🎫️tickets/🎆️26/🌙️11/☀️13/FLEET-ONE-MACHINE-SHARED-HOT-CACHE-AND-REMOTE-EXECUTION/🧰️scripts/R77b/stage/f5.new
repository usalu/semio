/** 📶️ Contract: every long phase of the dependency producer names a counter in its progress lines, so the stall rule of the ready stage (a line is new when it differs after the elapsed-seconds counters are replaced) cuts a hung phase and never a slow phase that moves: the input replay and the Source endpoint publication of a Windows store take longer than the stall limit. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{readyStallTrackerV1}from"../../../../../../🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🟦️.ts";

const source=readFileSync(fileURLToPath(new URL("../../🟦️.ts",import.meta.url)),"utf8");
const line=(resource:string)=>'[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"'+resource+'","mode":"shared","elapsedMs":0}}';

test("the input replay and the publication loop publish a counter",()=>{
 expect(source).toContain('resource:"dependencies:store-input-replay:"+work.completed');
 const from=source.indexOf("await publishJavascriptDependencyPreservingSourceDescriptorV1(binding"),to=source.indexOf("await verifyReady();const finals"),loop=source.slice(from,to);
 expect(from).toBeGreaterThan(0);expect(to).toBeGreaterThan(from);expect(loop).toContain('resource:"dependencies:publish-binding:"+published');
});

test("a moving counter is progress for the stall rule and a repeated line is not",()=>{
 for(const phase of["dependencies:store-input-replay:","dependencies:publish-binding:"]){
  const tracker=readyStallTrackerV1();expect(tracker.observe([line(phase+"64")]),phase+" first").toBe(true);expect(tracker.observe([line(phase+"64")]),phase+" repeated").toBe(false);expect(tracker.observe([line(phase+"128")]),phase+" moved").toBe(true);
 }
 const old=readyStallTrackerV1();expect(old.observe([line("dependencies:store-input-replay")])).toBe(true);expect(old.observe([line("dependencies:store-input-replay")])).toBe(false);
});
