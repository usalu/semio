#!/usr/bin/env python3
"""🧱️ Spec of group G18: the shared test app fixtures leave the facade hub 7 for the test crate testing-fixtures (rank 10.5). The facade keeps its `cfg(test)` module names as re-exports until the tests move."""
from lib import *
import groupdrv as G
import re


def unconditional(text):
    return re.sub(r"(?m)^#\[cfg\(test\)\]\n(?=#\[path)", "", text)


class S(G.Spec):
    name = "g18"
    group = ["testing-fixtures"]
    roots_b = {}
    hub_file = PLUG + "/🪝️mounts/🧵️wp7-instance-tier2/🦀️.rs"
    hub_units = {}
    extra_edits = {PLUG + "/🧪️testing/🧰️fixtures/🦀️.rs": unconditional}


if __name__ == "__main__":
    G.main(S)
