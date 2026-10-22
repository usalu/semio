import json, lib
from lib import S
UNIT = "📇catalog/📣publication"
doc = {"a": {"b/c": 1, "~": 2, "": 3, "x": [10, {"y": 20}]}, "t": True, "n": None, "f": 1.5}
ptr_ok = [("/a/b~1c", 1), ("/a/~0", 2), ("/a/", 3), ("/a/x/1/y", 20), ("/t", True), ("/n", None), ("/a/x/0", 10), ("/f", 1.5), ("/a/x", [10, {"y": 20}])]
ptr_missing = ["/a/missing", "/a/x/5", "/t/x", "/n/x", "/toString", "/__proto__", "/constructor", "/a/x/01", "/a/x/y", "/f/g"]
ptr_invalid = ["", "a", "/a~", "/a~2", "~0", "/a/~"]
HEAD = "[package]\nname = \"x\"\n\n"
SUB = "[package.metadata.semio.publications]\n"
toml_ok = [("a manifest path", HEAD + SUB + 'manifest = "pub.json"\n', "pub.json"), ("a trailing comment", HEAD + SUB + 'manifest = "p.json" # note\n', "p.json"), ("comments and blank lines inside", HEAD + SUB + "# c\n\nmanifest = \"a/b.json\"\n", "a/b.json"),
           ("later tables are ignored", HEAD + SUB + 'manifest = "m.json"\n[dependencies]\nserde = "1"\n', "m.json"), ("CRLF line endings", HEAD.replace("\n", "\r\n") + SUB.replace("\n", "\r\n") + 'manifest = "w.json"\r\n', "w.json"), ("no publications table", HEAD + "[dependencies]\n", None),
           ("a sibling table is not the subtable", HEAD + "[package.metadata.semio.publications.extra]\nmanifest = \"no.json\"\n", None), ("another table before and after", "[lib]\nname = \"l\"\n" + SUB + 'manifest = "z.json"\n[features]\n', "z.json")]
toml_err = [("a repeated subtable", HEAD + SUB + 'manifest = "a.json"\n' + SUB + 'manifest = "b.json"\n', "Repeated owner publication subtable"), ("an empty subtable", HEAD + SUB, "Missing owner publication manifest path"), ("two manifest lines", HEAD + SUB + 'manifest = "a.json"\nmanifest = "b.json"\n', "requires exactly one manifest path"),
            ("another key", HEAD + SUB + 'other = "a.json"\n', "requires exactly one manifest path"), ("an escaped path", HEAD + SUB + 'manifest = "a\\\\b.json"\n', "requires exactly one manifest path"), ("single quotes", HEAD + SUB + "manifest = 'a.json'\n", "requires exactly one manifest path"),
            ("an empty path", HEAD + SUB + 'manifest = ""\n', "requires exactly one manifest path"), ("a non-string value", HEAD + SUB + "manifest = 3\n", "requires exactly one manifest path")]
admit = {"files": {"comp/ok.wasm": "12345", "comp/dir/inner.txt": "x", "afile": "x"}, "cases": [{"target": "comp/ok.wasm", "expected": {"status": "admitted", "limitBytes": 67108864, "byteLength": 5, "path": "comp/ok.wasm"}}, {"target": "comp/missing.wasm", "expected": {"status": "absent", "limitBytes": 67108864}}, {"target": "nodir/missing.wasm", "expected": {"status": "absent", "limitBytes": 67108864}}],
         "errors": [{"target": "../escape.wasm", "message": "Publication component escapes workspace"}, {"target": "comp/dir", "message": "no-follow regular ancestry"}, {"target": "afile/inner", "message": "no-follow regular ancestry"}, {"target": "comp", "message": "no-follow regular ancestry"}], "exceedsBytes": 67108865}
corpus = {"version": 1, "document": doc, "pointerOk": [{"pointer": p, "expected": v} for p, v in ptr_ok], "pointerMissing": ptr_missing, "pointerInvalid": ptr_invalid, "tomlOk": [{"name": n, "text": t, "expected": e} for n, t, e in toml_ok], "tomlErrors": [{"name": n, "text": t, "message": m} for n, t, m in toml_err], "admission": admit}
schema = S.obj({"version": S.const(1), "document": S.anyv, "pointerOk": S.arr(S.obj({"pointer": S.str, "expected": S.anyv}), 1), "pointerMissing": S.strs(1), "pointerInvalid": S.strs(1), "tomlOk": S.arr(S.obj({"name": S.str, "text": S.str, "expected": S.one(S.str, S.null)}), 1), "tomlErrors": S.arr(S.obj({"name": S.str, "text": S.str, "message": S.str}), 1),
 "admission": S.obj({"files": S.map(S.str), "cases": S.arr(S.obj({"target": S.str, "expected": S.anyv}), 1), "errors": S.arr(S.obj({"target": S.str, "message": S.str}), 1), "exceedsBytes": S.int})})
