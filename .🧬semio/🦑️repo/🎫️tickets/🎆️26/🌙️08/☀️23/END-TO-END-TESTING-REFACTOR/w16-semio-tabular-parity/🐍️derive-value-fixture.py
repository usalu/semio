"""🌲️ Derive the semio VALUE fixture from the real committed building model JSON, ONCE.

Source: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🧫️fixtures/🔣️hexagonal-cut-concrete-forest-left.model.json
(424 KB of real `spatial.modelspace` geometry), read with Python's own `json` module using
`parse_int`/`parse_float` hooks so every numeric SOURCE LEXEME survives verbatim.
"""
import importlib.util, os, shutil, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "🐍️stub.py")).read())
install()

ROOT = "/Users/ueli/Documents/semio"
CASE = os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-value")
SRC = os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🧫️fixtures/🔣️hexagonal-cut-concrete-forest-left.model.json")
spec = importlib.util.spec_from_file_location("valuepy", os.path.join(CASE, "🐍️component.py"))
mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)

fx = os.path.join(CASE, "🧫️fixtures")
raw = open(SRC, "rb").read()
shutil.copyfile(SRC, os.path.join(fx, "🌲️hexagonal-cut-concrete-forest-left.model.json"))
doc = mod.derive_document_from_json(raw)
t = time.time(); dsl = mod.print_dsl(doc).encode("utf-8"); print("print %.2fs" % (time.time()-t))
pack = mod.pack_bytes(doc)
t = time.time(); back = mod.parse_dsl(dsl.decode("utf-8")); print("parse %.2fs" % (time.time()-t))
assert back == doc, "the derived DSL does not round-trip"
assert mod.parse_pack(pack) == doc
open(os.path.join(fx, "🌲️hexagonal-cut-concrete-forest.dsl.semio"), "wb").write(dsl)
open(os.path.join(fx, "🌲️hexagonal-cut-concrete-forest.pack.semio"), "wb").write(pack)
print("source bytes", len(raw))
print("root entries", len(doc["root"]["entries"]), "nodes", len(doc["nodes"]))
print("dsl", len(dsl), "pack", len(pack))
print("node ids", [n["id"]["value"] for n in doc["nodes"]])
print("root keys", [e["key"] for e in doc["root"]["entries"]])
