import json, copy, re, lib
from lib import S
UNIT = "🏗authoring/🗂collection-catalog"
V = "️"
base = {
 "x-semio-collection-catalog": {
  "keyType": "ShapeKey", "rowType": "ShapeRow", "labelType": "ShapeLabel", "tag": "kind", "content": "row", "headerField": "header",
  "rust": {"snapshot": "crate::model::Snapshot", "diff": "crate::model::SnapshotDiff", "deltas": "crate::deltas", "rows": "crate::rows", "header": "crate::model::Header", "id": "crate::model::Id", "canonicalJsonOwner": "Snapshot"},
  "typescript": {"rows": "../rows/index.ts"}, "output": {"rust": "../out/catalog/🦀" + V + ".rs", "typescript": "../out/🟦" + V + ".ts"},
  "collections": [
   {"key": "nodes", "variant": "Nodes", "entity": "node", "row": "NodeRow", "patch": "NodePatch", "delta": "NodeDelta", "rowsField": "nodes", "diffField": "nodeDeltas", "label": {"article": "a", "en": "node", "de": "Knoten"}},
   {"key": "edges", "variant": "Edges", "entity": "edge", "row": "EdgeRow", "patch": "EdgePatch", "delta": "EdgeDelta", "rowsField": "edges", "diffField": "edgeDeltas", "label": {"article": "an", "en": "edge", "de": "Kante"}}]},
 "$defs": {"ShapeKey": {"enum": ["nodes", "edges"]}, "ShapeRow": {"oneOf": [{"properties": {"kind": {"const": "nodes"}, "row": {"$ref": "#/$defs/NodeRow"}}}, {"properties": {"kind": {"const": "edges"}, "row": {"$ref": "#/$defs/EdgeRow"}}}]}}}
cat = base["x-semio-collection-catalog"]
parsed = copy.deepcopy(cat)
X = "x-semio-collection-catalog"
def patch(op, path, value=None, message=""):
    return {"op": op, "path": path, "value": value, "message": message}
C0 = X + ".collections.0."
invalid = [
 ("a key that is not camelCase", patch("set", C0 + "key", "Nodes"), "collection catalog key is invalid"),
 ("a variant that is not PascalCase", patch("set", C0 + "variant", "nodes"), "collection catalog variant is invalid"),
 ("an entity that is not kebab-case", patch("set", C0 + "entity", "Node"), "collection catalog entity is invalid"),
 ("a row type with an underscore", patch("set", C0 + "row", "Node_Row"), "collection catalog row is invalid"),
 ("a rows field that is not an identifier", patch("set", C0 + "rowsField", "1nodes"), "collection catalog rowsField is invalid"),
 ("an article other than a or an", patch("set", C0 + "label.article", "the"), "article must be a or an"),
 ("an empty English label", patch("set", C0 + "label.en", ""), "needs English and German labels"),
 ("a missing German label", patch("delete", C0 + "label.de"), "needs English and German labels"),
 ("duplicate keys", patch("set", X + ".collections.1.key", "nodes"), "key values must be unique"),
 ("duplicate diff fields", patch("set", X + ".collections.1.diffField", "nodeDeltas"), "diffField values must be unique"),
 ("no collections", patch("set", X + ".collections", []), "declares no collections"),
 ("a key type that is not PascalCase", patch("set", X + ".keyType", "shapeKey"), "keyType is invalid"),
 ("a tag that is not camelCase", patch("set", X + ".tag", "Kind"), "tag is invalid"),
 ("a Rust path with a capitalised crate", patch("set", X + ".rust.snapshot", "Crate::model::Snapshot"), "snapshot is invalid"),
 ("a Rust path with a trailing separator", patch("set", X + ".rust.diff", "crate::"), "diff is invalid"),
 ("a canonical JSON owner that is a path", patch("set", X + ".rust.canonicalJsonOwner", "a::B"), "canonicalJsonOwner is invalid"),
 ("a TypeScript rows entry that is not a relative ts file", patch("set", X + ".typescript.rows", "rows.ts"), "rows is invalid"),
 ("an absolute Rust output", patch("set", X + ".output.rust", "/abs/out.rs"), "rust is invalid"),
 ("a hidden TypeScript output segment", patch("set", X + ".output.typescript", "../.hidden/x.ts"), "typescript is invalid"),
 ("a key enum in the wrong order", patch("set", "$defs.ShapeKey.enum", ["edges", "nodes"]), "enum must list the catalog keys in order"),
 ("a row union with a missing alternative", patch("set", "$defs.ShapeRow.oneOf", [{"properties": {"kind": {"const": "nodes"}, "row": {"$ref": "#/$defs/NodeRow"}}}]), "oneOf must hold one alternative per collection"),
 ("a row union tagging the wrong key", patch("set", "$defs.ShapeRow.oneOf.1.properties.kind.const", "nodes"), "must tag edges"),
 ("a row union referencing the wrong row", patch("set", "$defs.ShapeRow.oneOf.0.properties.row.$ref", "#/$defs/Other"), "must reference the NodeRow row"),
 ("a row union reference that is not a string", patch("set", "$defs.ShapeRow.oneOf.0.properties.row.$ref", 7), "must reference the NodeRow row"),
 ("a missing extension", patch("delete", X), "x-semio-collection-catalog must be an object"),
 ("a missing definitions table", patch("delete", "$defs"), "$defs must be an object"),
 ("a missing rust table", patch("delete", X + ".rust"), "rust must be an object"),
 ("a collection that is not an object", patch("set", X + ".collections.0", 7), "collection 0 must be an object"),
]
def mini_ts():
    c = cat["collections"]
    specs = {"./x": 0}
    tsd = []
    imports = "import type { EdgeRow, NodeRow } from \"../rows/index.ts\";"
    return imports
