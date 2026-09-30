"""🐍️ EN 1998's contribution to the norm reference implementation — the four things that are
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
    "update-site",
    "insert-building",
    "remove-building",
    "change-system-v-rd-n",
    "change-storey-permanent-gk-n",
    "change-storey-stiffness-x",
    "change-storey-drift-xm",
    "change-building-plan-regular",
    "change-elevation-regular",
    "change-member-detailing",
    "change-masonry-wall-ratio",
    "insert-bridge",
    "change-bridge-v-rd-n",
    "insert-assessment",
    "change-assessment-rkn",
    "insert-silo",
    "insert-tank",
    "insert-foundation",
    "insert-retaining-wall",
    "insert-tower",
    "change-tower-m-rd-nm",
    "remove-bridge",
    "remove-assessment",
    "remove-silo",
    "remove-tank",
    "remove-foundation",
    "remove-retaining-wall",
    "remove-tower",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-annex": ("📎️change-annex", "🌍️switches-to-en"),
    "update-site": ("🌚️update-site", "🌋️zone-3-soft-soil"),
    "insert-building": ("➕️insert-building", "🏢️adds-an-annex"),
    "remove-building": ("➖️remove-building", "🏚️drops-the-office"),
    "change-system-v-rd-n": ("💪️change-system-v-rd-n", "💪️stronger-y"),
    "change-storey-permanent-gk-n": ("⚖️change-storey-permanent-gk-n", "⚖️heavier"),
    "change-storey-stiffness-x": ("📐️change-storey-stiffness-x", "📐️softer"),
    "change-storey-drift-xm": ("📏️change-storey-drift-xm", "📏️more-drift"),
    "change-building-plan-regular": ("🧭️change-building-plan-regular", "🧭️twist"),
    "change-elevation-regular": ("📏️change-elevation-regular", "🏙️irregular"),
    "change-member-detailing": ("✅️change-member-detailing", "✅️beam-unfit"),
    "change-masonry-wall-ratio": ("🧱️change-masonry-wall-ratio", "🧱️four-pct"),
    "insert-bridge": ("🌉insert-bridge", "🌉️adds-a-viaduct"),
    "change-bridge-v-rd-n": ("🛑️change-bridge-v-rd-n", "🛑️stronger-pier"),
    "insert-assessment": ("🔧insert-assessment", "🔧️adds-a-kl3-check"),
    "change-assessment-rkn": ("🏋️change-assessment-rkn", "🏋️stronger"),
    "insert-silo": ("🫙insert-silo", "🫙️adds-a-grain-silo"),
    "insert-tank": ("🛢insert-tank", "🛢️adds-a-water-tank"),
    "insert-foundation": ("🪨insert-foundation", "🪨️adds-a-pad"),
    "insert-retaining-wall": ("🧱️insert-retaining-wall", "🧱️adds-a-wall"),
    "insert-tower": ("🗼insert-tower", "🏭️adds-a-chimney"),
    "change-tower-m-rd-nm": ("↪️change-tower-m-rd-nm", "↪️stronger-base"),
    "remove-bridge": ("➖️remove-bridge", "🌉️drops-the-viaduct"),
    "remove-assessment": ("➖️remove-assessment", "🔧️drops-the-check"),
    "remove-silo": ("➖️remove-silo", "🫙️drops-the-silo"),
    "remove-tank": ("➖️remove-tank", "🛢️drops-the-tank"),
    "remove-foundation": ("➖️remove-foundation", "🪨️drops-the-pad"),
    "remove-retaining-wall": ("➖️remove-retaining-wall", "🧱️drops-it"),
    "remove-tower": ("➖️remove-tower", "🗼️drops-the-tower"),
}

#: 🗣️ The real committed EN 1998 document, read where the domain already keeps it.
DSL_ASSET = "asset://🏢️seismic-rc-frame/🏢️seismic-rc-frame/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.en1998.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("EN 1998", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
