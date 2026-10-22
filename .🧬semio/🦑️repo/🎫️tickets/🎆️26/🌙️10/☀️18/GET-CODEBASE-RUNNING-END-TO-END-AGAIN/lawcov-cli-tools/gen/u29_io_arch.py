import lib
from lib import S
UNIT = "🚪io/🏛architecture"
sem = [
 ("a Mutation impl", "impl Mutation for X {}\n", ["Mutation"]), ("a MutationDiff impl", "impl MutationDiff<S> for Y {}\n", ["MutationDiff"]), ("a DiffAlgebra impl", "impl DiffAlgebra for Z {}\n", ["DiffAlgebra"]),
 ("apply_to_artifact", "fn apply_to_artifact(a: &mut A) {}\n", ["apply_to_artifact"]), ("a diff function returning a diff type", "fn diff_text(a: &A) -> TextDiff { todo!() }\n", ["diff_text"]), ("a diff function returning something else", "fn diff_names() -> Vec<String> { vec![] }\n", []),
 ("every kind at once, sorted", "impl Mutation for A {}\nimpl DiffAlgebra for B {}\nfn apply_to_artifact() {}\nfn diff_x() -> XDiff { todo!() }\n", ["DiffAlgebra", "Mutation", "apply_to_artifact", "diff_x"]),
 ("names in strings and comments", "// impl Mutation for X\nlet s = \"impl DiffAlgebra for Y\";\nfn plain() {}\n", []), ("a trait that merely contains the word", "impl MutationLike for X {}\nimpl Display for Y {}\n", []), ("an empty source", "", []),
 ("a generic impl", "impl<T: Clone> Mutation for Wrapper<T> where T: Debug {}\n", ["Mutation"]),
]
deps = [
 ("serde_json", "use serde_json::Value;\n", ["serde_json"]), ("uri helpers", "fn f() { to_uri(); parse_coordinate(); parse_uri_controlled(); }\n", ["parse_coordinate", "parse_uri_controlled", "to_uri"]),
 ("a cfg(test) module is excluded", "#[cfg(test)]\nmod tests { use serde_json::Value; fn t() { to_uri(); } }\nfn live() {}\n", []), ("a cfg(test) function is excluded", "#[cfg(test)]\nfn t() { to_uri(); }\nfn live() { parse_uri(); }\n", ["parse_uri"]),
 ("serialisation attributes name their owner", "#[value(serialize_controlled_with = \"crate::io::text::ser\", deserialize_controlled_with = \"crate::io::binary::de\")]\nstruct S;\n", ["io::binary", "io::text"]), ("a foreign io owner", "#[value(retire_with = \"crate::io::weird::f\")]\nstruct S;\n", ["io::"]),
 ("codec traits and functions", "use semio::OpText;\nfn f() { encode_op(); decode_diff(); }\n", ["OpText", "decode_diff", "encode_op"]), ("the framework io schema crate", "use semio_framework_io_schema::X;\n", ["semio_framework_io_schema"]), ("pack runtime paths", "fn f() { pack_rt::encode_pack(); }\n", ["pack_rt::encode_pack"]),
 ("strings do not count", "let s = \"serde_json to_uri io::text\";\n", []), ("a clean schema", "pub struct Schema { pub a: u8 }\n", []), ("a cfg(test) attribute with an item list", "#[cfg(test)] use serde_json;\nfn f() {}\n", []),
 ("any non-listed io path", "fn f() { crate::io::weird(); }\n", ["io::"]),
]
ts_syms = [
 ("json projections", "export function widgetToJson() {}\nexport const widgetFromJsonText = 1;\nexport function toJsonThing() {}\n", ["widgetFromJsonText", "widgetToJson"]),
 ("encoders and decoders", "export function encodeFooJson() {}\nexport function decodeBarProtobuf() {}\nexport const writeBazJsonValue = 1;\nexport function decodeNothing() {}\n", ["decodeBarProtobuf", "encodeFooJson", "writeBazJsonValue"]),
 ("fixed names", "export const floatLexeme = 1;\nexport const jsonGeometry = 2;\nexport class TxtProtobufThing {}\nexport const txtProtobufKey = 3;\nexport const txtProtobufString = 4;\nexport const txtProtobufOther = 5;\n", ["TxtProtobufThing", "floatLexeme", "jsonGeometry", "txtProtobufKey", "txtProtobufString"]),
 ("remodeling and tikz", "export function decodeRemodelingSnapshot() {}\nexport function decodeRemodelingOther() {}\nexport function renderFooTikz() {}\nexport function inferBarTikzPlan() {}\nexport function inferTikzLater() {}\n", ["decodeRemodelingSnapshot", "inferBarTikzPlan", "renderFooTikz"]),
 ("re-exports and aliases", "const x = 1;\nexport { x as widgetToJson };\nexport { y as plain } from './y';\n", ["widgetToJson"]), ("comments and strings are not exports", "// export function aToJson() {}\nconst s = 'export function bToJson() {}';\n", []), ("no exports", "const a = 1;\n", []),
]
dif = [
 ("both representations exist", "impl MutationDiff<S> for A {}\nimpl MutationDiff<S> for B {}\n", "impl DiffText for A {}\ndiff_text!(B);\n", "impl DiffBinary for A {}\ndiff_binary!(B);\n", []),
 ("the binary owner lacks one", "impl MutationDiff<S> for A {}\nimpl MutationDiff<S> for B {}\n", "impl DiffText for A {}\ndiff_text!(B);\n", "impl DiffBinary for A {}\n", ["B"]),
 ("the text owner lacks both", "impl MutationDiff<S> for A {}\nimpl MutationDiff<S> for B {}\n", "", "impl DiffBinary for A {}\ndiff_binary!(B);\n", ["A", "B"]),
 ("qualified paths", "impl MutationDiff<S> for crate::m::A {}\n", "impl DiffText for other::A {}\n", "diff_binary!(x::y::A);\n", []),
 ("no semantic diff", "struct X;\n", "", "", []), ("duplicates collapse and names sort", "impl MutationDiff<S> for B {}\nimpl MutationDiff<T> for B {}\nimpl MutationDiff<S> for A {}\n", "", "", ["A", "B"]),
 ("a string mentioning the impl is ignored", "let s = \"impl MutationDiff<S> for Q\";\n", "", "", []),
]
corpus = {"version": 1, "semantic": [{"name": n, "source": s, "expected": e} for n, s, e in sem], "dependencies": [{"name": n, "source": s, "expected": e} for n, s, e in deps], "symbols": [{"name": n, "source": s, "expected": e} for n, s, e in ts_syms],
          "diffs": [{"name": n, "semantic": a, "text": b, "binary": c, "expected": e} for n, a, b, c, e in dif]}
