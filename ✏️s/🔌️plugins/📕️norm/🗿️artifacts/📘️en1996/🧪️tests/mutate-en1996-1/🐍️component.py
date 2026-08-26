"""🐍️ EN 1996's contribution to the norm reference implementation — the four things that are
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
    "change-m-ed-knm",
    "change-n-ed-kn",
    "change-v-ed-kn",
    "change-h-ed-kn",
    "change-z-mm3",
    "change-area-mm2",
    "change-shear-area-mm2",
    "change-fk-mpa",
    "change-f-vk-mpa",
    "change-annex",
    "change-masonry-class",
    "change-design-situation",
    "change-mu",
    "change-wall-thickness-mm",
    "change-fire-resistance-min",
    "change-unit",
    "change-exposure",
    "change-mortar",
    "change-bed-joint-thickness-mm",
    "change-storeys",
    "change-h-ef-mm",
    "change-t-ef-mm",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-m-ed-knm": ("📐change-m-ed-knm", "raises-the-design-bending-moment-to-12-5-knm"),
    "change-n-ed-kn": ("🔽change-n-ed-kn", "raises-the-design-axial-force-to-320-kn"),
    "change-v-ed-kn": ("🔼change-v-ed-kn", "raises-the-design-shear-force-to-48-kn"),
    "change-h-ed-kn": ("↔️change-h-ed-kn", "raises-the-design-sliding-force-to-26-kn"),
    "change-z-mm3": ("➡️change-z-mm3", "raises-the-section-modulus-to-9500000-mm3"),
    "change-area-mm2": ("⬅️change-area-mm2", "enlarges-the-gross-area-to-640000-mm2"),
    "change-shear-area-mm2": ("📏change-shear-area-mm2", "enlarges-the-shear-area-to-384000-mm2"),
    "change-fk-mpa": ("🟩change-fk-mpa", "raises-the-characteristic-compressive-strength-to-7-5-mpa"),
    "change-f-vk-mpa": ("✂️change-f-vk-mpa", "raises-the-characteristic-shear-strength-to-0-375-mpa"),
    "change-annex": ("🔨change-annex", "switches-from-the-german-na-to-the-recommended-en-annex"),
    "change-masonry-class": ("🗺️change-masonry-class", "downgrades-manufacturing-control-to-class-4"),
    "change-design-situation": ("🧱change-design-situation", "switches-the-design-situation-to-seismic"),
    "change-mu": ("🏗️change-mu", "raises-the-bed-joint-friction-coefficient-to-0-625"),
    "change-wall-thickness-mm": ("🎢change-wall-thickness-mm", "thickens-the-wall-to-300-mm"),
    "change-fire-resistance-min": ("🧊change-fire-resistance-min", "raises-the-fire-resistance-requirement-from-r60-to-r90"),
    "change-unit": ("🌡️change-unit", "switches-the-masonry-unit-from-clay-to-calcium-silicate"),
    "change-exposure": ("💧change-exposure", "moves-the-wall-to-exposure-class-mx3"),
    "change-mortar": ("🌬️change-mortar", "upgrades-the-general-purpose-mortar-to-m10"),
    "change-bed-joint-thickness-mm": ("🔥change-bed-joint-thickness-mm", "thickens-the-bed-joint-to-the-15-mm-upper-limit"),
    "change-storeys": ("❄️change-storeys", "adds-a-third-storey-at-the-simplified-method-limit"),
    "change-h-ef-mm": ("⚡change-h-ef-mm", "lengthens-the-effective-height-to-2750-mm"),
    "change-t-ef-mm": ("🔆change-t-ef-mm", "raises-the-effective-thickness-to-300-mm"),
}

#: 🗣️ The real committed EN 1996 document, read where the domain already keeps it.
DSL_ASSET = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️loadbearing-wall/🖼️assets/🗣️loadbearing-wall.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.en1996.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("EN 1996", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
