/** 🧬️ Neutral retained-genesis chart and XState oracle; source contracts are separate from native behavior proof. */
import { expect, test } from "bun:test";
import { readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import Ajv from "ajv";
import { initialTransition, setup, transition } from "xstate";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";
const table: Record<string, Record<string,string>> = {unbound:{BIND:"aliased"},aliased:{MUTATE:"copying",WITHDRAWN:"aliased",FINISH:"complete",CANCEL:"cancelled"},copying:{COPIED:"retiring",CANCEL:"cancelled"},retiring:{RETIRED:"owned",CANCEL:"cancelled"},owned:{MUTATE:"owned",FINISH:"complete",CANCEL:"cancelled"},complete:{},cancelled:{}};
const machine=setup({types:{events:{} as {type:string}}}).createMachine({initial:"unbound",states:Object.fromEntries(Object.entries(table).map(([name,events])=>[name,{on:Object.fromEntries(Object.entries(events).map(([event,target])=>[event,{target}]))}]))});
for(const row of fixture.cases)test(`retained genesis ${row.name}`,()=>{
  expect(new Ajv().compile(schema)(fixture)).toBe(true);
  let state="unbound", oracle=initialTransition(machine)[0];
  const actual:string[]=[], independent:string[]=[];
  for(const event of row.events){state=table[state]![event]??state;oracle=transition(machine,oracle,{type:event})[0];actual.push(state);independent.push(oracle.value as string);}
  expect(actual).toEqual(row.states);expect(independent).toEqual(row.states);
});
let repository=import.meta.dir;while(!readdirSync(repository).includes("nx.json"))repository=dirname(repository);
const sources=(root:string):string[]=>readdirSync(root,{withFileTypes:true}).flatMap(entry=>{const path=join(root,entry.name);return entry.isDirectory()?["node_modules","target","🗑️generated","🤖️generated"].includes(entry.name)?[]:sources(path):entry.name==="🦀️.rs"?[path]:[];});
const files=sources(join(repository,"✏️s/🔌️plugins")).map(path=>({path,text:readFileSync(path,"utf8")}));
for(const domain of fixture.domains)test(`${domain} initialization aliases genesis before bounded adoption`,()=>{
  const source=files.find(row=>row.text.includes(`enum ${domain}StoreInitializationPhase`))!;expect(source).toBeDefined();
  const begin=source.text.indexOf(`enum ${domain}StoreInitializationPhase`);const text=source.text.slice(begin,source.text.indexOf("pub fn",begin)>begin?source.text.indexOf("pub fn",begin):undefined);
  expect(text).toContain("BindGenesis");expect(text).toContain("genesis.share_snapshot()");expect(text).toContain("genesis.digest()");expect(text).toContain("adopt_current_owned");expect(text).toContain("settle_current_retirement_step");expect(text).not.toContain("artifact_initial_digest(&initial)");
});
