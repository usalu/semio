import {join} from "node:path";
import assert from "node:assert/strict";
const repo="/Users/ueli/Documents/semio";
const ticket=join(repo,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O");
const path=join(repo,"✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🔁️hub-tail-after-check-in/🔣️.json");
const fixture=await Bun.file(path).json();
const rows=[];
for(const [index,envelope]of fixture.envelopes.entries())if(envelope.diff.schema==="semio.history.transition"){
 const bytes=new Uint8Array(envelope.diff.payload);let pos=0;
 const integer=()=>{let result=0n,shift=0n;for(;;){assert(pos<bytes.length);const byte=bytes[pos++]!;result|=BigInt(byte&127)<<shift;if(byte<128){assert(result<=BigInt(Number.MAX_SAFE_INTEGER));return Number(result);}shift+=7n;assert(shift<64n);}};
 const string=()=>{const length=integer();assert(pos+length<=bytes.length);const value=new TextDecoder("utf-8",{fatal:true}).decode(bytes.subarray(pos,pos+length));pos+=length;return value;};
 const optional=()=>{assert(pos<bytes.length);const tag=bytes[pos++]!;assert(tag<=1);return tag===0?null:string();};
 assert.equal(integer(),2);
 const checkpointId=string(),parentId=optional(),changeId=string();const mutationIds=Array.from({length:integer()},string);const description=optional(),savedAt=string();const authors=Array.from({length:integer()},()=>({id:string(),name:string(),avatar:optional()}));const message=optional(),timestamp=string();
 assert.equal(pos,bytes.length,"Captured commit already contains trailing authority bytes");
 rows.push({index,mutationId:envelope.mutation_id,payloadBytes:bytes.length,checkpoint:{checkpointId,parentId,changeId,mutationIds,description,savedAt,authors,message,timestamp,lineId:null},timestamp:envelope.timestamp,dependencies:envelope.dependencies});
 console.log("[DEBUG] independent Writer captured checkpoint index="+index+" semantic_end="+pos+" payload_bytes="+bytes.length+" current_optional_line_field_missing=true");
}
assert.equal(rows.length,2);
await Bun.write(join(ticket,"📥️inputs/writer-current-hub-tail-authority/captured-checkpoint-semantics.json"),JSON.stringify(rows,null,2)+"\n");
await Bun.write(join(ticket,"📓️writer-current-hub-tail-transition-authority-readback.md"),"# Writer Current Hub Tail Transition Authority\n\nActual full214 owning gate fails strict current checkpoint transition decoding at byte281 before remote fold. This independent Bun/standard UTF8 readback consumes the exact captured checkpoint fields through timestamp and reaches the exact payload end for both commits. Current Replication decode requires the final optional line-id field; the captured fixture omits it. No parser/default/legacy acceptance is authored.\n\n"+rows.map(row=>"- Envelope"+row.index+": commit payload"+row.payloadBytes+"bytes; full named semantic fields retained in [closed readback](📥️inputs/writer-current-hub-tail-authority/captured-checkpoint-semantics.json). Expected current authored line identity is explicitly absent (`null`), rather than a default language/line or old wire branch.\n").join("")+"\nA genuine first-party current encoder and content-address witness must precede handcrafting both payloads/transition IDs and exact dependency references. All17-envelope/mixed15hub+2local original folding and pack reload assertions remain unchanged. This report is source/fixture research, not a successful current decoder or runtime fold receipt.\n");
