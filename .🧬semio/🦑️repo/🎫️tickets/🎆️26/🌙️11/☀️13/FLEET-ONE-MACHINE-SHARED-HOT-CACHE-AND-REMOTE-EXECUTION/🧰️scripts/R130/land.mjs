import {readFileSync,writeFileSync,mkdirSync,existsSync,rmSync} from "node:fs";
import {createHash} from "node:crypto";
import {join,basename,dirname} from "node:path";
import {PATCHES,NEWS,NEWS_ROOT,applyEdits} from "./patches.mjs";
const BACKUP="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R130/backup";
const steps=(process.env.R130_STEPS??"1").split(",").map(Number),wanted=item=>steps.includes(item.step);
const dry=process.argv.includes("--dry"),sha=text=>createHash("sha256").update(text).digest("hex").slice(0,12);
mkdirSync(BACKUP,{recursive:true});
const plan=[];
for(const{file,edits}of PATCHES.filter(wanted)){
 let text=readFileSync(file,"utf8");const before=sha(text);
 text=applyEdits(text,edits,file);
 plan.push({file,text,before,after:sha(text)});
}
const created=[];
for(const item of NEWS.filter(wanted)){const text=readFileSync(join(NEWS_ROOT,item.from),"utf8");if(existsSync(item.to)&&readFileSync(item.to,"utf8")!==text){console.error("exists and differs",item.to);process.exit(4);}created.push({file:item.to,text});}
if(dry){for(const row of plan)console.log("dry",row.before,"->",row.after,basename(row.file));for(const row of created)console.log("dry new",basename(row.file));process.exit(0);}
for(const row of plan){const key=join(BACKUP,sha(row.file)+".orig");if(!existsSync(key))writeFileSync(key,readFileSync(row.file));writeFileSync(join(BACKUP,sha(row.file)+".path"),row.file);}
writeFileSync(join(BACKUP,"created.json"),JSON.stringify(created.filter(row=>!existsSync(row.file)).map(row=>row.file)));
for(const row of plan){writeFileSync(row.file,row.text);console.log("landed",row.before,"->",row.after,basename(row.file));}
for(const row of created){if(!existsSync(row.file)){mkdirSync(dirname(row.file),{recursive:true});writeFileSync(row.file,row.text);console.log("created",basename(row.file));}}
