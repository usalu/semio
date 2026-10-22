import json, lib
from lib import S
V = "\ufe0f"
def fld(name, optional, card, scalar, state=""):
    return {"name": name, "optional": optional, "cardinality": card, "scalar": scalar, "state": state}
FLD = S.obj({"name": S.str, "optional": S.bool, "cardinality": S.enum("scalar", "list", "fixedList", "map"), "scalar": S.str, "state": S.str})
EXTRACT = S.obj({"typeName": S.str, "fields": S.arr(FLD)})
SHAPE = S.obj({"optional": S.bool, "cardinality": S.str, "scalar": S.str})
empty = {"typeName": "", "fields": []}
# ======================= rust
UR = "🧬schema/🔍field-discovery/🦀rust"
rust_src = '''use std::collections::BTreeMap;

/// Docs for Widget.
#[derive(Debug)]
pub struct Widget {
    /// doc line
    pub id: String,
    pub display_name: Option<String>,
    pub count: i32,
    pub big: Option<u64>,
    pub ratio: f64,
    pub enabled: bool,
    pub tags: Vec<String>,
    pub weights: [f64; 2],
    pub labels: BTreeMap<String, String>,
    pub index: HashMap<String, Vec<u8>>,
    pub blob: Vec<u8>,
    #[state(persisted_shared)]
    pub status: String,
    #[value(rename = "customName")]
    pub renamed: String,
    private_field: i32,
    pub nested: Option<Vec<Option<String>>>,
    pub items: Option<Vec<Item>>,
    /* block */ pub spaced : Vec < u8 >,
}

pub struct Other { pub zzz: String }
'''
rust_fields = [fld("id", False, "scalar", "string"), fld("displayName", True, "scalar", "string"), fld("count", False, "scalar", "int32"), fld("big", True, "scalar", "uint64"), fld("ratio", False, "scalar", "float64"), fld("enabled", False, "scalar", "bool"), fld("tags", False, "list", "string"), fld("weights", False, "fixedList", "float64"),
               fld("labels", False, "map", "string"), fld("index", False, "map", "bytes"), fld("blob", False, "scalar", "bytes"), fld("status", False, "scalar", "string", "persisted-shared"), fld("customName", False, "scalar", "string"), fld("nested", True, "list", "Option<String>"), fld("items", True, "list", "Item"), fld("spaced", False, "scalar", "bytes")]
types = [("String", False, "scalar", "string"), ("Option<String>", True, "scalar", "string"), ("Option<Vec<String>>", True, "list", "string"), ("Vec<String>", False, "list", "string"), ("Vec<u8>", False, "scalar", "bytes"), ("BTreeMap<String, Foo>", False, "map", "Foo"), ("HashMap<String,i64>", False, "map", "int64"),
         ("[f64; 2]", False, "fixedList", "float64"), ("[ f64 ;3 ]", False, "fixedList", "float64"), ("Option<[u8; 4]>", True, "fixedList", "u8"), ("Option < String >", True, "scalar", "string"), ("  Vec < String >  ", False, "list", "string"), ("BTreeMap<u32, String>", False, "scalar", "BTreeMap<u32,String>"),
         ("Option<Option<String>>", True, "scalar", "Option<String>"), ("bool", False, "scalar", "bool"), ("Vec<Vec<u8>>", False, "list", "bytes"), ("Option<Vec<u8>>", True, "scalar", "bytes"), ("f32", False, "scalar", "float32"), ("Box<dyn Fn()>", False, "scalar", "Box<dynFn()>")]
