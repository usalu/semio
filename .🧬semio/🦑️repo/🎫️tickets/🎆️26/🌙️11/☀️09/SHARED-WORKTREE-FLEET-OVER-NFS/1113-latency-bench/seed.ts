const [module,worktree,store,origin="b1"]=process.argv.slice(2);
const {seedV1}=await import(`${module}/🔁️replica/🟦️.ts`);
const t=performance.now();
const result=await seedV1({worktree,storeRoot:store,originId:origin,groupId:"g1",concurrency:8},undefined,()=>{});
console.log(JSON.stringify({seedSeconds:((performance.now()-t)/1000).toFixed(1),published:result.published,bytes:result.bytes}));
