import ts from "/home/deck/semio/node_modules/typescript/lib/typescript.js";
import {readFileSync,readdirSync} from "node:fs";
const [orig,dir]=process.argv.slice(2);
const stm=(file,text)=>{const sf=ts.createSourceFile(file,text,ts.ScriptTarget.Latest,true);return sf.statements.filter(s=>!ts.isImportDeclaration(s)).map(s=>text.slice(s.getStart(sf),s.end).replace(/^export /,"").replace(/new URL\("\.\.\/(\.\.?\/)/g,'new URL("$1').replace(/\s+/g," "));};
const a=stm("o.ts",readFileSync(orig,"utf8"));
const b=readdirSync(dir,{withFileTypes:true}).filter(d=>d.isDirectory()&&!["🚪️admission","🧾️collect-errors","🧮️derived-nodes","🧭️file-owner","🗂️generation-files","🕰️generations","📥️inference","📋️model","⏱️phases","🧾️registry-consistency","🔁retry","🦀️rust","🧬️schema","🎯️test-owners","🧪️tests"].includes(d.name)).flatMap(d=>stm(d.name,readFileSync(`${dir}/${d.name}/🟦️.ts`,"utf8")));
const set=new Map();for(const s of b)set.set(s,(set.get(s)??0)+1);
let missing=0;for(const s of a){if(set.get(s))set.set(s,set.get(s)-1);else{missing++;console.log("DIFF:",s.slice(0,160));}}
const extra=[...set].filter(([,n])=>n>0);
console.log("orig",a.length,"new",b.length,"changed",missing);for(const [s] of extra)console.log("EXTRA:",s.slice(0,200));
