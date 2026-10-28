const rss=()=>Math.round(process.memoryUsage().rss/1048576);
const mode=process.argv[2],size=Number(process.argv[3]||1000000),n=Number(process.argv[4]||300);
const big=JSON.stringify("x".repeat(size));
const source='export const a=import.meta.semioCapturedSourceResolutions;export const b=import.meta.url;export function f(x){return x+1}';
Bun.gc(true);const base=rss();
for(let i=0;i<n;i++){
 if(mode==="per"){const t=new Bun.Transpiler({loader:"ts",target:"bun",define:{"import.meta.semioCapturedSourceResolutions":big,"import.meta.url":JSON.stringify("file:///x/"+i)}});t.transformSync(source);}
 else if(mode==="small"){const t=new Bun.Transpiler({loader:"ts",target:"bun",define:{"import.meta.url":JSON.stringify("file:///x/"+i)}});t.transformSync(source);}
 else if(mode==="shared"){globalThis.__t??=new Bun.Transpiler({loader:"ts",target:"bun",define:{"import.meta.semioCapturedSourceResolutions":big,"import.meta.url":"__m.url"}});globalThis.__t.transformSync(source);}
}
console.log(mode,"size",size,"n",n,"rss before",base,"after",rss(),"delta",rss()-base);
