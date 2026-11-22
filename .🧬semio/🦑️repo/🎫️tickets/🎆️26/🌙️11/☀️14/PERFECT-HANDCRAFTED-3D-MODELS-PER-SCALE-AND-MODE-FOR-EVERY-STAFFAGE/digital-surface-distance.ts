/** 📐 Measure closest points on posed triangular equipment and occupant surfaces. */
import {Vector3,Triangle,Box3,Ray} from 'three'
export function geometry(q,m,s){const g=m(s,q),points=Array.from({length:g.world.length/3},(_,i)=>new Vector3().fromArray(g.world,3*i)),faces=[];for(let i=0;i<g.indices.length;i+=3){const t=new Triangle(points[g.indices[i]],points[g.indices[i+1]],points[g.indices[i+2]]);faces.push([t,new Box3().setFromPoints([t.a,t.b,t.c])])}return {points,faces}}
export function surface(A,B){let min=Infinity;for(const aa of A)for(const bb of B)for(const [P,T]of [[aa.points,bb.faces],[bb.points,aa.faces]])for(const p of P)for(const [tri,box]of T){if(box.distanceToPoint(p)>=min)continue;min=Math.min(min,tri.closestPointToPoint(p,new Vector3()).distanceTo(p))}return isFinite(min)?Math.round(min*1000):null}

export function intersects(A,B){
 const offset=new Vector3(),hit=new Vector3()
 for(const a of A)for(const b of B)for(const [p,P] of a.faces)for(const [q,Q] of b.faces){
  if(!P.intersectsBox(Q))continue
  for(const [edge,triangle] of [[p,q],[q,p]])for(const [u,v] of [[edge.a,edge.b],[edge.b,edge.c],[edge.c,edge.a]]){
   offset.subVectors(v,u);const length=offset.length();if(!length)continue
   const ray=new Ray(u,offset.clone().multiplyScalar(1/length))
   if(ray.intersectTriangle(triangle.a,triangle.b,triangle.c,false,hit)&&hit.distanceTo(u)<=length+1e-8)return true
  }
 }
 return false
}
