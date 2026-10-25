import{readFileSync}from"node:fs";
import{judgeProcessStartsV1,listProcessSpawnFilesV1,scanProcessStartsV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🚷️raw-process-spawn/🟦️.ts";

const path="🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📌️runtime-pin/⌨️command/🟦️.ts",sites=scanProcessStartsV1(path,readFileSync(path,"utf8"));
console.log(JSON.stringify(sites),JSON.stringify(judgeProcessStartsV1(sites).findings??"").slice(0,300));
console.log(listProcessSpawnFilesV1(process.cwd()).includes(path));
