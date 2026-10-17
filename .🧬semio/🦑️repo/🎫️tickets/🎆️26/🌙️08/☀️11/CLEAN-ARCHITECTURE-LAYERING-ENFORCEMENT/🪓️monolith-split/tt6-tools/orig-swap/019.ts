/** 🧩️ TS reducer, ring, store and codecs against the shared fixtures; oracles: ajv (schema), xstate (machine built from the matrix), fast-check (random event sequences, reducer vs xstate). */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import fc from "fast-check";
import { assign, createMachine, initialTransition, transition, type AnyMachineSnapshot } from "xstate";
import lifecycle from "../🧫️fixtures/⚖️lifecycle-law.json";
import ticks from "../🧫️fixtures/🎞️ticks.json";
import tracePages from "../🧫️fixtures/📼️trace-pages.json";
import schema from "../../🧬️schema/🔣️.json";
import * as M from "../../🟦️.ts";
import { isToolRunTerminal, TOOL_RUN_EVENT_KEYS, TOOL_RUN_STATES, toolRunAdmit, toolRunEffectCommits, type ToolRunEvent, toolRunEventKey, type ToolRunEventKey, ToolRunMachine, type ToolRunSlot, type ToolRunState } from "../../🚦️lifecycle/🟦️.ts";
import { ToolRunStepRing } from "../../📊️progress/🟦️.ts";
import { type ToolRunTraceOp, type ToolRunTracePage as MToolRunTracePage, ToolRunTraceStore } from "../../🗄️trace-store/🟦️.ts";
import { decodeToolRunTick, decodeToolRunTraceDelta, decodeToolRunTracePage, encodeToolRunTick, encodeToolRunTraceDelta, encodeToolRunTracePage } from "../../🔤️codec/🟦️.ts";
import { toolRunBytesToHex, toolRunEventFromJson, toolRunHexToBytes, toolRunIdentityFromJson, toolRunIdentityToJson, type ToolRunJson, toolRunProgressFromJson, toolRunProgressToJson, toolRunSlotFromJson, toolRunSlotToJson, toolRunStepFromJson, toolRunStepToJson, toolRunTickFromJson, toolRunTickToJson, toolRunTraceDeltaFromJson, toolRunTraceDeltaToJson, toolRunTraceOpFromJson, toolRunTraceOpToJson, toolRunTracePageFromJson, toolRunTracePageToJson } from "../../🧾️json/🟦️.ts";
import { TOOL_RUN_PANEL_ID, toolRunPanelGroupId, toolRunPanelGroupRun, toolRunPanelNewRuns } from "../../🪧️panel/🟦️.ts";
import { isToolRunActionLegal, TOOL_RUN_ACTIONS, TOOL_RUN_LABELS, toolRunFormat, toolRunReasonLabel, toolRunStateLabel } from "../../🏷️actions-labels/🟦️.ts";
import {applyPatch} from "fast-json-patch";

const isJsonObject=(value:ToolRunJson):value is {readonly [key:string]:ToolRunJson}=>typeof value==="object"&&value!==null&&!Array.isArray(value);
const jsonObject=(value:ToolRunJson):{readonly [key:string]:ToolRunJson}=>{
  if(!isJsonObject(value))throw new Error("A JSON object is required");
  return value;
};
import pendingSchema from "../../♻️retirement/🧬️schema/🔣️.json";
import pendingLaw from "../../♻️retirement/🧪️tests/🧫️fixtures/🔣️.json";
import progressCloneSchema from "../../📊️progress/📸️clone/🧬️schema/🔣️.json";
import progressCloneLaw from "../../📊️progress/📸️clone/🧪️tests/🧫️fixtures/🔣️.json";
import {ToolRunProgressClone} from "../../📊️progress/📸️clone/🟦️.ts";
import ringInsertSchema from "../../📊️progress/💍️steps/➕️insert/🧬️schema/🔣️.json";
import ringInsertLaw from "../../📊️progress/💍️steps/➕️insert/🧪️tests/🧫️fixtures/🔣️.json";
import {ToolRunStepRingInsert} from "../../📊️progress/💍️steps/➕️insert/🟦️.ts";
import presentationSchema from "../../📊️progress/🪟️presentation/🧬️schema/🔣️.json";
import presentationLaw from "../../📊️progress/🪟️presentation/🧪️tests/🧫️fixtures/🔣️.json";
import {ToolRunPresentation} from "../../📊️progress/🪟️presentation/🟦️.ts";
import updateSchema from "../../📊️progress/🪟️presentation/🧬️update/🧬️schema/🔣️.json";
import updateLaw from "../../📊️progress/🪟️presentation/🧬️update/🧪️tests/🧫️fixtures/🔣️.json";
import {ToolRunPresentationUpdate} from "../../📊️progress/🪟️presentation/🧬️update/🟦️.ts";
import seedSchema from "../../📊️progress/🪟️presentation/🌱️seed/🧬️schema/🔣️.json";
import seedLaw from "../../📊️progress/🪟️presentation/🌱️seed/🧪️tests/🧫️fixtures/🔣️.json";
import {ToolRunPresentationSeed} from "../../📊️progress/🪟️presentation/🌱️seed/🟦️.ts";
import slotSchema from "../../📊️progress/🪟️presentation/🗃️slot/🧬️schema/🔣️.json";
import slotLaw from "../../📊️progress/🪟️presentation/🗃️slot/🧪️tests/🧫️fixtures/🔣️.json";
import {ToolRunPresentationSlot} from "../../📊️progress/🪟️presentation/🗃️slot/🟦️.ts";
import tickSourceSchema from "../../📽️tick/🔐️source/🧬️schema/🔣️.json";
import tickSourceLaw from "../../📽️tick/🔐️source/🧪️tests/🧫️fixtures/🔣️.json";
import {ToolRunTickSource} from "../../📽️tick/🔐️source/🟦️.ts";

