import {readFileSync,writeFileSync,mkdirSync,existsSync} from "node:fs";
import {createHash} from "node:crypto";
import {join,basename,dirname} from "node:path";
import {PATCHES,NEWS,SC,HERE,applyEdits} from "./patches.mjs";
const BACKUP=SC+"backup",dry=process.argv.includes("--dry"),sha=text=>createHash("sha256").update(text).digest("hex").slice(0,12);
mkdirSync(BACKUP,{recursive:true});
const plan=[];
for(const{file,edits}of PATCHES){const text=readFileSync(file,"utf8"),next=applyEdits(text,edits,file);plan.push({file,text:next,before:sha(text),after:sha(next)});}
const created=[];
for(const item of NEWS){const text=readFileSync(HERE+"new/"+item.from,"utf8");if(existsSync(item.to)&&readFileSync(item.to,"utf8")!==text){console.error("exists and differs",item.to);process.exit(4);}created.push({file:item.to,text});}
if(dry){for(const row of plan)console.log("dry",row.before,"->",row.after,basename(row.file));for(const row of created)console.log("dry new",basename(row.file));process.exit(0);}
for(const row of plan){const key=join(BACKUP,sha(row.file)+".orig");if(!existsSync(key))writeFileSync(key,readFileSync(row.file));writeFileSync(join(BACKUP,sha(row.file)+".path"),row.file);}
writeFileSync(join(BACKUP,"created.json"),JSON.stringify(created.filter(row=>!existsSync(row.file)).map(row=>row.file)));
for(const row of plan){writeFileSync(row.file,row.text);console.log("landed",row.before,"->",row.after,basename(row.file));}
for(const row of created){if(!existsSync(row.file)){mkdirSync(dirname(row.file),{recursive:true});writeFileSync(row.file,row.text);console.log("created",basename(row.file));}}
