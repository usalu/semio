import {existsSync,mkdirSync,copyFileSync,constants,lstatSync,symlinkSync,readlinkSync} from "node:fs";
import {dirname,join} from "node:path";
import {incrementalTrackedFilesV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/⌨️command/🟦️.ts";
const [live,copy]=process.argv.slice(2) as [string,string],files=await incrementalTrackedFilesV1(live,new AbortController().signal);
let added=0;
for(const file of files){const to=join(copy,file);try{lstatSync(to);continue;}catch{}let meta;try{meta=lstatSync(join(live,file));}catch{continue;}mkdirSync(dirname(to),{recursive:true});if(meta.isSymbolicLink()){const target=readlinkSync(join(live,file));if(target.startsWith(live))continue;symlinkSync(target,to);}else if(meta.isFile())copyFileSync(join(live,file),to,constants.COPYFILE_FICLONE);added++;}
console.log(JSON.stringify({listed:files.length,added}));