test("schema oracle original Tick lends the same operation and trace allocations",()=>{
 expect(new Ajv({strict:true}).compile(tickSourceSchema)(tickSourceLaw)).toBe(true);
 for(const order of tickSourceLaw.orders){const basis=toolRunTickFromJson(ticks.ticks[0].tick);const appendOps=tickSourceLaw.operations.map(toolRunHexToBytes);const trace=[{identity:basis.identity,page:0,ops:[{op:"clear"} as ToolRunTraceOp]}];const original={...basis,appendOps,trace};const source=new ToolRunTickSource(original);expect(source.operation(1,0)).toBeUndefined();expect(source.trace(0,0)).toBeUndefined();const operation=source.operation(1,1)!;const page=source.trace(0,1)!;expect(operation.borrowed).toBe(appendOps[1]!);expect(page.borrowed).toBe(trace[0]!);const oracle=applyPatch(toolRunTickToJson(basis),[{op:"replace",path:"/appendOps",value:tickSourceLaw.operations},{op:"replace",path:"/trace",value:trace.map(toolRunTracePageToJson)}],true,false).newDocument;expect(toolRunTickToJson(original)).toEqual(oracle);expect(source.take(1)).toBeUndefined();const children=order==="operationFirst"?[operation,page]:[page,operation];for(const child of children){expect(child.close(0)).toBe(0);expect(child.close(1)).toBe(1);expect(child.close(1)).toBe(0);}expect(source.take(0)).toBeUndefined();expect(source.borrowed).toBe(original);expect(source.take(1)).toBe(original);expect(source.take(1)).toBeUndefined();}
});

test("schema oracle original presentation slot keeps previous root readable through private publication",()=>{
 expect(new Ajv({strict:true}).compile(slotSchema)(slotLaw)).toBe(true);
 for(const stop of [undefined,...slotLaw.interruptAfter])for(const order of slotLaw.orders){const identity=toolRunTickFromJson(ticks.ticks[0].tick).identity;const slot=new ToolRunPresentationSlot(slotLaw.toolId,identity,"starting");while(!slot.ready){expect(slot.advance(0)).toBe(0);expect(slot.advance(1)).toBe(1);}const source=slot.captureView(identity,slotLaw.captureState as ToolRunState,1)!;const previous=source.original;const borrowed=source.view;expect(borrowed.progress).toBe(previous.progress);expect(borrowed.provisionalEntities).toBe(previous.provenance.entities);expect(borrowed.payload).toBe(previous.payload);expect(borrowed.identity).toBe(identity);expect(slotLaw.captureState).toBe(borrowed.state);expect(borrowed.progress.state).toBe("starting");const envelopeOracle=applyPatch({identity:toolRunIdentityToJson(identity),state:"starting"},[{op:"replace",path:"/state",value:slotLaw.captureState}],true,false).newDocument;expect(envelopeOracle).toEqual({identity:toolRunIdentityToJson(borrowed.identity),state:borrowed.state});const input={identity,sequence:1n,progress:toolRunProgressFromJson(ticks.ticks[0].tick.progress!),steps:[],retractTo:null,end:1,append:["7","11"],payload:new Uint8Array([1,2,3])};const counters=input.progress.counters;const payload=input.payload;const oracle=applyPatch({...previous,progress:toolRunProgressToJson(previous.progress),payload:null},[{op:"replace",path:"/provenance",value:{marks:[{end:1,entity:"7"},{end:1,entity:"11"}],entities:slotLaw.expectedEntities}},{op:"replace",path:"/payload",value:Array.from(payload)}],true,false).newDocument;expect(slot.beginTick(input,0)).toBe(false);expect(slot.beginTick(input,1)).toBe(true);expect(slot.beginTick(input,1)).toBe(false);let turns=0;while(!slot.ready&&turns<(stop??10000)){const body=slot.body;expect(slot.advance(0)).toBe(0);expect(slot.body).toBe(body);expect(slot.advance(1)).toBeLessThanOrEqual(1);expect(source.original).toBe(previous);expect(previous.progress.counters).toEqual([]);turns++;}if(stop===undefined){expect(slot.ready).toBe(true);expect(slot.body!.provenance).toEqual(oracle.provenance);expect(slot.body!.payload).toBe(payload);expect(slot.body!.progress.counters).toBe(counters);expect(slot.body!.progress.sequence).toBe(1n);}if(stop!==undefined){const before=slot.body;slot.cancelPending();while(!slot.ready){expect(slot.advance(0)).toBe(0);expect(slot.advance(1)).toBeLessThanOrEqual(1);expect(slot.body).toBe(before);}const restart={identity,sequence:2n,steps:[],retractTo:0,end:2,append:[slotLaw.restartEntity],payload:undefined};expect(slot.beginTick(restart,1)).toBe(true);while(!slot.ready)expect(slot.advance(1)).toBeLessThanOrEqual(1);expect(slot.body!.progress.sequence).toBe(2n);expect(slot.body!.provenance.entities).toEqual([slotLaw.restartEntity]);expect(source.original).toBe(previous);}slot.cancel();expect(slot.capture(1)).toBeUndefined();if(order==="captureFirst")source.close(1);while(slot.retained){expect(slot.close(0)).toBe(0);expect(slot.close(1)).toBeLessThanOrEqual(1);}if(order==="slotFirst"){expect(source.original).toBe(previous);source.close(1);}}
});

