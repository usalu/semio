#!/usr/bin/env python3
"""🧱️ Spec of group G2: document-media."""
from lib import *
import groupdrv as G


class S(G.Spec):
    name = "g2"
    group = ["document-media"]
    roots_b = {"document-media": PLUG + "/🗃️document/🦀️.rs"}
    extra_mounts = [("lifted_app", PLUG + "/🗃️document/🛬️lifted/🧩️app/🦀️.rs")]
    root_extra = {"document-media": "pub use lifted_app::*;"}
    hub_file = PLUG + "/🪝️mounts/👥️wp3-owners/🦀️.rs"
    hub_units = {}


if __name__ == "__main__":
    G.main(S)
