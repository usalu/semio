import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";
const edits = JSON.parse(readFileSync(join(import.meta.dir,"shared-edits.json"),"utf8"));
const git = (args: string[], input?: string) => { const p=spawnSync("git",args,{encoding:"utf8",input,maxBuffer:16*1024*1024,windowsHide:true});if(p.status)throw new Error(p.stderr);return p.stdout; };
for (const p of [...Object.keys(edits), "🎓️teaching/README.md"]) {
 let staged=git(["show",":"+p]);
 if(edits[p]) for (const [before,after,...alternatives] of edits[p]) { if(staged.includes(after))continue; const match=[before,...alternatives].find(value=>staged.includes(value));if(!match)throw new Error("Index hunk differs: "+p);staged=staged.replace(match,after); }
 else { const marker="\n\n## Complete teaching videos\n";const current=readFileSync(p,"utf8");const start=current.indexOf(marker);if(start<0)throw new Error("Missing owned README section");const nextHeading=current.indexOf("\n## ",start+marker.length);const section=current.slice(start,nextHeading<0?undefined:nextHeading);const stagedStart=staged.indexOf(marker);if(stagedStart<0)staged=staged.trimEnd()+section;else { const stagedEnd=staged.indexOf("\n## ",stagedStart+marker.length);staged=staged.slice(0,stagedStart)+section+(stagedEnd<0?"":staged.slice(stagedEnd)); } }
 if(p==="🎓️teaching/README.md")staged=staged.replace("short explanatory videos of the same topic (not built yet).","explanatory video sources of the same topic.");
 const mode=git(["ls-files","--stage","--",p]).split(" ")[0];
 const hash=git(["hash-object","-w","--stdin"],staged).trim();
 git(["update-index","--add","--cacheinfo",mode+","+hash+","+p]);
 console.log("Staged task hunks only: "+p);
}
