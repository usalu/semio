import { readFileSync } from "node:fs";
import { strict as assert } from "node:assert";
import Ajv from "ajv";
const bytes={type:"array",items:{type:"integer",minimum:0,maximum:255}};
const object=(properties:Record<string,unknown>)=>({type:"object",properties,required:Object.keys(properties),additionalProperties:false});
const natural={type:"integer",minimum:0};
const text={type:"string"};
const decimal={type:"string",pattern:"^(0|[1-9][0-9]*)$"};
const word={...natural,maximum:4294967295};
const counter={...natural,maximum:18446744073709551615};
const limits={type:"array",minItems:11,maxItems:11,items:[word,word,word,word,word,counter,counter,counter,word,word,word],additionalItems:false};
const capability={...object({token:{...decimal,maxLength:39},id:text,scope:text,expiresMs:natural}),required:["token","id","scope"]};
const instanceOpen=object({
    request:object({activationGeneration:decimal,instanceId:natural,requestSequence:natural}),
    appId:text,actor:text,config:bytes,assets:{type:"array"},capabilities:{type:"array",items:capability},quotas:{type:"object"},
});
const inputs={Poll:object({events:{type:"array",items:object({instanceOpen})},command_page:{type:"null"},cold_pair_page:{type:"null"},limits}),Codec:object({artifact_schema:text,document_id:text,pack:bytes,spr:bytes,ops:bytes}),Restore:object({state:bytes})};
const schema={oneOf:Object.entries(inputs).map(([tag,payload])=>object({[tag]:payload}))};
const validate=new Ajv({strict:true}).compile(schema);
const rows=JSON.parse(readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🔁️persistent-local/📊️actor-input.json","utf8"));
for(const row of rows)assert.equal(validate(row.wire),row.valid,JSON.stringify(row.wire));
console.log(`actor input: ${rows.length} independent Ajv cases passed`);
