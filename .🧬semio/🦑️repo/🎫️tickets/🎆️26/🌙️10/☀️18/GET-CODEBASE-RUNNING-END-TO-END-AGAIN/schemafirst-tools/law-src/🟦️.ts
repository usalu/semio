/** 🧬️ Contract: the policy constants of a module live in its schema file (the single source) and its sources hold none of their values as a literal; every constant is described in English and German, the frozen policy objects equal the schema const, and a hand-written Python lexer finds exactly the literals the TypeScript parser finds. Each module is one directory `<emoji><module>/🔣️.json` next to this file. */
import{expect,test}from"bun:test";
import{existsSync,readdirSync,readFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{pathToFileURL}from"node:url";
import{spawnSync}from"node:child_process";
import Ajv from"ajv";
import ts from"typescript";
import rowSchema from"./🧬️schema/🔣️.json";

type Row={version:1;root:"library"|"process"|"repository";schema:string;container?:"definitions"|"$defs";document?:string;pointer?:string[];definitions:string[];sources:string[];exports?:{source:string;name:string;definition:string}[];minimumNumber?:number;minimumText?:number;allow?:{source:string;value:number|string;reason:string}[]};
type Literal={line:number;kind:"number"|"string";value:number|string;specifier?:true};
const roots={library:"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",process:"🧰️framework/🔨️modules/🏃️process",repository:""};
const here=import.meta.dir,oracle=join(here,"🐍️.py");
const repository=(()=>{let at=here;while(!existsSync(join(at,"🧰️framework"))&&dirname(at)!==at)at=dirname(at);return at;})();
const python=(()=>{try{return spawnSync("python3",["-I","-c","print(1)"]).status===0?"python3":"python";}catch{return"python";}})();
const rowFiles=readdirSync(here,{withFileTypes:true}).filter(entry=>entry.isDirectory()&&entry.name!=="🧬️schema"&&existsSync(join(here,entry.name,"🔣️.json"))).map(entry=>join(here,entry.name,"🔣️.json")).sort();
const rowOf=(file:string)=>JSON.parse(readFileSync(file,"utf8")) as Row;
const baseOf=(row:Row)=>join(repository,roots[row.root]);
const schemaOf=(row:Row)=>{const document=JSON.parse(readFileSync(join(baseOf(row),row.schema),"utf8")) as Record<string,Record<string,{const:Record<string,unknown>}>>;return{definitions:document[row.container??"definitions"]!};};

/** 🔎️ Every numeric and string literal of a source, read with the TypeScript parser; module specifiers are marked because they are no literals of the code. */
export function literalsOfText(text:string,script:"ts"|"js"):Literal[]{
 const source=ts.createSourceFile("source",text,ts.ScriptTarget.Latest,true,script==="ts"?ts.ScriptKind.TS:ts.ScriptKind.JS),found:Literal[]=[];
 const visit=(node:ts.Node)=>{
  const line=source.getLineAndCharacterOfPosition(node.getStart(source)).line+1;
  if(ts.isNumericLiteral(node))found.push({line,kind:"number",value:Number(node.text.replace(/_/gu,""))});
  else if(ts.isStringLiteral(node)||ts.isNoSubstitutionTemplateLiteral(node))found.push({line,kind:"string",value:node.text,...ts.isImportDeclaration(node.parent)||ts.isExportDeclaration(node.parent)||ts.isCallExpression(node.parent)&&node.parent.expression.kind===ts.SyntaxKind.ImportKeyword?{specifier:true as const}:{}});
  ts.forEachChild(node,visit);
 };
 visit(source);return found;
}
/** 🧮️ The values of a policy a source must not repeat: numbers anywhere from `minimumNumber` up, and the texts of top-level strings and string lists from `minimumText` characters on (the words of a closed vocabulary inside a table are names, not constants). */
export function forbiddenOf(constants:readonly Record<string,unknown>[],minimumNumber=100,minimumText=5):{numbers:Set<number>;strings:Set<string>}{
 const numbers=new Set<number>(),strings=new Set<string>(),stack:unknown[]=[...constants],text=(item:unknown)=>{if(typeof item==="string"&&item.length>=minimumText)strings.add(item);};
 for(const constant of constants)for(const value of Object.values(constant)){text(value);if(Array.isArray(value))value.forEach(text);}
 while(stack.length){
  const item=stack.pop();
  if(typeof item==="number"&&Math.abs(item)>=minimumNumber)numbers.add(item);
  else if(Array.isArray(item))stack.push(...item);
  else if(item!==null&&typeof item==="object")stack.push(...Object.values(item));
 }
 return{numbers,strings};
}
const constantsOf=(row:Row):Record<string,unknown>[]=>row.document?[(row.pointer??[]).reduce((value:any,key)=>value[key],JSON.parse(readFileSync(join(baseOf(row),row.document),"utf8")))]:row.definitions.map(name=>schemaOf(row).definitions[name]!.const);
const findingsOf=(row:Row)=>{
 const forbidden=forbiddenOf(constantsOf(row),row.minimumNumber,row.minimumText),allowed=new Set((row.allow??[]).map(entry=>`${entry.source}\0${JSON.stringify(entry.value)}`)),found:{source:string;line:number;kind:string;value:number|string}[]=[];
 for(const source of row.sources)for(const literal of literalsOfText(readFileSync(join(baseOf(row),source),"utf8"),source.endsWith(".ts")?"ts":"js")){
  const hit=literal.kind==="number"?forbidden.numbers.has(literal.value as number):!literal.specifier&&forbidden.strings.has(literal.value as string);
  if(hit&&!allowed.has(`${source}\0${JSON.stringify(literal.value)}`))found.push({source,line:literal.line,kind:literal.kind,value:literal.value});
 }
 return found;
};

test("the rows are closed and valid against their schema",()=>{
 const valid=new Ajv({strict:true,allErrors:true}).compile(rowSchema);
 for(const file of rowFiles)expect(valid(rowOf(file)),`${file}: ${JSON.stringify(valid.errors)}`).toBe(true);
});

test("the TypeScript parser and the Python lexer read the same literals",()=>{
 const cases=["const a=0x7f+1_000+.5+0o600+1e3;/* 99999 */ const s=\"x\\u0041\\n\"+`t${1234+`n${5678}`}u`;const r=/a[/]9999\\//g;const d=b/2/3;x=`only`; // 777\n","import x from \"./🧬️schema/🔣️.json\" with {type:\"json\"};export const y=[...{a:4096},'q\\'z',`${a}${{b:300}.b}`];\nconst w=a?.b??64n;\n","const t=(a)/100/200,u=[1]/300;return /x/.test(\"y\")?4000:5000;"];
 for(const text of cases){
  const run=spawnSync(python,["-I",oracle,"--literals"],{input:text,encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
  expect(JSON.parse(run.stdout) as Literal[],text).toEqual(literalsOfText(text,"js").map(literal=>({line:literal.line,kind:literal.kind,value:literal.value})));
 }
});

test("a policy value written as a literal is a finding and an allowed one is not",()=>{
 const forbidden=forbiddenOf([{limit:600000,name:"graph-memo",names:["abcdef","ab"],table:{word:"vocabulary"},nested:{list:[2048]}}]);
 expect([...forbidden.numbers].sort()).toEqual([2048,600000]);expect([...forbidden.strings].sort()).toEqual(["abcdef","graph-memo"]);
 const hits=literalsOfText("const a=600000,b=\"graph-memo\",c=99,d=2048+1;","ts").filter(literal=>forbidden.numbers.has(literal.value as number)||forbidden.strings.has(literal.value as string));
 expect(hits.map(hit=>hit.value)).toEqual([600000,"graph-memo",2048]);
});

for(const file of rowFiles){
 const row=rowOf(file),name=file.split("/").at(-2)!;
 test(`${name}: every policy constant is described in English and German and the policy values are closed under Ajv`,()=>{
  const schema=schemaOf(row),ajv=new Ajv({strict:true,allErrors:true});
  for(const definition of row.definitions){
   if(row.document){
    const values=constantsOf(row)[0]!,properties=(schema.definitions[definition] as unknown as{properties:Record<string,{description?:string}>}).properties;
    expect(Object.keys(properties).sort(),`${definition} properties`).toEqual(Object.keys(values).sort());
    for(const[key,property]of Object.entries(properties)){const match=/^EN: (.+) DE: (.+)$/su.exec(property.description??"");expect(match,`${definition}.${key} description`).not.toBeNull();expect(match![1]!.length,`${definition}.${key} en`).toBeGreaterThan(10);expect(match![2]!.length,`${definition}.${key} de`).toBeGreaterThan(10);expect(match![2],`${definition}.${key} de differs`).not.toBe(match![1]);}
    continue;
   }
   const values=schema.definitions[definition]!.const,described=schema.definitions[`${definition}Descriptions`]?.const as{en:Record<string,string>;de:Record<string,string>}|undefined;
   expect(described,`${definition}Descriptions`).toBeDefined();
   const keys=Object.keys(values).sort();
   expect(Object.keys(described!.en).sort(),`${definition} en`).toEqual(keys);expect(Object.keys(described!.de).sort(),`${definition} de`).toEqual(keys);
   for(const key of keys){expect(described!.en[key]!.length,`${definition}.${key} en`).toBeGreaterThan(10);expect(described!.de[key]!.length,`${definition}.${key} de`).toBeGreaterThan(10);expect(described!.de[key],`${definition}.${key} de differs`).not.toBe(described!.en[key]);}
   const check=ajv.compile({type:"object",minProperties:1,additionalProperties:{anyOf:[{type:"integer",minimum:0},{type:"number",exclusiveMinimum:0},{type:"string",minLength:1},{type:"boolean"},{type:"array",items:{anyOf:[{type:"string"},{type:"integer"}]}},{type:"object"}]}});
   expect(check(values),JSON.stringify(check.errors)).toBe(true);
   const text=ajv.compile({type:"object",required:["en","de"],additionalProperties:false,properties:{en:{type:"object",additionalProperties:{type:"string",minLength:1}},de:{type:"object",additionalProperties:{type:"string",minLength:1}}}});
   expect(text(described),JSON.stringify(text.errors)).toBe(true);
  }
 });
 test(`${name}: the frozen policy objects equal the schema const`,async()=>{
  const schema=schemaOf(row);
  for(const entry of row.exports??[]){
   const exported=(await import(pathToFileURL(join(baseOf(row),entry.source)).href))[entry.name] as Record<string,unknown>;
   expect(Object.isFrozen(exported),entry.name).toBe(true);expect(exported,entry.name).toEqual(schema.definitions[entry.definition]!.const);
  }
 });
 test(`${name}: the sources hold no policy value as a literal, and the Python lexer agrees`,()=>{
  const mine=findingsOf(row);
  const run=spawnSync(python,["-I",oracle,repository,file],{encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);
  expect((JSON.parse(run.stdout) as Record<string,unknown>)[file],"oracle").toEqual(mine);
  expect(mine,JSON.stringify(mine)).toEqual([]);
 });
}
