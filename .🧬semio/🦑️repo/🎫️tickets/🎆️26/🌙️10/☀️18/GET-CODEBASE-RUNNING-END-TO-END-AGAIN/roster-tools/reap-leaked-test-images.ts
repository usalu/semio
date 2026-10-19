import{nativeImageInformationV1}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📥️information/🟦️.ts";
import{spawnSync}from"node:child_process";
import{statSync}from"node:fs";
const apply=process.argv.includes("--apply"),minimumAgeSeconds=1800,current=process.env.SEMIO_CURRENT_SESSION??"";
const run=(program:string,args:string[])=>spawnSync(program,args,{encoding:"utf8"});
const information=nativeImageInformationV1(run("/usr/bin/hdiutil",["info","-plist"]).stdout);
const pattern=/^\/private\/tmp\/claude-[0-9]+\/[^/]+\/([0-9a-f-]{36})\/scratchpad\/.+\/i\/m$/u;
const processes=run("/bin/ps",["-axo","command"]).stdout;
for(const image of information.images as Record<string,unknown>[]){
 for(const entity of image["system-entities"] as Record<string,unknown>[]){
  const mount=entity["mount-point"];if(typeof mount!=="string")continue;
  const match=pattern.exec(mount);if(!match)continue;
  const backing=image["image-path"] as string,session=match[1]!;
  let ageSeconds=Infinity;try{ageSeconds=(Date.now()-statSync(backing).mtimeMs)/1000;}catch{}
  const open=run("/usr/sbin/lsof",["+f","--",mount]).stdout.trim().split("\n").filter(Boolean).length;
  const referenced=processes.includes(mount.slice(0,-2));
  const leaked=session!==current&&open===0&&!referenced&&ageSeconds>=minimumAgeSeconds;
  console.log(`${leaked?"LEAKED":"KEEP  "} session=${session.slice(0,8)} age=${Math.round(ageSeconds)}s open=${open} referenced=${referenced} ${mount}`);
  if(leaked&&apply){const result=run("/usr/bin/hdiutil",["detach",entity["dev-entry"] as string]);console.log(`  detach ${result.status===0?"ok":"FAILED "+result.stderr.trim()}`);}
 }
}
