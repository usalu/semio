// [gram-g] corpus oracle for nativeDependencyRoots: the previous implementation (git HEAD blob, written next to the module for the run) and the current one answer every cargo root of the tracked tree (tests false/true) with byte-identical JSON. usage: bun corpus-native.mjs [revision=HEAD]
import{execFileSync}from"node:child_process";import{writeFileSync,rmSync,existsSync,mkdirSync}from"node:fs";import{join,resolve}from"node:path";
const ROOT="/Users/ueli/Documents/semio",L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/",rev=process.argv[2]??"HEAD";
const rel=L+"🦀️native-manifests/🟨️.mjs",oldRel=L+"🦀️native-manifests/🟨️.gram-old.mjs";
const inference=await import(ROOT+"/"+L+"🟨️.mjs");await inference.libraryBootstrap;
const current=await import(ROOT+"/"+rel);
const oldPath=join(ROOT,process.env.C_DIR,"gram-g-tools/oracle/native-manifests-old.mjs"),moduleDir=join(ROOT,rel,"..");
mkdirSync(join(oldPath,".."),{recursive:true});
writeFileSync(oldPath,execFileSync("git",["show",rev+":"+rel],{cwd:ROOT,maxBuffer:1<<28}).toString().replace(/(from\s*|import\()\s*(["'])(\.[^"']*)\2/g,(m,a,q,spec)=>a+JSON.stringify(resolve(moduleDir,spec))));
try{
 const old=await import(oldPath);
 const coordinates=await import(ROOT+"/"+L+"📍️coordinates/🟨️.mjs"),sourceAnalyses=await import(ROOT+"/"+L+"🧬️source-analyses/🟨️.mjs"),targetPolicy=await import(ROOT+"/"+L+"🎯️target-policy/🟨️.mjs");
 old.wire({sourceAnalyses,coordinates,targetPolicy});
 const roots=execFileSync("git",["ls-files","-z","--","Cargo.toml","*/Cargo.toml"],{cwd:ROOT,maxBuffer:1<<28}).toString().split("\0").filter(f=>f&&!f.startsWith(".🧬semio")).map(f=>f.includes("/")?f.slice(0,f.lastIndexOf("/")):".");
 const stride=Number(process.env.STRIDE??"1");const sampled=roots.filter((_,i)=>i%stride===0);roots.length=0;roots.push(...sampled);
 const answer=(fn,...args)=>{try{return JSON.stringify(fn(...args));}catch(error){return JSON.stringify({error:error.message,code:error.code??null});}};
 const cacheNew=new Map(),closuresNew=new Map(),cacheOld=new Map();let compared=0,different=[],errors=0,members=0;
 const t0=performance.now();
 for(const tests of [false,true])for(const root of roots){
  const a=answer(current.nativeDependencyRoots,root,ROOT,tests,cacheNew,closuresNew),b=answer(old.nativeDependencyRoots,root,ROOT,tests,cacheOld);
  compared++;if(a!==b)different.push(root);if(a.startsWith('{"error"'))errors++;else members+=JSON.parse(a).length;
 }
 console.log(JSON.stringify({revision:rev,roots:roots.length,compared,different:different.slice(0,5),differentCount:different.length,errors,members,seconds:Math.round((performance.now()-t0)/100)/10}));
 process.exit(different.length?1:0);
}finally{}