test("schema oracle original presentation seed preserves identity and empty canonical leaves",()=>{
 expect(new Ajv({strict:true}).compile(seedSchema)(seedLaw)).toBe(true);
 for(const toolId of seedLaw.toolIds)for(const stop of [undefined,...seedLaw.interruptAfter]){const seed=new ToolRunPresentationSeed(toolId);const oracle=applyPatch({},Object.entries({toolId,...seedLaw.expected}).map(([key,value])=>({op:"add" as const,path:`/${key}`,value})),true,false).newDocument;let turns=0;while(!seed.complete&&turns<(stop??100)){expect(seed.take(1)).toBeUndefined();expect(seed.advance(0)).toBe(0);expect(seed.original).toBe(toolId);expect(seed.advance(1)).toBe(1);turns++;}if(stop===undefined){expect(seed.take(0)).toBeUndefined();const output=seed.take(1)!;expect(oracle).toEqual(output.original);expect(seed.take(1)).toBeUndefined();output.close(1);}else{seed.cancel();expect(seed.take(1)).toBeUndefined();}while(seed.retained){expect(seed.close(0)).toBe(0);expect(seed.close(1)).toBe(1);}expect(seed.original).toBeUndefined();}
});

test("schema oracle original presentation update publishes joint progress provenance and payload atomically",()=>{
 expect(new Ajv({strict:true}).compile(updateSchema)(updateLaw)).toBe(true);
 for(const withProgress of updateLaw.progressCases)for(const payload of updateLaw.payloadCases){const progress=toolRunTickFromJson(ticks.ticks[0].tick).progress!;const fields={toolId:"draw.command",progress,provenance:{marks:updateLaw.originalMarks,entities:["7","9"]},payload:new Uint8Array([4,5])};const input={identity:progress.identity,sequence:progress.sequence+1n,progress:withProgress?{...toolRunProgressFromJson(ticks.ticks[0].tick.progress!),stage:17,completed:99n,total:3n,counters:[{counter:2,value:5n}]}:undefined,steps:[toolRunStepFromJson(ringInsertLaw.cases[0]!.input)],retractTo:updateLaw.retractTo,end:updateLaw.end,append:updateLaw.append,payload:payload===null?undefined:new Uint8Array(Buffer.from(payload,"hex"))};const marks=updateLaw.originalMarks.filter(mark=>mark.end<=updateLaw.retractTo);for(const entity of updateLaw.append)if(!marks.some(mark=>mark.end===updateLaw.end&&mark.entity===entity))marks.push({end:updateLaw.end,entity});marks.sort((a,b)=>a.end-b.end||Number(BigInt(a.entity)-BigInt(b.entity)));expect(marks).toEqual(updateLaw.expectedMarks);const entities=[...new Set(marks.map(mark=>mark.entity))].sort((a,b)=>Number(BigInt(a)-BigInt(b)));expect(entities).toEqual(updateLaw.expectedEntities);
  const oracle=applyPatch({...fields,progress:toolRunProgressToJson(progress),payload:Array.from(fields.payload)},[{op:"replace",path:"/provenance",value:{marks,entities}},{op:"replace",path:"/payload",value:Array.from(input.payload??fields.payload)}],true,false).newDocument;
  for(const stop of [undefined,...updateLaw.interruptAfter]){const source=new ToolRunPresentation(fields);const event={...input,append:[...input.append],progress:input.progress?toolRunProgressFromJson(jsonObject(toolRunProgressToJson(input.progress))):undefined,steps:input.steps.map(step=>({...step,args:step.args.map(arg=>({...arg}))}))};const job=new ToolRunPresentationUpdate(source.capture(1)!,event);let turns=0;while(!job.complete&&turns<(stop??10000)){expect(job.take(1)).toBeUndefined();expect(job.advance(0)).toBe(0);expect(job.advance(1)).toBeLessThanOrEqual(1);expect(source.original).toBe(fields);turns++;}if(stop===undefined){const result=job.take(1)!;expect(result.original.toolId).toBe(oracle.toolId);expect(result.original.provenance).toEqual(oracle.provenance);expect(Array.from(result.original.payload!)).toEqual(oracle.payload);expect(result.original.progress.sequence).toBe(input.sequence);if(withProgress){expect(result.original.progress.stage).toBe(17);expect(result.original.progress.completed).toBe(3n);expect(result.original.progress.counters).toEqual(input.progress!.counters);}expect(result.original.progress.steps.at(-1)!.args).toEqual(input.steps[0]!.args);result.close(1);}else{job.cancel();expect(job.take(1)).toBeUndefined();}while(job.retained)expect(job.close(1)).toBeLessThanOrEqual(1);expect(source.original).toBe(fields);source.close(1);}
  if(updateLaw.foreignRun){const source=new ToolRunPresentation(fields);const event={...input,identity:{...progress.identity,id:{...progress.identity.id,run:progress.identity.id.run+1n}},append:[...input.append],progress:undefined,steps:input.steps.map(step=>({...step,args:step.args.map(arg=>({...arg}))}))};const job=new ToolRunPresentationUpdate(source.capture(1)!,event);expect(()=>job.advance(1)).toThrow("Pending presentation belongs to another original run");expect(job.complete).toBe(false);expect(job.take(1)).toBeUndefined();job.cancel();while(job.retained)expect(job.close(1)).toBeLessThanOrEqual(1);expect(source.original).toBe(fields);source.close(1);}
 }
});

