/** 🌿 Read the exact paper drawings and rig, omitting the three mesh arrays unused by the paper extent rule. */
export function paperInput(raw:string){
 const marker=raw.indexOf('"volumes":')
 if(marker<0)throw Error('Missing volumes')
 const start=raw.indexOf('{',marker)
 const after=(i:number)=>{
  let quote=false,depth=0
  for(;i<raw.length;i++){
   const c=raw[i]
   if(quote){if(c==='\\')i++;else if(c==='"')quote=false;continue}
   if(c==='"')quote=true
   else if(c==='['||c==='{')depth++
   else if(c===']'||c==='}')if(--depth===0)return i+1
  }
  throw Error('Unclosed mesh arrays')
 }
 const end=after(start)
 const parts:string[]=[]
 let last=0
 const pattern=/"(?:body|thermal|structure)"\s*:\s*\[/g
 pattern.lastIndex=start
 for(let m=pattern.exec(raw);m&&m.index<end;m=pattern.exec(raw)){
  const array=raw.indexOf('[',m.index)
  parts.push(raw.slice(last,array),'[]')
  last=after(array);pattern.lastIndex=last
 }
 parts.push(raw.slice(last))
 return JSON.parse(parts.join('')).asset
}
