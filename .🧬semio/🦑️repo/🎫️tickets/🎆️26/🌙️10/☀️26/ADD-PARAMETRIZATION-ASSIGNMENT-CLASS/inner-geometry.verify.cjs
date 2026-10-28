const fs=require("fs"),assert=require("node:assert/strict"),path=require("path");
const fixture=JSON.parse(fs.readFileSync(path.join(__dirname,"inner-geometry.case.json"),"utf8"));
const source=fs.readFileSync(path.resolve("🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-geometry.tex"),"utf8");
function edges(name){const marker="\\newcommand{\\"+name+"}[1]{\n",start=source.indexOf(marker);assert(start>=0,name+" exists");const body=source.slice(start+marker.length).split("\n}")[0];return Array.from(body.matchAll(/#1\{(\d+)\}\{(\d+)\}/g),m=>[Number(m[1]),Number(m[2])]);}
let state=fixture.seed,attempt=0,vertices;
do{attempt++;vertices=Array.from({length:fixture.vertices},(_,i)=>i+1);for(let i=vertices.length-1;i>=0;i--){state=(Math.imul(1664525,state)+1013904223)>>>0;const j=Math.floor(state/2**32*(i+1));[vertices[i],vertices[j]]=[vertices[j],vertices[i]];}vertices=vertices.slice(0,4).sort((a,b)=>a-b);}while(!vertices.every((v,i)=>{const gap=(vertices[(i+1)%4]-v+fixture.vertices)%fixture.vertices;return gap>=fixture.gapMinimum&&gap<=fixture.gapMaximum;}));
assert.equal(attempt,fixture.acceptedAttempt);
assert.deepEqual(vertices.map((v,i)=>[v,vertices[(i+1)%4]]),fixture.innerCycle);
const base=edges("AssignmentDirectedEdges"),selected=edges("AssignmentSelectedEdges");
assert.equal(base.length,fixture.baseArcCount);
assert.equal(new Set(base.map(e=>e.join(","))).size,fixture.baseArcCount);
assert(base.every(([a,b])=>a!==b&&a>=1&&a<=fixture.vertices&&b>=1&&b<=fixture.vertices));
assert.equal(selected.length,fixture.selectedArcCount);
assert.deepEqual(selected.slice(0,fixture.vertices),fixture.outerCycle);
assert.deepEqual(selected.slice(fixture.vertices),fixture.innerCycle);
assert(selected.every(([a,b])=>base.some(([x,y])=>a===x&&b===y)));
for(const cycle of [fixture.outerCycle,fixture.innerCycle]){assert(cycle.every((e,i)=>e[1]===cycle[(i+1)%cycle.length][0]));assert.equal(new Set(cycle.map(e=>e[0])).size,cycle.length);}
assert(source.includes("\\AssignmentGraphRadiusY=53mm"));
assert(source.includes("\\AssignmentBox{2.5}{1.5}{2}"));
assert(source.includes("\\AssignmentPyramid{2.5}{1.5}{2}"));
assert(source.includes("\\AssignmentEllipticalCylinder{3.5}{2.5}{3}"));
assert(source.includes("parameter-rows=4,header-height=20mm"));
console.log("PASS inner cycle, seed selection, outer cycle, 240 base arcs, 20 selected arcs and unchanged shape metrics");
