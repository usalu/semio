/** 🖨️ Exercise the revised duck at the preset scale and both ends of every authored print band. */
import assert from 'node:assert/strict'
import {readFileSync} from 'node:fs'
import {catalogEntries} from './volumes-lib.ts'
import {DEFAULT_PRINT_PROFILE,measureSuitability} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts'
const asset=JSON.parse(readFileSync(catalogEntries().find(x=>x.slug==='mallard-duck')!.path,'utf8')).asset
let count=0
for(const [k,rep] of Object.entries(asset.representations) as [string,any][]){
 if(rep.status!=='authored')continue
 const ds=new Set([Number(k),...Object.values(rep.validRange)])
 for(const d of ds){
  const result=measureSuitability(asset,rep,d as number,DEFAULT_PRINT_PROFILE)
  console.log('[DEBUG]',k,d,JSON.stringify(result.violations))
  count+=result.violations.length
 }
}
assert.equal(count,0)
