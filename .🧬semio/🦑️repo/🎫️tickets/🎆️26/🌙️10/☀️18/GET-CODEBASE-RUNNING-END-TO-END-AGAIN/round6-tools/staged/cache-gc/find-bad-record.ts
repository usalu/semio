import {readdirSync,readFileSync,statSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {createHash} from "node:crypto";
import {join} from "node:path";

const root=process.argv[2]??"/Users/ueli/Documents/semio/.cache/b/g/m/inference",hex=(v:unknown)=>typeof v==="string"&&/^[a-f0-9]{64}$/u.test(v);
const names=/^(?:[a-f0-9]{24}\.json|emoji-project-json\.json|test-cases\.json)$/u;
let checked=0;const continue_=()=>{};const bad:string[]=[];
function walk(directory:string){
 for(const entry of readdirSync(directory,{withFileTypes:true})){
  const path=join(directory,entry.name);
  if(entry.isDirectory()){if(entry.name.endsWith(".records")||entry.name==="ob")continue;walk(path);continue;}
  if(!entry.isFile()||!names.test(entry.name))continue;
  checked++;
  try{
   const bytes=readFileSync(path);if(!bytes.length)throw Error("empty file");
   if(path.includes("/rr/")){const leaf=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(brotliDecompressSync(bytes)));if(!leaf||typeof leaf!=="object"||Array.isArray(leaf)||leaf.version!==1||typeof leaf.crate!=="string"||!hex(leaf.check))throw Error("rr leaf shape");continue;}
   if(path.includes("/ip/")&&!path.includes("/np/")){const text0=new TextDecoder("utf-8",{fatal:true}).decode(brotliDecompressSync(bytes));if(!text0.includes("\n")||text0.indexOf("\n")<1){const leaf=JSON.parse(text0);if(leaf.version!==1||typeof leaf.tsconfig!=="string"||!Array.isArray(leaf.chain))throw Error("ip leaf shape");continue;}}
   const text=new TextDecoder("utf-8",{fatal:true}).decode(brotliDecompressSync(bytes)),at=text.indexOf("\n");
   if(at<1)throw Error("no header line");
   const header=JSON.parse(text.slice(0,at)),body=text.slice(at+1);
   if(Object.keys(header).sort().join()!=="authorityObservations,blocks,digest,observations,resultSha256"&&Object.keys(header).sort().join()!=="authorityObservations,digest,observations,resultSha256")throw Error("header keys "+Object.keys(header).join());
   if(!hex(header.digest)||!hex(header.resultSha256))throw Error("header digests");
   if(createHash("sha256").update(body).digest("hex")!==header.resultSha256)throw Error("result hash mismatch");
   JSON.parse(body);
  }catch(error){bad.push(path+"\t"+statSync(path).size+"\t"+(error as Error).message);}
 }
}
walk(root);
console.log(JSON.stringify({checked,bad:bad.length}));
for(const row of bad)console.log(row);