test("schema oracle original presentation captures preserve joint fields through every alias order",()=>{
 expect(new Ajv({strict:true}).compile(presentationSchema)(presentationLaw)).toBe(true);
 for(const payload of presentationLaw.payloadCases)for(const order of presentationLaw.orders){const fields={toolId:"draw.command",progress:ticks.ticks[0].tick.progress,provenance:[[1,7],[3,9]],payload};const independent=applyPatch({},Object.entries(fields).map(([key,value])=>({op:"add" as const,path:`/${key}`,value})),true,false).newDocument;const source=new ToolRunPresentation(fields);const captures=[];for(let index=0;index<presentationLaw.aliases;index++){expect(source.capture(0)).toBeUndefined();const captured=source.capture(1)!;expect(captured.original).toBe(fields);expect(independent).toEqual(captured.original);captures.push(captured);}const sequence=order==="sourceFirst"?[source,...captures]:[...captures,source];for(const handle of sequence){for(const live of sequence.filter(other=>other.retained)){expect(live.original).toBe(fields);expect(independent).toEqual(live.original);}expect(handle.close(0)).toBe(0);expect(handle.close(1)).toBe(1);expect(handle.retained).toBe(false);}}
});

test("schema oracle original step ring insertion preserves originals through coalescing eviction and cancellation",()=>{
 expect(new Ajv({strict:true}).addSchema(schema).compile(ringInsertSchema)(ringInsertLaw)).toBe(true);
 for(const row of ringInsertLaw.cases){const original=Array.from({length:row.initialCount},(_,index)=>({...row.template,sequence:index+1}));const latest=original.at(-1);const coalesces=latest&&latest.reason===row.input.reason;
  const independent=coalesces?applyPatch(original,[{op:"replace",path:`/${original.length-1}/sequence`,value:row.input.sequence},{op:"replace",path:`/${original.length-1}/repeat`,value:Math.min(0xffffffff,latest.repeat+row.input.repeat)}],true,false).newDocument:applyPatch(original,[...(original.length===64?[{op:"remove" as const,path:"/0"}]:[]),{op:"add",path:"/-",value:row.input}],true,false).newDocument;
  expect(independent.length).toBe(row.expectedCount);expect(independent[0]!.sequence).toBe(row.expectedFirstSequence);expect(independent.at(-1)!.sequence).toBe(row.expectedLastSequence);expect(independent.at(-1)!.repeat).toBe(row.expectedLastRepeat);
  for(const stop of [undefined,...ringInsertLaw.interruptAfter]){const ring=original.map(toolRunStepFromJson);const input=toolRunStepFromJson(row.input);const job=new ToolRunStepRingInsert(ring,input);let turns=0;while(!job.complete&&turns<(stop??1000)){expect(job.advance(0)).toBe(0);expect(job.advance(1)).toBeLessThanOrEqual(1);turns++;}if(stop===undefined){expect(job.complete).toBe(true);expect(job.take(0)).toBeUndefined();expect(job.take(1)!.map(toolRunStepToJson)).toEqual(independent);expect(job.take(1)).toBeUndefined();}else{job.cancel();expect(job.take(1)).toBeUndefined();}while(job.retained)expect(job.close(1)).toBeLessThanOrEqual(1);}
 }
});

test("schema oracle original progress clone preserves every canonical field and canceled source",()=>{
 expect(new Ajv({strict:true}).compile(progressCloneSchema)(progressCloneLaw)).toBe(true);
 const maximum=JSON.parse(JSON.stringify(ticks.ticks[0]));maximum.tick.progress.counters=Array.from({length:progressCloneLaw.maximumLeaves.counters},(_,counter)=>({counter,value:counter+1}));maximum.tick.progress.steps=Array.from({length:progressCloneLaw.maximumLeaves.steps},(_,index)=>({...maximum.tick.steps[0],sequence:index+1,args:Array.from({length:progressCloneLaw.maximumLeaves.arguments},()=>({...maximum.tick.steps[0].args[0]}))}));
 for(const row of [...ticks.ticks,maximum]){const tick=toolRunTickFromJson(row.tick);if(!tick.progress)continue;const original=tick.progress;
  const oracle=applyPatch({},Object.entries(row.tick.progress).map(([key,value])=>({op:"add" as const,path:`/${key}`,value})),true,false).newDocument;
  for(const stop of [undefined,...progressCloneLaw.interruptAfter]){const job=new ToolRunProgressClone(original);let turns=0;while(!job.complete&&turns<(stop??10000)){expect(job.advance(0)).toBe(0);expect(job.advance(1)).toBeLessThanOrEqual(1);turns++;}
   if(stop===undefined){expect(job.complete).toBe(true);expect(job.take(0)).toBeUndefined();const output=job.take(1)!;expect(jsonObject(toolRunTickToJson({...tick,progress:output})).progress).toEqual(oracle);expect(job.take(1)).toBeUndefined();}else{job.cancel();expect(job.take(1)).toBeUndefined();}
   expect(jsonObject(toolRunTickToJson(tick)).progress).toEqual(oracle);while(job.retained)expect(job.close(1)).toBeLessThanOrEqual(1);
  }
 }
});

