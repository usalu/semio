import {readFileSync,readdirSync} from "node:fs";
const root=process.argv[2],pattern=new RegExp(process.argv[3]);
function visit(path:string){for(const item of readdirSync(path,{withFileTypes:true})){if(item.name.startsWith(".")||item.name.includes("🧪")||["node_modules","target"].includes(item.name))continue;const file=path+"/"+item.name;if(item.isDirectory())visit(file);else if(/\.(rs|ts|tsx|mjs|cjs|json|toml)$/.test(file)){const lines=readFileSync(file,"utf8").split("\n");for(let index=0;index<lines.length;index++)if(pattern.test(lines[index]))console.log(file+":"+(index+1)+":"+lines[index]);}}}
visit(root);
