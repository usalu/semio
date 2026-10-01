"""🐍️ EN 1997's contribution to the norm reference implementation — the four things that are
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
    "change-annex",
    "change-geotechnical-category",
    "change-design-situation",
    "change-design-approach",
    "change-groundwater-level",
    "change-investigation-depth",
    "change-footing-width",
    "change-footing-embedment",
    "change-pile-length",
    "change-pile-count",
    "change-wall-base-width",
    "change-slope-angle",
    "change-layer-phi-prime",
    "change-layer-oedometric-modulus",
    "insert-layer",
    "remove-layer",
    "insert-footing",
    "remove-footing",
    "insert-pile",
    "remove-pile",
]

#: 🧫️ The committed specification vectors: each kind's `✅apply` vector as (triad directory, fixture), and each refusal row
#: `<kind>-<slug>` as (kind, triad directory, fixture).
VECTORS = {
    "change-annex": ("🌍️change-annex", "✅apply"),
    "change-geotechnical-category": ("🗂️change-geotechnical-category", "✅apply"),
    "change-design-situation": ("📅️change-design-situation", "✅apply"),
    "change-design-approach": ("🧭️change-design-approach", "✅apply"),
    "change-groundwater-level": ("💧change-groundwater-level", "✅apply"),
    "change-investigation-depth": ("🔎️change-investigation-depth", "✅apply"),
    "change-footing-width": ("↔️change-footing-width", "✅apply"),
    "change-footing-embedment": ("⬇️change-footing-embedment", "✅apply"),
    "change-pile-length": ("📏️change-pile-length", "✅apply"),
    "change-pile-count": ("🔢change-pile-count", "✅apply"),
    "change-wall-base-width": ("🧱change-wall-base-width", "✅apply"),
    "change-slope-angle": ("⛰️change-slope-angle", "✅apply"),
    "change-layer-phi-prime": ("📐️change-layer-phi-prime", "✅apply"),
    "change-layer-oedometric-modulus": ("🌀️change-layer-oedometric-modulus", "✅apply"),
    "insert-layer": ("➕️insert-layer", "✅apply"),
    "insert-layer-dupe": ("insert-layer", "➕️insert-layer", "⛔dupe"),
    "remove-layer": ("➖️remove-layer", "✅apply"),
    "insert-footing": ("➕insert-footing", "✅apply"),
    "insert-footing-dupe": ("insert-footing", "➕insert-footing", "⛔dupe"),
    "remove-footing": ("➖remove-footing", "✅apply"),
    "insert-pile": ("📥insert-pile", "✅apply"),
    "insert-pile-dupe": ("insert-pile", "📥insert-pile", "⛔dupe"),
    "remove-pile": ("📤remove-pile", "✅apply"),
}

#: 🗣️ The real committed EN 1997 document, read where the domain already keeps it.
DSL_ASSET = "asset://🎬️demo/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.en1997.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("EN 1997", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
