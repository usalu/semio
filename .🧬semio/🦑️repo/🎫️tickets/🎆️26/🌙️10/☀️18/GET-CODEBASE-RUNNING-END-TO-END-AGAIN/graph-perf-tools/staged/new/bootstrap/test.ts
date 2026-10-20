/** 🪝️ Contract: task runs queued behind one dependency synchronization install at most once (each re-checks readiness under the exclusive hold it finally obtains) and a shared walk's evidence is adopted only when the exclusive writer proved it. The scenarios live in `./🔣️.json` as language-agnostic data. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";
import {adoptedCandidateV1,installUnlessReadyV1} from "../🟦️.ts";

test("bootstrap install fixture is closed",()=>{expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);});

test("a shared walk's evidence is adopted only after the exclusive writer published or found the receipt",()=>{
 for(const row of fixture.adoption)expect(adoptedCandidateV1(row.status as never),row.status).toBe(row.adopted);
});

test("runs queued for the exclusive hold install once and repeat only an installation that failed to ready the environment",async()=>{
 for(const scenario of fixture.installs){
  let ready=scenario.initiallyReady,prepares=0,tail=Promise.resolve();
  const store={} as never,control={} as never,recheck=scenario.recheck?async()=>ready:undefined,prepare=async()=>{prepares++;await new Promise(accept=>setTimeout(accept,1));ready=scenario.readyAfterInstall;};
  const runs=Array.from({length:scenario.runs},()=>{const turn=tail.then(()=>installUnlessReadyV1(store,control,recheck,prepare));tail=turn.then(()=>undefined);return turn;});
  const results=await Promise.all(runs);
  expect(prepares,scenario.id).toBe(scenario.expectedPrepares);
  expect(results.filter(Boolean).length,scenario.id).toBe(scenario.expectedPrepares);
 }
});
