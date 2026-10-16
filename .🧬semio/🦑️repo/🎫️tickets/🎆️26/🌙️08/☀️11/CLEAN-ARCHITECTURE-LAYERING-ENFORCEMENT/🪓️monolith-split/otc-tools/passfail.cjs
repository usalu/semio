const fs=require("fs");const [,,b,a]=process.argv;
const load=f=>{const m=new Map();const files=new Map();for(const l of fs.readFileSync(f,"utf8").split("\n")){if(!l)continue;const [st,file,title,msg]=l.split("\t");if(title==="<file>"){files.set(file,{st,msg});continue;}const key=title;const e=m.get(key)??{passed:0,other:0};if(st==="passed")e.passed++;else e.other++;m.set(key,e);}return {m,files};};
const B=load(b),A=load(a);
let regress=0,fixed=0,newPass=0,newFail=0,same=0;
for(const [k,s] of B.m){const t=A.m.get(k);if(t===undefined){console.log("MISSING after:",k.slice(0,150));regress++;continue;}
  if(t.passed<s.passed){console.log("REGRESSED (passed count fell):",k.slice(0,150),s,t);regress++;}else if(t.other<s.other){fixed+=s.other-t.other;}else same++;}
for(const [k,t] of A.m) if(!B.m.has(k)){ newPass+=t.passed; newFail+=t.other; }
console.log(`per-title: same=${same} fixedToPass=${fixed} regressed/missing=${regress} newlyCollectedPassing=${newPass} newlyCollectedFailing=${newFail}`);
const fb=[...B.files.keys()].map(f=>f),fa=[...A.files.keys()];
console.log(`file-level failures before=${fb.length} after=${fa.length}`);
const tail=f=>f.split("/").slice(-4).join("/");
for(const f of fa) if(!B.files.has(f)) console.log("  NEW file-level failure:",tail(f),"|",(A.files.get(f).msg||"").slice(0,160));
