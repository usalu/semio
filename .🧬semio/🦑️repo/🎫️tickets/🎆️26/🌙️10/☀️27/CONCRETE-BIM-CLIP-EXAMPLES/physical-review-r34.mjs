/** 🗺️ Render public occupied-band geometry for four-storey visual inspection. */
import {readFile,writeFile} from 'node:fs/promises';
import {createRequire} from 'node:module';
export async function render(root,ticket){
 const require=createRequire(root+'/package.json'),{createCanvas}=require('@napi-rs/canvas'),data=JSON.parse(await readFile(ticket+'/🗑️generated/physical-review-r34.json','utf8'));
 const canvas=createCanvas(1840,2240),c=canvas.getContext('2d');c.fillStyle='#f8fafb';c.fillRect(0,0,1840,2240);
 const colors={slabs:'#e2e8e8',stairs:'#91acbd',ramps:'#b3cddd',walls:'#364853',screens:'#364853',pilotis:'#cf725b',fillings:'#54acbb',curtain:'#54acbb',details:'#d2b88d',furnishings:'#dec79f',foundations:'#bfc8c8',railings:'#8e7967',services:'#809d80',beams:'#bbaa8b'};
 for(let k=0;k<data.views.length;k++){
 const v=data.views[k],ox=40+k%2*920,oy=40+Math.floor(k/2)*1120,s=37,point=p=>[ox+70+(p[0]+1)*s,oy+100+(23-p[1])*s];
 c.fillStyle='#152f3e';c.font='bold 27px sans-serif';c.fillText(({cellar:'Cellar / Keller',ground:'Ground / Erdgeschoss',first:'First / Obergeschoss',roof:'Roof / Dach'})[v.storey],ox+25,oy+35);c.font='15px sans-serif';c.fillText('Occupied section Z '+v.cut.toFixed(2)+' m · +X → +Y ↑',ox+25,oy+65);
 const rings=[...v.rings].sort((a,b)=>(a.mode==='below'?0:1)-(b.mode==='below'?0:1));
 for(const r of rings){if(!r.points.length)continue;c.beginPath();r.points.forEach((p,i)=>{const q=point(p);i?c.lineTo(...q):c.moveTo(...q);});c.closePath();c.fillStyle=colors[r.category]??'#888';c.fill();c.strokeStyle=r.mode==='cut'?'#334550':'#81999e';c.lineWidth=r.mode==='cut'?.45:.25;if(r.category!=='slabs')c.stroke();}
 c.font='12px sans-serif';c.textAlign='center';c.fillStyle='#172d39';for(const space of v.spaces){const ps=space.outline,xx=ps.map(p=>p[0]),yy=ps.map(p=>p[1]),p=point([(Math.min(...xx)+Math.max(...xx))/2,(Math.min(...yy)+Math.max(...yy))/2]);const name=space.name.en.split(';')[0];c.fillText(name,p[0],p[1]);}c.textAlign='left';c.font='13px sans-serif';c.fillStyle='#5b6e78';c.fillText('Diagnostic: floor-band material projection; no depth occlusion / Tiefenverdeckung',ox+25,oy+1060);
 }
 await writeFile(ticket+'/🗑️generated/physical-review-r34.png',canvas.toBuffer('image/png'));
 return data.source_hash;
}

/** 🪜 Inspect the authored stair vicinity through the same public occupied-band input. */
export async function renderStairs(root,ticket){
 const require=createRequire(root+'/package.json'),{createCanvas}=require('@napi-rs/canvas'),data=JSON.parse(await readFile(ticket+'/🗑️generated/physical-review-r34.json','utf8'));
 const canvas=createCanvas(1600,1600),c=canvas.getContext('2d');c.fillStyle='#f8fafb';c.fillRect(0,0,1600,1600);
 const colors={slabs:'#e2e8e8',stairs:'#638da7',ramps:'#b3cddd',walls:'#364853',screens:'#364853',pilotis:'#cf725b',fillings:'#54acbb',curtain:'#54acbb',details:'#d2b88d',furnishings:'#dec79f',foundations:'#bfc8c8',railings:'#8e7967',services:'#809d80',beams:'#bbaa8b'};
 for(let k=0;k<data.views.length;k++){
  const v=data.views[k],ox=k%2*800,oy=Math.floor(k/2)*800,s=k===3?120:180,x=k===3?2.5:4.1,y=k===3?10.8:11.1,point=p=>[ox+40+(p[0]-x)*s,oy+90+(y-p[1])*s];
  c.fillStyle='#152f3e';c.font='bold 26px sans-serif';c.fillText(v.storey+' · Z '+v.cut.toFixed(2)+' m',ox+30,oy+40);
  c.save();c.beginPath();c.rect(ox+20,oy+70,760,650);c.clip();
  for(const r of [...v.rings].sort((a,b)=>(a.mode==='below'?0:1)-(b.mode==='below'?0:1))){
   if(!r.points.length)continue;c.beginPath();r.points.forEach((p,i)=>{const q=point(p);i?c.lineTo(...q):c.moveTo(...q)});c.closePath();c.fillStyle=colors[r.category]??'#888';c.fill();c.strokeStyle=r.mode==='cut'?'#334550':'#81999e';c.lineWidth=r.mode==='cut'?1:.6;if(r.category!=='slabs')c.stroke();
  }
  c.restore();c.fillStyle='#5b6e78';c.font='14px sans-serif';c.fillText('Native occupied bands; no depth occlusion / Tiefenverdeckung',ox+30,oy+755);
 }
 await writeFile(ticket+'/🗑️generated/physical-stairs-r38.png',canvas.toBuffer('image/png'));return data.source_hash;
}
