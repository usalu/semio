"""🐍️ EN 1992's contribution to the norm reference implementation — the four things that are
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
    "change-title",
    "change-design-working-life",
    "change-delta-c-dev",
    "change-cement-type",
    "change-concrete-f-ck",
    "change-reinforcement-f-yk",
    "insert-member",
    "remove-member",
    "reorder-members",
    "change-member-width",
    "change-member-height",
    "change-member-effective-depth",
    "change-member-cover",
    "change-member-exposure",
    "change-member-span",
    "change-member-stirrup-spacing",
    "change-member-axis-distance",
    "change-member-fire-rating",
    "change-bar-layer-count",
    "change-bar-layer-diameter",
    "change-action-mk",
    "change-action-nk",
    "change-action-vk",
    "insert-anchor",
    "remove-anchor",
    "change-anchor-h-ef",
    "change-anchor-as",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-annex": ("🌍️change-annex", "✏️to-en"),
    "change-title": ("🏷️change-title", "✏️to-liquid"),
    "change-design-working-life": ("📅️change-design-working-life", "✏️to-100"),
    "change-delta-c-dev": ("📏️change-delta-c-dev", "✏️to-0-015"),
    "change-cement-type": ("🧪change-cement-type", "✏️to-s"),
    "change-concrete-f-ck": ("🧱change-concrete-f-ck", "✏️to-45000000"),
    "change-reinforcement-f-yk": ("🔩change-reinforcement-f-yk", "✏️to-550000000"),
    "insert-member": ("➕️insert-member", "➕️inserts-member"),
    "remove-member": ("➖️remove-member", "➖️removes-member"),
    "reorder-members": ("🔀️reorder-members", "🔀️reorders-members"),
    "change-member-width": ("↔️change-member-width", "✏️to-0-42"),
    "change-member-height": ("↕️change-member-height", "✏️to-0-6"),
    "change-member-effective-depth": ("📐️change-member-effective-depth", "✏️to-0"),
    "change-member-cover": ("🛡️change-member-cover", "✏️to-0-05"),
    "change-member-exposure": ("🌦change-member-exposure", "✏️to-xd1"),
    "change-member-span": ("🌉️change-member-span", "✏️to-8-25"),
    "change-member-stirrup-spacing": ("🪢change-member-stirrup-spacing", "✏️to-0-2"),
    "change-member-axis-distance": ("🔥change-member-axis-distance", "✏️to-0-045"),
    "change-member-fire-rating": ("🔥️change-member-fire-rating", "✏️to-r120"),
    "change-bar-layer-count": ("#️⃣change-bar-layer-count", "✏️to-8"),
    "change-bar-layer-diameter": ("⭕change-bar-layer-diameter", "✏️to-0-02"),
    "change-action-mk": ("⤴️change-action-mk", "✏️to-125000"),
    "change-action-nk": ("🏋️change-action-nk", "✏️to-37500"),
    "change-action-vk": ("↘️change-action-vk", "✏️to-62500"),
    "insert-anchor": ("⚓️insert-anchor", "➕️inserts-anchor"),
    "remove-anchor": ("🗑️remove-anchor", "➖️removes-anchor"),
    "change-anchor-h-ef": ("📍change-anchor-h-ef", "✏️to-0-125"),
    "change-anchor-as": ("🧷change-anchor-a-s", "✏️to-0-00015"),
}

#: 🗣️ The real committed EN 1992 document, read where the domain already keeps it.
DSL_ASSET = "asset://🛢️liquid-retaining-fem-anchor/🛢️liquid-retaining-fem-anchor/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.en1992.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("EN 1992", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
