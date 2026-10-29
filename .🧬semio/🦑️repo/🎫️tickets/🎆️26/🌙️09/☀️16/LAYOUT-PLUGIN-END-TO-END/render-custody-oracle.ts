import {operationEntityId} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧬️schema/🪪️entity-id/🟦️.ts";
import fixture from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/🧪️tests/🧫️fixtures/🪪️entity-id/🔣️.json";
import {sha256} from "@noble/hashes/sha2.js";
import Ajv from "ajv";
const encoder=new TextEncoder(),admitted=new Ajv({strict:true}).compile({type:"string",minLength:1});
for(const row of fixture.rows){const field=(value:string)=>encoder.encode(value).length+":"+value;const oracle=row.prefix+"-"+Buffer.from(sha256(encoder.encode(field(row.documentId)+field(row.authoringSeed)+field(row.localKey)))).toString("hex").slice(0,16);const actual=operationEntityId({parentDocumentId:row.documentId,authoringSeed:row.authoringSeed},row.prefix,row.localKey);if(actual!==oracle||actual!==row.expected)throw Error(row.name);}
for(const row of fixture.readOnly){const seed="";let accepted=true;try{operationEntityId({parentDocumentId:row.documentId,authoringSeed:seed},"frame","created")}catch{accepted=false}if(accepted!==row.authoringAccepted||admitted(seed)!==row.authoringAccepted)throw Error(row.name);}
console.log(JSON.stringify({entityIdCases:fixture.rows.length,readOnlyDeniedCases:fixture.readOnly.length}));
