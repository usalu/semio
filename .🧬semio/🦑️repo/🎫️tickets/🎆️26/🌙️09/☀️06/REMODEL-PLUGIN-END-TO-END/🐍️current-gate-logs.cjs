const fs=require("node:fs"),path=require("node:path");
const directory=path.resolve(__dirname,"🗑️generated");
const compact=process.argv.includes("--compact"),names=process.argv.slice(2).filter(value=>value!=="--compact");
if(names.length>8||names.some(name=>path.basename(name)!==name))throw Error("At most eight local generated log names required");
for(const name of names){
 const file=path.resolve(directory,name);if(path.dirname(file)!==directory||file.length>256)throw Error("Ticket output path required");
 if(!fs.existsSync(file)){console.log(JSON.stringify({name,missing:true}));continue;}
 const stat=fs.statSync(file),fd=fs.openSync(file,"r"),head=Buffer.alloc(2);let bytes;
 try{fs.readSync(fd,head,0,2,0);let start=Math.max(0,stat.size-262144);if(head[0]===255&&head[1]===254)start-=start%2;bytes=Buffer.alloc(stat.size-start);fs.readSync(fd,bytes,0,bytes.length,start);}finally{fs.closeSync(fd);}
 const text=bytes.toString(head[0]===255&&head[1]===254?"utf16le":"utf8").replace(/\u001b\[[0-?]*[ -/]*[@-~]/g,"");
 const signals=text.split(/\r?\n/).filter(line=>/\berror(?:\[[^\]]+\])?:\s|FAILED|test result:|^\s*(Checking|Compiling|Finished)\b|deadline|ENAMETOOLONG|E0753|source-admission|current again|Restart/i.test(line));
 let latestSource;
 for(const line of text.split(/\r?\n/)){if(!line.startsWith("[script] {"))continue;try{const item=JSON.parse(line.slice(9)),resource=item.activity?.resource,prefix="cargo:source-admission:";if(typeof resource==="string"&&resource.startsWith(prefix))latestSource=JSON.parse(resource.slice(prefix.length));}catch{}}
 const compactSignals=signals.filter(line=>!line.includes("source-admission")||/error|failed|refused/i.test(line)).slice(-3);
 console.log(JSON.stringify({name,bytes:stat.size,modified:stat.mtime.toISOString(),latestSource,signals:compact?compactSignals:signals.slice(-8),tail:text.slice(compact?-700:-2400)}));
}
