/** 🧾️ [nxdeadline] pretty-prints dense TS files for reading: node pretty.mjs <out-dir> <file...> */
import ts from "/Users/ueli/Documents/semio/node_modules/typescript/lib/typescript.js";
import {readFileSync,writeFileSync} from "node:fs";
import {basename,dirname,join} from "node:path";
const [out,...files]=process.argv.slice(2);
for(const f of files){const src=ts.createSourceFile(f,readFileSync(f,"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.TS);const text=ts.createPrinter({removeComments:false}).printFile(src);const name=basename(dirname(f))+".ts";writeFileSync(join(out,name),text);console.log(name,text.split("\n").length);}