test("schema oracle original pending tick and writer custody matches independent fields",()=>{
 expect(new Ajv({strict:true}).compile(pendingSchema)(pendingLaw)).toBe(true);
 for(const row of ticks.ticks){for(const owner of pendingLaw.owners){
  const original=JSON.parse(JSON.stringify(row.tick));
  const independent=applyPatch({},Object.entries(original).map(([key,value])=>({op:"add" as const,path:`/${key}`,value})),true,false).newDocument;
  expect(independent).toEqual(row.tick);
  for(const axis of Object.keys(pendingLaw.grant)){const refused=applyPatch({original,owner,axis},[],true,false).newDocument;expect(refused.original).toEqual(row.tick);}
 }}
});

const law = lifecycle as any;
const pagesFixture = tracePages as any;
const ticksFixture = ticks as any;
const hex = toolRunBytesToHex;

function matrixEvent(key: ToolRunEventKey, run: bigint, generation: number): ToolRunEvent {
  switch (key) {
    case "start":
      return { type: "start", run: run + 1n };
    case "settingsChanged":
    case "baseChanged":
    case "dismiss":
      return { type: key, run };
    case "closed":
      return { type: "closed" };
    case "abort":
    case "abortWhilePublishing":
      return { type: "abort", run, generation, publishing: key === "abortWhilePublishing" };
    default:
      return { type: key, run, generation };
  }
}

function stateName(slot: ToolRunSlot | null): string {
  return slot === null ? "none" : slot.state;
}

describe("schema oracle (ajv)", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const validator = (name: string) => ajv.getSchema(`${schema.$id}#/$defs/${name}`)!;



  test("hostile trace-page and tick values are rejected by the domain schema", () => {
    const pageValidate = validator("ToolRunTracePage");
    const page = pagesFixture.pages[0].page;
    expect(pageValidate(page)).toBe(true);
    expect(pageValidate({ ...page, ops: [{ op: "upsert", key: 1, verdict: "rejected", reason: 0, subject: { kind: "entity", entity: 1 } }] })).toBe(false);
    expect(pageValidate({ ...page, identity: { ...page.identity, baseRevision: "00" } })).toBe(false);
    const tickValidate = validator("ToolRunTick");
    expect(tickValidate({ ...ticksFixture.ticks[0].tick, steps: [{ ...ticksFixture.ticks[0].tick.steps[0], args: [{ unsigned: 1 }, { unsigned: 2 }, { unsigned: 3 }, { unsigned: 4 }, { unsigned: 5 }] }] })).toBe(false);
  });
});

