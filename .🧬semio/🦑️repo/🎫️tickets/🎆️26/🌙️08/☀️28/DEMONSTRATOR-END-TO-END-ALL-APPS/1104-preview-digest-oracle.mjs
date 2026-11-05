import{readFileSync}from"node:fs";import{Database}from"bun:sqlite";import Ajv from"ajv";
const base="✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🌊️flow-source/🔢️digest";
const corpus=JSON.parse(readFileSync(base+"/🔣️.json","utf8")),schema=JSON.parse(readFileSync(base+"/🧬️schema.json","utf8")),ajv=new Ajv({strict:true});if(!ajv.validate(schema,corpus))throw Error(JSON.stringify(ajv.errors));
const db=new Database(":memory:");try{
const step=db.query("with x as (select ((?1 | ?3)-(?1 & ?3)) as lo,?2 as hi) select (lo*435)%4294967296 as lo,(hi*435+(lo*435)/4294967296+(lo<<8))%4294967296 as hi from x");
for(const row of corpus.cases){let lo=0x84222325,hi=0xcbf29ce4;for(const part of row.parts){for(const byte of [...Buffer.from(part),31]){({lo,hi}=step.get(lo,hi,byte));}}const actual=(((BigInt(hi)<<32n)|BigInt(lo))^BigInt.asUintN(64,BigInt(row.provisionalGeneration)*0x9e3779b97f4a7c15n)).toString(16).padStart(16,"0");if(actual!==row.expected)throw Error(row.id+" expected "+row.expected+" actual "+actual);console.log(row.id+" "+actual);}
}finally{db.close()}
