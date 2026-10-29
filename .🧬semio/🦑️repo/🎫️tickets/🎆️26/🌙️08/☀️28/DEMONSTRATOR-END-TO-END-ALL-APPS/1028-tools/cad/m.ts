const {measure}=await import("../gatecensus/census.ts");
const dir=process.argv[2];
const r=await measure(dir,[]);
console.log(JSON.stringify(r,null,1));
