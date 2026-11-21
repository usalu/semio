import {readFileSync} from 'node:fs';
import {evaluate} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
import {worldFrameOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🦴️skeleton/🟦️.ts';
import {Vector3} from 'three';
const a=JSON.parse(readFileSync('🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/market-parasol/🔣️.json','utf8')).asset;
const ribs=['right','b','c','d','left','f','g','h'];const max=Array(8).fill(0);for(const select of [{kind:'clip',clip:'wind-flap'},{kind:'transition',from:'furled',to:'open-still',durationS:6}]){
max.fill(0);for(let i=0;i<=96;i++){const {bones}=evaluate(a,{...select,timeS:6*i/96});for(let j=0;j<8;j++){const u=index=>new Vector3(...worldFrameOf(bones['rib-'+ribs[index%8]].frame).u);max[j]=Math.max(max[j],u(j).angleTo(u(j+1))*180/Math.PI);}}console.log(select.kind,max.map(k=>Math.round(k*1000)/1000));}
