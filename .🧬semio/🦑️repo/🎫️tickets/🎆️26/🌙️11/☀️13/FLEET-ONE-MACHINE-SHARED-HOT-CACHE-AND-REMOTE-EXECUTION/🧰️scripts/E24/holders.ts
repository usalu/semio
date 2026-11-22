import{observeDependencyStoreHoldersV1,dependencyStoreHolderPresentV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/💰️funding/🟦️.ts";
import{execSync}from"node:child_process";
console.log("ppid",process.ppid,"ps",execSync("ps -o etime=,ppid= -p "+process.ppid,{encoding:"utf8"}).trim());
try{process.kill(process.ppid,0);console.log("alive")}catch(e){console.log("kill err",(e as any).code)}
console.log(dependencyStoreHolderPresentV1(true,Date.now()-100000,Date.now()-50000,0));
