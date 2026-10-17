#!/usr/bin/env python3
"""🧱️ Spec of group G12: the five tier-1 instance crates (WP6, ranks 43 to 47). Their staging files leave the instance parent, which re-points its `mod` lines to the crates."""
from lib import *
import groupdrv as G


class S(G.Spec):
    name = "g12"
    group = ["instance-stores", "instance-history", "instance-children", "instance-jobs", "instance-docs"]
    roots_b = {}
    hub_file = PLUG + "/🪝️mounts/🧱️wp6-instance-tier1/🦀️.rs"
    hub_units = {}


if __name__ == "__main__":
    G.main(S)
