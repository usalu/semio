// [asm-g] cost of a full membership walk (workspaceBuildFilesV1 + stamping every directory and member) vs the stat proof of the previous frontier.
import{readFileSync}from"node:fs";import{join}from"node:path";import{lstat}from"node:fs/promises";import{brotliDecompressSync}from"node:zlib";
const ROOT="/Users/ueli/Documents/semio",D=process.argv[2],B=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
process.chdir(ROOT);
const g=await import(B+"🕸️graph/🕰️generations/🟨️.mjs"),graph=await import(B+"🕸️graph/🟦️.ts");
const cpu=()=>{const u=process.cpuUsage();return Math.round((u.user+u.system)/1000);};
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const frontier=JSON.parse(brotliDecompressSync(readFileSync(join(D,"f",manifest.frontier+".json.br"))));
console.log("frontier dirs",frontier.directories.length,"members",frontier.members.length);
const statEntry=async p=>{try{const s=await lstat(join(ROOT,p));return{directory:s.isDirectory(),file:s.isFile(),mtimeMs:s.mtimeMs,ino:s.ino,mode:s.mode};}catch(e){return undefined;}};
let w=performance.now(),c=cpu();const holds=await g.frontierHoldsV1(frontier,statEntry);console.log("frontierHolds",holds,"ms",Math.round(performance.now()-w),"cpu",cpu()-c);
// count how many dirs/members moved
let moved=0,movedMembers=0,gone=0;{let next=0;const rows=frontier.directories;await Promise.all(Array.from({length:64},async()=>{while(next<rows.length){const row=rows[next++],s=await statEntry(row[0]);if(!s)gone++;else if(s.mtimeMs!==row[1]||s.ino!==row[2])moved++;}}));}
console.log("directories moved",moved,"gone",gone);
w=performance.now();c=cpu();const all=await graph.workspaceBuildFilesV1(ROOT);console.log("walk files",all.length,"ms",Math.round(performance.now()-w),"cpu",cpu()-c);
w=performance.now();c=cpu();{const members=graph.buildGraphMembersV1(all);let next=0;await Promise.all(Array.from({length:64},async()=>{while(next<members.length){await statEntry(members[next++]);}}));console.log("stamp members",members.length,"ms",Math.round(performance.now()-w),"cpu",cpu()-c);}
// advance vs full walk on the real tree
{
 const{advanceFrontierV1,walkFrontierV1}=await import(B+"🕸️graph/🕰️generations/🌿️frontier-walk/🟨️.mjs"),{readdir}=await import("node:fs/promises");
 const listEntry=async d=>{try{return(await readdir(join(ROOT,d),{withFileTypes:true})).map(e=>({name:e.name,kind:e.isSymbolicLink()?"other":e.isDirectory()?"directory":e.isFile()?"file":"other"}));}catch{return undefined;}};
 const descends=graph.buildWalkDescendsV1(),member=graph.graphMemberPredicateV1(),ports={stat:statEntry,list:listEntry,descend:(n,p,parent)=>descends(n,p,parent==="."?"":parent),member};
 w=performance.now();c=cpu();const adv=await advanceFrontierV1({...ports,frontier});console.log("advance","unchanged" in adv?"unchanged":"listed "+adv.listed,"ms",Math.round(performance.now()-w),"cpu",cpu()-c);
 w=performance.now();c=cpu();const fresh=await walkFrontierV1(ports);console.log("walk (new module)","dirs",fresh.directories.length,"members",fresh.members.length,"ms",Math.round(performance.now()-w),"cpu",cpu()-c);
 const sh=x=>JSON.stringify({d:x.directories.map(r=>r[0]).sort(),m:[...x.members].map(r=>[r[0],r[1]]).sort((a,b)=>a[0]<b[0]?-1:1)});
 console.log("advance equals walk:",sh(adv)===sh(fresh));
 const oldMembers=graph.buildGraphMembersV1(all).sort(),newMembers=fresh.members.map(r=>r[0]).sort();
 console.log("members equal workspaceBuildFilesV1 members:",JSON.stringify(oldMembers)===JSON.stringify(newMembers),oldMembers.length,newMembers.length);
 const oldDirs=[...(graph.__dirs?.(all)??[])];
}
