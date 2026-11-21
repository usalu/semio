import { readFileSync } from "node:fs";
import { Matrix4, Vector3 } from "three";
import loadManifold from "manifold-3d";

const fixture=JSON.parse(readFileSync(process.argv[2],"utf8"));
const row=fixture.cases.find(value=>value.name==="two authored tool supports form one skew open recess");
const wasm=await loadManifold();
wasm.setup();

function placed(shape) {
  const cube=wasm.Manifold.cube(shape.size);
  const tilted=cube.rotate([0,shape.angles[0]*180/Math.PI,0]);
  const pitched=tilted.rotate([shape.angles[1]*180/Math.PI,0,0]);
  const result=pitched.translate(shape.origin);
  for(const body of [cube,tilted,pitched])body.delete();
  return result;
}

const stock=placed(row.stock),tool=placed(row.tool),cut=stock.subtract(tool);
for (const normal of [[1,.67,-1.315],[1,.69,-1.13],[.27,.41,-.4],[1,-.63,-.26],[.323,1,-.913],[1,-.78,0],[1,-1.27,0],[1,.83,0]]) {
  const z=new Vector3(...normal).normalize();
  const x=new Vector3(0,0,1).cross(z).normalize();
  const y=z.clone().cross(x);
  const local=cut.transform(new Matrix4().makeBasis(x,y,z).invert().toArray());
  const section=local.slice(0);
  const polygons=section.toPolygons();
  const reflex=polygons.some(ring=>ring.some(([x,y],i)=>{
    const [a,b]=ring[(i+1)%ring.length], [c,d]=ring[(i+2)%ring.length];
    return (a-x)*(d-b)-(b-y)*(c-a)<-1e-7;
  }));
  const vertex=polygons.some(ring=>ring.some(([a,b])=>Math.hypot(a,b)<1e-7));
  console.log(JSON.stringify({normal,area:section.area(),rings:polygons.length,reflex,vertex}));
  section.delete();
  local.delete();
}
for(const shape of [cut,tool,stock])shape.delete();
