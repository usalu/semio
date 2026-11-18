import{readFileSync,writeFileSync,mkdirSync}from"node:fs";import{resolve,relative}from"node:path";
const R="/Users/ueli/Documents/semio",S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R135/";
const PH=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",HOME=PH+"/🐢️speed",LAB=S+"speedlab";
const outside=(text,realDir)=>text.replace(/((?:from|import)\s*\(?\s*)"(\.{1,2}\/[^"]*)"/gu,(all,head,rel)=>{const target=resolve(realDir,rel);return relative(HOME,target).startsWith("..")?head+'"'+target+'"':all;});
const put=(from,to,realDir)=>{mkdirSync(resolve(LAB,to,".."),{recursive:true});writeFileSync(LAB+"/"+to,outside(readFileSync(S+"stage/speed/"+from,"utf8"),realDir));};
put("speed.mjs","🟨️.mjs",HOME);put("speed.d.mts","🟨️.d.mts",HOME);put("schema.json","🧬️schema/🔣️.json",HOME);
put("speedtest.ts","🧪️tests/🟦️.ts",HOME+"/🧪️tests");put("fixture.json","🧪️tests/🔣️.json",HOME+"/🧪️tests");put("oracle.py","🧪️tests/🐍️.py",HOME+"/🧪️tests");

import{readFileSync as rd,writeFileSync as wr}from"node:fs";
const lab=LAB+"/🟨️.mjs";wr(lab,rd(lab,"utf8").replace('"./🧬️schema/🔣️.json"','"'+LAB+'/🧬️schema/🔣️.json"'));