owners_corpus = {"EdgeRow": "../rows/edge.ts", "NodeRow": "../rows/node.ts"}
def render_ts(owners):
    c = cat["collections"]
    by = {}
    for e in c:
        by.setdefault(owners[e["row"]], set()).add(e["row"])
    imports = "\n".join('import type { %s } from "%s";' % (", ".join(sorted(by[s])), s) for s in sorted(by))
    K, R = cat["keyType"], cat["rowType"]
    const = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", re.sub(r"Key$", "", K)).upper() + "S"
    def lab(l):
        return "{article: %s,en: %s,de: %s}" % (json.dumps(l["article"]), json.dumps(l["en"], ensure_ascii=False), json.dumps(l["de"], ensure_ascii=False))
    return ("/** 🤖" + V + " Generated by the collection-catalog generator (`📚" + V + "library/🏗" + V + "authoring/🗂" + V + "collection-catalog`) from the `x-semio-collection-catalog` extension of its snapshot schema. Edit the schema and regenerate. */\n" + imports + "\n\n"
            "/** 🗂" + V + " Key of one id-keyed collection. */\nexport type " + K + " = " + " | ".join(json.dumps(e["key"]) for e in c) + ";\n\n"
            "/** 🗂" + V + " One typed row of one collection, adjacently tagged by its key. */\nexport type " + R + " =\n" + "\n".join("  | { %s: %s; %s: %s }" % (cat["tag"], json.dumps(e["key"]), cat["content"], e["row"]) for e in c) + ";\n\n"
            "/** 🗂" + V + " One collection: its key, entity, snapshot and diff fields, and row labels. */\nexport interface " + K + "Entry { readonly key: " + K + "; readonly entity: string; readonly rowsField: string; readonly diffField: string; readonly label: { readonly article: \"a\" | \"an\"; readonly en: string; readonly de: string } }\n\n"
            "/** 🗂" + V + " Every collection, in declaration order. */\nexport const " + const + ": readonly " + K + "Entry[] = [\n" + "\n".join("  { key: %s, entity: %s, rowsField: %s, diffField: %s, label: %s }," % (json.dumps(e["key"]), json.dumps(e["entity"]), json.dumps(e["rowsField"]), json.dumps(e["diffField"]), lab(e["label"])) for e in c) + "\n];\n")
