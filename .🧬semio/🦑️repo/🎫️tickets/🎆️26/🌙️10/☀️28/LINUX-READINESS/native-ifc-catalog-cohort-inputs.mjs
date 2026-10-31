import fs from "node:fs/promises";
import path from "node:path";
import ts from "typescript";
import {renderRustCatalog} from "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️catalog/🛠️extract/🦀️render.ts";
const ifc="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc",owner=ifc+"/📚️catalog/🛠️extract",law=owner+"/🧪️tests/🦀️native/🟦️.ts",corpusPath=path.dirname(law)+"/🔣️.json",policyPath="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json";
const corpus=JSON.parse(await fs.readFile(corpusPath,"utf8")),policy=JSON.parse(await fs.readFile(policyPath,"utf8")),sources=new Set(),resources=new Set([corpusPath,policyPath]);
async function source(file){file=path.normalize(file);if(sources.has(file))return;sources.add(file);const ast=ts.createSourceFile(file,await fs.readFile(file,"utf8"),ts.ScriptTarget.Latest,true);for(const node of ast.statements){if(!ts.isImportDeclaration(node)&&!ts.isExportDeclaration(node)||!node.moduleSpecifier||!ts.isStringLiteral(node.moduleSpecifier))continue;const specifier=node.moduleSpecifier.text;if(!specifier.startsWith("."))continue;const target=path.normalize(path.join(path.dirname(file),specifier));if(target.endsWith(".ts"))await source(target);else if(target.endsWith(".json"))resources.add(target);}}
await source(law);
for(const name of (await fs.readdir(owner)).filter(name=>name.endsWith(".ts")))await source(owner+"/"+name);
let config=owner+"/tsconfig.json";
while(config){resources.add(config);const parsed=ts.parseConfigFileTextToJson(config,await fs.readFile(config,"utf8"));if(parsed.error)throw Error("Actual compiler config required");config=parsed.config.extends?path.normalize(path.join(path.dirname(config),parsed.config.extends)):null;}
for(const row of corpus.catalogs){const base=ifc+"/"+row.path;const document=base+"/🔣️.json";resources.add(document);resources.add(base+"/data.rs");const catalog=JSON.parse(await fs.readFile(document,"utf8"));for(const component of renderRustCatalog(catalog,{maximumSourceBytes:policy.storage.sourceBoundary.maxSourceBytes}).components)resources.add(base+"/🧩️components/"+component.folder+"/🦀️.rs");}
const runtimeSource=[...sources],freshness=[...new Set([...resources,...runtimeSource])],allExist=(await Promise.all(freshness.map(file=>fs.stat(file).then(()=>true,()=>false)))).every(Boolean);
if(!allExist||resources.size>64)throw Error("Complete bounded actual catalog cohort required");
const result={at:new Date().toISOString(),law,resources:[...resources],runtimeSource,freshness,allExist,maxAbsolutePath:Math.max(...freshness.map(file=>path.resolve(file).length)),tests:6};
await fs.writeFile(new URL("🗑️generated/native-ifc-catalog-cohort-inputs.json",import.meta.url),JSON.stringify(result,null,2)+"\n");
console.log(JSON.stringify({law,resources:resources.size,runtimeSource:sources.size,allExist,maxAbsolutePath:result.maxAbsolutePath}));
