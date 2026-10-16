"""Files reachable from the top crate's library root through #[path] mounts and include!, with the `cfg(test)`-only ones marked."""
import re, sys
from lib import *
import modtree
LIB = T + "/📦️packages/🦀️rust/📚️library/🦀️.rs"
def reach():
    return modtree.tree(LIB)
if __name__ == "__main__":
    fs = reach()
    tests = [f for f in fs if "/🧪️tests/" in f or "test-hosts" in f or "worker-cell" in f or "outcome" in f]
    print(len(fs), "files reachable;", len(tests), "test-related")
    for f in tests:
        print("  ", os.path.relpath(f, E)[-90:])
