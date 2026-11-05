import {existsSync,readFileSync} from "node:fs";
import {resolve,dirname,join} from "node:path";
const seen=new Set<string>();
for(const target of process.argv.slice(2)) {
  let directory=resolve(target);
  while(directory.startsWith(process.cwd())) {
    const file=join(directory,"AGENTS.md");
    if(!seen.has(file)&&existsSync(file)){seen.add(file);console.log(`${file}\n${readFileSync(file,"utf8")}`);}
    const parent=dirname(directory);if(parent===directory)break;directory=parent;
  }
}
