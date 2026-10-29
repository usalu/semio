import fs from 'node:fs/promises';
import crypto from 'node:crypto';
const root='C:/git/semio';
const ticket=root+'/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES';
const modelPath=root+'/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json';
function identity(tag){const ns=Buffer.from('6ba7b8119dad11d180b400c04fd430c8','hex');const b=crypto.createHash('sha1').update(ns).update('semio:villa-savoye:'+tag).digest().subarray(0,16);b[6]=(b[6]&15)|80;b[8]=(b[8]&63)|128;let n=BigInt('0x'+b.toString('hex')),s='';const alphabet='0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$';for(let i=0;i<22;i++){s=alphabet[Number(n&63n)]+s;n>>=6n;}return s;}
const q=(value,basis='sourced')=>({value,unit:'m',basis,source:'Savoye-as-built-p69-and-CSIC2022-Annex1-5'});
const prop=(value,basis='inferred')=>({value,ifc_type:typeof value==='boolean'?'IfcBoolean':'IfcLabel',basis,source:'Savoye-as-built-p69-and-CSIC2022-Annex1-5'});
export async function prepare(){
 const input=JSON.parse(await fs.readFile(ticket+'/foundations-r16-input.json','utf8'));
 const model=JSON.parse(await fs.readFile(modelPath,'utf8'));
 const r=input.registration,d=input.datums;
 const point=([x,y])=>[+(r.x_scale*(r.x_origin_px-x)).toFixed(6),+(r.y_origin_m+r.y_scale*(y-r.y_origin_px)).toFixed(6)];
 const box=([x0,y0,x1,y1])=>[point([x1,y0]),point([x0,y0]),point([x0,y1]),point([x1,y1])];
 const square=(axis,size)=>[[axis[0]-size/2,axis[1]-size/2],[axis[0]+size/2,axis[1]-size/2],[axis[0]+size/2,axis[1]+size/2],[axis[0]-size/2,axis[1]+size/2]];
 const elements=[],relations=[],column_updates=[];
 const z0=d.bearing,z1=+(z0+d.blinding_thickness).toFixed(6),z2=+(z1+d.rc_base_thickness).toFixed(6),z3=+(z2+d.pyramid_height).toFixed(6);
 const relation=(tag,relating,related,en,de)=>({guid:identity(tag),tag,class:'IfcRelConnectsElements',relating,related,name:{en,de}});
 function footing(tag,material,parts,en,de,id){return {guid:identity(tag),tag,class:'IfcFooting',predefined_type:'PAD_FOOTING',name:{en,de},storey:'ground',material,geometry:{kind:'registered-foundation',base_level:'ground',parts},attributes:{},psets:{Pset_FootingCommon:{LoadBearing:prop(true)},Pset_ArchiveSource:{ArchivePad:prop(id,'sourced'),PlanTrace:prop('Annex1 raster trace; approximately +/-0.04m'),BearingDatum:prop('Annex5 -1.65m; book -1.50m remains an unresolved reference conflict','sourced'),VerticalProfile:prop('0.40m blinding and0.15m pyramid inferred;0.10m RCbase and stemtop-0.30m sourced')}},metadata:{author:'Structural engineer',revision:3,domains:['architecture','structure']}};}
 for(const pad of input.pads){
  const blinding='FT-BLINDING-'+pad.id,tag='FT-PAD-'+pad.id;
  let blindingOutline=box(pad.blinding_pixels);
  if(pad.material_ownership_cut){const wall=model.elements.find(e=>e.tag===pad.material_ownership_cut.wall);const thickness=wall.attributes.thickness.value;const xs=blindingOutline.map(p=>p[0]),ys=blindingOutline.map(p=>p[1]);const minx=Math.min(...xs),maxx=Math.max(...xs),miny=Math.min(...ys),maxy=Math.max(...ys),cutx=Math.min(wall.geometry.path[0][0],wall.geometry.path[1][0]),cuty=wall.geometry.path[0][1]+thickness/2;if(wall.geometry.path[0][1]!==wall.geometry.path[1][1]||!(minx<cutx&&cutx<maxx&&miny<cuty&&cuty<maxy))throw Error('Unexpected cellar interface');blindingOutline=[[minx,miny],[cutx,miny],[cutx,cuty],[maxx,cuty],[maxx,maxy],[minx,maxy]];}
  elements.push(footing(blinding,'MAT-LIME-CONCRETE',[{outline:blindingOutline,z0,z1}],'Lime-concrete blinding '+pad.id,'Kalkbeton-Sauberkeitsschicht '+pad.id,pad.id));
  const parts=[];
  for(let i=0;i<pad.columns.length;i++){
   const column=model.elements.find(e=>e.tag===pad.columns[i]);if(!column)throw Error('Missing column '+pad.columns[i]);
   if(column.geometry.sections)throw Error('Unexpected preexisting sections '+column.tag);
   const axis=column.geometry.axis,size=['2','4'].includes(column.tag)?.25:.22;
   const outline=box(pad.rc_pixels[i]);
   parts.push({outline,z0:z1,z1:z2},{outline,top_outline:square(axis,size),z0:z2,z1:z3});
   const profile=model.types.find(t=>t.tag===column.type).profile;
   column_updates.push({tag:column.tag,geometry:{...column.geometry,base_level:z3,sections:[{profile:{shape:'rectangle',width:q(size),depth:q(size)},axis,base_level:z3,top_level:d.stem_top},{profile,axis,base_level:d.stem_top,top_level:column.geometry.top_level}]}});
  }
  elements.push(footing(tag,'MAT-RC',parts,'Reinforced pad '+pad.id,'Stahlbeton-Einzelfundament '+pad.id,pad.id));
  relations.push(relation('REL-FOUNDATION-BLINDING-'+pad.id,blinding,[tag],'Blinding supports reinforced pad','Sauberkeitsschicht trägt Stahlbetonfundament'),relation('REL-FOUNDATION-SUPPORT-'+pad.id,tag,pad.columns,'Pad supports numbered columns','Einzelfundament trägt nummerierte Stützen'));
 }
 return {elements,relations,column_updates,input};
}
export async function publish(){const delta=await prepare();const model=JSON.parse(await fs.readFile(modelPath,'utf8'));for(const e of delta.elements){if(model.elements.some(x=>x.tag===e.tag))throw Error('Existing foundation '+e.tag);model.elements.push(e);}for(const u of delta.column_updates)model.elements.find(e=>e.tag===u.tag).geometry=u.geometry;model.relations.push(...delta.relations);model.relations.find(e=>e.tag==='REL-CON-ground').related.push(...delta.elements.map(e=>e.tag));const stage=ticket+'/foundation-publish-stage.json';await fs.writeFile(stage,JSON.stringify(model,null,2)+'\n');await fs.rename(stage,modelPath);return {elements:model.elements.length,added:delta.elements.length,columns:delta.column_updates.length};}
