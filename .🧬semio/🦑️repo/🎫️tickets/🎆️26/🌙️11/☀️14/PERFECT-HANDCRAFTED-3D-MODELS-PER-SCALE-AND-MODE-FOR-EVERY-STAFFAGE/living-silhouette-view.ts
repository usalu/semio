/** 🌳 Measure a single dense botanical view with the exact product masks, pose, resolution and acceptance limits. */
import {createHash} from 'node:crypto'
import {readFileSync,writeFileSync,renameSync} from 'node:fs'
import {join} from 'node:path'
import {catalogEntries} from './volumes-lib.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {collector,featureOf,VOLUME_LIMITS} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts'
import {measureBodyView,worldSolids,VIEWPOINTS} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
const [id,k,view]=process.argv.slice(2),d=Number(k) as 50|100|200|500
if(!VIEWPOINTS.includes(view))throw Error('Unknown viewpoint')
const path=process.argv[5]??catalogEntries().find(e=>e.slug===id)!.path,raw=readFileSync(path,'utf8'),hash=createHash('sha256').update(raw).digest('hex'),a=JSON.parse(raw).asset
const rule='volumes-silhouette',result=collector(rule),v=a.volumes[k],rep=a.representations[k],feature=featureOf(d)
if(rep.status==='authored'){
 const state=evaluateDrawing(a,{kind:'pose',pose:a.presentation.defaultPose},{kind:'scale',scale:{denominator:d}})
 const softRegions=new Set(a.thermal.regions.filter(r=>r.areaBasis==='exchange'||VOLUME_LIMITS.softRegionKinds.includes(r.kind)).map(r=>r.id))
 const soft=new Set(v.thermal.filter(s=>s.of&&softRegions.has(s.region)).map(s=>s.of))
 const body=worldSolids(a,state.bones,d)
 const m=measureBodyView(a,state,body,view,d,feature,soft),el=view==='elevation'
 const hard=(el?VOLUME_LIMITS.silhouetteP95FeaturesElevation:VOLUME_LIMITS.silhouetteP95FeaturesOther)*feature
 const check=(name,value,limit,bound)=>result.measure({scale:d,list:'body',name:name+':'+view,value,limit,bound})
 check('iou',m.iou,el?VOLUME_LIMITS.silhouetteIouElevation:VOLUME_LIMITS.silhouetteIouOther,'min')
 check('p95M',m.p95M,hard,'max')
 if(m.softExtentM>0)check('p95SoftM',m.p95SoftM,Math.max(hard,VOLUME_LIMITS.softMassExtentFraction*m.softExtentM),'max')
 check('hausdorffM',m.hausdorffM,Number.MAX_VALUE,'max')
 const failing=result.measurements.filter(x=>!x.ok)
 if(failing.length)result.fail('volumes.'+k+'.body','The '+view+' body outline exceeds its declared tolerance.','Der Körperumriss der Ansicht '+view+' überschreitet seine Toleranz.',{scale:d,list:'body',measurements:failing})
}
const dest=process.argv[6]??join(import.meta.dir,'🗑️generated/living-current',[id,'strict',k,view].join('-')+'.json')
const temp=dest+'.tmp';writeFileSync(temp,JSON.stringify({id,hash,scale:d,view,code:process.env.LIVING_AUDIT_CODE,rules:[rule],findings:result.findings,measurements:result.measurements}));renameSync(temp,dest)
console.log('[DEBUG]',id,k,view,result.findings.length)
