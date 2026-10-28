import initialize from 'manifold-3d';
import {BoxGeometry,REVISION} from 'three';
import {readFileSync,writeFileSync} from 'node:fs';
const root=import.meta.dir,base='C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/';
const rows=JSON.parse(readFileSync(base+'faceted-shell-cases.json','utf8')).cases,module=await initialize();module.setup();const results=[];
for(const row of rows){
 const vertices:number[][]=[],triangles:number[][]=[],points=new Map<string,number>();
 const shell=(low:number[],high:number[],reverse=false)=>{const g=new BoxGeometry(...high.map((v,i)=>v-low[i]) as [number,number,number]);const p=g.getAttribute('position'),ids=[];for(let i=0;i<p.count;i++){const v=[p.getX(i),p.getY(i),p.getZ(i)].map((v,k)=>v+(low[k]+high[k])/2),key=v.join(',');if(!points.has(key)){points.set(key,vertices.length);vertices.push(v);}ids.push(points.get(key)!);}const ix=g.index!;for(let i=0;i<ix.count;i+=3){const a=ids[ix.getX(i)],b=ids[ix.getX(i+1)],c=ids[ix.getX(i+2)];triangles.push(reverse?[a,c,b]:[a,b,c]);}g.dispose();};
 shell([0,0,0],[2,1,3]);if(row.change==='join'){const d=row.offset;shell(d,[d[0]+2,d[1]+1,d[2]+3]);}
 if(row.change==='voids')for(const v of row.voids){const x=v.outer.map(p=>p[0]),y=v.outer.map(p=>p[1]);shell([Math.min(...x),Math.min(...y),v.base],[Math.max(...x),Math.max(...y),v.base+v.height],v.reverse);}
 if(row.change==='open')triangles.splice(0,2);if(row.change==='duplicate')triangles.push(...triangles.slice(0,2));
 let status,volume;try{const m=new module.Manifold(new module.Mesh({numProp:3,vertProperties:new Float32Array(vertices.flat()),triVerts:new Uint32Array(triangles.flat())}));try{status=m.status();volume=m.volume();}finally{m.delete();}}catch(error){status=String(error);}
 results.push({id:row.id,status,volume,scope:'Manifold generic triangle-soup observation; not IFC declared-shell or void-role validity'});
}
writeFileSync(root+'/🗑️generated/faceted-shell-independent/manifold.json',JSON.stringify({three:REVISION,rows:results},null,2));console.log('[DEBUG] '+JSON.stringify(results));
