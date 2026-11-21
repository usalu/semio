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
for (const [family, rows] of [["cases",law.cases],["convexCases",law.convexCases],["empty",law.empty]]) {
  for (const [index,row] of rows.entries()) {
    const stock = placed(row.stock);
    const tool = placed(row.tool);
    const actual = stock.minGap(tool,100);
    const reverse = tool.minGap(stock,100);
    console.log(JSON.stringify({family,index,name:row.name,distance:actual,reverse}));
    if (Math.abs(actual-reverse)>1e-6 || row.gap!==undefined&&Math.abs(actual-row.gap)>1e-6) throw Error("Inconsistent clearance for "+row.name);
    tool.delete();
    stock.delete();
  }
}
