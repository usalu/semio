import Parser from "web-tree-sitter";
import {rustTokens} from "../../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import {execFileSync} from "node:child_process";
import {readFileSync,writeFileSync,existsSync} from "node:fs";
import {dirname,join,resolve,relative} from "node:path";
await Parser.init();
const parser=new Parser();
parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",process.cwd())),"out/tree-sitter-rust.wasm")));
const root=process.cwd(),ticket=resolve(import.meta.dir,".."),output=join(ticket,"rust-artifact-reference-routing.md");
const identities=new Set(["ArtifactDialect","ArtifactRef","Dialect","StandardId","SubsetId","ArtifactKindId","is_canonical_artifact_kind"]);
const owner="semio_framework_artifact_reference";
const isFacade=(path:string)=>/^(?:semio_framework_io_schema|semio_framework_plugin|semio_framework_os_kernel|semio_framework|protocol|store)$/.test(path)||/(?:^|::)(?:io_schema|os_io|io)$/.test(path);
const expand=(text:string):string[]=>{
 const brace=text.indexOf("{");if(brace<0)return[text.trim().replace(/::$/,"")];
 const prefix=text.slice(0,brace),middle=text.slice(brace+1,text.lastIndexOf("}"));let depth=0,start=0;const parts:string[]=[];
 for(let i=0;i<=middle.length;i++){if(middle[i]==="{")depth++;if(middle[i]==="}")depth--;if(i===middle.length||middle[i]===","&&depth===0){const part=middle.slice(start,i).trim();if(part)parts.push(...expand(prefix+part));start=i+1;}}
 return parts;
};
const files=execFileSync("rg",["--files","-g","*.rs","🧰️framework","✏️s"],{encoding:"utf8",maxBuffer:32*1024*1024}).trim().split("\n");
if(process.argv.includes("--inventory")){
 const sources=files.filter(file=>readFileSync(file,"utf8").includes(owner));
 const contracts=execFileSync("rg",["-l","framework/schema/artifact-reference","🧰️framework","✏️s","-g","*.json","-g","*.proto"],{encoding:"utf8",maxBuffer:8*1024*1024}).trim().split("\n");
 const dependencies=execFileSync("rg",["-l","semio-framework-artifact-reference","🧰️framework","✏️s","-g","Cargo.toml"],{encoding:"utf8",maxBuffer:8*1024*1024}).trim().split("\n");
 const report=join(ticket,"artifact-reference-current-consumer-manifest.md");
 writeFileSync(report,"# Current Canonical Artifact Reference Consumers\n\nThis read-only inventory records current source references, contract references and first-party Cargo declarations. Initial token-aware routing actually edited 1,116 Rust sources and 119 manifests. A subsequent no-op apply accidentally replaced its original per-edit report with an empty edit list; this current-state manifest is explicit about that limitation and does not attribute every inventory row as a new edit. Canonical model, binding and native traits are separate owners.\n\n## Rust Source References ("+sources.length+")\n\n"+sources.map(file=>"- `"+file+"`").join("\n")+"\n\n## Contract References ("+contracts.length+")\n\n"+contracts.map(file=>"- `"+file+"`").join("\n")+"\n\n## Cargo Dependencies ("+dependencies.length+")\n\n"+dependencies.map(file=>"- `"+file+"`").join("\n")+"\n");
 process.stdout.write(JSON.stringify({sources:sources.length,contracts:contracts.length,dependencies:dependencies.length,report})+"\n");process.exit(0);
}
const prefixCount=new Map<string,number>(),changed=new Set<string>(),traits=new Set<string>();
const apply=process.argv.includes("--apply");
for(const file of files){
 if(file.includes("/🧬️schema/🗿️artifact-reference/")||file.includes("/🗿️artifact-reference/📦️packages/"))continue;
 let source=readFileSync(file,"utf8");if(![...identities].some(x=>source.includes(x)))continue;
 const tree=parser.parse(source)!;const useNodes:any[]=[],walk=(node:any)=>{if(node.type==="use_declaration")useNodes.push(node);else for(const child of node.namedChildren)walk(child);};walk(tree.rootNode);
 const edits:{start:number,end:number,text:string}[]=[],seen=new Set<string>();
 for(const node of useNodes){
  const match=rustTokens(node.text).map(token=>token.text).join(" " ).match(/^(pub(?:\s*\([^)]*\))?\s+)?use\s+([\s\S]+);$/);if(!match)continue;
  let moved=false;const imports=expand(match[2]).map(path=>{
   const m=path.trim().match(/^(.*)::([A-Za-z_]\w*)(?:\s+as\s+([A-Za-z_]\w*))?$/);if(m)m[1]=m[1].replace(/\s+/g,"");
   if(m&&identities.has(m[2])&&isFacade(m[1])){moved=true;prefixCount.set(m[1],(prefixCount.get(m[1])??0)+1);seen.add(m[2]);return owner+"::"+m[2]+(m[3]?" as "+m[3]:"");}
   return path;
  });
  if(moved){
   const visibility=match[1]??"";
   edits.push({start:node.startIndex,end:node.endIndex,text:visibility+"use {"+[...new Set(imports)].join(",")+"};"});
  }
  for(const path of imports){if(/::\*$/.test(path)&&isFacade(path.slice(0,-3)))for(const type of identities)if(new RegExp("\\b"+type+"\\b").test(source))seen.add(type);}
 }
 const tokens=rustTokens(source);
 for(let i=0;i<tokens.length;i++){
  const token=tokens[i];if(token.kind!=="identifier"||!identities.has(token.text)||tokens[i-1]?.text!=="::")continue;
  if(useNodes.some(n=>n.startIndex<=token.start&&n.endIndex>=token.end))continue;
  let first=i-2;while(first>=2&&tokens[first-1]?.text==="::"&&tokens[first-2]?.kind==="identifier")first-=2;
  const path=tokens.slice(first,i-1).map(x=>x.text).join("");if(!isFacade(path))continue;
  prefixCount.set(path,(prefixCount.get(path)??0)+1);edits.push({start:tokens[first].start,end:token.end,text:owner+"::"+token.text});
 }
 for(const token of tokens.filter(t=>t.kind==="string")){
  const text=token.text.replace(/\b((?:\w+::)*(?:io_schema|os_io|io)|semio_framework_io_schema)::(ArtifactDialect|ArtifactRef|Dialect|StandardId|SubsetId|ArtifactKindId|is_canonical_artifact_kind)\b/g,owner+"::$2");
  if(text!==token.text)edits.push({start:token.start,end:token.end,text});
 }
 if(seen.size&&useNodes.some(n=>expand(n.text.replace(/^(?:pub(?:\([^)]*\))?\s+)?use\s+/,"").replace(/;$/,"")).some(p=>/::\*$/.test(p)&&/(?:^|::)(?:io_schema|os_io|io)$/.test(p.slice(0,-3))))){
  let at=source.indexOf("\n");while(at>=0&&(source.slice(at+1).startsWith("//!")||source.slice(at+1).startsWith("#![")))at=source.indexOf("\n",at+1);
  edits.push({start:at+1,end:at+1,text:"use "+owner+"::{"+[...seen].join(",")+"};\n"});
 }
 if(!edits.length)continue;
 for(const edit of edits.sort((a,b)=>b.start-a.start))source=source.slice(0,edit.start)+edit.text+source.slice(edit.end);
 if(parser.parse(source)!.rootNode.hasError()&&!tree.rootNode.hasError())throw Error("Introduced Rust parse failure: "+file);
 changed.add(file);if(apply)writeFileSync(file,source);
}
const metadata={packages:[...JSON.parse(readFileSync(join(ticket,"🗑️generated/artifact-reference-cargo-metadata.json"),"utf8")).packages,...JSON.parse(readFileSync(join(ticket,"🗑️generated/artifact-reference-plugin-cargo-metadata.json"),"utf8")).packages]};
const manifests=new Set<string>();
for(const pkg of metadata.packages){
 if(pkg.name==="semio-framework-artifact-reference")continue;
 const packageRoot=dirname(pkg.manifest_path);
 let owning=relative(root,packageRoot.split("/📦️packages/")[0]);
 if(!owning)continue;
 const authored=[...changed].some(f=>f.startsWith(owning+"/"));
 if(!authored)continue;
 let manifest=readFileSync(pkg.manifest_path,"utf8");
 if(!manifest.includes("\nsemio-framework-artifact-reference ")){
  const section=pkg.name==="semio-framework-value"?"dev-dependencies":"dependencies";const match=manifest.match(new RegExp("^\\["+section+"\\]\\s*$","m"));
  if(!match)throw Error("Missing dependency section: "+pkg.manifest_path);
  manifest=manifest.slice(0,match.index+match[0].length)+"\nsemio-framework-artifact-reference = { workspace = true }"+manifest.slice(match.index+match[0].length);
  if(apply)writeFileSync(pkg.manifest_path,manifest);manifests.add(relative(root,pkg.manifest_path));
 }
}
writeFileSync(output,"# Rust Artifact Reference Consumer Routing\n\n"+(apply?"Applied":"Read-only preview")+" token-aware source routing. Comments and unrelated literals are preserved. Native parse/format traits require separate explicit imports after this step. Every changed source uses the canonical semantic crate; manifest dependencies are first-party workspace references.\n\n## Facade Prefixes\n\n"+[...prefixCount].sort((a,b)=>b[1]-a[1]).map(([p,n])=>"- "+p+": "+n).join("\n")+"\n\n## Source Manifest\n\n"+[...changed].sort().map(f=>"- \`"+f+"\`").join("\n")+"\n\n## Dependency Manifest\n\n"+[...manifests].sort().map(f=>"- \`"+f+"\`").join("\n")+"\n");
process.stdout.write(JSON.stringify({applied:apply,sources:changed.size,manifests:manifests.size,prefixes:prefixCount.size,report:relative(root,output)})+"\n");
