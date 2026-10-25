import {readFileSync,writeFileSync} from "node:fs";
import {selectPackageManagerV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";
const manager=selectPackageManagerV1("javascript"),[ext,ws,hand,out]=process.argv.slice(2) as [string,string,string,string];
const cases:any[]=[];
for(const [file,prefix,network] of [[ext,"external packages: ",true],[ws,"workspace members only: ",false]] as const)for(const row of JSON.parse(readFileSync(file,"utf8")) as any[]){
  const members:string[]=row.manifests["."].workspaces,manifests:[string,unknown][]=[["package.json",row.manifests["."]],...members.map(member=>[member+"/package.json",row.manifests[member]] as [string,unknown])];
  cases.push({name:prefix+row.case,lock:row.lock,manifests,expected:manager.checkLock(row.lock,new Map(manifests)),bun:{refused:row.frozenReal===1,network}});
}
cases.push(...JSON.parse(readFileSync(hand,"utf8")));
writeFileSync(out,JSON.stringify({version:1,bun:{version:"1.4.2"},cases},null,1)+"\n");
console.log(cases.length,"cases");
