import {createRequire} from "node:module";import {writeFileSync} from "node:fs";
const lib="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const ts=createRequire(lib+"🟨️.mjs")("typescript");
const cases={
"static-import":`import a from "./a.ts";\nimport {b,c as d} from './b.ts';\nimport * as e from "./e.mjs";\nimport "./side.ts";\nimport f,{g} from "pkg-f";\nimport h,* as i from "pkg-h";`,
"type-only-imports":`import type {A} from "./types.ts";\nimport type B from "./b-types.ts";\nimport {type C} from "./c-types.ts";\nimport type * as D from "./d-types.ts";\nimport type from "./type-default.ts";\nexport type {E} from "./e-types.ts";\nexport type * from "./all-types.ts";`,
"re-exports":`export * from "./star.ts";\nexport * as ns from "./ns.ts";\nexport {x,y as z} from "./named.ts";\nexport {local};\nexport const value=1;\nexport import q=require("./exported-require.cjs");`,
"dynamic-import":`const a=await import("./dyn-a.ts");\nconst b=import('./dyn-b.ts');\nconst c=import(\`./dyn-c.ts\`);\nconst d=import(name);\nconst e=import(\`./dyn-\${x}.ts\`);\nobj.import("./not-an-import.ts");\nobj?.import("./optional-chain.ts");`,
"require-calls":`const a=require("./req-a.cjs");\nconst b=require('req-b');\nconst c=require(\`req-c\`);\nconst d=createRequire(import.meta.url)("typescript");\nimport e=require("./import-equals.cjs");\nx.require("./member-require.cjs");`,
"comments":`// import "./line-comment.ts";\n/* import "./block-comment.ts"; */\n/** import {x} from "./doc-comment.ts" */\nimport "./real.ts";\n// new URL("./comment-url.txt", import.meta.url)`,
"strings":`const a="import './in-string.ts'";\nconst b='export * from "./in-single.ts"';\nconst c="escaped \\"import 'x'\\" quote";\nimport "./after-strings.ts";\nconst d="a\\u0062c";\nimport "./esc\\u0061ped.ts";`,
"template-literals":"const a=`import \"./in-template.ts\"`;\nconst b=`outer ${import(\"./in-substitution.ts\")} tail`;\nconst c=`a ${`nested ${import(\"./nested.ts\")}`} z`;\nconst d=`${ {k:1}.k }`;\nimport \"./after-template.ts\";",
"regex-literals":`const a=/["']/g;\nconst b=/\\/import ".\\/in-regex.ts"/;\nconst c=x/2/3;\nimport "./after-regex.ts";\nconst d=/[/]\\//;`,
"regex-quote-to-line-end":`const r=/'/;\nimport "./hidden-by-regex-quote.ts";\nconst s='';`,
"url-references":`const a=new URL("./data.json",import.meta.url);\nconst b=new URL("../dir/",import.meta.url);\nconst c=new URL('./single.txt',import.meta.url);\nconst d=new URL(\`./tpl.txt\`,import.meta.url);\nconst e=new URL("https://example.test/x",import.meta.url);\nconst f=new URL("not-relative.txt",import.meta.url);\nconst g=new URL("./other.txt",base);\nconst h=new URL("./three.txt",import.meta.url,1);\nconst i=new URL("./trailing-comma.txt",import.meta.url,);\nconst j=new URL("./spaced.txt", import . meta . url);`,
"url-in-trivia":`// new URL("./line.txt",import.meta.url)\n/* new URL("./block.txt",import.meta.url) */\nconst s='new URL("./string.txt",import.meta.url)';\nconst t=\`new URL("./template.txt",import.meta.url)\`;\nconst r=/new URL\\("x"\\)/;\nconst u=new URL("./real.txt",import.meta.url);`,
"url-in-template-substitution":"const a=`${new URL(\"./in-sub.txt\",import.meta.url)}`;\nconst b=`x ${ `y ${new URL(\"./nested-sub.txt\",import.meta.url)}` }`;",
"declare-module":`declare module "ambient-top" { export const a:number; }\nimport "./local.ts";\nnamespace N { declare module "ambient-nested" {} }`,
"declare-module-script":`declare module "ambient-script-top" { export const a:number; }\nnamespace N { declare module "ambient-script-nested" {} }`,
"amd-define":`define(["amd-a","./amd-b"],function(a,b){});\ndefine("named",["amd-c"],function(c){});\ndefine(function(){});`,
"shebang-and-bom":`#!/usr/bin/env bun\nimport "./after-shebang.ts";`,
"jsx-like-and-generics":`const a=<T,>(x:T)=>x;\nconst b=a<number>(1);\nimport "./after-generics.ts";\nlet c=1<2>3;`,
"numbers-and-operators":`const a=0x1F+1_000+.5+1e3+10n;\nconst b=a**2>>1>>>2;\nconst c=x??y?.z;\nimport "./after-numbers.ts";`,
"unterminated":`import "./first.ts";\nconst s="never closed\nimport "./after-unterminated-string.ts";`,
"empty":``,
"duplicates":`import "./dup.ts";\nimport "./dup.ts";\nexport * from "./dup.ts";\nconst u=new URL("./dup.txt",import.meta.url);\nconst v=new URL("./dup.txt",import.meta.url);`,
"unicode-identifiers":`const héllo=1;\nconst 日本=2;\nconst 😀="emoji";\nimport "./after-unicode.ts";`,
"conflict-marker":`<<<<<<< ours\nimport "./ours.ts";\n=======\nimport "./theirs.ts";\n>>>>>>> theirs\nimport "./after-conflict.ts";`,
"export-import-equals":`export import a=require("./exp-import-equals.cjs");\nexport import type b=require("./exp-type-equals.cjs");`,
"import-meta-and-keywords":`const u=import.meta.url;\nconst d=import.meta.dirname;\nconst from=1,as=2,type=3,require=4;\nimport {from as f2} from "./keyword-names.ts";`
};
const out=[];
for(const [name,source] of Object.entries(cases)){
 const references=[],seen=new Set(),add=(kind,specifier)=>{const id=kind+"\0"+specifier;if(!seen.has(id)){seen.add(id);references.push({kind,specifier});}};
 for(const row of ts.preProcessFile(source,true,true).importedFiles)add("module",row.fileName);
 if(source.includes("URL")){const sf=ts.createSourceFile("engine.ts",source,ts.ScriptTarget.Latest,false),visit=node=>{
  if(ts.isNewExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="URL"&&node.arguments?.length===2&&ts.isStringLiteralLike(node.arguments[0])&&node.arguments[0].text.startsWith(".")&&node.arguments[1].getText(sf)==="import.meta.url"){const s=node.arguments[0].text;add(new URL(s,"file:///engine.ts").pathname.endsWith("/")?"directory-url":"file-url",s);}
  ts.forEachChild(node,visit);};visit(sf);}
 out.push({name,source,references});
}
writeFileSync(process.argv[2],JSON.stringify(out,null,1)+"\n");
for(const c of out)console.log(c.name.padEnd(30),c.references.map(r=>(r.kind[0]==="m"?"":r.kind[0]+":")+r.specifier).join(" | "));
