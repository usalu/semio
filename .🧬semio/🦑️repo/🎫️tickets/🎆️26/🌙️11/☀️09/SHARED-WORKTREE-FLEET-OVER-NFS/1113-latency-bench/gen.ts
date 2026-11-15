import {mkdirSync,writeFileSync,utimesSync} from "node:fs";
import {join} from "node:path";
const [root,top="100",mid="25",leaf="20",per="2"]=process.argv.slice(2);
if(!root)throw Error("usage: gen.ts <root> [top mid leaf filesPerLeaf]");
const T=+top,M=+mid,L=+leaf,P=+per;
let files=0,dirs=0;
mkdirSync(root,{recursive:true});
writeFileSync(join(root,".gitignore"),"ignored/\n*.log\n");
const old=Math.floor(Date.now()/1000)-86400*30;
for(let a=0;a<T;a++)for(let b=0;b<M;b++){
 const d=join(root,`pkg${a}`,`mod${b}`);
 mkdirSync(d,{recursive:true});dirs++;
 for(let c=0;c<L;c++){
  const dd=join(d,`sub${c}`);
  mkdirSync(dd);dirs++;
  for(let f=0;f<P;f++){const p=join(dd,`file${f}.txt`);writeFileSync(p,`file ${a}/${b}/${c}/${f}\n`);utimesSync(p,old,old);files++;}
 }
}
console.log(JSON.stringify({files,dirs}));
