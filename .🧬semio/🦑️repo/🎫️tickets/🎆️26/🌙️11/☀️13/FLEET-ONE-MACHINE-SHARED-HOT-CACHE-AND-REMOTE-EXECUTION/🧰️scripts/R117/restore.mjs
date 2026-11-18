import{copyFileSync,existsSync,rmSync,readFileSync}from"node:fs";
const R="/Users/ueli/Documents/semio/",SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R117/",BK=SC+"backup/",step=process.argv[2]??"step1";
const manifest=JSON.parse(readFileSync(import.meta.dir+"/files.json","utf8"));
for(const[k]of manifest[step])if(existsSync(BK+k)){copyFileSync(BK+k,R+k);console.log("restored",k.slice(-30));}
for(const[k]of manifest["tests"+step.slice(4)]??[]){const dir=R+k.replace(/\/🟦️\.ts$/,"");if(dir.startsWith(R+"🧰️framework/")&&existsSync(dir)){rmSync(dir,{recursive:true,force:true});console.log("removed",dir.slice(-30));}}
