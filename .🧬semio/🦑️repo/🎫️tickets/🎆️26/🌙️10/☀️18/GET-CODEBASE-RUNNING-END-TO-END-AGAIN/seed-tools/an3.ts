const R="/Users/ueli/Documents/semio/";
for(const f of [".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){
 const t=await Bun.file(R+f).text(); const j=Bun.JSONC.parse(t);
 const s=JSON.stringify(j,null,2);
 console.log(f,t.length, s.length, t===s, t===s+"\n", t.includes("//"), (t.match(/^\s*\/\//gm)||[]).length, t.startsWith("{\n  \"version\""));
 // find first diff
 let i=0; const u=s+"\n"; while(i<t.length&&t[i]===u[i])i++; console.log("firstdiff",i,JSON.stringify(t.slice(i-60,i+80)),JSON.stringify(u.slice(i-60,i+80)));
}
