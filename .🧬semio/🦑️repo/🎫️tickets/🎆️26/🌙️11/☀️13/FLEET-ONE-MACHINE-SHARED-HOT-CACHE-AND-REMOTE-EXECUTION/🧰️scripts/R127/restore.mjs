import {readFileSync,writeFileSync,readdirSync} from "node:fs";
import {join} from "node:path";
const BACKUP="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R127/backup";
for(const name of readdirSync(BACKUP).filter(n=>n.endsWith(".path"))){const file=readFileSync(join(BACKUP,name),"utf8");writeFileSync(file,readFileSync(join(BACKUP,name.replace(".path",".orig"))));console.log("restored",file);}
