const fs=require("fs");
const assert=require("node:assert/strict");
const root="🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment";
for (const lang of ["de","en"]) {
  const text=fs.readFileSync(root+"/parametrization-"+lang+".content.tex","utf8");
  assert.equal((text.match(/\\section\{/g)||[]).length,1);
  assert.equal((text.match(/\\subsection\{/g)||[]).length,2);
  assert.equal((text.match(/\\clearpage/g)||[]).length,2);
  assert.equal((text.match(/\\begin\{AssignmentTable\}/g)||[]).length,2);
  assert.equal((text.match(/\\AssignmentExerciseTable\{/g)||[]).length,2);
  for (const command of ["AssignmentWorkedBoxTable","AssignmentWorkedBoxGraph","AssignmentWorkedBoxLedger","AssignmentCompleteGraph","AssignmentSelectedLedger"]) assert.equal((text.match(new RegExp("\\\\"+command+"\\b","g"))||[]).length,1);
  assert.ok(!/\\(?:makeassignmenttitle|maketitle|subtitle)\b/.test(text));
  assert.ok(!/Abschlussprüfung|Final check|Übergänge Zwischen|Transitions Between/.test(text));
}
const add=(a,b)=>a.map((v,i)=>v+b[i]);
const negate=a=>a.map(v=>-v);
const a=[2,1,1],b=[3,1,1],c=[3,2,1];
assert.deepEqual([b[0],b[2]],[c[0],c[2]]);
assert.deepEqual(add(add([1,0,0],[0,1,0]),negate([1,1,0])),[0,0,0]);
assert.deepEqual(add(add([1,0,0],[0,1,0]),negate([1,2,0])),[0,-1,0]);
const map=([l,w,h])=>[l,l+w,h];
const inverse=([p,q,r])=>[p,q-p,r];
for(const point of [a,b,c]) assert.deepEqual(inverse(map(point)),point);
assert.equal(map(b).filter((v,i)=>v!==map(a)[i]).length,2);
assert.equal(b.filter((v,i)=>v!==a[i]).length,1);
process.stdout.write("Concise bilingual structure and worked-example math passed.\n");
