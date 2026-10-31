/** 🏛️ Render actual occupied faces and exact public sections through the existing canvas interface. */
import {readFile,writeFile} from 'node:fs/promises';
import {createRequire} from 'node:module';
import {fileURLToPath} from 'node:url';
import path from 'node:path';
const ticket=path.dirname(fileURLToPath(import.meta.url));
let root=ticket;while(!root.endsWith('semio')){const next=path.dirname(root);if(next===root)throw Error('Workspace root not found');root=next;}
const require=createRequire(root+'/package.json'),{createCanvas}=require('@napi-rs/canvas');
const data=JSON.parse(await readFile(ticket+'/🗑️generated/physical-vertical-r92.json','utf8'));
const colors={slabs:'#d8dedc',stairs:'#92acb9',ramps:'#adc6d2',walls:'#e4e7e3',screens:'#e4e7e3',pilotis:'#c9d2ce',fillings:'#5da0ac',curtain:'#70abb3',details:'#c7b99b',furnishings:'#bca17b',foundations:'#aeb9b4',railings:'#6f7f7b',services:'#8eae8f',beams:'#c9c8bc'};
const trace=(c,ps,project)=>{c.beginPath();ps.forEach((p,i)=>{const q=project(p);i?c.lineTo(...q):c.moveTo(...q)});c.closePath();};
function sheet(){const canvas=createCanvas(2200,1700),c=canvas.getContext('2d');c.fillStyle='#f8faf9';c.fillRect(0,0,2200,1700);return {canvas,c};}
function title(c,text,index){const x=40+(index%2)*1100,y=40+Math.floor(index/2)*850;c.fillStyle='#203c43';c.font='bold 25px sans-serif';c.fillText(text,x,y+25);c.font='14px sans-serif';c.fillText('Villa Savoye | Original 1930-1931 reconstruction / Rekonstruktion',x,y+52);return {x,y};}
const cross=(a,b)=>[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]];
const sub=(a,b)=>a.map((v,i)=>v-b[i]);
const dot=(a,b)=>a.reduce((s,v,i)=>s+v*b[i],0);
const normal=ps=>{for(let i=1;i<ps.length-1;i++){const n=cross(sub(ps[i],ps[0]),sub(ps[i+1],ps[0]));if(dot(n,n)>1e-22)return n;}return [0,0,0];};
for(const mode of ['elevations','sections','axons']){
 const {canvas,c}=sheet();
 for(let k=0;k<4;k++){
  const axis=k%2,sign=k<2?-1:1,sec=data.sections[k],label=mode==='sections'?sec.label:mode==='elevations'?'View / Ansicht '+(axis?'Y':'X')+(sign<0?' -':' +'):'Axon / Axonometrie '+(k+1),{x,y}=title(c,label,k);
  c.save();c.beginPath();c.rect(x,y+70,1040,700);c.clip();
  if(mode==='sections'){
   const project=p=>[x+55+(p[0]+1)*40,y+620-p[1]*40];
   for(const z of [-2.42,0,3.17,6.6,9.6]){c.strokeStyle='#d2dedd';c.lineWidth=.7;c.beginPath();c.moveTo(x+20,y+620-z*40);c.lineTo(x+1030,y+620-z*40);c.stroke();c.fillStyle='#587171';c.font='12px sans-serif';c.fillText(z.toFixed(2)+' m',x+5,y+616-z*40);}
   for(const r of sec.rings){trace(c,r.points,project);c.fillStyle=colors[r.category]??'#b9c3bd';c.fill();c.strokeStyle='#304b52';c.lineWidth=.45;c.stroke();}
  }else{
   const view=mode==='elevations'?(axis?[0,sign,0]:[sign,0,0]):[[1,-1,.8],[-1,-1,.8],[-1,1,.8],[1,1,.8]][k];
   const norm=Math.hypot(...view),v=view.map(a=>a/norm),right=mode==='elevations'?(axis?[sign,0,0]:[0,-sign,0]):(()=>{const a=cross([0,0,1],v),n=Math.hypot(...a);return a.map(q=>q/n)})(),up=mode==='elevations'?[0,0,1]:cross(v,right);
   const centre=[9.65,10.8,3.4],scale=mode==='elevations'?40:29,project=p=>{const q=sub(p,centre);return [x+520+dot(q,right)*scale,y+390-dot(q,up)*scale]};
   const faces=[];
   for(const solid of data.solids)for(const f of solid.faces){const ps=f.map(i=>solid.vertices[i]),n=normal(ps);if(dot(n,v)<=1e-12)continue;const centre=ps[0].map((_,i)=>ps.reduce((s,p)=>s+p[i],0)/ps.length);faces.push({ps,depth:dot(centre,v),category:solid.category});}
   faces.sort((a,b)=>a.depth-b.depth);
   for(const f of faces){trace(c,f.ps,project);c.fillStyle=colors[f.category]??'#b9c3bd';c.fill();c.strokeStyle='#617a7a';c.lineWidth=.25;c.stroke();}
  }
  c.restore();c.fillStyle='#657a7c';c.font='13px sans-serif';c.fillText(mode==='sections'?'Exact occupied vertical cuts / Exakte Materialquerschnitte':'Actual occupied faces; painter depth order / Reale Materialflaechen, Tiefensortierung',x,y+795);
 }
 await writeFile(ticket+'/🗑️generated/physical-'+mode+'-r92.png',canvas.toBuffer('image/png'));
}
console.log(JSON.stringify({source_hash:data.source_hash,solids:data.solids.length,outputs:['elevations','sections','axons']}));

for(const mode of ['plans','sanitary']){
 const {canvas,c}=sheet();
 for(let k=0;k<4;k++){
  const plan=data.plans[mode==='plans'?k:4+(k%2)],{x,y}=title(c,plan.label,k);
  const detail=mode==='sanitary',origin=detail?(k<2?[.2,13.7]:[4.2,11.5]):[-1,-1],scale=detail?130:30;
  const project=p=>[x+80+(p[0]-origin[0])*scale,y+80+(p[1]-origin[1])*scale];
  c.save();c.beginPath();c.rect(x,y+65,1040,700);c.clip();
  for(const r of plan.rings){trace(c,r.points,project);c.fillStyle=r.material==='MAT-MOSAIC-BLUE'?'#6490ab':r.material==='MAT-MOSAIC-BLACK'?'#22282c':colors[r.category]??'#b9c3bd';c.fill();c.strokeStyle='#304b52';c.lineWidth=.5;c.stroke();}
  c.restore();c.fillStyle='#657a7c';c.font='13px sans-serif';c.fillText('Exact occupied horizontal cut / Exakter Materialgrundriss',x,y+795);
 }
 await writeFile(ticket+'/🗑️generated/physical-'+mode+'-r92.png',canvas.toBuffer('image/png'));
}
