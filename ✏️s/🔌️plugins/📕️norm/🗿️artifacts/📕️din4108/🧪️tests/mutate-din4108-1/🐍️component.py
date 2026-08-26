"""🐍️ DIN 4108's contribution to the norm reference implementation — the four things that are
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
    "change-category",
    "change-climate",
    "change-airtightness-n50",
    "change-psi-times-l-sum",
    "change-rh-int",
    "change-catalog-id",
    "change-material-id",
    "change-airtightness-class",
    "change-t-int-c",
    "change-solar-absorptance",
    "change-irradiance-wm2",
    "change-moisture-mu-exterior",
    "change-moisture-mu-interior",
    "change-envelope-area-m2",
    "change-bb2-details-conform",
    "change-application-type",
    "change-declared-application-class",
    "insert-layer",
    "remove-layer",
    "reorder-layers",
    "change-layer-thickness",
    "change-layer-lambda",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-category": ("🪜change-category", "retypes-the-assembly-as-office"),
    "change-climate": ("🛠️change-climate", "moves-the-building-to-climate-zone-4"),
    "change-airtightness-n50": ("🧰change-airtightness-n50", "tightens-n50-to-1-point-5-per-hour"),
    "change-psi-times-l-sum": ("🧯change-psi-times-l-sum", "raises-the-thermal-bridge-sum-to-0-point-05"),
    "change-rh-int": ("🪣change-rh-int", "raises-indoor-relative-humidity-to-0-point-65"),
    "change-catalog-id": ("🧵change-catalog-id", "repoints-the-catalogue-entry-to-aw-07"),
    "change-material-id": ("🪥change-material-id", "swaps-the-insulation-material-to-eps"),
    "change-airtightness-class": ("🪚change-airtightness-class", "upgrades-the-airtightness-class-to-class1"),
    "change-t-int-c": ("🚨change-t-int-c", "raises-the-indoor-design-temperature-to-22-point-5-c"),
    "change-solar-absorptance": ("🏷️change-solar-absorptance", "lightens-the-facade-to-absorptance-0-point-25"),
    "change-irradiance-wm2": ("🧲change-irradiance-wm2", "raises-design-irradiance-to-750-w-per-m2"),
    "change-moisture-mu-exterior": ("🪛change-moisture-mu-exterior", "raises-the-exterior-mu-value-to-20"),
    "change-moisture-mu-interior": ("🪝change-moisture-mu-interior", "raises-the-interior-mu-value-to-2-point-5"),
    "change-envelope-area-m2": ("🪢change-envelope-area-m2", "grows-the-envelope-to-150-m2"),
    "change-bb2-details-conform": ("🔀change-bb2-details-conform", "declares-the-beiblatt-2-details-non-conforming"),
    "change-application-type": ("🪤change-application-type", "reclassifies-the-application-type-as-wab"),
    "change-declared-application-class": ("🧶change-declared-application-class", "declares-application-class-kh"),
    "insert-layer": ("🔢insert-layer", "inserts-an-interior-plaster-layer-at-index-1"),
    "remove-layer": ("🛡️remove-layer", "removes-the-load-bearing-masonry-layer"),
    "reorder-layers": ("🧷reorder-layers", "moves-the-insulation-in-front-of-the-masonry"),
    "change-layer-thickness": ("🪡change-layer-thickness", "thickens-the-insulation-layer-to-0-point-2-m"),
    "change-layer-lambda": ("🪒change-layer-lambda", "degrades-the-masonry-lambda-to-0-point-5"),
}

#: 🗣️ The real committed DIN 4108 document, read where the domain already keeps it.
DSL_ASSET = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.din4108.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("DIN 4108", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
