import{mkdirSync,writeFileSync}from"node:fs";import{join}from"node:path";
const root=process.argv[2],N=Number(process.argv[3]);const dir=join(root,"j","a".repeat(20));
mkdirSync(join(dir,"node_modules"),{recursive:true});writeFileSync(join(dir,"package.json"),"{}");
const t=performance.now();let n=0;for(let d=0;n<N;d++){const p=join(dir,"node_modules","pkg"+d,"lib");mkdirSync(p,{recursive:true});n+=2;for(let f=0;f<40&&n<N;f++,n++)writeFileSync(join(p,"f"+f+".js"),"x".repeat(300+f*50));}
console.log("built",n,Math.round(performance.now()-t),"ms");