describe("lifecycle law", () => {
  test("states and event keys mirror the fixture", () => {
    expect([...TOOL_RUN_STATES]).toEqual(law.states);
    expect([...TOOL_RUN_EVENT_KEYS]).toEqual(law.eventKeys);
    expect(new Set(law.matrix.map((row: any) => `${row.from}×${row.event}`)).size).toBe(180);
  });

  test("reducer matches every matrix row", () => {
    for (const row of law.matrix) {
      const slot: ToolRunSlot | null = row.from === "none" ? null : { run: 7n, generation: 2, state: row.from };
      const result = ToolRunMachine.apply(slot, matrixEvent(row.event, 7n, 2));
      const label = `${row.from} × ${row.event}`;
      if ("rejection" in row) {
        expect(result.ok ? "accepted" : result.rejection, label).toBe(row.rejection);
        continue;
      }
      expect(result.ok, label).toBe(true);
      if (!result.ok) continue;
      expect(stateName(result.transition.slot), label).toBe(row.to);
      expect(result.transition.effect, label).toBe(row.effect);
      expect(toolRunEffectCommits(result.transition.effect), label).toBe(row.commits);
      if (result.transition.slot !== null) {
        expect(result.transition.slot.generation, label).toBe(row.generation === "reset" ? 0 : row.generation === "increment" ? 3 : 2);
        expect(result.transition.slot.run, label).toBe(row.event === "start" ? 8n : 7n);
      }
    }
  });

  test("reducer matches every case", () => {
    for (const row of law.cases) {
      const result = ToolRunMachine.apply(toolRunSlotFromJson(row.slot), toolRunEventFromJson(row.event));
      if ("rejection" in row.expect) expect(result.ok ? "accepted" : result.rejection, row.name).toBe(row.expect.rejection);
      else {
        expect(result.ok, row.name).toBe(true);
        if (result.ok) expect({ slot: toolRunSlotToJson(result.transition.slot), effect: result.transition.effect }, row.name).toEqual(row.expect.transition);
      }
    }
  });

  test("reducer replays every scenario with its commit count", () => {
    for (const scenario of law.scenarios) {
      let slot: ToolRunSlot | null = null;
      let commits = 0;
      const observed: string[] = [];
      for (const json of scenario.events) {
        const result = ToolRunMachine.apply(slot, toolRunEventFromJson(json));
        if (result.ok) {
          commits += Number(toolRunEffectCommits(result.transition.effect));
          slot = result.transition.slot;
          observed.push(stateName(slot));
        } else observed.push(result.rejection);
      }
      expect(observed, scenario.name).toEqual(scenario.states);
      expect(commits, scenario.name).toBe(scenario.commits);
    }
  });

  test("actions, chords, labels, templates and limits mirror the fixture", () => {
    expect(TOOL_RUN_ACTIONS.map((action) => ({ id: action.id, chord: action.chord, label: action.label, args: action.args }))).toEqual(law.actions.map((row: any) => ({ id: row.id, chord: row.chord, label: row.label, args: row.args })));
    for (const row of law.actions) {
      for (const state of ["none", ...TOOL_RUN_STATES]) {
        const legal = isToolRunActionLegal(row.id, state === "none" ? null : (state as ToolRunState));
        expect(legal, `${row.id} in ${state}`).toBe(row.legalIn.includes(state));
      }
    }
    expect(Object.entries(TOOL_RUN_LABELS).map(([key, text]) => ({ key, ...text }))).toEqual(law.labels);
    for (const state of TOOL_RUN_STATES) expect(TOOL_RUN_LABELS[toolRunStateLabel(state)]).toBeDefined();
    for (const row of law.reservedReasons) expect(toolRunReasonLabel(row.code)).toBe(row.label);
    expect(toolRunReasonLabel(M.TOOL_RUN_RESERVED_REASON_FLOOR - 1)).toBeUndefined();
    for (const row of law.templates) expect(toolRunFormat(row.template, (name) => row.values[name])).toBe(row.expected);
    expect(law.limits).toEqual({
      stepRingCapacity: M.TOOL_RUN_STEP_RING_CAPACITY,
      stepArgsMax: M.TOOL_RUN_STEP_ARGS_MAX,
      countersMax: M.TOOL_RUN_COUNTERS_MAX,
      provisionalOpsMax: M.TOOL_RUN_PROVISIONAL_OPS_MAX,
      memberOpsMax: M.TOOL_RUN_MEMBER_OPS_MAX,
      traceResidentRecords: M.TOOL_RUN_TRACE_RESIDENT_RECORDS,
      tracePageOpsMax: M.TOOL_RUN_TRACE_PAGE_OPS_MAX,
      tracePageBytesMax: M.TOOL_RUN_TRACE_PAGE_BYTES_MAX,
      tickBytesMax: M.TOOL_RUN_TICK_BYTES_MAX,
      traceLogCompactFloor: M.TOOL_RUN_TRACE_LOG_COMPACT_FLOOR,
      statusAnnounceIntervalMs: M.TOOL_RUN_STATUS_ANNOUNCE_INTERVAL_MS,
      reservedReasonFloor: M.TOOL_RUN_RESERVED_REASON_FLOOR,
    });
  });
});

describe("state machine oracle (xstate + fast-check)", () => {
  type Context = { run: bigint; generation: number };
  type OracleEvent = { type: ToolRunEventKey; run?: bigint; generation?: number };
  const states: Record<string, { on: Record<string, unknown> }> = {};
  for (const from of ["none", ...law.states]) states[from] = { on: {} };
  for (const row of law.matrix.filter((row: any) => "to" in row)) {
    const guard = ({ context, event }: { context: Context; event: OracleEvent }) => {
      if (row.event === "closed") return true;
      if (row.event === "start") return row.from === "none" || event.run! > context.run;
      return event.run === context.run && (event.generation === undefined || event.generation === context.generation);
    };
    const assignContext = ({ context, event }: { context: Context; event: OracleEvent }): Context => (row.event === "start" ? { run: event.run!, generation: 0 } : row.generation === "increment" ? { run: context.run, generation: (context.generation + 1) >>> 0 } : context);
    states[row.from]!.on[row.event] = { target: `#toolRun.${row.to}`, guard, actions: assign(assignContext as any) };
  }
  const machine = createMachine({ id: "toolRun", initial: "none", context: { run: 0n, generation: 0 } as Context, states } as any);

  const oracleEvent = (event: ToolRunEvent): OracleEvent => ({ type: toolRunEventKey(event), ...("run" in event ? { run: event.run } : {}), ...("generation" in event ? { generation: event.generation } : {}) });

  test("the xstate machine built from the matrix agrees with the reducer on every row", () => {
    const [initial] = initialTransition(machine);
    expect(initial.value).toBe("none");
    for (const row of law.matrix) {
      const slot: ToolRunSlot | null = row.from === "none" ? null : { run: 7n, generation: 2, state: row.from };
      const snapshot = machine.resolveState({ value: row.from, context: { run: 7n, generation: 2 } } as any);
      const event = matrixEvent(row.event, 7n, 2);
      const result = ToolRunMachine.apply(slot, event);
      expect((snapshot as AnyMachineSnapshot).can(oracleEvent(event) as any), `${row.from} × ${row.event}`).toBe(result.ok);
      if (!result.ok) continue;
      const [next] = transition(machine, snapshot, oracleEvent(event) as any);
      expect(next.value, `${row.from} × ${row.event}`).toBe(stateName(result.transition.slot));
      if (result.transition.slot !== null) expect(next.context, `${row.from} × ${row.event}`).toEqual({ run: result.transition.slot.run, generation: result.transition.slot.generation });
    }
  });

  test("random event sequences: reducer and xstate agree on acceptance, state, run and generation", () => {
    const weighted = <T extends string>(same: T, ...others: T[]) => fc.oneof({ arbitrary: fc.constant(same), weight: 8 }, ...others.map((other) => ({ arbitrary: fc.constant(other), weight: 1 })));
    const choice = fc.record({ key: fc.constantFrom(...TOOL_RUN_EVENT_KEYS), run: weighted("same", "next", "older"), generation: weighted("same", "next", "zero") });
    fc.assert(
      fc.property(fc.array(choice, { minLength: 20, maxLength: 160 }), (choices) => {
        let slot: ToolRunSlot | null = null;
        let [snapshot] = initialTransition(machine);
        for (const pick of choices) {
          const baseRun = slot?.run ?? 1n;
          const run = pick.run === "same" ? baseRun : pick.run === "next" ? baseRun + 1n : baseRun > 0n ? baseRun - 1n : 0n;
          const generation = pick.generation === "same" ? (slot?.generation ?? 0) : pick.generation === "next" ? (slot?.generation ?? 0) + 1 : 0;
          const event = matrixEvent(pick.key, run - (pick.key === "start" ? 1n : 0n), generation);
          const oracle = oracleEvent(event);
          const result = ToolRunMachine.apply(slot, event);
          const accepted = (snapshot as AnyMachineSnapshot).can(oracle as any);
          if (result.ok !== accepted) return false;
          if (!result.ok) continue;
          [snapshot] = transition(machine, snapshot, oracle as any);
          slot = result.transition.slot;
          if (snapshot.value !== stateName(slot)) return false;
          if (slot !== null && (snapshot.context.run !== slot.run || snapshot.context.generation !== slot.generation)) return false;
        }
        return true;
      }),
      { numRuns: 3_000 },
    );
  });
});

