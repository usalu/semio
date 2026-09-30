"""🐍️ EN 1994's contribution to the norm reference implementation — the four things that are
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
    "change-structure-kind",
    "change-steel-fy-pa",
    "change-fire-rating",
    "change-insulation-thickness-m",
    "change-fatigue-detail",
    "insert-beam",
    "remove-beam",
    "change-beam-action-q-area-pa",
    "change-beam-stud-spacing-m",
    "change-beam-span-m",
    "change-beam-slab-thickness-m",
    "change-beam-stud-diameter-m",
    "change-beam-stud-count",
    "change-beam-stud-fu-pa",
    "change-beam-transverse-as",
    "change-beam-construction",
    "insert-column",
    "remove-column",
    "change-column-action-force-n",
    "change-column-kind",
    "insert-slab",
    "remove-slab",
    "change-slab-action-q-area-pa",
    "change-slab-thickness-m",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-annex": ("🌍️change-annex", "🌐️switches-national"),
    "change-structure-kind": ("🏗️change-structure-kind", "✏️to-bridge"),
    "change-steel-fy-pa": ("🏋️change-steel-fy-pa", "🏋️upgrades-fy"),
    "change-fire-rating": ("🔥️change-fire-rating", "🔥️upgrades-to-r90"),
    "change-insulation-thickness-m": ("🧯️change-insulation-thickness-m", "✏️to-0"),
    "change-fatigue-detail": ("🔁️change-fatigue-detail", "✏️to-flange"),
    "insert-beam": ("➕️insert-beam", "➕️inserts-beam"),
    "remove-beam": ("➖️remove-beam", "➖️removes-beam"),
    "change-beam-action-q-area-pa": ("🌀️change-beam-action-q-area-pa", "✏️sets"),
    "change-beam-stud-spacing-m": ("✂️change-beam-stud-spacing-m", "✏️to-0-15"),
    "change-beam-span-m": ("📏️change-beam-span-m", "📏️sets-span-10m"),
    "change-beam-slab-thickness-m": ("🧱change-beam-slab-thickness-m", "✏️to-0-16"),
    "change-beam-stud-diameter-m": ("⭕️change-beam-stud-diameter-m", "✏️to-0-022"),
    "change-beam-stud-count": ("#️⃣change-beam-stud-count", "#️⃣sets-count"),
    "change-beam-stud-fu-pa": ("💪️change-beam-stud-fu-pa", "💪️sets-fu"),
    "change-beam-transverse-as": ("↔️change-beam-transverse-as", "↔️sets-as"),
    "change-beam-construction": ("🛠️change-beam-construction", "✏️sets"),
    "insert-column": ("➗️insert-column", "➗️inserts-column"),
    "remove-column": ("⛔️remove-column", "⛔️removes-column"),
    "change-column-action-force-n": ("⬇️change-column-action-force-n", "⬇️sets-n"),
    "change-column-kind": ("↪️change-column-kind", "↪️sets-kind"),
    "insert-slab": ("➕insert-slab", "➕inserts-slab"),
    "remove-slab": ("➖remove-slab", "➖removes-slab"),
    "change-slab-action-q-area-pa": ("📐️change-slab-action-q-area-pa", "✏️sets"),
    "change-slab-thickness-m": ("📏change-slab-thickness-m", "📏sets-thickness"),
}

#: 🗣️ The real committed EN 1994 document, read where the domain already keeps it.
DSL_ASSET = "asset://🌉️composite-bridge-girder/🌉️composite-bridge-girder/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.en1994.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("EN 1994", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
