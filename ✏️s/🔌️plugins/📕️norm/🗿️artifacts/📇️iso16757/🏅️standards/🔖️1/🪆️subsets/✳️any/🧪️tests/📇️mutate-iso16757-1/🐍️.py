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

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-exchange-process": ("🔄️change-exchange-process", "✏️sets"),
    "change-script-limits": ("🚦️change-script-limits", "✏️to-20000"),
    "replace-part-number-rule": ("🧮️replace-part-number-rule", "✏️sets"),
    "change-part-number-input": ("🎛️change-part-number-input", "✏️sets"),
    "remove-part-number-input": ("🔌️remove-part-number-input", "➖️removes"),
    "change-selection-class": ("🎯️change-selection-class", "✏️to-class"),
    "change-selection-series": ("🧵️change-selection-series", "✏️to-series"),
    "add-selection-constraint": ("🔒️add-selection-constraint", "➕️adds"),
    "remove-selection-constraint": ("🔓️remove-selection-constraint", "✏️sets"),
    "rename-catalogue": ("📇️rename-catalogue", "✏️to-fixture"),
    "rename-manufacturer": ("🏭️rename-manufacturer", "🏭️adds-the-ag"),
    "introduce-product-group": ("🧺️introduce-product-group", "✏️sets"),
    "retire-product-group": ("🧹️retire-product-group", "➖️retires"),
    "rename-product-group": ("🗂️rename-product-group", "✏️to-panel"),
    "introduce-product": ("📦️introduce-product", "➕️introduces"),
    "retire-product": ("🚫️retire-product", "🚫️removes-the-pr600"),
    "rename-product": ("🏷️rename-product", "✏️renames-pr600"),
    "introduce-property-definition": ("📐️introduce-property-definition", "✏️new"),
    "retire-property-definition": ("🧽️retire-property-definition", "✏️sets"),
    "introduce-subject": ("🌳️introduce-subject", "🌳️appends-towel"),
    "retire-subject": ("✂️retire-subject", "➖️retires-subject"),
    "introduce-product-class": ("🏷️introduce-product-class", "✏️sets"),
    "retire-product-class": ("🗑️retire-product-class", "➖️retires"),
    "introduce-product-series": ("📚introduce-product-series", "📚appends-a-pr"),
    "retire-product-series": ("🗑️retire-product-series", "➖️retires"),
    "introduce-product-index": ("🔎introduce-product-index", "➕️introduces"),
    "retire-product-index": ("🗑️retire-product-index", "➖️retires"),
    "introduce-geometry-object": ("📐introduce-geometry-object", "➕️introduces"),
    "retire-geometry-object": ("🗑️retire-geometry-object", "➖️retires"),
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
