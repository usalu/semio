import {appendFileSync,fstatSync} from "node:fs";
import {spawnSync} from "node:child_process";
const rows:string[]=[];
for(let fd=0;fd<24;fd++){try{const s=fstatSync(fd);rows.push(fd+":"+(s.isFIFO()?"fifo":s.isSocket()?"sock":s.isCharacterDevice()?"chr":s.isFile()?"file":"other"));}catch{}}
appendFileSync(process.env.FDS_LOG!,process.argv.slice(2,4).join(" ")+" | "+rows.join(" ")+" | MAKEFLAGS="+(process.env.CARGO_MAKEFLAGS??"")+" | bun "+process.versions.bun+"\n");
process.exit(spawnSync(process.argv[2]!,process.argv.slice(3),{stdio:"inherit"}).status??1);