rust_contains = {
 "root": ["use crate::model::Snapshot;", "use crate::model::SnapshotDiff;", "use crate::model::Header;", "use crate::model::Id;", "use crate::deltas::{EdgeDelta, NodeDelta};", "use crate::rows::{EdgePatch, EdgeRow, NodePatch, NodeRow};", "#[canonical_json(owner = Snapshot)]",
          "#[value(tag = \"kind\", content = \"row\", rename_all = \"camelCase\")]", "pub enum ShapeKey {\n    Nodes,\n    Edges,\n}", "pub enum ShapeRow {\n    Nodes(NodeRow),\n    Edges(EdgeRow),\n}", "pub struct ShapeLabel {", "pub const ALL: [ShapeKey; 2] = [ShapeKey::Nodes, ShapeKey::Edges];",
          "#[path = \"🪪" + V + "key-descriptors/🦀" + V + ".rs\"]\nmod key_descriptors;", "#[path = \"🔎" + V + "key-base-access/🦀" + V + ".rs\"]\nmod key_base_access;", "#[path = \"🧾" + V + "row-operations/🦀" + V + ".rs\"]\nmod row_operations;", "#[path = \"🔁" + V + "row-conversions/🦀" + V + ".rs\"]\nmod row_conversions;"],
 "🪪" + V + "key-descriptors": ["            Self::Nodes => \"node\",", "            Self::Edges => \"edge\",", "Self::Nodes => ShapeLabel { article: \"a\", en: \"node\", de: \"Knoten\" },", "Self::Edges => ShapeLabel { article: \"an\", en: \"edge\", de: \"Kante\" },", "            Self::Nodes => \"nodeDeltas\",", "            \"edgeDeltas\" => Some(Self::Edges),", "            _ => None,"],
 "🔎" + V + "key-base-access": ["pub fn position(self, base: &Snapshot, id: &Id) -> Option<usize> {", "Self::Nodes => base.nodes.iter().position(|row| row.header.id == *id),", "Self::Edges => base.edges.len(),", "Self::Nodes => base.nodes.get(index).cloned().map(ShapeRow::Nodes),", "Self::Edges => SnapshotDiff { edgeDeltas: Some(EdgeDelta::removal(&base.edges, index)), ..Default::default() },", "Self::Nodes => SnapshotDiff { nodeDeltas: Some(NodeDelta::modification(id.0.clone(), NodePatch { name: Some(name), ..Default::default() })), ..Default::default() },"],
 "🧾" + V + "row-operations": ["Self::Nodes(_) => ShapeKey::Nodes,", "Self::Edges(row) => &row.header,", "Self::Edges(row) => SnapshotDiff { edgeDeltas: Some(EdgeDelta::insertion(index, row.clone())), ..Default::default() },", "let mut delta = NodeDelta::removal(&base.nodes, index);", "delta.absorb(NodeDelta::insertion(index, row.clone()));", "Self::Nodes(row) => base.nodes.get(index) == Some(row),"],
 "🔁" + V + "row-conversions": ["impl From<NodeRow> for ShapeRow {", "        Self::Nodes(row)", "impl From<EdgeRow> for ShapeRow {", "        Self::Edges(row)"],
}
owner_cases = [
 {"name": "each row type is imported from the module that declares it", "output": "out/x.ts", "rows": "../rows/index.ts", "files": {"rows/index.ts": "export type { NodeRow } from \"./node.ts\";\nexport * from \"./edge.ts\";\n", "rows/node.ts": "export interface NodeRow { id: string }\n", "rows/edge.ts": "export type EdgeRow = { id: string };\n"}, "expected": {"EdgeRow": "../rows/edge.ts", "NodeRow": "../rows/node.ts"}},
 {"name": "a module in the output directory is imported with a dot prefix", "output": "out/x.ts", "rows": "./rows.ts", "files": {"out/rows.ts": "export interface NodeRow {}\nexport declare class EdgeRow {}\n"}, "expected": {"EdgeRow": "./rows.ts", "NodeRow": "./rows.ts"}},
 {"name": "the generated catalog itself is left out of the tree", "output": "out/x.ts", "rows": "./rows.ts", "files": {"out/rows.ts": "export interface NodeRow {}\nexport enum EdgeRow { A }\n", "out/x.ts": "export interface NodeRow {}\nexport interface EdgeRow {}\n"}, "expected": {"EdgeRow": "./rows.ts", "NodeRow": "./rows.ts"}},
]
owner_errors = [
 {"name": "a row type declared twice", "output": "out/x.ts", "rows": "../rows/index.ts", "files": {"rows/index.ts": "export * from \"./a.ts\";\nexport * from \"./b.ts\";\n", "rows/a.ts": "export interface NodeRow {}\nexport interface EdgeRow {}\n", "rows/b.ts": "export type NodeRow = {};\n"}, "message": "must be declared by exactly one module below ../rows/index.ts, found 2"},
 {"name": "a row type declared nowhere", "output": "out/x.ts", "rows": "../rows/index.ts", "files": {"rows/index.ts": "export interface NodeRow {}\n"}, "message": "row type EdgeRow must be declared by exactly one module below ../rows/index.ts, found 0"},
 {"name": "a missing rows module", "output": "out/x.ts", "rows": "../rows/index.ts", "files": {}, "message": "does not name a TypeScript module"},
 {"name": "a row module that imports the catalog closes a cycle", "output": "out/x.ts", "rows": "../rows/index.ts", "files": {"rows/index.ts": "export * from \"./a.ts\";\n", "rows/a.ts": "import type { Z } from \"../out/x.ts\";\nexport interface NodeRow {}\nexport interface EdgeRow {}\n"}, "message": "would close a cycle"},
]
gen_files = {"rows/index.ts": "export interface NodeRow { id: string }\nexport interface EdgeRow { id: string }\n"}
generation = {"schemaPath": "schema/🔣" + V + ".json", "files": gen_files, "expectedStaleCount": 6, "tsPath": "out/🟦" + V + ".ts", "rustPaths": ["out/catalog/🦀" + V + ".rs"] + ["out/catalog/" + c + "/🦀" + V + ".rs" for c in ["🪪" + V + "key-descriptors", "🔎" + V + "key-base-access", "🧾" + V + "row-operations", "🔁" + V + "row-conversions"]]}
corpus = {"version": 1, "document": base, "parsed": parsed,
 "invalid": [{"name": n, "patch": p, "message": m} for n, p, m in invalid], "notDocuments": [None, [], "text", 3],
 "ownersDemo": owners_corpus, "renderTs": render_ts({"NodeRow": "../rows/node.ts", "EdgeRow": "../rows/edge.ts"}), "renderTsOwners": {"NodeRow": "../rows/node.ts", "EdgeRow": "../rows/edge.ts"},
 "renderTsSameModule": render_ts({"NodeRow": "../rows/index.ts", "EdgeRow": "../rows/index.ts"}), "rust": rust_contains, "ownerCases": owner_cases, "ownerErrors": owner_errors, "generation": generation}
