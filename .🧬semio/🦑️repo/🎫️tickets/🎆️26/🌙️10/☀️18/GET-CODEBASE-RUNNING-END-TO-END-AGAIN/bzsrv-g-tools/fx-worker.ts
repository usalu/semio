import {writeFileSync,mkdirSync} from "node:fs";
import {dirname,join} from "node:path";
import {installWorkerOutputCaptureV1,readWorkerInputV1,serveWorkerV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👷️workers/📨️protocol/🟦️.ts";
if(process.argv.includes("--persistent_worker")){
 const capture=installWorkerOutputCaptureV1();
 await serveWorkerV1({input:readWorkerInputV1(),write:capture.writeProtocol,handler:async request=>{
  const out=request.arguments.find(value=>value.endsWith(".out"))??"";
  const target=process.env.FX_EXECROOT?join(process.env.FX_EXECROOT,out):out;
  mkdirSync(dirname(target),{recursive:true});writeFileSync(target,"worker pid "+process.pid+"\n");
  return {exitCode:0,output:""};
 }});
 process.exit(0);
}
