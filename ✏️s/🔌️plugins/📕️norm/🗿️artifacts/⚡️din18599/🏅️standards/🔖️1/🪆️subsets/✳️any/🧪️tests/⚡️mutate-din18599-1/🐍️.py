"""🐍️ DIN V 18599's contribution to the norm reference implementation — the four things that are
genuinely per-standard, and nothing else.

The second producer this case's differential comparison needs is
`semio_norm_vocabulary`, the ONE independent Python implementation of the norm mutation vocabulary,
imported here rather than copied. Its module docstring carries the survey that established no
third-party library reads or writes `s.norm.*`, the two committed documents it was written from, and
the honest boundary on the `.dsl.semio` carrier. This file adds no verb, no addressing rule and no
carrier rule: everything below is DATA read off this subset's own committed catalog, its own
committed specification vectors and its own committed example document.

Stating it this way is the point. The fifteen norm adapters used to hold fifteen byte-identical
copies of that engine, which made the reference surface read as fifteen independent implementations
when it was one. One import says what fifteen copies concealed — a shared bug here agrees with itself
in all fifteen cases, and that is now visible instead of pretended.
"""

from __future__ import annotations

# region 🔖️Imports
from importlib import import_module

_vocabulary = import_module("🐍️")
Subset = _vocabulary.Subset
build_adapter = _vocabulary.build_adapter

# endregion 🔖️Imports


# region 🔖️Vocabulary
#: 🏷️ Every kind this subset's committed catalog declares, in catalog order.
KINDS = [
    "change-use-class",
    "change-net-floor-area-m2",
    "update-climate",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-use-class": ("🏷️use-class", "🏢️reclassifies-the-building-as-an-office"),
    "change-net-floor-area-m2": ("📐️net-floor-area-m2", "📏️extends-net-floor-area-to-160-m2"),
    "update-climate": ("🌦️update-climate", "🌧️refuses-a-negative-january-irradiance"),
}

#: 🗣️ The real committed DIN V 18599 document, read where the domain already keeps it.
DSL_ASSET = "asset://🎬️demo/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.din18599.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("DIN V 18599", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
