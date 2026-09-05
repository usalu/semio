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

_vocabulary = import_module("🗣️vocabulary")
Subset = _vocabulary.Subset
build_adapter = _vocabulary.build_adapter

# endregion 🔖️Imports


# region 🔖️Vocabulary
#: 🏷️ Every kind this subset's committed catalog declares, in catalog order.
KINDS = [
    "change-n-ed-kn",
    "change-m-ed-knm",
    "change-a-mm2",
    "change-w-el-mm3",
    "change-alloy",
    "change-chi",
    "change-it-mm4",
    "change-l-cr-mm",
    "change-theta-c",
    "change-delta-sigma-ed",
    "change-delta-sigma-c",
    "change-fatigue-m",
    "change-n-cycles",
    "change-v-weld-ed-kn",
    "change-weld-throat-mm",
    "change-weld-length-mm",
    "change-beta-w",
    "change-sheet-b-mm",
    "change-sheet-t-mm",
    "change-sheet-k-sigma",
    "change-sheet-w-el-mm3",
    "change-sheet-m-ed-knm",
    "change-shell-t-mm",
    "change-shell-r-mm",
    "change-sigma-ed-shell-mpa",
    "change-annex",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-n-ed-kn": ("🏋️change-n-ed-kn", "🏋️raises-axial-force-to-180-kn"),
    "change-m-ed-knm": ("⤴️change-m-ed-knm", "⤴️raises-design-moment-to-9-5-knm"),
    "change-a-mm2": ("📐️change-a-mm2", "📐️enlarges-section-area-to-2250-mm2"),
    "change-w-el-mm3": ("🧊️change-w-el-mm3", "🧊️raises-section-modulus-to-40000-mm3"),
    "change-alloy": ("⚗️change-alloy", "⚗️switches-alloy-to-aw7020t6"),
    "change-chi": ("⬇️change-chi", "⬇️lowers-buckling-chi-to-0-5"),
    "change-it-mm4": ("🌀️change-it-mm4", "🌀️raises-torsion-constant-to-10240-mm4"),
    "change-l-cr-mm": ("📏️change-l-cr-mm", "📏️lengthens-buckling-length-to-4000-mm"),
    "change-theta-c": ("🌡️change-theta-c", "🌡️raises-temperature-to-225-c"),
    "change-delta-sigma-ed": ("↕️change-delta-sigma-ed", "↕️raises-fatigue-stress-range-to-62-5-mpa"),
    "change-delta-sigma-c": ("🏷️change-delta-sigma-c", "🏷️upgrades-detail-category-to-90-mpa"),
    "change-fatigue-m": ("📉️change-fatigue-m", "📉️flattens-sn-slope-to-m-5"),
    "change-n-cycles": ("🔁️change-n-cycles", "🔁️doubles-fatigue-cycles-to-2000000"),
    "change-v-weld-ed-kn": ("✂️change-v-weld-ed-kn", "✂️raises-weld-shear-to-48-kn"),
    "change-weld-throat-mm": ("🔥️change-weld-throat-mm", "🔥️thickens-weld-throat-to-6-5-mm"),
    "change-weld-length-mm": ("🧵️change-weld-length-mm", "🧵️lengthens-weld-to-200-mm"),
    "change-beta-w": ("🧮️change-beta-w", "🧮️raises-weld-correlation-beta-w-to-0-75"),
    "change-sheet-b-mm": ("↔️change-sheet-b-mm", "↔️widens-sheet-to-320-mm"),
    "change-sheet-t-mm": ("📑️change-sheet-t-mm", "📑️thickens-sheet-to-3-5-mm"),
    "change-sheet-k-sigma": ("🎚️change-sheet-k-sigma", "🎚️raises-sheet-plate-buckling-k-sigma-to-6-25"),
    "change-sheet-w-el-mm3": ("📊️change-sheet-w-el-mm3", "📊️raises-sheet-section-modulus-to-12800-mm3"),
    "change-sheet-m-ed-knm": ("🌊️change-sheet-m-ed-knm", "🌊️raises-sheet-design-moment-to-1-25-knm"),
    "change-shell-t-mm": ("🐚️change-shell-t-mm", "🐚️thickens-shell-to-6-25-mm"),
    "change-shell-r-mm": ("⭕️change-shell-r-mm", "🐚️widens-shell-radius-to-750-mm"),
    "change-sigma-ed-shell-mpa": ("🗜️change-sigma-ed-shell-mpa", "🐚️raises-shell-design-stress-to-165-mpa"),
    "change-annex": ("🌍️change-annex", "🌍️switches-national-annex-to-en"),
}

#: 🗣️ The real committed EN 1999 document, read where the domain already keeps it.
DSL_ASSET = "asset://📚️examples/🏠️aluminium-roof-purlin/🖼️assets/🏠️aluminium-roof-purlin/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.en1999.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("EN 1999", KINDS, VECTORS, DSL_ASSET, ENVELOPE, vector_root="asset://🧬️schema/🧬️mutations"))
# endregion 🔖️Registration
