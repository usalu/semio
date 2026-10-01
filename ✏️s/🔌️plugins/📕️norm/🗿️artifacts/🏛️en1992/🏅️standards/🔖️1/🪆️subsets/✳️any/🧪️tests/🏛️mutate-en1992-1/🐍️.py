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

#: 🧫️ The committed specification vectors: each kind's `✅apply` vector as (triad directory, fixture), and each refusal row
#: `<kind>-<slug>` as (kind, triad directory, fixture).
VECTORS = {
    "change-annex": ("🌍️change-annex", "✅apply"),
    "change-title": ("🏷️change-title", "✅apply"),
    "change-design-working-life": ("📅️change-design-working-life", "✅apply"),
    "change-delta-c-dev": ("📏️change-delta-c-dev", "✅apply"),
    "change-cement-type": ("🧪change-cement-type", "✅apply"),
    "change-concrete-f-ck": ("🧱change-concrete-f-ck", "✅apply"),
    "change-reinforcement-f-yk": ("🔩change-reinforcement-f-yk", "✅apply"),
    "insert-member": ("➕️insert-member", "✅apply"),
    "remove-member": ("➖️remove-member", "✅apply"),
    "reorder-members": ("🔀️reorder-members", "✅apply"),
    "change-member-width": ("↔️change-member-width", "✅apply"),
    "change-member-height": ("↕️change-member-height", "✅apply"),
    "change-member-effective-depth": ("📐️change-member-effective-depth", "✅apply"),
    "change-member-cover": ("🛡️change-member-cover", "✅apply"),
    "change-member-exposure": ("🌦️change-member-exposure", "✅apply"),
    "change-member-span": ("🌉️change-member-span", "✅apply"),
    "change-member-stirrup-spacing": ("🪢change-member-stirrup-spacing", "✅apply"),
    "change-member-axis-distance": ("🔥change-member-axis-distance", "✅apply"),
    "change-member-fire-rating": ("🔥️change-member-fire-rating", "✅apply"),
    "change-bar-layer-count": ("#️⃣change-bar-layer-count", "✅apply"),
    "change-bar-layer-diameter": ("⭕change-bar-layer-diameter", "✅apply"),
    "change-action-mk": ("⤴️change-action-mk", "✅apply"),
    "change-action-nk": ("🏋️change-action-nk", "✅apply"),
    "change-action-vk": ("↘️change-action-vk", "✅apply"),
    "insert-anchor": ("⚓️insert-anchor", "✅apply"),
    "insert-anchor-dupe": ("insert-anchor", "⚓️insert-anchor", "⛔dupe"),
    "remove-anchor": ("🗑️remove-anchor", "✅apply"),
    "remove-anchor-gone": ("remove-anchor", "🗑️remove-anchor", "❓gone"),
    "change-anchor-h-ef": ("📍change-anchor-h-ef", "✅apply"),
    "change-anchor-as": ("🧷change-anchor-a-s", "✅apply"),
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
