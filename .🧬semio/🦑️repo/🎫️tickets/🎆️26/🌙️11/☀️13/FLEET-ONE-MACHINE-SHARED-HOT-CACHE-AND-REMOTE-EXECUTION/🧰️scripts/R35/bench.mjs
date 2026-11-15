import{execSync}from"node:child_process";
const mode=process.argv[2]??"plain",MB=1048576;
const fp=()=>Number(execSync("footprint "+process.pid+" 2>/dev/null | grep -m1 phys_footprint:").toString().match(/(\d+) MB/)?.[1]);
const waves=20,per=250,rows=[],settleMs=[];let kept=[];
for(let w=1;w<=waves;w++){
 const index=Buffer.alloc(300*MB,w&255),patched=Buffer.from(index),packed=Buffer.from(patched.subarray(0,50*MB));
 const wave=[];for(let p=0;p<per;p++){const one=[];for(let r=0;r<400;r++)one.push({kind:"file",path:"p"+w+"/"+p+"/"+r+"/"+"x".repeat(60),fact:{size:r,digest:"d".repeat(40)+r}});wave.push(one);}
 kept=mode==="retain"?kept.concat(wave):wave;
 const t=performance.now();
 if(mode==="gc"){Bun.gc(true);}
 if(mode==="settle"){Bun.gc(true);Bun.shrink();Bun.gc(true);}
 settleMs.push(performance.now()-t);
 rows.push(fp());
}
const tail=rows.slice(2);console.log(mode.padEnd(7),"footprint MB from wave 3: min",Math.min(...tail),"median",tail.sort((a,b)=>a-b)[tail.length>>1],"max",Math.max(...tail),"| settle ms/wave",Math.round(settleMs.reduce((a,b)=>a+b)/waves));
