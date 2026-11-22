/** 🌿 Visit exact solids and resolve their body references without retaining the full document in memory. */
const whitespace=(c:string)=>c===' '||c==='\n'||c==='\t'||c==='\r'
function endOf(raw:string,i:number){
 const first=raw[i]
 if(first==='"'){
  for(let j=i+1;j<raw.length;j++){if(raw[j]==='\\')j++;else if(raw[j]==='"')return j+1}
 }else if(first==='['||first==='{'){
  let depth=1
  for(let j=i+1;j<raw.length;j++){
   const c=raw[j]
   if(c==='"')j=endOf(raw,j)-1
   else if(c==='['||c==='{')depth++
   else if((c===']'||c==='}')&&--depth===0)return j+1
  }
 }else{
  let j=i;while(j<raw.length&&!whitespace(raw[j])&&!',]}'.includes(raw[j]))j++
  if(j>i)return j
 }
 throw Error('Invalid JSON value at '+i)
}
function* fields(raw:string,start:number){
 if(raw[start]!=='{')throw Error('Expected an object at '+start)
 for(let i=start+1;i<raw.length;){
  while(whitespace(raw[i])||raw[i]===',')i++
  if(raw[i]==='}')return
  const end=endOf(raw,i),key=JSON.parse(raw.slice(i,end));i=end
  while(whitespace(raw[i]))i++
  if(raw[i++]!==':')throw Error('Missing colon after '+key)
  while(whitespace(raw[i]))i++
  const stop=endOf(raw,i);yield {key,start:i,stop};i=stop
 }
}
function field(raw:string,object:number,key:string){
 for(const item of fields(raw,object))if(item.key===key)return item
 throw Error('Missing field '+key)
}
export function sceneInput(raw:string){
 const object=field(raw,raw.indexOf('{'),'asset'),asset={}
 for(const {key,start,stop} of fields(raw,object.start))if(!['volumes','representations','symbols'].includes(key))asset[key]=JSON.parse(raw.slice(start,stop))
 return asset
}
export function eachSolid(raw:string,scale:string){
 const asset=field(raw,raw.indexOf('{'),'asset'),volumes=field(raw,asset.start,'volumes'),volume=field(raw,volumes.start,scale)
 const ranges=new Map<string,[number,number][]>(),body=new Map<string,[number,number]>()
 for(const {key,start,stop} of fields(raw,volume.start)){
  if(!['body','thermal','structure'].includes(key))continue
  const list:[number,number][]=[]
  for(let i=start+1;i<stop-1;){
   if(raw[i]!=='{'){i++;continue}
   const end=endOf(raw,i),span:[number,number]=[i,end];list.push(span)
   if(key==='body'){
    const id=field(raw,i,'id')
    body.set(JSON.parse(raw.slice(id.start,id.stop)),span)
   }
   i=end
  }
  ranges.set(key,list)
 }
 return function* visit(mode:string){
  const list=ranges.get(mode)
  if(!list)throw Error('Missing volume list '+mode)
  for(const [i,j] of list){
   const s=JSON.parse(raw.slice(i,j))
   if(!s.of){yield s;continue}
   const source=body.get(s.of)
   if(!source)throw Error('Missing reference '+s.of)
   yield {...JSON.parse(raw.slice(source[0],source[1])),...s}
  }
 }
}
