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
for (const [index,row] of law.convexCases.entries()) {
  const a=placed(row.stock);
  const b=placed(row.tool);
  const result=a.intersect(b);
  const z=index===0?2:0.5;
  const cut=result.slice(z);
  console.log(JSON.stringify({origin:[0,0,z],normal:[0,0,1],area:cut.area(),polygons:cut.toPolygons()}));
  for(const item of [cut,result,b,a]) item.delete();
}

const difference = await Bun.file(root + "/🔣️original-skew-box-difference.json").json();
for (const row of difference.cases.filter(row => row.kind === "recess" || row.kind === "cavity")) {
  const stock = placed(row.stock);
  const tool = placed(row.tool);
  const recess = stock.subtract(tool);
  const section = row.section ?? { origin: [0,0,row.kind === "cavity" ? 2 : 1.3], normal: [0,0,1] };
  const cut = recess.slice(section.origin[2]);
  const polygons = cut.toPolygons();
  console.log(JSON.stringify({ ...section, area: cut.area(), polygons }));
  if (row.section && (Math.abs(cut.area()-section.area)>1e-6 || polygons.length!==1)) {
    throw Error("Independent recess section disagrees with "+row.name);
  }
  for (const item of [cut,recess,tool,stock]) item.delete();
}
