const fs=require("fs"),path=require("path");
let root=__dirname;
while(!fs.existsSync(path.join(root,"🧰️framework/🛍️products/📓️print")))root=path.dirname(root);
const source=fs.readFileSync(path.join(root,"🧰️framework/🛍️products/📓️print/🖋️latex/semio-assignment.sty"),"utf8");
const cases=JSON.parse(fs.readFileSync(path.join(__dirname,"vertical-style.case.json"),"utf8")).cases;
const slash=String.fromCharCode(92),helper=slash+"semio_assignment_matrix_text:n";
if(!source.includes(helper))throw Error("Filled matrix text must use a measured-height centering helper");
for(const name of ["semio_assignment_matrix_value:n","AssignmentParameterMatrixRow"]){
  const start=source.indexOf(slash+name),end=source.indexOf("\n}",start);
  if(!source.slice(start,end).includes(helper))throw Error(name+" must use shared text centering");
}
if(!source.includes(slash+"rule[-.5"+slash+"l_semio_assignment_matrix_row_dim]{0pt}{"+slash+"l_semio_assignment_matrix_row_dim}"))throw Error("Writing geometry changed");
console.log("PASS "+cases.length+" vertical style contracts");
