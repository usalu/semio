import importlib.util, json, os, sys, types

ROOT = "/Users/ueli/Documents/semio"
CASE = os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-table")
SUBSET = os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table")

stub = types.ModuleType("semio_repo_test")
class Adapter:
    def __init__(self, impl): self.impl = impl; self.oracles = {}
    def oracle(self, sid, fn): self.oracles[sid] = fn; return self
    def subject(self, sid, fn): return self
class Context: pass
class Outcome:
    def __init__(self, projection, raw=None, diagnostics=None): self.projection = projection; self.raw = raw
import hashlib
def digest(b): return hashlib.sha256(b).hexdigest()[:32]
stub.Adapter = Adapter; stub.Context = Context; stub.Outcome = Outcome; stub.digest = digest
sys.modules["semio_repo_test"] = stub

spec = importlib.util.spec_from_file_location("tablepy", os.path.join(CASE, "🐍️component.py"))
mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)

dsl = open(os.path.join(SUBSET, "📚️examples/📃️sheet/🖼️assets/🗣️example.dsl.semio"), "rb").read()
pack = open(os.path.join(SUBSET, "📚️examples/📃️sheet/🖼️assets/🎒️example.pack.semio"), "rb").read()
doc = mod.parse_dsl(dsl.decode())
print("parsed:", json.dumps(doc, ensure_ascii=False))
printed = mod.print_dsl(doc).encode()
print("dsl byte-exact:", printed == dsl, len(printed), len(dsl))
up = mod.parse_pack(pack)
print("pack==dsl doc:", up == doc)
rp = mod.pack_bytes(doc)
print("pack byte-exact:", rp == pack, len(rp), len(pack))

# spec vectors
dirs = {
 "create-column": ("🏗️create-column", "appends-a-float-column-and-null-pads-every-row"),
 "delete-column": ("🗑️delete-column", "drops-the-middle-column-and-cascades-into-every-row"),
 "rename-column": ("🏷️rename-column", "renames-city-to-town-without-touching-any-row"),
 "reorder-columns": ("🔀reorder-columns", "moves-the-area-column-to-the-front-and-realigns-every-row"),
 "insert-row": ("📥insert-row", "inserts-a-row-between-the-two-existing-rows"),
 "remove-row": ("➖remove-row", "removes-the-leading-row"),
 "reorder-rows": ("🔃reorder-rows", "moves-the-last-row-to-the-front"),
 "edit-cell": ("✏️edit-cell", "rewrites-the-population-cell-of-the-second-row"),
}
ok = True
for kind,(d,slug) in dirs.items():
    base = os.path.join(SUBSET, "🧬️schema/🧬️mutations", d, "🧪️tests", slug)
    before = json.load(open(os.path.join(base, "📸️snapshot/⬅️before/🔣️component.json")))
    mut = json.load(open(os.path.join(base, "🦠️mutation/🔣️component.json")))
    after = json.load(open(os.path.join(base, "📸️snapshot/➡️after/🔣️component.json")))
    got = mod.apply_mutation(before, mut)
    good = got == after
    inv = got
    for s in mod.inverse_mutation(before, mut):
        inv = mod.apply_mutation(inv, s)
    back = inv == before
    print(f"  {kind}: apply={good} inverse={back}")
    ok = ok and good and back
print("ALL SPEC VECTORS:", ok)
