import {appendFileSync,fstatSync} from "node:fs";
const rows:string[]=[];
for(let fd=0;fd<12;fd++){try{const s=fstatSync(fd);rows.push(fd+":"+(s.isFIFO()?"fifo":s.isSocket()?"sock":s.isCharacterDevice()?"chr":"other"));}catch{}}
if(process.env.FDS_LOG)appendFileSync(process.env.FDS_LOG,"[DEBUG] first "+process.argv.slice(2,4).join(" ").slice(-40)+" "+rows.join(" ")+"\n");
