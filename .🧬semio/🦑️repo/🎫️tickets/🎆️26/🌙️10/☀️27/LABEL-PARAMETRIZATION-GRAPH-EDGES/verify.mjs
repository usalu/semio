import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
const root=path.dirname(fileURLToPath(import.meta.url)),c=JSON.parse(fs.readFileSync(path.join(root,'cases.json'),'utf8')),s=fs.readFileSync('🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-geometry.tex','utf8');
const macro=n=>{const start=s.indexOf('\\newcommand{\\'+n+'}');assert.ok(start>=0,'Missing '+n);const end=s.indexOf('\\newcommand',start+1);return s.slice(start,end<0?s.length:end)};
const label=macro('AssignmentGraphEdgeLabel');
assert.ok(label.includes('\\path (v#1) to[bend left=8]'));
assert.ok(label.includes('node[midway,sloped,above=1mm'));
assert.ok(label.includes('font=\\fontsize{8}{9}\\selectfont'));
assert.ok(label.includes('\\AssignmentEdgeIdentifier{#1}{#2}'));
assert.ok(!label.includes('\\draw'));
assert.ok(macro('AssignmentEdgeIdentifier').includes('#1\\,$\\to$\\,#2'));
assert.ok(macro('AssignmentLedgerEntry').includes('\\AssignmentEdgeIdentifier{#1}{#2}'));
for(const n of ['AssignmentGraphPicture','AssignmentWorkedExampleGraphPicture']){const body=macro(n);assert.ok(body.lastIndexOf('\\end{pgfonlayer}')<body.indexOf('\\AssignmentGraphEdgeLabel')||body.includes('\\AssignmentSelectedEdges{\\AssignmentGraphEdgeLabel}'));assert.ok(body.includes('\\AssignmentGraphEdgeLabel'));}
assert.deepEqual([...macro('AssignmentWorkedExampleGraphPicture').matchAll(/\\AssignmentGraphEdgeLabel\{(\d+)\}\{(\d+)\}/g)].map(m=>[+m[1],+m[2]]),c.workedEdges);
assert.deepEqual([...macro('AssignmentSelectedEdges').matchAll(/#1\{(\d+)\}\{(\d+)\}/g)].map(m=>[+m[1],+m[2]]),c.selectedEdges);
assert.ok(macro('AssignmentGraphPicture').indexOf('\\AssignmentSelectedEdges{\\AssignmentGraphEdgeLabel}')>macro('AssignmentGraphPicture').indexOf('\\end{pgfonlayer}'));
console.log('PASS: 6/20 identifiers, midpoint/tangent/gap/layer, shared ledger notation');
