"""🐍️ EN 1996's contribution to the norm reference implementation — the four things that are
genuinely per-standard, and nothing else.

The second producer this case's differential comparison needs is `semio_norm_vocabulary`, the ONE
independent Python implementation of the norm mutation vocabulary, imported here rather than copied. Its
module docstring carries the survey that established no third-party library reads or writes `s.norm.*`
and the honest boundary on the `.dsl.semio` carrier. This file adds no verb, no addressing rule and no
carrier rule: everything below is DATA read off this subset's own committed catalog, its own committed
specification vectors and its own committed example document.
"""

from __future__ import annotations

# region 🔖️Imports
from importlib import import_module

_vocabulary = import_module("🐍️")
Subset = _vocabulary.Subset
build_adapter = _vocabulary.build_adapter

# endregion 🔖️Imports


# region 🔖️Vocabulary
#: 🏷️ Every kind of `En1996Mutation`, in the committed catalog's (declaration) order.
KINDS = [
    "change-concentrated-bearing-length",
    "change-slab-span",
    "change-wall-length",
    "change-wall-height",
    "change-wall-thickness",
    "change-eccentricity-bottom",
    "change-eccentricity-top",
    "change-phi-infinity",
    "change-qk-snow",
    "insert-concentrated",
    "insert-load-case",
    "insert-opening",
    "insert-wall",
    "remove-concentrated",
    "remove-load-case",
    "remove-opening",
    "remove-wall",
    "change-annex",
    "change-qp-wind",
    "change-design-situation",
    "change-load-case-situation",
    "change-concentrated-force",
    "change-gk-slab",
    "change-qk-imposed",
    "change-is-basement",
    "change-storeys",
    "change-masonry-class",
    "change-imposed-category",
    "change-wall-label-de",
    "change-wall-label-en",
    "change-exposure",
    "change-concentrated-bearing-area",
    "change-slab-bearing-depth",
    "change-tributary-area",
    "change-fire-rei",
    "change-as-horizontal",
    "change-as-vertical",
    "change-f-yd",
    "change-reinforced",
    "change-bed-joint-thickness",
    "change-fm",
    "change-mortar-class",
    "change-mortar-type",
    "change-c-pe",
    "change-density",
    "change-support-sides",
    "change-unit-fb",
    "change-unit-group",
    "change-unit-height",
    "change-unit-length",
    "change-unit-material",
    "change-unit-width",
    "change-wall-type",
    "change-mu",
    "change-opening-height",
    "change-opening-sill",
    "change-opening-width",
    "change-hk-earth",
]

