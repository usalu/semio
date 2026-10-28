const fs=require("fs"),path=require("path"),assert=require("assert/strict");
const fixture=JSON.parse(fs.readFileSync(path.join(__dirname,"directed-geometry.fixture.json"),"utf8"));
const root="🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment";
for(const language of ["de","en"]){
 const source=fs.readFileSync(path.join(root,"parametrization-"+language+".content.tex"),"utf8");
 const table=source.match(/\\AssignmentWorkedExampleTable((?:\{[^{}]*\}){7})/);
 assert(table,"Worked example table");
 const shapes=Array.from(table[1].matchAll(/\{([^{}]*)\}/g),m=>m[1]).slice(4);
 const graph=source.match(/\\AssignmentWorkedExampleGraph\n((?:  \{[^\n]*\}\n){6})/);
 assert(graph,"Exactly six localized graph labels");
 const labels=graph[1].trim().split("\n").map(value=>value.trim().slice(1,-1));
 fixture.workedExampleDirections.forEach(([a,b],index)=>{
  const delta=[0,0,1][b-1]-[0,0,1][a-1];
  const prefix=delta===0?"":"$\\Delta L="+(delta>0?"+":"")+delta+"$; ";
  assert.equal(labels[index],prefix+"Form: "+shapes[a-1]+" $\\to$ "+shapes[b-1]);
 });
 assert(!/AssignmentWorkedExampleLedger|marked pairs|markierten Paare|120|\\maketitle|\\tableofcontents/.test(source));
 assert.equal((source.match(/\\begin\{AssignmentTable\}/g)||[]).length,2);
 assert(source.includes(language==="de"?"gerichteten $K_{16}$":"directed $K_{16}$"));
}
console.log("Directed bilingual content contract passed: six ordered labels and preserved two matrices.");
