/** 🛢️ Authors the pierced pressure shell, insulation quarters and domed ABS top from the service-pipe positions. */
import Module from 'manifold-3d'
import {readFileSync,writeFileSync} from 'node:fs'
const kit=await Module();kit.setup();const M=kit.Manifold
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,f=a.volumes['50'],b=Object.fromEntries(f.body.map(q=>[q.id,q]))
const tc=[...Array.from({length:5},(_,i)=>b['stub-'+i]),b['hot-out']]
function head(s,n){
 const hc=-Math.sqrt(432**2-192**2),phi=Math.atan2(-hc,192),p=[]
 for(let i=0;i<=n;i++){const t=phi*i/n;p.push([192+(48-s)*Math.cos(t),(48-s)*Math.sin(t)])}
 for(let i=1;i<=n;i++){const t=phi+(Math.PI/2-phi)*i/n;p.push([(480-s)*Math.cos(t),hc+(480-s)*Math.sin(t)])}
 p[p.length-1][0]=0;return p
}
function coords(q,n){const ret=[];for(let i=0;i<q.length;i+=3)ret.push([q[i],q[i+1]-n,q[i+2]].map(x=>Math.round(x*1e4)/1e7));return ret}
function mesh(m,id,bone,material,origin,tone){
 if(m.isEmpty()||m.status()!=='NoError')throw Error(id+' '+m.status())
 const g=m.getMesh(),p=g.vertProperties,t=g.triVerts,tri=[]
 for(let i=0;i<t.length;i+=3)tri.push([t[i],t[i+1],t[i+2]])
 return {id,bone,material,role:'body',tone,kind:'polyhedron',vertices:coords(p,origin),triangles:tri}
}
const surface=(p,n)=>M.revolve(p,n).rotate([-90,0,0])
for(const [d,n,k]of [[50,48,6],[100,40,4],[200,32,3],[500,24,2]]){
 const a0=head(0,k),a1=head(3,k),tank=s=>surface([...s.toReversed().map(([r,y])=>[r,200-y]),...s.map(([r,y])=>[r,1200+y])],n)
 const outside=tank(a0),inside=tank(a1),skin=outside.subtract(inside)
 const cuts=tc.map((q,i)=>i===5?M.cylinder(320,15,15,n/2).rotate([-90,0,0]).translate([203.9,1200,0]):M.cylinder(130,12,12,n/2).rotate([0,q.axis[0]*90,0]).translate([q.axis[0]*220,q.origin[1]*1000,0]))
 const drill=M.union(cuts),bodies=[],jacket=M.cylinder(1320,300,300,n).rotate([-90,0,0]).translate([0,60,0]).subtract(outside).subtract(drill)
 const pressure=skin.subtract(drill)
 for(let j=0;j<4;j++){
  const base=60+j*330,idx=4-j,bone='zone-'+idx,zone=M.cube([700,330,700]).translate([-350,base,-350])
  bodies.push(mesh(jacket.intersect(zone),bone,bone,'foam-jacket',base,.70))
  bodies.push(mesh(pressure.intersect(zone),'tank-'+idx,bone,'enamelled-steel',base,.50))
 }
 const ac=Math.max(5,k*2),ell=(ra,rb)=>Array.from({length:ac+1},(_,i)=>[ra*Math.cos(i*Math.PI/(2*ac)),1380+rb*Math.sin(i*Math.PI/(2*ac))])
 const coverLine=[...ell(300,100),...ell(297,97).reverse()]
 const cap=surface(coverLine,n).subtract(drill)
 bodies.push(mesh(cap,'cap','cap','cap-plastic',1380,.78))
 writeFileSync(process.argv[3]+'/cylinder-'+d+'.json',JSON.stringify(bodies))
}
