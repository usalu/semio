import loadManifold from "manifold-3d";
import { Matrix4 } from "three";
const root = "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit";
const law = await Bun.file(root + "/🔣️original-skew-box-intersection.json").json();
const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
function placed(shape) {
  const box = Manifold.cube(shape.size);
  const tilt = box.rotate([0,shape.angles[0]*180/Math.PI,0]);
  const pitch = tilt.rotate([shape.angles[1]*180/Math.PI,0,0]);
  const reflect = pitch.transform(new Matrix4().makeScale(shape.mirror?-1:1,1,1).toArray());
  const tool = reflect.translate(shape.origin);
  for(const item of [reflect,pitch,tilt,box]) item.delete();
  return tool;
}
const stock=placed({"origin":[0,0,0],"size":[4,4,4],"angles":[0,0],"mirror":false});
const tool=placed({"origin":[3.2870817239479524,1.4487743082534066,1.9182100493051244],"size":[1,1,1],"angles":[0.43,-0.31],"mirror":false});
const run=(a,b)=>{
  const result=a.add(b);
  const shells=result.decompose();
  const measured={volume:result.volume(),area:result.surfaceArea(),bounds:[result.boundingBox().min,result.boundingBox().max],euler:2-2*result.genus(),shells:shells.length};
  for(const body of [...shells,result])body.delete();
  return measured;
};
console.log(JSON.stringify({direct:run(stock,tool),reverse:run(tool,stock)}));
stock.delete();
tool.delete();
