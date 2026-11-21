import {readFile} from "node:fs/promises";
import {resolve} from "node:path";
import Parser from "web-tree-sitter";
const repo = process.cwd();
const mesh = "🧰️framework/🔨️modules/🧊️3d/🥽️mesh";
const history = JSON.parse(await readFile(resolve(repo,mesh,"🧪️tests/📦️targets/📋️preserved.json"),"utf8"));
const movedPath = mesh + "/🛠️modeling/🔁️reconstruction/🧪️tests/🦀️.rs";
await Parser.init({wasmBinary:await Bun.file(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm",repo)).arrayBuffer()});
const parser=new Parser();
parser.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm",repo)));
function extract(source:string) {
  const tree=parser.parse(source)!;
  try {
    return new Map(tree.rootNode.descendantsOfType("function_item").map(node=>[
      node.childForFieldName("name")!.text,(node.text.match(/\bassert(?:_eq|_ne)?!/g)??[]).length,
    ]));
  } finally {
    tree.delete();
  }
}
async function archive(file:string){
  const args=["repo","tools","exec","--light","git","--","show","3d62b2ef22c562d836e27f09600a7f24fe84f5e8:"+file];
  const task=Bun.spawn(args,{stdout:"pipe",stderr:"pipe"});
  const [status,content,error]=await Promise.all([task.exited,new Response(task.stdout).text(),new Response(task.stderr).text()]);
  if(status!==0)throw Error(error);
  return content;
}
const originalMoved=extract(await archive(movedPath));
const currentMoved=extract(await readFile(resolve(repo,movedPath),"utf8"));
for(const row of history){
 const original=extract(await archive(row.path));
 const current=extract(await readFile(resolve(repo,row.path),"utf8"));
 const out=row.tests.map((name:string)=>({name,original:original.get(name)??originalMoved.get(name),current:current.get(name)??currentMoved.get(name)}));
 console.log(JSON.stringify({path:row.path,total:out.length,original:out.reduce((sum,x)=>sum+(x.original??0),0),current:out.reduce((sum,x)=>sum+(x.current??0),0),delta:out.filter(x=>x.original!==x.current)}));
}
parser.delete();
