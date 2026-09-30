"""🐍️ EN 1999's contribution to the norm reference implementation — the four things that are
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
    "change-materials",
    "change-sections",
    "change-members",
    "change-connections",
    "change-fire-scenarios",
    "change-fatigue-details",
    "change-cold-formed",
    "change-shells",
    "add-member",
    "remove-member",
    "change-member-n-ed",
    "change-member-my-ed",
    "change-member-buckling-length",
    "change-material-designation",
    "change-plate-thickness",
    "change-weld-throat",
    "change-bolt-count",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-annex": ("🌍️change-annex", "✏️to-en"),
    "change-materials": ("🧱change-materials", "✏️sets-materials"),
    "change-sections": ("📐️change-sections", "✏️sets-sections"),
    "change-members": ("🏗️change-members", "✏️sets-members"),
    "change-connections": ("🔗change-connections", "✏️larger-weld-throat"),
    "change-fire-scenarios": ("🔥️change-fire-scenarios", "🔥️hotter-fire"),
    "change-fatigue-details": ("🔄️change-fatigue-details", "✏️more-stress"),
    "change-cold-formed": ("❄️change-cold-formed", "✏️thinner-sheet"),
    "change-shells": ("🫙change-shells", "✏️thicker-shell"),
    "add-member": ("➕add-member", "➕️adds-member"),
    "remove-member": ("➖remove-member", "➖️removes-member"),
    "change-member-n-ed": ("🏋️change-member-n-ed", "✏️to-2500"),
    "change-member-my-ed": ("⤴️change-member-my-ed", "✏️to-5000"),
    "change-member-buckling-length": ("📏️change-member-buckling-length", "✏️to-0"),
    "change-material-designation": ("⚗️change-material-designation", "✏️to-en-aw"),
    "change-plate-thickness": ("🧱change-plate-thickness", "✏️to-0-0125"),
    "change-weld-throat": ("🔥️change-weld-throat", "✏️to-0-006"),
    "change-bolt-count": ("🔩change-bolt-count", "✏️to-3"),
}

#: 🗣️ The real committed EN 1999 document, read where the domain already keeps it.
DSL_ASSET = "asset://🏠️aluminium-roof-purlin/🏠️aluminium-roof-purlin/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.en1999.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("EN 1999", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
