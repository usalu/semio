const a = JSON.parse(await Bun.file(process.argv[2]).text());
function sh(v:any,d:number,k:string){ if(d>4) return; if(Array.isArray(v)){ console.log(" ".repeat(d)+k+"[]"+v.length); if(v.length&&typeof v[0]==="object") sh(v[0],d+1,"0"); return;} if(v&&typeof v==="object"){ console.log(" ".repeat(d)+k+"{"+Object.keys(v).slice(0,30).join(",")+"}"); for(const kk of Object.keys(v).slice(0,30)) if(typeof v[kk]==="object") sh(v[kk],d+1,kk);} }
sh(a,0,"root");
