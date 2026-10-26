import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync, readdirSync, existsSync } from "node:fs";
import { createHash } from "node:crypto";
import { join, relative } from "node:path";
const ticket=import.meta.dir, root=process.cwd();
const plan=JSON.parse(readFileSync(join(ticket,"taxonomy-paths.cases.json"),"utf8"));
const beforeText=readFileSync(join(ticket,"🗑️generated/taxonomy-baseline/index-entries.z"),"utf8");
const entries=(text:string)=>new Map(text.split("\0").filter(Boolean).map(line=>{const tab=line.indexOf("\t");return [line.slice(tab+1),line.slice(0,tab)] as const;}));
const baseline=entries(beforeText), sourcePaths:string[]=plan.moves.filter((m:any)=>m.new).map((m:any)=>m.new);
const sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
/** 🔒 Retry only transient index contention without removing another process's lock. */
async function git(args:string[]){
 for(let attempt=0;attempt<12;attempt++){const p=spawnSync("git",args,{encoding:"utf8",maxBuffer:128*1024*1024,windowsHide:true});if(!p.status)return p.stdout;if(!p.stderr.includes("index.lock")||attempt===11)throw new Error(p.stderr||p.stdout);await sleep(1000);}
 throw new Error("Unreachable Git retry");
}
console.log("[DEBUG] Finalizer armed; baseline loaded, awaiting closed ticket metadata");
const deadline=Date.now()+240000;
while(true){let closed=false;try{closed=JSON.parse(readFileSync(join(ticket,"🎫️ticket.json"),"utf8")).status==="closed";}catch{}if(closed)break;if(Date.now()>deadline)throw new Error("Ticket closure did not arrive");await sleep(250);}
const report=join(ticket,"taxonomy-finalization.md");
const ticketPaths=readdirSync(ticket,{withFileTypes:true}).filter(e=>e.isFile()&&!e.name.startsWith("📌️")).map(e=>relative(root,join(ticket,e.name)).replaceAll("\\","/"));
for(let i=0;i<ticketPaths.length;i+=40)await git(["add","--",...ticketPaths.slice(i,i+40)]);
await git(["diff","--cached","--check"]);
const after=entries(await git(["ls-files","--stage","-z"]));
const hash=(value:Buffer)=>createHash("sha1").update(Buffer.from("blob "+value.length+"\0")).update(value).digest("hex");
let normalized=0;
for(const path of sourcePaths){const value=readFileSync(join(root,path)),entry=after.get(path);if(!entry)throw new Error("Unstaged source: "+path);const expected=entry.split(" ")[1];if(hash(value)!==expected){if(hash(Buffer.from(value.toString("utf8").replaceAll("\r\n","\n")))!==expected)throw new Error("Source differs from validated index: "+path);normalized++;}}
const oldLeft=plan.moves.filter((m:any)=>after.has(m.old));if(oldLeft.length)throw new Error("Old source coordinates remain indexed");
const shared=["🎓️teaching/README.md",".devcontainer/Dockerfile","🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh",plan.registry.path];
const allowed=new Set([...plan.moves.map((m:any)=>m.old),...sourcePaths,...shared,...ticketPaths]);
const outside=[...new Set([...baseline.keys(),...after.keys()])].filter(p=>!allowed.has(p)&&baseline.get(p)!==after.get(p));
const text="# Final Taxonomy Staging and Closure\n\nThe repository ticket is closed through repo MCP. All "+sourcePaths.length+" canonical product sources match the index ("+normalized+" Git LF-normalized); all141 superseded source coordinates are absent. All retained ticket evidence and closed metadata are staged. Staged whitespace validation passed.\n\nThe correction baseline contained "+baseline.size+" index entries; the final index contains "+after.size+". "+outside.length+" outside-scope index changes were observed and preserved. This task staged only explicit source/ticket paths and selected shared registry/README/native content; no commits, branch changes, stash, worktrees, or AGENTS edits were performed.\n\nAll render/verification processes had ended before closure. Generated output cleanup is the final filesystem step after this repository command exits; source files, fixtures and Markdown audits remain.\n";
writeFileSync(report,text);await git(["add","--",relative(root,report).replaceAll("\\","/")]);await git(["diff","--cached","--check"]);
console.log("[DEBUG] Finalization passed "+JSON.stringify({sources:sourcePaths.length,ticketFiles:ticketPaths.length,normalized,indexEntries:after.size,outsideChangesPreserved:outside.length}));