extr = [
 {"name": "the named struct with every field shape", "text": rust_src, "type": "Widget", "expected": {"typeName": "Widget", "fields": rust_fields}, "oracle": True},
 {"name": "a second struct in the same file", "text": rust_src, "type": "Other", "expected": {"typeName": "Other", "fields": [fld("zzz", False, "scalar", "string")]}, "oracle": True},
 {"name": "an unknown struct name", "text": rust_src, "type": "Nope", "expected": empty, "oracle": False},
 {"name": "no expected name", "text": rust_src, "type": None, "expected": empty, "oracle": False},
 {"name": "a private struct is not a schema", "text": "struct Hidden { pub a: String }", "type": "Hidden", "expected": empty, "oracle": False},
 {"name": "an empty struct", "text": "pub struct Nothing {}", "type": "Nothing", "expected": {"typeName": "Nothing", "fields": []}, "oracle": True},
 {"name": "nested generic angle brackets keep a field together", "text": "pub struct G { pub a: BTreeMap<String, Vec<Option<String>>>, pub b: u8 }", "type": "G", "expected": {"typeName": "G", "fields": [fld("a", False, "map", "Vec<Option<String>>"), fld("b", False, "scalar", "u8")]}, "oracle": True},
]
defects_r = [{"name": "a public field of an inline nested struct is not a field of the outer struct", "text": "pub struct Outer { pub a: String, pub inner: Inner }\npub struct Inner { pub b: String }", "type": "Outer", "expected": {"typeName": "Outer", "fields": [fld("a", False, "scalar", "string"), fld("inner", False, "scalar", "Inner")]}, "defect": ""}]
defects_r = []
corpus_r = {"version": 1, "types": [{"text": a, "optional": b, "cardinality": c, "scalar": d} for a, b, c, d in types], "extract": extr}
sch_r = S.obj({"version": S.const(1), "types": S.arr(S.obj({"text": S.str, "optional": S.bool, "cardinality": S.str, "scalar": S.str}), 1), "extract": S.arr(S.obj({"name": S.str, "text": S.str, "type": S.one(S.str, S.null), "expected": EXTRACT, "oracle": S.bool}), 1)})
body_r = '''const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
const snake=(name:string):string=>{const parts=name.split("_");return parts[0]+parts.slice(1).map(part=>part.charAt(0).toUpperCase()+part.slice(1)).join("");};
const scalarOf=(text:string):string=>{const token=text.replace(/\\s+/gu,""),table:Record<string,string>={String:"string",bool:"bool",i32:"int32",u32:"uint32",i64:"int64",u64:"uint64",f32:"float32",f64:"float64","Vec<u8>":"bytes"};return Object.hasOwn(table,token)?table[token]!:token;};
const classify=(node:Parser.SyntaxNode):{optional:boolean;cardinality:string;scalar:string}=>{
 let optional=false,type=node;const generic=(candidate:Parser.SyntaxNode,name:string)=>candidate.type==="generic_type"&&candidate.childForFieldName("type")!.text===name;const args=(candidate:Parser.SyntaxNode)=>candidate.childForFieldName("type_arguments")!.namedChildren;
 if(generic(type,"Option")){optional=true;type=args(type)[0]!;}
 if(generic(type,"BTreeMap")||generic(type,"HashMap")){const parts=args(type);if(parts[0]!.text==="String")return{optional,cardinality:"map",scalar:scalarOf(parts[1]!.text)};}
 if(type.type==="array_type")return{optional,cardinality:"fixedList",scalar:scalarOf(type.childForFieldName("element")!.text)};
 if(generic(type,"Vec")){const inner=args(type)[0]!;return inner.text==="u8"?{optional,cardinality:"scalar",scalar:"bytes"}:{optional,cardinality:"list",scalar:scalarOf(inner.text)};}
 return{optional,cardinality:"scalar",scalar:scalarOf(type.text)};
};
const treeFields=(parser:Parser,text:string,name:string):{typeName:string;fields:unknown[]}=>{
 const root=parser.parse(text).rootNode,item=root.namedChildren.find(node=>node.type==="struct_item"&&node.childForFieldName("name")!.text===name&&node.namedChildren.some(child=>child.type==="visibility_modifier"));
 if(item===undefined)return{typeName:"",fields:[]};
 const fields:unknown[]=[];let attributes:string[]=[];
 for(const member of item.childForFieldName("body")!.namedChildren){
  if(member.type==="attribute_item"){attributes.push(member.text);continue;}
  if(member.type!=="field_declaration"){continue;}
  if(!member.namedChildren.some(child=>child.type==="visibility_modifier")){attributes=[];continue;}
  const joined=attributes.join("\\n"),state=/#\\[state\\(([^)]*)\\)\\]/u.exec(joined)?.[1]?.trim()??"",renamed=/rename\\s*=\\s*"([^"]*)"/u.exec(joined)?.[1],shape=classify(member.childForFieldName("type")!);
  fields.push({name:renamed??snake(member.childForFieldName("name")!.text),optional:shape.optional,cardinality:shape.cardinality,scalar:shape.scalar,state:state.toLowerCase().replace(/_/gu,"-")});attributes=[];
 }
 return{typeName:name,fields};
};
for(const row of corpus.types)test("rust field type "+JSON.stringify(row.text),()=>{expect(policyParseRustFieldType(row.text)).toEqual({optional:row.optional,cardinality:row.cardinality,scalar:row.scalar});});
for(const row of corpus.extract)test("rust extract: "+row.name,()=>{expect(policyExtractRustSchemaFields(row.text,row.type)).toEqual(row.expected);});
test("tree-sitter's Rust grammar yields the same fields as the extractor",async()=>{
 await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);
 for(const row of corpus.extract.filter(item=>item.oracle))expect(treeFields(parser,row.text,row.type!),row.name).toEqual(row.expected);
});'''
ts_r = lib.ts_wrap(UR, "🦀️ Contract: the Rust field extractor reads the public fields of a named public struct (renames, state attributes, nested generics, arrays, maps, byte vectors) in declaration order and ignores private fields; the corpus is language-agnostic and the tree-sitter Rust grammar (web-tree-sitter) is the independent third-party oracle.",
 'import Parser from "web-tree-sitter";\nimport {createRequire} from "node:module";\nimport {dirname,join} from "node:path";\nimport {policyExtractRustSchemaFields,policyParseRustFieldType} from "@@' + UR + '/🟦.ts@@";', body_r, python=False)
