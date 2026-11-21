import {readFileSync} from 'node:fs';
import {strict as assert} from 'node:assert';
import {Box3,Vector3} from 'three';
const {asset}=JSON.parse(readFileSync('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/market-parasol/🔣️.json','utf8'));
const f=JSON.parse(readFileSync(new URL('./digital-canopy-case.json',import.meta.url),'utf8'));
const near=(p,q)=>assert(Math.abs(p-q)<f.toleranceM,`${p} m versus ${q} m`);
const extent=s=>new Box3().setFromPoints(s.profile.flatMap(([z,x])=>[s.from,s.to].map(y=>new Vector3(x,y,z))));
assert.equal(asset.rig.radialFolds.length,f.panels);
for(const d of f.scaleDenominators){
 const v=asset.volumes[d],panels=v.body.filter(s=>s.id.startsWith('panel-'));
 assert.equal(panels.length,f.panels*f.pleatsPerGore*2);
 for(const panel of panels){
  const body=extent(panel);near(body.max.y-body.min.y,f.clothThicknessM);
  const side=panel.id.slice(6),faces=['th-underside-','th-panel-'].map(key=>v.thermal.find(s=>s.id===key+side));
  const [lower,upper]=faces.map(extent);near(lower.min.y,body.min.y);near(upper.max.y,body.max.y);near(lower.max.y,upper.min.y);
  assert(faces.every(s=>s.bone===panel.bone));assert.deepEqual(faces.map(s=>s.sectionFaces),[['+v'],['-v']]);
 }
}
console.log('Eight 0.5 mm sewn gores, three pleats each, and both conducting layers verified at all four digital scales.');
