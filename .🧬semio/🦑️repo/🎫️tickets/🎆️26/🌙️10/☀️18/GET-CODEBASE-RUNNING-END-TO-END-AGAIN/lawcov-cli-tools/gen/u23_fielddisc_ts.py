import lib
from lib import S
def fld(name, optional, card, scalar, state=""):
    return {"name": name, "optional": optional, "cardinality": card, "scalar": scalar, "state": state}
FLD = S.obj({"name": S.str, "optional": S.bool, "cardinality": S.enum("scalar", "list", "fixedList", "map"), "scalar": S.str, "state": S.str})
EXTRACT = S.obj({"typeName": S.str, "fields": S.arr(FLD)})
empty = {"typeName": "", "fields": []}
UT = "🧬schema/🔍field-discovery/🟦typescript"
ts_src = '''export interface Widget {
  id: string;
  displayName?: string;
  count: number;
  big: number | undefined;
  nullable: string | null;
  tags: string[];
  names: Array<string>;
  pair: [number, number];
  labels: Record<string, string>;
  blob: Uint8Array;
  /** @state persisted_shared */
  status: string;
  "quoted-key": boolean;
  'single': number;
  ref: Other;
  id: number;
}

export interface Other { zzz: string; }
'''
ts_fields = [fld("id", False, "scalar", "string"), fld("displayName", True, "scalar", "string"), fld("count", False, "scalar", "number"), fld("big", True, "scalar", "number"), fld("nullable", True, "scalar", "string"), fld("tags", False, "list", "string"), fld("names", False, "list", "string"), fld("pair", False, "fixedList", "number"),
             fld("labels", False, "map", "string"), fld("blob", False, "scalar", "Uint8Array"), fld("status", False, "scalar", "string", "persisted-shared"), fld("quoted-key", False, "scalar", "bool"), fld("single", False, "scalar", "number"), fld("ref", False, "scalar", "Other")]
inherit = '''interface Base { baseField: string; shared: number; }
interface Mixin { mixed: boolean; }
export interface Child extends Base, Mixin { own: string; }
export interface Missing extends Unknown { x: string; }
export type Alias = Child;
export type Chain = Alias;
'''
types = [("string", False, False, "scalar", "string"), ("string;", False, False, "scalar", "string"), ("string", True, False, "scalar", "string"), ("string | undefined", False, False, "scalar", "string"), ("string | null", False, False, "scalar", "string"), ("  string  |  undefined ", False, False, "scalar", "string"),
         ("string[]", False, False, "list", "string"), ("Array<string>", False, False, "list", "string"), ("Array < Foo >", False, False, "list", "Foo"), ("Record<string, number>", False, False, "map", "number"), ("Record < string , boolean > | undefined", False, False, "map", "bool"),
         ("[number, number]", False, False, "fixedList", "number"), ("[number, number, number]", False, False, "fixedList", "number"), ("[number, string]", False, False, "scalar", "[number,string]"), ("[number]", False, False, "scalar", "[number]"), ("Foo[] | undefined", False, False, "list", "Foo"),
         ("boolean", False, False, "scalar", "bool"), ("Map<string, number>", False, False, "scalar", "Map<string,number>"), ("string[][]", False, False, "list", "string[]"), ("string | number", False, False, "scalar", "string|number")]
# optional true expected when | undefined/null or mark
types_expected = []
for t, mark, _, c, s in types:
    n = " ".join(t.split()).rstrip(";").strip()
    opt = mark or n.endswith("| undefined") or n.endswith("| null")
    types_expected.append({"text": t, "mark": mark, "optional": opt, "cardinality": c, "scalar": s})
mods = {"/w/a.ts": "export interface Widget { a: string; }\nexport interface Hidden { h: number; }\n", "/w/b.ts": "export { Widget as Public } from \"./a.ts\";\nexport type { Hidden } from \"./a.ts\";\nexport type Direct = Widget;\n", "/w/c.ts": "import { Widget } from \"./a.ts\";\nimport type { Hidden as Renamed } from \"./a.ts\";\nexport type Imported = Widget;\nexport type RenamedAlias = Renamed;\nexport interface Local { l: string; }\nexport { Local as Exposed };\n",
        "/w/d.ts": "import { Widget } from \"./a.ts\";\nexport interface Mixed extends Widget { own: number; }\n"}