for k in ("ownersDemo",):
    corpus.pop(k)
doc_s = S.anyv
schema = S.obj({"version": S.const(1), "document": doc_s, "parsed": doc_s, "invalid": S.arr(S.obj({"name": S.str, "patch": S.obj({"op": S.enum("set", "delete"), "path": S.str, "value": S.anyv, "message": S.str}), "message": S.str}), 1), "notDocuments": S.arr(S.anyv, 1),
 "renderTs": S.str, "renderTsOwners": S.map(S.str), "renderTsSameModule": S.str, "rust": S.map(S.strs(1)),
 "ownerCases": S.arr(S.obj({"name": S.str, "output": S.str, "rows": S.str, "files": S.map(S.str), "expected": S.map(S.str)}), 1), "ownerErrors": S.arr(S.obj({"name": S.str, "output": S.str, "rows": S.str, "files": S.map(S.str), "message": S.str}), 1),
 "generation": S.obj({"schemaPath": S.str, "files": S.map(S.str), "expectedStaleCount": S.int, "tsPath": S.str, "rustPaths": S.strs(1)})})
bodyts = '''const clone=<T,>(value:T):T=>JSON.parse(JSON.stringify(value));
const apply=(document:any,change:{op:string;path:string;value?:unknown}):any=>{
 const keys=change.path.split(".");let cursor=document;for(const key of keys.slice(0,-1))cursor=cursor[key];const last=keys[keys.length-1]!;
 if(change.op==="delete")delete cursor[last];else cursor[last]=change.value;return document;
};
const place=(root:string,files:Record<string,string>):void=>{for(const[path,text]of Object.entries(files)){const target=join(root,...path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);}};
const reader=(root:string,files:Record<string,string>)=>{const table=new Map(Object.entries(files).map(([path,text])=>[resolve(root,...path.split("/")),text]));return(path:string):string|undefined=>table.get(resolve(path));};
const owners=():Map<string,string>=>new Map(Object.entries(corpus.renderTsOwners));
test("a complete document is admitted as the declared catalog",()=>{expect(parseCollectionCatalogV1(corpus.document)).toEqual(corpus.parsed);});
for(const row of corpus.invalid)test("parse refuses "+row.name,()=>{expect(()=>parseCollectionCatalogV1(apply(clone(corpus.document),row.patch))).toThrow(row.message);});
for(const[index,value]of corpus.notDocuments.entries())test("parse refuses the non-document "+index,()=>{expect(()=>parseCollectionCatalogV1(value)).toThrow("schema document must be an object");});
test("the TypeScript catalog is rendered byte for byte",()=>{expect(renderCollectionCatalogTypeScriptV1(parseCollectionCatalogV1(corpus.document),owners())).toBe(corpus.renderTs);expect(renderCollectionCatalogTypeScriptV1(parseCollectionCatalogV1(corpus.document),new Map([["NodeRow","../rows/index.ts"],["EdgeRow","../rows/index.ts"]]))).toBe(corpus.renderTsSameModule);});
test("a row type without an owning module cannot be rendered",()=>{expect(()=>renderCollectionCatalogTypeScriptV1(parseCollectionCatalogV1(corpus.document),new Map([["NodeRow","./x.ts"]]))).toThrow("EdgeRow has no owning module");});
test("the generated TypeScript parses cleanly and exports the declared shapes (TypeScript compiler API)",()=>{
 const text=renderCollectionCatalogTypeScriptV1(parseCollectionCatalogV1(corpus.document),owners()),file=ts.createSourceFile("catalog.ts",text,ts.ScriptTarget.ES2022,true,ts.ScriptKind.TS);
 expect((file as unknown as {parseDiagnostics:unknown[]}).parseDiagnostics).toEqual([]);
 const names:string[]=[];file.forEachChild(node=>{if(ts.isTypeAliasDeclaration(node)||ts.isInterfaceDeclaration(node))names.push(node.name.text);else if(ts.isVariableStatement(node))for(const declaration of node.declarationList.declarations)names.push((declaration.name as ts.Identifier).text);else if(ts.isImportDeclaration(node))names.push("import:"+(node.moduleSpecifier as ts.StringLiteral).text);});
 expect(names.sort()).toEqual(["SHAPES","ShapeKey","ShapeKeyEntry","ShapeRow","import:../rows/edge.ts","import:../rows/node.ts"]);
 const emitted=ts.transpileModule(text,{reportDiagnostics:true,compilerOptions:{module:ts.ModuleKind.ESNext,target:ts.ScriptTarget.ES2022,verbatimModuleSyntax:true}});expect(emitted.diagnostics).toEqual([]);
 expect(emitted.outputText).toContain("export const SHAPES");
});
for(const[name,needles]of Object.entries(corpus.rust))test("the Rust "+(name==="root"?"root":"module "+name)+" holds every generated item",()=>{
 const files=renderCollectionCatalogRustV1(parseCollectionCatalogV1(corpus.document)),file=files.find(item=>item.folder===(name==="root"?null:name));
 expect(file,name).toBeDefined();for(const needle of needles)expect(file!.text.includes(needle),needle).toBe(true);
 expect(file!.text.startsWith("//! ")).toBe(true);
});
test("the Rust catalog is one root and four child modules",()=>{const files=renderCollectionCatalogRustV1(parseCollectionCatalogV1(corpus.document));expect(files.map(item=>item.folder)).toEqual([null,...Object.keys(corpus.rust).filter(name=>name!=="root")]);});
for(const row of corpus.ownerCases)test("row owners: "+row.name,()=>{
 const root=resolve("/w");expect(Object.fromEntries([...resolveCollectionRowOwnersV1({...parseCollectionCatalogV1(corpus.document),typescript:{rows:row.rows}},join(root,...row.output.split("/")),reader(root,row.files)).entries()].sort())).toEqual(row.expected);
});
for(const row of corpus.ownerErrors)test("row owners fail: "+row.name,()=>{
 const root=resolve("/w");expect(()=>resolveCollectionRowOwnersV1({...parseCollectionCatalogV1(corpus.document),typescript:{rows:row.rows}},join(root,...row.output.split("/")),reader(root,row.files))).toThrow(row.message);
});
test("the generator writes every output once, then reports a stale one",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"collection-")));
 try{
  place(root,corpus.generation.files);place(root,{[corpus.generation.schemaPath]:JSON.stringify(corpus.document)});const schemaPath=join(root,...corpus.generation.schemaPath.split("/"));
  expect(generateCollectionCatalogV1(schemaPath,"check").length).toBe(corpus.generation.expectedStaleCount);for(const path of[...corpus.generation.rustPaths,corpus.generation.tsPath])expect(existsSync(join(root,...path.split("/")))).toBe(false);
  expect(generateCollectionCatalogV1(schemaPath,"write").length).toBe(corpus.generation.expectedStaleCount);expect(generateCollectionCatalogV1(schemaPath,"check")).toEqual([]);
  expect(readFileSync(join(root,...corpus.generation.tsPath.split("/")),"utf8")).toBe(renderCollectionCatalogTypeScriptV1(parseCollectionCatalogV1(corpus.document),new Map([["NodeRow","../rows/index.ts"],["EdgeRow","../rows/index.ts"]])));
  expect(readFileSync(join(root,...corpus.generation.rustPaths[0]!.split("/")),"utf8")).toBe(renderCollectionCatalogRustV1(parseCollectionCatalogV1(corpus.document))[0]!.text);
  writeFileSync(join(root,...corpus.generation.rustPaths[2]!.split("/")),"// hand edit\\n");const stale=generateCollectionCatalogV1(schemaPath,"check");expect(stale.length).toBe(1);expect(stale[0]!.endsWith(join("🔎\\ufe0fkey-base-access","🦀\\ufe0f.rs"))).toBe(true);
  expect(generateCollectionCatalogV1(schemaPath,"write").length).toBe(1);expect(generateCollectionCatalogV1(schemaPath,"check")).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});'''
