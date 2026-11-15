import {readFileSync} from "node:fs";import {createHash} from "node:crypto";import {brotliDecompressSync} from "node:zlib";
import {readGenerationManifestV1,readGenerationPointerV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs";
import {generationDocumentPathsV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.new.ts";
import {transformGenerationV1,generationSignatureV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🧳️portable/🟦️.ts";
const store=import.meta.dir+"/mnt-g/generations",root="/Users/ueli/Documents/semio",other="/srv/other/semio-b";
const pointer=readGenerationPointerV1(readFileSync(store+"/current.json")),manifest=readGenerationManifestV1(readFileSync(`${store}/m/${pointer.manifest}.json`));
const paths=generationDocumentPathsV1(pointer.manifest!,manifest);
const docs:any[]=[];let missing=0;for(const p of paths){try{docs.push({path:p,bytes:readFileSync(`${store}/${p}`)});}catch{missing++;}}
console.log("documents",docs.length,"missing",missing);
if(missing){console.log("incomplete generation on disk (retired fragments); using present subset not possible");process.exit(0);}
let t=performance.now();const neutral=transformGenerationV1(docs,root,"neutralize");console.log("neutralize ms",Math.round(performance.now()-t),"docs",neutral.documents.length);
let left=0;for(const d of neutral.documents)if(d.path.startsWith("x/")||d.path.startsWith("o/")||d.path.startsWith("f/")){if(Buffer.from(brotliDecompressSync(d.bytes)).includes(root))left++;}
console.log("documents still holding the root",left);
t=performance.now();const local=transformGenerationV1(neutral.documents,other,"expand");console.log("expand ms",Math.round(performance.now()-t));
const m2=readGenerationManifestV1(local.documents.find(d=>d.path===local.manifestPath)!.bytes);console.log("signature equal",generationSignatureV1(m2)===generationSignatureV1(manifest),"owners",Object.keys(m2.owners).length);
let withOther=0,withOld=0;for(const d of local.documents){if(d.path.startsWith("m/"))continue;const raw=Buffer.from(brotliDecompressSync(d.bytes));if(raw.includes(other))withOther++;if(raw.includes(root))withOld++;}
console.log("docs with new root",withOther,"with old root",withOld);
