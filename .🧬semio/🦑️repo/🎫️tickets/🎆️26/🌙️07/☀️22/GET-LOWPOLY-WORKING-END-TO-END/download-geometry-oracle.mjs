
/** 📐️ Parses explicit downloaded mesh bytes through independent loaders into neutral geometry assertions. */
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import {pathToFileURL} from "node:url";
import {createHash} from "node:crypto";
import Ajv from "ajv";
import {Matrix3, Matrix4, Vector3} from "three";
import {OBJLoader} from "three/examples/jsm/loaders/OBJLoader.js";
import {PLYLoader} from "three/examples/jsm/loaders/PLYLoader.js";
import {STLLoader} from "three/examples/jsm/loaders/STLLoader.js";

const schema = JSON.parse(fs.readFileSync(new URL("./export-oracle-inputs/geometry.schema.json",import.meta.url),"utf8"));
const validate = new Ajv({strict:true}).compile(schema);
const close = (a,b,tolerance) => a.length===b.length && a.every((value,index)=>Number.isFinite(value)&&Math.abs(value-b[index])<=tolerance);

/** 🛂️ Resolves one explicit regular input inside the current repository without copying its contents. */
function inputPath(value, maximumBytes) {
 const root=fs.realpathSync(process.cwd());
 const absolute=path.resolve(value);
 assert.ok(absolute.length<=256,"input path exceeds 256 UTF-16 units");
 const real=fs.realpathSync(absolute);
 assert.ok(real.length<=256,"resolved input path exceeds 256 UTF-16 units");
 const relative=path.relative(root,real);
 assert.ok(relative!==".."&&!relative.startsWith(".."+path.sep)&&!path.isAbsolute(relative),"input must stay inside the repository");
 const stat=fs.statSync(real);
 assert.ok(stat.isFile(),"input must be a regular file");
 assert.ok(stat.size>0&&stat.size<=maximumBytes,"input byte bound");
 return real;
}