modcases = [
 {"name": "a re-export with a rename", "module": "/w/b.ts", "type": "Public", "expected": {"typeName": "Public", "fields": [fld("a", False, "scalar", "string")]}},
 {"name": "a type-only re-export", "module": "/w/b.ts", "type": "Hidden", "expected": {"typeName": "Hidden", "fields": [fld("h", False, "scalar", "number")]}},
 {"name": "an exported alias of an unexported name finds nothing through the local scope", "module": "/w/b.ts", "type": "Direct", "expected": empty},
 {"name": "an exported alias of an imported name", "module": "/w/c.ts", "type": "Imported", "expected": {"typeName": "Imported", "fields": [fld("a", False, "scalar", "string")]}},
 {"name": "an alias of a renamed import", "module": "/w/c.ts", "type": "RenamedAlias", "expected": {"typeName": "RenamedAlias", "fields": [fld("h", False, "scalar", "number")]}},
 {"name": "a local interface exported under another name", "module": "/w/c.ts", "type": "Exposed", "expected": {"typeName": "Exposed", "fields": [fld("l", False, "scalar", "string")]}},
 {"name": "an interface extending an imported one inherits its fields", "module": "/w/d.ts", "type": "Mixed", "expected": {"typeName": "Mixed", "fields": [fld("own", False, "scalar", "number"), fld("a", False, "scalar", "string")]}},
 {"name": "an unresolvable specifier finds nothing", "module": "/w/b.ts", "type": "Nowhere", "expected": empty},
]
extr = [
 {"name": "an interface with every property shape", "text": ts_src, "type": "Widget", "expected": {"typeName": "Widget", "fields": ts_fields}, "oracle": True},
 {"name": "a second interface in the same file", "text": ts_src, "type": "Other", "expected": {"typeName": "Other", "fields": [fld("zzz", False, "scalar", "string")]}, "oracle": True},
 {"name": "an unknown interface", "text": ts_src, "type": "Nope", "expected": empty, "oracle": False},
 {"name": "no expected name", "text": ts_src, "type": None, "expected": empty, "oracle": False},
 {"name": "an interface extending local interfaces lists its own fields first", "text": inherit, "type": "Child", "expected": {"typeName": "Child", "fields": [fld("own", False, "scalar", "string"), fld("baseField", False, "scalar", "string"), fld("shared", False, "scalar", "number"), fld("mixed", False, "scalar", "bool")]}, "oracle": False},
 {"name": "an interface extending an unknown one cannot be resolved", "text": inherit, "type": "Missing", "expected": empty, "oracle": False},
 {"name": "an alias resolves to the aliased interface", "text": inherit, "type": "Alias", "expected": {"typeName": "Alias", "fields": [fld("own", False, "scalar", "string"), fld("baseField", False, "scalar", "string"), fld("shared", False, "scalar", "number"), fld("mixed", False, "scalar", "bool")]}, "oracle": False},
 {"name": "an alias of an alias resolves transitively", "text": inherit, "type": "Chain", "expected": {"typeName": "Chain", "fields": [fld("own", False, "scalar", "string"), fld("baseField", False, "scalar", "string"), fld("shared", False, "scalar", "number"), fld("mixed", False, "scalar", "bool")]}, "oracle": False},
 {"name": "a non-exported interface is not a schema", "text": "interface Hidden { a: string; }\n", "type": "Hidden", "expected": empty, "oracle": False},
 {"name": "readonly members and methods", "text": "export interface R { readonly a: string; method(): void; b?: number; }\n", "type": "R", "expected": {"typeName": "R", "fields": [fld("a", False, "scalar", "string"), fld("b", True, "scalar", "number")]}, "oracle": True},
]
defects = [
 {"name": "a line comment that looks like a member is not a field", "text": "export interface C {\n  // legacy: string;\n  real: number;\n}\n", "type": "C", "expected": {"typeName": "C", "fields": [fld("real", False, "scalar", "number")]}, "defect": "🟦️typescript/🟦️.ts:policyExtractTypescriptSchemaFields scans the body with a regex and does not skip comments, so `// legacy: string;` becomes a field"},
 {"name": "an inline object type is one field, not several", "text": "export interface D {\n  inline: { a: string; b: number };\n  after: boolean;\n}\n", "type": "D", "expected": {"typeName": "D", "fields": [fld("inline", False, "scalar", "{a:string;b:number}"), fld("after", False, "scalar", "bool")]}, "defect": "🟦️typescript/🟦️.ts: the member regex ends a type at the first `;`, so an inline object type yields a truncated type and its inner members leak as top-level fields (`b`)"},
]
defects.append({"name": "the last member may omit its semicolon", "text": "export interface Other { zzz: string }\n", "type": "Other", "expected": {"typeName": "Other", "fields": [fld("zzz", False, "scalar", "string")]}, "defect": "🟦️typescript/🟦️.ts: every member must end with `;`, so a member without a trailing semicolon (valid TypeScript, e.g. the last member of a one-line interface) is dropped"})
defects.append({"name": "members separated by newlines only", "text": "export interface Asi {\n  a: string\n  b: number\n}\n", "type": "Asi", "expected": {"typeName": "Asi", "fields": [fld("a", False, "scalar", "string"), fld("b", False, "scalar", "number")]}, "defect": "🟦️typescript/🟦️.ts: semicolon-free interfaces (valid, common style) merge members into one mistyped field"})
defects[1]["expected"] = {"typeName": "D", "fields": [fld("inline", False, "scalar", "{ a: string; b: number }"), fld("after", False, "scalar", "boolean")]}
corpus = {"version": 1, "types": types_expected, "extract": extr, "defects": defects, "modules": {"files": mods, "cases": modcases}, "filePath": {"files": {"a.ts": "export interface Widget { a: string; }\n", "b.ts": "export { Widget as Public } from \"./a.ts\";\n", "c.ts": "export { Widget as Deep } from \"./b\";\n"}, "entry": "b.ts", "type": "Public", "expected": {"typeName": "Public", "fields": [fld("a", False, "scalar", "string")]}, "missing": {"entry": "x.ts", "text": "export { Widget as Gone } from \"./nothing.ts\";\n", "type": "Gone"}, "packageSpecifier": "export { Widget as Pkg } from \"pkg\";\n"}}
sch = S.obj({"version": S.const(1), "types": S.arr(S.obj({"text": S.str, "mark": S.bool, "optional": S.bool, "cardinality": S.str, "scalar": S.str}), 1),
 "extract": S.arr(S.obj({"name": S.str, "text": S.str, "type": S.one(S.str, S.null), "expected": EXTRACT, "oracle": S.bool}), 1), "defects": S.arr(S.obj({"name": S.str, "text": S.str, "type": S.str, "expected": EXTRACT, "defect": S.str}), 1),
 "modules": S.obj({"files": S.map(S.str), "cases": S.arr(S.obj({"name": S.str, "module": S.str, "type": S.str, "expected": EXTRACT}), 1)}),
 "filePath": S.obj({"files": S.map(S.str), "entry": S.str, "type": S.str, "expected": EXTRACT, "missing": S.obj({"entry": S.str, "text": S.str, "type": S.str}), "packageSpecifier": S.str})})
