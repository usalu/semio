/** 🖨️ Measure one independent viewpoint and named pose at an exact native print-band denominator. */
import {readFileSync} from 'node:fs'
import {DEFAULT_PRINT_PROFILE,measureSuitability} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts'
const [file,k,end,pose,view]=process.argv.slice(2),a=JSON.parse(readFileSync(file,'utf8')).asset,r=a.representations[k]
if(!a.poses.some(p=>p.id===pose&&!p.key)||!['front','plan','elevation'].includes(view))throw Error('Unknown print input')
for(const p of a.poses)if(p.id!==pose)p.key=true
r.geometry=Object.fromEntries(Object.entries(r.geometry).map(([key,shapes])=>[key,key===view?shapes:[]]))
const measured=measureSuitability(a,r,Number(end),DEFAULT_PRINT_PROFILE)
console.log(JSON.stringify({k,end,pose,view,suitable:measured.suitable,violations:measured.violations}))
process.exitCode=measured.suitable?0:1
