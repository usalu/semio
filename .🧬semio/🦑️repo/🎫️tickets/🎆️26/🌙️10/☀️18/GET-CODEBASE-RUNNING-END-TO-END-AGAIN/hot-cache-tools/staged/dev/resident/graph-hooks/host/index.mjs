/** 🔌 The two seams of the graph generation host (membership proof and fact checks) answered from the resident journal when a service for this worktree is up, and by the unchanged stat code otherwise. */
import {forwardCommandV1} from "../../client/index.mjs";
import {dirtyPathsFromJournalV1,membershipFromJournalV1} from "../index.mjs";

const COALESCE_MILLISECONDS=250;

/** 🔌 Wraps `membership` and `holds` of the host. `adopt(files)` receives the member list after a journal answer, like the stat path stores it. */
export function residentGraphSeamsV1({root,membership,holds,adopt=()=>undefined,environment=process.env,ask=undefined,now=()=>Date.now()}){
 let down=false,current,dirtyCache;
 const call=ask??(async(command,args)=>{
  if(environment.SEMIO_BUILD_SERVICE==="off"||down)return undefined;
  const out=[];
  let result;
  try{result=await forwardCommandV1({root,command,args,cwd:root,languages:["en","de"],environment,stdout:{write:text=>out.push(String(text))},stderr:{write(){}}});}catch{down=true;return undefined;}
  if(!result.handled){down=true;return undefined;}
  if(result.exit!==0)return undefined;
  try{return JSON.parse(out.join(""));}catch{return undefined;}
 });
 const remember=async(digest,cursor)=>{if(cursor&&typeof cursor.instance==="string")await call("adopt-frontier",[digest,cursor.instance+":"+cursor.seq]);};
 return Object.freeze({
  async membership(previous,signal){
   if(previous){
    const reply=await call("dirty",["--digest",previous.manifest.membership]);
    const answer=membershipFromJournalV1(previous,reply);
    if(answer){
     adopt(answer.frontier.members.map(row=>row[0]));current=answer.digest;
     await remember(answer.digest,answer.cursor);
     return{digest:answer.digest,frontier:answer.frontier,changes:answer.changes};
    }
   }
   const cursor=await call("cursor",[]);
   const result=await membership(previous,signal);
   current=result.digest;
   await remember(result.digest,cursor);
   return result;
  },
  async holds(rows,signal){
   if(current===undefined)return holds(rows,signal);
   const digest=current;
   if(!dirtyCache||dirtyCache.digest!==digest||now()-dirtyCache.at>COALESCE_MILLISECONDS)dirtyCache={digest,at:now(),promise:call("dirty",["--digest",digest]).then(dirtyPathsFromJournalV1)};
   const set=await dirtyCache.promise;
   if(!set)return holds(rows,signal);
   const doubtful=rows.filter(row=>set.has(row.path));
   return doubtful.length===0?true:holds(doubtful,signal);
  },
 });
}
