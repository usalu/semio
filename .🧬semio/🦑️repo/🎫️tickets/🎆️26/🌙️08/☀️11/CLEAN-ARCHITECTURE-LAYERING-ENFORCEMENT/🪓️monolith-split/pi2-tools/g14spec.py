#!/usr/bin/env python3
"""🧱️ Spec of group G14: instance-typed-ops, time-travel-turns, tool-run-turns, instance-commands (ranks 51 to 54)."""
from lib import *
import groupdrv as G

I = PLUG + "/🧵️instance"


class S(G.Spec):
    name = "g14"
    group = ["instance-typed-ops", "time-travel-turns", "tool-run-turns", "instance-commands"]
    roots_b = {"instance-typed-ops": I + "/🧾️typed-operations/🦀️.rs", "time-travel-turns": PLUG + "/⏪️time-travel/🧵️instance/🦀️.rs", "tool-run-turns": PLUG + "/⏯️tool-run/🧵️instance/🦀️.rs", "instance-commands": I + "/🧷️command-routes/🦀️.rs"}
    hub_file = PLUG + "/🪝️mounts/🧵️wp7-instance-tier2/🦀️.rs"
    hub_units = {}


if __name__ == "__main__":
    G.main(S)
