import fs from "node:fs";
import path from "node:path";
import {fileURLToPath} from "node:url";
const ticket=path.dirname(fileURLToPath(import.meta.url));
let root=ticket;
while(!fs.existsSync(path.join(root,"🧰️framework/🛍️products/📓️print"))){const parent=path.dirname(root);if(parent===root)throw Error("Print product root required");root=parent;}
const print=path.join(root,"🧰️framework/🛍️products/📓️print");
const generated=path.join(ticket,"🗑️generated/example-table-chrome");
const mode=process.argv[2];
fs.mkdirSync(generated,{recursive:true});
function update(relative,transform){
  const target=path.join(print,relative),original=fs.readFileSync(target,"utf8"),updated=transform(original);
  if(updated===original)return;
  const temporary=path.join(generated,path.basename(target)+".new");
  fs.writeFileSync(temporary,updated);
  if(fs.readFileSync(target,"utf8")!==original)throw Error("Concurrent edit: "+relative);
  fs.renameSync(temporary,target);
}
if(mode==="contract"){
  update("🧪️tests/📝️assignment/🔣️.json",text=>{
    const data=JSON.parse(text);
    for(const row of data.silhouette.languages)if(row.titles.length===2)row.titles.unshift(row.language==="de"?"Beispiel":"Example");
    return JSON.stringify(data,null,2)+"\n";
  });
  update("🧪️tests/📝️assignment/🟦️.ts",text=>{
    text=text.replace('titles: { type: "array", minItems: 2, maxItems: 2','titles: { type: "array", minItems: 3, maxItems: 3');
    const before='expect(blocks.map(block => block[2]?.match(/\\\\AssignmentExerciseTable\\{(One|Two)\\}/)?.[1])).toEqual(["One", "Two"]);';
    const after='expect(blocks[0]?.[2]).toMatch(/\\\\AssignmentWorked\\w*Table\\b/);\n    expect(blocks.slice(1).map(block => block[2]?.match(/\\\\AssignmentExerciseTable\\{(One|Two)\\}/)?.[1])).toEqual(["One", "Two"]);';
    if(text.includes(before))text=text.replace(before,after);
    if(!text.includes('expect(blocks.slice(1).map('))throw Error("Expected table contract missing");
    return text;
  });
  update("🧪️tests/📝️assignment/🥒️.feature",text=>text.includes("Worked example uses the native table silhouette")?text:text+"\n  Scenario: Worked example uses the native table silhouette\n    Given the worked example parameter matrix in either supported language\n    Then its localized title and number use the same connected table outline as the exercise matrices\n    And its type icons and parameter values remain in the matrix cells\n");
}else if(mode==="render"){
  for(const language of ["en","de"])update("🧾️template/📝️assignment/parametrization-"+language+".content.tex",text=>{
    const title=language==="de"?"Beispiel":"Example";
    const match=[...text.matchAll(/^\\AssignmentWorked\w*Table[^\n]*$/gm)];
    if(match.length!==1)throw Error("Expected one worked table call");
    const call=match[0][0],wrapped="\\begin{AssignmentTable}{"+title+"}\n"+call+"\n\\end{AssignmentTable}";
    return text.includes(wrapped)?text:text.replace(call,wrapped);
  });
}else throw Error("Expected contract or render mode");
console.log("Example table "+mode+" updated");