#: 🧫️ The committed specification vector each kind is measured on, as (leaf directory, scenario directory).
VECTORS = {
    "change-concentrated-bearing-length": ("↔️change-concentrated-bearing-length", "↔️applies-change-concentrated-bearing-length"),
    "change-slab-span": ("↔️change-slab-span", "↔️applies-change-slab-span"),
    "change-wall-length": ("↔️change-wall-length", "↔️applies-change-wall-length"),
    "change-wall-height": ("↕️change-wall-height", "↕️shortens-first-wall"),
    "change-wall-thickness": ("↕️change-wall-thickness", "↕️thickens-first-wall"),
    "change-eccentricity-bottom": ("↗️change-eccentricity-bottom", "↗️applies-change-eccentricity-bottom"),
    "change-eccentricity-top": ("↘️change-eccentricity-top", "↘️applies-change-eccentricity-top"),
    "change-phi-infinity": ("♾️change-phi-infinity", "♾️applies-change-phi-infinity"),
    "change-qk-snow": ("❄️change-qk-snow", "❄️applies-change-qk-snow"),
    "insert-concentrated": ("➕️insert-concentrated", "➕️applies-insert-concentrated"),
    "insert-load-case": ("➕️insert-load-case", "➕️applies-insert-load-case"),
    "insert-opening": ("➕️insert-opening", "➕️applies-insert-opening"),
    "insert-wall": ("➕️insert-wall", "➕️inserts-a-wall"),
    "remove-concentrated": ("➖️remove-concentrated", "➖️applies-remove-concentrated"),
    "remove-load-case": ("➖️remove-load-case", "➖️applies-remove-load-case"),
    "remove-opening": ("➖️remove-opening", "➖️applies-remove-opening"),
    "remove-wall": ("➖️remove-wall", "➖️removes-first-wall"),
    "change-annex": ("🌍️change-annex", "🌍️switches-annex-to-en"),
    "change-qp-wind": ("🌬️change-qp-wind", "🌬️applies-change-qp-wind"),
    "change-design-situation": ("🎭️change-design-situation", "🌋️switches-the-design-situation-to-seismic"),
    "change-load-case-situation": ("🎭️change-load-case-situation", "🎭️applies-change-load-case-situation"),
    "change-concentrated-force": ("🏋️change-concentrated-force", "🏋️applies-change-concentrated-force"),
    "change-gk-slab": ("🏋️change-gk-slab", "🏋️applies-change-gk-slab"),
    "change-qk-imposed": ("🏋️change-qk-imposed", "🏋️applies-change-qk-imposed"),
    "change-is-basement": ("🏗️change-is-basement", "🏗️applies-change-is-basement"),
    "change-storeys": ("🏢️change-storeys", "🏢️applies-change-storeys"),
    "change-masonry-class": ("🏭️change-masonry-class", "🏭️applies-change-masonry-class"),
    "change-imposed-category": ("🏷️change-imposed-category", "🏷️applies-change-imposed-category"),
    "change-wall-label-de": ("🏷️change-wall-label-de", "🏷️applies-change-wall-label-de"),
    "change-wall-label-en": ("🏷️change-wall-label-en", "🏷️applies-change-wall-label-en"),
    "change-exposure": ("💧️change-exposure", "💧️applies-change-exposure"),
    "change-concentrated-bearing-area": ("📐️change-concentrated-bearing-area", "📐️applies-change-concentrated-bearing-area"),
    "change-slab-bearing-depth": ("📐️change-slab-bearing-depth", "📐️applies-change-slab-bearing-depth"),
    "change-tributary-area": ("📐️change-tributary-area", "📐️applies-change-tributary-area"),
    "change-fire-rei": ("🔥️change-fire-rei", "🔥️applies-change-fire-rei"),
    "change-as-horizontal": ("🔩change-as-horizontal", "🔩applies-change-as-horizontal"),
    "change-as-vertical": ("🔩change-as-vertical", "🔩applies-change-as-vertical"),
    "change-f-yd": ("🔩change-f-yd", "🔩applies-change-f-yd"),
    "change-reinforced": ("🔩change-reinforced", "🔩applies-change-reinforced"),
    "change-bed-joint-thickness": ("🥪️change-bed-joint-thickness", "🥪️applies-change-bed-joint-thickness"),
    "change-fm": ("🧈change-fm", "🧈applies-change-fm"),
    "change-mortar-class": ("🧈change-mortar-class", "🧈upgrades-mortar-to-m20"),
    "change-mortar-type": ("🧈change-mortar-type", "🧈applies-change-mortar-type"),
    "change-c-pe": ("🧮change-c-pe", "🧮applies-change-c-pe"),
    "change-density": ("🧱change-density", "🧱applies-change-density"),
    "change-support-sides": ("🧱change-support-sides", "🧱sets-four-sided-support"),
    "change-unit-fb": ("🧱change-unit-fb", "🧱raises-unit-strength"),
    "change-unit-group": ("🧱change-unit-group", "🧱applies-change-unit-group"),
    "change-unit-height": ("🧱change-unit-height", "🧱applies-change-unit-height"),
    "change-unit-length": ("🧱change-unit-length", "🧱applies-change-unit-length"),
    "change-unit-material": ("🧱change-unit-material", "🧱applies-change-unit-material"),
    "change-unit-width": ("🧱change-unit-width", "🧱applies-change-unit-width"),
    "change-wall-type": ("🧱change-wall-type", "🧱applies-change-wall-type"),
    "change-mu": ("🧲️change-mu", "🧲️raises-the-bed-joint-friction-coefficient-to-0-625"),
    "change-opening-height": ("🪟change-opening-height", "🪟applies-change-opening-height"),
    "change-opening-sill": ("🪟change-opening-sill", "🪟applies-change-opening-sill"),
    "change-opening-width": ("🪟change-opening-width", "🪟applies-change-opening-width"),
    "change-hk-earth": ("🪨change-hk-earth", "🪨applies-change-hk-earth"),
}

#: 🗣️ The real committed EN 1996 document, read where the domain already keeps it.
DSL_ASSET = "asset://🧱️loadbearing-wall/🧱️loadbearing-wall/🗣️.dsl.semio"

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
