import {readFileSync,writeFileSync,rmSync,existsSync} from "node:fs";
const SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R133";
const manifest=JSON.parse(readFileSync(SC+"/backup/manifest.json","utf8"));
for(const row of manifest){if(row.backup===null){rmSync(row.file,{force:true});continue;}writeFileSync(row.file,readFileSync(row.backup,"utf8"));}
console.log("restored",manifest.length);
