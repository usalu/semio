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

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-manufacturer-file": ("🏭️change-manufacturer-file", "✏️renames-the-header-manufacturer-to-acme"),
    "change-correction-as-of": ("📅️change-correction-as-of", "📅️advances-the-correction-cut-off-to-2025-03"),
    "change-strict-mode": ("🔒️change-strict-mode", "🔒️turns-strict-mode-on"),
    "change-edition-profile": ("🔖️change-edition-profile", "🆕️switches-sheet-8-from-legacy-to-current"),
    "remove-edition-profile": ("🧹️remove-edition-profile", "🧹️clears-the-sheet-8-legacy-override"),
    "add-product": ("📦️add-product", "📦️appends-vlv-80-002-and-its-index-entry"),
    "remove-product": ("🗑️remove-product", "🚫️removes-vlv-50-001-and-its-index-entry"),
    "rename-product": ("🏷️rename-product", "🏷️retitles-vlv-50-001-and-resyncs-its-index-tags"),
    "change-product-configuration": ("🎛️change-product-configuration", "📏️reparameterises-vlv-50-001-to-dn-80-and-resyncs-index-dn"),
    "add-geometry": ("🧊️add-geometry", "🧊️adds-the-geom-valve-80-definition"),
    "remove-geometry": ("🚮️remove-geometry", "🚫️removes-the-geom-valve-50-definition"),
    "resize-geometry": ("📐️resize-geometry", "📐️doubles-the-geom-valve-50-bounding-box"),
    "add-geometry-connection": ("🔌️add-geometry-connection", "🚰️attaches-the-drain-connection-to-geom-valve-50"),
    "remove-geometry-connection": ("✂️remove-geometry-connection", "🔌️detaches-the-out-connection-from-geom-valve-50"),
    "change-geometry-parameters": ("🧮️change-geometry-parameters", "➗️rescales-geom-valve-50-to-half-and-adds-clearance"),
    "add-curve": ("📈️add-curve", "📈️adds-the-curve-dp-pressure-drop-curve"),
    "remove-curve": ("📉️remove-curve", "🚫️removes-the-curve-kvs-flow-curve"),
    "change-curve-points": ("📍️change-curve-points", "📍️resamples-curve-kvs-onto-three-points"),
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
    return build_adapter(Subset("VDI 3805", KINDS, VECTORS, DSL_ASSET, ENVELOPE, vector_root="shared://🧬️mutations"))
# endregion 🔖️Registration
