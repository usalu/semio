const [oldRoot,newRoot,worktree]=process.argv.slice(2) as [string,string,string];
const a=await import(`${oldRoot}/🚧️boundary/🟦️.ts`),b=await import(`${newRoot}/🚧️boundary/🟦️.ts`);
const git=await import(`${oldRoot}/🌿️git/🟦️.ts`);
const tracked=(await git.trackedEntriesV1(worktree))!;
let bad=0,count=0;
const check=(name:string)=>{count++;if(a.foldNameV1(name)!==b.foldNameV1(name)){if(bad++<5)console.log("MISMATCH",JSON.stringify(name));}};
for(const e of tracked)check(e.path);
for(let cp=0;cp<0x110000;cp++){if(cp>=0xd800&&cp<=0xdfff)continue;const c=String.fromCodePoint(cp);check(c);check("é"+c+"Ab");}
check("");check("\ud800x");check("Straße ẞ ΑΣ Σ ı ſ İ");
console.log("checked",count,"mismatches",bad);
let t=performance.now();for(const e of tracked)a.foldNameV1(e.path);console.log("old ms",(performance.now()-t).toFixed(0));
t=performance.now();for(const e of tracked)b.foldNameV1(e.path);console.log("new ms",(performance.now()-t).toFixed(0));
