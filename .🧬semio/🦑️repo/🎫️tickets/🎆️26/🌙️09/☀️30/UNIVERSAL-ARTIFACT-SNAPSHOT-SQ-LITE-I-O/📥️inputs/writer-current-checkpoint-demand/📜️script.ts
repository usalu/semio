import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {join,relative} from "node:path";
import assert from "node:assert/strict";
const repo="/Users/ueli/Documents/semio",ticket=join(import.meta.dir,"../.."),base="✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/";
const observed=JSON.parse(readFileSync(join(ticket,"📥️inputs/writer-current-hub-tail-authority/captured-checkpoint-semantics.json"),"utf8"));
const law={schemaVersion:1,checkpoints:observed.map((entry:any)=>({index:entry.index,transition:{kind:"commit",...entry.checkpoint},clock:entry.timestamp}))};
const pairs:{path:string;before:string|null;after:string}[]=[];
function put(path:string,after:string){let before:string|null=null;try{before=readFileSync(path,"utf8");}catch{}pairs.push({path,before,after});}
function edit(path:string,old:string,next:string){const before=readFileSync(path,"utf8");assert.equal(before.split(old).length,2,path);pairs.push({path,before,after:before.replace(old,next)});}
const fixture=join(repo,base,"🧫️fixtures/🔁️hub-tail-after-check-in/🧭️current-checkpoints/🔣️.json");
put(fixture,JSON.stringify(law,null,2)+"\n");put(join(fixture,"../🧬️schema/🔣️.json"),JSON.stringify({$schema:"http://json-schema.org/draft-07/schema#",title:"Writer Current Checkpoint Semantics",const:law},null,2)+"\n");
const source=join(repo,base,"🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"),before=readFileSync(source,"utf8");
const sourceLaw=`
test("Writer hub tail commits retain the complete current transition and content address",async()=>{
 const {default:law}=await import("../../../../../🧫️fixtures/🔁️hub-tail-after-check-in/🧭️current-checkpoints/🔣️.json");
 const {default:schema}=await import("../../../../../🧫️fixtures/🔁️hub-tail-after-check-in/🧭️current-checkpoints/🧬️schema/🔣️.json");
 const {default:captured}=await import("../../../../../🧫️fixtures/🔁️hub-tail-after-check-in/🔣️.json");
 const {default:Ajv}=await import("ajv");expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const {historyTransitionId}=await import("../../../../../../../../../../../../../🧰️framework/🔨️modules/📡️replication/🟦️.ts");
 const {blake3}=await import("@noble/hashes/blake3.js");
 const {default:protocolSchema}=await import("../../../../../../../../../../../../../🧰️framework/🔨️modules/📡️replication/🔗️causal/🧬️schema/🔣️history-transition/🔣️.json");
 const validate=new Ajv({strict:true}).compile({$ref:"#/definitions/Transition",definitions:protocolSchema.definitions});
 for(const row of law.checkpoints){expect(validate(row.transition)).toBe(true);const t=row.transition,bytes:number[]=[];const u=(out:number[],n:number)=>{let value=BigInt(n);do{const part=Number(value&127n);value>>=7n;out.push(part|(value?128:0));}while(value);};const s=(value:string)=>{const text=Buffer.from(value,"utf8");u(bytes,text.length);bytes.push(...text);};const o=(value:string|null)=>{bytes.push(value===null?0:1);if(value!==null)s(value);};
 u(bytes,2);s(t.checkpointId);o(t.parentId);s(t.changeId);u(bytes,t.mutationIds.length);for(const id of t.mutationIds)s(id);o(t.description);s(t.savedAt);u(bytes,t.authors.length);for(const author of t.authors){s(author.id);s(author.name);o(author.avatar);}o(t.message);s(t.timestamp);o(t.lineId);
 const material:number[]=[];u(material,row.clock.actor);u(material,row.clock.physical_ms);u(material,row.clock.logical);u(material,bytes.length);material.push(...bytes);const expectedId="transition-"+Buffer.from(blake3(Uint8Array.from(material))).toString("hex").slice(0,16);expect(historyTransitionId(row.clock,bytes)).toBe(expectedId);
 console.log("[DEBUG] Writer current checkpoint producer index="+row.index+" bytes="+bytes.length+" id="+expectedId+" payload="+JSON.stringify(bytes));
 expect(captured.envelopes[row.index]!.diff.payload).toEqual(bytes);expect(captured.envelopes[row.index]!.mutation_id).toBe(expectedId);
 }
});
`;
pairs.push({path:source,before,after:before+sourceLaw});
const rust=join(repo,base,"🚪️io/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs");
const prelude=`    let current: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🔁️hub-tail-after-check-in/🧭️current-checkpoints/🔣️.json")).expect("closed current checkpoint semantics");
    let current_rows = current["checkpoints"].as_array().expect("current checkpoint rows");
    assert_eq!(current_rows.len(), 2);
    for row in current_rows {
        let value = &row["transition"];
        let text = |key: &str| value[key].as_str().expect("current checkpoint string").to_owned();
        let optional = |key: &str| value[key].as_str().map(str::to_owned);
        let checkpoint = protocol::TransitionCheckpoint { checkpoint_id: text("checkpointId"), parent_id: optional("parentId"), change_id: text("changeId"), mutation_ids: value["mutationIds"].as_array().unwrap().iter().map(|id| protocol::MutationId(id.as_str().unwrap().to_owned())).collect(), description: optional("description"), saved_at: text("savedAt"), authors: value["authors"].as_array().unwrap().iter().map(|author| protocol::TransitionAuthor { id: author["id"].as_str().unwrap().to_owned(), name: author["name"].as_str().unwrap().to_owned(), avatar: author["avatar"].as_str().map(str::to_owned) }).collect(), message: optional("message"), timestamp: text("timestamp"), line_id: optional("lineId") };
        let transition = protocol::HistoryTransition::Commit(checkpoint);
        let payload = protocol::encode_history_transition(&transition);
        assert_eq!(protocol::decode_history_transition(&payload).expect("current producer decoder"), transition);
        let envelope = hub_tail_envelope(&tail[row["index"].as_u64().unwrap() as usize]);
        let id = protocol::history_transition_id(&envelope.timestamp, &payload);
        println!("[DEBUG] Writer current Native checkpoint bytes={} id={} full_semantics=true", payload.len(), id.0);
        assert_eq!(payload, envelope.diff.payload, "captured checkpoint includes the explicit current line field");
        assert_eq!(id, envelope.mutation_id, "current checkpoint address covers all bytes");
    }
`;
edit(rust,'    let mut folded = new_writer_store',prelude+'    let mut folded = new_writer_store');
writeFileSync(join(import.meta.dir,"guarded-pairs.json"),JSON.stringify(pairs,null,2)+"\n");
for(const pair of pairs){let current:string|null=null;try{current=readFileSync(pair.path,"utf8");}catch{}assert.equal(current,pair.before,"Concurrent owner change "+relative(repo,pair.path));}
for(const pair of pairs){mkdirSync(join(pair.path,".."),{recursive:true});writeFileSync(pair.path,pair.after);}
console.log("[DEBUG] Writer current checkpoint closed neutral schema and Source/Native demands mounted paths="+pairs.length+" original_fold_predicates_preserved=true");
