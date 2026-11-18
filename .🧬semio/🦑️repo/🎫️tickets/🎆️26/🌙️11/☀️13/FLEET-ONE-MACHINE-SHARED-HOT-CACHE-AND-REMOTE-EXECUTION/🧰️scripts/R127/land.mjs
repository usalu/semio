import {readFileSync,writeFileSync,mkdirSync,existsSync} from "node:fs";
import {createHash} from "node:crypto";
import {basename,join} from "node:path";
import {PATCHES} from "./patches.mjs";
const BACKUP="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R127/backup";
const dry=process.argv.includes("--dry"),sha=text=>createHash("sha256").update(text).digest("hex").slice(0,12);
mkdirSync(BACKUP,{recursive:true});
const plan=[];
for(const{file,edits}of PATCHES){
 let text=readFileSync(file,"utf8");const before=sha(text);
 for(const[from,to]of edits){const count=text.split(from).length-1;if(count!==1){console.error("anchor count",count,file,from.slice(0,80));process.exit(4);}text=text.replace(from,()=>to);}
 plan.push({file,text,before,after:sha(text)});
}
if(dry){for(const row of plan)console.log("dry",row.before,"->",row.after,basename(row.file));process.exit(0);}
for(const row of plan){const key=join(BACKUP,sha(row.file)+".orig");if(!existsSync(key))writeFileSync(key,readFileSync(row.file));writeFileSync(join(BACKUP,sha(row.file)+".path"),row.file);}
for(const row of plan){writeFileSync(row.file,row.text);console.log("landed",row.before,"->",row.after,basename(row.file));}
