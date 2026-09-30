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

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-annex": ("🌍️change-annex", "✏️to-en"),
    "change-geotechnical-category": ("🗂️change-geotechnical-category", "✏️to-3"),
    "change-design-situation": ("📅️change-design-situation", "✏️to-bs-t"),
    "change-design-approach": ("🧭️change-design-approach", "✏️to-da3"),
    "change-groundwater-level": ("💧change-groundwater-level", "✏️to-2-5"),
    "change-investigation-depth": ("🔎️change-investigation-depth", "✏️to-25"),
    "change-footing-width": ("↔️change-footing-width", "✏️to-3"),
    "change-footing-embedment": ("⬇️change-footing-embedment", "✏️to-1-8"),
    "change-pile-length": ("📏️change-pile-length", "✏️to-16"),
    "change-pile-count": ("🔢change-pile-count", "✏️to-3"),
    "change-wall-base-width": ("🧱change-wall-base-width", "✏️to-2-9"),
    "change-slope-angle": ("⛰️change-slope-angle", "✏️to-30"),
    "change-layer-phi-prime": ("📐️change-layer-phi-prime", "✏️to-32-5"),
    "change-layer-oedometric-modulus": ("🌀️change-layer-oedometric-modulus", "✏️new"),
    "insert-layer": ("➕️insert-layer", "➕️inserts-layer"),
    "remove-layer": ("➖️remove-layer", "➖️removes-layer"),
    "insert-footing": ("➕insert-footing", "➕️inserts-footing"),
    "remove-footing": ("➖remove-footing", "➖️removes-footing"),
    "insert-pile": ("📥insert-pile", "➕️inserts-pile"),
    "remove-pile": ("📤remove-pile", "➖️removes-pile"),
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
