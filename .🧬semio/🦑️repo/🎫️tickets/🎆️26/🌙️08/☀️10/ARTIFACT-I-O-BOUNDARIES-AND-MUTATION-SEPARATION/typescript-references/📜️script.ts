import fs from "node:fs";
import path from "node:path";
import {execFileSync} from "node:child_process";
import ts from "typescript";
const root=process.cwd(),ticket=path.dirname(import.meta.dir),files=execFileSync("rg",["--files","-g","*.ts","-g","*.tsx"],{encoding:"utf8",maxBuffer:30_000_000}).trim().split("\n").map(f=>path.resolve(root,f));
const rel=(file:string,target:string)=>{const value=path.relative(path.dirname(file),target);return value.startsWith(".")?value:"./"+value};
const map=new Map<string,Map<string,string>>(),physical=/^(?:.*(?:To|From)SqliteDatabase|.*SQLITE_SCHEMA|validate.*SqliteDialect)$/;
for(const file of files){const at=file.indexOf("/🪆️subsets/");if(at<0||!file.includes("/🚪️io/🪶️sqlite/")||!file.endsWith("/🟦️.ts")||file.includes("/🧪️tests/"))continue;
const own=file.slice(0,file.indexOf("/",at+"/🪆️subsets/".length)),source=fs.readFileSync(file,"utf8"),ast=ts.createSourceFile(file,source,ts.ScriptTarget.Latest,true),symbols=new Set<string>();
for(const node of ast.statements){if(ts.isExportDeclaration(node)&&node.exportClause&&ts.isNamedExports(node.exportClause))for(const item of node.exportClause.elements)symbols.add(item.name.text);else if(node.modifiers?.some(m=>m.kind===ts.SyntaxKind.ExportKeyword)){if(ts.isVariableStatement(node))for(const d of node.declarationList.declarations)symbols.add(d.name.getText());else if("name"in node&&node.name)symbols.add(node.name.getText())}}
for(const facet of ["","📸️snapshot/","🔺️diff/","🧬️mutations/"]){const key=path.join(own,"🧬️schema",facet,"🟦️.ts"),members=map.get(key)??new Map<string,string>();for(const name of symbols)if(physical.test(name)&&(!members.has(name)||file===path.join(own,"🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts")))members.set(name,file);map.set(key,members)}
}
const changed:string[]=[];
for(const file of files){const source=fs.readFileSync(file,"utf8");if(!source.includes("schema")&&!source.includes("owner"))continue;const ast=ts.createSourceFile(file,source,ts.ScriptTarget.Latest,true),edits:{start:number,end:number,value:string}[]=[],imports:string[]=[];
for(const node of ast.statements){if(!ts.isImportDeclaration(node)&&!ts.isExportDeclaration(node))continue;const literal=node.moduleSpecifier;if(!literal||!ts.isStringLiteral(literal)||!literal.text.startsWith("."))continue;const target=path.resolve(path.dirname(file),literal.text),members=map.get(target);if(!members?.size)continue;const bindings=ts.isImportDeclaration(node)?node.importClause?.namedBindings:node.exportClause;
if(bindings&&(ts.isNamedImports(bindings)||ts.isNamedExports(bindings))){if(!bindings.elements.some(item=>members.has((item.propertyName??item.name).text)))continue;const groups=new Map<string,string[]>();for(const item of bindings.elements){const to=members.get((item.propertyName??item.name).text)??target,group=groups.get(to)??[];group.push(item.getText());groups.set(to,group)}edits.push({start:node.getStart(ast),end:node.end,value:[...groups].map(([to,items])=>`${ts.isImportDeclaration(node)?"import":"export"} {${items.join(",")}} from ${JSON.stringify(rel(file,to))};`).join("\n")});}
else if(bindings&&ts.isNamespaceImport(bindings)){const id=bindings.name.text,routes=new Map<string,string>();const route=(to:string)=>{const alias=routes.get(to)??id+"Sqlite"+routes.size;routes.set(to,alias);return alias};const visit=(n:ts.Node)=>{
 if(ts.isPropertyAccessExpression(n)&&ts.isIdentifier(n.expression)&&n.expression.text===id&&members.has(n.name.text))edits.push({start:n.expression.getStart(ast),end:n.expression.end,value:route(members.get(n.name.text)!)});
 if(ts.isCallExpression(n)&&n.expression.getText(ast)==="Reflect.get"&&n.arguments.length===2&&n.arguments[0]?.getText(ast)===id&&ts.isStringLiteral(n.arguments[1]!)&&members.has((n.arguments[1]as ts.StringLiteral).text))edits.push({start:n.arguments[0]!.getStart(ast),end:n.arguments[0]!.end,value:route(members.get((n.arguments[1]as ts.StringLiteral).text)!)});
 ts.forEachChild(n,visit)};visit(ast);for(const[to,alias]of routes)imports.push(`import * as ${alias} from ${JSON.stringify(rel(file,to))};`);}
}
if(!edits.length)continue;let next=source;for(const edit of edits.sort((a,b)=>b.start-a.start))next=next.slice(0,edit.start)+edit.value+next.slice(edit.end);fs.writeFileSync(file,imports.join("\n")+(imports.length?"\n":"")+next);changed.push(path.relative(root,file));
}
fs.appendFileSync(path.join(ticket,"typescript-extraction-files.md"),"\n## Direct SQLite Consumer Closure\n\n"+changed.map(f=>"- `"+f+"`").join("\n")+"\n");
console.log(`[DEBUG] Direct SQLite TS consumer references updated=${changed.length}`);
