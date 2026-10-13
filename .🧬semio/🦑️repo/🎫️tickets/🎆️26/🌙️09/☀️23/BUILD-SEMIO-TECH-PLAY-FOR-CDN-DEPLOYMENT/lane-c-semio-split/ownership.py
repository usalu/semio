"""🗺️ Assigns every semio source file to the crate that will compile it after the cut, from the wave-1 module tree."""
import os
from semio_split import *
from mount_graph import walk

CRATE = {"base": "semio-s-artifact-stdio-semio-base", "agg": "semio-s-artifact-stdio-semio", "catalog": "semio-s-artifact-stdio-semio-catalog"}
for _s in NAMES:
    CRATE[_s] = "semio-s-artifact-stdio-semio-" + _s
for _s in CONVERSION_SUBSETS:
    CRATE[_s + "-import"] = "semio-s-artifact-stdio-semio-%s-import" % _s
    CRATE[_s + "-export"] = "semio-s-artifact-stdio-semio-%s-export" % _s


def rust_name(key):
    return CRATE[key].replace("-", "_")


def crate_roots():
    roots = {"base": SUBSETS + "/✉️base/🦀️.rs", "agg": ROOT_RS, "catalog": SEMIO + "/🧩️composition/🦀️.rs"}
    for s in NAMES:
        roots[s] = SUBSETS + "/" + EMOJI[s] + "/🦀️.rs"
    for s in CONVERSION_SUBSETS:
        roots[s + "-import"] = SUBSETS + "/" + EMOJI[s] + "/🚪️io/📥️import/🦀️.rs"
        roots[s + "-export"] = SUBSETS + "/" + EMOJI[s] + "/🚪️io/📤️export/🦀️.rs"
    return roots


def owners(sqlite_tests_to_agg=True):
    """🗺️ {file: crate key}; conversion trees win over their subset, the subset over the aggregator."""
    roots = crate_roots()
    own = {}
    order = [k for k in roots if k.endswith(("-import", "-export"))] + ["base"] + NAMES
    for key in order:
        for f in walk(roots[key]):
            own.setdefault(f, key)
    for f in walk(ROOT_RS):
        own.setdefault(f, "agg")
    if sqlite_tests_to_agg:
        for s in NAMES:
            t = SUBSETS + "/" + EMOJI[s] + "/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"
            if os.path.exists(t):
                own[t] = "agg"
    return own