ts_ = lib.ts_wrap(UNIT, "🗂️ Contract: the collection catalog generator admits only a complete `x-semio-collection-catalog` schema, renders the Rust modules and the TypeScript catalog from it deterministically, finds the module that owns each row type without closing an import cycle, and detects stale generated files; the corpus is language-agnostic, the TypeScript compiler API parses the generated TypeScript and `@@O@@` re-renders the catalog independently.",
 'import ts from "typescript";\nimport {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {resolve} from "node:path";\nimport {generateCollectionCatalogV1,parseCollectionCatalogV1,renderCollectionCatalogRustV1,renderCollectionCatalogTypeScriptV1,resolveCollectionRowOwnersV1} from "@@' + UNIT + '/🟦.ts@@";', bodyts)
oracle = lib.PY_HEAD.format(doc="🐍️ Independent model of the collection catalog: validates the schema extension with its own patterns, re-renders the TypeScript catalog and derives the expected Rust items from the corpus catalog (stdlib only).") + r'''import copy

X = "x-semio-collection-catalog"
IDENT, PASCAL, CAMEL = r"^[A-Za-z_][A-Za-z0-9_]*$", r"^[A-Z][A-Za-z0-9]*$", r"^[a-z][A-Za-z0-9]*$"
KEBAB = r"^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$"
PATH = r"^(?:crate|[a-z_][a-z0-9_]*)(?:::[A-Za-z_][A-Za-z0-9_]*)*$"
OUTPUT = r"^(?:\.\./)*(?:[^/\\.][^/\\]*/)*[^/\\.][^/\\]*$"


def get(d, k):
    return d.get(k) if isinstance(d, dict) else None


def obj(v):
    if not isinstance(v, dict):
        raise ValueError("must be an object")
    return v


def txt(o, k, pat):
    v = o.get(k)
    if not isinstance(v, str) or not re.match(pat, v):
        raise ValueError("%s is invalid" % k)
    return v


def parse(doc):
    root = obj(doc)
    cat = obj(root.get(X))
    rust, ts, out = obj(cat.get("rust")), obj(cat.get("typescript")), obj(cat.get("output"))
    cols = cat.get("collections")
    if not isinstance(cols, list) or not cols:
        raise ValueError("declares no collections")
    entries = []
    for i, c in enumerate(cols):
        e = obj(c)
        lab = obj(e.get("label"))
        if lab.get("article") not in ("a", "an"):
            raise ValueError("article must be a or an")
        if not isinstance(lab.get("en"), str) or not lab["en"] or not isinstance(lab.get("de"), str) or not lab["de"]:
            raise ValueError("needs English and German labels")
        entries.append({"key": txt(e, "key", CAMEL), "variant": txt(e, "variant", PASCAL), "entity": txt(e, "entity", KEBAB), "row": txt(e, "row", PASCAL), "patch": txt(e, "patch", PASCAL), "delta": txt(e, "delta", PASCAL), "rowsField": txt(e, "rowsField", IDENT), "diffField": txt(e, "diffField", IDENT), "label": {"article": lab["article"], "en": lab["en"], "de": lab["de"]}})
    for f in ("key", "variant", "entity", "diffField"):
        if len({e[f] for e in entries}) != len(entries):
            raise ValueError("%s values must be unique" % f)
    p = {"keyType": txt(cat, "keyType", PASCAL), "rowType": txt(cat, "rowType", PASCAL), "labelType": txt(cat, "labelType", PASCAL), "tag": txt(cat, "tag", CAMEL), "content": txt(cat, "content", CAMEL), "headerField": txt(cat, "headerField", IDENT),
         "rust": {k: txt(rust, k, IDENT if k == "canonicalJsonOwner" else PATH) for k in ("snapshot", "diff", "deltas", "rows", "header", "id", "canonicalJsonOwner")}, "typescript": {"rows": txt(ts, "rows", r"^\.{1,2}/.+\.ts$")},
         "output": {"rust": txt(out, "rust", OUTPUT), "typescript": txt(out, "typescript", OUTPUT)}, "collections": entries}
    defs = obj(root.get("$defs"))
    keys, rows = obj(defs.get(p["keyType"])), obj(defs.get(p["rowType"]))
    if keys.get("enum") != [e["key"] for e in entries]:
        raise ValueError("enum must list the catalog keys in order")
    if not isinstance(rows.get("oneOf"), list) or len(rows["oneOf"]) != len(entries):
        raise ValueError("oneOf must hold one alternative per collection")
    for i, alt in enumerate(rows["oneOf"]):
        props = obj(obj(alt).get("properties"))
        if obj(props.get(p["tag"])).get("const") != entries[i]["key"]:
            raise ValueError("must tag " + entries[i]["key"])
        ref = obj(props.get(p["content"])).get("$ref")
        if not isinstance(ref, str) or not ref.endswith("/$defs/" + entries[i]["row"]):
            raise ValueError("must reference the %s row" % entries[i]["row"])
    return p


def apply(doc, ch):
    keys = ch["path"].split(".")
    cur = doc
    for k in keys[:-1]:
        cur = cur[int(k)] if isinstance(cur, list) else cur[k]
    last = keys[-1]
    if ch["op"] == "delete":
        del cur[last]
    elif isinstance(cur, list):
        cur[int(last)] = ch["value"]
    else:
        cur[last] = ch["value"]
    return doc


p = parse(corpus["document"])
if p != corpus["parsed"]:
    failures.append("parsed")
for c in corpus["invalid"]:
    try:
        parse(apply(copy.deepcopy(corpus["document"]), c["patch"]))
        failures.append("accepted " + c["name"])
    except ValueError as e:
        if c["message"].split(" ")[-1] not in str(e) and c["message"].replace("collection catalog ", "") not in str(e) and c["message"] not in str(e):
            failures.append("message " + c["name"] + ": " + str(e))
for v in corpus["notDocuments"]:
    try:
        parse(v)
        failures.append("accepted non-document")
    except ValueError:
        pass
V = "️"
c = p["collections"]
K, R = p["keyType"], p["rowType"]
for text, owners in ((corpus["renderTs"], corpus["renderTsOwners"]),):
    by = {}
    for e in c:
        by.setdefault(owners[e["row"]], set()).add(e["row"])
    lines = text.split("\n")
    imports = [l for l in lines if l.startswith("import type")]
    if imports != ['import type { %s } from "%s";' % (", ".join(sorted(by[s])), s) for s in sorted(by)]:
        failures.append("ts imports")
    if "export type %s = %s;" % (K, " | ".join(json.dumps(e["key"]) for e in c)) not in text:
        failures.append("ts key union")
    for e in c:
        if "  | { %s: %s; %s: %s }" % (p["tag"], json.dumps(e["key"]), p["content"], e["row"]) not in text:
            failures.append("ts row alt " + e["key"])
        if "{ key: %s, entity: %s, rowsField: %s, diffField: %s," % (json.dumps(e["key"]), json.dumps(e["entity"]), json.dumps(e["rowsField"]), json.dumps(e["diffField"])) not in text:
            failures.append("ts table " + e["key"])
    if not text.endswith("];\n") or "export const SHAPES:" not in text:
        failures.append("ts table const")
rust = corpus["rust"]
root_text = " ".join(rust["root"])
for e in c:
    for needle in ("%s,\n" % e["variant"], "%s(%s)" % (e["variant"], e["row"])):
        if needle not in root_text:
            failures.append("rust root " + needle)
if "pub const ALL: [%s; %d] = [%s];" % (K, len(c), ", ".join("%s::%s" % (K, e["variant"]) for e in c)) not in rust["root"]:
    failures.append("rust ALL")
if ["root"] + sorted(k for k in rust if k != "root") != ["root"] + sorted(["🪪" + V + "key-descriptors", "🔎" + V + "key-base-access", "🧾" + V + "row-operations", "🔁" + V + "row-conversions"]):
    failures.append("rust children")
''' + lib.PY_TAIL
lib.write_unit(UNIT, corpus, schema, ts_, oracle)
