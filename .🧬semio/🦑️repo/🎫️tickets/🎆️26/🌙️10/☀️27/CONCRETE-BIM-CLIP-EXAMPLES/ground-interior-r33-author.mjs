/** 🏠 Apply registered ground records while preserving concurrently authored source records. */
import {readFile,writeFile,open} from 'node:fs/promises';
import {dirname,resolve,join} from 'node:path';
import {fileURLToPath} from 'node:url';
const ticket=dirname(fileURLToPath(import.meta.url));
const root=resolve(ticket,'../../../../../../..');
const modelPath=join(root,'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json');
const contractPath=join(root,'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🏠ground.json');
const contract=JSON.parse(await readFile(contractPath,'utf8'));
const before=await readFile(modelPath,'utf8');
const source=JSON.parse(before);
const count=source.elements.length;
const basinX=Math.min(...source.elements.find(e=>e.tag==='FIX-HALL-BASIN').geometry.parts.flatMap(p=>p.outline).map(p=>p[0]));
if(Math.abs(basinX-6.2)>1e-8) throw Error('One-time R33 baseline already changed; do not reapply fixture translations');
const tags=[];
const evidence={state:'original-completed-1930-1931',status:'inferred_reconstruction',sources:['CMN2021','ground-registration-r31','ground-registration-audit-r32'],uncertainty:{en:'Registered broad topology from contemporary visitor-plan pixels; ±0.20 m coordinate uncertainty. Original-completed target is an inferred reconstruction; original fixture sizes, room subdivisions and opening construction are unmeasured. Older FLC19414 is a separate design-phase check, not a measurement of this state.',de:'Registrierte Grobtopologie aus dem heutigen Besucherplan; ±0,20 m Koordinatenunsicherheit. Ursprünglicher Fertigzustand als abgeleitete Rekonstruktion; ursprüngliche Ausstattungsmaße, Raumteilungen und Öffnungsaufbauten sind ungemessen. Älteres FLC19414 bleibt eine getrennte Entwurfsprüfung.'},source_kind:'mixed'};
for(const [tag,path] of Object.entries(contract.walls)){
 const element=source.elements.find(e=>e.tag===tag);
 element.geometry.path=path;
 element.historical_evidence=structuredClone(evidence);
 tags.push(tag);
}
for(const [tag,range] of Object.entries(contract.openings)){
 const element=source.elements.find(e=>e.tag===tag);
 for(const [i,key] of ['start','end'].entries()) element.attributes[key]={value:range[i],unit:'m',basis:'inferred',source:'CMN2021-registered-ground-r33'};
 element.historical_evidence=structuredClone(evidence);
 tags.push(tag);
}
for(const [tag,delta] of Object.entries(contract.fixture_translations)){
 const element=source.elements.find(e=>e.tag===tag);
 for(const part of element.geometry.parts) for(const key of ['outline','top_outline']) if(part[key]) part[key]=part[key].map(p=>p.map((v,i)=>v+(delta[i]??0)));
 element.historical_evidence=structuredClone(evidence);
 tags.push(tag);
}
for(const [tag,outline] of Object.entries(contract.spaces)){
 const space=source.spaces.find(e=>e.tag===tag);
 space.outline=outline;
 space.historical_evidence=structuredClone(evidence);
 tags.push(tag);
}
source.spaces.find(s=>s.tag==='SP-GARAGE').name={en:'Garage; historical three-car capacity separately assessed',de:'Garage; historische Dreierkapazität getrennt geprüft'};
source.spaces.find(s=>s.tag==='SP-BOILER').name={en:'Ground service cupboard; no heating plant assigned',de:'Erdgeschoss-Servicekammer; keine Heizungsanlage zugeordnet'};
source.elements.find(e=>e.tag==='OP-GARAGE').name={en:'Registered continuous garage portal',de:'Registriertes durchgängiges Garagenportal'};
if(source.elements.length!==count) throw Error('Ground interior edit changed element count');
if(await readFile(modelPath,'utf8')!==before) throw Error('Source changed during preparation; retry fresh');
const result=JSON.stringify(source,null,2)+'\n';
const handle=await open(modelPath,'r+');
try {await handle.writeFile(Buffer.concat([Buffer.from(result),Buffer.alloc(Math.max(0,Buffer.byteLength(before)-Buffer.byteLength(result)),32)]));} finally {await handle.close();}
await writeFile(join(ticket,'🗑️generated','ground-interior-r33-delta.json'),JSON.stringify({elements_before:count,elements_after:source.elements.length,changed:tags},null,2));
export const changed=tags;
