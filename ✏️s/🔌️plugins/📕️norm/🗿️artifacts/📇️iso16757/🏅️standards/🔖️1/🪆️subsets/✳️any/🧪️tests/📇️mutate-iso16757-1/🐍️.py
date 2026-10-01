"""🐍️ ISO 16757's contribution to the norm reference implementation — the four things that are
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
    "change-exchange-process",
    "change-script-limits",
    "replace-part-number-rule",
    "change-part-number-input",
    "remove-part-number-input",
    "change-selection-class",
    "change-selection-series",
    "add-selection-constraint",
    "remove-selection-constraint",
    "rename-catalogue",
    "rename-manufacturer",
    "introduce-product-group",
    "retire-product-group",
    "rename-product-group",
    "introduce-product",
    "retire-product",
    "rename-product",
    "introduce-property-definition",
    "retire-property-definition",
    "introduce-subject",
    "retire-subject",
    "introduce-product-class",
    "retire-product-class",
    "introduce-product-series",
    "retire-product-series",
    "introduce-product-index",
    "retire-product-index",
    "introduce-geometry-object",
    "retire-geometry-object",
]

#: 🧫️ The committed specification vectors: each kind's `✅apply` vector as (triad directory, fixture), and each refusal row
#: `<kind>-<slug>` as (kind, triad directory, fixture).
VECTORS = {
    "change-exchange-process": ("🔄️change-exchange-process", "✅apply"),
    "change-script-limits": ("🚦️change-script-limits", "✅apply"),
    "replace-part-number-rule": ("🧮️replace-part-number-rule", "✅apply"),
    "change-part-number-input": ("🎛️change-part-number-input", "✅apply"),
    "remove-part-number-input": ("🔌️remove-part-number-input", "✅apply"),
    "change-selection-class": ("🎯️change-selection-class", "✅apply"),
    "change-selection-series": ("🧵️change-selection-series", "✅apply"),
    "add-selection-constraint": ("🔒️add-selection-constraint", "✅apply"),
    "remove-selection-constraint": ("🔓️remove-selection-constraint", "✅apply"),
    "rename-catalogue": ("📇️rename-catalogue", "✅apply"),
    "rename-manufacturer": ("🏭️rename-manufacturer", "✅apply"),
    "introduce-product-group": ("🧺️introduce-product-group", "✅apply"),
    "retire-product-group": ("🧹️retire-product-group", "✅apply"),
    "rename-product-group": ("🗂️rename-product-group", "✅apply"),
    "introduce-product": ("📦️introduce-product", "✅apply"),
    "retire-product": ("🚫️retire-product", "✅apply"),
    "rename-product": ("🏷️rename-product", "✅apply"),
    "introduce-property-definition": ("📐️introduce-property-definition", "✅apply"),
    "retire-property-definition": ("🧽️retire-property-definition", "✅apply"),
    "introduce-subject": ("🌳️introduce-subject", "✅apply"),
    "retire-subject": ("✂️retire-subject", "✅apply"),
    "introduce-product-class": ("🏷️introduce-product-class", "✅apply"),
    "retire-product-class": ("🗑️retire-product-class", "✅apply"),
    "introduce-product-series": ("📚introduce-product-series", "✅apply"),
    "retire-product-series": ("🗑️retire-product-series", "✅apply"),
    "introduce-product-index": ("🔎introduce-product-index", "✅apply"),
    "retire-product-index": ("🗑️retire-product-index", "✅apply"),
    "introduce-geometry-object": ("📐introduce-geometry-object", "✅apply"),
    "retire-geometry-object": ("🗑️retire-geometry-object", "✅apply"),
}

#: 🗣️ The real committed ISO 16757 document, read where the domain already keeps it.
DSL_ASSET = "asset://🎬️demo/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.iso16757.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("ISO 16757", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
