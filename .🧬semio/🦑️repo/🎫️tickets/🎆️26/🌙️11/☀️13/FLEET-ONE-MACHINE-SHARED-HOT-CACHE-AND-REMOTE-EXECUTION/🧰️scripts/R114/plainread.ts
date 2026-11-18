import{readdirSync,statSync}from"node:fs";
const R="/Users/ueli/Documents/semio",B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const{libraryBootstrap}=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await libraryBootstrap;
const OB:any=await import(B+"👁️observation/🟨️.mjs");
const dir=R+"/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const names=readdirSync(dir).filter(n=>n.endsWith(".json")&&!n.includes("owner-peaks")).slice(0,Number(process.argv[2]||60));
const cpu=()=>{const u=process.cpuUsage();return(u.user+u.system)/1000;};
let ok=0,miss=0,rows=0,bytes=0;const per:number[]=[];const t0=performance.now(),c0=cpu();
for(const n of names){const c=cpu();let r:any;try{r=await OB.verifyStoredProjectRecordV1({cachePath:dir+n,key:["none"]});}catch(e:any){r={verdict:"err:"+String(e?.message).slice(0,50)};}per.push(cpu()-c);if(r.headerDigest)ok++;else miss++;bytes+=statSync(dir+n).size;if(ok+miss<4)console.log(n.slice(0,30),JSON.stringify(r).slice(0,120));}
per.sort((a,b)=>a-b);console.log("PLAIN-READ",JSON.stringify({records:names.length,readable:ok,unreadable:miss,cpuPerRecordMs:Math.round((cpu()-c0)/names.length*10)/10,p50:Math.round(per[Math.floor(per.length/2)]*10)/10,p90:Math.round(per[Math.floor(per.length*0.9)]*10)/10,avgBytes:Math.round(bytes/names.length)}));process.exit(0);
