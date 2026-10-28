/** 🧂️ Contract: the regular expression literals the hot graph modules matched once per path, per character or per line are written at module level only, so no call builds a new RegExp object (a graph build evaluated them 5 million times); the check reads each module with the TypeScript parser and fails for a listed literal inside any function. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import Ajv from"ajv";
import ts from"typescript";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";

const build=join(import.meta.dir,"..","..");

test("the fixture is closed and valid against its schema",()=>{
 const valid=new Ajv({strict:true,allErrors:true}).compile(fixtureSchema);
 expect(valid(fixture),JSON.stringify(valid.errors)).toBe(true);
});

for(const file of[...new Set(fixture.literals.map(row=>row.file))])test(`${file}: the listed literals are written at module level only`,()=>{
 const text=readFileSync(join(build,file),"utf8"),source=ts.createSourceFile(file,text,ts.ScriptTarget.Latest,true,file.endsWith(".ts")?ts.ScriptKind.TS:ts.ScriptKind.JS);
 const wanted=new Set(fixture.literals.filter(row=>row.file===file).map(row=>row.literal)),seen=new Map<string,{module:number;inside:number}>();
 const visit=(node:ts.Node)=>{
  if(node.kind===ts.SyntaxKind.RegularExpressionLiteral&&wanted.has(node.getText())){
   let up:ts.Node|undefined=node.parent,inside=false;while(up){if(ts.isFunctionLike(up)){inside=true;break;}up=up.parent;}
   const row=seen.get(node.getText())??{module:0,inside:0};if(inside)row.inside++;else row.module++;seen.set(node.getText(),row);
  }
  ts.forEachChild(node,visit);
 };
 visit(source);
 for(const literal of wanted){
  const row=seen.get(literal);
  expect(row,`${literal} is written at module level`).toBeDefined();
  expect(row!.inside,`${literal} is evaluated inside a function`).toBe(0);
  expect(row!.module,`${literal} is written at module level`).toBeGreaterThan(0);
 }
});
