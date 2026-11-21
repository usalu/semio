/** 🖋️ Restate only the affected coarse print views of five mammals on the native limb and claw frames. */
import {readFileSync,writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {entryOf,meshesAt,poseState,catalogEntries} from './volumes-lib.ts'
const library=join(import.meta.dir,'r3-goat-draw2d.ts')
Bun.plugin({name:'read-only-contour-tools',setup(build){build.onLoad({filter:/r3-goat-draw2d\.ts$/},args=>{
 const original=readFileSync(args.path,'utf8'),cut=original.indexOf('const STROKES:')
 if(cut<0)throw Error('Missing isolated contour declarations')
 return {contents:original.slice(0,cut)+'\nexport {outline}',loader:'ts'}
})}})
const {outline}=await import(library)
const selected={'goat':['elevation','front'],'pig':['elevation','front'],'sheep':['elevation','front'],'roe-deer':['elevation','front','plan'],'labrador-dog':['front']}
const patch=(raw,block,value)=>{
 const i=raw.indexOf('{',raw.indexOf('"'+block+'":'));let depth=0,q=false,end=i
 for(;end<raw.length;end++){const c=raw[end];if(q){if(c==='\\')end++;else if(c==='"')q=false}else if(c==='"')q=true;else if(c==='{')depth++;else if(c==='}'&&--depth===0){end++;break}}
 return raw.slice(0,i)+JSON.stringify(value)+raw.slice(end)
}
for(const id of process.argv.slice(2)){
 const file=catalogEntries().find(x=>x.slug===id)!.path,raw=readFileSync(file,'utf8'),a=JSON.parse(raw).asset,r=a.representations['500'],v=a.volumes['500']
 const ms=meshesAt(a,500,v.body),byId=new Map(v.body.map((s,i)=>[s.id,ms[i]])),thermal=new Map(v.thermal.filter(s=>s.of).map(s=>[s.of,s.region]))
 for(const view of selected[id]){
  const original=r.geometry[view],groups=new Map(),byBone=new Map()
  for(const s of original){const l=byBone.get(s.bone)??[];l.push(s);byBone.set(s.bone,l)}
  for(const part of v.body){
   const existing=byBone.get(part.bone)??[];let target=existing[0]
   if(part.bone==='head'){
    if(part.id.startsWith('horn-'))target=original.find(s=>s.id===(part.id==='horn-n'?'horn':'horn-f'))??original.find(s=>s.id==='horn')
    else if(part.id.includes('antler'))target=original.find(s=>s.id===(part.id.startsWith('near-')?'near-antler':'far-antler'))
    else target=original.find(s=>s.id==='head')
   }
   const name=target?.id??(part.bone.startsWith('hoof-')?'foot-'+part.bone.slice(5):part.bone)
   const group=groups.get(name)??{name,shape:target,parts:[]};group.parts.push(part);groups.set(name,group)
  }
  const drawings=[]
  for(const {name,shape,parts} of groups.values()){
   const one=parts[0];const groupRegions=[...new Set(parts.map(p=>thermal.get(p.id)).filter(Boolean))]
   const s=shape?{...shape}:{id:name,bone:one.bone,role:'body',z:one.bone.endsWith('f')?1:4,tone:one.tone??.55}
   s.kind='poly';s.points=[[0,0],[.1,0],[0,.1]];s.closed=true;s.filled=true
   if(groupRegions.length){delete s.region;delete s.regions;if(groupRegions.length===1)s.region=groupRegions[0];else s.regions=groupRegions}
   drawings.push(s)
  }
  const probe={...a,representations:{...a.representations,'500':{...r,geometry:{...r.geometry,[view]:drawings}}}}
  const projected=poseState(probe,500)[view];const shapes=[]
  for(const s of drawings){
   const rings=projected.find(p=>p.shape.id===s.id)?.rings[0]
   if(!rings||rings.length<3)throw Error('Projection probe failed '+id+' '+view+' '+s.id+' '+s.bone)
   const o=rings[0],x=rings[1],y=rings[2]
   const ax=(x.x-o.x)*10,ay=(x.y-o.y)*10,bx=(y.x-o.x)*10,by=(y.y-o.y)*10,det=ax*by-ay*bx
   if(Math.abs(det)<1e-8)throw Error('Singular pose '+s.id)
   const local=([x,y])=>[+( (by*(x-o.x)-bx*(y-o.y))/det).toFixed(5),+((ax*(y-o.y)-ay*(x-o.x))/det).toFixed(5)]
   const targets=groups.get(s.id).parts.map(p=>byId.get(p.id));const boundary=outline(view,targets,.014)
   const pieces=boundary.rings.length?boundary.rings:[boundary.ring]
   for(const [j,path] of pieces.entries())if(path.length>=3)shapes.push({...s,id:j?s.id+'-part-'+(j+1):s.id,points:path.map(local),penMm:Math.max(.18,r.pens.body??0)})
  }
  r.geometry[view]=shapes.sort((a,b)=>a.z-b.z)
 }
 const res=patch(raw,'representations',a.representations)
 if(raw!==readFileSync(file,'utf8'))throw Error(id+' changed concurrently')
 writeFileSync(file,res);console.log('[DEBUG]',id,selected[id].join(','))
}