schema = S.obj({"version": S.const(1), "semantic": S.arr(S.obj({"name": S.str, "source": S.str, "expected": S.strs()}), 1), "dependencies": S.arr(S.obj({"name": S.str, "source": S.str, "expected": S.strs()}), 1), "symbols": S.arr(S.obj({"name": S.str, "source": S.str, "expected": S.strs()}), 1),
                "diffs": S.arr(S.obj({"name": S.str, "semantic": S.str, "text": S.str, "binary": S.str, "expected": S.strs()}), 1)})
body = '''const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
const SYMBOLS=/(?:To|From)(?:Native)?Json(?:Text|Value|Projection)?$|Json(?:Text|Projection)$|^(?:decode|encode|write).*Json(?:Value)?$|^decode.*Protobuf$|^floatLexeme$|^jsonGeometry$|^TxtProtobuf|^txtProtobuf(?:Key|String)$|^decodeRemodeling(?:Snapshot|Diff|Mutation|Artifact)$|^(?:render|infer).*Tikz(?:Plan)?$/u;
const exportsOf=(source:string):string[]=>{
 const file=ts.createSourceFile("x.ts",source,ts.ScriptTarget.ES2022,true),names:string[]=[];
 for(const statement of file.statements){
  const exported=ts.canHaveModifiers(statement)&&(ts.getModifiers(statement)??[]).some(modifier=>modifier.kind===ts.SyntaxKind.ExportKeyword);
  if(exported&&ts.isVariableStatement(statement))for(const declaration of statement.declarationList.declarations)names.push((declaration.name as ts.Identifier).text);
  else if(exported&&(ts.isFunctionDeclaration(statement)||ts.isClassDeclaration(statement))&&statement.name)names.push(statement.name.text);
  else if(ts.isExportDeclaration(statement)&&statement.moduleSpecifier===undefined&&statement.exportClause&&ts.isNamedExports(statement.exportClause))for(const element of statement.exportClause.elements)names.push(element.name.text);
 }
 return names;
};
for(const row of corpus.semantic)test("semantic io items: "+row.name,()=>{expect(semanticArtifactIoItems(row.source)).toEqual(row.expected);});
for(const row of corpus.dependencies)test("schema wire dependencies: "+row.name,()=>{expect(schemaRustWireDependencies(row.source)).toEqual(row.expected);});
for(const row of corpus.symbols)test("typescript wire symbols: "+row.name,()=>{expect(schemaTypeScriptWireSymbols(row.source)).toEqual(row.expected);});
for(const row of corpus.diffs)test("missing diff wire types: "+row.name,()=>{expect(missingArtifactDiffWireTypes(row.semantic,row.text,row.binary)).toEqual(row.expected);});
test("the TypeScript compiler API enumerates the same exports the scanner filters",()=>{for(const row of corpus.symbols)expect(exportsOf(row.source).filter(name=>SYMBOLS.test(name)).sort(),row.name).toEqual(row.expected);});
test("every Rust fixture of the corpus is syntactically valid (tree-sitter)",async()=>{
 await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);
 const wrapped=(text:string):string=>text;
 for(const row of[...corpus.semantic,...corpus.dependencies])if(!row.source.includes("todo!()")||true)expect(parser.parse(wrapped(row.source)).rootNode.hasError(),row.name).toBe(false);
 for(const row of corpus.diffs)for(const text of[row.semantic,row.text,row.binary])expect(parser.parse(text).rootNode.hasError(),row.name+": "+text).toBe(false);
});
test("tree-sitter finds the declarations the semantic scanner reports",async()=>{
 await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);
 for(const row of corpus.semantic.filter(item=>item.expected.length>0)){
  const root=parser.parse(row.source).rootNode,traits=new Set<string>(),functions=new Set<string>();
  const visit=(node:Parser.SyntaxNode):void=>{if(node.type==="impl_item"){const trait=node.childForFieldName("trait");if(trait!==null)traits.add(trait.type==="generic_type"?trait.childForFieldName("type")!.text:trait.text);}if(node.type==="function_item")functions.add(node.childForFieldName("name")!.text);for(const child of node.children)visit(child);};visit(root);
  for(const name of row.expected)expect(traits.has(name)||functions.has(name),row.name+" "+name).toBe(true);
 }
});'''
ts_ = lib.ts_wrap(UNIT, "🏛️ Contract: the artifact I/O architecture scanner finds semantic mutation and diff declarations, physical wire dependencies of a schema (outside `cfg(test)`), physical TypeScript wire exports and semantic diffs that lack a text or binary implementation, ignoring strings and comments; the corpus is language-agnostic, tree-sitter's Rust grammar and the TypeScript compiler API are the independent third-party oracles for the fixtures and the export enumeration.",
 'import Parser from "web-tree-sitter";\nimport ts from "typescript";\nimport {createRequire} from "node:module";\nimport {dirname,join} from "node:path";\nimport {missingArtifactDiffWireTypes,schemaRustWireDependencies,schemaTypeScriptWireSymbols,semanticArtifactIoItems} from "@@' + UNIT + '/🟦.ts@@";', body, python=False)
lib.write_unit(UNIT, corpus, schema, ts_)
