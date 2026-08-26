"""🐍️ Ticket scratch: exercises the independent semio-text implementation outside the test host."""
import importlib.util, json, sys, types, hashlib, os

ROOT = "/Users/ueli/Documents/semio"
STUB = types.ModuleType("semio_repo_test")
class Outcome:
    def __init__(self, projection, raw=None, diagnostics=None):
        self.projection, self.raw = projection, raw
class Adapter:
    def __init__(self, impl): self.impl, self.h = impl, {}
    def oracle(self, s, h): self.h[s] = h; return self
STUB.Outcome, STUB.Adapter, STUB.Context = Outcome, Adapter, object
STUB.digest = lambda p: hashlib.sha256(p or b"").hexdigest()[:32]
sys.modules["semio_repo_test"] = STUB

path = os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-text/🐍️component.py")
spec = importlib.util.spec_from_file_location("impl", path)
mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)

base = os.path.join(ROOT, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio")
dsl = open(os.path.join(base, "🏅️standards/🔖️v1/🪆️subsets/✳️text/📚️examples/📃️note/🖼️assets/🗣️example.dsl.semio"), "rb").read()
pack = open(os.path.join(base, "🏅️standards/🔖️v1/🪆️subsets/✳️text/📚️examples/📃️note/🖼️assets/🎒️example.pack.semio"), "rb").read()

doc = mod.parse_dsl(dsl.decode())
print("parsed:", json.dumps(doc, ensure_ascii=False))
printed = mod.print_dsl(doc).encode()
print("dsl byte-exact:", printed == dsl, len(printed), len(dsl))
unpacked = mod.parse_pack(pack)
print("pack == dsl doc:", unpacked == doc)
repacked = mod.pack_bytes(doc)
print("pack byte-exact:", repacked == pack, len(repacked), len(pack))

# committed specification vectors
VEC = {
 "insert-run": ("📥insert-run", "inserts-a-german-run-between-two-english-runs"),
 "remove-run": ("🗑️remove-run", "removes-the-middle-run"),
 "edit-run": ("✏️edit-run", "rewrites-the-marked-runs-content"),
 "change-run-language": ("🌐change-run-language", "retags-the-second-run-as-german"),
 "reorder-runs": ("🔀reorder-runs", "moves-the-first-run-to-the-end"),
 "add-mark": ("➕add-mark", "adds-a-link-mark-ahead-of-the-bold-mark"),
 "remove-mark": ("➖remove-mark", "detaches-the-italic-mark-from-the-run"),
}
root = os.path.join(base, "🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations")
for kind, (d, f) in VEC.items():
    p = os.path.join(root, d, "🧪️tests", f)
    before = json.load(open(os.path.join(p, "📸️snapshot/⬅️before/🔣️component.json")))
    after = json.load(open(os.path.join(p, "📸️snapshot/➡️after/🔣️component.json")))
    mut = json.load(open(os.path.join(p, "🦠️mutation/🔣️component.json")))
    got = mod.apply_mutation(before, mut)
    inv = mod.apply_mutation(got, mod.inverse_mutation(before, mut))
    print("%-20s vector=%-5s inverse=%s" % (kind, got == after, inv == before))
