#!/usr/bin/env python3
# Scaffold helper for lawcov-cli: resolves emoji-normalized unit paths and writes a unit's own tests (corpus, schema, ts test, python oracle).
import os, json, re, subprocess

ROOT = "/Users/ueli/Documents/semio"
VS = "️"


def N(s):
    return s.replace(VS, "")


def resolve(norm_rel, base=ROOT):
    cur = base
    for part in [p for p in N(norm_rel).split("/") if p]:
        direct = os.path.join(cur, part)
        found = None
        if os.path.exists(direct):
            found = direct
        else:
            for e in os.listdir(cur):
                if N(e) == part:
                    found = os.path.join(cur, e)
                    break
        if found is None:
            raise FileNotFoundError(os.path.join(cur, part))
        cur = found
    return cur


LIBN = "🧰framework/🛍products/🦑repo/🔨modules/📚library"
LIB = resolve(LIBN)
FILES = {"C": "🔣" + VS + ".json", "T": "🟦" + VS + ".ts", "O": "🐍" + VS + "oracle.py", "SD": "🧬" + VS + "schema", "TD": "🧪" + VS + "tests", "CI": "./🔣" + VS + ".json", "SI": "./🧬" + VS + "schema/🔣" + VS + ".json", "TS": "🟦" + VS + ".ts"}


class S:
    str = {"type": "string"}
    int = {"type": "integer"}
    num = {"type": "number"}
    bool = {"type": "boolean"}
    anyv = {}
    null = {"type": "null"}

    @staticmethod
    def obj(props, required=None, extra=False):
        req = list(props) if required is None else required
        d = {"type": "object", "additionalProperties": extra if extra else False, "required": req, "properties": props}
        return d

    @staticmethod
    def arr(item, minimum=0):
        d = {"type": "array", "items": item}
        if minimum:
            d["minItems"] = minimum
        return d

    @staticmethod
    def enum(*v):
        return {"enum": list(v)}

    @staticmethod
    def const(v):
        return {"const": v}

    @staticmethod
    def one(*s):
        return {"anyOf": list(s)}

    @staticmethod
    def map(item):
        return {"type": "object", "additionalProperties": item}

    @staticmethod
    def strs(minimum=0):
        return S.arr(S.str, minimum)

    @staticmethod
    def opt(s):
        return s


def dump(obj):
    return json.dumps(obj, ensure_ascii=False, indent=1) + "\n"


def write_unit(unit_norm, corpus, schema, ts, oracle=None, shim=None):
    udir = resolve(os.path.join(LIBN, unit_norm)) if unit_norm != "." else LIB
    tdir = os.path.join(udir, FILES["TD"])
    os.makedirs(os.path.join(tdir, FILES["SD"]), exist_ok=True)

    def sub(text):
        def repl(m):
            key = m.group(1)
            if key in FILES:
                return FILES[key] if key not in ("TD", "SD") else FILES[key]
            target = resolve(os.path.join(LIBN, key))
            rel = os.path.relpath(target, tdir)
            return rel if rel.startswith(".") else "./" + rel
        return re.sub(r"@@([^@]+)@@", repl, text)
    cname = FILES["C"]
    corp = corpus if isinstance(corpus, str) else dump(corpus)
    open(os.path.join(tdir, cname), "w", encoding="utf8").write(corp)
    open(os.path.join(tdir, FILES["SD"], cname), "w", encoding="utf8").write(dump(schema))
    open(os.path.join(tdir, FILES["T"]), "w", encoding="utf8").write(sub(ts))
    if oracle:
        open(os.path.join(tdir, FILES["O"]), "w", encoding="utf8").write(sub(oracle))
    return tdir


def toolchain_import(unit_norm):
    depth = len([p for p in N(unit_norm).split("/") if p and p != "."]) + 1
    return "../" * (depth + 4) + "🔨" + VS + "modules/🏃" + VS + "process/🧪" + VS + "testing/🧰" + VS + "toolchain/🟦" + VS + ".ts"


def run_test(tdir):
    return subprocess.run(["nice", "-n", "10", "bun", "test", os.path.join(tdir, FILES["T"])], capture_output=True, text=True, cwd=ROOT)


def ts_wrap(unit_norm, doc, imports, body, python=True, extra_head=""):
    head = ['/** ' + doc + ' */', 'import {expect,setDefaultTimeout,test} from "bun:test";', 'import Ajv from "ajv";']
    if python:
        head += ['import {dirname,join} from "node:path";', 'import {fileURLToPath} from "node:url";', 'import {testPythonV1} from "' + toolchain_import(unit_norm) + '";']
    head += [imports, 'import corpus from "@@CI@@";', 'import schema from "@@SI@@";', "", "setDefaultTimeout(120000);"]
    tail = ['test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});']
    if python:
        extra_head = extra_head + '\nconst here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();'
        tail.append('test("the independent Python model reaches the same verdicts",()=>{\n if(!python)return;\n const result=Bun.spawnSync([python,"-I",join(here,"@@O@@"),join(here,"@@C@@")]);\n expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);\n});')
    return "\n".join(head) + extra_head + "\n" + body + "\n" + "\n".join(tail) + "\n"


PY_HEAD = '#!/usr/bin/env python3\n"""{doc}\n\nusage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""\nimport json, re, sys\n\ncorpus = json.load(open(sys.argv[1], encoding="utf8"))\nfailures = []\nVS = "\\ufe0f"\n\n\ndef norm(s):\n    return s.replace("\\ufe0f", "").replace("\\ufe0e", "")\n\n\n'
PY_TAIL = '\nprint(json.dumps({"failures": failures}))\nsys.exit(1 if failures else 0)\n'
