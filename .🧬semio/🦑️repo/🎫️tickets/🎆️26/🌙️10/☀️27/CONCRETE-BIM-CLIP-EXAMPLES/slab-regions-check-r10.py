"""🧪 Run source-independent and measured-source regional slab checks through any Python test host."""
import importlib

suite = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪨️regions")
suite.model._source.cache_clear()
for name in sorted(vars(suite)):
    if name.startswith("test_"):
        getattr(suite, name)()
        print("PASS", name)