body = '''const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const scratch=(files:Record<string,string>):string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"catpub-")));roots.push(root);for(const[path,text]of Object.entries(files)){const target=join(root,...path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);}return root;};
for(const row of corpus.pointerOk)test("pointer "+JSON.stringify(row.pointer),()=>{expect(publicationPointer(corpus.document,row.pointer)).toEqual(row.expected);});
for(const pointer of corpus.pointerMissing)test("pointer "+JSON.stringify(pointer)+" is missing",()=>{expect(()=>publicationPointer(corpus.document,pointer)).toThrow("Missing publication pointer: "+pointer);});
for(const pointer of corpus.pointerInvalid)test("pointer "+JSON.stringify(pointer)+" is invalid",()=>{expect(()=>publicationPointer(corpus.document,pointer)).toThrow("Invalid publication pointer: "+pointer);});
test("fast-json-patch resolves every valid pointer to the same value and finds none of the missing ones",()=>{
 for(const row of corpus.pointerOk)expect(getValueByPointer(corpus.document,row.pointer),row.pointer).toEqual(row.expected);
 for(const pointer of corpus.pointerMissing.filter(item=>!["/toString","/__proto__","/constructor","/a/x/01"].includes(item))){let found:unknown;try{found=getValueByPointer(corpus.document,pointer);}catch{found=undefined;}expect(found,pointer).toBeUndefined();}
});
for(const row of corpus.tomlOk)test("owner manifest path: "+row.name,()=>{expect(ownerPublicationManifestPath(row.text)).toBe(row.expected??undefined);});
for(const row of corpus.tomlErrors)test("owner manifest path fails on "+row.name,()=>{expect(()=>ownerPublicationManifestPath(row.text)).toThrow(row.message);});
test("@iarna/toml reads the same manifest path from every valid document",()=>{
 for(const row of corpus.tomlOk){const parsed=parse(row.text) as {package?:{metadata?:{semio?:{publications?:{manifest?:string}}}}};expect(parsed.package?.metadata?.semio?.publications?.manifest,row.name).toBe(row.expected??undefined);}
});
for(const row of corpus.admission.cases)test("component admission of "+row.target,()=>{const root=scratch(corpus.admission.files);expect(publicationComponentAdmission(root,row.target)).toEqual(row.expected);});
for(const row of corpus.admission.errors)test("component admission refuses "+row.target,()=>{const root=scratch(corpus.admission.files);expect(()=>publicationComponentAdmission(root,row.target)).toThrow(row.message);});
test("an absolute target outside the workspace and a symlink are refused, an oversized file is flagged",()=>{
 const root=scratch(corpus.admission.files),outside=scratch({"x":"y"});expect(()=>publicationComponentAdmission(root,join(outside,"x"))).toThrow("Publication component escapes workspace");
 if(process.platform==="win32")return;
 symlinkSync(join(root,"comp"),join(root,"link"));expect(()=>publicationComponentAdmission(root,"link/ok.wasm")).toThrow("no-follow regular ancestry");
 const fd=openSync(join(root,"comp","big.wasm"),"w");ftruncateSync(fd,corpus.admission.exceedsBytes);closeSync(fd);
 expect(publicationComponentAdmission(root,"comp/big.wasm")).toEqual({status:"exceeds-limit",limitBytes:67108864,byteLength:corpus.admission.exceedsBytes,path:"comp/big.wasm"});
});'''
ts = lib.ts_wrap(UNIT, "📣️ Contract: catalog publication resolves strict RFC 6901 pointers without prototype aliases, reads the one admitted `manifest` path of the owner publication subtable and records bounded component admission without following symlinks; the corpus is language-agnostic, `fast-json-patch` and `@iarna/toml` are the independent third-party oracles for pointers and the TOML subtable, and `@@O@@` re-derives both.",
 'import {getValueByPointer} from "fast-json-patch";\nimport {parse} from "@iarna/toml";\nimport {afterAll} from "bun:test";\nimport {closeSync,ftruncateSync,mkdirSync,mkdtempSync,openSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {dirname,join} from "node:path";\nimport {ownerPublicationManifestPath,publicationComponentAdmission,publicationPointer} from "@@' + UNIT + '/🟦.ts@@";', body)
oracle = lib.PY_HEAD.format(doc="🐍️ Independent model of RFC 6901 pointers and the owner publication subtable (stdlib only).") + r'''
def resolve(d, ptr):
    if not re.match(r"^(?:/(?:[^~]|~[01])*)+$", ptr):
        raise ValueError("invalid")
    cur = d
    for tok in ptr[1:].split("/"):
        key = tok.replace("~1", "/").replace("~0", "~")
        if isinstance(cur, dict) and key in cur:
            cur = cur[key]
        elif isinstance(cur, list) and re.match(r"^(0|[1-9]\d*)$", key) and int(key) < len(cur):
            cur = cur[int(key)]
        else:
            raise KeyError("missing")
    return cur


for c in corpus["pointerOk"]:
    if resolve(corpus["document"], c["pointer"]) != c["expected"]:
        failures.append("pointer " + c["pointer"])
for p in corpus["pointerMissing"]:
    try:
        resolve(corpus["document"], p)
        failures.append("found " + p)
    except KeyError:
        pass
for p in corpus["pointerInvalid"]:
    try:
        resolve(corpus["document"], p)
        failures.append("valid " + p)
    except ValueError:
        pass


def manifest(text):
    active = seen = False
    result = None
    for src in re.split(r"\r?\n", text):
        line = src.strip()
        if line.startswith("["):
            active = line == "[package.metadata.semio.publications]"
            if active and seen:
                raise ValueError("repeated")
            seen = seen or active
            continue
        if not active or not line or line.startswith("#"):
            continue
        m = re.match(r'^manifest\s*=\s*"([^"\\]+)"\s*(?:#.*)?$', line)
        if not m or result is not None:
            raise ValueError("exactly one")
        result = m.group(1)
    if seen and not result:
        raise ValueError("missing")
    return result


for c in corpus["tomlOk"]:
    if manifest(c["text"]) != c["expected"]:
        failures.append("toml " + c["name"])
for c in corpus["tomlErrors"]:
    try:
        manifest(c["text"])
        failures.append("toml accepted " + c["name"])
    except ValueError:
        pass
''' + lib.PY_TAIL
lib.write_unit(UNIT, corpus, schema, ts, oracle)
