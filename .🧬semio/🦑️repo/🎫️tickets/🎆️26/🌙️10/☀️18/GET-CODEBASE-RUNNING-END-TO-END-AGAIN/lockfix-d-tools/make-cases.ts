import {readFileSync,writeFileSync,mkdirSync,rmSync,mkdtempSync} from "node:fs";
import {join,dirname} from "node:path";
import {tmpdir} from "node:os";
import {selectPackageManagerV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";
const manager=selectPackageManagerV1("javascript"),corpusPath=process.argv[2]!,genLock=readFileSync(process.argv[3]!,"utf8"),bunBin=process.env.BUN_BIN!;
const corpus=JSON.parse(readFileSync(corpusPath,"utf8"));
const base=(a:any,b:any):[string,unknown][]=>[["package.json",{name:"root",private:true,workspaces:["pkgs/a","pkgs/b"]}],["pkgs/a/package.json",{name:"a",version:"1.0.0",...a}],["pkgs/b/package.json",{name:"b",version:"1.0.0",...b}]];
const refuses=(lock:string,manifests:[string,unknown][]):boolean=>{
 const root=mkdtempSync(join(tmpdir(),"mk-"));
 try{for(const [path,doc] of manifests){const file=join(root,path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,JSON.stringify(doc));}
  writeFileSync(join(root,"bun.lock"),lock);
  return Bun.spawnSync([bunBin,"install","--frozen-lockfile","--dry-run","--ignore-scripts","--cache-dir",join(root,".cache")],{cwd:root,stdout:"ignore",stderr:"ignore"}).exitCode!==0;
 }finally{rmSync(root,{recursive:true,force:true});}
};
const specs:[string,any,any][]=[
 ["last-user baseline: locked package with a transitive user",{dependencies:{debug:"4.4.1",ms:"2.1.3"}},{devDependencies:{"is-number":"7.0.0"}}],
 ["last-user: declaration removed while a locked package still depends on it",{dependencies:{debug:"4.4.1"}},{devDependencies:{"is-number":"7.0.0"}}],
 ["last-user: declaration moved to another member of the scope",{dependencies:{debug:"4.4.1"}},{dependencies:{ms:"2.1.3"},devDependencies:{"is-number":"7.0.0"}}],
 ["last-user: a package and its transitive dependency both removed",{},{devDependencies:{"is-number":"7.0.0"}}],
 ["last-user: package removed while its transitive dependency stays declared",{dependencies:{ms:"2.1.3"}},{devDependencies:{"is-number":"7.0.0"}}],
 ["last-user: a dependency removed from every member",{dependencies:{debug:"4.4.1",ms:"2.1.3"}},{}],
];
const names=new Set<string>(corpus.cases.map((row:any)=>row.name));
for(const [name,a,b] of specs){
 const full="external packages: "+name;if(names.has(full))continue;
 const manifests=base(a,b);
 corpus.cases.push({name:full,lock:genLock,manifests,expected:JSON.parse(JSON.stringify(manager.checkLock(genLock,new Map(manifests)))),bun:{refused:refuses(genLock,manifests),network:true}});
}
for(const row of corpus.cases){const next=JSON.parse(JSON.stringify(manager.checkLock(row.lock,new Map(row.manifests as [string,unknown][]))));if(JSON.stringify(next)!==JSON.stringify(row.expected)){console.log("CHANGED",row.name,"frozen",row.expected.filter((d:any)=>d.frozen).length,"->",next.filter((d:any)=>d.frozen).length);row.expected=next;}}
writeFileSync(corpusPath,JSON.stringify(corpus,null,1)+"\n");
console.log(corpus.cases.length,"cases");
for(const row of corpus.cases.slice(-6))console.log(row.name,"| frozen",row.expected.filter((d:any)=>d.frozen).map((d:any)=>d.dependency),"| bun",JSON.stringify(row.bun));
