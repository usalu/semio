/** 🗄️ Independently integrate the punched door faces and follow their three hinge axes in the product pose. */
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {join} from 'node:path';
import {ShapeUtils,Vector2,Vector3} from 'three';
import {evaluate} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
const spec=JSON.parse(readFileSync(join(import.meta.dir,'rack-case.json'),'utf8'));
const path=process.argv[2]??'🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/server-rack/🔣️.json';
const {asset}=JSON.parse(readFileSync(path,'utf8'));
const close=(number:number,want:number,epsilon=1e-6)=>assert.ok(Math.abs(number-want)<epsilon,`${number} expected ${want}`);
const area=(ring:number[][])=>Math.abs(ShapeUtils.area(ring.map(([u,v])=>new Vector2(u,v))));
const cut=(part:any)=>part.holes.reduce((a:number,ring:number[][])=>a+area(ring),0);
const ids=['door-field','rear-door-a','rear-door-b'];
const bones=['door-hinge','rear-door-a','rear-door-b'];
for(const [d,v] of Object.entries(asset.volumes) as [string,any][]){
  for(const [i,id] of ids.entries()){
    const part=v.body.find((s:any)=>s.id===id);
    assert.ok(part?.holes?.length,`opening in ${id} at ${d}`);
    close(part.to-part.from,spec.gaugeM);
    assert.equal(part.holes.length,spec.holes[d][i?1:0]);
    const width=i?spec.rearWidthM:spec.frontWidthM;
    close(cut(part)/(width*spec.doorHeightM),spec.freeFraction,.00008);
    const thermal=v.thermal.find((s:any)=>s.of===id);
    assert.equal(thermal?.region,i?'rear-doors':'door');
    assert.equal(thermal?.layer,i?'rear-sheet':'door-sheet');
    assert.equal(part.bone,i?id:'door');
    assert.ok(v.structure.some((s:any)=>s.member===part.bone && s.part==='member'));
  }
  const pads=v.structure.filter((s:any)=>s.anchor?.startsWith('wheel-'));
  assert.equal(pads.reduce((n:number,s:any)=>n+(s.repeat?.count??1),0),4);
  assert.ok(v.thermal.filter((s:any)=>s.layer==='cabinet-sheet'&&s.kind==='box').every((s:any)=>Math.min(...s.size)<=.00121));
  const roof=v.body.find((s:any)=>s.id==='roof');
  close(roof.to-roof.from,spec.cabinetGaugeM);
  assert.ok(roof.holes.length>=2);
}
const poses=['off','service-open'].map(id=>evaluate(asset,{kind:'pose',pose:id},{kind:'scale',scale:{denominator:500}}));
const vec=(obj:any)=>new Vector3(obj.x,obj.y,obj.z);
for(const [i,bone] of bones.entries()){
  const axis=new Vector3(...[spec.frontAxis,spec.rearLeftAxis,spec.rearRightAxis][i]);
  const rest=poses[0]!.bones[bone]!.frame,opened=poses[1]!.bones[bone]!.frame;
  assert.ok(rest && opened);
  close(vec(rest.o).distanceTo(axis),0);close(vec(opened.o).distanceTo(axis),0);
  const expect=vec(rest.v).applyAxisAngle(new Vector3(0,1,0),spec.serviceAngles[i]*Math.PI/180);
  close(expect.distanceTo(vec(opened.v)),0);
}
close(asset.rig.bones.reduce((n:number,b:any)=>n+(b.mass?.kg||0),0),spec.massKg);
console.log(JSON.stringify({scales:4,doorSheets:12,serviceAxes:3,grossOpenFraction:spec.freeFraction}));
