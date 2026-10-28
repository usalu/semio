/** ⚖️ Classifies every manifest row of staged batches as comment-only (code equal after dropping comments) or code-changing: node neutral.mjs <repoRoot> <batchDir>... */
import {readFileSync,existsSync} from "node:fs";
import {join} from "node:path";
import {createRequire} from "node:module";
import {spawnSync} from "node:child_process";
const [root,...batches]=process.argv.slice(2);
const ts=createRequire(join(root,"package.json"))("typescript");
const printer=ts.createPrinter({removeComments:true});
const tsCode=(text,name)=>printer.printFile(ts.createSourceFile(name,text,ts.ScriptTarget.Latest,false,name.endsWith(".tsx")?ts.ScriptKind.TSX:ts.ScriptKind.TS));
const pyCode=(path)=>spawnSync("python3",["-I","-c","import ast,sys\nt=ast.parse(open(sys.argv[1]).read())\nfor n in ast.walk(t):\n  b=getattr(n,'body',None)\n  if isinstance(b,list) and b and isinstance(b[0],ast.Expr) and isinstance(getattr(b[0],'value',None),ast.Constant) and isinstance(b[0].value.value,str): b.pop(0)\nprint(ast.dump(t))",path],{encoding:"utf8"}).stdout;
const rsCode=(t)=>t.replace(/\/\/[^\n]*|\/\*[\s\S]*?\*\//gu,"").replace(/\s+/gu," ");
for(const b of batches){
 const m=JSON.parse(readFileSync(join(b,"manifest.json"),"utf8"));
 for(const [rel,e] of Object.entries(m)){
  const bp=join(b,"base",e.file),np=join(b,"files",e.file);
  if(e.base===null||e.new===null||!existsSync(bp)){console.log(`STRUCT ${b.split("/").pop()} ${rel}`);continue;}
  const a=readFileSync(bp,"utf8"),n=readFileSync(np,"utf8");let same;
  if(/\.(m?ts|mjs|js|tsx|d\.mts)$/u.test(rel))same=tsCode(a,rel)===tsCode(n,rel);
  else if(rel.endsWith(".json"))same=JSON.stringify(JSON.parse(a))===JSON.stringify(JSON.parse(n));
  else if(rel.endsWith(".py"))same=pyCode(bp)===pyCode(np);
  else if(rel.endsWith(".rs"))same=rsCode(a)===rsCode(n);
  else same=a===n;
  console.log(`${same?"COMMENT":"CODE"} ${b.split("/").pop()} ${rel}`);
 }
}
