"""🐍️ EN 1997's contribution to the norm reference implementation — the four things that are
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
    "change-v-ed-kn",
    "change-h-ed-kn",
    "change-footing-area-m2",
    "change-phi-deg",
    "change-c-kpa",
    "change-gamma-kn-m3",
    "change-bm",
    "change-dfm",
    "change-es-mpa",
    "change-nu",
    "change-design-approach",
    "change-annex",
    "change-settlement-limit-mm",
    "change-n-pile-ed-kn",
    "change-alpha-s",
    "change-pile-dm",
    "change-qs-kpa",
    "change-pile-lm",
    "change-qb-kpa",
    "change-pile-base-area-m2",
    "change-pile-n-profiles",
    "change-z-investigated-m",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-v-ed-kn": ("🪒change-v-ed-kn", "raises-the-design-vertical-load-to-750-kn"),
    "change-h-ed-kn": ("🪥change-h-ed-kn", "raises-the-design-horizontal-load-to-120-kn"),
    "change-footing-area-m2": ("🧴change-footing-area-m2", "enlarges-the-footing-area-to-6-25-m2"),
    "change-phi-deg": ("🧼change-phi-deg", "raises-the-friction-angle-to-35-degrees"),
    "change-c-kpa": ("🧽change-c-kpa", "gives-the-drained-sand-12-5-kpa-of-effective-cohesion"),
    "change-gamma-kn-m3": ("🪠change-gamma-kn-m3", "raises-the-soil-unit-weight-to-20-kn-m3"),
    "change-bm": ("🧹change-bm", "widens-the-footing-to-2-5-m"),
    "change-dfm": ("🧺change-dfm", "deepens-the-founding-level-to-2-m"),
    "change-es-mpa": ("🪑change-es-mpa", "stiffens-the-soil-modulus-to-45-mpa"),
    "change-nu": ("🪞change-nu", "raises-poissons-ratio-to-0-375"),
    "change-design-approach": ("🛋️change-design-approach", "switches-from-design-approach-1-to-design-approach-2"),
    "change-annex": ("🛏️change-annex", "switches-from-the-german-na-to-the-recommended-en-annex"),
    "change-settlement-limit-mm": ("🚿change-settlement-limit-mm", "relaxes-the-settlement-limit-to-40-mm"),
    "change-n-pile-ed-kn": ("🛁change-n-pile-ed-kn", "raises-the-design-pile-axial-load-to-1200-kn"),
    "change-alpha-s": ("🌿change-alpha-s", "lowers-the-shaft-resistance-factor-to-0-5"),
    "change-pile-dm": ("🍀change-pile-dm", "enlarges-the-pile-diameter-to-0-75-m"),
    "change-qs-kpa": ("🌾change-qs-kpa", "raises-the-unit-shaft-resistance-to-120-kpa"),
    "change-pile-lm": ("🌵change-pile-lm", "lengthens-the-pile-to-15-m"),
    "change-qb-kpa": ("🌴change-qb-kpa", "raises-the-unit-base-resistance-to-3200-kpa"),
    "change-pile-base-area-m2": ("🌳change-pile-base-area-m2", "doubles-the-pile-base-area-to-0-5-m2"),
    "change-pile-n-profiles": ("🌲change-pile-n-profiles", "adds-a-third-investigated-ground-profile"),
    "change-z-investigated-m": ("🍁change-z-investigated-m", "deepens-the-investigated-depth-to-12-m"),
}

#: 🗣️ The real committed EN 1997 document, read where the domain already keeps it.
DSL_ASSET = "asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.en1997.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("EN 1997", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
