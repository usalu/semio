import {mkdirSync,writeFileSync} from "node:fs";import {createHash,randomBytes} from "node:crypto";import {join} from "node:path";
const base=process.argv[2]!,out:string[]=[];
for(const mb of [1,3,3,6,12,12,30,30]){const b=randomBytes(mb*1048576),h=createHash("sha256").update(b).digest("hex"),d=join(base,"cas","v1","o",h.slice(0,2));mkdirSync(d,{recursive:true});writeFileSync(join(d,h.slice(2)),b);out.push(h);}
writeFileSync(join(base,"big.json"),JSON.stringify(out));
