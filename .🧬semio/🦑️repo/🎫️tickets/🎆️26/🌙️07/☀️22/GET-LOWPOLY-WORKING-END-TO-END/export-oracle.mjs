/** 📐️ Independent Three.js oracle for the mounted export geometry expectation. */
import assert from "node:assert/strict";
import fs from "node:fs";
import Ajv from "ajv";
import {PlaneGeometry, Vector3, Triangle} from "three";
const fixture=JSON.parse(fs.readFileSync("✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔌️mounted/export.json","utf8"));
const schema=JSON.parse(fs.readFileSync("✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔌️mounted/export.schema.json","utf8"));
const validate=new Ajv({strict:true}).compile(schema);assert.ok(validate(fixture),JSON.stringify(validate.errors));
const plane=new PlaneGeometry(2,2);plane.rotateX(-Math.PI/2);
const positions=plane.getAttribute("position");
const points=Array.from({length:positions.count},(_,index)=>[positions.getX(index),positions.getY(index),positions.getZ(index)].map(value=>Math.abs(value)<1e-6?0:value)).sort((a,b)=>a[0]-b[0]||a[1]-b[1]||a[2]-b[2]);
assert.deepEqual(points,fixture.expected.positions);
let area=0;const index=plane.getIndex();
for(let offset=0;offset<index.count;offset+=3){const vertices=[0,1,2].map(corner=>new Vector3().fromBufferAttribute(positions,index.getX(offset+corner)));area+=new Triangle(...vertices).getArea();}
assert.ok(Math.abs(area-fixture.expected.area)<1e-6);
assert.equal(index.count/3,fixture.formats.find(row=>row.format==="stl").faces);
plane.dispose();
console.log("[DEBUG] Independent Three.js export fixture:4corners,area4,2triangles passed.");
