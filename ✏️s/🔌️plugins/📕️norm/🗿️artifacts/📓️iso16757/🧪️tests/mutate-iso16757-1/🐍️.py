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
from semio_norm_vocabulary import Subset, build_adapter

# endregion 🔖️Imports


# region 🔖️Vocabulary
#: 🏷️ Every kind this subset's committed catalog declares, in catalog order.
KINDS = [
    "change-exchange-process",
    "update-script-limits",
    "replace-part-number-rule",
    "change-part-number-input",
    "remove-part-number-input",
    "change-selection-class",
    "change-selection-series",
    "add-selection-constraint",
    "remove-selection-constraint",
    "rename-catalogue",
    "rename-manufacturer",
    "create-product-group",
    "delete-product-group",
    "rename-product-group",
    "create-product",
    "delete-product",
    "rename-product",
    "create-property-definition",
    "delete-property-definition",
    "create-subject",
    "delete-subject",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-exchange-process": ("🍃change-exchange-process", "advances-the-exchange-stage-to-determine-product"),
    "update-script-limits": ("🌷update-script-limits", "doubles-the-step-budget-and-quintuples-the-timeout"),
    "replace-part-number-rule": ("🍂replace-part-number-rule", "swaps-the-literal-rule-for-a-height-driven-script"),
    "change-part-number-input": ("🌱change-part-number-input", "raises-the-height-part-number-input-to-750"),
    "remove-part-number-input": ("🌿remove-part-number-input", "drops-the-length-part-number-input"),
    "change-selection-class": ("🌴change-selection-class", "retargets-the-selection-at-the-towel-radiator-class"),
    "change-selection-series": ("🌼change-selection-series", "narrows-the-selection-to-the-pr-plus-series"),
    "add-selection-constraint": ("🛁add-selection-constraint", "appends-a-width-under-800-constraint"),
    "remove-selection-constraint": ("🛋️remove-selection-constraint", "drops-the-trailing-length-constraint"),
    "rename-catalogue": ("🌲rename-catalogue", "restamps-the-catalogue-as-the-2026-edition"),
    "rename-manufacturer": ("🌳rename-manufacturer", "adds-the-ag-suffix-to-the-manufacturer"),
    "create-product-group": ("🍀create-product-group", "appends-a-towel-radiators-group"),
    "delete-product-group": ("🌹delete-product-group", "removes-the-radiators-group-and-strands-its-class"),
    "rename-product-group": ("🚿rename-product-group", "renames-the-radiators-group-to-panel-radiators"),
    "create-product": ("🍁create-product", "appends-a-pr900-product-to-the-existing-series"),
    "delete-product": ("🌸delete-product", "removes-the-pr600-product-from-the-catalogue"),
    "rename-product": ("🛏️rename-product", "renames-pr600-to-the-compact-variant-name"),
    "create-property-definition": ("🌾create-property-definition", "appends-a-selection-scoped-length-property"),
    "delete-property-definition": ("🌺delete-property-definition", "removes-the-height-property-definition"),
    "create-subject": ("🌵create-subject", "appends-a-towel-radiator-subject-under-the-radiator-parent"),
    "delete-subject": ("🌻delete-subject", "removes-the-radiator-subject-from-the-dictionary"),
}

#: 🗣️ The real committed ISO 16757 document, read where the domain already keeps it.
DSL_ASSET = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio"

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
