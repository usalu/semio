/** 🕳️ Author source-informed cellar geometry from the archived 4.75m structural grid. */
import {readFile,writeFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
const root='C:/git/semio', base=root+'/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model';
const source=base+'/🔣️.json', schemaPath=base+'/🧬️schema/🔣️.schema.json';
const model=JSON.parse(await readFile(source,'utf8')), schema=JSON.parse(await readFile(schemaPath,'utf8'));
const alphabet='0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$';
function guid(tag){const hash=createHash('sha1').update(Buffer.from('6ba7b8119dad11d180b400c04fd430c8','hex')).update('semio:villa-savoye:'+tag).digest();hash[6]=(hash[6]&15)|80;hash[8]=(hash[8]&63)|128;let n=BigInt('0x'+hash.subarray(0,16).toString('hex')),text='';for(let i=0;i<22;i++){text=alphabet[Number(n%64n)]+text;n/=64n;}return text;}
const name=(en,de)=>({en,de}), q=(value,basis='inferred')=>({value,unit:'m',basis,source:'CSIC2022-2.1-9-Annex1'});
const p=([x,y])=>[+(19-(x-92)*19/734).toFixed(6),+(1.25+(y-225)*19/735).toFixed(6)];
const rect=(x0,y0,x1,y1)=>[[x0,y0],[x1,y0],[x1,y1],[x0,y1]];
const fromPixels=(x0,y0,x1,y1)=>{const a=p([x1,y0]),b=p([x0,y1]);return rect(a[0],a[1],b[0],b[1]);};
const bounds=fromPixels(392,396,652,828), inside=fromPixels(410,415,637,807), centre=p([585,535]);
const metadata=(domains=['architecture','structure'])=>({author:'Structural engineer',revision:model.revision,domains});
const added=[];
function element(tag,cls,predefined,en,de,geometry,material='MAT-RC',storey='cellar',attributes={}){const e={guid:guid(tag),tag,class:cls,predefined_type:predefined,name:name(en,de),storey,material,geometry,attributes,psets:{},metadata:metadata()};model.elements.push(e);added.push(e);return e;}
function relation(tag,cls,relating,related,en,de){model.relations.push({guid:guid(tag),tag,class:cls,relating,related,name:name(en,de)});}
function solid(tag,cls,predefined,en,de,parts,material='MAT-RC',storey='cellar'){return element(tag,cls,predefined,en,de,{kind:'cellar-construction',parts},material,storey);}
function slab(tag,en,de,outline,level,thickness,voids=[]){return element(tag,'IfcSlab','FLOOR',en,de,{outline,voids,level},'MAT-RC',level,{thickness:q(thickness,'sourced')});}
function wall(tag,a,b,thickness,en,de,material='MAT-STONE-LIME',top=-.07){const e=element(tag,'IfcWall','STANDARD',en,de,{path:[a,b],base_level:'cellar',top_level:top},material,'cellar',{thickness:q(thickness)});e.psets={Pset_WallCommon:{LoadBearing:{value:material==='MAT-STONE-LIME',ifc_type:'IfcBoolean',basis:'sourced',source:'CSIC2022-2.1'},IsExternal:{value:tag!=='W-CELLAR-DIVIDER',ifc_type:'IfcBoolean',basis:'inferred'}}};return e;}
model.classes.IfcFooting={supertype:'IfcBuiltElement',adds:['PredefinedType']};model.classes.IfcBeam={supertype:'IfcBuiltElement',adds:['PredefinedType']};
for(const [tag,en,de,density] of [['MAT-STONE-LIME','Exposed stone and lime masonry','Sichtbares Kalk-Bruchsteinmauerwerk',22],['MAT-LIME-CONCRETE','Lean lime concrete','Magerer Kalkbeton',20]]) model.materials.push({guid:guid(tag),tag,class:'IfcMaterial',name:name(en,de),density:{value:density,unit:'kN/m³',basis:'assumed'}});
model.storeys.unshift({guid:guid('cellar'),tag:'cellar',id:'cellar',class:'IfcBuildingStorey',name:name('Cellar','Keller'),elevation:q(-2.42,'sourced')});
model.relations.find(r=>r.class==='IfcRelAggregates'&&r.relating==='BUILDING').related.unshift('cellar');
slab('SL-CELLAR','Cellar concrete floor','Kellerbetonsohle',bounds,'cellar',.08);
const stairVoid=fromPixels(542,497,610,574);
slab('SL-GROUND-CELLAR','Ribbed slab over the cellar','Rippendecke über dem Keller',bounds,'ground',.07,[{tag:'V-CELLAR-STAIR',outline:stairVoid}]);
const [xmin,ymin]=bounds[0],[xmax,ymax]=bounds[2], t=.42;
wall('W-CELLAR-W',[xmin+t/2,ymin+t],[xmin+t/2,ymax-t],t,'Cellar west retaining masonry','Westliches Kellerstützmauerwerk');
wall('W-CELLAR-E',[xmax-t/2,ymin+t],[xmax-t/2,ymax-t],t,'Cellar east retaining masonry','Östliches Kellerstützmauerwerk');
wall('W-CELLAR-S',[xmin,ymin+t/2],[xmax,ymin+t/2],t,'Cellar south retaining masonry','Südliches Kellerstützmauerwerk');
wall('W-CELLAR-N',[xmin,ymax-t/2],[xmax,ymax-t/2],t,'Cellar north retaining masonry','Nördliches Kellerstützmauerwerk');
const div0=p([512,575]),div1=p([512,807]);wall('W-CELLAR-DIVIDER',div0,div1,.36,'Partial cellar dividing masonry','Teilweise Kellertrennmauer');
const stoneParts=[{outline:rect(xmin-.12,ymin-.12,xmax+.12,ymin+t+.12),z0:-2.72,z1:-2.42},{outline:rect(xmin-.12,ymax-t-.12,xmax+.12,ymax+.12),z0:-2.72,z1:-2.42},{outline:rect(xmin-.12,ymin+t+.12,xmin+t+.12,ymax-t-.12),z0:-2.72,z1:-2.42},{outline:rect(xmax-t-.12,ymin+t+.12,xmax+.12,ymax-t-.12),z0:-2.72,z1:-2.42},{outline:rect(div0[0]-.3,div0[1]-.1,div0[0]+.3,div1[1]+.1),z0:-2.72,z1:-2.42}];
solid('FT-CELLAR','IfcFooting','STRIP_FOOTING','Lime concrete cellar strip footings','Kalkbeton-Streifenfundamente des Kellers',stoneParts,'MAT-LIME-CONCRETE');
const ribs=[];for(const py of [494,574,650,729]){const y=p([512,py])[1];const z0=py===574?-.35:-.27;for(const [a,b] of [[xmin+t,div0[0]-.18],[div0[0]+.18,xmax-t]]){if(py===494 && b<centre[0]+1.0)continue;ribs.push({outline:rect(a,y-.08,b,y+.08),z0,z1:-.07});}}
solid('RIB-CELLAR','IfcBeam','BEAM','Cellar slab ribs and local deepening','Kellerdeckenrippen mit örtlicher Vertiefung',ribs,'MAT-RC','ground');
const sx0=Math.min(...stairVoid.map(v=>v[0])),sx1=Math.max(...stairVoid.map(v=>v[0])),sy0=stairVoid[0][1],sy1=stairVoid[2][1];
solid('BEAM-CELLAR-STAIR','IfcBeam','BEAM','Cellar stair opening trimmers','Wechsel des Kellertreppenlochs',[{outline:rect(sx0-.12,sy0-.12,sx1+.12,sy0),z0:-.25,z1:-.07},{outline:rect(sx0-.12,sy1,sx1+.12,sy1+.12),z0:-.25,z1:-.07},{outline:rect(sx0-.12,sy0,sx0,sy1),z0:-.27,z1:-.07},{outline:rect(sx1,sy0,sx1+.12,sy1),z0:-.25,z1:-.07}],'MAT-RC','ground');
for(const [tag,width,point,en,de] of [['CELLAR-C14',.22,p([512,496]),'Cellar column 14','Kellerstütze 14'],['STAIR-PA',.11,centre,'Basement stair auxiliary support Pa','Hilfsstütze Pa der Kellertreppe']]){const type='TY-'+tag;model.types.push({guid:guid(type),tag:type,class:'IfcColumnType',predefined_type:'COLUMN',name:name(en,de),material:'MAT-RC',profile:{shape:'rectangle',width:q(width,'sourced'),depth:q(width,'sourced')},attributes:{},psets:{Pset_ColumnCommon:{LoadBearing:{value:true,ifc_type:'IfcBoolean',basis:'sourced',source:'CSIC2022-2.2-9'}}},metadata:metadata(['structure'])});const e=element(tag,'IfcColumn','COLUMN',en,de,{axis:point,base_level:'cellar',top_level:-.07});delete e.material;e.type=type;relation('REL-TYPE-'+tag,'IfcRelDefinesByType',type,[tag],'Type defines cellar support','Typ definiert Kellerstütze');}
solid('FT-STAIR-PA','IfcFooting','PAD_FOOTING','Stair support 30cm square pad footing','30cm-Quadratfundament der Treppenstütze',[{outline:rect(centre[0]-.15,centre[1]-.15,centre[0]+.15,centre[1]+.15),z0:-2.72,z1:-2.42}]);
const arc=(r,a,b)=>Array.from({length:9},(_,i)=>{const t=(a+(b-a)*i/8)*Math.PI/180;return [+(centre[0]+r*Math.cos(t)).toFixed(6),+(centre[1]+r*Math.sin(t)).toFixed(6)];});
const steps=[];for(let i=0;i<12;i++)steps.push({outline:[...arc(.9,-90+i*15,-90+(i+1)*15),...arc(.055,-90+(i+1)*15,-90+i*15)],z0:+(-2.42+2.42*i/15-.08).toFixed(9),z1:+(-2.42+2.42*(i+1)/15).toFixed(9)});
for(let i=0;i<3;i++)steps.push({outline:rect(centre[0]-.25*(i+1),centre[1]+.055,centre[0]-.25*i,centre[1]+.9),z0:+(-2.42+2.42*(12+i)/15-.08).toFixed(9),z1:+(-2.42+2.42*(13+i)/15).toFixed(9)});
element('ST-CG','IfcStair','HALF_TURN_STAIR','Cellar curved service stair','Geschwungene Kellertreppe',{kind:'curved_straight',centre,radius:.9,inner_radius:.055,from_level:'cellar',to_level:'ground',step_count:15,start_angle:-90,turns:.5,step_parts:steps,landing_parts:[]});
wall('W-CELLAR-STAIR-E',[sx1+.06,sy0],[sx1+.06,sy1],.12,'Full height cellar stair enclosure','Raumhohe Kellertreppeneinfassung','MAT-BRICK',-.07);
wall('W-CELLAR-STAIR-S',[sx0,sy0-.06],[sx1,sy0-.06],.12,'Full height cellar stair enclosure','Raumhohe Kellertreppeneinfassung','MAT-BRICK',-.07);
wall('W-CELLAR-STAIR-N',[sx0,sy1+.06],[sx1,sy1+.06],.12,'Full height cellar stair enclosure','Raumhohe Kellertreppeneinfassung','MAT-BRICK',-.07);
const west=fromPixels(520,415,637,807),east=fromPixels(410,415,504,807);
for(const [tag,outline,en,de] of [['SP-CELLAR-W',west,'Cellar west bay','Westliche Kellerbucht'],['SP-CELLAR-E',east,'Cellar east bay','Östliche Kellerbucht']])model.spaces.push({guid:guid(tag),tag,class:'IfcSpace',predefined_type:'INTERNAL',name:name(en,de),storey:'cellar',outline,metadata:metadata(['architecture'])});
relation('REL-SP-cellar','IfcRelAggregates','cellar',['SP-CELLAR-W','SP-CELLAR-E'],'Cellar comprises two connected bays','Keller umfasst zwei verbundene Buchten');
for(const storey of ['cellar','ground']){const tags=added.filter(e=>e.storey===storey).map(e=>e.tag);const existing=model.relations.find(r=>r.class==='IfcRelContainedInSpatialStructure'&&r.relating===storey);if(existing)existing.related.push(...tags);else relation('REL-CON-'+storey,'IfcRelContainedInSpatialStructure',storey,tags,'Storey contains building elements','Geschoss enthält Bauteile');}
for(const material of new Set(added.filter(e=>e.material).map(e=>e.material))){const tags=added.filter(e=>e.material===material).map(e=>e.tag);const existing=model.relations.find(r=>r.class==='IfcRelAssociatesMaterial'&&r.relating===material);if(existing)existing.related.push(...tags);else relation('REL-MAT-'+material,'IfcRelAssociatesMaterial',material,tags,'Material is assigned','Material ist zugewiesen');}
for(const type of ['TY-CELLAR-C14','TY-STAIR-PA'])model.relations.find(r=>r.class==='IfcRelAssociatesMaterial'&&r.relating==='MAT-RC').related.push(type);
relation('REL-SUPPORT-CELLAR','IfcRelConnectsElements','FT-CELLAR',['W-CELLAR-W','W-CELLAR-E','W-CELLAR-N','W-CELLAR-S','W-CELLAR-DIVIDER','CELLAR-C14'],'Strip footings support cellar masonry','Streifenfundamente tragen Kellermauerwerk');
relation('REL-SUPPORT-STAIR-PA','IfcRelConnectsElements','FT-STAIR-PA',['STAIR-PA'],'Pad footing supports stair pilaster','Einzelfundament trägt Treppenstütze');
for(const spec of model.models)if(['architecture','structure'].includes(spec.id))spec.element_tags.push(...added.filter(e=>e.metadata.domains.includes(spec.id)).map(e=>e.tag));
model.notes.push({id:'cellar-source',text:name('Cellar topology, exposed stone/lime masonry and 30cm strip-footing depth follow Zaparaín Hernández et al. 2022, DOI10.3989/ic.88625 §§2.1,9 and annex1. Plan coordinates are inferred from the 4.75m structural grid, rotated to the CMN orientation; the stair centre is (6.23842,9.26361)m. The two-bay partial basement, short curved/straight flight, 8cm ground slab, 11cm Pa support, 30cm Pa pad, 7cm ceiling slab and 16cm ribs are distinguished. Fifteen risers, masonry thickness, floor footprints, exact rib extents and trim positions remain inferred. Nominal cellar height is 2.40m in the text; floor -2.42m and footing -2.72m are section datums, while the modeled ceiling underside is -0.07m. Small side projections are omitted because their functions remain unresolved; no garage hoist hatch is invented.','Kellertopologie, sichtbares Kalk-Bruchsteinmauerwerk und 30cm Streifenfundamenthöhe folgen Zaparaín Hernández et al. 2022, DOI10.3989/ic.88625 §§2.1,9 und Anhang1. Plankoordinaten sind aus dem 4,75m-Tragwerksraster in CMN-Orientierung abgeleitet. Teilkeller mit zwei Buchten, kurzer gebogener/gerader Treppenlauf, 8cm Sohle, 11cm Pa-Stütze, 30cm Pa-Fundament, 7cm Deckenplatte und 16cm Rippen werden unterschieden. Fünfzehn Steigungen, Mauerstärken und genaue Rippenlagen bleiben abgeleitet. Die Textangabe 2,40m Kellerhöhe wird von den Schnittkoten -2,42m/-2,72m und der modellierten Deckenunterkante -0,07m unterschieden. Kleine seitliche Vorsprünge bleiben mangels Funktionsnachweis ausgenommen; keine erfundene Garagenhebeöffnung.')});
schema.properties.storeys.minItems=4;schema.properties.storeys.maxItems=4;schema.properties.storeys.items.properties.id.enum.unshift('cellar');schema.$defs.level.enum.unshift('cellar');
schema.$defs.geometry.properties.top_level.anyOf[0].pattern='^(cellar|ground|first|roof|overall|soffit:[A-Za-z0-9-]+)$';
schema.$defs.ifc_class.enum.push('IfcFooting','IfcBeam');schema.$defs.geometry.properties.step_parts={type:'array',minItems:1,items:{$ref:'#/$defs/solid_part'}};
schema.$defs.type.properties.profile={oneOf:[schema.$defs.type.properties.profile,{type:'object',additionalProperties:false,required:['shape','width','depth'],properties:{shape:{const:'rectangle'},width:{$ref:'#/$defs/positive'},depth:{$ref:'#/$defs/positive'}}}]};
schema.$defs.element.allOf[1].if.properties.class.enum.push('IfcFooting','IfcBeam');
await writeFile(schemaPath,JSON.stringify(schema,null,2)+'\n');await writeFile(source,JSON.stringify(model,null,2)+'\n');console.log({added:added.map(e=>e.tag),count:model.elements.length,centre,bounds});
