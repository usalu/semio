import ts from "typescript";
import {readFileSync,writeFileSync} from "node:fs";
import {dirname,relative,resolve} from "node:path";
const rows:{file:string;spec:string;target:string}[]=JSON.parse(readFileSync(process.argv[2]!,"utf8"));
const apply=process.argv[3]==="--apply";
const framework=resolve("🧰️framework");
const facade=resolve(framework,"🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts");
const rename:Record<string,string>={runRepositoryCargoTestsV1:"runAdmittedCargoTestsV1",runRepositoryCargoLintV1:"runAdmittedCargoLintV1",runRepositoryCargoCommandV1:"runAdmittedCargoCommandV1",runRepositoryCargoTests:"runAdmittedCargoPackageTestsV1",repositoryCargoTestPolicyV1:"admittedCargoTestPolicyV1",runRepositoryExactCargoLaws:"runAdmittedExactCargoLawsV1",buildRepositoryCargoArtifacts:"buildAdmittedCargoArtifactsV1"};
const dropped=new Set(["configuredRepositoryCargoPreparationPolicyV1","repositoryCargoNetworkV1"]);
const productCargo=(target:string)=>/🛍️products\/🦑️repo\/🔨️modules\/📚️library\/(🏃️process\/🦀️cargo\/(🎟️admission|🎯️exact|🔣️policy|📥️dependencies)|⚡️caching\/📦️artifacts\/🏗️native-build)\/🟦️\.ts$/.test(target);
const files=[...new Set(rows.filter(r=>r.file.endsWith("📜️script.ts")&&productCargo(r.target)).map(r=>r.file))];
let changed=0;
for(const file of files){
 const original=readFileSync(file,"utf8");
 const sf=ts.createSourceFile(file,original,ts.ScriptTarget.Latest,true);
 const productImports=rows.filter(r=>r.file===file&&productCargo(r.target)).map(r=>r.spec);
 const edits:{start:number;end:number;text:string}[]=[];
 const specifiers:string[]=[];
 const local:Record<string,string>={};
 let facadeImport:ts.ImportDeclaration|undefined;
 const imports:ts.ImportDeclaration[]=[];
 sf.forEachChild(n=>{
  if(!ts.isImportDeclaration(n))return;
  const spec=(n.moduleSpecifier as ts.StringLiteral).text;
  if(resolve(dirname(file),spec)===facade)facadeImport=n;
  if(productImports.includes(spec)&&n.importClause?.namedBindings&&ts.isNamedImports(n.importClause.namedBindings))imports.push(n);
 });
 if(!imports.length)continue;
 for(const n of imports){
  for(const e of (n.importClause!.namedBindings as ts.NamedImports).elements){
   const imported=e.propertyName?.text??e.name.text,alias=e.propertyName?e.name.text:undefined;
   if(dropped.has(imported))continue;
   const next=rename[imported];if(!next)throw Error(file+": unmapped import "+imported);
   specifiers.push(alias?`${next} as ${alias}`:next);
   if(!alias)local[imported]=next;
  }
 }
 const facadeSpec=(()=>{let p=relative(dirname(file),facade).split("\\").join("/");return p.startsWith(".")?p:"./"+p;})();
 const first=imports[0]!;
 let addedToFacade=false;
 if(facadeImport&&facadeImport.importClause?.namedBindings&&ts.isNamedImports(facadeImport.importClause.namedBindings)){
  const existing=(facadeImport.importClause.namedBindings as ts.NamedImports).elements.map(e=>e.getText());
  const names=[...existing,...specifiers.filter(s=>!existing.includes(s))];
  edits.push({start:facadeImport.importClause.namedBindings.getStart(),end:facadeImport.importClause.namedBindings.getEnd(),text:"{ "+names.join(", ")+" }"});
  addedToFacade=true;
 }
 for(const n of imports){
  if(n===first&&!addedToFacade&&specifiers.length)edits.push({start:n.getStart(),end:n.getEnd(),text:`import { ${specifiers.join(", ")} } from ${JSON.stringify(facadeSpec)};`});
  else{
   let end=n.getEnd();if(original[end]==="\n")end++;
   let start=n.getStart();
   edits.push({start,end,text:""});
  }
 }
 edits.sort((a,b)=>b.start-a.start);
 let text=original;
 for(const e of edits)text=text.slice(0,e.start)+e.text+text.slice(e.end);
 const bodyStart=text.indexOf("\n",text.lastIndexOf("import "));
 const head=text.slice(0,bodyStart),body=text.slice(bodyStart);
 let newBody=body;
 for(const [from,to] of Object.entries(local)){
  newBody=newBody.replace(new RegExp(`(?<![\\w$.])${from}(?![\\w$])`,"g"),to);
 }
 newBody=newBody.replace(/(?<![\w$.])admittedCargoTestPolicyV1\(/g,"await admittedCargoTestPolicyV1(");
 const dropPair=(source:string,re:string)=>{
  const withTrailing=new RegExp(re+"\\s*,\\s*","g"),withLeading=new RegExp("\\s*,\\s*"+re,"g");
  return source.replace(withTrailing,"").replace(withLeading,"").replace(new RegExp(re,"g"),"");
 };
 newBody=dropPair(newBody,"preparation:\\s*configuredRepositoryCargoPreparationPolicyV1\\(\\)");
 newBody=dropPair(newBody,"network:\\s*repositoryCargoNetworkV1\\([^()]*\\)");
 const result=head+newBody;
 if(result!==original){changed++;if(apply)writeFileSync(file,result);else console.log("would change",file);}
}
console.log((apply?"changed ":"would change ")+changed+" of "+files.length);
