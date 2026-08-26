import importlib.util, json, os, sys, glob
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import importlib
stubmod = importlib.import_module("🐍️stub") if False else None
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)),"🐍️stub.py")).read())
install()
ROOT="/Users/ueli/Documents/semio"
CASE=os.path.join(ROOT,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-flow")
EX=os.path.join(ROOT,"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🌊️pipeline/🖼️assets")
spec=importlib.util.spec_from_file_location("flowpy", os.path.join(CASE,"🐍️component.py"))
mod=importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)
dsl=open(os.path.join(EX,"🗣️example.dsl.semio"),"rb").read()
pack=open(os.path.join(EX,"🎒️example.pack.semio"),"rb").read()
doc=mod.parse_dsl(dsl.decode())
print("dsl byte-exact:", mod.print_dsl(doc).encode()==dsl)
print("pack decodes equal:", mod.parse_pack(pack)==doc)
print("pack byte-exact:", mod.pack_bytes(doc)==pack)
ok=True
for f in sorted(glob.glob(os.path.join(CASE,"🧫️fixtures","*.json"))):
    v=json.load(open(f))
    got=mod.apply_mutation(v["before"], v["mutation"])
    a = got==v["after"]
    r=got
    for s in mod.inverse_mutation(v["before"], v["mutation"]): r=mod.apply_mutation(r,s)
    b = r==v["before"]
    print("  %-20s apply=%s inverse=%s" % (v["kind"],a,b))
    ok = ok and a and b
print("ALL VECTORS:", ok)
print("before==parsed artifact:", json.load(open(os.path.join(CASE,"🧫️fixtures","🦠️no-mutation.json")))["before"]==doc)
