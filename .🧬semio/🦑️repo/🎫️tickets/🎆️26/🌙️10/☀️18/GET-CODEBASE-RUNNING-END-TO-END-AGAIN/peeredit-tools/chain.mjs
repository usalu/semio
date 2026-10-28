/** 🔗 Read-only independent oracle (TypeScript `preProcessFile` + Bun's resolver, no library code): breadth-first import walk from entry scripts, printing the shortest import chain from an entry to each target file and the size of the reachable workspace set. Usage: bun chain.mjs <root> <entry> <target>... */
import{readFileSync,existsSync,statSync}from"node:fs";
import{dirname,join,relative}from"node:path";
import{createRequire}from"node:module";
const [root,entry,...targets]=process.argv.slice(2);
const ts=createRequire(join(root,"package.json"))("typescript");
const start=join(root,entry),parent=new Map([[start,null]]),queue=[start];
const code=/\.[cm]?[jt]sx?$/;
const designation=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🔣️.json"),providers=process.env.PORTS==="0"?{}:JSON.parse(readFileSync(designation,"utf8")).providers;
const portsOf=text=>{const names=new Set();for(const m of text.matchAll(/(?<!function\s)\bdefineScriptProviderV1\b(?!\s*[,}])/gu)){const lit=/^(?:\s*<[^()]*>)?\s*\(\s*(["'])([a-z0-9-]+)\1/u.exec(text.slice(m.index+m[0].length));names.add(lit?lit[2]:"*");}return[...names];};
while(queue.length){const file=queue.shift();if(!code.test(file))continue;let text;try{text=readFileSync(file,"utf8");}catch{continue;}
 const info=ts.preProcessFile(text,true,true);
 for(const port of portsOf(text))for(const[name,rel]of Object.entries(providers))if(port==="*"||port===name){const to=join(dirname(designation),rel);if(!parent.has(to)&&existsSync(to)){parent.set(to,file+" [port "+name+"]");queue.push(to);}}
 for(const ref of info.importedFiles){const spec=ref.fileName;let to;try{to=Bun.resolveSync(spec,dirname(file));}catch{continue;}
  if(!to.startsWith(root+"/")||to.includes("/node_modules/")||parent.has(to))continue;if(!existsSync(to)||!statSync(to).isFile())continue;parent.set(to,file);queue.push(to);}
 if(process.memoryUsage().rss>1.5*2**30){console.error("[DEBUG] rss abort");process.exit(3);}
}
const rel=p=>relative(root,p);
console.log(JSON.stringify({entry,reachable:parent.size}));
for(const t of targets){const abs=join(root,t);if(!parent.has(abs)){console.log("NOT REACHED",t);continue;}const chain=[];for(let at=abs;at;at=parent.get(at)?.split(" [port")[0]){const via=parent.get(at)?.includes(" [port")?parent.get(at).slice(parent.get(at).indexOf(" [port")):"";chain.unshift(rel(at)+via);}console.log("CHAIN to",t);for(const c of chain)console.log("   ",c);}
