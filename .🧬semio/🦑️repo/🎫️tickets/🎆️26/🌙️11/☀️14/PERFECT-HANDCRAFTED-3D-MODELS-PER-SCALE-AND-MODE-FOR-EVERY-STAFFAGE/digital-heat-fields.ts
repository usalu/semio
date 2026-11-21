/** 🌡️ Solve every authored pose and check its region fields against an independent decimal radiation calculation. */
import Decimal from 'decimal.js';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync,mkdirSync,renameSync} from 'node:fs';
import {join} from 'node:path';
import {catalogEntries} from './volumes-lib.ts';
import {paperInput} from './living-paper-input.ts';
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts';
import {solveThermal} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts';
const config=JSON.parse(readFileSync(join(import.meta.dir,'digital-heat-fields.json'),'utf8')),c=config.constants;
const sigma=Decimal.acos(-1).pow(5).mul(2).mul(new Decimal(c.boltzmannJK).pow(4)).div(new Decimal(c.planckJs).pow(3).mul(15).mul(new Decimal(c.speedOfLightMS).pow(2)));
const dir=join(import.meta.dir,'🗑️generated','digital-climate');mkdirSync(dir,{recursive:true});
const signature=process.env.STAFFAGE_HEAT_SIGNATURE;
for(const e of catalogEntries().filter(e=>process.argv.slice(2).includes(e.slug))){
 let raw=readFileSync(e.path,'utf8');const hash=createHash('sha256').update(raw).digest('hex'),a=paperInput(raw);raw='';
 const problems=[];let climates=0,fields=0,maxEmissionError=0,minC=Infinity,maxC=-Infinity;
 const near=(measured,expected,where)=>{if(!Number.isFinite(measured)||Math.abs(measured-expected)>config.absoluteFluxToleranceWM2+config.relativeRadianceTolerance*Math.abs(expected))problems.push(where);};
 for(const p of a.poses){
  const state=evaluateDrawing(a,{kind:'pose',pose:p.id});
  for(const [climate,env] of [['authored',a.thermal.environment],['shade',{...a.thermal.environment,...config.shade}]]){
   const heat=solveThermal(state,env);climates++;
   for(const r of a.thermal.regions){
    const value=heat.regions[r.id],where=p.id+'/'+climate+'/'+r.id;fields++;
    if(!value){problems.push(where+' missing region');continue;}
    for(const [k,x] of Object.entries(value))if(typeof x==='number'&&!Number.isFinite(x))problems.push(where+' '+k);
    const id=r.layers.at(-1)?.material??r.material,material=a.materials.find(m=>m.id===id),temp=value.surfaceTempC;
    if(!material||!Number.isFinite(temp)||temp<=-273.15){problems.push(where+' surface');continue;}
    minC=Math.min(minC,temp);maxC=Math.max(maxC,temp);
    const emission=sigma.mul(material.emissivity).mul(new Decimal(temp).plus(c.icePointK).pow(4)).toNumber();
    maxEmissionError=Math.max(maxEmissionError,Math.abs(emission-value.emittedRadiantWM2));near(value.emittedRadiantWM2,emission,where+' emission');
    near(value.netRadiativeWM2,value.netRadiativeSkyWM2+value.netRadiativeGroundWM2+value.netRadiativeSurroundingsWM2,where+' sky/ground/surroundings');
    near(value.sensibleW,value.sensibleWM2*r.areaM2*value.exposedFraction,where+' sensible area');
    near(value.netRadiativeW,value.netRadiativeWM2*r.areaM2*value.exposedFraction,where+' radiative area');
   }
  }
 }
 const row={asset:e.slug,category:e.category,hash,signature,poses:a.poses.length,climates,fields,minC,maxC,maxEmissionError,findings:problems};
 const dest=join(dir,e.slug+'.json'),partial=dest+'.'+process.pid;writeFileSync(partial,JSON.stringify(row));renameSync(partial,dest);
 console.log(e.slug,climates,fields,problems.length);Bun.gc(true);
}
