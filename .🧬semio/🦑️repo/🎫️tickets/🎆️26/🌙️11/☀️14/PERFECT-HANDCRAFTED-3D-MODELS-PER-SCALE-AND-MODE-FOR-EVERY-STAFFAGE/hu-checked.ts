/** 🧍 Checks one shipped document, named poses and its measured digital scales without retaining the entire catalog. */
import Ajv from 'ajv'
import {readFileSync,writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {createHash} from 'node:crypto'
import {STAFFAGE,GENERATED,catalogEntries} from './volumes-lib.ts'
import {validateAsset} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts'
import {loadEnvironments} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts'
import {checkVolumes} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const slug=process.argv[2],mode=process.argv[3]??'all',scale=Number(process.argv[4])||undefined
const entries=catalogEntries(),e=entries.find(e=>e.slug===slug&&e.category==='humans')!
const raw=readFileSync(e.path),document=JSON.parse(raw.toString()),a=document.asset
const digest=(source:Uint8Array)=>createHash('sha256').update(source).digest('hex')
const problems:string[]=[]
if(mode!=='thermal'){
 const schema=JSON.parse(readFileSync(join(STAFFAGE,'🧬️schema/🔣️.json'),'utf8'))
 const ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(schema)
 const conforms=ajv.compile({$ref:schema.$id+'#/$defs/AssetDocument'})
 if(!conforms(document))for(const issue of conforms.errors??[])problems.push(issue.instancePath+' '+issue.message)
 const assets=new Map([[a.id,a]])
 for(const rep of Object.values(a.representations))for(const alt of rep.alternatives??[]){
  if(alt.kind==='asset'&&!assets.has(alt.asset)){
   const ref=entries.find(e=>e.slug===alt.asset)
   if(ref)assets.set(alt.asset,JSON.parse(readFileSync(ref.path,'utf8')).asset)
  }
 }
 const environments=new Set((await loadEnvironments()).presets.map(p=>p.id))
 for(const issue of validateAsset(a,{assets,environments}))problems.push(issue.code+' '+issue.path+' '+issue.message.en)
 for(const pose of a.poses){
  const state=evaluateDrawing(a,{kind:'pose',pose:pose.id})
  for(const id of pose.contacts.filter(k=>/^(heel|ball|contact|wheel-contact|box-base|tip)-?([lr])?$/.test(k))){
   const y=state.anchors[id].at.y
   if(Math.abs(y)>.008)problems.push(pose.id+' '+id+' '+y+' m above support plane')
  }
 }
}
if(mode!=='document'){
 const settings={...(scale?{scale}:{}),...(mode==='thermal'?{rules:['volumes-thermal-coverage']}:mode==='structure'?{rules:['volumes-structure-coverage','volumes-feature','volumes-monotonic','volumes-solids','volumes-copied','volumes-slender','volumes-bones']}: {})}
 for(const issue of checkVolumes(a,settings))problems.push(issue.rule+' '+issue.scale+' '+issue.message.en)
}
const result={slug,mode,scale,sha256:digest(raw),current:digest(raw)===digest(readFileSync(e.path)),poses:a.poses.length,problems}
writeFileSync(join(GENERATED,'hu-review',slug+'-'+mode+(scale?'-'+scale:'')+'.json'),JSON.stringify(result))
console.log(JSON.stringify(result))
