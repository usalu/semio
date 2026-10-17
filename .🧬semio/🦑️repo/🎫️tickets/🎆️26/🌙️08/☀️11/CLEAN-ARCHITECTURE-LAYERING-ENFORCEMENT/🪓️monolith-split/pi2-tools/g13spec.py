#!/usr/bin/env python3
"""🧱️ Spec of group G13: instance-extension, instance-render, instance-dispatch (ranks 48 to 50)."""
from lib import *
import groupdrv as G

I = PLUG + "/🧵️instance"


class S(G.Spec):
    name = "g13"
    group = ["instance-extension", "instance-render", "instance-dispatch"]
    roots_b = {"instance-extension": I + "/🧩️extensions/🦀️.rs", "instance-render": I + "/🖼️render/🦀️.rs"}
    hub_file = PLUG + "/🪝️mounts/🧱️wp6-instance-tier1/🦀️.rs"
    hub_units = {}


if __name__ == "__main__":
    G.main(S)
