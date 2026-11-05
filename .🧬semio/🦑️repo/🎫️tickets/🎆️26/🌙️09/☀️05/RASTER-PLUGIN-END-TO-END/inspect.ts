import { readFileSync, readdirSync, statSync } from "node:fs";
import { resolve, join } from "node:path";

const args=process.argv.slice(2);
if(args[0]==="--syntax") {
  for(const name of args.slice(1)){
    try{new Bun.Transpiler({loader:name.endsWith("tsx")?"tsx":"ts"}).scan(readFileSync(name,"utf8"));console.log(JSON.stringify({path:name,syntax:"valid"}));}
    catch(error){process.exitCode=1;console.error(error);}
  }
} else if(args[0]==="--json") {
  for(const name of args.slice(1)){
    const path=resolve(name),value=JSON.parse(readFileSync(path,"utf8"));
    console.log(JSON.stringify({path,characters:[...path].length,keys:Object.keys(value)}));
  }
} else if(args[0]==="--find") {
  const pattern=new RegExp(args[2]!);
  const visit=(path:string)=>{
    for(const entry of readdirSync(path,{withFileTypes:true})) {
      if(["node_modules","target",".git",".nx"].includes(entry.name))continue;
      const file=join(path,entry.name);
      if(entry.isDirectory())visit(file);
      else if(/\.(rs|ts|tsx|json|graphql|proto)$/.test(file)&&pattern.test(readFileSync(file,"utf8")))console.log(file);
    }
  };
  visit(args[1]!);
} else if(args[0]==="--match") {
  const lines=readFileSync(args[1]!,"utf8").split("\n"),pattern=new RegExp(args[2]!),radius=Number(args[3]??5),shown=new Set<number>();
  for(let i=0;i<lines.length;i++)if(pattern.test(lines[i]!))for(let j=Math.max(0,i-radius);j<=Math.min(lines.length-1,i+radius);j++)if(!shown.has(j)){console.log(`${j+1}: ${lines[j]}`);shown.add(j);}
} else for (const name of args) {
  const path = resolve(name);
  console.log(path);
  console.log(statSync(path).isDirectory() ? readdirSync(path).join("\n") : readFileSync(path, "utf8"));
}
