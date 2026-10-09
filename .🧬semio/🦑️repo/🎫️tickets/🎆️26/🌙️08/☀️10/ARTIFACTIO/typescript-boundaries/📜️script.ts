import fs from "node:fs";
import path from "node:path";
import {execFileSync} from "node:child_process";
import ts from "typescript";
const root=process.cwd(),ticket=path.dirname(import.meta.dir),changed=new Set<string>(),maps=new Map<string,Map<string,string>>();
const owner=(p:string,a:string,v="🔖️1",s="✳️any")=>path.resolve(root,"✏️s/🔌️plugins",p,"🗿️artifacts",a,"🏅️standards",v,"🪆️subsets",s);
const rel=(file:string,target:string)=>{const s=path.relative(path.dirname(file),target);return s.startsWith(".")?s:"./"+s};
const put=(file:string,text:string)=>{fs.mkdirSync(path.dirname(file),{recursive:true});fs.writeFileSync(file,text);changed.add(file)};
const names=(n:ts.Statement):string[]=>ts.isVariableStatement(n)?n.declarationList.declarations.map(d=>d.name.getText()):"name"in n&&n.name?[n.name.getText()]:[];
const extract=(source:string,target:string,selected:string[],header="")=>{
 const text=fs.readFileSync(source,"utf8"),syntax=ts.createSourceFile(source,text,ts.ScriptTarget.Latest,true),take=syntax.statements.filter(n=>names(n).some(s=>selected.includes(s))),body=take.map(n=>text.slice(n.getFullStart(),n.end)).join("\n");
 const needed=syntax.statements.filter(n=>!take.includes(n)&&!ts.isImportDeclaration(n)&&names(n).some(s=>new RegExp("\\b"+s+"\\b").test(body)));
 const imports=syntax.statements.filter(ts.isImportDeclaration).map(n=>n.getText().replace(/(["'])(\.\.?\/[^"']+)\1/g,(_,q,s)=>JSON.stringify(rel(target,path.resolve(path.dirname(source),s))))).join("\n");
 let next=text;for(const n of [...take,...needed.filter(n=>!n.modifiers?.some(m=>m.kind===ts.SyntaxKind.ExportKeyword))].sort((a,b)=>b.getFullStart()-a.getFullStart())){const replacement=take.includes(n)?"":text.slice(n.getFullStart(),n.getStart(syntax))+"export "+n.getText();next=next.slice(0,n.getFullStart())+replacement+next.slice(n.end)}
 put(source,next);put(target,header+imports+"\n"+(needed.length?`import {${needed.flatMap(names).join(",")}} from ${JSON.stringify(rel(target,source))};\n`:"")+body+"\n");
 const map=maps.get(source)??new Map<string,string>();for(const s of selected)map.set(s,target);maps.set(source,map);
};
const forms=owner("📋️forms","📋️forms"),formsSource=path.join(forms,"🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts");
extract(formsSource,path.join(forms,"🚪️io/📝️text/🔺️diff/🔣️json/🟦️.ts"),["parseFormsJsonDiff"]);
extract(formsSource,path.join(forms,"🚪️io/📝️text/🧬️mutations/🎛️change-block-field/🔣️json/🟦️.ts"),["parseFormsJsonChangeBlockField","formsBlockFieldJson"]);
for(const [p,a,format,functions]of [["🏛️architect","🏛️program","🔣️json",["delta","diff","programDiffFromJson","programDiffToJson"]],["📏️layout","📏️layout","🪪️native-json",["delta","diff","layoutDiffFromNativeJson","layoutDiffNativeJson"]]]as const){const o=owner(p,a);extract(path.join(o,"🚪️io/📝️text/📸️snapshot",format,"🟦️.ts"),path.join(o,"🚪️io/📝️text/🔺️diff",format,"🟦️.ts"),[...functions]);}
const txt=owner("🗄️stdio","🔤️txt","🔖️utf-8");
const txtFiles=execFileSync("rg",["--files",path.join(txt,"🧬️schema/🧬️mutations"),"-g","🟦️.ts"],{encoding:"utf8"}).trim().split("\n");
for(const source of txtFiles){if(source.includes("🧪️tests"))continue;const text=fs.readFileSync(source,"utf8"),syntax=ts.createSourceFile(source,text,ts.ScriptTarget.Latest,true),selected=syntax.statements.flatMap(names).filter(n=>/^decode.*Protobuf$/.test(n));if(selected.length)extract(source,path.join(txt,"🚪️io/💾️binary/🧬️mutations",path.basename(path.dirname(source)),"🟦️.ts"),selected);}
const pdf=owner("🗄️stdio","📖️pdf","7️⃣1.7","🧱️base"),pdfSource=path.join(pdf,"🧬️schema/📸️snapshot/🟦️.ts");
let pdfText=fs.readFileSync(pdfSource,"utf8");
pdfText=pdfText.replace(/^import \{[^\n]*FromNativeJson[^\n]*\n/gm,"");
const syntax=ts.createSourceFile(pdfSource,pdfText,ts.ScriptTarget.Latest,true),schemaNode=syntax.statements.find(n=>ts.isVariableStatement(n)&&n.declarationList.declarations.some(d=>d.name.getText()==="schema"))!;
const schemaDeclaration=(schemaNode as ts.VariableStatement).declarationList.declarations[0]!,schemaExpression=schemaDeclaration.initializer!.getText().replace(/ as const$/,"");
const nativeSchema=JSON.parse(schemaExpression),canonical=structuredClone(nativeSchema);
const canonicalize=(s:any)=>{if(!s||typeof s!=="object")return;if(s.type==="number"){delete s.type;s.semioPrimitive="binary64";}for(const value of Object.values(s))if(Array.isArray(value))value.forEach(canonicalize);else canonicalize(value)};canonicalize(canonical);
const def=(name:string)=>canonical.$defs[name];
const union=(name:string,kind:string)=>(def(name).anyOf??def(name).oneOf).find((s:any)=>s.properties?.kind?.const===kind);
union("PdfObject","int").properties.value={semioPrimitive:"i64"};
for(const key of ["popup","inReplyTo"]){const s=def("PdfMarkupAnnotation").properties[key];for(const branch of s.anyOf)if(branch.type==="integer"){delete branch.type;branch.semioPrimitive="u64";}}
for(const branch of union("PdfAnnotationKind","popup").properties.parent.anyOf)if(branch.type==="integer"){delete branch.type;branch.semioPrimitive="u64";}
put(path.join(pdf,"🚪️io/📝️text/📸️snapshot/🪪️native-json/🧬️schema/🟦️.ts"),"/** 🔣️ PDF 1.7 native JSON transport schema. */\nexport const schema="+JSON.stringify(nativeSchema,null,2)+" as const;\n");
pdfText=pdfText.slice(0,schemaNode.getStart(syntax))+"export const schema = "+JSON.stringify(canonical,null,2)+" as const;"+pdfText.slice(schemaNode.end);
pdfText=pdfText.replace(/^export const (parse\w+) = \(value: unknown\): (\w+) => [^\n]*validateAgainst<[^>]+>\(schema, ([^,]+), value\)[^\n]*;/gm,(_,name,type,pointer)=>`export const ${name} = (value: unknown): ${type} => validateAgainst<${type}>(schema, ${pointer}, value);`);
pdfText=pdfText.replace(/^export const parsePdfEmbeddedCMap[^\n]*;/m,'export const parsePdfEmbeddedCMap = (value:unknown):PdfEmbeddedCMap=>validateAgainst<PdfEmbeddedCMap>(schema,"/$defs/PdfEmbeddedCMap",value);');
const marker='  if (typeof schema["$ref"] === "string")';
pdfText=pdfText.replace(marker,'  if (schema.semioPrimitive === "binary64") { try { parseBinary64(value); return true; } catch { return (errors.push(`${at}: expected owned binary64`), false); } }\n  if (schema.semioPrimitive === "i64" || schema.semioPrimitive === "u64") return typeof value === "bigint" && value >= (schema.semioPrimitive === "i64" ? -9223372036854775808n : 0n) && value <= (schema.semioPrimitive === "i64" ? 9223372036854775807n : 18446744073709551615n) || (errors.push(`${at}: expected owned integer word`), false);\n'+marker);
pdfText=pdfText.replace('import type {Binary64}', 'import {parseBinary64,type Binary64}');put(pdfSource,pdfText);
const layoutSource=path.join(owner("📏️layout","📏️layout"),"🧬️schema/🟦️.ts"),layoutCodec=path.join(owner("📏️layout","📏️layout"),"🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts");
put(layoutSource,fs.readFileSync(layoutSource,"utf8").replace(/^export \{layoutArtifactFromNativeJson[^\n]*\n/m,""));maps.set(layoutSource,new Map([["layoutArtifactFromNativeJson",layoutCodec],["layoutArtifactNativeJson",layoutCodec],["layoutDiffFromNativeJson",path.join(owner("📏️layout","📏️layout"),"🚪️io/📝️text/🔺️diff/🪪️native-json/🟦️.ts")],["layoutDiffNativeJson",path.join(owner("📏️layout","📏️layout"),"🚪️io/📝️text/🔺️diff/🪪️native-json/🟦️.ts")]]));
const files=execFileSync("rg",["--files","-g","*.ts","-g","*.tsx"],{encoding:"utf8",maxBuffer:30_000_000}).trim().split("\n").map(f=>path.resolve(root,f));
for(const file of files){const source=fs.readFileSync(file,"utf8");if(![...maps.keys()].some(key=>source.includes(path.basename(path.dirname(key)))))continue;const ast=ts.createSourceFile(file,source,ts.ScriptTarget.Latest,true),edits:{start:number,end:number,value:string}[]=[],extra:string[]=[];
for(const n of ast.statements){if(!ts.isImportDeclaration(n)&&!ts.isExportDeclaration(n))continue;const lit=n.moduleSpecifier;if(!lit||!ts.isStringLiteral(lit)||!lit.text.startsWith("."))continue;const target=path.resolve(path.dirname(file),lit.text),map=maps.get(target);if(!map)continue;const bindings=ts.isImportDeclaration(n)?n.importClause?.namedBindings:n.exportClause;
if(bindings&&(ts.isNamedImports(bindings)||ts.isNamedExports(bindings))){const groups=new Map<string,string[]>();for(const item of bindings.elements){const key=map.get((item.propertyName??item.name).text)??target;const group=groups.get(key)??[];group.push(item.getText());groups.set(key,group)}edits.push({start:n.getStart(ast),end:n.end,value:[...groups].map(([key,items])=>`${ts.isImportDeclaration(n)?"import":"export"}${ts.isImportDeclaration(n)&&n.importClause?.isTypeOnly?" type":""} {${items.join(",")}} from ${JSON.stringify(rel(file,key))};`).join("\n")});}
else if(bindings&&ts.isNamespaceImport(bindings)){const id=bindings.name.text,targets=new Map<string,string>();const visit=(node:ts.Node)=>{if(ts.isPropertyAccessExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text===id&&map.has(node.name.text)){const key=map.get(node.name.text)!,alias=targets.get(key)??id+"Io"+targets.size;targets.set(key,alias);edits.push({start:node.expression.getStart(ast),end:node.expression.end,value:alias})}ts.forEachChild(node,visit)};visit(ast);for(const [key,alias]of targets)extra.push(`import * as ${alias} from ${JSON.stringify(rel(file,key))};`);}}
let next=source;for(const edit of edits.sort((a,b)=>b.start-a.start))next=next.slice(0,edit.start)+edit.value+next.slice(edit.end);if(extra.length)next=extra.join("\n")+"\n"+next;if(next!==source)put(file,next);}
for(const source of txtFiles){if(!fs.existsSync(source)||source.includes("🧪️tests"))continue;put(source,fs.readFileSync(source,"utf8").replace(/^import \{TxtProtobufReader[^\n]*\n/m,""));}
const previous=path.join(ticket,"typescript-extraction-files.md");fs.appendFileSync(previous,"\n## Completed Facet And Schema Separation\n\n"+[...changed].sort().map(f=>"- `"+path.relative(root,f)+"`").join("\n")+"\n");
console.log(`[DEBUG] TypeScript facet and schema boundaries changed=${changed.size}`);
