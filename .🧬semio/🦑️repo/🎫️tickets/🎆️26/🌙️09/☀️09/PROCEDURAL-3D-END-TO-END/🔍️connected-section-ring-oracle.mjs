/** 🕳️ Finds two-ring plane sections in the neutral skew-recess solids using Manifold. */
import loadManifold from "manifold-3d";
import {Matrix4} from "three";
import {readFileSync} from "node:fs";

const wasm=await loadManifold();
wasm.setup();
const fixture=JSON.parse(readFileSync(process.argv[2],"utf8"));
const signed=ring=>ring.reduce((sum,a,i)=>{
  const b=ring[(i+1)%ring.length];
  return sum+a[0]*b[1]-a[1]*b[0];
},0)*0.5;
function place(stock){
  const cube=wasm.Manifold.cube(stock.size);
  const tilt=cube.rotate([0,stock.angles[0]*180/Math.PI,0]);
  const pitch=tilt.rotate([stock.angles[1]*180/Math.PI,0,0]);
  const reflect=pitch.transform(new Matrix4().makeScale(stock.mirror?-1:1,1,1).toArray());
  const solid=reflect.translate(stock.origin);
  for(const part of [cube,tilt,pitch,reflect])part.delete();
  return solid;
}
for(const row of fixture.cases.filter(row=>row.kind==="recess")){
  const stock=place(row.stock),tool=place(row.tool),material=stock.subtract(tool);
  for(const [axis,turn] of [[0,[0,-90,0]],[1,[90,0,0]],[2,[0,0,0]]]){
    const moved=material.rotate(turn);
    const range=moved.boundingBox();
    const low=range.min[2],high=range.max[2];
    for(let index=1;index<40;index++){
      const plane=Number((low+(high-low)*index/40).toFixed(2));
      if(row.coincidentFloor?.normal[axis]===1&&row.coincidentFloor.origin[axis]===plane)continue;
      const region=moved.slice(plane);
      const rings=region.toPolygons().map(signed).sort((a,b)=>b-a);
      const area=region.area();
      region.delete();
      if(rings.length===2&&rings[1]<-0.1){
        const origin=[0,0,0],normal=[0,0,0];
        origin[axis]=plane;normal[axis]=1;
        console.log("[DEBUG] "+JSON.stringify({name:row.name,origin,normal,area,ringAreas:rings}));
        break;
      }
    }
    moved.delete();
  }
  for(const part of [material,tool,stock])part.delete();
}