ts_r = ts_r.replace('@@📦tree-sitter-rust.wasm@@', '@@WASM_RUST@@')
lib.write_unit(UR, corpus_r, sch_r, ts_r)
# ======================= graphql
UG = "🧬schema/🔍field-discovery/🔗graphql"
gql_src = '''"""Block doc for the schema"""
type Widget {
  "single line doc"
  id: String!
  displayName: String
  count: Int!
  big: UInt64
  ratio: Float!
  enabled: Boolean!
  tags: [String!]!
  names: [String]
  labels: [LabelsEntry!]!
  blob: Bytes
  status: String! @state(class: PERSISTED_SHARED)
  filtered(first: Int = 10, after: String): [Item!]
  # comment: Fake!
  ref: Other!
}

type Other { zzz: String! }

type Empty
'''
gql_fields = [fld("id", False, "scalar", "string"), fld("displayName", True, "scalar", "string"), fld("count", False, "scalar", "int32"), fld("big", True, "scalar", "UInt64"), fld("ratio", False, "scalar", "float64"), fld("enabled", False, "scalar", "bool"), fld("tags", False, "list", "string"), fld("names", True, "list", "string"),
              fld("labels", False, "map", "Labels"), fld("blob", True, "scalar", "Bytes"), fld("status", False, "scalar", "string", "persisted-shared"), fld("filtered", True, "list", "Item"), fld("ref", False, "scalar", "Other")]