describe("trace codec and store", () => {
  test("pages and deltas encode to and decode from the fixture bytes", () => {
    for (const row of pagesFixture.pages) {
      const page = toolRunTracePageFromJson(row.page);
      expect(hex(encodeToolRunTracePage(page)), row.name).toBe(row.hex);
      expect(toolRunTracePageToJson(decodeToolRunTracePage(toolRunHexToBytes(row.hex))), row.name).toEqual(row.page);
    }
    for (const row of pagesFixture.deltas) {
      const delta = toolRunTraceDeltaFromJson(row.delta);
      expect(hex(encodeToolRunTraceDelta(delta)), row.name).toBe(row.hex);
      expect(toolRunTraceDeltaToJson(decodeToolRunTraceDelta(toolRunHexToBytes(row.hex))), row.name).toEqual(row.delta);
    }
  });

  test("malformed wire values are rejected", () => {
    for (const row of pagesFixture.malformed) {
      const decode = row.target === "page" ? decodeToolRunTracePage : row.target === "delta" ? decodeToolRunTraceDelta : decodeToolRunTick;
      expect(() => decode(toolRunHexToBytes(row.hex)), row.name).toThrow(M.ToolRunCodecError);
    }
  });

  test("full u64 range and the op cap", () => {
    const identity: M.ToolRunIdentity = { id: { appInstanceId: 0xffff_ffff, run: 0xffff_ffff_ffff_ffffn }, generation: 1, baseRevision: new Uint8Array(32).fill(0xab) };
    const page: MToolRunTracePage = { identity, page: 0xffff_ffff, ops: [{ op: "upsert", key: 0xffff_ffff_ffff_ffffn, verdict: "warning", reason: 0xffff, subject: { kind: "entity", entity: 0xffff_ffff_ffff_ffffn } }] };
    expect(decodeToolRunTracePage(encodeToolRunTracePage(page))).toEqual(page);
    expect(() => encodeToolRunTracePage({ identity, page: 0, ops: Array.from({ length: M.TOOL_RUN_TRACE_PAGE_OPS_MAX + 1 }, () => ({ op: "clear" }) as const) })).toThrow(M.ToolRunCodecError);
    expect(M.toolRunGroupId(identity.id)).toBe("toolRun:18446744073709551615");
  });

  const storeFor = (row: any) => {
    const store = new ToolRunTraceStore(toolRunIdentityFromJson(row.identity), row.capacity, row.compactFloor);
    const applied = row.pages.map((ops: any[]) => store.applyOps(ops.map(toolRunTraceOpFromJson)));
    return { store, applied };
  };

  test("residency matches the fixture", () => {
    for (const row of pagesFixture.residency) {
      const { store, applied } = storeFor(row);
      expect(applied, row.name).toEqual(row.applied);
      const resident = [...store.records()].sort((a, b) => (a[0] < b[0] ? -1 : 1)).map(([key, record]) => ({ key: Number(key), verdict: record.verdict, reason: record.reason }));
      expect(resident, row.name).toEqual(row.resident);
      expect(store.logBase, row.name).toBe(row.logBase);
      const logPages = Array.from({ length: store.nextPage - store.logBase }, (_, index) => store.logPage(store.logBase + index)!.map(toolRunTraceOpToJson));
      expect(logPages, row.name).toEqual(row.logPages);
    }
  });

  test("delivery matches the fixture", () => {
    for (const row of pagesFixture.delivery) {
      const { store } = storeFor(pagesFixture.residency.find((candidate: any) => candidate.name === row.residency));
      store.rebind({ ...store.identity, generation: row.generation });
      const cursor = row.cursor === null ? null : { run: BigInt(row.cursor.run), generation: row.cursor.generation, page: row.cursor.page };
      const delta = store.deltaAfter(cursor, row.byteBudget);
      expect({ clear: delta.clear, next: delta.next, pages: delta.pages.map((page) => page.page) }, row.name).toEqual(row.expect);
      expect(decodeToolRunTraceDelta(encodeToolRunTraceDelta(delta)), row.name).toEqual(delta);
    }
  });

  test("stale pages are rejected and a new run resets the store", () => {
    const identity: M.ToolRunIdentity = { id: { appInstanceId: 1, run: 5n }, generation: 0, baseRevision: new Uint8Array(32) };
    const store = new ToolRunTraceStore(identity, 4, 4);
    const ops: ToolRunTraceOp[] = [{ op: "upsert", key: 1n, verdict: "testing", reason: 0, subject: { kind: "entity", entity: 1n } }];
    expect(store.applyPage({ identity: { ...identity, generation: 1 }, page: 0, ops })).toBe("toolRun.stale");
    expect(store.applyPage({ identity, page: 9, ops })).toEqual({ logged: 1, evicted: 0, overflowed: 0, compacted: false });
    store.rebind({ ...identity, id: { appInstanceId: 1, run: 6n } });
    expect([store.size, store.logBase, store.nextPage]).toEqual([0, 0, 0]);
  });
});

