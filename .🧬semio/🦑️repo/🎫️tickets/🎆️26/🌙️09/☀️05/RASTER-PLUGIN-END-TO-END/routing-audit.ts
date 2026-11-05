import{readFileSync,readdirSync}from"node:fs";
import{join}from"node:path";
const pattern=new RegExp(process.argv[3]??"ActionMeta\\s*\\{|view_state:\\s*Some\\(");
function visit(root:string){for(const row of readdirSync(root,{withFileTypes:true})){if(["target","node_modules",".git",".🧬semio"].includes(row.name))continue;const file=join(root,row.name);if(process.argv.includes("--live")&&/🧪️|tests|testing|fixture/.test(file))continue;if(row.isDirectory())visit(file);else if(file.endsWith(".rs")){for(const[line,text]of readFileSync(file,"utf8").split("\n").entries())if(pattern.test(text))console.log(file+":"+(line+1)+":"+text.trim().slice(0,1400));}}}
visit(process.argv[2]!);
