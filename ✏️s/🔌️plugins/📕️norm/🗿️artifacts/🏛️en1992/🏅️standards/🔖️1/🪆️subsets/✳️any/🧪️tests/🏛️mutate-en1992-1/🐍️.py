"""🐍️ EN 1992's contribution to the norm reference implementation — the four things that are
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
    "change-annex",
    "change-m-ed-knm",
    "change-v-ed-kn",
    "change-f-ck",
    "change-b-mm",
    "change-d-mm",
    "change-as-mm2",
    "change-f-yk",
    "change-rho-l",
    "change-n-ed-kn",
    "change-p-kn",
    "change-ac-mm2",
    "change-use-fem",
    "change-span-m",
    "change-udl-kn-m",
    "change-fire-rating",
    "change-provided-axis-distance-mm",
    "change-bridge-sigma-c-mpa",
    "change-bridge-delta-sigma-s-mpa",
    "change-tightness-class",
    "change-hd-over-h",
    "change-liquid-sigma-s-mpa",
    "change-liquid-rho-p-eff",
    "change-liquid-f-ct-eff-mpa",
    "change-liquid-es-mpa",
    "change-liquid-sr-max-mm",
    "change-anchor-h-ef-mm",
    "change-anchor-cracked",
    "change-anchor-f-uk-mpa",
    "change-anchor-f-yk-mpa",
    "change-anchor-as-mm2",
    "change-anchor-d-mm",
    "change-anchor-c1-mm",
    "change-anchor-n-ed-kn",
    "change-anchor-v-ed-kn",
]

#: 🧫️ The committed specification vector each kind publishes, as (triad directory, fixture name).
VECTORS = {
    "change-annex": ("🌍️change-annex", "switches-annex-to-en"),
    "change-m-ed-knm": ("🐮change-m-ed-knm", "raises-m-ed-knm-to-187-5"),
    "change-v-ed-kn": ("🦒change-v-ed-kn", "raises-v-ed-kn-to-96-5"),
    "change-f-ck": ("🐜change-f-ck", "raises-f-ck-to-45-0"),
    "change-b-mm": ("🦂change-b-mm", "raises-b-mm-to-375-0"),
    "change-d-mm": ("🕷️change-d-mm", "raises-d-mm-to-512-5"),
    "change-as-mm2": ("🐍change-as-mm2", "raises-a-s-mm2-to-1608-5"),
    "change-f-yk": ("🦔change-f-yk", "raises-f-yk-to-550-0"),
    "change-rho-l": ("🐘change-rho-l", "raises-rho-l-to-0-015625"),
    "change-n-ed-kn": ("🐷change-n-ed-kn", "raises-n-ed-kn-to-62-5"),
    "change-p-kn": ("🐗change-p-kn", "raises-p-kn-to-45-5"),
    "change-ac-mm2": ("🐞change-ac-mm2", "raises-a-c-mm2-to-168750-0"),
    "change-use-fem": ("🕸️change-use-fem", "turns-use-fem-on"),
    "change-span-m": ("🦏change-span-m", "raises-span-m-to-7-5"),
    "change-udl-kn-m": ("🐪change-udl-kn-m", "raises-udl-kn-m-to-26-25"),
    "change-fire-rating": ("🔥️change-fire-rating", "switches-fire-rating-to-r120"),
    "change-provided-axis-distance-mm": ("🦌change-provided-axis-distance-mm", "raises-provided-axis-distance-mm-to-42-5"),
    "change-bridge-sigma-c-mpa": ("🦗change-bridge-sigma-c-mpa", "raises-bridge-sigma-c-mpa-to-15-75"),
    "change-bridge-delta-sigma-s-mpa": ("🦟change-bridge-delta-sigma-s-mpa", "raises-bridge-delta-sigma-s-mpa-to-132-5"),
    "change-tightness-class": ("💧️change-tightness-class", "switches-tightness-class-to-tc2"),
    "change-hd-over-h": ("🦉change-hd-over-h", "raises-hd-over-h-to-12-5"),
    "change-liquid-sigma-s-mpa": ("🐑change-liquid-sigma-s-mpa", "raises-liquid-sigma-s-mpa-to-235-5"),
    "change-liquid-rho-p-eff": ("🦄change-liquid-rho-p-eff", "raises-liquid-rho-p-eff-to-0-0078125"),
    "change-liquid-f-ct-eff-mpa": ("🐎change-liquid-f-ct-eff-mpa", "raises-liquid-f-ct-eff-mpa-to-3-25"),
    "change-liquid-es-mpa": ("🐴change-liquid-es-mpa", "raises-liquid-e-s-mpa-to-205000-0"),
    "change-liquid-sr-max-mm": ("🐐change-liquid-sr-max-mm", "raises-liquid-s-r-max-mm-to-312-5"),
    "change-anchor-h-ef-mm": ("🦭change-anchor-h-ef-mm", "raises-anchor-h-ef-mm-to-105-0"),
    "change-anchor-cracked": ("💥️change-anchor-cracked", "turns-anchor-cracked-on"),
    "change-anchor-f-uk-mpa": ("🐳change-anchor-f-uk-mpa", "raises-anchor-f-uk-mpa-to-900-0"),
    "change-anchor-f-yk-mpa": ("🛡️change-anchor-f-yk-mpa", "raises-anchor-f-yk-mpa-to-720-0"),
    "change-anchor-as-mm2": ("🦋change-anchor-as-mm2", "raises-anchor-a-s-mm2-to-157-0"),
    "change-anchor-d-mm": ("⭕️change-anchor-d-mm", "raises-anchor-d-mm-to-16-0"),
    "change-anchor-c1-mm": ("🐌change-anchor-c1-mm", "raises-anchor-c1-mm-to-137-5"),
    "change-anchor-n-ed-kn": ("🐊change-anchor-n-ed-kn", "raises-anchor-n-ed-kn-to-22-5"),
    "change-anchor-v-ed-kn": ("🦎change-anchor-v-ed-kn", "raises-anchor-v-ed-kn-to-11-25"),
}

#: 🗣️ The real committed EN 1992 document, read where the domain already keeps it.
DSL_ASSET = "asset://📚️examples/📕️liquid-retaining-fem-anchor/🖼️assets/🧪️liquid-retaining-fem-anchor/🗣️.dsl.semio"

#: ✉️ The envelope token that artifact's text preamble must carry.
ENVELOPE = "norm.en1992.dsl"
# endregion 🔖️Vocabulary


# region 🔖️Registration
def adapter():
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    return build_adapter(Subset("EN 1992", KINDS, VECTORS, DSL_ASSET, ENVELOPE))
# endregion 🔖️Registration
