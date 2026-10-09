import Parser from "web-tree-sitter";
import { dirname, join } from "node:path";
import { readFileSync, writeFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
await Parser.init();
const parser = new Parser();
parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", process.cwd())), "out/tree-sitter-rust.wasm")));
const binary = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs";
const semantic = binary.replace("/🚪️io/💾️binary/", "/🧬️schema/");
const source = readFileSync(binary, "utf8"), schema = readFileSync(semantic, "utf8");
const tree = parser.parse(source)!;
const names = new Set(["GENERATION2D_OWNER_BYTES", "GENERATION2D_RETAINED_STACK_CAPACITY", "GENERATION2D_MAXIMUM_DOMAIN_ITEMS", "Generation2dReplayDisplaced", "Generation2dReplayRetirement", "generation2d_close_flow_frontier", "generation2d_retire_displaced", "generation2d_apply_initialization_mutation", "generation2d_retire_mutation_cold", "generation2d_retire_mutations_cold", "generation2d_apply_retained_mutations_for_test"]);
const spans: {start:number; end:number; name:string; text:string}[] = [];
let comments: Parser.SyntaxNode[] = [];
for (const node of tree.rootNode.namedChildren) {
 if (node.type === "line_comment") { if (node.text.startsWith("///")) comments.push(node); else comments=[]; continue; }
 if (node.type === "attribute_item") { comments.push(node); continue; }
 const name = node.childForFieldName("name")?.text ?? node.childForFieldName("type")?.text ?? "";
 if (names.has(name) || name.startsWith("generation2d_copy_")) {
  names.add(name); spans.push({start:comments[0]?.startIndex ?? node.startIndex,end:node.endIndex,name,text:source.slice(comments[0]?.startIndex ?? node.startIndex,node.endIndex)});
 }
 comments=[];
}
const functions = spans.filter(row => /^generation2d_/u.test(row.name)).map(row => row.name);
let next = source;
for (const row of spans.toReversed()) next = next.slice(0,row.start)+next.slice(row.end);
const imported = [...names].filter(name => /^generation2d_/u.test(name) || /^GENERATION/u.test(name));
const normal = imported.filter(name => !["generation2d_retire_mutations_cold", "generation2d_apply_retained_mutations_for_test"].includes(name));
next = next.replace(/(use crate::standards::v1::subsets::any::schema::mutations::[^;]+;)/u, `$1\nuse crate::standards::v1::subsets::any::schema::mutations::{${normal.join(", ")}};\n#[cfg(test)]\nuse crate::standards::v1::subsets::any::schema::mutations::{generation2d_retire_mutations_cold, generation2d_apply_retained_mutations_for_test};`);
const definitions = spans.map(row => row.text.replace(/^(?:pub(?:\(crate\))?\s+)?fn (generation2d_\w+)/mu,"pub(crate) fn $1").replace(/^const (GENERATION2D_\w+)/mu,"pub(crate) const $1").replace("store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES", "4_096"));
const body = schema.trimEnd()+"\n\n"+definitions.join("\n\n")+"\n";
for (const [file,content] of [[binary,next],[semantic,body]]) {
 const parsed=parser.parse(content)!; if(parsed.rootNode.hasError()) throw Error(`Syntax invalid: ${file}`); parsed.delete(); writeFileSync(file,content);
}
const changed=[binary,semantic];
const files=execFileSync("rg",["--files","-g","*.rs","✏️s/🔌️plugins/🌀️procedural"],{encoding:"utf8",maxBuffer:16*1024*1024}).trim().split("\n");
for(const file of files) {
 if(file===binary||file===semantic) continue;
 const before=readFileSync(file,"utf8"), after=before.replace(/io::binary::mutations::(generation2d_\w+)/gu,(path,name)=>functions.includes(name)?path.replace("io::binary::mutations::","schema::mutations::"):path);
 if(after!==before){writeFileSync(file,after);changed.push(file);}
}
const report=["# Generation2d Semantic Replay Ownership", "", `Moved ${spans.length} declarations (direct semantic replay, displaced-owner retirement, retained clone helpers, structural bounds, cold disposal and test replay) into canonical schema mutations. Binary ingress imports its needed helpers privately. Physical JSON/pack parsing and store initialization cursor remain in IO. Tree-sitter validates resulting Rust syntax; no native runtime claim.`, "", "## Changed Files", "", ...changed.map(path=>`- ${path}`), ""].join("\n");
writeFileSync(join(import.meta.dir,"../semantic-replay-extraction.md"),report);
console.log(`[DEBUG] semantic-replay-extraction declarations=${spans.length} files=${changed.length} syntax=tree-sitter`);
parser.delete();tree.delete();
