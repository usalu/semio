import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
function scan(root:string):void{for(const row of readdirSync(root,{withFileTypes:true})){const path=join(root,row.name);if(row.isDirectory()&&!row.name.startsWith(".")&&row.name!=="node_modules"&&!row.name.includes("tests"))scan(path);else if(row.isFile()&&/\.(ts|mjs|go|json)$/.test(path)){const lines=readFileSync(path,"utf8").split("\n");for(let index=0;index<lines.length;index++)if(/SEMIO_[A-Z_]*(DIRECTORY|ROOT|HOME)|XDG_CACHE_HOME/.test(lines[index]!))console.log(path+":"+(index+1)+":"+[...new Set(lines[index]!.match(/SEMIO_[A-Z_]*(DIRECTORY|ROOT|HOME)|XDG_CACHE_HOME/g))].join(","));}}}
for(const root of process.argv.slice(2))scan(root);