describe("tick codec and step ring", () => {
  test("ticks encode to and decode from the fixture bytes", () => {
    for (const row of ticksFixture.ticks) {
      const tick = toolRunTickFromJson(row.tick);
      expect(hex(encodeToolRunTick(tick)), row.name).toBe(row.hex);
      expect(toolRunTickToJson(decodeToolRunTick(toolRunHexToBytes(row.hex))), row.name).toEqual(row.tick);
    }
  });

  test("tick byte cap", () => {
    const identity: M.ToolRunIdentity = { id: { appInstanceId: 0, run: 0n }, generation: 0, baseRevision: new Uint8Array(32) };
    expect(() => encodeToolRunTick({ identity, sequence: 0n, steps: [], trace: [], appendOps: [new Uint8Array(M.TOOL_RUN_TICK_BYTES_MAX)], appendEntities: [] })).toThrow(M.ToolRunCodecError);
  });

  test("step ring matches the fixture", () => {
    for (const row of ticksFixture.stepRing) {
      const ring = new ToolRunStepRing();
      for (const push of row.push) {
        if ("repeatSequence" in push) {
          for (let sequence = push.repeatSequence.from; sequence < push.repeatSequence.from + push.repeatSequence.count; sequence += 1) ring.push({ sequence: BigInt(sequence), kind: push.repeatSequence.kind, stage: 0, reason: sequence & 0xffff, repeat: 1, args: [] });
        } else ring.push(toolRunStepFromJson(push));
      }
      expect(ring.length, row.name).toBe(row.expect.length);
      expect(ring.oldest() === undefined ? null : toolRunStepToJson(ring.oldest()!), row.name).toEqual(row.expect.first);
      expect(ring.newest() === undefined ? null : toolRunStepToJson(ring.newest()!), row.name).toEqual(row.expect.last);
    }
  });
});

describe("start admission", () => {
  test("every admission row follows the lane law", () => {
    for (const row of law.admission) {
      const live = row.live.map((entry: any) => [entry.lane, entry.state] as const);
      const result = toolRunAdmit(row.lane, live);
      expect(result, row.name).toEqual(row.rejection === undefined ? { ok: true, replaces: row.replaces } : { ok: false, rejection: row.rejection });
    }
  });

  test("fast-check: a start is busy exactly when a non-terminal run on its lane exists, and never admits a second non-terminal mutating run", () => {
    const lane = fc.record({ mutating: fc.boolean(), toolId: fc.constantFrom("fill", "preview"), windowId: fc.constantFrom("a", "b") });
    fc.assert(
      fc.property(fc.array(fc.tuple(lane, fc.constantFrom(...TOOL_RUN_STATES)), { maxLength: 6 }), lane, (live, next) => {
        const result = toolRunAdmit(next, live);
        const blocking = live.some(([other, state]) => !isToolRunTerminal(state) && (next.mutating && other.mutating ? true : !next.mutating && !other.mutating && other.toolId === next.toolId && other.windowId === next.windowId));
        expect(result.ok).toBe(!blocking);
        if (result.ok && next.mutating) expect(live.filter(([other, state]) => other.mutating && !isToolRunTerminal(state)).length).toBe(0);
      }),
      { numRuns: 2000 },
    );
  });
});

describe("tool run panel groups", () => {
  test("group keys and reveals follow the fixture", () => {
    const cases = law.panelGroups;
    expect(TOOL_RUN_PANEL_ID).toBe(cases.rootId);
    for (const row of cases.keys) {
      expect(toolRunPanelGroupRun(row.key), row.key).toBe(row.run === null ? null : BigInt(row.run));
      if (row.run !== null) expect(toolRunPanelGroupId(BigInt(row.run))).toBe(row.key);
    }
    for (const row of cases.reveals) {
      const { current, added } = toolRunPanelNewRuns(new Set(row.previous.map(BigInt)), row.keys);
      expect([[...current].sort(), added], row.name).toEqual([row.current.map(BigInt), row.added.map(BigInt)]);
    }
  });
});
