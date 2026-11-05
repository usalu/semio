
/** 🧪️ Validates the file parser oracle using retained authored inputs and independent exporter references. */
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import {fileURLToPath} from "node:url";
import {BufferGeometry,Float32BufferAttribute,Mesh,MeshBasicMaterial} from "three";
import {OBJExporter} from "three/examples/jsm/exporters/OBJExporter.js";
import {PLYExporter} from "three/examples/jsm/exporters/PLYExporter.js";
import {STLExporter} from "three/examples/jsm/exporters/STLExporter.js";
import {runCli} from "./download-geometry-oracle.mjs";
const ticket=path.dirname(fileURLToPath(import.meta.url));
const inputs=path.join(ticket,"export-oracle-inputs");
const generated=path.join(ticket,"🗑️generated","export-oracle");
assert.ok(generated.length<=256);
fs.mkdirSync(generated,{recursive:true});
assert.equal(fs.realpathSync(generated),generated);
let positive=0,negative=0;
function check(file,format,expect) {
 const result=runCli(["--file",file,"--format",format,"--expect",expect]);
 assert.equal(result.corners,4);
 assert.equal(result.triangleCount,2);
 assert.equal(result.area,1);
 positive++;
 return result;
}
const rich=path.join(inputs,"square.json");
const plain=path.join(inputs,"square-stl.json");
for(const format of ["obj","ply","stl"])check(path.join(inputs,"square."+format),format,format==="stl"?plain:rich);
const geometry=new BufferGeometry();
geometry.setAttribute("position",new Float32BufferAttribute([0,0,0,1,0,0,1,1,0,0,1,0],3));
geometry.setAttribute("normal",new Float32BufferAttribute([0,0,1,0,0,1,0,0,1,0,0,1],3));
geometry.setAttribute("uv",new Float32BufferAttribute([0,0,1,0,1,1,0,1],2));
geometry.setIndex([0,1,2,0,2,3]);
const material=new MeshBasicMaterial();
const mesh=new Mesh(geometry,material);
mesh.updateMatrixWorld(true);
function reference(name,format,data,expect) {
 const file=path.join(generated,name);
 const bytes=typeof data==="string"?data:ArrayBuffer.isView(data)?Buffer.from(data.buffer,data.byteOffset,data.byteLength):Buffer.from(data);
 fs.writeFileSync(file,bytes);
 return check(file,format,expect);
}
reference("three.obj","obj",new OBJExporter().parse(mesh),rich);
reference("three-ascii.ply","ply",new PLYExporter().parse(mesh,undefined,{binary:false}),rich);
reference("three-le.ply","ply",new PLYExporter().parse(mesh,undefined,{binary:true,littleEndian:true}),rich);
reference("three-be.ply","ply",new PLYExporter().parse(mesh,undefined,{binary:true,littleEndian:false}),rich);
reference("three-ascii.stl","stl",new STLExporter().parse(mesh,{binary:false}),plain);
reference("three-binary.stl","stl",new STLExporter().parse(mesh,{binary:true}),plain);
const original=JSON.parse(fs.readFileSync(rich,"utf8"));
function refused(name,mutate,file=path.join(inputs,"square.obj"),format="obj") {
 const expectation=structuredClone(original);
 mutate(expectation);
 const target=path.join(generated,name+".json");
 fs.writeFileSync(target,JSON.stringify(expectation));
 assert.throws(()=>runCli(["--file",file,"--format",format,"--expect",target]));
 negative++;
}
refused("wrong-winding",expect=>{expect.windingNormal=[0,0,-1];});
refused("reversed-triangle",expect=>{delete expect.windingNormal;expect.orientedTriangles[0].reverse();});
refused("wrong-area",expect=>{expect.area=2;});
refused("wrong-corner",expect=>{expect.corners[0]=[0,0,2];});
refused("wrong-uv",expect=>{expect.vertices[0].uv=[0.5,0.5];});
refused("wrong-normal",expect=>{expect.vertices[0].normal=[1,0,0];});
refused("stl-has-no-uv",()=>{},path.join(inputs,"square.stl"),"stl");
assert.throws(()=>runCli(["--file",path.join(inputs,"square.obj"),"--format","obj"]));negative++;
assert.throws(()=>runCli(["--file",path.join(inputs,"square.obj"),"--format","obj","--format","ply","--expect",rich]));negative++;
assert.throws(()=>runCli(["--file",process.execPath,"--format","obj","--expect",rich]));negative++;
const noFaces=path.join(generated,"no-faces.obj");
fs.writeFileSync(noFaces,"v 0 0 0\nv 1 0 0\nv 0 1 0\n");
assert.throws(()=>runCli(["--file",noFaces,"--format","obj","--expect",rich]));negative++;
assert.throws(()=>runCli(["--file",path.join(inputs,"square.obj"),"--format","unsupported","--expect",rich]));negative++;
geometry.dispose();
material.dispose();
console.log("[DEBUG] "+JSON.stringify({scope:"parser-selftest-only",positive,negative,productionPayloadsVerified:0,generated}));
