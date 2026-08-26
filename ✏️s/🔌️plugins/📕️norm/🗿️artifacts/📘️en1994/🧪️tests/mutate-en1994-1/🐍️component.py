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
from semio_norm_vocabulary import Subset, build_adapter

# endregion 🔖️Imports


# region 🔖️Vocabulary
#: 🏷️ Every kind this subset's committed catalog declares, in catalog order.
KINDS = [
    "change-annex",
    "change-m-ed-knm",
    "change-v-ed-kn",
    "change-m-pla",
    "change-m-pl-rd",
    "change-eta",
    "change-vl-rd",
    "change-insulation-thickness-mm",
    "change-fire-rating",
    "change-deck-type",
    "change-delta-sigma-mpa",
    "change-fatigue-detail",
    "change-d-mm",
    "change-h-sc-mm",
    "change-f-ck-mpa",
    "change-fu-mpa",
    "change-e-cm-mpa",
    "change-v-ed-per-stud-kn",
    "change-span-m",
    "change-fy-mpa",
    "change-n-cycles-stud",
    "change-delta-tau-stud-mpa",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-annex": ("🏞️change-annex", "switches-national-annex-to-en"),
    "change-m-ed-knm": ("🗻change-m-ed-knm", "raises-design-moment-to-320-knm"),
    "change-v-ed-kn": ("🍄change-v-ed-kn", "raises-design-shear-to-225-kn"),
    "change-m-pla": ("🏝️change-m-pla", "raises-steel-plastic-moment-to-128-knm"),
    "change-m-pl-rd": ("🐚change-m-pl-rd", "raises-plastic-moment-resistance-to-375-knm"),
    "change-eta": ("🏕️change-eta", "raises-shear-connection-degree-to-0-875"),
    "change-vl-rd": ("🐞change-vl-rd", "raises-longitudinal-shear-resistance-to-240-kn"),
    "change-insulation-thickness-mm": ("🏖️change-insulation-thickness-mm", "thickens-fire-insulation-to-40-mm"),
    "change-fire-rating": ("🏟️change-fire-rating", "upgrades-fire-rating-to-r90"),
    "change-deck-type": ("🐝change-deck-type", "switches-deck-to-re-entrant"),
    "change-delta-sigma-mpa": ("🌏️change-delta-sigma-mpa", "raises-steel-stress-range-to-96-mpa"),
    "change-fatigue-detail": ("⛰️change-fatigue-detail", "switches-fatigue-detail-to-flange-butt-weld"),
    "change-d-mm": ("🌰change-d-mm", "thickens-stud-shank-to-22-mm"),
    "change-h-sc-mm": ("🌐change-h-sc-mm", "lengthens-stud-to-125-mm"),
    "change-f-ck-mpa": ("🪵change-f-ck-mpa", "upgrades-concrete-cylinder-strength-to-40-mpa"),
    "change-fu-mpa": ("🪨change-fu-mpa", "upgrades-stud-ultimate-strength-to-500-mpa"),
    "change-e-cm-mpa": ("🌍️change-e-cm-mpa", "raises-concrete-modulus-to-35000-mpa"),
    "change-v-ed-per-stud-kn": ("🏜️change-v-ed-per-stud-kn", "raises-per-stud-shear-to-62-5-kn"),
    "change-span-m": ("🌊change-span-m", "lengthens-span-to-12-m"),
    "change-fy-mpa": ("🌼change-fy-mpa", "upgrades-steel-yield-to-460-mpa"),
    "change-n-cycles-stud": ("🏔️change-n-cycles-stud", "raises-stud-cycle-count-to-5000000"),
    "change-delta-tau-stud-mpa": ("🌎️change-delta-tau-stud-mpa", "raises-stud-shear-stress-range-to-110-mpa"),
}

#: 🗣️ The real committed EN 1994 document, read where the domain already keeps it.
DSL_ASSET = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️composite-bridge-girder/🖼️assets/🗣️composite-bridge-girder.dsl.semio"

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
