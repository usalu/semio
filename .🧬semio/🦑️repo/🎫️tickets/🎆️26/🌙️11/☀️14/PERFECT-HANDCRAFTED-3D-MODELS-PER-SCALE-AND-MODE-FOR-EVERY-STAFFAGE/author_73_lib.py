"""Author 73 helpers (wraps author_63_lib for the 2D root repairs and checks). Run from T."""
import sys, os, json, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import author_63_lib as L
from author_63_lib import patch_section, asset_path, asset, load, save, run, T
GEN = os.path.join(T, "🗑️generated", "author-73"); os.makedirs(GEN, exist_ok=True)
def backup(slug, tag):
    import shutil; shutil.copy(asset_path(slug), os.path.join(GEN, f"{slug}.{tag}.json"))
def patch_rep(slug, scale, fn):
    """fn(geometry dict of one scale) edits in place; rewrites only the representations section of the catalog asset."""
    def g(rep):
        fn(rep[str(scale)]["geometry"], rep[str(scale)]); return rep
    return patch_section(slug, "representations", g)
def check(slug, scale=None, full=False): return L.check(slug, scale, full)
