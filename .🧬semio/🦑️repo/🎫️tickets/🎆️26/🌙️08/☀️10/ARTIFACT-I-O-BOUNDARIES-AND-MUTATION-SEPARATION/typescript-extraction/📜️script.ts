import fs from "node:fs";
import path from "node:path";
import {execFileSync} from "node:child_process";
import ts from "typescript";
const root=process.cwd(), ticket=path.dirname(import.meta.dir), generated=path.join(ticket,"🗑️generated");
const subset=(plugin:string,artifact:string,version="🔖️1",member="✳️any")=>path.resolve(root,"✏️s/🔌️plugins",plugin,"🗿️artifacts",artifact,"🏅️standards",version,"🪆️subsets",member);
const owners={forms:subset("📋️forms","📋️forms"),program:subset("🏛️architect","🏛️program"),layout:subset("📏️layout","📏️layout"),drawing:subset("🗄️stdio","🧿️semio","🔖️v1","🖊️drawing"),graph:subset("🗄️stdio","🧿️semio","🔖️v1","🕸️graph"),pdf:subset("🗄️stdio","📖️pdf","7️⃣1.7","🧱️base"),remodel:subset("📸️remodel","📸️remodeling"),block:subset("🧱️block","🧊️3d"),txt:subset("🗄️stdio","🔤️txt","🔖️utf-8")};
const files=execFileSync("rg",["--files","-g","*.ts","-g","*.tsx","-g","*.json","-g","*.feature"],{cwd:root,encoding:"utf8",maxBuffer:30_000_000}).trim().split("\n").map(f=>path.resolve(root,f));
const moves=new Map<string,string>(), origin=new Map<string,string>(), splits=new Map<string,Map<string,string>>(), changes=new Set<string>();
const rel=(from:string,to:string)=>{const value=path.relative(path.dirname(from),to).split(path.sep).join("/");return value.startsWith(".")?value:"./"+value};
const put=(file:string,text:string,from=file)=>{fs.mkdirSync(path.dirname(file),{recursive:true});fs.writeFileSync(file,text);origin.set(file,from);changes.add(file)};
const tree=(owner:string,source:string,target:string)=>{const from=path.join(owner,source),to=path.join(owner,target);for(const file of files)if(file.startsWith(from+path.sep))moves.set(file,to+file.slice(from.length))};
const snapshot="🚪️io/📝️text/📸️snapshot/";
tree(owners.forms,"🧬️schema/🌱️value/🔣️json",snapshot+"🔣️json");
tree(owners.forms,"🧬️schema/📨️response/📤️export",snapshot+"📨️response/📤️export");
tree(owners.forms,"🧬️schema/🧾️dictionary/🪪️native-json",snapshot+"🧾️dictionary/🪪️native-json");
tree(owners.program,"🧬️schema/🔣️json",snapshot+"🔣️json");
tree(owners.layout,"🧬️schema/🪪️native-json",snapshot+"🪪️native-json");
tree(owners.drawing,"🧬️schema/📸️snapshot/🪪️native-json",snapshot+"🪪️native-json");
tree(owners.pdf,"🧬️schema/📸️snapshot/🪪️native-json",snapshot+"🪪️native-json");
const fileSyntax=(file:string)=>ts.createSourceFile(file,fs.readFileSync(file,"utf8"),ts.ScriptTarget.Latest,true);
const names=(node:ts.Statement):string[]=>ts.isVariableStatement(node)?node.declarationList.declarations.map(d=>d.name.getText()):"name"in node&&node.name?[node.name.getText()]:[];
const extract=(source:string,target:string,selected:string[],header:string)=>{
 const syntax=fileSyntax(source),take=syntax.statements.filter(node=>names(node).some(n=>selected.includes(n))),pieces=take.map(node=>syntax.text.slice(node.getFullStart(),node.end));
 let text=syntax.text;for(const node of [...take].reverse())text=text.slice(0,node.getFullStart())+text.slice(node.end);
 put(source,text);put(target,header+pieces.join("\n")+"\n",source);
 const map=splits.get(source)??new Map<string,string>();for(const name of selected)map.set(name,target);splits.set(source,map);
 return {syntax,pieces};
};
const graphSource=path.join(owners.graph,"🧬️schema/📸️snapshot/🟦️.ts"), graphTarget=path.join(owners.graph,snapshot+"🔣️json/🟦️.ts");
extract(graphSource,graphTarget,["graphRecord","graphText","graphList","graphId","graphProperties","graphWord","graphJson","parseSemioGraphJsonValue","semioGraphJsonValue"],`/** 🔣️ Physical graph JSON projection and admission. */\nimport type {SemioGraphSnapshot} from ${JSON.stringify(rel(graphSource,graphSource))};\n`+fileSyntax(graphSource).statements.filter(ts.isImportDeclaration).map(n=>n.getText()).join("\n")+"\n");
const blockSource=path.join(owners.block,"🧬️schema/📸️snapshot/🟦️.ts"),blockTarget=path.join(owners.block,snapshot+"🔣️json/🟦️.ts");
extract(blockSource,blockTarget,["jsonGeometry","block3dSnapshotToJsonText","block3dSnapshotFromJsonText"],`/** 🔣️ Physical Block 3D snapshot JSON codec. */\nimport {parseBlock3dSnapshot,type Block3dSnapshot} from ${JSON.stringify(rel(blockSource,blockSource))};\n`+fileSyntax(blockSource).statements.filter(ts.isImportDeclaration).filter(n=>/as p |as sharedJson /.test(n.getText())).map(n=>n.getText()).join("\n")+"\n");
put(blockSource,fs.readFileSync(blockSource,"utf8").replace(/^import \* as sharedJson[^\n]*\n/m,""));
const txtSource=path.join(owners.txt,"🧬️schema/🔨️modules/🧬️mutation-support/🟦️.ts"),txtTarget=path.join(owners.txt,"🚪️io/💾️binary/🧬️mutations/🔣️protobuf/🟦️.ts");
extract(txtSource,txtTarget,["TxtProtobufReader","txtProtobufKey","txtProtobufString"],`/** 🛰️ TXT physical protobuf field reader and UTF-8 decoding. */\nimport {failTxtMutationDecode,txtUnicode} from ${JSON.stringify(rel(txtSource,txtSource))};\n`);
const remodelSource=path.join(owners.remodel,"🧬️schema/📸️snapshot/🟦️.ts"),remodelTarget=path.join(owners.remodel,snapshot+"🔣️json/🟦️.ts");
const remodelSyntax=fileSyntax(remodelSource),remodelText=remodelSyntax.text;
const remodelNames=["exactFloat","exactInteger","decodeValue","decodeRecord","decodeRemodelingSnapshot","floatLexeme","indentOf","prettyJson","writeValueJson","writeRecordJson","encodeRemodelingSnapshot","remodelingSnapshotToJsonText","remodelingSnapshotFromJsonText"];
const copyNames=["isPlainObject","finiteNumber","wholeNumber"];
const header=`/** 🔣️ Remodeling physical JSON scalars, defaults and text codec. */\nimport {RemodelingCodecError,camelOf,REMODELING_SNAPSHOT_SPEC,type RemodelingSnapshot,type RecordSpec,type ValueSpec,type Float32Buffer} from ${JSON.stringify(rel(remodelSource,remodelSource))};\nconst fail=(path:string,message:string):never=>{throw new RemodelingCodecError(path,message)};\n`+remodelSyntax.statements.filter(ts.isImportDeclaration).map(n=>n.getText()).join("\n")+"\n"+remodelSyntax.statements.filter(n=>names(n).some(name=>copyNames.includes(name))).map(n=>n.getText()).join("\n")+"\n";
extract(remodelSource,remodelTarget,remodelNames,header);
const strictNames=["exactFloat","exactInteger","decodeValue","decodeRecord"];
let strict=remodelSyntax.statements.filter(n=>names(n).some(name=>strictNames.includes(name))).map(n=>n.getText()).join("\n");
strict=strict.replace(/,transport=true|,transport:boolean|, transport=true|,transport\)/g,match=>match.endsWith(")")?")":"").replace(/if\(transport&&typeof value===\"number\"\)[^\n]*\n/,"").replace(/if\(transport\)\{if\(typeof value.bits[^\n]*\n/,"");
strict=strict.replace(/if\(transport\)\{if\(typeof value===\"number\"[\s\S]*?\}else if\(typeof value===\"bigint\"\)/,"if(typeof value===\"bigint\")");
strict=strict.replace(/case \"bytes\":\{[^\n]*\}/,'case "bytes":return value instanceof Uint8Array?value.slice():fail(path,"expected owned literal octets");');
strict=strict.replace(/value===null\|\|transport&&value===undefined\?null/,'value===null?null').replace('if (!transport || !spec.serdeDefault && !field.jsonOptional)','if (true)').replace(/, transport\)/g,")");
strict=strict.replace(/if \(true\) fail\(([^\n]*?)\);\n      out\[key\] = field.dflt\(\);\n      continue;/g,"fail($1);");
put(remodelSource,fs.readFileSync(remodelSource,"utf8")+"\n/** 🛂️ Canonical owned value admission without physical coercion. */\n"+strict+"\n");
for(const name of strictNames)splits.get(remodelSource)!.delete(name);
const remodelRoot=path.join(owners.remodel,"🧬️schema/🟦️.ts");
extract(remodelRoot,path.join(owners.remodel,snapshot+"🔣️json/🌱️artifact/🟦️.ts"),["decodeRemodelingArtifact"],`/** 🔣️ Declared Remodeling artifact JSON admission. */\nimport {decodeRemodelingSnapshot} from ${JSON.stringify(rel(remodelRoot,remodelTarget))};\nimport type {RemodelingArtifact} from ${JSON.stringify(rel(remodelRoot,remodelRoot))};\n`);
const artifactIo=path.join(owners.remodel,snapshot+"🔣️json/🌱️artifact/🟦️.ts");put(artifactIo,fs.readFileSync(artifactIo,"utf8").replace("snapshot.decodeRemodelingSnapshot","decodeRemodelingSnapshot"),remodelRoot);
const diffSource=path.join(owners.remodel,"🧬️schema/🔺️diff/🟦️.ts"),diffTarget=path.join(owners.remodel,"🚪️io/📝️text/🔺️diff/🔣️json/🟦️.ts");
extract(diffSource,diffTarget,["decodeRemodelingDiff","remodelingDiffToJsonText","encodeRemodelingDiff"],`/** 🔣️ Remodeling physical JSON diff codec. */\nimport {REMODELING_DIFF_SPEC,type RemodelingDiff} from ${JSON.stringify(rel(diffSource,diffSource))};\nimport {decodeRecord,writeRecordJson} from ${JSON.stringify(rel(diffSource,remodelTarget))};\n`);
const mutationSource=path.join(owners.remodel,"🧬️schema/🧬️mutations/🟦️.ts"),mutationTarget=path.join(owners.remodel,"🚪️io/📝️text/🧬️mutations/🔣️json/🟦️.ts");
extract(mutationSource,mutationTarget,["decodeRemodelingMutation"],`/** 🔣️ Remodeling physical tagged JSON mutation admission. */\nimport {REMODELING_MUTATION_SPECS,type RemodelingMutationTag,type RemodelingMutation} from ${JSON.stringify(rel(mutationSource,mutationSource))};\nimport {decodeRecord} from ${JSON.stringify(rel(mutationSource,remodelTarget))};\n`);
put(diffSource,fs.readFileSync(diffSource,"utf8").replace(/^  writeRecordJson,\n/m,""));
put(mutationSource,fs.readFileSync(mutationSource,"utf8").replace(/^  decodeRecord,\n/m,""));
for(const [from,to]of moves){if(fs.existsSync(to))throw Error("Move collision: "+to);put(to,fs.readFileSync(from,"utf8"),from);fs.unlinkSync(from);changes.add(from)}
const targets=[...new Set([...files.filter(f=>!moves.has(f)),...origin.keys()])];
for(const file of targets){
 if(!fs.existsSync(file)||!/[.]tsx?$/.test(file))continue;
 const from=origin.get(file)??file,source=fs.readFileSync(file,"utf8"),syntax=ts.createSourceFile(file,source,ts.ScriptTarget.Latest,true),edits:{start:number,end:number,value:string}[]=[],extras:string[]=[];
 for(const node of syntax.statements){
  if(!ts.isImportDeclaration(node)&&!ts.isExportDeclaration(node))continue;
  const literal=node.moduleSpecifier;if(!literal||!ts.isStringLiteral(literal)||!literal.text.startsWith("."))continue;
  const old=path.resolve(path.dirname(from),literal.text),to=moves.get(old)??old,split=splits.get(old);
  const bindings=ts.isImportDeclaration(node)?node.importClause?.namedBindings:node.exportClause;
  if(split&&bindings&&ts.isNamedImports(bindings)||split&&bindings&&ts.isNamedExports(bindings)){
   const groups=new Map<string,string[]>();for(const item of (bindings as ts.NamedImports|ts.NamedExports).elements){const name=(item.propertyName??item.name).text,target=split!.get(name)??to;const group=groups.get(target)??[];group.push(item.getText());groups.set(target,group)}
   const declarations=[...groups].map(([target,members])=>`${ts.isImportDeclaration(node)?"import":"export"}${ts.isImportDeclaration(node)&&node.importClause?.isTypeOnly?" type":ts.isExportDeclaration(node)&&node.isTypeOnly?" type":""} {${members.join(", ")}} from ${JSON.stringify(rel(file,target))};`).join("\n");
   edits.push({start:node.getStart(syntax),end:node.end,value:declarations});continue;
  }
  if(split&&bindings&&ts.isNamespaceImport(bindings)){
   const identifier=bindings.name.text,alias=identifier+"Physical",memberNames=new Set<string>();
   const visit=(n:ts.Node)=>{if(ts.isPropertyAccessExpression(n)&&ts.isIdentifier(n.expression)&&n.expression.text===identifier&&split.has(n.name.text)){const target=split.get(n.name.text)!;memberNames.add(target);edits.push({start:n.expression.getStart(syntax),end:n.expression.end,value:alias+([...memberNames].indexOf(target))})}ts.forEachChild(n,visit)};visit(syntax);
   for(const [index,target]of [...memberNames].entries())extras.push(`import * as ${alias}${index} from ${JSON.stringify(rel(file,target))};`);
  }
  const value=JSON.stringify(rel(file,to));if(value!==literal.getText())edits.push({start:literal.getStart(syntax),end:literal.end,value});
 }
 let next=source;for(const edit of edits.sort((a,b)=>b.start-a.start))next=next.slice(0,edit.start)+edit.value+next.slice(edit.end);
 if(extras.length)next=extras.join("\n")+"\n"+next;
 if(file.startsWith(owners.remodel)){next=next.replace(/(decodeRecord\([^;\n]+?),\s*false\)/g,"$1)");}
 if(next!==source)put(file,next,from);
}
fs.mkdirSync(generated,{recursive:true});fs.writeFileSync(path.join(generated,"typescript-extraction-files.json"),JSON.stringify([...changes].map(f=>path.relative(root,f)),null,2));
fs.writeFileSync(path.join(ticket,"typescript-extraction-files.md"),"# TypeScript Codec Extraction Files\n\n"+[...changes].sort().map(f=>"- `"+path.relative(root,f)+"`").join("\n")+"\n");
console.log(`[DEBUG] TypeScript extraction moved=${moves.size} updated=${changes.size}`);
