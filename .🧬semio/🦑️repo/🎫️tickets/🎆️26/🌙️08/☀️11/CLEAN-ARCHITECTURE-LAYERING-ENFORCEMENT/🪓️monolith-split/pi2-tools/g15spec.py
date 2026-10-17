#!/usr/bin/env python3
"""🧱️ Spec of group G15: instance-ladders (rank 55)."""
from lib import *
import groupdrv as G


class S(G.Spec):
    name = "g15"
    group = ["instance-ladders"]
    roots_b = {"instance-ladders": PLUG + "/🪜️maintenance-ladder/🦀️.rs"}
    hub_file = PLUG + "/🪝️mounts/🧵️wp7-instance-tier2/🦀️.rs"
    hub_units = {}


if __name__ == "__main__":
    G.main(S)
