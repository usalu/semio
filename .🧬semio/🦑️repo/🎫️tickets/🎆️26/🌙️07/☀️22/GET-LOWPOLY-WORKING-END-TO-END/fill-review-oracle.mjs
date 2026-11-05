/** 🧪 Independent exact-colour four-connected fill expectations. */
import graphlib from "graphlib";
import Ajv from "ajv";
import assert from "node:assert/strict";
import fs from "node:fs";
const directory = new URL("./", import.meta.url);
const fixture = JSON.parse(fs.readFileSync(new URL("fill-review-cases.json", directory), "utf8"));
const schema = JSON.parse(fs.readFileSync(new URL("fill-review-schema.json", directory), "utf8"));
const validate = new Ajv({strict:true}).compile(schema);
assert.ok(validate(fixture), JSON.stringify(validate.errors));
for (const row of fixture.cases) {
  assert.equal(row.rows.length, row.height);
  assert.ok(row.rows.every(line => line.length === row.width));
  const pixels = row.rows.join("").split("").map(symbol => row.palette[symbol]);
  assert.ok(pixels.every(Boolean));
  const seed = row.seed ?? [Math.round(row.seedUv[0]*(row.width-1)), Math.round((1-row.seedUv[1])*(row.height-1))];
  assert.ok(seed[0] < row.width && seed[1] < row.height);
  const start = seed[1]*row.width+seed[0], target = pixels[start];
  const graph = new graphlib.Graph({directed:false});
  for (let index=0;index<pixels.length;index++) if (pixels[index].every((channel,key)=>channel===target[key])) graph.setNode(String(index));
  for (let y=0;y<row.height;y++) for (let x=0;x<row.width;x++) {
    const index=y*row.width+x;
    for (const next of [x+1<row.width?index+1:-1,y+1<row.height?index+row.width:-1]) if (graph.hasNode(String(index))&&graph.hasNode(String(next))) graph.setEdge(String(index),String(next));
  }
  const distances=graphlib.alg.dijkstra(graph,String(start),()=>1,node=>graph.nodeEdges(node));
  const reached=Object.entries(distances).filter(([,path])=>Number.isFinite(path.distance)).map(([node])=>Number(node)).sort((a,b)=>a-b);
  const output=pixels.map((pixel,index)=>reached.includes(index)?row.color:pixel);
  const changed=output.flatMap((pixel,index)=>pixel.some((channel,key)=>channel!==pixels[index][key])?[index]:[]);
  const offsets=changed.filter((index,position)=>position===0||index!==changed[position-1]+1).map(index=>index*4);
  const expected=row.expected.rows.join("").split("").map(symbol=>symbol==="R"?row.color:row.palette[symbol]);
  assert.deepEqual(reached,row.expected.reached,row.id+": reach");
  assert.deepEqual(changed,row.expected.changed,row.id+": changed");
  assert.deepEqual(offsets,row.expected.runOffsets,row.id+": whole-pixel runs");
  assert.deepEqual(output,expected,row.id+": direct RGBA replacement");
}
console.log("[DEBUG] Independent Graphlib/Ajv colour-law cases passed: "+fixture.cases.length);
