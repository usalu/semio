/** 🧭 Runs the swing thermal rule under the external memory guard and records the existing product phase and cancellation checkpoints. */
import {readFileSync} from 'node:fs'
import {checkVolumes,measureVolumes} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
const mode=process.argv[3]??'thermal'
let recent=0,peak=0
const sample=(stage:string)=>{
 const now=Date.now()
 if(now-recent<1500&&stage==='checkpoint')return
 const memory=process.memoryUsage()
 peak=Math.max(peak,memory.rss)
 console.error('[DEBUG]',stage,Math.round(memory.rss/1048576)+' MiB RSS',Math.round(memory.heapUsed/1048576)+' MiB JS',Math.round(memory.arrayBuffers/1048576)+' MiB buffers')
 recent=now
}
const options={scale:50,...(mode==='thermal'?{rules:['volumes-thermal-coverage']}: {})}
const operation={progress:(_d,_t,phase)=>sample(phase?.en??'phase'),signal:{throwIfAborted:()=>sample('checkpoint')}}
const data=mode==='areas'?measureVolumes(a,{scale:50,rules:['volumes-thermal-coverage']},operation):checkVolumes(a,options,operation)
sample('complete');console.log(JSON.stringify({asset:a.id,mode,peakMiB:peak/1048576,data:mode==='areas'?data.filter(x=>x.name?.includes('outerAreaM2')):data.map(x=>x.message?.en)}))
