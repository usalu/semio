/** 🗺️ Render public occupied-band geometry for four-storey visual inspection. */
import {readFile,writeFile} from 'node:fs/promises';
import {createRequire} from 'node:module';
export async function render(root,ticket){
 const require=createRequire(root+'/package.json'),{createCanvas}=require('@napi-rs/canvas'),data=JSON.parse(await readFile(ticket+'/🗑️generated/physical-review-r86.json','utf8'));
 const canvas=createCanvas(1840,2240),c=canvas.getContext('2d');c.fillStyle='#f8fafb';c.fillRect(0,0,1840,2240);
 const colors={slabs:'#e2e8e8',stairs:'#91acbd',ramps:'#b3cddd',walls:'#364853',screens:'#364853',pilotis:'#cf725b',fillings:'#54acbb',curtain:'#54acbb',details:'#d2b88d',furnishings:'#dec79f',foundations:'#bfc8c8',railings:'#8e7967',services:'#809d80',beams:'#bbaa8b'};
 for(let k=0;k<data.views.length;k++){
 const v=data.views[k],ox=40+k%2*920,oy=40+Math.floor(k/2)*1120,s=37,point=p=>[ox+70+(p[0]+1)*s,oy+100+(23-p[1])*s];
 c.fillStyle='#152f3e';c.font='bold 27px sans-serif';c.fillText(({cellar:'Cellar / Keller',ground:'Ground / Erdgeschoss',first:'First / Obergeschoss',roof:'Roof / Dach'})[v.storey],ox+25,oy+35);c.font='15px sans-serif';c.fillText('Occupied section Z '+v.cut.toFixed(2)+' m; +X right/rechts; +Y up/oben',ox+25,oy+65);
 const rings=[...v.rings].sort((a,b)=>(a.mode==='below'?0:1)-(b.mode==='below'?0:1));
 for(const r of rings){if(!r.points.length)continue;c.beginPath();r.points.forEach((p,i)=>{const q=point(p);i?c.lineTo(...q):c.moveTo(...q);});c.closePath();c.fillStyle=colors[r.category]??'#888';c.fill();c.strokeStyle=r.mode==='cut'?'#334550':'#81999e';c.lineWidth=r.mode==='cut'?.45:.25;if(r.category!=='slabs')c.stroke();}
 c.font='12px sans-serif';c.textAlign='center';c.fillStyle='#172d39';for(const space of v.spaces){const ps=space.outline,xx=ps.map(p=>p[0]),yy=ps.map(p=>p[1]),p=point([(Math.min(...xx)+Math.max(...xx))/2,(Math.min(...yy)+Math.max(...yy))/2]);const name=space.name.en.split(';')[0];c.fillText(name,p[0],p[1]);}c.textAlign='left';c.font='13px sans-serif';c.fillStyle='#5b6e78';c.fillText('Diagnostic: floor-band material projection; no depth occlusion / Tiefenverdeckung',ox+25,oy+1060);
 }
 await writeFile(ticket+'/🗑️generated/physical-review-r86.png',canvas.toBuffer('image/png'));
 return data.source_hash;
}

/** 🪜 Inspect the authored stair vicinity through the same public occupied-band input. */
export async function renderStairs(root,ticket){
 const require=createRequire(root+'/package.json'),{createCanvas}=require('@napi-rs/canvas'),data=JSON.parse(await readFile(ticket+'/🗑️generated/physical-review-r86.json','utf8'));
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
 await writeFile(ticket+'/🗑️generated/physical-stairs-r86.png',canvas.toBuffer('image/png'));return data.source_hash;
}

/** 🔎 Render current repaired fixture surroundings from exact occupied cut-band rings. */
export async function renderFixtures(root,ticket){
 const require=createRequire(root+'/package.json'),{createCanvas}=require('@napi-rs/canvas'),data=JSON.parse(await readFile(ticket+'/🗑️generated/physical-review-r86.json','utf8'));
 const canvas=createCanvas(1800,1000),c=canvas.getContext('2d');c.fillStyle='#f8fafb';c.fillRect(0,0,1800,1000);
 const colors={walls:'#364853',pilotis:'#cf725b',furnishings:'#dec79f',details:'#d2b88d',fillings:'#54acbb',services:'#809d80',beams:'#bbaa8b'};
 for(let k=0;k<data.detail_views.length;k++){
 const v=data.detail_views[k],ox=k*900,b=v.bounds,s=Math.min(800/(b[2]-b[0]),760/(b[3]-b[1])),point=p=>[ox+50+(p[0]-b[0])*s,120+(b[3]-p[1])*s];
 c.fillStyle='#152f3e';c.font='bold 26px sans-serif';c.fillText(({laundry:'Laundry / Waschkueche',guest:'Guest basin / Gaestebecken'})[v.label]+'; Z '+v.cut.toFixed(2)+'m',ox+40,45);c.font='14px sans-serif';c.fillText('Native occupied material / Belegtes Material',ox+40,75);
 c.save();c.beginPath();c.rect(ox+40,100,820,790);c.clip();
 for(const r of v.rings){if(!colors[r.category]||!r.points.length)continue;c.beginPath();r.points.forEach((p,i)=>{const q=point(p);i?c.lineTo(...q):c.moveTo(...q)});c.closePath();c.fillStyle=colors[r.category];c.fill();c.strokeStyle='#405762';c.lineWidth=.7;c.stroke();}
 if(v.clear_aperture){c.beginPath();v.clear_aperture.forEach((p,i)=>{const q=point(p);i?c.lineTo(...q):c.moveTo(...q)});c.closePath();c.setLineDash([7,5]);c.strokeStyle='#2563a6';c.lineWidth=2;c.stroke();c.setLineDash([]);}
 c.restore();c.font='14px sans-serif';c.fillStyle='#5b6e78';c.fillText(v.clear_aperture?'Dashed: roof clear aperture above / Lichtöffnung darüber':'Diagnostic section; no depth occlusion / Tiefenverdeckung',ox+40,950);
 }
 await writeFile(ticket+'/🗑️generated/physical-fixtures-r86.png',canvas.toBuffer('image/png'));return data.source_hash;
}
