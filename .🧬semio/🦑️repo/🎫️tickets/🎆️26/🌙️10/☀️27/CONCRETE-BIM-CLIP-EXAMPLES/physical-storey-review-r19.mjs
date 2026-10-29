/** 🗺️ Draw independent storey review sheets from derived occupied model geometry. */
import {readFile,writeFile} from 'node:fs/promises';
import {createRequire} from 'node:module';

export async function renderReview(root,ticket,{input="geometry-plan-r19.json",output="physical-storeys-r19.png"}={}) {
  const require=createRequire(root+'/package.json');
  const {createCanvas}=require('@napi-rs/canvas');
  const g=JSON.parse(await readFile(ticket+'/🗑️generated/'+input,'utf8'));
  const canvas=createCanvas(1840,2240),ctx=canvas.getContext('2d');
  ctx.fillStyle='#f8fafb';ctx.fillRect(0,0,1840,2240);
  const specs=[['cellar',-2.42,-1.20,'Cellar / Keller'],['ground',0,1.20,'Ground / Erdgeschoss'],['first',3.17,4.40,'First / Obergeschoss'],['roof',6.60,7.80,'Roof / Dach']];
  for(let index=0;index<specs.length;index++) {
    const [storey,floor,cut,title]=specs[index],ox=40+(index%2)*920,oy=40+Math.floor(index/2)*1120,scale=37;
    const point=p=>[ox+65+(p[0]+1)*scale,oy+100+(23-p[1])*scale];
    const polygon=(ring,fill,stroke='#34434d',width=.55)=>{if(!ring?.length)return;ctx.beginPath();ring.forEach((p,i)=>{const [x,y]=point(p);i?ctx.lineTo(x,y):ctx.moveTo(x,y);});ctx.closePath();ctx.fillStyle=fill;ctx.fill();if(stroke){ctx.strokeStyle=stroke;ctx.lineWidth=width;ctx.stroke();}};
    const outlineAt=(p,z)=>p.top_outline?p.outline.map((a,i)=>a.map((v,k)=>v+(p.top_outline[i][k]-v)*Math.max(0,Math.min(1,(z-p.z0)/(p.z1-p.z0))))):p.outline;
    const parts=(items,color,mode='cut')=>{for(const item of items)for(const p of item.parts??[]){if(mode==='cut'?(p.z0<=cut&&cut<p.z1):(item.storey===storey&&p.z0<cut&&p.z1>floor-.05))polygon(outlineAt(p,Math.min(cut,p.z1)),color);}};
    ctx.fillStyle='#172c39';ctx.font='bold 26px sans-serif';ctx.fillText(title,ox+35,oy+35);ctx.font='15px sans-serif';ctx.fillText('Physical cut Z '+cut.toFixed(2)+'m · +X → +Y ↑',ox+35,oy+62);
    for(const slab of g.slabs.filter(x=>x.level===storey))for(const p of slab.parts)polygon(p.outline,'#e8ecec','#c4ced1',.4);
    for(const sp of g.spaces.filter(x=>x.storey===storey))polygon(sp.outline,'#edf3f1','#bacac6',.4);
    for(const stair of g.stairs.filter(x=>x.storey===storey))for(const p of stair.steps.concat(stair.landings))if(p.z0<cut)polygon(p.outline,'#c1cbd4','#60778a',.7);
    for(const ramp of g.ramp.ramps)if(ramp.flights[0].z0===floor){for(const f of ramp.flights){const dx=f.end[0]-f.start[0],dy=f.end[1]-f.start[1],len=Math.hypot(dx,dy),nx=-dy/len*f.width/2,ny=dx/len*f.width/2;polygon([[f.start[0]+nx,f.start[1]+ny],[f.end[0]+nx,f.end[1]+ny],[f.end[0]-nx,f.end[1]-ny],[f.start[0]-nx,f.start[1]-ny]],'#d5e1e9','#8299a7',.8);}for(const p of ramp.landings)polygon(p.outline,'#d5e1e9','#8299a7',.8);}
    parts(g.details.concat(g.furnishings,g.beams),'#e1c99e','visible');
    parts(g.walls.concat(g.screens),'#38464f');parts(g.pilotis,'#cb614b');
    parts(g.fillings.concat([g.curtain_wall]),'#51a9bf');parts(g.railings,'#74665b','visible');
    ctx.font='12px sans-serif';ctx.textAlign='center';ctx.fillStyle='#263e4a';
    for(const sp of g.spaces.filter(x=>x.storey===storey)){const xs=sp.outline.map(p=>p[0]),ys=sp.outline.map(p=>p[1]),p=point([(Math.min(...xs)+Math.max(...xs))/2,(Math.min(...ys)+Math.max(...ys))/2]);ctx.fillText(sp.name.en,p[0],p[1]);}
    ctx.textAlign='left';ctx.font='13px sans-serif';ctx.fillStyle='#5b6e78';ctx.fillText('Geometry audit only · no native clip render · furniture below cut shown in plan',ox+35,oy+1070);
  }
  await writeFile(ticket+'/🗑️generated/'+output,canvas.toBuffer('image/png'));
}
