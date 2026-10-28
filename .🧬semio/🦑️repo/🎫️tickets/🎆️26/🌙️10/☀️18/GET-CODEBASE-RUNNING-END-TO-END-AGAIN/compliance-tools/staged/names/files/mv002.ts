import{openSync}from"node:fs";
import{limitOpenFilesV1}from"../../👶️child/🟦️.ts";
const limit=limitOpenFilesV1(Number(process.argv[2]));
let opened=0;
try{for(;;){openSync("/dev/null","r");opened++;}}catch(error){process.stdout.write(JSON.stringify({limit,opened,code:(error as NodeJS.ErrnoException).code})+"\n");}
