/** 🌿 Visit the exact solids of one scale as slices of its source document. */
function after(raw:string,i:number){
 let quote=false,depth=0
 for(;i<raw.length;i++){
  const c=raw[i]
  if(quote){if(c==='\\')i++;else if(c==='"')quote=false;continue}
  if(c==='"')quote=true
  else if(c==='['||c==='{')depth++
  else if(c===']'||c==='}')if(--depth===0)return i+1
 }
 throw Error('Unclosed volume')
}
export function sceneInput(raw:string){
 const start=raw.indexOf('{',raw.indexOf('"asset":'))
 const asset={}
 for(let i=start+1;i<raw.length;){
  while(/[\s,]/.test(raw[i]!))i++
  if(raw[i]==='}')break
  if(raw[i]!=='"')throw Error('Missing asset field')
  const end=raw.indexOf('"',i+1),key=raw.slice(i+1,end);i=raw.indexOf(':',end)+1
  while(/\s/.test(raw[i]!))i++
  let next=i
  if(raw[i]==='{'||raw[i]==='[')next=after(raw,i)
  else if(raw[i]==='"'){next=i+1;while(raw[next]!=='"'){if(raw[next]==='\\')next++;next++}next++}
  else{while(!/[\s,}]/.test(raw[next]!))next++}
  if(!['volumes','representations','symbols'].includes(key))asset[key]=JSON.parse(raw.slice(i,next))
  i=next
 }
 return asset
}
export function eachSolid(raw:string,scale:string){
 const start=raw.indexOf('{',raw.indexOf('"volumes":'))
 const begin=raw.indexOf('{',raw.indexOf('"'+scale+'":',start))
 if(begin<start)throw Error('Missing scale '+scale)
 const ranges={}
 let pos=begin
 for(const mode of ['body','thermal','structure']){
  const index=raw.indexOf('[',raw.indexOf('"'+mode+'":',pos))
  const end=after(raw,index),list=[]
  for(let i=index+1;i<end-1;){
   if(raw[i]!=='{'){i++;continue}
   const j=after(raw,i);list.push([i,j]);i=j
  }
  ranges[mode]=list;pos=end
 }
 const body=new Map(ranges.body.map(([i,j])=>{
  const id=/"id"\s*:\s*"([^"]+)"/.exec(raw.slice(i,Math.min(i+400,j)))?.[1];if(!id)throw Error('Missing solid ID near '+i);return [id,[i,j]]
 }))
 return function* visit(mode:string){
  for(const [i,j] of ranges[mode]){
   const s=JSON.parse(raw.slice(i,j))
   if(!s.of){yield s;continue}
   const [u,v]=body.get(s.of)??[]
   if(u===undefined)throw Error('Missing reference '+s.of)
   yield {...JSON.parse(raw.slice(u,v)),...s}
  }
 }
}
