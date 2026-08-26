import sys, types, hashlib
def install():
    stub = types.ModuleType("semio_repo_test")
    class Adapter:
        def __init__(self, impl): self.impl = impl; self.oracles = {}
        def oracle(self, sid, fn): self.oracles[sid] = fn; return self
        def subject(self, sid, fn): return self
    class Context: pass
    class Outcome:
        def __init__(self, projection, raw=None, diagnostics=None): self.projection = projection; self.raw = raw
    stub.Adapter=Adapter; stub.Context=Context; stub.Outcome=Outcome
    stub.digest=lambda b: hashlib.sha256(b).hexdigest()[:32]
    sys.modules["semio_repo_test"]=stub
    return stub
