/** 🍁 Locate the parts of the maple envelope that change between the fine and intermediate crown. */
import {readFileSync} from 'node:fs'
import {join} from 'node:path'
import {entryOf,frames,toWorld} from './volumes-lib.ts'
import {distanceField,Mask} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
const {asset}=entryOf('vegetation/japanese-maple'),fr=frames(asset),cell=.0075
const box={minX:-3,minY:-.1,maxX:3,maxY:4.65}
function convex(p){const q=p.sort((a,b)=>a.x-b.x||a.y-b.y),turn=(a,b,c)=>(b.x-a.x)*(c.y-a.y)-(b.y-a.y)*(c.x-a.x)
 function side(z){const o=[];for(const p of z){while(o.length>1&&turn(o[o.length-2],o[o.length-1],p)<=0)o.pop();o.push(p)}o.pop();return o}
 return [...side(q),...side(q.reverse())]
}
function render(items,axis){
 const m=new Mask(box,cell)
 for(const s of items){if(s.texture||s.slender||!s.vertices)continue
 const p=s.vertices.map(v=>toWorld(fr[s.bone],v)).map(v=>({x:v[axis],y:v[1]}));m.fill([convex(p)])
 }
 return m
}
function outline(m){const e=new Uint8Array(m.cells.length),{width:w}=m;for(let k=w;k<e.length-w;k++)if(m.cells[k]&&(!m.cells[k-1]||!m.cells[k+1]||!m.cells[k-w]||!m.cells[k+w]))e[k]=1;return e}
function score(a,b,detail){const ea=outline(a),eb=outline(b);const samples=[],groups={}
 for(const [src,to,sign] of [[ea,eb,-1],[eb,ea,1]]){const dist=distanceField(to,a.width,a.height)
  for(let i=0;i<src.length;i++)if(src[i]){const d=Math.sqrt(dist[i])*cell;samples.push(d)
   if(d<.105)continue
   const x=box.minX+(i%a.width+.5)*cell,y=box.minY+(Math.floor(i/a.width)+.5)*cell
   const key=[sign,x<0?'left':'right',y<1.25?'stems':y<1.9?'lower':y<2.6?'middle':y<3.45?'upper':'top'].join(' ')
   groups[key]=(groups[key]??0)+1
  }}
 samples.sort((a,b)=>a-b);return {p95:+samples[Math.floor(.95*samples.length)].toFixed(4),...(detail?{worst:Object.entries(groups).sort((a,b)=>b[1]-a[1]).slice(0,6)}:{})}
}
const fine=asset.volumes['50'].body,coarse=asset.volumes['100'].body
for(const axis of [0,2]){
 const a=render(fine,axis)
 for(const [id,extras] of Object.entries({none:[],odd:fine.filter(x=>/^t[123]-f(1|3|5|7|9|11)$/.test(x.id)),staggered:fine.filter(x=>/^t1-f(1|3|5|7|9|11)$|^t2-f(2|4|6|8|10)$|^t3-f(1|3|5|7)$/.test(x.id)),slopes:fine.filter(x=>/^t1-f(1|3|4|6|7|9|10|12)$|^t2-f(1|3|4|6|8|9)$|^t3-f(1|3|4|5|7)$/.test(x.id))})){
  console.log('[DEBUG]',axis,id,score(a,render([...coarse,...extras],axis),id==='none'))
 }
}
