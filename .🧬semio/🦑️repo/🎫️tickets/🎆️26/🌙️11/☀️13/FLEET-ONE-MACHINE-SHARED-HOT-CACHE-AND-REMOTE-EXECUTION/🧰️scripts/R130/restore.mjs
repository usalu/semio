import {readFileSync,writeFileSync,readdirSync,rmSync,existsSync} from "node:fs";
import {join,dirname} from "node:path";
const BACKUP="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R130/backup";
for(const name of readdirSync(BACKUP).filter(n=>n.endsWith(".path"))){const file=readFileSync(join(BACKUP,name),"utf8");writeFileSync(file,readFileSync(join(BACKUP,name.replace(".path",".orig"))));console.log("restored",file);}
if(existsSync(join(BACKUP,"created.json")))for(const file of JSON.parse(readFileSync(join(BACKUP,"created.json"),"utf8"))){rmSync(file,{force:true});try{const dir=dirname(file);if(readdirSync(dir).length===0)rmSync(dir,{recursive:false});}catch{}console.log("removed",file);}
