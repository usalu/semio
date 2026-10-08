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

#: 🧫️ The committed specification vectors: each kind's `✅apply` vector as (triad directory, fixture), and each refusal row
#: `<kind>-<slug>` as (kind, triad directory, fixture).
VECTORS = {
    "change-annex": ("📎️change-annex", "✅apply"),
    "update-site": ("🌚️update-site", "✅apply"),
    "update-site-noop": ("update-site", "🌚️update-site", "🟰noop"),
    "insert-building": ("➕️insert-building", "✅apply"),
    "insert-building-dupe": ("insert-building", "➕️insert-building", "⛔dupe"),
    "remove-building": ("➖️remove-building", "✅apply"),
    "remove-building-middle-row": ("remove-building", "➖️remove-building", "🔬️middle-row"),
    "change-system-v-rd-n": ("💪️change-system-v-rd-n", "✅apply"),
    "change-storey-permanent-gk-n": ("⚖️change-storey-permanent-gk-n", "✅apply"),
    "change-storey-stiffness-x": ("📐️change-storey-stiffness-x", "✅apply"),
    "change-storey-drift-xm": ("📏️change-storey-drift-xm", "✅apply"),
    "change-building-plan-regular": ("🧭️change-building-plan-regular", "✅apply"),
    "change-elevation-regular": ("📏️change-elevation-regular", "✅apply"),
    "change-member-detailing": ("✅️change-member-detailing", "✅apply"),
    "change-masonry-wall-ratio": ("🧱️change-masonry-wall-ratio", "✅apply"),
    "insert-bridge": ("🌉insert-bridge", "✅apply"),
    "insert-bridge-dupe": ("insert-bridge", "🌉insert-bridge", "⛔dupe"),
    "change-bridge-v-rd-n": ("🛑️change-bridge-v-rd-n", "✅apply"),
    "insert-assessment": ("🔧insert-assessment", "✅apply"),
    "insert-assessment-dupe": ("insert-assessment", "🔧insert-assessment", "⛔dupe"),
    "change-assessment-rkn": ("🏋️change-assessment-rkn", "✅apply"),
    "insert-silo": ("🫙insert-silo", "✅apply"),
    "insert-silo-dupe": ("insert-silo", "🫙insert-silo", "⛔dupe"),
    "insert-tank": ("🛢️insert-tank", "✅apply"),
    "insert-tank-dupe": ("insert-tank", "🛢️insert-tank", "⛔dupe"),
    "insert-foundation": ("🪨insert-foundation", "✅apply"),
    "insert-foundation-dupe": ("insert-foundation", "🪨insert-foundation", "⛔dupe"),
    "insert-retaining-wall": ("🧱️insert-retaining-wall", "✅apply"),
    "insert-retaining-wall-dupe": ("insert-retaining-wall", "🧱️insert-retaining-wall", "⛔dupe"),
    "insert-tower": ("🗼insert-tower", "✅apply"),
    "insert-tower-dupe": ("insert-tower", "🗼insert-tower", "⛔dupe"),
    "change-tower-m-rd-nm": ("↪️change-tower-m-rd-nm", "✅apply"),
    "remove-bridge": ("➖️remove-bridge", "✅apply"),
    "remove-bridge-middle-row": ("remove-bridge", "➖️remove-bridge", "🔬️middle-row"),
    "remove-assessment": ("➖️remove-assessment", "✅apply"),
    "remove-assessment-middle-row": ("remove-assessment", "➖️remove-assessment", "🔬️middle-row"),
    "remove-silo": ("➖️remove-silo", "✅apply"),
    "remove-silo-middle-row": ("remove-silo", "➖️remove-silo", "🔬️middle-row"),
    "remove-tank": ("➖️remove-tank", "✅apply"),
    "remove-tank-middle-row": ("remove-tank", "➖️remove-tank", "🔬️middle-row"),
    "remove-foundation": ("➖️remove-foundation", "✅apply"),
    "remove-foundation-middle-row": ("remove-foundation", "➖️remove-foundation", "🔬️middle-row"),
    "remove-retaining-wall": ("➖️remove-retaining-wall", "✅apply"),
    "remove-retaining-wall-middle-row": ("remove-retaining-wall", "➖️remove-retaining-wall", "🔬️middle-row"),
    "remove-tower": ("➖️remove-tower", "✅apply"),
    "remove-tower-middle-row": ("remove-tower", "➖️remove-tower", "🔬️middle-row"),
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
