/** 🧾 Compare streaming views with the complete JSON document in arbitrary legal property order. */
import assert from 'node:assert/strict'
import {eachSolid,sceneInput} from './living-volume-input.ts'
const solid={metadata:{id:'nested'},positions:Array.from({length:301},(_,i)=>i),bone:'spine',id:'brush'},ref={id:'guard',of:'brush',region:'coat'}
const data={schema:'scene',asset:{volumes:{50:{structure:[],thermal:[ref],body:[solid]}},id:'squirrel',representations:{},symbols:{},title:{en:'"asset": { in a label'}}}
const raw=JSON.stringify(data,null,2).replace('"asset":','"asset" :')
assert.deepEqual(sceneInput(raw),{id:'squirrel',title:data.asset.title});assert.deepEqual([...eachSolid(raw,'50')('thermal')],[{...solid,...ref}])
