import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, resolve, relative } from "node:path";
import { createHash } from "node:crypto";
import assert from "node:assert/strict";
import Parser from "web-tree-sitter";
import ts from "typescript";
import TOML from "@iarna/toml";

const ticket=dirname(import.meta.dir), root=resolve(ticket,"../../../../../../.."), output=resolve(ticket,"🗑️generated/graph-os-record-cut"), epoch=process.argv[2];
assert.ok(epoch&&/^[a-z0-9-]+$/.test(epoch));
const target=resolve(output,`independent-actual-library-alias-residual-${epoch}.json`);assert.ok(!existsSync(target));
const hash=(source:string)=>createHash("sha256").update(source).digest("hex");
const helperPath=resolve(ticket,"graph-os-record-cut-proposal/📜️script.ts"), helper=readFileSync(helperPath,"utf8"), prefix=helper.slice(helper.indexOf("function flattenRustUse("),helper.indexOf("const walk ="));
const lexical=new Function("assert",ts.transpileModule(prefix+"\nreturn {flattenRustUse,rustLexicalAliases,resolveRustAlias,canonicalTokenPathEdits,lexicalScopeCache};",{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText)(assert);
const census=JSON.parse(readFileSync(resolve(output,"parsed-owned-caller-census-4.json"),"utf8")), raw=JSON.parse(readFileSync(resolve(output,"full-caller-manifest-census-2.json"),"utf8"));
const names=new Set<string>(Object.values(census.taxonomy).flat() as string[]), rows=JSON.parse(readFileSync(resolve(output,"publication-21-production-12/publication.json"),"utf8")).rows;
await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",root)),"out/tree-sitter-rust.wasm")));
const libraries:any[]=[], frames:any[]=[];
for(const captured of raw.manifests){
 const path=resolve(root,captured.path);if(!existsSync(path))continue;const source=readFileSync(path,"utf8"), table:any=TOML.parse(source);assert.deepEqual(Bun.TOML.parse(source),table);
 const library=resolve(dirname(path),table.lib?.path??"src/lib.rs");if(!existsSync(library))continue;
 const body=readFileSync(library,"utf8"), tree=parser.parse(body)!, aliases=new Map(lexical.rustLexicalAliases(tree.rootNode));
 for(const [name,binding] of aliases as Map<string,string[]|null>)if(binding?.[0]==="self")aliases.set(name,[table.package.name.replaceAll("-","_"),...binding.slice(1)]);
 const mounts=[...body.matchAll(/#\[path\s*=\s*"([^"\n]+)"\]/g)].map(match=>resolve(dirname(library),match[1])).filter(path=>existsSync(path));
 libraries.push({manifest:captured.path,library,package:table.package.name,aliases,mounts});frames.push({path:captured.path,source,inverse:source,sha256:hash(source)},{path:relative(root,library),source:body,inverse:body,sha256:hash(body)});tree.delete();
}
const originalPaths = new Set(rows.map((row:any)=>row.path));
const selector = new RegExp("\\b(?:"+[...names].filter(name=>/^[A-Za-z_][A-Za-z_0-9]*$/.test(name)).join("|")+")\\b");
const extraPaths:string[]=[];
for(const directory of ["🧰️framework","✏️s"])for(const path of new Bun.Glob("**/*.rs").scanSync({cwd:resolve(root,directory),onlyFiles:true})){ const relativePath=directory+"/"+path; if(originalPaths.has(relativePath)||path.split("/").some(part=>["target","node_modules",".git"].includes(part)))continue; const source=readFileSync(resolve(root,relativePath),"utf8"); if(selector.test(source)){rows.push({path:relativePath});extraPaths.push(relativePath);} }
const observations:any[]=[], candidates:any[]=[], qualified:any[]=[], ownershipGaps:any[]=[];
const owned=(parts:string[])=>{
 let offset=parts[0]==="semio_framework_os_kernel"||parts[0]==="semio_framework_os_dsl_derive"?1:-1;
 if(offset<0)return false;if(parts[offset]==="os_dsl"||parts[offset]==="dsl")offset++;
 if(parts[offset]==="schema")return true;
 if(parts[offset]==="notation")return ["EdgeLabel","EdgeLink","EdgeNode","EdgeValue","print_edge","parse_edge_text"].includes(parts[offset+1]);
 if(names.has(parts[offset]))return true;
 return ["os_protocol","os_store","protocol","store"].includes(parts[offset])&&names.has(parts[offset+1]);
};
for(const row of rows.filter(row=>row.path.endsWith(".rs"))){
 const path=resolve(root,row.path);if(!existsSync(path))continue;const source=readFileSync(path,"utf8"), tree=parser.parse(source)!;
 const owners=libraries.filter(owner=>path===owner.library||path.startsWith(dirname(owner.library)+"/")||owner.mounts.some(mount=>path===mount||path.startsWith(dirname(mount)+"/"))).sort((a,b)=>dirname(b.library).length-dirname(a.library).length);
 const direct=owners.filter(owner=>path===owner.library||owner.mounts.includes(path)), selected=direct.length?direct:owners.slice(0,1);
 if(!selected.length)ownershipGaps.push({path:row.path,reason:"No actual existing Cargo lib or physical path association"});
 const associations=selected.length?selected:[{manifest:null,library:null,aliases:new Map()}];
 const found:any[]=[];
 for(const owner of associations){lexical.lexicalScopeCache.clear();const classify=(parts:string[],node:any,kind:string)=>{let resolved=lexical.resolveRustAlias(parts,node,owner.aliases);if(resolved[0]==="crate"&&Array.isArray(owner.aliases.get(resolved[1])))resolved=[...owner.aliases.get(resolved[1]),...resolved.slice(2)];const item={path:row.path,start:node.startIndex,end:node.endIndex,reference:parts.join("::"),resolved,kind,manifest:owner.manifest,library:owner.library};if(owned(resolved)){candidates.push(item);found.push(item);}else if(parts.some(part=>names.has(part)))qualified.push(item);};
  const visit=(node:any)=>{if(["line_comment","block_comment","string_literal","raw_string_literal","char_literal"].includes(node.type))return;if(node.type==="use_declaration"){for(const leaf of lexical.flattenRustUse(node.childForFieldName("argument")))classify(leaf.path,node,"import");return;}if(["scoped_identifier","scoped_type_identifier"].includes(node.type)){classify(node.text.replace(/^::/,"").split("::"),node,"ordinary-path");return;}for(const child of node.namedChildren)visit(child);};visit(tree.rootNode);
  lexical.canonicalTokenPathEdits(tree.rootNode,(parts,node)=>{classify(parts,node,"macro-token-path");return null;});
  for(const match of source.matchAll(/\b[A-Za-z_][A-Za-z_0-9]*(?:::[A-Za-z_][A-Za-z_0-9]*)+/g)){let node=tree.rootNode.descendantForIndex(match.index!,match.index!+match[0].length),at=node,bound=false,literal=false;while(at){if(["string_literal","raw_string_literal"].includes(at.type))literal=true;if(at.type==="attribute_item"&&/\bbound\s*=/.test(at.text))bound=true;at=at.parent;}if(bound&&literal)classify(match[0].split("::"),node,"semantic-derive-bound");}
 }
 observations.push({path:row.path,source,inverse:source,sha256:hash(source),owners:selected.map(owner=>({manifest:owner.manifest,library:owner.library,aliases:[...owner.aliases]})),candidates:found});tree.delete();
}
parser.delete();const unique=[...new Map(candidates.map(row=>[JSON.stringify(row),row])).values()];writeFileSync(target,JSON.stringify({at:new Date().toISOString(),frames,observations,extraPaths,candidates:unique,qualified,ownershipGaps,helper:{path:helperPath,sha256:hash(helper)},scope:"Actual current Cargo lib paths and physical mounts; arbitrary lexical namespace aliases for ordinary/import/token-tree/repetition/bound heads, with product symbols and local shadowing retained; no native claim",sourceWrites:0,nativeExecuted:0},null,2));
console.log(JSON.stringify({path:target,libraries:libraries.length,rows:observations.length,extraPaths:extraPaths.length,candidates:unique.length,ownershipGaps:ownershipGaps.length,sourceWrites:0,nativeExecuted:0}));
