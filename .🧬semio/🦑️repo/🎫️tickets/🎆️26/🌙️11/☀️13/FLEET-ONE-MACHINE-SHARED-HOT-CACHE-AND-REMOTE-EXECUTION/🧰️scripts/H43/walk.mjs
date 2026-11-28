import {lstatSync,readdirSync} from "node:fs";import {join} from "node:path";
const t=performance.now(),pending=[process.argv[2]];let n=0,b=0;
while(pending.length){const p=pending.pop(),r=lstatSync(p);n++;if(r.isDirectory()){for(const x of readdirSync(p))pending.push(join(p,x));continue;}b+=r.size;}
console.log(JSON.stringify({entries:n,bytes:b,seconds:Math.round((performance.now()-t)/100)/10}));
