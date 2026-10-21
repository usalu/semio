/** 🏗 Synthetic workspace generator: layered owners, source files, node_modules and output noise, plus the graph description the prototype ports read. */
import {mkdirSync,rmSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";

function rng(seed){let s=seed>>>0;return()=>{s=(s*1664525+1013904223)>>>0;return s/4294967296;};}

/** 🏗 Builds the workspace under `root`; returns the graph description. */
export function buildWorkspace({root,owners=1500,filesPerOwner=20,layers=8,depsPerOwner=2,nodeModulesFiles=12,outputNoise=10,seed=7}){
 rmSync(root,{recursive:true,force:true});mkdirSync(root,{recursive:true});
 const random=rng(seed),perLayer=Math.ceil(owners/layers),list=[];
 for(let i=0;i<owners;i++){
  const layer=Math.floor(i/perLayer),name="p"+String(i).padStart(4,"0"),deps=[];
  if(layer>0){const pool=Array.from({length:perLayer},(_,k)=>(layer-1)*perLayer+k).filter(k=>k<owners);const chosen=new Set();while(chosen.size<Math.min(depsPerOwner,pool.length))chosen.add(pool[Math.floor(random()*pool.length)]);for(const k of chosen)deps.push("p"+String(k).padStart(4,"0"));}
  list.push({name,root:"pkgs/"+name,deps,tasks:[{name:"build",command:["build",name],dependsOn:deps.map(d=>d+":build"),outputs:[".out/"+name]},{name:"test",command:["test",name],dependsOn:[name+":build"],outputs:[".out/"+name+"-test"]}]});
  const base=root+"/pkgs/"+name;
  mkdirSync(base+"/src",{recursive:true});
  for(let f=0;f<filesPerOwner;f++){
   const body=Array.from({length:30},(_,k)=>"export const v"+k+" = "+Math.floor(random()*1e9)+";").join("\n");
   writeFileSync(base+"/src/f"+f+".ts","// "+name+" file "+f+"\n"+body+"\n");
  }
  writeFileSync(base+"/package.json",JSON.stringify({name,dependencies:Object.fromEntries(deps.map(d=>[d,"workspace:*"]))}));
  if(i%10===0){mkdirSync(base+"/node_modules/x",{recursive:true});for(let f=0;f<nodeModulesFiles;f++)writeFileSync(base+"/node_modules/x/m"+f+".js","module.exports="+f+";");}
 }
 mkdirSync(root+"/.out",{recursive:true});
 for(let i=0;i<outputNoise;i++)writeFileSync(root+"/.out/noise"+i,"x");
 writeFileSync(root+"/package.json",'{"name":"ws"}');writeFileSync(root+"/tsconfig.base.json",'{"compilerOptions":{}}');writeFileSync(root+"/bun.lock","lock");
 mkdirSync(root+"/.git",{recursive:true});writeFileSync(root+"/.git/HEAD","ref: refs/heads/main\n");
 const graph={generation:1,owners:list,env:{paths:["package.json","tsconfig.base.json","bun.lock"],prefixes:[]}};
 writeFileSync(root+"/graph.json",JSON.stringify(graph));
 return graph;
}

/** 🔢 Digest of the environment files, the prototype's `environmentDigest`. */
export function environmentDigest(root,readFile){
 const hash=createHash("sha256");for(const path of["package.json","tsconfig.base.json","bun.lock"])hash.update(readFile(root+"/"+path));return hash.digest("hex");
}
