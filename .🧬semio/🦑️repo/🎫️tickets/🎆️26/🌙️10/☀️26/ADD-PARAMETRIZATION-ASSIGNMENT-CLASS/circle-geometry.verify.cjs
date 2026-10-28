const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');
const fixture = JSON.parse(fs.readFileSync(path.join(__dirname, 'circle-geometry.case.json'), 'utf8'));
const source = fs.readFileSync(path.resolve('🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-geometry.tex'), 'utf8');
const block = name => source.split('\\newcommand{\\' + name + '}')[1].split('\\newcommand')[0];
const edges = name => [...block(name).matchAll(/#1\{(\d+)\}\{(\d+)\}/g)].map(match => [+match[1], +match[2]]);
const selected = edges('AssignmentSelectedEdges');
assert.deepEqual(selected, [...fixture.selectedRing, ...fixture.selectedChords], 'Selected perimeter must close 16→1');
const all = edges('AssignmentDirectedEdges');
assert.equal(all.length, fixture.directedArcCount);
assert.equal(new Set(all.map(edge => edge.join(','))).size, fixture.directedArcCount);
for (let from = 1; from <= fixture.vertices; from++) {
  for (let to = 1; to <= fixture.vertices; to++) {
    if (from !== to) assert(all.some(edge => edge[0] === from && edge[1] === to));
  }
}
const radii = [...source.matchAll(/\\AssignmentGraphRadiusY=(\d+)mm/g)].map(match => +match[1]);
assert.deepEqual(radii, [fixture.radiusYmm, fixture.workedExampleRadiusYmm]);
const ratio = (2 * fixture.radiusYmm + fixture.estimatedFixedHeightMm) / fixture.previousVisibleHeightMm;
assert(ratio >= fixture.visibleHeightRatioTolerance[0] && ratio <= fixture.visibleHeightRatioTolerance[1]);
assert.equal((block('AssignmentWorkedExampleGraphPicture').match(/\\AssignmentWorkedExampleGraphEdge\{/g) || []).length, 6);
assert(source.includes('(v#1) to[bend left=8] (v#2);'));
assert(source.includes('(v#1) to[bend left=16]'));
assert(source.includes('\\dimexpr\\linewidth/2-14mm\\relax'));
assert(source.includes('\\path[use as bounding box] (0,3) rectangle (2,-60);'));
process.stdout.write(JSON.stringify({pass:true,directedArcs:all.length,selectedArcs:selected.length,radiusYmm:radii[0],workedExampleRadiusYmm:radii[1],estimatedVisibleHeightMm:2*fixture.radiusYmm+fixture.estimatedFixedHeightMm,estimatedVisibleHeightRatio:ratio})+'\n');
