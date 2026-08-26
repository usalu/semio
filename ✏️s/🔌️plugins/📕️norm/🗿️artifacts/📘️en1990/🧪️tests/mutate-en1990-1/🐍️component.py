"""🐍️ EN 1990's contribution to the norm reference implementation — the four things that are
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
    "change-annex",
    "change-permanent-action",
    "change-resistance",
    "change-consequence-class",
    "change-seismic-action",
    "insert-variable-action",
    "remove-variable-action",
    "change-variable-action-category",
    "change-variable-action-value",
    "reorder-variable-actions",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-annex": ("🐷set-snapshot", "switches-the-national-annex-from-de-to-en"),
    "change-permanent-action": ("🐐change-permanent-action", "raises-the-permanent-action-to-62-5-kn"),
    "change-resistance": ("🐘change-resistance", "raises-the-design-resistance-to-320-kn"),
    "change-consequence-class": ("🐑change-consequence-class", "escalates-the-building-from-cc2-to-cc3"),
    "change-seismic-action": ("🦄change-seismic-action", "enables-the-seismic-situation-with-an-85-kn-a-ed"),
    "insert-variable-action": ("🐴insert-variable-action", "seeds-the-first-variable-action-q-snow-at-12-5-kn"),
    "remove-variable-action": ("🐎remove-variable-action", "refuses-to-remove-action-0-from-an-unseeded-child-slot"),
    "change-variable-action-category": ("🐮change-variable-action-category", "refuses-to-recategorise-a-missing-action-0"),
    "change-variable-action-value": ("🦌change-variable-action-value", "refuses-to-revalue-a-missing-action-0"),
    "reorder-variable-actions": ("🐗reorder-variable-actions", "refuses-to-move-action-0-to-slot-1-in-an-empty-list"),
}

#: 🗣️ The real committed EN 1990 document, read where the domain already keeps it.
DSL_ASSET = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️high-consequence-office/🖼️assets/🗣️high-consequence-office.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.en1990.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("EN 1990", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
