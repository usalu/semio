import {expect,test} from "bun:test";
import {readFileSync,existsSync} from "node:fs";
import {resolve,join,dirname} from "node:path";
import Ajv from "ajv";
import JSON5 from "json5";
import ts from "typescript";
import {build} from "esbuild";
import {validateJsonSchemaSubset} from "../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";

const inputs=resolve(import.meta.dir,".."),root=resolve(inputs,"../../../../../../../.."),schema=JSON.parse(readFileSync(join(inputs,"schema.json"),"utf8")),corpus=JSON.parse(readFileSync(join(inputs,"corpus.json"),"utf8"));
const custody=JSON.parse(readFileSync(join(inputs,"custody.json"),"utf8")) as {paths:string[]};
const current=(path:string)=>path.replace("🧪️semio-tech-s-2d-js","🧪️semio-tech-framework-2d-js").replace("🧪️semio-tech-geometry-brep-js","🧪️semio-tech-framework-3d-js");
const source=(path:string)=>readFileSync(join(root,path),"utf8"),oldNames=["2d","3d"].map(dimension=>`@semio-tech/${["s",dimension,"js"].join("-")}`);
function edges(text:string,path:string):string[]{
  const ast=ts.createSourceFile(path,text,ts.ScriptTarget.Latest,true,path.endsWith(".tsx")?ts.ScriptKind.TSX:ts.ScriptKind.TS),result:string[]=[];
  const visit=(node:ts.Node):void=>{if((ts.isImportDeclaration(node)||ts.isExportDeclaration(node))&&node.moduleSpecifier&&ts.isStringLiteralLike(node.moduleSpecifier))result.push(node.moduleSpecifier.text);if(ts.isCallExpression(node)&&(node.expression.kind===ts.SyntaxKind.ImportKeyword||ts.isIdentifier(node.expression)&&node.expression.text==="require")&&node.arguments[0]&&ts.isStringLiteralLike(node.arguments[0]))result.push(node.arguments[0].text);ts.forEachChild(node,visit);};visit(ast);return result;
}

test("variable neutral package projections agree with independent JSON Schema validation",()=>{
  const oracle=new Ajv({strict:true}).compile(schema);
  for(const row of corpus.vectors){expect(validateJsonSchemaSubset(schema,row.projection).length===0,row.id).toBe(row.accepted);expect(oracle(row.projection),row.id).toBe(row.accepted);}
});

for(const [dimension,folder,entry] of [["2d","◻️2d","../../🟦️.ts"],["3d","🧊️3d","./🟦️.ts"]])test(`actual ${dimension} package publishes its canonical defining entry`,async()=>{
  const owner=`🧰️framework/🔨️modules/${folder}/📦️packages/🟦️typescript`,manifest=JSON.parse(source(`${owner}/package.json`)),project=JSON.parse(source(`${owner}/📋️project.json`)),name=`@semio-tech/framework-${dimension}-js`;
  const projection={name:manifest.name,nxName:project.name,entry:manifest.exports["."],moduleEdges:edges(source(`🧰️framework/🔨️modules/${folder}/🟦️.ts`),"source.ts")};
  expect(validateJsonSchemaSubset(schema,projection)).toEqual([]);expect(new Ajv().compile(schema)(projection)).toBe(true);expect(projection.name).toBe(name);expect(projection.nxName).toBe(name);expect(projection.entry).toBe(entry);expect(existsSync(resolve(root,owner,projection.entry))).toBe(true);expect(existsSync(resolve(root,owner,manifest.$schema))).toBe(true);expect(manifest.repository.directory).toBe(owner);expect(manifest.scripts.test).toBe(`bun nx run ${name}:test`);
  const bundle=await build({entryPoints:[resolve(root,owner,projection.entry)],bundle:true,write:false,metafile:true,platform:"node",format:"esm",define:{"import.meta.vitest":"false"}});expect(Object.keys(bundle.metafile!.inputs).filter(input=>input.includes("🛍️products"))).toEqual([]);expect(Object.values(bundle.metafile!.outputs).flatMap(output=>output.imports)).toEqual([]);
});

test("all actual callers, manifests, naming folders, launch rows and workspace locators bind canonical package identities",()=>{
  let callers=0,dependencies=0,locks=0;
  for(const original of custody.paths){const path=current(original),text=source(path);for(const name of oldNames)expect(text.includes(name),path).toBe(false);if(path.endsWith("package.json")&&path.startsWith("✏️s/")){const manifest=JSON.parse(text);expect(manifest.dependencies["@semio-tech/framework-3d-js"],path).toBe("workspace:*");dependencies++;}if(path.endsWith("bun.lock")){const lock=JSON5.parse(text);for(const name of ["@semio-tech/framework-2d-js","@semio-tech/framework-3d-js"]){expect(lock.packages[name]?.[0].startsWith(`${name}@workspace:`),path).toBe(true);}locks++;}if(/\.tsx?$/u.test(path))callers+=edges(text,path).filter(edge=>edge==="@semio-tech/framework-3d-js").length;}
  expect(dependencies).toBe(21);expect(callers).toBe(31);expect(locks).toBe(6);
  for(const [folder,old,newName] of [["◻️2d","🧪️semio-tech-s-2d-js","🧪️semio-tech-framework-2d-js"],["🧊️3d","🧪️semio-tech-geometry-brep-js","🧪️semio-tech-framework-3d-js"]]){expect(existsSync(join(root,`🧰️framework/🔨️modules/${folder}/🧪️tests/${old}`))).toBe(false);expect(existsSync(join(root,`🧰️framework/🔨️modules/${folder}/🧪️tests/${newName}/🟦️.ts`))).toBe(true);}
});

for(const [owner,implementation,required] of [["🎠️kernel","🟦️typescript","runVitestV1"],["🧵️job","🦀️rust","runBudgetedTestCommand"]])test(`${owner} executable imports the defining General runner and bundles without product inputs`,async()=>{
  const path=`🧰️framework/🔨️modules/${owner}/📦️packages/${implementation}/📜️script.ts`,text=source(path),imports=edges(text,path);
  expect(imports.filter(edge=>edge.includes("🛍️products"))).toEqual([]);expect(text.includes(required)).toBe(true);
  const bundled=await build({entryPoints:[join(root,path)],bundle:true,write:false,metafile:true,platform:"node",format:"esm",packages:"external",define:{"import.meta.vitest":"false"},plugins:[{name:"product-boundary",setup(builder){builder.onResolve({filter:/.*/},args=>{if(args.path.includes("🛍️products"))return {errors:[{text:`product executable import refused: ${args.path}`} ]};return undefined;});}}]});
  expect(Object.keys(bundled.metafile!.inputs).filter(input=>input.includes("🛍️products"))).toEqual([]);
},120000);
