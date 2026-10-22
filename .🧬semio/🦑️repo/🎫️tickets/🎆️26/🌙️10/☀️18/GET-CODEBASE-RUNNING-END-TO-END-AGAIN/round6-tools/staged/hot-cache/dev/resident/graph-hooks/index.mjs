/** 🕰 Seams for the graph generation load: membership proof and fact checks answered from the resident journal instead of stat passes. Pure functions over service replies. */
import {createHash} from "node:crypto";

const byCodePoint=(a,b)=>a<b?-1:a>b?1:0;

function canonicalMembers(members){return JSON.stringify(members);}

/** 🖇 Applies journal changes to a previous frontier and returns the host's membership object, or undefined when the journal cannot vouch (caller falls back to the stat pass). */
export function membershipFromJournalV1(previous,reply,{now=Date.now(),isConfig=path=>["📋️project.json","Cargo.toml"].includes(path.slice(path.lastIndexOf("/")+1)),digest=members=>createHash("sha256").update(canonicalMembers(members)).digest("hex")}={}){
 if(!previous||!reply||reply.complete!==true)return undefined;
 if(reply.changes.length===0)return{digest:previous.manifest.membership,frontier:previous.frontier,changes:[],cursor:reply.cursor};
 const directories=new Map(previous.frontier.directories.map(row=>[row[0],row])),members=new Map(previous.frontier.members.map(row=>[row[0],row])),changes=[];
 for(const change of reply.changes){
  const table=change.directory?directories:members,was=table.get(change.path);
  if(change.kind==="removed"){if(was)table.delete(change.path);changes.push({path:change.path,kind:"removed",directory:change.directory});continue;}
  const row=change.directory?[change.path,change.stamp[0],change.stamp[1]]:[change.path,change.mode];
  table.set(change.path,row);
  const kind=was?"changed":"added";
  if(was&&(change.directory?was[1]===row[1]&&was[2]===row[2]:was[1]===row[1]))continue;
  changes.push({path:change.path,kind,directory:change.directory,...(!change.directory&&isConfig(change.path)?{config:true}:{})});
 }
 const frontier={version:1,capturedAt:now,directories:[...directories.values()].sort((a,b)=>byCodePoint(a[0],b[0])),members:[...members.values()].sort((a,b)=>byCodePoint(a[0],b[0]))};
 changes.sort((a,b)=>byCodePoint(a.path,b.path));
 return{digest:changes.some(c=>!c.directory)?digest(frontier.members):previous.manifest.membership,frontier,changes,cursor:reply.cursor};
}

/** 🧭 Paths whose facts may have moved: every changed path, the directory of every added or removed entry, and everything below a changed directory. */
export function dirtyPathsFromJournalV1(reply){
 if(!reply||reply.complete!==true)return undefined;
 const exact=new Set(),subtrees=[];
 for(const change of reply.changes){
  exact.add(change.path);
  const cut=change.path.lastIndexOf("/");exact.add(cut<0?"":change.path.slice(0,cut));
  if(change.directory&&change.kind!=="changed")subtrees.push(change.path+"/");
 }
 return{has:path=>exact.has(path)||subtrees.some(prefix=>path.startsWith(prefix)),size:exact.size};
}

/** ✅ Fact checker that trusts rows whose path the journal proves untouched since the generation was verified and asks `fallback` about the rest. */
export function createResidentHoldsV1({dirty,fallback}){
 return async(rows,signal)=>{
  const set=await dirty();
  if(!set)return fallback(rows,signal);
  const doubtful=rows.filter(row=>set.has(row.path));
  return doubtful.length===0?true:fallback(doubtful,signal);
 };
}
