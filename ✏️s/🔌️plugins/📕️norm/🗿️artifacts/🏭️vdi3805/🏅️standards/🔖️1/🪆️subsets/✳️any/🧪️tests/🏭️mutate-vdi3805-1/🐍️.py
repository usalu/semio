"""🐍️ VDI 3805's contribution to the norm reference implementation — the four things that are
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
    "change-manufacturer-file",
    "change-limits",
    "change-correction-as-of",
    "change-strict-mode",
    "change-edition-profile",
    "remove-edition-profile",
    "add-product",
    "remove-product",
    "rename-product",
    "change-product-configuration",
    "add-geometry",
    "remove-geometry",
    "resize-geometry",
    "add-geometry-connection",
    "remove-geometry-connection",
    "change-geometry-parameters",
    "add-curve",
    "remove-curve",
    "change-curve-points",
]

#: 🧫️ The committed specification vectors: each kind's `✅apply` vector as (triad directory, fixture), and each refusal row
#: `<kind>-<slug>` as (kind, triad directory, fixture).
VECTORS = {
    "change-manufacturer-file": ("🏭️change-manufacturer-file", "✅apply"),
    "change-limits": ("🚧️change-limits", "✅apply"),
    "change-correction-as-of": ("📅️change-correction-as-of", "✅apply"),
    "change-strict-mode": ("🔒️change-strict-mode", "✅apply"),
    "change-edition-profile": ("🔖️change-edition-profile", "✅apply"),
    "remove-edition-profile": ("🧹️remove-edition-profile", "✅apply"),
    "add-product": ("📦️add-product", "✅apply"),
    "remove-product": ("🗑️remove-product", "✅apply"),
    "rename-product": ("🏷️rename-product", "✅apply"),
    "change-product-configuration": ("🎛️change-product-configuration", "✅apply"),
    "add-geometry": ("🧊️add-geometry", "✅apply"),
    "remove-geometry": ("🚮️remove-geometry", "✅apply"),
    "resize-geometry": ("📐️resize-geometry", "✅apply"),
    "add-geometry-connection": ("🔌️add-geometry-connection", "✅apply"),
    "remove-geometry-connection": ("✂️remove-geometry-connection", "✅apply"),
    "change-geometry-parameters": ("🧮️change-geometry-parameters", "✅apply"),
    "add-curve": ("📈️add-curve", "✅apply"),
    "remove-curve": ("📉️remove-curve", "✅apply"),
    "change-curve-points": ("📍️change-curve-points", "✅apply"),
}

#: 🗣️ The real committed VDI 3805 document, read where the domain already keeps it.
DSL_ASSET = "asset://🎬️demo/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.vdi3805.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("VDI 3805", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
