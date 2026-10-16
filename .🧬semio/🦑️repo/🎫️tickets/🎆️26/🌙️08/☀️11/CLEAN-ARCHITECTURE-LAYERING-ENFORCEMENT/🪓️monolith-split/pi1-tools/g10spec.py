#!/usr/bin/env python3
"""🧱️ Spec of group G10: time-travel-panel (WP5). tool-run and time-travel keep their pwp7 `instance_turns` mounts and are not flipped here."""
from lib import *
import groupdrv as G


class S(G.Spec):
    name = "g10"
    group = ["time-travel-panel"]
    roots_b = {}
    hub_file = PLUG + "/🪝️mounts/⏯️wp5-features/🦀️.rs"
    hub_units = {}


if __name__ == "__main__":
    G.main(S)
