"""🐍️ EN 1995's contribution to the norm reference implementation — the four things that are
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
    "change-n-ed-kn",
    "change-v-ed-kn",
    "change-w-mm3",
    "change-a-mm2",
    "change-b-mm",
    "change-h-mm",
    "change-fmk",
    "change-fc0-k",
    "change-service-class",
    "change-load-duration",
    "change-m-crit-knm",
    "change-f-ed-kn",
    "change-a-ef-mm2",
    "change-fvk",
    "change-fire-duration-min",
    "change-section-depth-mm",
    "change-a-vert-ms2",
    "change-n-cycles-bridge",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-annex": ("📐change-annex", "switches-from-the-german-na-to-the-recommended-en-annex"),
    "change-m-ed-knm": ("🧴change-m-ed-knm", "raises-the-design-bending-moment-to-32-knm"),
    "change-n-ed-kn": ("🧽change-n-ed-kn", "raises-the-design-axial-force-to-75-kn"),
    "change-v-ed-kn": ("🧺change-v-ed-kn", "raises-the-design-shear-force-to-22-5-kn"),
    "change-w-mm3": ("🪑change-w-mm3", "raises-the-section-modulus-to-4000000-mm3"),
    "change-a-mm2": ("🪣change-a-mm2", "enlarges-the-gross-area-to-72000-mm2"),
    "change-b-mm": ("🧵change-b-mm", "widens-the-beam-to-240-mm"),
    "change-h-mm": ("🪤change-h-mm", "deepens-the-beam-to-360-mm"),
    "change-fmk": ("🧷change-fmk", "upgrades-the-bending-strength-class-to-28-mpa"),
    "change-fc0-k": ("🪡change-fc0-k", "raises-the-parallel-compressive-strength-to-26-5-mpa"),
    "change-service-class": ("🧹change-service-class", "moves-the-beam-from-service-class-1-to-service-class-2"),
    "change-load-duration": ("🪒change-load-duration", "shortens-the-load-duration-class-from-medium-to-short"),
    "change-m-crit-knm": ("🪥change-m-crit-knm", "raises-the-critical-buckling-moment-to-96-knm"),
    "change-f-ed-kn": ("🧶change-f-ed-kn", "raises-the-design-fastener-force-to-24-kn"),
    "change-a-ef-mm2": ("🪝change-a-ef-mm2", "enlarges-the-effective-connection-area-to-16000-mm2"),
    "change-fvk": ("🧲change-fvk", "lowers-the-characteristic-shear-strength-to-3-5-mpa"),
    "change-fire-duration-min": ("🪢change-fire-duration-min", "raises-the-fire-exposure-from-r30-to-r60"),
    "change-section-depth-mm": ("🪠change-section-depth-mm", "raises-the-size-effect-depth-to-360-mm"),
    "change-a-vert-ms2": ("🧰change-a-vert-ms2", "doubles-the-vertical-footfall-acceleration-to-0-5-m-s2"),
    "change-n-cycles-bridge": ("🧼change-n-cycles-bridge", "quadruples-the-bridge-fatigue-cycles-to-2000000"),
}

#: 🗣️ The real committed EN 1995 document, read where the domain already keeps it.
DSL_ASSET = "asset://📚️examples/📕️glulam-footbridge/🖼️assets/🧪️glulam-footbridge/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.en1995.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("EN 1995", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
