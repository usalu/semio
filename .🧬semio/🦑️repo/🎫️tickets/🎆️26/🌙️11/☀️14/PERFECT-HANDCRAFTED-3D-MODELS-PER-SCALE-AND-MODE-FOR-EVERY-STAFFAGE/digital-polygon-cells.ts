/** 📐 Compare contour stitching at the input and exact slab resolutions against polygon-clipping. */
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';
import pc from 'polygon-clipping';
const p=resolve(process.argv[2]),{rings}=JSON.parse(readFileSync(process.argv[3],'utf8')),cell=Number(process.argv[4]),debug=process.argv[5]==='--trace';
Bun.plugin({name:'measure-slab',setup(b){b.onLoad({filter:/🔺️polygon\/🟦️\.ts$/},args=>{
 let s=readFileSync(args.path,'utf8').replace('const cell = 1e-7;',`const cell = ${cell};`);
 if(debug)s=s.replace('if (current !== i || ring.length < 3) closed = false;','if (current !== i || ring.length < 3) { console.log("chain",i,ring.length,current,ring.slice(0,2),ring.at(-1));closed=false;}');
 return {contents:s+'\nexport { slabUnion,primaryUnion };',loader:'ts'};
});}});
const lib=await import(p),want=pc.union(...rings.map(r=>[r.map(q=>[q.x,q.y])])).reduce((v,m)=>v+m.reduce((s,r)=>s+lib.ringArea(r.map(([x,y])=>({x,y}))),0),0);
const m=lib.slabUnion(rings);console.log(cell,m.rings.length,m.area,m.rings.reduce((s,r)=>s+lib.ringArea(r),0),want);
