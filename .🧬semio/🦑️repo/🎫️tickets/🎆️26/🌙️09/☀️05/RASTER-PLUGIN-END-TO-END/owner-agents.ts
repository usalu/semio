import{existsSync,readFileSync}from"node:fs";import{dirname,resolve,join}from"node:path";
let p=resolve(process.argv[2]??".");for(;;){const f=join(p,"AGENTS.md");if(existsSync(f))console.log(f+"\n"+readFileSync(f,"utf8"));const parent=dirname(p);if(parent===p)break;p=parent;}
