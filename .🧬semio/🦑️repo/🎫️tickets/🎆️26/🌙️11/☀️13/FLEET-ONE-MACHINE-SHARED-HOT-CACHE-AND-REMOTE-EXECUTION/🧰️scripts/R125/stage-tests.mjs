import{readFileSync,writeFileSync,mkdirSync}from"node:fs";import{resolve,dirname}from"node:path";
const R="/Users/ueli/Documents/semio",S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R125/";
const PH=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",FARM=PH+"/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm";
const abs=(text,dir)=>text.replace(/((?:from|import)\s*\(?\s*)"(\.{1,2}\/[^"]*)"/gu,(all,head,rel)=>head+'"'+resolve(dir,rel)+'"');
for(const[name,stage,dir]of[["farm-width","farm-width.test.ts",PH+"/🌡️host-capacity/🧪️tests/🧮️farm-width"],["measured-ask","measured-ask.test.ts",FARM+"/🧪️tests/🧮️measured-ask"]]){mkdirSync(S+"tests",{recursive:true});writeFileSync(S+"tests/"+name+".ts",abs(readFileSync(S+"stage/"+stage,"utf8"),dir));}
