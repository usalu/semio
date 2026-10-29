/** 🧺 Publish the reviewed finite fixture arrangement through guarded fresh field merges. */
import fs from 'node:fs/promises';
import crypto from 'node:crypto';
const canonical=value=>{if(Array.isArray(value))return '['+value.map(canonical).join(',')+']';if(value&&typeof value==='object')return '{'+Object.keys(value).sort().map(k=>JSON.stringify(k)+':'+canonical(value[k])).join(',')+'}';return JSON.stringify(value);};
const digest=value=>crypto.createHash('sha256').update(canonical(JSON.parse(JSON.stringify(value)))).digest('hex');
const protectedDomain=model=>({elements:model.elements.filter(e=>['IfcSlab','IfcColumn','IfcBeam','IfcFooting'].includes(e.class)),types:model.types,materials:model.materials,links:model.links,relations:model.relations,models:model.models,identities:[...model.elements,...model.spaces,...(model.types??[])].map(e=>({tag:e.tag,guid:e.guid,class:e.class})),grid:model.grid,roles:model.roles,storeys:model.storeys});
export async function publish(root,ticket){
 const path=root+'/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json';
 const input=await fs.readFile(path,'utf8'),model=JSON.parse(input),before=JSON.parse(input);
 const fixture=JSON.parse(await fs.readFile(ticket+'/fixture-repair-r77.json','utf8'));
 const expected=JSON.parse(await fs.readFile(ticket+'/fixture-repair-r77-expected.json','utf8'));
 const find=tag=>model.elements.find(e=>e.tag===tag);
 if(digest(protectedDomain(model))!==expected.structural)throw Error('Protected structural domain changed since trial');
 for(const[tag,hash]of Object.entries(expected.geometry))if(digest(find(tag).geometry)!==hash)throw Error('Affected geometry changed: '+tag);
 for(const[key,value]of Object.entries(expected.opening))if(find('OP-D-GUEST').attributes[key].value!==value)throw Error('Guest opening station changed');
 const rectangle=([x0,y0,x1,y1])=>[[x0,y0],[x1,y0],[x1,y1],[x0,y1]];
 for(const tag of fixture.laundry.tags)for(const part of find(tag).geometry.parts)part.outline=part.outline.map(([x,y])=>[x,y+fixture.laundry.translation[1]]);
 const g=fixture.guest,[x0,y0,x1,y1]=g.new_bounds,t=g.rim;
 const bounds=[[x0,y0,x1,y1],[x0,y0,x1,y0+t],[x0,y1-t,x1,y1],[x0,y0+t,x0+t,y1-t],[x1-t,y0+t,x1,y1-t]];
 find(g.basin).geometry.parts=find(g.basin).geometry.parts.map((p,i)=>({...p,outline:rectangle(bounds[i])}));
 find(g.tap).geometry.parts=find(g.tap).geometry.parts.map((p,i)=>({...p,outline:rectangle(i?g.tap_spout_bounds:g.tap_stem_bounds)}));
 for(const key of ['start','end'])Object.assign(find(g.opening).attributes[key],{value:expected.opening[key]+g.station_delta,basis:'inferred',source:'CMN2021-R77-inferred-guest-arrangement'});
 const notes={
  laundry:{en:'R77 inferred rear-wall arrangement: twin troughs and taps shifted north0.50m within the independently registered laundry room, clearing column32 without cutting structural material. Hollow bowls, bottoms, support legs and all sizes/heights are retained. Fixture positions, dimensions and the25mm rear-wall gap remain unmeasured teaching reconstruction; CMN §7 attests the twin-trough programme.',de:'R77 abgeleitete rückwärtige Anordnung: Doppeltröge und Armaturen innerhalb der registrierten Waschküche um0,50m nach Norden verschoben; Stütze32 bleibt unverändert und frei. Hohle Becken, Böden, Beine sowie Maße und Höhen bleiben erhalten. Positionen, Maße und25mm Wandabstand sind ungemessene Lehrrekonstruktion; CMN §7 belegt das Doppeltröge-Programm.'},
  guest:{en:'R77 inferred coordinated guest washroom arrangement:0.50m square hollow basin centred3.225/11.175 below the independently registered0.55m clear zenith-light opening (approximately±0.15m registration uncertainty); south-rim tap discharges north. Guest door stations shift0.50m north, retaining0.85m clear width/head and identities, to preserve an assumed0.80m open-leaf body approach. Basin size, tap form, door stations and standing area are unmeasured reconstruction; CMN §12 supports the washroom/zenith-light programme, not these metric dimensions. No closed-door or code-compliance clearance is claimed.',de:'R77 abgeleitete koordinierte Gäste-Waschanordnung: hohles0,50m-Quadratbecken mit Mitte3,225/11,175 unter der unabhängig registrierten0,55m-Lichtöffnung (etwa±0,15m Registrierungsunsicherheit); Armatur am Südrand mit Auslauf nach Norden. Türstationen um0,50m nach Norden verschoben;0,85m lichte Breite, Höhe und Identitäten bleiben erhalten, mit angenommenem0,80m-Körperweg bei geöffnetem Türblatt. Beckengröße, Armatur, Türstationen und Standfläche sind ungemessene Rekonstruktion; CMN §12 belegt Waschraum und Oberlicht, nicht diese Maße. Kein Nachweis bei geschlossenem Türblatt oder nach Baurecht.'}
 };
 const tags=[...fixture.laundry.tags,g.basin,g.tap,g.opening,g.door];
 for(const tag of tags){
  const obj=find(tag),note=fixture.laundry.tags.includes(tag)?notes.laundry:notes.guest;
  const evidence=obj.historical_evidence??{state:model.historical_state.id,status:'inferred_reconstruction',source_kind:'mixed',sources:[]};
  evidence.sources=[...new Set([...evidence.sources,'CMN2021',...(fixture.laundry.tags.includes(tag)?['ground-registration-r31','ground-registration-audit-r32']:['CSIC2022 Annex3 guest clear-opening registration R27'])])];
  evidence.uncertainty=Object.fromEntries(['en','de'].map(lang=>[lang,[evidence.uncertainty?.[lang],note[lang]].filter(Boolean).join(' ')]));
  obj.historical_evidence=evidence;
  const quantity=obj.psets?.ArchitecturalEvidence?.Geometry;
  if(quantity)Object.assign(quantity,{value:note.en,basis:'inferred',source:'CMN2021'});
 }
 if(digest(protectedDomain(model))!==expected.structural)throw Error('Protected domain would change');
 const changes=[];
 function diff(a,b,p=''){
  if(JSON.stringify(a)===JSON.stringify(b))return;
  if(a&&b&&typeof a==='object'&&typeof b==='object'&&Array.isArray(a)===Array.isArray(b)){for(const key of new Set([...Object.keys(a),...Object.keys(b)]))diff(a[key],b[key],p+'/'+key);}
  else changes.push({pointer:p,before:a===undefined?'[absent]':a,after:b===undefined?'[absent]':b});
 }
 diff(before,model);
 for(const change of changes){const match=change.pointer.match(/^\/elements\/(\d+)\//);if(!match||!tags.includes(model.elements[Number(match[1])].tag))throw Error('Unexpected changed pointer '+change.pointer);}
 if(await fs.readFile(path,'utf8')!==input)throw Error('Concurrent source change during guarded merge');
 const output=JSON.stringify(model,null,2)+'\n';await fs.writeFile(path,output);
 const result={source_before:crypto.createHash('sha256').update(input).digest('hex'),source_after:crypto.createHash('sha256').update(output).digest('hex'),protected_before:expected.structural,protected_after:digest(protectedDomain(model)),changed_tags:tags,geometry_part_counts:Object.fromEntries(Object.keys(expected.geometry).map(tag=>[tag,find(tag).geometry.parts.length])),changes};
 await fs.writeFile(ticket+'/🗑️generated/fixture-repair-r77-publication.json',JSON.stringify(result,null,2)+'\n');return result;
}
