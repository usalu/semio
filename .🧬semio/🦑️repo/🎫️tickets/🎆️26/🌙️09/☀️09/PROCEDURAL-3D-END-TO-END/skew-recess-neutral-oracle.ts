import loadManifold from "manifold-3d";
const wasm = await loadManifold();
wasm.setup();
const { Manifold } = wasm;
const stock = Manifold.cube([4, 4, 4]);
const angles = [0.37, 0.21];
const [s,c,sp,cp] = [Math.sin(angles[0]),Math.cos(angles[0]),Math.sin(angles[1]),Math.cos(angles[1])];
const toolOrigin = [2-8*s, 2+8*sp*c, 2-8*cp*c];
const box = Manifold.cube([8,8,16]);
const tilt = box.rotate([0,angles[0]*180/Math.PI,0]);
const pitch = tilt.rotate([angles[1]*180/Math.PI,0,0]);
const tool = pitch.translate(toolOrigin);
const result = stock.subtract(tool);
console.log(JSON.stringify({
  stock:{ origin:[0,0,0],size:[4,4,4],angles:[0,0],mirror:false },
  tool:{ origin:toolOrigin,size:[8,8,16],angles,mirror:false },
  volume:result.volume(),area:result.surfaceArea(),bounds:[result.boundingBox().min,result.boundingBox().max],euler:2-2*result.genus()
}));
for(const item of [result,tool,pitch,tilt,box,stock]) item.delete();
