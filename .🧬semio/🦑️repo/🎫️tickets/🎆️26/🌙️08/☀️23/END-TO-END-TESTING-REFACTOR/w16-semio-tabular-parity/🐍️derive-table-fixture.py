import importlib.util, json, os, shutil, sys, types, hashlib
ROOT = "/Users/ueli/Documents/semio"
CASE = os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-table")
SRC  = os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🧫️fixtures/📊️reuse-marketplaces.csv")
stub = types.ModuleType("semio_repo_test")
class A:
    def __init__(s,i): pass
    def oracle(s,a,b): return s
    def subject(s,a,b): return s
stub.Adapter=A; stub.Context=object; stub.Outcome=object; stub.digest=lambda b: hashlib.sha256(b).hexdigest()[:32]
sys.modules["semio_repo_test"]=stub
spec = importlib.util.spec_from_file_location("tablepy", os.path.join(CASE, "🐍️component.py"))
mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)

fx = os.path.join(CASE, "🧫️fixtures")
os.makedirs(fx, exist_ok=True)
raw = open(SRC,"rb").read()
shutil.copyfile(SRC, os.path.join(fx, "📊️reuse-marketplaces.csv"))
doc = mod.derive_document_from_csv(raw)
dsl = mod.print_dsl(doc).encode("utf-8")
pack = mod.pack_bytes(doc)
open(os.path.join(fx, "📊️reuse-marketplaces.dsl.semio"),"wb").write(dsl)
open(os.path.join(fx, "📊️reuse-marketplaces.pack.semio"),"wb").write(pack)
assert mod.parse_dsl(dsl.decode()) == doc
assert mod.parse_pack(pack) == doc
print("csv bytes", len(raw))
print("columns", len(doc["columns"]), "rows", len(doc["rows"]), "cells", sum(len(r["cells"]) for r in doc["rows"]))
print("dsl bytes", len(dsl), "pack bytes", len(pack))
print("dsl sha256-32", stub.digest(dsl))
print("pack sha256-32", stub.digest(pack))
print("colnames", [c["name"] for c in doc["columns"]])
