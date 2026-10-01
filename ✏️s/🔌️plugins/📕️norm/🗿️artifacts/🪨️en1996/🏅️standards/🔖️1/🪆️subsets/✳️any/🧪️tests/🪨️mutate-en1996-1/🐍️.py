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
import json
from importlib import import_module
from pathlib import Path

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

#: 🧫️ The committed specification vector each kind is measured on, as (leaf directory, scenario directory), and each
#: further refusal, no-op or clamp vector, keyed by its `<kind>-<slug>` row as (kind, leaf directory, scenario directory).
VECTORS = {
    "change-concentrated-bearing-length": ("📏change-concentrated-bearing-length", "✅apply"),
    "change-slab-span": ("↔️change-slab-span", "✅apply"),
    "change-wall-length": ("↔️change-wall-length", "✅apply"),
    "change-wall-height": ("↕️change-wall-height", "✅apply"),
    "change-wall-thickness": ("↕️change-wall-thickness", "✅apply"),
    "change-eccentricity-bottom": ("↗️change-eccentricity-bottom", "✅apply"),
    "change-eccentricity-top": ("↘️change-eccentricity-top", "✅apply"),
    "change-phi-infinity": ("♾️change-phi-infinity", "✅apply"),
    "change-qk-snow": ("❄️change-qk-snow", "✅apply"),
    "insert-concentrated": ("➕️insert-concentrated", "✅apply"),
    "insert-load-case": ("➕️insert-load-case", "✅apply"),
    "insert-opening": ("➕️insert-opening", "✅apply"),
    "insert-wall": ("➕️insert-wall", "✅apply"),
    "remove-concentrated": ("➖️remove-concentrated", "✅apply"),
    "remove-load-case": ("➖️remove-load-case", "✅apply"),
    "remove-opening": ("➖️remove-opening", "✅apply"),
    "remove-wall": ("➖️remove-wall", "✅apply"),
    "change-annex": ("🌍️change-annex", "✅apply"),
    "change-qp-wind": ("🌬️change-qp-wind", "✅apply"),
    "change-design-situation": ("🎭️change-design-situation", "✅apply"),
    "change-load-case-situation": ("🎭️change-load-case-situation", "✅apply"),
    "change-concentrated-force": ("🏋️change-concentrated-force", "✅apply"),
    "change-gk-slab": ("🏋️change-gk-slab", "✅apply"),
    "change-qk-imposed": ("🏋️change-qk-imposed", "✅apply"),
    "change-is-basement": ("🏗️change-is-basement", "✅apply"),
    "change-storeys": ("🏢️change-storeys", "✅apply"),
    "change-masonry-class": ("🏭️change-masonry-class", "✅apply"),
    "change-imposed-category": ("🏷️change-imposed-category", "✅apply"),
    "change-wall-label-de": ("🏷️change-wall-label-de", "✅apply"),
    "change-wall-label-en": ("🏷️change-wall-label-en", "✅apply"),
    "change-exposure": ("💧️change-exposure", "✅apply"),
    "change-concentrated-bearing-area": ("📐️change-concentrated-bearing-area", "✅apply"),
    "change-slab-bearing-depth": ("📐️change-slab-bearing-depth", "✅apply"),
    "change-tributary-area": ("📐️change-tributary-area", "✅apply"),
    "change-fire-rei": ("🔥️change-fire-rei", "✅apply"),
    "change-as-horizontal": ("🔩change-as-horizontal", "✅apply"),
    "change-as-vertical": ("🔩change-as-vertical", "✅apply"),
    "change-f-yd": ("🔩change-f-yd", "✅apply"),
    "change-reinforced": ("🔩change-reinforced", "✅apply"),
    "change-bed-joint-thickness": ("🥪️change-bed-joint-thickness", "✅apply"),
    "change-fm": ("🧈change-fm", "✅apply"),
    "change-mortar-class": ("🧈change-mortar-class", "✅apply"),
    "change-mortar-type": ("🧈change-mortar-type", "✅apply"),
    "change-c-pe": ("🧮change-c-pe", "✅apply"),
    "change-density": ("🧱change-density", "✅apply"),
    "change-support-sides": ("🧱change-support-sides", "✅apply"),
    "change-unit-fb": ("🧱change-unit-fb", "✅apply"),
    "change-unit-group": ("🧱change-unit-group", "✅apply"),
    "change-unit-height": ("🧱change-unit-height", "✅apply"),
    "change-unit-length": ("🧱change-unit-length", "✅apply"),
    "change-unit-material": ("🧱change-unit-material", "✅apply"),
    "change-unit-width": ("🧱change-unit-width", "✅apply"),
    "change-wall-type": ("🧱change-wall-type", "✅apply"),
    "change-mu": ("🧲️change-mu", "✅apply"),
    "change-opening-height": ("🪟change-opening-height", "✅apply"),
    "change-opening-sill": ("🪟change-opening-sill", "✅apply"),
    "change-opening-width": ("🪟change-opening-width", "✅apply"),
    "change-hk-earth": ("🪨change-hk-earth", "✅apply"),
    "change-annex-noop": ("change-annex", "🌍️change-annex", "🟰noop"),
    "change-design-situation-noop": ("change-design-situation", "🎭️change-design-situation", "🟰noop"),
    "change-storeys-noop": ("change-storeys", "🏢️change-storeys", "🟰noop"),
    "change-masonry-class-noop": ("change-masonry-class", "🏭️change-masonry-class", "🟰noop"),
}

#: 📐️ Each kind's committed leaf payload schema, read where the subset keeps it; its stated bounds are the payload's.
SCHEMAS = {kind: json.loads((Path(__file__).resolve().parents[2] / "🧬️schema" / "🧬️mutations" / VECTORS[kind][0] / "🧬️schema" / "🔣️.json").read_text(encoding="utf-8")) for kind in KINDS}

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
    return build_adapter(Subset("EN 1996", KINDS, VECTORS, DSL_ASSET, ENVELOPE, schemas=SCHEMAS))
# endregion 🔖️Registration
