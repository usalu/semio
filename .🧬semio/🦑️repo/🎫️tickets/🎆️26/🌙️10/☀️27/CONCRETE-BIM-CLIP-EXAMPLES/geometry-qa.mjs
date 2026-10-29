/** 🗺️ Render source-derived four-storey plan QA without claiming native clip validation. */
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {dirname,join} from 'node:path';
import {fileURLToPath} from 'node:url';
import {createRequire} from 'node:module';
const ticket=dirname(fileURLToPath(import.meta.url));
const root='C:/git/semio';
const require=createRequire(join(root,'package.json'));
const {createCanvas,loadImage}=require('@napi-rs/canvas');
const output=join(ticket,'🗑️generated');
await mkdir(output,{recursive:true});
const g=JSON.parse(await readFile(join(output,'geometry-r11.json'),'utf8'));
const esc=v=>String(v).replaceAll('&','&amp;').replaceAll('<','&lt;').replaceAll('>','&gt;');
const width=2100,height=2600,scale=42,panelW=1000,panelH=1170;
const storeys=['cellar','ground','first','roof'];
let svg='<svg xmlns="http://www.w3.org/2000/svg" width="'+width+'" height="'+height+'" viewBox="0 0 '+width+' '+height+'"><rect width="100%" height="100%" fill="#f6f3ec"/><style>text{font-family:Arial,sans-serif;fill:#253539} .tag{font-size:10px;paint-order:stroke;stroke:#fff;stroke-width:2px;stroke-linejoin:round}</style><text x="50" y="48" font-size="27">Villa Savoye · Physical Geometry Review</text><text x="50" y="76" font-size="16">Source-derived QA · plan axes · four storeys · native clip playback remains a separate gate</text>';
for(let k=0;k<storeys.length;k++){
 const level=storeys[k],ox=50+(k%2)*1020,oy=125+Math.floor(k/2)*1190;
 const at=p=>[ox+70+p[0]*scale,oy+1080-p[1]*scale];
 const poly=(ring,fill,stroke='#82918e',sw=.8)=>'<polygon points="'+ring.map(p=>at(p).join(',')).join(' ')+'" fill="'+fill+'" stroke="'+stroke+'" stroke-width="'+sw+'"/>';
 const path=(ring,color,sw)=>'<polyline points="'+ring.map(p=>at(p).join(',')).join(' ')+'" fill="none" stroke="'+color+'" stroke-width="'+sw+'"/>';
 const text=(value,p,size=10)=>{const [x,y]=at(p);return '<text class="tag" x="'+x+'" y="'+y+'" font-size="'+size+'">'+esc(value)+'</text>';};
 svg+='<rect x="'+ox+'" y="'+oy+'" width="'+panelW+'" height="'+panelH+'" fill="none" stroke="#bac5bf"/><text x="'+(ox+20)+'" y="'+(oy+30)+'" font-size="24">'+esc(level)+' · '+g.storeys[level]+' m</text>';
 for(const [i,x] of g.grid.x.entries())svg+=path([[x,0],[x,22]],'#d7dfd8',.6);
 for(const y of g.grid.y)svg+=path([[0,y],[19,y]],'#d7dfd8',.6);
 for(const slab of g.slabs.filter(s=>s.storey===level))for(const p of slab.parts)svg+=poly(p.outline,'#ece9df','#b3b8b0',.45);
 for(const room of g.spaces.filter(s=>s.storey===level))svg+=poly(room.outline,'none','#d3aa72',.7);
 for(const kind of ['details','furnishings','foundations','beams','railings'])for(const item of g[kind]||[])if(item.storey===level)for(const part of item.parts){const color=kind==='furnishings'?'#bdcdcc':kind==='details'?'#d8cfb5':kind==='railings'?'#9db8bb':'#d5d3c5';svg+=poly(part.outline,color,'#728781',.55);}
 for(const stair of g.stairs.filter(s=>s.storey===level)){for(const p of [...stair.steps,...stair.landings])svg+=poly(p.outline,'#e0dfd4','#9a9e91',.6);svg+=text(stair.tag,stair.centre);}
 for(const ramp of g.ramp.ramps)for(const f of ramp.flights)if(Math.min(f.z0,f.z1)>=g.storeys[level]-.1 && Math.min(f.z0,f.z1)<g.storeys[level]+2){const [a,b]=[f.start,f.end],dx=b[0]-a[0],dy=b[1]-a[1],len=Math.hypot(dx,dy),nx=-dy/len*f.width/2,ny=dx/len*f.width/2;svg+=poly([[a[0]+nx,a[1]+ny],[b[0]+nx,b[1]+ny],[b[0]-nx,b[1]-ny],[a[0]-nx,a[1]-ny]],'#d8d9d1','#7f8c80');}
 const wall=(w)=>{const a=w.start,b=w.end,dx=b[0]-a[0],dy=b[1]-a[1],len=Math.hypot(dx,dy),n=[-dy/len*w.thickness/2,dx/len*w.thickness/2];let intervals=[[0,len]];for(const op of w.openings||[])if(op.sill_z<g.storeys[level]+1.1&&op.head_z>g.storeys[level]+1.1)intervals=intervals.flatMap(([lo,hi])=>[[lo,Math.min(hi,op.start)],[Math.max(lo,op.end),hi]].filter(([l,h])=>h>l+1e-9));for(const [lo,hi]of intervals){const p=[a[0]+dx/len*lo,a[1]+dy/len*lo],q=[a[0]+dx/len*hi,a[1]+dy/len*hi];svg+=poly([[p[0]+n[0],p[1]+n[1]],[q[0]+n[0],q[1]+n[1]],[q[0]-n[0],q[1]-n[1]],[p[0]-n[0],p[1]-n[1]]],'#455d57','#455d57',.4);}for(const op of w.openings||[]){const p=[a[0]+dx/len*op.start,a[1]+dy/len*op.start],q=[a[0]+dx/len*op.end,a[1]+dy/len*op.end];if(op.fill_tag)svg+=path([p,q],op.fill_class==='IfcDoor'?'#cb793a':'#4289a5',1.5);}};
 for(const w of g.walls.filter(s=>s.storey===level))wall(w);
 if(level==='ground')for(const p of g.curtain_wall.paths)svg+=path(p,'#4289a5',2);
 if(level==='roof')for(const screen of g.screens)svg+=path(screen.path,'#455d57',screen.thickness*scale);
 for(const col of g.pilotis){const pieces=col.parts.filter(p=>p.z0<=g.storeys[level]+1.1&&p.z1>=g.storeys[level]+1.1);for(const p of pieces){svg+=poly(p.outline,'#ce4c41','#892b2b',.6);svg+=text(col.tag,[col.x+.14,col.y+.15]);}}
 for(const room of g.spaces.filter(s=>s.storey===level)){const centre=room.outline.reduce((a,p)=>[a[0]+p[0]/room.outline.length,a[1]+p[1]/room.outline.length],[0,0]);svg+=text(room.tag,centre,11);}
}
svg+='<text x="50" y="2555" font-size="16">Red: actual support contours · green: cut walls · blue: glazing · orange: doors · grey: authored structure and fixtures</text></svg>';
await writeFile(join(output,'storeys-r11.svg'),svg);
const canvas=createCanvas(width,height);const ctx=canvas.getContext('2d');ctx.drawImage(await loadImage(Buffer.from(svg)),0,0);await writeFile(join(output,'storeys-r11.png'),canvas.toBuffer('image/png'));
export const result={svg:join(output,'storeys-r11.svg'),png:join(output,'storeys-r11.png')};
