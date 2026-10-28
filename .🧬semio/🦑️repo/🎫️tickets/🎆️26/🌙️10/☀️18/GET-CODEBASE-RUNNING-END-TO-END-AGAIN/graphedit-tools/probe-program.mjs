/** 🪁 Read-only oracle: the files of the course's declared TypeScript programs (plain `typescript`), compared with the owner's raw `file` rows in the live index. */
import{readFileSync}from"node:fs";
import{join,dirname}from"node:path";
import{createRequire}from"node:module";
import{brotliDecompressSync}from"node:zlib";
const [root,owner,...configs]=process.argv.slice(2);
const ts=createRequire(join(root,"package.json"))("typescript");
const files=new Set();
for(const rel of configs){const config=join(root,rel),source=ts.readJsonConfigFile(config,ts.sys.readFile),parsed=ts.parseJsonSourceFileConfigFileContent(source,ts.sys,dirname(config),undefined,config);
 const program=ts.createProgram({rootNames:parsed.fileNames,options:{...parsed.options,noEmit:true,incremental:false,composite:false},projectReferences:parsed.projectReferences});
 for(const f of program.getSourceFiles())if(f.fileName.startsWith(root+"/")&&!f.fileName.includes("/node_modules/")&&!f.fileName.slice(root.length+1).startsWith(".cache/"))files.add(f.fileName);}
const codec=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs"));
const store=join(root,".cache/b/g/m/generations"),cur=JSON.parse(readFileSync(join(store,"current.json"),"utf8")),man=JSON.parse(readFileSync(join(store,"m",cur.manifest+".json"),"utf8"));
const idx=codec.openFactIndexV1(brotliDecompressSync(readFileSync(join(store,"x",man.factIndex+".facts.br")),{maxOutputLength:536870912}),{maximumBytes:536870912});
const o=idx.owners.indexOf(owner),ids=new Set();idx.classes.forEach(c=>{if(c.o.includes(o))for(const r of c.r)ids.add(r);});
const rows=new Set();for(const i of ids)if(idx.rowKindOf(i)==="file")rows.add(idx.rows[i].path);
const inBoth=[...files].filter(f=>rows.has(f)).length,programOnly=[...files].filter(f=>!rows.has(f)),rowsOnly=[...rows].filter(f=>!files.has(f));
console.log(JSON.stringify({generation:cur.generation,programFiles:files.size,fileRows:rows.size,inBoth,programOnly:programOnly.length,rowsOnly:rowsOnly.length}));
console.log("program-only sample",programOnly.slice(0,4).map(p=>p.slice(root.length+1)));
console.log("rows-only sample",rowsOnly.slice(0,6).map(p=>p.slice(root.length+1)));
