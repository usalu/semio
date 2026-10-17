#!/usr/bin/env python3
"""🧱️ Spec of group G16: instance-app (rank 56), the composer `VcsArtifactApp` with `PluginApp` and the bundle builders."""
from lib import *
import groupdrv as G


class S(G.Spec):
    name = "g16"
    group = ["instance-app"]
    roots_b = {"instance-app": PLUG + "/🧵️instance/🦀️.rs"}
    hub_file = PLUG + "/🪝️mounts/🧵️wp7-instance-tier2/🦀️.rs"
    hub_units = {}


if __name__ == "__main__":
    G.main(S)
