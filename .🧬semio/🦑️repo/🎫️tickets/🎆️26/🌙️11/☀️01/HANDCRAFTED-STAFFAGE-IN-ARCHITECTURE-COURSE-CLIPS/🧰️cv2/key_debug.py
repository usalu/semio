import importlib
import sys

sys.path.insert(0, str(__import__("pathlib").Path(__file__).parent))
A = importlib.import_module("key_audit")
S = A.S
a, b = A.G / "dbg-a", A.G / "dbg-b"
A.build(a)
A.build(b)
A.edit(b, A.ENV, '"airTempC": 22', '"airTempC": 24')
entry = A.CASES["line"]
for name, root in (("a", a), ("b", b)):
    print(name, S.cast_key(entry, root), S._source_digest(root)[:10], len(S._catalog_bytes("pendant-lamp", root)), len(S._environment_bytes(root)))
