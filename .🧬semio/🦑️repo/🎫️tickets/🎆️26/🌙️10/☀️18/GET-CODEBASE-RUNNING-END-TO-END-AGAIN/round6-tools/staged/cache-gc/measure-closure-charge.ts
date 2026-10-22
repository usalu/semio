import {lstatSync,readdirSync,readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {join,relative,sep} from "node:path";

const root=process.argv[2]??"/Users/ueli/Documents/semio/.cache/b/g/m",rows:Record<string,{files:number;bytes:number;expanded:number}>={};
const add=(kind:string,bytes:number,expanded:number)=>{const row=rows[kind]??={files:0,bytes:0,expanded:0};row.files++;row.bytes+=bytes;row.expanded+=expanded;};
const brotli=/\/(?:np|ip|rr)\/[a-f0-9]{24}\.json$|\/(?:emoji-project-json|test-cases)\.json$|\/ob\/[a-f0-9]{64}\.br$|^generations\/[ofx]\/|^generations\/f\//u;
function walk(directory:string){
 for(const entry of readdirSync(directory,{withFileTypes:true})){
  const path=join(directory,entry.name),name=relative(root,path).split(sep).join("/");
  if(entry.isDirectory()){walk(path);continue;}
  if(!entry.isFile()||name.startsWith(".")||name==="storage-events.jsonl"||name==="graph.json"||name.endsWith(".sqlite"))continue;
  const size=lstatSync(path).size;
  if(brotli.test("/"+name)||brotli.test(name)){let expanded=0;try{expanded=brotliDecompressSync(readFileSync(path)).length;}catch{}add(name.includes("/ob/")?"observation":name.startsWith("generations/")?"generation":"inference",size,expanded);}
  else add(name.startsWith("build-input-closures")||name.startsWith("build-lock-closures")?"closure":name.endsWith(".pack")?"pack":name.endsWith(".idx")?"index":"other",size,0);
 }
}
walk(root);
let total=0;for(const[kind,row]of Object.entries(rows)){const charge=row.bytes+row.expanded;total+=charge;console.log(kind.padEnd(12),String(row.files).padStart(7),(row.bytes/2**20).toFixed(0).padStart(7)+" MiB read",(row.expanded/2**20).toFixed(0).padStart(7)+" MiB expanded");}
console.log("charge per capture (read + expanded):",(total/2**30).toFixed(2),"GiB; 16 GiB cap exhausted after",Math.floor(17179869184/total),"captures");