body = '''const scalarOf=(text:string):string=>{const token=text.replace(/\\s+/gu,""),table:Record<string,string>={string:"string",boolean:"bool"};return Object.hasOwn(table,token)?table[token]!:token;};
const oracle=(text:string,name:string):{typeName:string;fields:unknown[]}=>{
 const source=ts.createSourceFile("x.ts",text,ts.ScriptTarget.ES2022,true),declaration=source.statements.find((node):node is ts.InterfaceDeclaration=>ts.isInterfaceDeclaration(node)&&node.name.text===name&&(ts.getCombinedModifierFlags(node)&ts.ModifierFlags.Export)!==0);
 if(declaration===undefined)return{typeName:"",fields:[]};
 const fields:unknown[]=[],seen=new Set<string>();
 for(const member of declaration.members){
  if(!ts.isPropertySignature(member))continue;
  const key=member.name.getText(source).replace(/^["']|["']$/gu,"");if(seen.has(key))continue;seen.add(key);
  let type=member.type!,optional=member.questionToken!==undefined;
  if(ts.isUnionTypeNode(type)){const parts=type.types.filter(part=>part.kind!==ts.SyntaxKind.UndefinedKeyword&&!(ts.isLiteralTypeNode(part)&&part.literal.kind===ts.SyntaxKind.NullKeyword));if(parts.length!==type.types.length){optional=true;if(parts.length===1)type=parts[0]!;}}
  let cardinality="scalar",scalar:string;
  if(ts.isTypeReferenceNode(type)&&type.typeName.getText(source)==="Record"&&type.typeArguments?.length===2){cardinality="map";scalar=scalarOf(type.typeArguments[1]!.getText(source));}
  else if(ts.isArrayTypeNode(type)){cardinality="list";scalar=scalarOf(type.elementType.getText(source));}
  else if(ts.isTypeReferenceNode(type)&&type.typeName.getText(source)==="Array"&&type.typeArguments?.length===1){cardinality="list";scalar=scalarOf(type.typeArguments[0]!.getText(source));}
  else if(ts.isTupleTypeNode(type)&&type.elements.length>1&&type.elements.every(element=>element.getText(source)===type.elements[0]!.getText(source))){cardinality="fixedList";scalar=scalarOf(type.elements[0]!.getText(source));}
  else scalar=scalarOf(type.getText(source));
  const tag=ts.getJSDocTags(member).find(item=>item.tagName.text==="state"),state=tag===undefined?"":String(tag.comment??"").trim().toLowerCase().replace(/_/gu,"-");
  fields.push({name:key,optional,cardinality,scalar,state});
 }
 return{typeName:name,fields};
};
const resolver=(files:Record<string,string>)=>(specifier:string,from:string)=>{if(!specifier.startsWith("."))return null;const target=resolve(dirname(from),specifier),found=target in files?target:target+".ts" in files?target+".ts":null;return found===null?null:{moduleId:found,text:files[found]!};};
const posixResolve=(path:string):string=>path;
for(const[index,row]of corpus.types.entries())test("ts field type "+index+": "+JSON.stringify(row.text),()=>{expect(policyParseTsFieldType(row.text,row.mark)).toEqual({optional:row.optional,cardinality:row.cardinality,scalar:row.scalar});});
for(const row of corpus.extract)test("typescript extract: "+row.name,()=>{expect(policyExtractTypescriptSchemaFields(row.text,row.type)).toEqual(row.expected);});
for(const row of corpus.defects)test.failing("typescript defect: "+row.name,()=>{expect(policyExtractTypescriptSchemaFields(row.text,row.type)).toEqual(row.expected);});
test("the TypeScript compiler API reads the same members as the extractor",()=>{for(const row of corpus.extract.filter(item=>item.oracle))expect(oracle(row.text,row.type!),row.name).toEqual(row.expected);});
test("the compiler API agrees with the truth the defect cases expect",()=>{const row=corpus.defects[0]!;expect(oracle(row.text,row.type).fields).toEqual(row.expected.fields);});
for(const row of corpus.modules.cases)test("module resolution: "+row.name,()=>{
 const files:Record<string,string>={};for(const[path,text]of Object.entries(corpus.modules.files))files[resolve(path)]=text;const module=resolve(row.module);
 expect(policyExtractTypescriptSchemaFields(files[module]!,row.type,resolver(files),module)).toEqual(row.expected);
});
test("a module cycle terminates",()=>{
 const files:Record<string,string>={[resolve("/w/x.ts")]:'export { Loop as Loop } from "./y.ts";\\n',[resolve("/w/y.ts")]:'export { Loop as Loop } from "./x.ts";\\n'};
 expect(policyExtractTypescriptSchemaFields(files[resolve("/w/x.ts")]!,"Loop",resolver(files),resolve("/w/x.ts"))).toEqual({typeName:"",fields:[]});
});
test("a file-based extraction follows relative re-exports on disk and ignores package specifiers",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"tsfields-")));
 try{
  for(const[path,text]of Object.entries(corpus.filePath.files))writeFileSync(join(root,path),text);
  const entry=join(root,corpus.filePath.entry);expect(policyExtractTypescriptSchemaFile(entry,readFileSync(entry,"utf8"),corpus.filePath.type)).toEqual(corpus.filePath.expected);
  expect(policyExtractTypescriptSchemaFile(join(root,corpus.filePath.missing.entry),corpus.filePath.missing.text,corpus.filePath.missing.type)).toEqual({typeName:"",fields:[]});
  expect(policyExtractTypescriptSchemaFile(join(root,"pkg.ts"),corpus.filePath.packageSpecifier,"Pkg")).toEqual({typeName:"",fields:[]});
  mkdirSync(join(root,"dir.ts"));expect(policyExtractTypescriptSchemaFile(join(root,"z.ts"),'export { W as Z } from "./dir.ts";\\n',"Z")).toEqual({typeName:"",fields:[]});
 }finally{rmSync(root,{recursive:true,force:true});}
});'''
ts_ = lib.ts_wrap(UT, "🟦️ Contract: the TypeScript field extractor reads the members of an exported interface (optional marks, `| undefined`/`| null`, arrays, `Record`, homogeneous tuples, `@state` JSDoc), follows `extends`, aliases, imports and re-exports through a resolver without executing code and reads relative modules from disk; the corpus is language-agnostic and the TypeScript compiler API is the independent third-party oracle.",
 'import ts from "typescript";\nimport {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {dirname,join,resolve} from "node:path";\nimport {policyExtractTypescriptSchemaFields,policyParseTsFieldType} from "@@' + UT + '/🟦.ts@@";\nimport {policyExtractTypescriptSchemaFile} from "@@' + UT + '/📂module-resolution/🟦.ts@@";', body, python=False)
lib.write_unit(UT, corpus, sch, ts_)
