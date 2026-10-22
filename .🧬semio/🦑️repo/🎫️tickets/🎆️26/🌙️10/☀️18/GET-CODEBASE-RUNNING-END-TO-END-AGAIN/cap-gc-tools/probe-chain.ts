import {readFileSync,existsSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
const root=process.cwd(),entry=process.argv[2]!,target=process.argv[3]!,parent=new Map<string,string>(),pending=[entry];
while(pending.length){const path=pending.shift()!;if(!/\.[cm]?[jt]s$/u.test(path)||!existsSync(join(root,path)))continue;const text=readFileSync(join(root,path),"utf8"),scanner=new Bun.Transpiler({loader:/\.[cm]?ts$/u.test(path)?"ts":"js",target:"node"});let rows;try{rows=/\.c(?:js|ts)$/u.test(path)?scanner.scanImports(text):scanner.scan(text).imports;}catch{continue;}
 for(const row of rows){if(!row.path.startsWith("."))continue;const child=relative(root,resolve(dirname(join(root,path)),row.path)).replaceAll("\\","/");if(parent.has(child)||child===entry)continue;parent.set(child,path);if(child.includes(target)){const chain=[child];let at=child;while(parent.has(at)){at=parent.get(at)!;chain.push(at);}console.log(chain.reverse().map(value=>value.split("/").slice(-4).join("/")).join("\n  -> "));process.exit(0);}pending.push(child);}}
console.log("no chain");
