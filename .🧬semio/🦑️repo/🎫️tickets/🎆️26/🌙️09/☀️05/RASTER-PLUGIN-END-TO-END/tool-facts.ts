import {existsSync,openSync,readSync,closeSync,realpathSync,statSync} from "node:fs";
import {findToolV1,resolveRegisteredToolV1} from "@semio-tech/process/tools";
for(const tool of ["node","bun"]){
  const entry=findToolV1(tool);if(!entry)continue;
  const path=resolveRegisteredToolV1(entry,process.cwd(),{env:process.env}).command;
  if(!path||!existsSync(path))continue;
  const handle=openSync(path,"r"),header=Buffer.alloc(1024);let count=0;
  try{count=readSync(handle,header,0,header.length,0);}finally{closeSync(handle);}
  console.log(JSON.stringify({tool,path,real:realpathSync(path),bytes:statSync(path).size,header:header[0]===35?header.subarray(0,count).toString("utf8"):header.subarray(0,32).toString("hex")}));
}
