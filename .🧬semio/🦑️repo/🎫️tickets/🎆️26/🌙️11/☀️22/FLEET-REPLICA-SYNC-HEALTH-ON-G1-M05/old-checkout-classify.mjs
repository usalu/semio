import {readFileSync,existsSync,statSync,mkdirSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
const [old,live,out]=process.argv.slice(2),sha=bytes=>createHash("sha256").update(bytes).digest("hex");
const git=(...args)=>spawnSync("git",["--no-optional-locks","-c","core.quotepath=false",...args],{cwd:old,maxBuffer:1<<30});
const entries=git("status","--porcelain=v1","-z","--untracked-files=all","--no-renames").stdout.toString("utf8").split("\0").filter(Boolean).map(line=>({code:line.slice(0,2),path:line.slice(3)}));
mkdirSync(out,{recursive:true});
const rows=[];
for(const {code,path} of entries){
 const oldFile=join(old,path),liveFile=join(live,path),oldBytes=existsSync(oldFile)&&statSync(oldFile).isFile()?readFileSync(oldFile):undefined,liveBytes=existsSync(liveFile)&&statSync(liveFile).isFile()?readFileSync(liveFile):undefined;
 const base=code==="??"?undefined:git("show",`HEAD:${path}`),baseBytes=base!==undefined&&base.status===0?base.stdout:undefined;
 let verdict;
 if(oldBytes===undefined)verdict=liveBytes===undefined?"deleted-both":"deleted-old-only";
 else if(liveBytes===undefined)verdict="absent-live";
 else if(sha(oldBytes)===sha(liveBytes))verdict="identical";
 else if(baseBytes!==undefined&&sha(baseBytes)===sha(liveBytes))verdict="live-at-base";
 else if(baseBytes!==undefined&&sha(baseBytes)===sha(oldBytes))verdict="old-at-base";
 else if(baseBytes===undefined)verdict="diverged-no-base";
 else{
  const slot=join(out,sha(Buffer.from(path)).slice(0,12));mkdirSync(slot,{recursive:true});
  writeFileSync(join(slot,"old"),oldBytes);writeFileSync(join(slot,"base"),baseBytes);writeFileSync(join(slot,"live"),liveBytes);writeFileSync(join(slot,"path.txt"),path);
  const merged=spawnSync("git",["merge-file","-p","--diff3","-L","live","-L","base","-L","old",join(slot,"live"),join(slot,"base"),join(slot,"old")],{maxBuffer:1<<28});
  writeFileSync(join(slot,"merged"),merged.stdout);
  verdict=merged.status===0?(sha(merged.stdout)===sha(liveBytes)?"merged-equals-live":`clean-merge ${slot}`):`conflict(${merged.status}) ${slot}`;
 }
 rows.push(`${verdict}\t${code}\t${oldBytes===undefined?"-":new Date(statSync(oldFile).mtimeMs).toISOString().slice(0,16)}\t${path}`);
}
writeFileSync(join(out,"classes.tsv"),rows.join("\n")+"\n");
const counts={};for(const row of rows){const key=row.split("\t")[0].split(" ")[0];counts[key]=(counts[key]??0)+1;}
console.log(entries.length,counts);
