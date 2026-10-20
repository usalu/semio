import {mock} from "bun:test";
import {readFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {fileURLToPath} from "node:url";
import {resolve} from "node:path";

const revision=resolve(import.meta.dir,"../../../../../../../..","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🟨️.mjs");
const read=(path:string,_maximum:number,retain=false)=>{const bytes=readFileSync(path);return{sha256:createHash("sha256").update(bytes).digest("hex"),byteLength:bytes.length,text:retain?bytes.toString("utf8"):null};};
mock.module(revision,()=>({physicalSourceOriginV1:(moduleURL:string,compiled?:string)=>compiled??moduleURL,capturePhysicalSourceTextV1:(url:string,maximum:number)=>read(fileURLToPath(url),maximum,true).text,captureBoundedSourceRevisionV1:read}));
