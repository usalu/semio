const [oldRoot,newRoot,worktree]=process.argv.slice(2) as [string,string,string];
const a=await import(`${oldRoot}/🚧️boundary/🟦️.ts`),b=await import(`${newRoot}/🚧️boundary/🟦️.ts`);
const git=await import(`${oldRoot}/🌿️git/🟦️.ts`);
const tracked=(await git.trackedEntriesV1(worktree))!;
let bad=0,count=0;
const check=(name:string)=>{count++;if(a.refusePathV1(name)!==b.refusePathV1(name)){if(bad++<8)console.log("MISMATCH",JSON.stringify(name),a.refusePathV1(name),b.refusePathV1(name));}};
for(const e of tracked)check(e.path);
const names=["con","CON","Con.txt","nul.verylongextension.x","aux ","aux .txt","CONIN$","conout$","clock$","com1","COM¹","lpt9.log","LPT0","x/con/y","x/CON /y","a./b","a /b","a/b.","a/b ","","/","//","a//b","ı","CONıN$","ſ","p","pr","prn","prn.","a/prn","a/prn/","é/aux","a<b","a|b","a\u0001b","a\u007fb","x".repeat(86),"é".repeat(86),"é".repeat(128),"é".repeat(127),"a/"+"é".repeat(128),"💩".repeat(64),"💩".repeat(65),"é","a/é/con"];
for(const n of names)check(n);
for(const x of names)for(const y of names)check(`${x}/${y}`);
const chars=["a","C","o","N",".", " ","/","ı","é","<","\u0000","1","¹","$","n","u","L","p","t","m","x"];
let seed=7;const rnd=()=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed;};
for(let i=0;i<400000;i++){let n="";const len=1+rnd()%9;for(let j=0;j<len;j++)n+=chars[rnd()%chars.length];check(n);}
console.log("checked",count,"mismatches",bad);
const t0=performance.now();for(const e of tracked)a.refusePathV1(e.path);const t1=performance.now();for(const e of tracked)b.refusePathV1(e.path);console.log("old ms",(t1-t0).toFixed(0),"new ms",(performance.now()-t1).toFixed(0));