gx = [
 {"name": "the named type with every field shape", "text": gql_src, "type": "Widget", "expected": {"typeName": "Widget", "fields": gql_fields}, "oracle": True},
 {"name": "a second type", "text": gql_src, "type": "Other", "expected": {"typeName": "Other", "fields": [fld("zzz", False, "scalar", "string")]}, "oracle": True},
 {"name": "a braceless empty type at the end of the document", "text": gql_src, "type": "Empty", "expected": {"typeName": "Empty", "fields": []}, "oracle": True},
 {"name": "an unknown type", "text": gql_src, "type": "Nope", "expected": empty, "oracle": False},
 {"name": "no expected name", "text": gql_src, "type": None, "expected": empty, "oracle": False},
 {"name": "a comment or string with a colon is not a field", "text": 'type T {\n  # hidden: Fake!\n  "a: String"\n  real: Int\n}\n', "type": "T", "expected": {"typeName": "T", "fields": [fld("real", True, "scalar", "int32")]}, "oracle": True},
 {"name": "an argument list does not leak its names", "text": "type T { f(a: Int!, b: [String!]): String! }", "type": "T", "expected": {"typeName": "T", "fields": [fld("f", False, "scalar", "string")]}, "oracle": True},
]
gql_defects = [{"name": "a braceless empty type followed by another type", "text": "type Empty\ntype Other { zzz: String! }\n", "type": "Empty", "expected": {"typeName": "Empty", "fields": []}, "defect": "🔗️graphql/🟦️.ts: the braceless-type fallback uses a negative lookahead for any later `{`, so `type Empty` followed by another type definition is read as absent (typeName empty)"}]
corpus_g = {"version": 1, "extract": gx, "defects": gql_defects}
sch_g = S.obj({"version": S.const(1), "extract": S.arr(S.obj({"name": S.str, "text": S.str, "type": S.one(S.str, S.null), "expected": EXTRACT, "oracle": S.bool}), 1), "defects": S.arr(S.obj({"name": S.str, "text": S.str, "type": S.str, "expected": EXTRACT, "defect": S.str}), 1)})
body_g = '''const scalarOf=(name:string):string=>{const table:Record<string,string>={String:"string",Int:"int32",Float:"float64",Boolean:"bool"};return Object.hasOwn(table,name)?table[name]!:name;};
const oracle=(text:string,name:string):{typeName:string;fields:unknown[]}=>{
 const definition=parse(text).definitions.find((node):node is ObjectTypeDefinitionNode=>node.kind===Kind.OBJECT_TYPE_DEFINITION&&node.name.value===name);if(definition===undefined)return{typeName:"",fields:[]};
 return{typeName:name,fields:(definition.fields??[]).map(field=>{
  let type=field.type,optional=true,cardinality="scalar",scalar="";
  if(type.kind===Kind.NON_NULL_TYPE){optional=false;type=type.type;}
  if(type.kind===Kind.LIST_TYPE){let inner=type.type;if(inner.kind===Kind.NON_NULL_TYPE)inner=inner.type;const named=(inner as NamedTypeNode).name.value;if(named.endsWith("Entry")){cardinality="map";scalar=scalarOf(named.slice(0,-"Entry".length));}else{cardinality="list";scalar=scalarOf(named);}}
  else scalar=scalarOf((type as NamedTypeNode).name.value);
  const directive=field.directives?.find(item=>item.name.value==="state"),argument=directive?.arguments?.find(item=>item.name.value==="class"),state=argument!==undefined&&argument.value.kind===Kind.ENUM?argument.value.value.toLowerCase().replace(/_/gu,"-"):"";
  return{name:field.name.value,optional,cardinality,scalar,state};
 })};
};
for(const row of corpus.extract)test("graphql extract: "+row.name,()=>{expect(policyExtractGraphqlSchemaFields(row.text,row.type)).toEqual(row.expected);});
for(const row of corpus.defects)test.failing("graphql defect: "+row.name,()=>{expect(policyExtractGraphqlSchemaFields(row.text,row.type)).toEqual(row.expected);});
test("graphql-js parses the same fields as the extractor",()=>{for(const row of corpus.extract.filter(item=>item.oracle))expect(oracle(row.text,row.type!),row.name).toEqual(row.expected);});
test("the documents the corpus extracts from are valid GraphQL",()=>{for(const row of [...corpus.extract,...corpus.defects])expect(()=>parse(row.text),row.name).not.toThrow();});'''
ts_g = lib.ts_wrap(UG, "🔗️ Contract: the GraphQL field extractor reads the fields of a named object type (non-null as required, list and `…Entry` list as list and map, `@state(class: …)` as state), ignores comments, strings and argument lists and recognises a braceless empty type; the corpus is language-agnostic and graphql-js is the independent third-party oracle.",
 'import {Kind,parse,type NamedTypeNode,type ObjectTypeDefinitionNode} from "graphql";\nimport {policyExtractGraphqlSchemaFields} from "@@' + UG + '/🟦.ts@@";', body_g, python=False)
lib.write_unit(UG, corpus_g, sch_g, ts_g)
# ======================= protobuf
UP = "🧬schema/🔍field-discovery/🛰protobuf"
pb_src = '''syntax = "proto3";
package demo;

message Widget {
  string id = 1;
  optional string display_name = 2;
  int32 count = 3;
  optional uint64 big = 4;
  double ratio = 5;
  bool enabled = 6;
  repeated string tags = 7;
  map<string, string> labels = 8;
  bytes blob = 9;
  // @state persisted_shared
  string status = 10;
  nested.Type ref = 11;
  repeated bytes chunks = 12;
  // plain comment
  optional bool flag = 13;
}

message Other { string zzz = 1; }
'''
pb_fields = [fld("id", False, "scalar", "string"), fld("displayName", True, "scalar", "string"), fld("count", False, "scalar", "int32"), fld("big", True, "scalar", "uint64"), fld("ratio", False, "scalar", "double"), fld("enabled", False, "scalar", "bool"), fld("tags", False, "list", "string"), fld("labels", False, "map", "string"),
             fld("blob", False, "scalar", "bytes"), fld("status", False, "scalar", "string", "persisted-shared"), fld("ref", False, "scalar", "nested.Type"), fld("chunks", False, "list", "bytes"), fld("flag", True, "scalar", "bool")]
