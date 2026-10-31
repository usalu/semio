/** 🗄️ Generate inferred hollow cabinet construction inside existing registered envelopes. */
import {writeFile} from 'node:fs/promises';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
const ticket=path.dirname(fileURLToPath(import.meta.url));
const rrStore92=x=>Math.round(x*1e12)/1e12;
const cabinetParts92=(x0,y0,x1,y1,height,orientation='y')=>{var L=orientation==='y'?x1-x0:y1-y0,D=orientation==='y'?y1-y0:x1-x0,parts=[];var point=(u,v)=>orientation==='y'?[rrStore92(x0+u),rrStore92(y0+v)]:[rrStore92(x1-v),rrStore92(y0+u)];var box=(u0,v0,u1,v1,z0,z1,role='cabinet_carcass',material='MAT-WOOD')=>{if(z1<=z0||u1<=u0||v1<=v0)throw Error('Invalid cabinet piece');parts.push({outline:[[u0,v0],[u1,v0],[u1,v1],[u0,v1]].map(([u,v])=>point(u,v)),z0:rrStore92(z0),z1:rrStore92(z1),role,material});};for(var u of [.04,L-.10])for(var v of [.035,D-.095])box(u,v,u+.06,v+.06,0,.08);box(0,0,L,D,.08,.12);box(0,0,L,D,height-.04,height);box(0,0,.04,D,.12,height-.04);box(L-.04,0,L,D,.12,height-.04);box(.04,0,L-.04,.02,.12,height-.04);for(var i=1,count=height>1.2?3:1;i<=count;i++){var z=.12+i*(height-.16)/(count+1);box(.04,.02,L-.04,D-.025,z,z+.022,'cabinet_shelf');}for(var [u0,u1] of [[.042,L/2-.002],[L/2+.002,L-.042]])box(u0,D-.025,u1,D,.12,height-.04,'cabinet_front');var zh=.12+.55*(height-.16);for(var u of [L*.25,L*.75]){for(var z of [zh-.045,zh+.039])box(u-.004,D,u+.004,D+.012,z,z+.006,'cabinet_pull','MAT-METAL');box(u-.005,D+.012,u+.005,D+.02,zh-.045,zh+.045,'cabinet_pull','MAT-METAL');}return parts;};
const assets={'FIX-GUEST-WARDROBE':cabinetParts92(2.012,10.219,2.45,11.5,1.75,'x'),'FIX-SON-STORAGE':cabinetParts92(.5,16.65,2.3,17.1,.6),'FIX-PANTRY-STORAGE':[...cabinetParts92(3.3,6.5,3.7,8.35,1.9,'x'),...cabinetParts92(3.3,5.85,3.7,6.45,.8,'x')]};
await writeFile(path.join(ticket,'🗑️generated','fixed-storage-assets-r92.json'),JSON.stringify(assets,null,2)+'\n');

export {cabinetParts92};