/** 🧪️ Verifies parsed geometry while exposing only plain data outside the third-party parsing boundary. */
export function verifyExport({file,format,expect}) {
 assert.ok(["obj","ply","stl"].includes(format),"format must be obj, ply or stl");
 file=inputPath(file,64*1024*1024);
 expect=inputPath(expect,1024*1024);
 const expected=JSON.parse(fs.readFileSync(expect,"utf8"));
 assert.ok(validate(expected),JSON.stringify(validate.errors));
 const tolerance=expected.tolerance??1e-6;
 if(expected.windingNormal)assert.ok(Math.hypot(...expected.windingNormal)>tolerance,"winding reference must be nonzero");
 const bytes=fs.readFileSync(file);
 const geometries=[];
 let object;
 if(format==="obj"){
  object=new OBJLoader().parse(bytes.toString("utf8"));
  object.updateMatrixWorld(true);
  object.traverse(child=>{if(child.isMesh)geometries.push({geometry:child.geometry,matrix:child.matrixWorld});});
 }else{
  const buffer=bytes.buffer.slice(bytes.byteOffset,bytes.byteOffset+bytes.byteLength);
  geometries.push({geometry:format==="ply"?new PLYLoader().parse(buffer):new STLLoader().parse(buffer),matrix:new Matrix4()});
 }
 const corners=[];
 const vertices=[];
 const triangles=[];
 let area=0,triangleCount=0;
 try {
  for(const {geometry,matrix} of geometries){
   const positions=geometry.getAttribute("position");
   const normals=geometry.getAttribute("normal");
   const uv=geometry.getAttribute("uv");
   assert.ok(positions&&positions.count>=3,"mesh contains positions");
   const index=geometry.getIndex();
   const count=index?.count??positions.count;
   assert.equal(count%3,0,"triangles are complete");
   const normalMatrix=new Matrix3().getNormalMatrix(matrix);
   for(let offset=0;offset<count;offset+=3){
    const triangle=[];
    for(let corner=0;corner<3;corner++){
     const i=index?index.getX(offset+corner):offset+corner;
     assert.ok(Number.isInteger(i)&&i>=0&&i<positions.count,"valid position index");
     const point=new Vector3().fromBufferAttribute(positions,i).applyMatrix4(matrix);
     const position=point.toArray();
     assert.ok(position.every(Number.isFinite),"finite position");
     const vertex={position};
     if(normals)vertex.normal=new Vector3().fromBufferAttribute(normals,i).applyNormalMatrix(normalMatrix).toArray();
     if(uv)vertex.uv=[uv.getX(i),uv.getY(i)];
     vertices.push(vertex);
     if(!corners.some(candidate=>close(candidate,position,tolerance)))corners.push(position);
     triangle.push(point);
    }
    const cross=new Vector3().subVectors(triangle[1],triangle[0]).cross(new Vector3().subVectors(triangle[2],triangle[0]));
    const magnitude=cross.length();
    assert.ok(magnitude>tolerance*tolerance,"nondegenerate triangle");
    triangles.push(triangle.map(point=>point.toArray()));
    area+=magnitude/2;
    triangleCount++;
    if(expected.windingNormal)assert.ok(cross.normalize().dot(new Vector3(...expected.windingNormal).normalize())>=1-tolerance,"triangle winding disagrees with expected orientation");
   }
  }
  assert.ok(triangleCount>0,"at least one parsed triangle");
  assert.equal(corners.length,expected.corners.length,"semantic corner count");
  for(const corner of corners)assert.ok(expected.corners.some(candidate=>close(candidate,corner,tolerance)),"unexpected semantic corner "+JSON.stringify(corner));
  for(const corner of expected.corners)assert.ok(corners.some(candidate=>close(candidate,corner,tolerance)),"missing semantic corner");
  assert.ok(Math.abs(area-expected.area)<=tolerance*Math.max(1,expected.area),"surface area");
  if(expected.triangleCount!==undefined)assert.equal(triangleCount,expected.triangleCount,"triangle count");
  if(expected.orientedTriangles){
   assert.equal(triangles.length,expected.orientedTriangles.length,"oriented triangle count");
   const unmatched=expected.orientedTriangles.slice();
   for(const triangle of triangles){
    const found=unmatched.findIndex(wanted=>[0,1,2].some(shift=>triangle.every((point,index)=>close(point,wanted[(index+shift)%3],tolerance))));
    assert.ok(found>=0,"unexpected triangle or reversed winding");
    unmatched.splice(found,1);
   }
   assert.equal(unmatched.length,0,"missing oriented triangle");
  }
  if(expected.vertices){
   const matches=(actual,wanted)=>close(actual.position,wanted.position,tolerance)&&(!wanted.normal||(actual.normal&&close(actual.normal,wanted.normal,tolerance)))&&(!wanted.uv||(actual.uv&&close(actual.uv,wanted.uv,tolerance)));
   for(const actual of vertices)assert.ok(expected.vertices.some(wanted=>matches(actual,wanted)),"unexpected position/normal/UV tuple "+JSON.stringify(actual));
   for(const wanted of expected.vertices)assert.ok(vertices.some(actual=>matches(actual,wanted)),"missing expected position/normal/UV tuple "+JSON.stringify(wanted));
  }
  return {scope:"explicit-file-geometry",file,format,expect,sha256:createHash("sha256").update(bytes).digest("hex"),bytes:bytes.length,corners:corners.length,triangleCount,area,attributesChecked:expected.vertices?["position",...["normal","uv"].filter(key=>expected.vertices.some(vertex=>vertex[key]))]:["position"],windingChecked:!!(expected.windingNormal||expected.orientedTriangles)};
 }finally{for(const {geometry} of geometries)geometry.dispose();}
}

/** ⌨️ Requires three explicit inputs so fixture-only checks cannot be confused with downloaded file verification. */
export function runCli(args) {
 const options={};
 for(let i=0;i<args.length;i+=2){
  const key=args[i];
  assert.ok(["--file","--format","--expect"].includes(key),"unknown argument "+key);
  assert.ok(args[i+1]&&!args[i+1].startsWith("--"),"missing value for "+key);
  assert.ok(options[key]===undefined,"duplicate argument "+key);
  options[key]=args[i+1];
 }
 assert.equal(Object.keys(options).length,3,"required: --file PATH --format obj|ply|stl --expect PATH");
 return verifyExport({file:options["--file"],format:options["--format"],expect:options["--expect"]});
}
if(process.argv[1]&&pathToFileURL(path.resolve(process.argv[1])).href===import.meta.url)console.log("[DEBUG] "+JSON.stringify(runCli(process.argv.slice(2))));