px = [
 {"name": "the named message with every field shape", "text": pb_src, "type": "Widget", "expected": {"typeName": "Widget", "fields": pb_fields}, "oracle": True},
 {"name": "a second message", "text": pb_src, "type": "Other", "expected": {"typeName": "Other", "fields": [fld("zzz", False, "scalar", "string")]}, "oracle": True},
 {"name": "an unknown message", "text": pb_src, "type": "Nope", "expected": empty, "oracle": False},
 {"name": "no expected name", "text": pb_src, "type": None, "expected": empty, "oracle": False},
 {"name": "a oneof group contributes its fields", "text": "syntax = \"proto3\";\npackage demo;\nmessage M { oneof choice { string a = 1; int32 b = 2; } string c = 3; }\n", "type": "M", "expected": {"typeName": "M", "fields": [fld("a", False, "scalar", "string"), fld("b", False, "scalar", "int32"), fld("c", False, "scalar", "string")]}, "oracle": False},
]
pb_defects = [{"name": "the fields of a nested message are not fields of the outer message", "text": "syntax = \"proto3\";\npackage demo;\nmessage Outer {\n  string a = 1;\n  message Inner { string inner_field = 1; }\n}\n", "type": "Outer", "expected": {"typeName": "Outer", "fields": [fld("a", False, "scalar", "string")]}, "defect": "🛰️protobuf/🟦️.ts:policyExtractProtobufSchemaFields scans the whole message body with one global regex, so the fields of a nested `message Inner` are reported as fields of the outer message"}]
ptypes = [("string", False, False, False, "scalar", "string"), ("string", True, False, False, "scalar", "string"), ("string", False, True, False, "list", "string"), ("map<string, int32>", False, False, False, "map", "int32"), ("map < string , bytes >", True, False, False, "map", "bytes"), ("map<int32, string>", False, False, False, "scalar", "map<int32,string>"), ("  uint64 ", False, False, False, "scalar", "uint64"), ("bytes", False, True, False, "list", "bytes")]
corpus_p = {"version": 1, "types": [{"text": t, "optional": o, "repeated": r, "cardinality": c, "scalar": s} for t, o, r, _, c, s in ptypes], "extract": px, "defects": pb_defects}
sch_p = S.obj({"version": S.const(1), "types": S.arr(S.obj({"text": S.str, "optional": S.bool, "repeated": S.bool, "cardinality": S.str, "scalar": S.str}), 1), "extract": S.arr(S.obj({"name": S.str, "text": S.str, "type": S.one(S.str, S.null), "expected": EXTRACT, "oracle": S.bool}), 1), "defects": S.arr(S.obj({"name": S.str, "text": S.str, "type": S.str, "expected": EXTRACT, "defect": S.str}), 1)})
body_p = '''const scalarOf=(name:string):string=>{const table:Record<string,string>={string:"string",bool:"bool",bytes:"bytes"};return Object.hasOwn(table,name)?table[name]!:name;};
const oracle=(text:string,name:string):{typeName:string;fields:unknown[]}=>{
 const parsed=protobuf.parse(text,{alternateCommentMode:true}),type=parsed.root.lookupType((parsed.package?parsed.package+".":"")+name);
 return{typeName:name,fields:type.fieldsArray.map(field=>{
  const map=(field as unknown as {map:boolean}).map===true,state=/@state\\s+([a-z0-9_-]+)/u.exec(field.comment??"")?.[1];
  return{name:field.name,optional:(field.options as {proto3_optional?:boolean}|undefined)?.proto3_optional===true,cardinality:map?"map":field.repeated?"list":"scalar",scalar:scalarOf(field.type),state:state?state.toLowerCase().replace(/_/gu,"-"):""};
 })};
};
for(const row of corpus.types)test("proto field type "+JSON.stringify(row.text),()=>{expect(policyParseProtoFieldType(row.text,row.optional,row.repeated)).toEqual({optional:row.optional,cardinality:row.cardinality,scalar:row.scalar});});
for(const row of corpus.extract)test("protobuf extract: "+row.name,()=>{expect(policyExtractProtobufSchemaFields(row.text,row.type)).toEqual(row.expected);});
for(const row of corpus.defects)test.failing("protobuf defect: "+row.name,()=>{expect(policyExtractProtobufSchemaFields(row.text,row.type)).toEqual(row.expected);});
test("protobufjs parses the same fields as the extractor",()=>{for(const row of corpus.extract.filter(item=>item.oracle))expect(oracle(row.text,row.type!),row.name).toEqual(row.expected);});
test("protobufjs agrees on the nested message the extractor leaks",()=>{for(const row of corpus.defects)expect(oracle(row.text,row.type).fields).toEqual(row.expected.fields);});'''
ts_p = lib.ts_wrap(UP, "🛰️ Contract: the Protobuf field extractor reads the fields of a named message (`optional`, `repeated`, `map<string,…>`, `// @state` comments) in declaration order; the corpus is language-agnostic and protobufjs is the independent third-party oracle for names, rules, map and optional flags and comments.",
 'import protobuf from "protobufjs";\nimport {policyExtractProtobufSchemaFields,policyParseProtoFieldType} from "@@' + UP + '/🟦.ts@@";', body_p, python=False)
lib.write_unit(UP, corpus_p, sch_p, ts_p)
